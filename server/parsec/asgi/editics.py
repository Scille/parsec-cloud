# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

"""
Editics routes works as a re-implementation of the OnlyOffice server, with a slightly modified
protocol (called the "editics protocol", by opposition of the original "OnlyOffic protocol")
to support end-to-end encryption.

OnlyOffice server uses websocket for communication with the client, however we
instead rely on SSE + HTTP POST (since websocket are sometime blocked by
enterprise proxies).

Editics ASGI routes:

    GET  /authenticated/{raw_organization_id}/editics/{realm_id}/{document_id}/listen
    POST /authenticated/{raw_organization_id}/editics/{realm_id}/{document_id}/send

    GET  /authenticated/{raw_organization_id}/editics/{realm_id}/{document_id}/initial_content
    POST /authenticated/{raw_organization_id}/editics/{realm_id}/{document_id}/initial_content

## Joining a session

Since an editics session lives within the memory of a server, all participants
must directly connect to it (this differs from the rest of Parsec server where
the PostgreSQL database is the centralized source of truth).

In practice, this means a device needs to use the `editics_join_session`
authenticated API command (TODO: this command is not yet implemented) to obtain
the URL of the editics server to connect to.

## Initial content

The initial content is an archive containing the document converted in the
OnlyOffice format (i.e. {bin, medias}). It is the starting base over
which all the patches created during the edition session are to be applied.

The initial content is uploaded once (using a dedicated route) by the first
editor connecting.

Any viewer that connects prior to that is expected to obtain the document
its own way (i.e. read the document from the workspace, then convert it with x2t),
and will do a reload once once the initial content gets uploaded (see
`parsecInitialContentUploaded` server event).

Initial content size is limited to 100Mo (configurable).

### Initial content upload/fetch API

Initial content upload/fetch API uses a custom format (Content-type `application/vnd.parsec.bundled-initial-content`):

    bundle = <key_index as u32 big endian > + <part 0> + <part 1> + ...
    part = <part_ciphertext size as u32 big endian> + <part_ciphertext>
    part_ciphertext = sign_and_encrypt_for_workspace(key_index, <part_cleartext> )
    part_cleartext = 0x00 + zstd(msgpack({'path': <file_path>, 'content': <file_content>}))

Each part contain one file and has a size limit of 25Mo (configurable).

The initial content bin file must have the path `/Editior.bin`, and is expected to
be in the first part (so that OnlyOffice editor can start loading it without
waiting for the full response).

Other files have arbitrary path with a namespace (e.g. `/media/foo.png`).

Parts are for:
- Server: Parts are stored in PostgreSQL as-is (TODO: not implemented yet), hence
  they are useful a upload/fetch requests to stream the content (instead of having
  the full document on the server's memory).
- Client: OnlyOffice editor loads the first part (i.e. document .bin) without waiting
  for the full response body.
  Note part streaming is not used to reduce memory usage in the client since both
  OnlyOffice and x2t (used when saving) require to have the whole document in memory.

We use a custom format for the outer envelop (i.e. `bundle`):
- We need a simple format that supports streaming (to load the .bin in OnlyOffice editor asap)
- Multipart is more complicated and would requires a custom implementation in client
    (Javascript web API doesn't provide a multipart parser).
- Msgpack is only available in libparsec, but the editics requests are done in
    the client GUI (cannot pass a `ReadableStream` Javascript object to libparsec).

The inner envelop (i.e. `part_ciphertext`) uses the regular serialization format v0
(based on msgpack + zstd) used in Parsec (i.e. certificates, manifests etc.). This
is because the decryption must be done in libparsec so using this format is simple.

## Medias added during edition

Parsec has a specific behavior when a media (most likely an image, OnlyOffice
doesn't support custom font) is added to the document during the edition session:
- Regular OnlyOffice: the media is uploaded in the OnlyOffice server and a regular
  URL (e.g. `medias/image.png`) is inserted in the document.
- Parsec: the media is embedded within the URL (i.e. `data:[<media-type>][;base64],<data>`)).

Having the media self-contained in the URL simplify the code (no need for a dedicated route),
but also means the media content is transmitted through the `saveChanges` events (hence
the /send route that should accept payload as big as the media size limit...).

Note those self-contained URLs only exist during the edition session: x2t turns them
into a regular URLs (referencing a file in the archive) when converting the document
into its final format (e.g. docx).

Media size is limited to 25Mo (configurable).
"""

from __future__ import annotations

from collections.abc import AsyncGenerator, AsyncIterable
from enum import Enum
from struct import pack, unpack_from
from typing import Annotated, NoReturn

from anyio.streams.memory import MemoryObjectReceiveStream
from fastapi import APIRouter, Depends, HTTPException, Request
from fastapi.responses import Response, StreamingResponse
from fastapi.sse import EventSourceResponse
from pydantic import BaseModel
from starlette.requests import ClientDisconnect

from parsec._parsec import ApiVersion, DateTime, OrganizationID, VlobID
from parsec.asgi.rpc import IncompatibleAPIVersionsError, settle_compatible_versions
from parsec.backend import Backend
from parsec.components.auth import AuthEditicsAuthBadOutcome, EditicsAuthInfo, EditicsToken
from parsec.components.editics import (
    EditicsFetchInitialContentBadOutcome,
    EditicsParticipantListenBadOutcome,
    EditicsParticipantSendBadOutcome,
    EditicsParticipantSessionEventStream,
    EditicsUploadInitialContentBadOutcome,
)
from parsec.editics_protocol import (
    EDITICS_API_VERSION,
    EditicsClientEventAdapter,
    EditicsParticipantSessionID,
    EditicsServerEvent,
)

EDITICS_SUPPORTED_API_VERSIONS = (EDITICS_API_VERSION,)

editics_router = APIRouter(include_in_schema=False)

ACCEPT_TYPE_SSE = "text/event-stream"

INITIAL_CONTENT_MEDIA_TYPE = "application/vnd.parsec.bundled-initial-content"


class EditicsHttpStatus(Enum):
    """
    HTTP statuses used by the editics routes.

    Try mimic the statuses from the RPC API for simplicity, though the two APIs
    are totally independent.
    """

    # Basic request parsing & authentication errors

    MissingAuthenticationInfo = 401
    BadAuthenticationInfo = 403
    BadAcceptType = 406
    ContentTooLarge = 413
    BadContentTypeOrInvalidBody = 415
    UnsupportedApiVersion = 422
    TokenExpired = 498
    UserRevoked = 460
    UserFrozen = 461
    UserNoRealmRole = 462
    OrganizationExpired = 465

    # Post-authentication, editics workflow errors

    # First client event must be `auth`
    ParticipantNotAuth = 480
    # Subsequent `auth` are reconnection, and a device cannot do impersonation
    # by reconnecting with a participant ID related to another device!
    ParticipantDeviceMismatch = 481
    # A reconnecting participant ID mismatch the transport participant ID (i.e.
    # the one in the listen/send routes URL). This is not supposed to happen and
    # indicates a buggy client.
    ParticipantIdMismatch = 482
    # A reconnecting participant cannot change the mode it originally joined with
    ParticipantModeMismatch = 483
    # Participant has READER role and tries to edit
    ReaderMustBeViewer = 484
    # The session has no initial content yet: the first editor knows (from its
    # `auth` server event) it must upload it before anything else. This is also
    # used by the fetch route to tell a client the content is not available yet.
    InitialContentMissing = 485
    # The session already has its initial content (first-upload-wins): the client
    # that lost the concurrent upload race must fetch it instead of uploading
    # its own conversion.
    InitialContentAlreadyUploaded = 486


# e.g. `ParticipantNotAuth` -> `Participant not auth`
EDITICS_HTTP_STATUS_DETAILS = {
    status: ("".join(" " + c.lower() if c.isupper() else c for c in status.name))
    .strip()
    .capitalize()
    for status in EditicsHttpStatus
}


def _editics_abort(status: EditicsHttpStatus, api_version: ApiVersion, **headers: str) -> NoReturn:
    detail = EDITICS_HTTP_STATUS_DETAILS[status]
    raise HTTPException(
        status_code=status.value,
        headers={"Api-Version": str(api_version), **headers},
        detail={"detail": detail},
    )


async def _editics_get_initial_content_parts_with_limit_check(
    request: Request,
    max_document_size: int,
    max_part_size: int,
    api_version: ApiVersion,
) -> tuple[int, AsyncGenerator[bytes | memoryview[int]]]:
    """
    Read the wire format:

        bundle = <key_index as u32 big endian > + <part 0> + <part 1> + ...
        part = <part_ciphertext size as u32 big endian> + <part_ciphertext>

    The implementation is rather cumbersome but is needed in order to have in
    memory a single part at a time instead of the whole initial document.
    """
    chunks_stream = _editics_get_body_with_limit_check(request, max_document_size, api_version)
    # Data from previous chunk belonging to the next part
    pending_data: memoryview[int] = memoryview(b"")

    async def _read(n: int) -> bytes | memoryview[int]:
        nonlocal pending_data

        if len(pending_data) >= n:
            # Already enough data
            data = pending_data[:n]
            pending_data = pending_data[n:]
            return data

        else:
            # No enough data yet
            current_data: list[bytes | memoryview[int]]
            if pending_data:
                current_data = [pending_data]
                pending_data = memoryview(b"")
                remaining_data_size = n - len(pending_data)
            else:
                current_data = []
                remaining_data_size = n
            while True:
                # Raises `StopAsyncIteration` once the body done
                chunk = await chunks_stream.asend(None)

                remaining_data_size -= len(chunk)
                current_data.append(chunk)
                if remaining_data_size == 0:
                    # We have enough data!
                    # (Note CPython has a fast path if `current_data` contains a
                    # single bytes instance by just returning it as-is)
                    return b"".join(current_data)
                elif remaining_data_size < 0:
                    # We have too much data!
                    pending_data = memoryview[int](current_data[-1])[remaining_data_size:]
                    current_data[-1] = current_data[-1][:remaining_data_size]
                    return b"".join(current_data)

    try:
        (key_index,) = unpack_from("!I", await _read(4), 0)
    except (StopAsyncIteration, ValueError):
        _editics_abort(EditicsHttpStatus.BadContentTypeOrInvalidBody, api_version)

    async def _stream_parts() -> AsyncGenerator[bytes | memoryview[int]]:
        while True:
            try:
                (part_size,) = unpack_from("!I", await _read(4), 0)
            except StopAsyncIteration:
                break
            except ValueError:
                _editics_abort(EditicsHttpStatus.BadContentTypeOrInvalidBody, api_version)

            if part_size > max_part_size:
                _editics_abort(EditicsHttpStatus.ContentTooLarge, api_version)

            try:
                part = await _read(part_size)
            except StopAsyncIteration:
                _editics_abort(EditicsHttpStatus.BadContentTypeOrInvalidBody, api_version)

            yield part
            del part

    return (key_index, _stream_parts())


async def _editics_get_body_with_limit_check(
    request: Request, max_content_length: int, api_version: ApiVersion
) -> AsyncGenerator[bytes]:
    try:
        content_length = int(request.headers["Content-Length"])

    except ValueError:
        _editics_abort(EditicsHttpStatus.ContentTooLarge, api_version)

    except KeyError:
        # Header missing, we must be in chunk-encoding mode
        content_length = max_content_length

    else:
        if content_length > max_content_length:
            _editics_abort(EditicsHttpStatus.ContentTooLarge, api_version)

    streamed_size = 0
    try:
        async for chunk in request.stream():
            streamed_size += len(chunk)
            if streamed_size > content_length:
                _editics_abort(EditicsHttpStatus.ContentTooLarge, api_version)
            yield chunk

    # The client disconnected while sending the body.
    # Here we simply raise an HTTP exception to ignore the `ClientDisconnect` exception
    # so that it doesn't get logged as an error.
    except ClientDisconnect:
        _editics_abort(EditicsHttpStatus.ContentTooLarge, api_version)


async def _editics_auth_or_abort(
    backend: Backend,
    request: Request,
    raw_organization_id: str,
    raw_realm_id: str,
    raw_document_id: str,
) -> tuple[ApiVersion, EditicsAuthInfo, VlobID]:
    # 1) Get API version

    try:
        client_api_version = ApiVersion.from_str(request.headers.get("Api-Version", ""))
        settled_api_version, _ = settle_compatible_versions(
            EDITICS_SUPPORTED_API_VERSIONS, (client_api_version,)
        )
    except (ValueError, IncompatibleAPIVersionsError):
        supported_api_versions = ";".join(str(v) for v in EDITICS_SUPPORTED_API_VERSIONS)
        status_code = EditicsHttpStatus.UnsupportedApiVersion
        raise HTTPException(
            status_code=status_code.value,
            headers={"Supported-Api-Versions": supported_api_versions},
            detail=EDITICS_HTTP_STATUS_DETAILS[status_code],
        )

    # 2) Parse authorization header

    try:
        raw_authorization = request.headers["Authorization"]
    except KeyError:
        _editics_abort(EditicsHttpStatus.MissingAuthenticationInfo, settled_api_version)

    try:
        expected_bearer, raw_editics_token = raw_authorization.split()
        if expected_bearer.lower() != "bearer":
            raise ValueError
        token = EditicsToken.from_raw(raw_editics_token.encode())
    except ValueError:
        _editics_abort(EditicsHttpStatus.MissingAuthenticationInfo, settled_api_version)

    # 3) Parse organization/realm/document IDs

    try:
        organization_id = OrganizationID(raw_organization_id)
        realm_id = VlobID.from_hex(raw_realm_id)
        document_id = VlobID.from_hex(raw_document_id)
    except ValueError:
        _editics_abort(EditicsHttpStatus.BadAuthenticationInfo, settled_api_version)

    # 4) Actual auth lookup

    outcome = await backend.auth.editics_auth(
        now=DateTime.now(),
        organization_id=organization_id,
        realm_id=realm_id,
        token=token,
    )
    match outcome:
        case EditicsAuthInfo() as auth_info:
            return (settled_api_version, auth_info, document_id)

        case (
            AuthEditicsAuthBadOutcome.DEVICE_NOT_FOUND
            | AuthEditicsAuthBadOutcome.ORGANIZATION_NOT_FOUND
            | AuthEditicsAuthBadOutcome.INVALID_TOKEN
        ):
            _editics_abort(EditicsHttpStatus.BadAuthenticationInfo, settled_api_version)
        case AuthEditicsAuthBadOutcome.TOKEN_OUT_OF_BALLPARK:
            _editics_abort(EditicsHttpStatus.TokenExpired, settled_api_version)
        case AuthEditicsAuthBadOutcome.USER_REVOKED:
            _editics_abort(EditicsHttpStatus.UserRevoked, settled_api_version)
        case AuthEditicsAuthBadOutcome.USER_FROZEN:
            _editics_abort(EditicsHttpStatus.UserFrozen, settled_api_version)
        case (
            AuthEditicsAuthBadOutcome.REALM_NOT_FOUND
            | AuthEditicsAuthBadOutcome.USER_NO_REALM_ROLE
            | AuthEditicsAuthBadOutcome.REALM_DELETED
        ):
            _editics_abort(EditicsHttpStatus.UserNoRealmRole, settled_api_version)
        case AuthEditicsAuthBadOutcome.ORGANIZATION_EXPIRED:
            _editics_abort(EditicsHttpStatus.OrganizationExpired, settled_api_version)


# --- Routes ------------------------------------------------------------------


@editics_router.post(
    "/authenticated/{raw_organization_id}/editics/{raw_realm_id}/{raw_document_id}/initial_content"
)
async def editics_upload_initial_content(
    raw_organization_id: str,
    raw_realm_id: str,
    raw_document_id: str,
    psid: EditicsParticipantSessionID,
    request: Request,
) -> Response:
    backend: Backend = request.app.state.backend

    api_version, auth_info, document_id = await _editics_auth_or_abort(
        backend, request, raw_organization_id, raw_realm_id, raw_document_id
    )

    if request.headers.get("Content-Type") != INITIAL_CONTENT_MEDIA_TYPE:
        _editics_abort(EditicsHttpStatus.BadContentTypeOrInvalidBody, api_version)

    key_index, stream_parts = await _editics_get_initial_content_parts_with_limit_check(
        request,
        backend.config.editics_max_document_size,
        backend.config.editics_max_part_size,
        api_version,
    )

    outcome = await backend.editics.upload_initial_content(
        auth_info,
        document_id,
        psid,
        key_index,
        stream_parts,
    )
    match outcome:
        case None:
            return Response(
                status_code=204,
                headers={"Api-Version": str(api_version)},
            )
        case EditicsUploadInitialContentBadOutcome.NOT_ALLOWED:
            _editics_abort(EditicsHttpStatus.ReaderMustBeViewer, api_version)
        case EditicsUploadInitialContentBadOutcome.ALREADY_UPLOADED:
            _editics_abort(EditicsHttpStatus.InitialContentAlreadyUploaded, api_version)
        case EditicsUploadInitialContentBadOutcome.NO_PART_PROVIDED:
            _editics_abort(EditicsHttpStatus.BadContentTypeOrInvalidBody, api_version)


@editics_router.get(
    "/authenticated/{raw_organization_id}/editics/{raw_realm_id}/{raw_document_id}/initial_content"
)
async def editics_fetch_initial_content(
    raw_organization_id: str,
    raw_realm_id: str,
    raw_document_id: str,
    request: Request,
) -> Response:
    backend: Backend = request.app.state.backend

    api_version, auth_info, document_id = await _editics_auth_or_abort(
        backend, request, raw_organization_id, raw_realm_id, raw_document_id
    )

    outcome = await backend.editics.fetch_initial_content(auth_info, document_id)
    match outcome:
        case (key_index, stream_parts):
            # Stream the wire format:
            #   bundle = <key_index as u32 big endian> + <part 0> + <part 1> + ...
            #   part = <part_ciphertext size as u32 big endian> + <part_ciphertext>
            async def _stream_content():
                first = True
                async for part in stream_parts:
                    if first:
                        packet = pack("!I", key_index)
                        first = False
                    else:
                        packet = b""
                    packet += pack("!I", len(part)) + part
                    yield packet

            return StreamingResponse(
                content=_stream_content(),
                media_type=INITIAL_CONTENT_MEDIA_TYPE,
                headers={"Api-Version": str(api_version)},
            )
        case EditicsFetchInitialContentBadOutcome.NOT_UPLOADED:
            _editics_abort(EditicsHttpStatus.InitialContentMissing, api_version)


async def _editics_listen_auth(
    raw_organization_id: str,
    raw_realm_id: str,
    raw_document_id: str,
    psid: EditicsParticipantSessionID,
    request: Request,
) -> AsyncGenerator[EditicsParticipantSessionEventStream]:
    backend: Backend = request.app.state.backend

    if request.headers.get("Accept") != ACCEPT_TYPE_SSE:
        _editics_abort(EditicsHttpStatus.BadAcceptType, EDITICS_API_VERSION)

    api_version, auth_info, document_id = await _editics_auth_or_abort(
        backend, request, raw_organization_id, raw_realm_id, raw_document_id
    )

    with backend.editics.participant_listen(
        auth_info,
        document_id,
        psid,
    ) as outcome:
        match outcome:
            case MemoryObjectReceiveStream() as channel_receive:
                yield channel_receive

            case EditicsParticipantListenBadOutcome.PARTICIPANT_DEVICE_MISMATCH:
                _editics_abort(EditicsHttpStatus.ParticipantDeviceMismatch, api_version)


@editics_router.get(
    "/authenticated/{raw_organization_id}/editics/{raw_realm_id}/{raw_document_id}/listen",
    response_class=EventSourceResponse,
)
async def editics_listen(
    # Auth logic may raise an `HTTPException` which is not allowed from within the
    # event source generator. So it must be executed first in this separated context.
    channel_receive: Annotated[EditicsParticipantSessionEventStream, Depends(_editics_listen_auth)],
) -> AsyncIterable[EditicsServerEvent]:
    # Note we do not handle SSE's last-event-ID here, this is because OnlyOffice
    # protocol (on which the editics protocol is based) already has its own
    # re-connection logic
    async for event in channel_receive:
        yield event


@editics_router.post(
    "/authenticated/{raw_organization_id}/editics/{raw_realm_id}/{raw_document_id}/send"
)
async def editics_send(
    raw_organization_id: str,
    raw_realm_id: str,
    raw_document_id: str,
    psid: EditicsParticipantSessionID,
    request: Request,
) -> Response:
    backend: Backend = request.app.state.backend

    if request.headers.get("Content-Type") != "application/json":
        _editics_abort(EditicsHttpStatus.BadContentTypeOrInvalidBody, EDITICS_API_VERSION)

    api_version, auth_info, document_id = await _editics_auth_or_abort(
        backend, request, raw_organization_id, raw_realm_id, raw_document_id
    )

    raw_event = b""
    async for chunk in _editics_get_body_with_limit_check(
        request, backend.config.editics_max_part_size, api_version
    ):
        raw_event += chunk
    try:
        event = EditicsClientEventAdapter.validate_json(raw_event)
    except ValueError:
        _editics_abort(EditicsHttpStatus.BadContentTypeOrInvalidBody, api_version)
    del raw_event

    output = await backend.editics.participant_send(
        auth_info,
        document_id,
        psid,
        event,
    )
    match output:
        case BaseModel():
            return Response(
                content=output.model_dump_json(),
                media_type="application/json",
                headers={"Api-Version": str(api_version)},
            )
        case None:
            return Response(status_code=204, headers={"Api-Version": str(api_version)})
        case EditicsParticipantSendBadOutcome.PARTICIPANT_NOT_AUTH:
            _editics_abort(EditicsHttpStatus.ParticipantNotAuth, api_version)
        case EditicsParticipantSendBadOutcome.INITIAL_CONTENT_MISSING:
            _editics_abort(EditicsHttpStatus.InitialContentMissing, api_version)
        case EditicsParticipantSendBadOutcome.PARTICIPANT_DEVICE_MISMATCH:
            _editics_abort(EditicsHttpStatus.ParticipantDeviceMismatch, api_version)
        case EditicsParticipantSendBadOutcome.PARTICIPANT_MODE_MISMATCH:
            _editics_abort(EditicsHttpStatus.ParticipantModeMismatch, api_version)
        case EditicsParticipantSendBadOutcome.PARTICIPANT_SESSION_ID_MISMATCH:
            _editics_abort(EditicsHttpStatus.ParticipantIdMismatch, api_version)
        case EditicsParticipantSendBadOutcome.READER_MUST_BE_VIEWER:
            _editics_abort(EditicsHttpStatus.ReaderMustBeViewer, api_version)

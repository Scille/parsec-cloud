# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

"""
Editics routes works as a re-implementation of the OnlyOffice server, with a slightly modified
protocol (called the "editics protocol", by opposition of the original "OnlyOffic protocol")
to support end-to-end encryption.

OnlyOffice server uses websocket for communication with the client, however we
instead rely on SSE + HTTP POST (since websocket are sometime blocked by
enterprise proxies).

Editics ASGI routes:

    GET  /authenticated/{raw_organization_id}/editics/sessions/{realm_id}/{document_id}/listen
    POST /authenticated/{raw_organization_id}/editics/sessions/{realm_id}/{document_id}/send

    GET  /authenticated/{raw_organization_id}/editics/sessions/{realm_id}/{document_id}/initial_content
    POST /authenticated/{raw_organization_id}/editics/sessions/{realm_id}/{document_id}/initial_content

## Joining a session

Since an editics session lives within the memory of a server, all participants
must directly connect to it (this differs from the rest of Parsec server where
the PostgreSQL database is the centralized source of truth).

In practice this means a device needs to use the `editics_join_session`
authenticated API command (TODO: this command is not yet implemented) to obtain
the URL of the editics server to connect to.

## Initial content

The initial content is an archive containing the document converted in the
OnlyOffice format (i.e. {bin, medias}). It is the starting base over
which all the patches created during the edition session are to be applied.

The initial content is uploaded once (using a dedicated route) by the first
editor connecting.

Any viewer that connects prior to that is expected to obtain the document
its own way (i.e. get the initial document from the workspace, then convert
it with x2t), and will do a reload once once the initial content get uploaded
(see `parsecInitialContentUploaded` server event).

### format (Content-type `application/vnd.parsec.bundled-initial-content`)

bundle = <key_index as u32 big endian > + <part 0> + <part 1> + ...
part = <part_ciphertext size as u32 big endian> + <part_ciphertext>
part_ciphertext = encrypt_for_workspace(key_index, <part_cleartext> )
part_cleartext = 0x00 + zstd(msgpack([{'name': <media 0 name>, 'content': <media 0 content>}, ...]))

First part is always the bin (so that OnlyOffice editor can start loading it
without waiting for the full response), subsequent parts are medias files.

We use a custom format for the outer envelop (i.e. `bundle`):
- We need a simple format that supports streaming (to load the .bin in OnlyOffice editor asap)
- Multipart is more complicated and would requires a custom implementation in client
  (Javascript web API doesn't provide a multipart parser).
- Msgpack is only available in libparsec, but the editics requests are done in
  the client GUI (cannot pass a `ReadableStream` Javascript object to libparsec).

The inner envelop (i.e. `part_ciphertext`) uses the regular serialization format v0
(based on msgpack + zstd) used in Parsec (i.e. certificates, manifests etc.). This
is because the decryption must be done in libparsec so using this format is simple.

Maximal full document size is limited to 100Mo.

The medias are stored in multiple parts.
There is no size limit for a part (since a document can be < 100Mo with a single
very big image in it).
A part can contain a single or multiple media files (useful when there is plenty
of small media files)
Parts are for:
- Server: Parts are stored in PostgreSQL as-is (TODO: not implemented yet), hence
  they are useful a upload/fetch requests to stream the content (instead of having
  the full document on the server's memory).
- Client: OnlyOffice editor loads the first part (i.e. document .bin) without waiting
  for the full response body.
  Note part streaming is not used to reduce memory usage in the client since both
  OnlyOffice and x2t (used when saving) require to have the whole document in memory.
"""

from __future__ import annotations

from collections.abc import AsyncGenerator, AsyncIterable, Buffer, Sequence
from enum import Enum
from struct import pack, unpack_from
from typing import Annotated, NoReturn

from anyio.streams.memory import MemoryObjectReceiveStream
from fastapi import APIRouter, Depends, HTTPException, Request
from fastapi.responses import Response
from fastapi.sse import EventSourceResponse
from pydantic import BaseModel

from parsec._parsec import ApiVersion, DateTime, OrganizationID, VlobID
from parsec.asgi.rpc import IncompatibleAPIVersionsError, settle_compatible_versions
from parsec.backend import Backend
from parsec.components.auth import AuthEditicsAuthBadOutcome, EditicsAuthInfo, EditicsToken
from parsec.components.editics import (
    EditicsFetchInitialContentBadOutcome,
    EditicsParticipantListenBadOutcome,
    EditicsParticipantSendBadOutcome,
    EditicsSessionJoinEventStream,
    EditicsUploadInitialContentBadOutcome,
)
from parsec.editics_protocol import (
    EDITICS_API_VERSION,
    EditicsClientEventAdapter,
    EditicsParticipantID,
    EditicsServerEvent,
)

EDITICS_SUPPORTED_API_VERSIONS = (EDITICS_API_VERSION,)

editics_router = APIRouter(include_in_schema=False)

ACCEPT_TYPE_SSE = "text/event-stream"

INITIAL_CONTENT_MEDIA_TYPE = "application/vnd.parsec.bundled-initial-content"
INITIAL_CONTENT_MAX_SIZE = 100 * 1024**2


class EditicsHttpStatus(Enum):
    """
    HTTP statuses used by the editics routes.

    The common statuses use the same values as the RPC API's `CustomHttpStatus`
    (kept in sync for simplicity, though the two APIs are totally independent,
    hence 463/464 have here totally different meanings than for the RPC API).
    """

    # Basic request parsing & authentication errors

    MissingAuthenticationInfo = 401
    BadAuthenticationInfo = 403
    BadAcceptType = 406
    PayloadTooLarge = 413
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
    OriginalContentMissing = 485
    # The session already has its initial content (first-upload-wins): the client
    # that lost the concurrent upload race must fetch it instead of uploading
    # its own conversion.
    OriginalContentAlreadyUploaded = 486


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
        raise HTTPException(
            status_code=EditicsHttpStatus.UnsupportedApiVersion.value,
            headers={"Supported-Api-Versions": supported_api_versions},
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


# --- Initial content bundle ---------------------------------------------------


def pack_initial_content_bundle(key_index: int, parts: Sequence[Buffer]) -> bytes:
    """
    Build the wire format of the initial content bundle (see the module docstring):

        bundle = <key_index as u32 big endian> + <part 0> + <part 1> + ...
        part = <part_ciphertext size as u32 big endian> + <part_ciphertext>
    """
    return b"".join(
        [pack("!I", key_index), *(pack("!I", len(part)) + bytes(part) for part in parts)]
    )


def unpack_initial_content_bundle(raw: bytes) -> tuple[int, list[bytes]]:
    """
    Reverse of `pack_initial_content_bundle` (the parts' ciphertexts are opaque
    to the server, so only the outer envelope is parsed).

    Raise `ValueError` if the bundle is malformed.
    """
    if len(raw) < 4:
        raise ValueError("Bundle is missing the key index")
    (key_index,) = unpack_from("!I", raw, 0)
    parts: list[bytes] = []
    offset = 4
    while offset < len(raw):
        if len(raw) - offset < 4:
            raise ValueError("Bundle contains a truncated part size")
        (part_size,) = unpack_from("!I", raw, offset)
        offset += 4
        if len(raw) - offset < part_size:
            raise ValueError("Bundle contains a truncated part ciphertext")
        parts.append(raw[offset : offset + part_size])
        offset += part_size
    if not parts:
        raise ValueError("Bundle contains no part (the document bin is expected first)")
    return (key_index, parts)


# --- Routes ------------------------------------------------------------------


@editics_router.post(
    "/authenticated/{raw_organization_id}/editics/sessions/{raw_realm_id}/{raw_document_id}/initial_content"
)
async def editics_upload_initial_content(
    raw_organization_id: str,
    raw_realm_id: str,
    raw_document_id: str,
    participant_id: EditicsParticipantID,
    request: Request,
) -> Response:
    backend: Backend = request.app.state.backend

    api_version, auth_info, document_id = await _editics_auth_or_abort(
        backend, request, raw_organization_id, raw_realm_id, raw_document_id
    )

    if request.headers.get("Content-Type") != INITIAL_CONTENT_MEDIA_TYPE:
        _editics_abort(EditicsHttpStatus.BadContentTypeOrInvalidBody, api_version)

    # The bundle size is limited (see the module docstring): refuse the
    # oversized uploads early, before reading their whole body.
    try:
        content_length = int(request.headers["Content-Length"])
    except (ValueError, KeyError):
        # Header missing (i.e. chunked encoding) or invalid: the size will be
        # checked on the actual body below.
        content_length = INITIAL_CONTENT_MAX_SIZE
    else:
        if content_length > INITIAL_CONTENT_MAX_SIZE:
            _editics_abort(EditicsHttpStatus.PayloadTooLarge, api_version)

    raw_bundle = await request.body()
    if len(raw_bundle) > INITIAL_CONTENT_MAX_SIZE:
        _editics_abort(EditicsHttpStatus.PayloadTooLarge, api_version)

    try:
        key_index, parts = unpack_initial_content_bundle(raw_bundle)
    except ValueError:
        _editics_abort(EditicsHttpStatus.BadContentTypeOrInvalidBody, api_version)

    outcome = backend.editics.upload_initial_content(
        auth_info, document_id, participant_id, (key_index, parts)
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
            _editics_abort(EditicsHttpStatus.OriginalContentAlreadyUploaded, api_version)


@editics_router.get(
    "/authenticated/{raw_organization_id}/editics/sessions/{raw_realm_id}/{raw_document_id}/initial_content"
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

    outcome = backend.editics.fetch_initial_content(auth_info, document_id)
    match outcome:
        case (key_index, parts):
            return Response(
                content=pack_initial_content_bundle(key_index, parts),
                media_type=INITIAL_CONTENT_MEDIA_TYPE,
                headers={"Api-Version": str(api_version)},
            )
        case EditicsFetchInitialContentBadOutcome.NOT_UPLOADED:
            _editics_abort(EditicsHttpStatus.OriginalContentMissing, api_version)


async def _editics_listen_auth(
    raw_organization_id: str,
    raw_realm_id: str,
    raw_document_id: str,
    participant_id: EditicsParticipantID,
    request: Request,
) -> AsyncGenerator[EditicsSessionJoinEventStream]:
    backend: Backend = request.app.state.backend

    if request.headers.get("Accept") != ACCEPT_TYPE_SSE:
        _editics_abort(EditicsHttpStatus.BadAcceptType, EDITICS_API_VERSION)

    api_version, auth_info, document_id = await _editics_auth_or_abort(
        backend, request, raw_organization_id, raw_realm_id, raw_document_id
    )

    with backend.editics.participant_listen(
        auth_info,
        document_id,
        participant_id,
    ) as outcome:
        match outcome:
            case MemoryObjectReceiveStream() as channel_receive:
                yield channel_receive

            case EditicsParticipantListenBadOutcome.PARTICIPANT_DEVICE_MISMATCH:
                _editics_abort(EditicsHttpStatus.ParticipantDeviceMismatch, api_version)


@editics_router.get(
    "/authenticated/{raw_organization_id}/editics/sessions/{raw_realm_id}/{raw_document_id}/listen",
    response_class=EventSourceResponse,
)
async def editics_listen(
    # Auth logic may raise an `HTTPException` which is not allowed from within the
    # event source generator. So it must be executed first in this separated context.
    channel_receive: Annotated[EditicsSessionJoinEventStream, Depends(_editics_listen_auth)],
) -> AsyncIterable[EditicsServerEvent]:
    # Note we do not handle SSE's last-event-ID here, this is because OnlyOffice
    # protocol (on which the editics protocol is based) already has its own
    # re-connection logic
    async for event in channel_receive:
        yield event


@editics_router.post(
    "/authenticated/{raw_organization_id}/editics/sessions/{raw_realm_id}/{raw_document_id}/send"
)
async def editics_send(
    raw_organization_id: str,
    raw_realm_id: str,
    raw_document_id: str,
    participant_id: EditicsParticipantID,
    request: Request,
) -> Response:
    backend: Backend = request.app.state.backend

    if request.headers.get("Content-Type") != "application/json":
        _editics_abort(EditicsHttpStatus.BadContentTypeOrInvalidBody, EDITICS_API_VERSION)

    try:
        content_length = int(request.headers["Content-Length"])
    except (ValueError, KeyError):
        content_length = backend.config.editics_send_max_content_length
    else:
        if content_length > backend.config.editics_send_max_content_length:
            _editics_abort(EditicsHttpStatus.PayloadTooLarge, EDITICS_API_VERSION)

    api_version, auth_info, document_id = await _editics_auth_or_abort(
        backend, request, raw_organization_id, raw_realm_id, raw_document_id
    )

    try:
        event = EditicsClientEventAdapter.validate_json(await request.body())
    except ValueError:
        _editics_abort(EditicsHttpStatus.BadContentTypeOrInvalidBody, api_version)

    output = await backend.editics.participant_send(
        auth_info,
        document_id,
        participant_id,
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
        case EditicsParticipantSendBadOutcome.ORIGINAL_CONTENT_MISSING:
            _editics_abort(EditicsHttpStatus.OriginalContentMissing, api_version)
        case EditicsParticipantSendBadOutcome.PARTICIPANT_DEVICE_MISMATCH:
            _editics_abort(EditicsHttpStatus.ParticipantDeviceMismatch, api_version)
        case EditicsParticipantSendBadOutcome.PARTICIPANT_MODE_MISMATCH:
            _editics_abort(EditicsHttpStatus.ParticipantModeMismatch, api_version)
        case EditicsParticipantSendBadOutcome.PARTICIPANT_ID_MISMATCH:
            _editics_abort(EditicsHttpStatus.ParticipantIdMismatch, api_version)
        case EditicsParticipantSendBadOutcome.READER_MUST_BE_VIEWER:
            _editics_abort(EditicsHttpStatus.ReaderMustBeViewer, api_version)

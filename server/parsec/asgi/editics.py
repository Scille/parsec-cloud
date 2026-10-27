# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
"""
Editics routes works as a re-implementation of the OnlyOffice server, with a slightly modified
protocol (called the "editics protocol", by opposition of the original "OnlyOffic protocol")
to support end-to-end encryption.

OnlyOffice server uses websocket for communication with the client, however we
instead rely on SSE + HTTP POST (since websocket are sometime blocked by
enterprise proxies).

Editics ASGI routes:

    GET  /authenticated/{raw_organization_id}/editics/sessions/{realm_id}/{document_id}/listen/{participant_id}
    POST /authenticated/{raw_organization_id}/editics/sessions/{realm_id}/{document_id}/send/{participant_id}

The editics API is versioned: the client provides its version in the `Api-Version` request
header and the server replies with its own version (see `EDITICS_API_VERSION` in
`parsec/editics_protocol.py`). The HTTP error statuses are defined in `EditicsHttpStatus`.

- `GET .../listen` opens the SSE stream (server→client).
- `POST .../send` carries one client event (client→server).

Participant ID is an arbitrary UUID chosen client-side that represent the connection
session between the client and the server (see "Client connection lifecyle" below).

## Joining a session

Since an editics session lives within the memory of a server, all participant
must directly connect to it (this differs from the rest of Parsec server where
the PostgreSQL database is the centralized source of truth).

In practice this means a device needs to use the `editics_join_session`
authenticated API command (todo: this command is not yet implemented) to obtain
the URL of the editics server to connect to.

## Client connection lifecyle

OnlyOffice server has three layers regarding to client connect:
- The actual websocket connection
- session ID: ID shared between the client and the server to represent a
  reconnection-stable client edition session participation.
  In practice this ID is obtained when sending the first `auth` event (and correspond
  to the websocket connection ID), then provided in the `auth` event of the
  re-connections done by subsequent websocket connections.
  When a new websocket connects to server, any older websocket with the same
  session ID is considered a zombie and gets closed.
  Note the client sometime must change its session ID, typically if some changes
  has occurred while it was trying to reconnect.
- participant ID (i.e. <idOriginal> + <indexUser>): Similar to the session ID, but
  at a different layer. This ID is broadcasted to all the participants and is used
  to identify who holds a auth/save lock.

For simplicity in editics protocol, we merge together the session ID and participant ID:
instead we only have the concept of participant and the participant ID is chosen client-side
(this is needed since this info must be known to send a request to the server).


TODO: document the time granularity used by the server to generate timestamp for
document modifications (it seems OnlyOffice uses a second granularity for this...)
"""

from __future__ import annotations

from collections.abc import AsyncGenerator, AsyncIterable
from enum import Enum
from typing import Annotated, NoReturn

from anyio.streams.memory import MemoryObjectReceiveStream
from fastapi import APIRouter, Depends, HTTPException, Request
from fastapi.datastructures import Headers
from fastapi.responses import Response
from fastapi.sse import EventSourceResponse
from pydantic import BaseModel

from parsec._parsec import ApiVersion, DateTime, OrganizationID, VlobID
from parsec.asgi.rpc import IncompatibleAPIVersionsError, settle_compatible_versions
from parsec.backend import Backend
from parsec.components.auth import AuthEditicsAuthBadOutcome, EditicsAuthInfo, EditicsToken
from parsec.components.editics import (
    EditicsParticipantListenBadOutcome,
    EditicsParticipantSendBadOutcome,
)
from parsec.editics_protocol import (
    EDITICS_SUPPORTED_API_VERSIONS,
    EditicsClientEventAdapter,
    EditicsParticipantID,
    EditicsServerEvent,
)

editics_router = APIRouter(include_in_schema=False)

ACCEPT_TYPE_SSE = "text/event-stream"
# Max size for the RPC body. This must be big enough for the
# `parsecUploadOriginalContent` client event, which carries the base64-encoded
# encrypted original content archive (a converted document and its media can
# be tens of MB, and base64 adds ~33%; the fetch reply is an HTTP response, so
# it is not concerned, and the SSE stream is unaffected).
# TODO justify size (configurable with the client side ?)
MAX_CONTENT_LENGTH = 100 * 1024**2


class EditicsHttpStatus(Enum):
    """
    HTTP statuses used by the editics routes.

    The common statuses use the same values as the RPC API's `CustomHttpStatus`
    (kept in sync for simplicity, though the two APIs are totally independent,
    hence 463/464 have here totally different meanings than for the RPC API).
    """

    # Common statuses
    MissingAuthenticationInfo = 401
    BadAuthenticationInfo = 403
    # Information hiding: malformed organization/realm/document ID, missing
    # organization, missing realm and deleted realm are all indistinguishable
    # (a too precise outcome would leak the existence/deletion of a realm to
    # an organization member that has no access to it).
    NotFound = 404
    BadAcceptType = 406
    BadContentType = 415
    UnsupportedApiVersion = 422
    OrganizationExpired = 460
    UserRevoked = 461
    UserFrozen = 462
    TokenExpired = 498
    # Editics-specific statuses
    UserNoRealmRole = 463
    ParticipantNotAuth = 464
    ParticipantDeviceMismatch = 465
    ParticipantIdMismatch = 466
    ParticipantModeMismatch = 467
    ReaderMustBeViewer = 468
    OriginalContentMissing = 469


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


def _parse_api_version_or_abort(headers: Headers) -> ApiVersion:
    try:
        client_api_version = ApiVersion.from_str(headers.get("Api-Version", ""))
        settled_api_version, _ = settle_compatible_versions(
            EDITICS_SUPPORTED_API_VERSIONS, [client_api_version]
        )
    except (ValueError, IncompatibleAPIVersionsError):
        supported_api_versions = ";".join(str(v) for v in EDITICS_SUPPORTED_API_VERSIONS)
        raise HTTPException(
            status_code=EditicsHttpStatus.UnsupportedApiVersion.value,
            headers={"Supported-Api-Versions": supported_api_versions},
        )
    return settled_api_version


# The editics token is expected in the `Authorization: Bearer <editics_token>` header.
# Note the listen route (i.e. SSE) is typically consumed from a web browser through
# the `EventSource` API, which cannot set custom headers: hence the token is also
# accepted as an `authorization` query parameter.
# TODO: remove the `authorization` query parameter fallback once the client uses
# its own SSE implementation (i.e. no longer relies on `EventSource`)
def _parse_editics_auth_header(request: Request, api_version: ApiVersion) -> EditicsToken:
    raw_authorization = request.headers.get("Authorization")
    if raw_authorization is None:
        raw_authorization = request.query_params.get("authorization")
    if raw_authorization is None:
        _editics_abort(EditicsHttpStatus.MissingAuthenticationInfo, api_version)

    try:
        expected_bearer, raw_editics_token = raw_authorization.split()
        if expected_bearer.lower() != "bearer":
            raise ValueError
        editics_token = EditicsToken.from_raw(raw_editics_token.encode())
    except ValueError:
        _editics_abort(EditicsHttpStatus.MissingAuthenticationInfo, api_version)

    return editics_token


def _parse_organization_id(raw_organization_id: str, api_version: ApiVersion) -> OrganizationID:
    try:
        return OrganizationID(raw_organization_id)
    except ValueError:
        _editics_abort(EditicsHttpStatus.NotFound, api_version)


def _parse_vlob_id(raw: str, api_version: ApiVersion) -> VlobID:
    try:
        return VlobID.from_hex(raw)
    except ValueError:
        _editics_abort(EditicsHttpStatus.NotFound, api_version)


async def _editics_auth(
    backend: Backend,
    request: Request,
    raw_organization_id: str,
    raw_realm_id: str,
) -> tuple[ApiVersion, EditicsAuthInfo]:
    api_version = _parse_api_version_or_abort(request.headers)
    organization_id = _parse_organization_id(raw_organization_id, api_version)
    realm_id = _parse_vlob_id(raw_realm_id, api_version)
    token = _parse_editics_auth_header(request, api_version)

    outcome = await backend.auth.editics_auth(
        now=DateTime.now(),
        organization_id=organization_id,
        realm_id=realm_id,
        token=token,
    )
    match outcome:
        case EditicsAuthInfo() as auth_info:
            return api_version, auth_info

        case (
            AuthEditicsAuthBadOutcome.ORGANIZATION_NOT_FOUND
            | AuthEditicsAuthBadOutcome.REALM_NOT_FOUND
            | AuthEditicsAuthBadOutcome.REALM_DELETED
        ):
            # Information hiding: see `EditicsHttpStatus.NotFound`
            _editics_abort(EditicsHttpStatus.NotFound, api_version)

        case AuthEditicsAuthBadOutcome.ORGANIZATION_EXPIRED:
            _editics_abort(EditicsHttpStatus.OrganizationExpired, api_version)
        case AuthEditicsAuthBadOutcome.USER_REVOKED:
            _editics_abort(EditicsHttpStatus.UserRevoked, api_version)
        case AuthEditicsAuthBadOutcome.USER_FROZEN:
            _editics_abort(EditicsHttpStatus.UserFrozen, api_version)
        case AuthEditicsAuthBadOutcome.USER_NO_REALM_ROLE:
            _editics_abort(EditicsHttpStatus.UserNoRealmRole, api_version)
        case AuthEditicsAuthBadOutcome.TOKEN_OUT_OF_BALLPARK:
            _editics_abort(EditicsHttpStatus.TokenExpired, api_version)
        case AuthEditicsAuthBadOutcome.DEVICE_NOT_FOUND | AuthEditicsAuthBadOutcome.INVALID_TOKEN:
            _editics_abort(EditicsHttpStatus.BadAuthenticationInfo, api_version)


# --- Routes ------------------------------------------------------------------


async def _editics_listen_auth(
    raw_organization_id: str,
    raw_realm_id: str,
    raw_document_id: str,
    participant_id: EditicsParticipantID,
    request: Request,
) -> AsyncGenerator[MemoryObjectReceiveStream]:
    backend: Backend = request.app.state.backend

    api_version, auth_info = await _editics_auth(
        backend, request, raw_organization_id, raw_realm_id
    )

    if request.headers.get("Accept") != ACCEPT_TYPE_SSE:
        _editics_abort(EditicsHttpStatus.BadAcceptType, api_version)

    document_id = _parse_vlob_id(raw_document_id, api_version)

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
    "/authenticated/{raw_organization_id}/editics/sessions/{raw_realm_id}/{raw_document_id}/listen/{participant_id}",
    response_class=EventSourceResponse,
)
async def editics_listen(
    # Auth logic may raise an `HTTPException` which is not allowed from within the
    # event source generator. So it must be executed first in this separated context.
    channel_receive: Annotated[MemoryObjectReceiveStream, Depends(_editics_listen_auth)],
) -> AsyncIterable[EditicsServerEvent]:
    # Note we do not handle SSE's last-event-ID here, this is because OnlyOffice
    # protocol (on which the editics protocol is based) already has its own
    # re-connection logic
    async for event in channel_receive:
        yield event


@editics_router.post(
    "/authenticated/{raw_organization_id}/editics/sessions/{raw_realm_id}/{raw_document_id}/send/{participant_id}"
)
async def editics_send(
    raw_organization_id: str,
    raw_realm_id: str,
    raw_document_id: str,
    participant_id: EditicsParticipantID,
    request: Request,
) -> Response:
    backend: Backend = request.app.state.backend

    api_version, auth_info = await _editics_auth(
        backend, request, raw_organization_id, raw_realm_id
    )

    if request.headers.get("Content-Type") != "application/json":
        _editics_abort(EditicsHttpStatus.BadContentType, api_version)

    document_id = _parse_vlob_id(raw_document_id, api_version)

    try:
        content_length = int(request.headers["Content-Length"])
    except (ValueError, KeyError):
        content_length = MAX_CONTENT_LENGTH
    else:
        if content_length > MAX_CONTENT_LENGTH:
            raise HTTPException(status_code=413)

    try:
        event = EditicsClientEventAdapter.validate_json(await request.body())
    except ValueError:
        raise HTTPException(status_code=400, detail="Invalid client event")

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

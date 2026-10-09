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

from collections.abc import AsyncIterable

from anyio.streams.memory import MemoryObjectReceiveStream
from fastapi import APIRouter, HTTPException, Request
from fastapi.responses import Response
from fastapi.sse import EventSourceResponse
from pydantic import BaseModel

from parsec._parsec import DateTime, OrganizationID, VlobID
from parsec.asgi.rpc import CustomHttpStatus, _handshake_abort
from parsec.backend import Backend
from parsec.components.auth import AuthEditicsAuthBadOutcome, EditicsAuthInfo, EditicsToken
from parsec.components.editics import (
    EditicsParticipantListenBadOutcome,
    EditicsParticipantSendBadOutcome,
)
from parsec.editics_protocol import (
    EditicsClientEventAdapter,
    EditicsParticipantID,
    EditicsServerEvent,
)

# TODO: define our own API versioning (and store it in `editics_protocol.py`)
# TODO: define our own HTTP error codes here

editics_router = APIRouter(include_in_schema=False)

ACCEPT_TYPE_SSE = "text/event-stream"
# TODO justify size (configurable with the client side ?)
# Max size for the RPC body
MAX_CONTENT_LENGTH = 1 * 1024**2


# Dummy auth system based on a `Authorization: Editics <device_id_hex>.<participant_uuid_hex>` header
# TODO: replace this by a proper thing
def _parse_editics_auth_header(headers, request: Request) -> EditicsToken:
    try:
        raw_authorization = headers["Authorization"]
    except KeyError:
        # TODO: handle api version
        _handshake_abort(CustomHttpStatus.MissingAuthenticationInfo, api_version="TODO")

    try:
        expected_bearer, raw_editics_token = raw_authorization.split()
        if expected_bearer.lower() != "bearer":
            raise ValueError
        editics_token = EditicsToken.from_raw(raw_editics_token.encode())
    except ValueError:
        # TODO: handle api version
        _handshake_abort(CustomHttpStatus.MissingAuthenticationInfo, api_version="TODO")

    return editics_token


def _parse_organization_id(raw_organization_id: str) -> OrganizationID:
    try:
        return OrganizationID(raw_organization_id)
    except ValueError:
        raise HTTPException(status_code=404, detail="Organization not found")


def _parse_vlob_id(raw: str) -> VlobID:
    try:
        return VlobID.from_hex(raw)
    except ValueError:
        raise HTTPException(status_code=404, detail="Bad vlob id")


# --- Routes ------------------------------------------------------------------


@editics_router.get(
    "/authenticated/{raw_organization_id}/editics/sessions/{raw_realm_id}/{raw_document_id}/listen/{participant_id}",
    response_class=EventSourceResponse,
)
async def editics_listen(
    raw_organization_id: str,
    raw_realm_id: str,
    raw_document_id: str,
    participant_id: EditicsParticipantID,
    request: Request,
) -> AsyncIterable[EditicsServerEvent]:
    # Note we do not handle SSE's last-event-ID here, this is because OnlyOffice
    # protocol (on which the editics protocol is based) already has its own
    # re-connection logic

    backend: Backend = request.app.state.backend

    # TODO: add test to check request missing the `Accept: text/event-stream` header
    # TODO: add tests to check request with incorrect organization/realm/document ID

    organization_id = _parse_organization_id(raw_organization_id)
    realm_id = _parse_vlob_id(raw_realm_id)
    document_id = _parse_vlob_id(raw_document_id)
    token = _parse_editics_auth_header(request.headers, request)

    outcome = await backend.auth.editics_auth(
        now=DateTime.now(),
        organization_id=organization_id,
        realm_id=realm_id,
        token=token,
    )
    match outcome:
        case EditicsAuthInfo() as auth_info:
            pass
        case AuthEditicsAuthBadOutcome.ORGANIZATION_EXPIRED:
            _handshake_abort(CustomHttpStatus.OrganizationExpired, api_version="TODO")
        case AuthEditicsAuthBadOutcome.ORGANIZATION_NOT_FOUND:
            _handshake_abort(
                CustomHttpStatus.OrganizationNotFound,
                api_version="TODO",
            )
        case AuthEditicsAuthBadOutcome.TOKEN_OUT_OF_BALLPARK:
            _handshake_abort(
                CustomHttpStatus.TokenExpired,
                api_version="TODO",
            )
        case AuthEditicsAuthBadOutcome.DEVICE_NOT_FOUND | AuthEditicsAuthBadOutcome.INVALID_TOKEN:
            _handshake_abort(
                CustomHttpStatus.BadAuthenticationInfo,
                api_version="TODO",
            )
        case AuthEditicsAuthBadOutcome.USER_REVOKED:
            _handshake_abort(
                CustomHttpStatus.UserRevoked,
                api_version="TODO",
            )
        case AuthEditicsAuthBadOutcome.USER_FROZEN:
            _handshake_abort(
                CustomHttpStatus.UserFrozen,
                api_version="TODO",
            )
        case AuthEditicsAuthBadOutcome.REALM_NOT_FOUND:
            _handshake_abort(
                CustomHttpStatus.UserMustAcceptTos,
                api_version="TODO",
            )
        case AuthEditicsAuthBadOutcome.REALM_DELETED:
            _handshake_abort(
                CustomHttpStatus.UserMustAcceptTos,
                api_version="TODO",
            )
        case AuthEditicsAuthBadOutcome.USER_NO_REALM_ROLE:
            _handshake_abort(
                CustomHttpStatus.UserMustAcceptTos,
                api_version="TODO",
            )

    with backend.editics.participant_listen(
        auth_info,
        document_id,
        participant_id,
    ) as outcome:
        match outcome:
            case MemoryObjectReceiveStream() as channel_receive:
                async for event in channel_receive:
                    yield event
            case EditicsParticipantListenBadOutcome.PARTICIPANT_DEVICE_MISMATCH:
                raise NotImplementedError  # TODO


@editics_router.post(
    "/authenticated/{raw_organization_id}/editics/sessions/{raw_realm_id}/{raw_document_id}/send/{participant_id}"
)
async def editics_send(
    raw_organization_id: str,
    raw_realm_id: str,
    raw_document_id: str,
    participant_id: EditicsParticipantID,
    request: Request,
):
    backend: Backend = request.app.state.backend

    organization_id = _parse_organization_id(raw_organization_id)
    realm_id = _parse_vlob_id(raw_realm_id)
    document_id = _parse_vlob_id(raw_document_id)
    token = _parse_editics_auth_header(request.headers, request)

    outcome = await backend.auth.editics_auth(
        now=DateTime.now(),
        organization_id=organization_id,
        realm_id=realm_id,
        token=token,
    )
    match outcome:
        case EditicsAuthInfo() as auth_info:
            pass
        case AuthEditicsAuthBadOutcome.ORGANIZATION_EXPIRED:
            _handshake_abort(CustomHttpStatus.OrganizationExpired, api_version="TODO")
        case AuthEditicsAuthBadOutcome.ORGANIZATION_NOT_FOUND:
            _handshake_abort(
                CustomHttpStatus.OrganizationNotFound,
                api_version="TODO",
            )
        case AuthEditicsAuthBadOutcome.TOKEN_OUT_OF_BALLPARK:
            _handshake_abort(
                CustomHttpStatus.TokenExpired,
                api_version="TODO",
            )
        case AuthEditicsAuthBadOutcome.DEVICE_NOT_FOUND | AuthEditicsAuthBadOutcome.INVALID_TOKEN:
            _handshake_abort(
                CustomHttpStatus.BadAuthenticationInfo,
                api_version="TODO",
            )
        case AuthEditicsAuthBadOutcome.USER_REVOKED:
            _handshake_abort(
                CustomHttpStatus.UserRevoked,
                api_version="TODO",
            )
        case AuthEditicsAuthBadOutcome.USER_FROZEN:
            _handshake_abort(
                CustomHttpStatus.UserFrozen,
                api_version="TODO",
            )
        case AuthEditicsAuthBadOutcome.REALM_NOT_FOUND:
            _handshake_abort(
                CustomHttpStatus.UserMustAcceptTos,
                api_version="TODO",
            )
        case AuthEditicsAuthBadOutcome.REALM_DELETED:
            _handshake_abort(
                CustomHttpStatus.UserMustAcceptTos,
                api_version="TODO",
            )
        case AuthEditicsAuthBadOutcome.USER_NO_REALM_ROLE:
            _handshake_abort(
                CustomHttpStatus.UserMustAcceptTos,
                api_version="TODO",
            )

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
            return Response(content=output.model_dump_json(), media_type="application/json")
        case None:
            return Response(status_code=204)
        case EditicsParticipantSendBadOutcome.PARTICIPANT_NOT_AUTH:
            raise NotImplementedError  # TODO
        case EditicsParticipantSendBadOutcome.PARTICIPANT_DEVICE_MISMATCH:
            raise NotImplementedError  # TODO

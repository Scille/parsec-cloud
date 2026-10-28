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

## Joining a session

Since an editics session lives within the memory of a server, all participants
must directly connect to it (this differs from the rest of Parsec server where
the PostgreSQL database is the centralized source of truth).

In practice this means a device needs to use the `editics_join_session`
authenticated API command (TODO: this command is not yet implemented) to obtain
the URL of the editics server to connect to.
"""

from __future__ import annotations

from collections.abc import AsyncGenerator, AsyncIterable
from enum import Enum
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
    # Once `auth` server event received, the first editor knows he must upload the
    # original content before anything else.
    OriginalContentMissing = 485


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


# --- Routes ------------------------------------------------------------------


@editics_router.get(
    "/authenticated/{raw_organization_id}/editics/sessions/{raw_realm_id}/{raw_document_id}/{participant_id}/initial_content"
)
async def editics_fetch_initial_content(
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

    outcome = backend.editics.fetch_initial_content(auth_info, document_id)
    match outcome:
        case (key_index, encrypted_initial_content):
            return Response(
                content=encrypted_initial_content,
                media_type="application/json",
                headers={"Api-Version": str(api_version)},
            )
        case EditicsFetchInitialContentBadOutcome.NOT_UPLOADED:
            _editics_abort(EditicsHttpStatus.ParticipantNotAuth, api_version)


async def _editics_listen_auth(
    raw_organization_id: str,
    raw_realm_id: str,
    raw_document_id: str,
    participant_id: EditicsParticipantID,
    request: Request,
) -> AsyncGenerator[MemoryObjectReceiveStream]:
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

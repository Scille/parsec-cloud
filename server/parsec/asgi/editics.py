# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
"""
Editics ASGI routes:

    GET  /authenticated/{raw_organization_id}/editics/sessions/{realm_id}/{document_id}/join
    POST /authenticated/{raw_organization_id}/editics/sessions/{realm_id}/{document_id}/send

- `GET .../join` opens the SSE stream (server→client).
- `POST .../send` carries one client event (client→server).
"""

from __future__ import annotations

from collections.abc import AsyncIterable
from typing import Annotated
from uuid import UUID

from fastapi import APIRouter, Header, HTTPException, Request
from fastapi.responses import Response
from fastapi.sse import EventSourceResponse
from pydantic import BaseModel

from parsec._parsec import DeviceID, OrganizationID, VlobID
from parsec.backend import Backend
from parsec.components.editics import EditicsJoinSessionBadOutcome, EditicsSendInSessionBadOutcome
from parsec.editics_protocol import EditicsProtocolClientEventAdapter, EditicsProtocolServerEvent

# TODO: define our own API versioning (and store it in `editics_protocol.py`)
# TODO: define our own HTTP error codes here

editics_router = APIRouter(include_in_schema=False)

ACCEPT_TYPE_SSE = "text/event-stream"
# TODO justify size (configurable with the client side ?)
# Max size for the RPC body
MAX_CONTENT_LENGTH = 1 * 1024**2


# Dummy auth system based on a `Authorization: Editics <device_id_hex>.<participant_uuid_hex>` header
# TODO: replace this by a proper thing
def _parse_editics_auth_header(headers, request: Request) -> tuple[DeviceID, UUID]:
    raw = headers.get("Authorization")
    if not raw:
        raw = request.query_params.get("authorization")
    if not raw:
        raise HTTPException(status_code=401, detail="Missing Editics authorization")
    expected_scheme, _, rest = raw.partition(" ")
    if expected_scheme != "Editics" or not rest:
        raise HTTPException(status_code=401, detail="Missing Editics authorization")
    device_hex, _, participant_hex = rest.partition(".")
    if not participant_hex:
        raise HTTPException(status_code=401, detail="Missing Editics authorization")
    try:
        device_id = DeviceID.from_hex(device_hex)
    except ValueError:
        raise HTTPException(status_code=401, detail="Bad Editics authorization")
    try:
        participant_uuid = UUID(hex=participant_hex)
    except ValueError:
        raise HTTPException(status_code=401, detail="Bad Editics authorization")
    return (device_id, participant_uuid)


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
    "/authenticated/{raw_organization_id}/editics/sessions/{raw_realm_id}/{raw_document_id}/join",
    response_class=EventSourceResponse,
)
async def editics_join(
    raw_organization_id: str,
    raw_realm_id: str,
    raw_document_id: str,
    request: Request,
    last_event_id: Annotated[int | None, Header()] = None,
) -> AsyncIterable[EditicsProtocolServerEvent]:
    backend: Backend = request.app.state.backend

    # TODO: handle last_event_id

    # TODO: add test to check request missing the `Accept: text/event-stream` header
    # TODO: add tests to check request with incorrect organization/realm/document ID

    organization_id = _parse_organization_id(raw_organization_id)
    realm_id = _parse_vlob_id(raw_realm_id)
    document_id = _parse_vlob_id(raw_document_id)
    device_id, participant_id = _parse_editics_auth_header(request.headers, request)

    async with backend.editics.sse_api_join_session(
        organization_id,
        device_id,
        participant_id,
        realm_id,
        document_id,
        last_event_id=last_event_id,
    ) as outcome:
        match outcome:
            case list() as events:
                for event in events:
                    yield event
            case EditicsJoinSessionBadOutcome.ORGANIZATION_NOT_FOUND:
                raise NotImplementedError  # TODO
            case EditicsJoinSessionBadOutcome.ORGANIZATION_EXPIRED:
                raise NotImplementedError  # TODO
            case EditicsJoinSessionBadOutcome.AUTHOR_NOT_FOUND:
                raise NotImplementedError  # TODO
            case EditicsJoinSessionBadOutcome.AUTHOR_REVOKED:
                raise NotImplementedError  # TODO
            case EditicsJoinSessionBadOutcome.STOPPED:
                raise NotImplementedError  # TODO


@editics_router.post(
    "/authenticated/{raw_organization_id}/editics/sessions/{raw_realm_id}/{raw_document_id}/send"
)
async def editics_send(
    raw_organization_id: str, raw_realm_id: str, raw_document_id: str, request: Request
):
    backend: Backend = request.app.state.backend

    organization_id = _parse_organization_id(raw_organization_id)
    realm_id = _parse_vlob_id(raw_realm_id)
    document_id = _parse_vlob_id(raw_document_id)
    device_id, participant_id = _parse_editics_auth_header(request.headers, request)

    try:
        content_length = int(request.headers["Content-Length"])
    except (ValueError, KeyError):
        content_length = MAX_CONTENT_LENGTH
    else:
        if content_length > MAX_CONTENT_LENGTH:
            raise HTTPException(status_code=413)

    try:
        event = EditicsProtocolClientEventAdapter.validate_json(await request.body())
    except ValueError:
        raise HTTPException(status_code=400, detail="Invalid client event")

    reply = await backend.editics.api_send_in_session(
        organization_id,
        device_id,
        participant_id,
        realm_id,
        document_id,
        event,
    )
    match reply:
        case BaseModel():
            return Response(content=reply.model_dump_json(), media_type="application/json")
        case None:
            return Response(status_code=204)
        case EditicsSendInSessionBadOutcome.ORGANIZATION_NOT_FOUND:
            raise NotImplementedError  # TODO
        case EditicsSendInSessionBadOutcome.ORGANIZATION_EXPIRED:
            raise NotImplementedError  # TODO
        case EditicsSendInSessionBadOutcome.AUTHOR_NOT_FOUND:
            raise NotImplementedError  # TODO
        case EditicsSendInSessionBadOutcome.AUTHOR_REVOKED:
            raise NotImplementedError  # TODO
        case EditicsSendInSessionBadOutcome.STOPPED:
            raise NotImplementedError  # TODO

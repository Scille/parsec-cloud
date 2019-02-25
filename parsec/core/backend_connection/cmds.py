# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Tuple, List, Dict, Iterable, Optional
from uuid import UUID
import pendulum

from parsec.types import DeviceID, UserID, DeviceName, OrganizationID
from parsec.crypto import VerifyKey
from parsec.api.transport import Transport, TransportError
from parsec.api.protocole import (
    ProtocoleError,
    ping_serializer,
    organization_create_serializer,
    organization_bootstrap_serializer,
    events_subscribe_serializer,
    events_listen_serializer,
    beacon_read_serializer,
    message_send_serializer,
    message_get_serializer,
    vlob_group_check_serializer,
    vlob_read_serializer,
    vlob_create_serializer,
    vlob_update_serializer,
    blockstore_create_serializer,
    blockstore_read_serializer,
    user_get_serializer,
    user_find_serializer,
    user_invite_serializer,
    user_get_invitation_creator_serializer,
    user_claim_serializer,
    user_cancel_invitation_serializer,
    user_create_serializer,
    device_invite_serializer,
    device_get_invitation_creator_serializer,
    device_claim_serializer,
    device_cancel_invitation_serializer,
    device_create_serializer,
    device_revoke_serializer,
)
from parsec.core.types import RemoteDevice, RemoteUser, RemoteDevicesMapping
from parsec.core.backend_connection.exceptions import (
    BackendNotAvailable,
    BackendCmdsInvalidRequest,
    BackendCmdsInvalidResponse,
    BackendCmdsBadResponse,
)


async def _send_cmd(transport, serializer, **req):
    transport.logger.info("Request", cmd=req["cmd"])

    def _shorten_data(data):
        if len(req) > 300:
            return data[:150] + b"[...]" + data[-150:]
        else:
            return data

    try:
        raw_req = serializer.req_dumps(req)

    except ProtocoleError as exc:
        raise BackendCmdsInvalidRequest() from exc

    try:
        await transport.send(raw_req)
        raw_rep = await transport.recv()

    except TransportError as exc:
        transport.logger.info("Request failed (backend not available)", cmd=req["cmd"])
        raise BackendNotAvailable(exc) from exc

    try:
        rep = serializer.rep_loads(raw_rep)

    except ProtocoleError as exc:
        transport.logger.warning("Request failed (bad protocol)", cmd=req["cmd"], error=exc)
        raise BackendCmdsInvalidResponse(exc) from exc

    if rep["status"] == "invalid_msg_format":
        raise BackendCmdsInvalidRequest(rep)

    return rep


###  Backend authenticated cmds  ###


### Events&misc API ###


async def ping(transport: Transport, ping: str) -> str:
    rep = await _send_cmd(transport, ping_serializer, cmd="ping", ping=ping)
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)
    return rep["pong"]


async def events_subscribe(
    transport: Transport,
    message_received: bool = False,
    beacon_updated: Iterable[UUID] = (),
    pinged: Iterable[str] = (),
) -> None:
    rep = await _send_cmd(
        transport,
        events_subscribe_serializer,
        cmd="events_subscribe",
        message_received=message_received,
        beacon_updated=beacon_updated,
        pinged=pinged,
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)


async def events_listen(transport: Transport, wait: bool = True) -> dict:
    rep = await _send_cmd(transport, events_listen_serializer, cmd="events_listen", wait=wait)
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)
    rep.pop("status")
    return rep


# Beacon


async def beacon_read(transport: Transport, id: UUID, offset: int) -> List[Tuple[UUID, int]]:
    rep = await _send_cmd(
        transport, beacon_read_serializer, cmd="beacon_read", id=id, offset=offset
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)
    return [(item["src_id"], item["src_version"]) for item in rep["items"]]


# Message


async def message_send(transport: Transport, recipient: UserID, body: bytes) -> None:
    rep = await _send_cmd(
        transport, message_send_serializer, cmd="message_send", recipient=recipient, body=body
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)


async def message_get(transport: Transport, offset: int) -> List[Tuple[int, DeviceID, bytes]]:
    rep = await _send_cmd(transport, message_get_serializer, cmd="message_get", offset=offset)
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)
    return [(item["count"], item["sender"], item["body"]) for item in rep["messages"]]


### Vlob API ###


async def vlob_group_check(transport: Transport, to_check: list) -> list:
    rep = await _send_cmd(
        transport, vlob_group_check_serializer, cmd="vlob_group_check", to_check=to_check
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)
    return rep["changed"]


async def vlob_create(
    transport: Transport, id: UUID, rts: str, wts: str, blob: bytes, notify_beacon=UUID
) -> None:
    rep = await _send_cmd(
        transport,
        vlob_create_serializer,
        cmd="vlob_create",
        id=id,
        rts=rts,
        wts=wts,
        blob=blob,
        notify_beacon=notify_beacon,
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)


async def vlob_read(
    transport: Transport, id: UUID, rts: str, version: int = None
) -> Tuple[int, bytes]:
    rep = await _send_cmd(
        transport, vlob_read_serializer, cmd="vlob_read", id=id, rts=rts, version=version
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)
    return rep["version"], rep["blob"]


async def vlob_update(
    transport: Transport, id: UUID, wts: str, version: int, blob: bytes, notify_beacon: UUID
) -> None:
    rep = await _send_cmd(
        transport,
        vlob_update_serializer,
        cmd="vlob_update",
        id=id,
        version=version,
        wts=wts,
        blob=blob,
        notify_beacon=notify_beacon,
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)


### Blockstore API ###


async def blockstore_create(transport: Transport, id: UUID, block: bytes) -> None:
    rep = await _send_cmd(
        transport, blockstore_create_serializer, cmd="blockstore_create", id=id, block=block
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)


async def blockstore_read(transport: Transport, id: UUID) -> bytes:
    rep = await _send_cmd(transport, blockstore_read_serializer, cmd="blockstore_read", id=id)
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)
    return rep["block"]


### User API ###


async def user_get(
    transport: Transport, user_id: UserID
) -> Tuple[RemoteUser, Dict[DeviceID, RemoteDevice]]:
    rep = await _send_cmd(transport, user_get_serializer, cmd="user_get", user_id=user_id)
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)

    devices = []
    for rep_device in rep["devices"].values():
        devices.append(
            RemoteDevice(
                device_id=rep_device["device_id"],
                certified_device=rep_device["certified_device"],
                device_certifier=rep_device["device_certifier"],
                created_on=rep_device["created_on"],
                revocated_on=rep_device["revocated_on"],
                certified_revocation=rep_device["certified_revocation"],
                revocation_certifier=rep_device["revocation_certifier"],
            )
        )
    user = RemoteUser(
        user_id=rep["user_id"],
        certified_user=rep["certified_user"],
        user_certifier=rep["user_certifier"],
        devices=RemoteDevicesMapping(*devices),
        created_on=rep["created_on"],
    )
    trustchain = {
        k: RemoteDevice(
            device_id=v["device_id"],
            certified_device=v["certified_device"],
            device_certifier=v["device_certifier"],
            created_on=v["created_on"],
            revocated_on=v["revocated_on"],
            certified_revocation=v["certified_revocation"],
            revocation_certifier=v["revocation_certifier"],
        )
        for k, v in rep["trustchain"].items()
    }
    return (user, trustchain)


async def user_find(
    transport: Transport,
    query: str = None,
    page: int = 1,
    per_page: int = 100,
    omit_revocated: bool = False,
) -> List[UserID]:
    rep = await _send_cmd(
        transport,
        user_find_serializer,
        cmd="user_find",
        query=query,
        page=page,
        per_page=per_page,
        omit_revocated=omit_revocated,
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)
    return rep["results"]


async def user_invite(transport: Transport, user_id: UserID) -> bytes:
    rep = await _send_cmd(transport, user_invite_serializer, cmd="user_invite", user_id=user_id)
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)
    return rep["encrypted_claim"]


async def user_cancel_invitation(transport: Transport, user_id: UserID) -> None:
    rep = await _send_cmd(
        transport, user_cancel_invitation_serializer, cmd="user_cancel_invitation", user_id=user_id
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)


async def user_create(
    transport: Transport, certified_user: bytes, certified_device: bytes, is_admin: bool
) -> None:
    rep = await _send_cmd(
        transport,
        user_create_serializer,
        cmd="user_create",
        certified_user=certified_user,
        certified_device=certified_device,
        is_admin=is_admin,
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)


async def device_invite(transport: Transport, invited_device_name: DeviceName) -> bytes:
    rep = await _send_cmd(
        transport,
        device_invite_serializer,
        cmd="device_invite",
        invited_device_name=invited_device_name,
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)
    return rep["encrypted_claim"]


async def device_cancel_invitation(transport: Transport, invited_device_name: DeviceName) -> None:
    rep = await _send_cmd(
        transport,
        device_cancel_invitation_serializer,
        cmd="device_cancel_invitation",
        invited_device_name=invited_device_name,
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)


async def device_create(
    transport: Transport, certified_device: bytes, encrypted_answer: bytes
) -> None:
    rep = await _send_cmd(
        transport,
        device_create_serializer,
        cmd="device_create",
        certified_device=certified_device,
        encrypted_answer=encrypted_answer,
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)


async def device_revoke(
    transport: Transport, certified_revocation: bytes
) -> Optional[pendulum.Pendulum]:
    rep = await _send_cmd(
        transport,
        device_revoke_serializer,
        cmd="device_revoke",
        certified_revocation=certified_revocation,
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)
    return rep["user_revocated_on"]


###  Backend anonymous cmds  ###


# ping already defined in authenticated part


async def organization_create(transport: Transport, organization_id: OrganizationID) -> str:
    rep = await _send_cmd(
        transport,
        organization_create_serializer,
        cmd="organization_create",
        organization_id=organization_id,
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)
    return rep["bootstrap_token"]


async def organization_bootstrap(
    transport: Transport,
    organization_id: OrganizationID,
    bootstrap_token: str,
    root_verify_key: VerifyKey,
    certified_user: bytes,
    certified_device: bytes,
) -> None:
    rep = await _send_cmd(
        transport,
        organization_bootstrap_serializer,
        cmd="organization_bootstrap",
        organization_id=organization_id,
        bootstrap_token=bootstrap_token,
        root_verify_key=root_verify_key,
        certified_user=certified_user,
        certified_device=certified_device,
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)


async def user_get_invitation_creator(transport: Transport, invited_user_id: UserID) -> RemoteUser:
    rep = await _send_cmd(
        transport,
        user_get_invitation_creator_serializer,
        cmd="user_get_invitation_creator",
        invited_user_id=invited_user_id,
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)
    return RemoteUser(
        user_id=rep["user_id"],
        created_on=rep["created_on"],
        certified_user=rep["certified_user"],
        user_certifier=rep["user_certifier"],
    )


async def user_claim(transport: Transport, invited_user_id: UserID, encrypted_claim: bytes) -> None:
    rep = await _send_cmd(
        transport,
        user_claim_serializer,
        cmd="user_claim",
        invited_user_id=invited_user_id,
        encrypted_claim=encrypted_claim,
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)


async def device_get_invitation_creator(
    transport: Transport, invited_device_id: DeviceID
) -> RemoteUser:
    rep = await _send_cmd(
        transport,
        device_get_invitation_creator_serializer,
        cmd="device_get_invitation_creator",
        invited_device_id=invited_device_id,
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)
    return RemoteUser(
        user_id=rep["user_id"],
        created_on=rep["created_on"],
        certified_user=rep["certified_user"],
        user_certifier=rep["user_certifier"],
    )


async def device_claim(
    transport: Transport, invited_device_id: DeviceID, encrypted_claim: bytes
) -> bytes:
    rep = await _send_cmd(
        transport,
        device_claim_serializer,
        cmd="device_claim",
        invited_device_id=invited_device_id,
        encrypted_claim=encrypted_claim,
    )
    if rep["status"] != "ok":
        raise BackendCmdsBadResponse(rep)
    return rep["encrypted_answer"]

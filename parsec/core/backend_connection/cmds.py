# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Tuple, List, Dict, Optional
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
    message_send_serializer,
    message_get_serializer,
    RealmRole,
    vlob_read_serializer,
    vlob_create_serializer,
    vlob_update_serializer,
    vlob_group_check_serializer,
    vlob_poll_changes_serializer,
    vlob_maintenance_get_reencryption_batch_serializer,
    vlob_maintenance_save_reencryption_batch_serializer,
    realm_status_serializer,
    realm_get_roles_serializer,
    realm_update_roles_serializer,
    realm_start_reencryption_maintenance_serializer,
    realm_finish_reencryption_maintenance_serializer,
    block_create_serializer,
    block_read_serializer,
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
from parsec.core.types import UnverifiedRemoteUser, UnverifiedRemoteDevice, EntryID
from parsec.core.backend_connection.exceptions import (
    raise_on_bad_response,
    BackendNotAvailable,
    BackendCmdsInvalidRequest,
    BackendCmdsInvalidResponse,
)


async def _send_cmd(transport, serializer, keepalive=False, **req):
    """
    Raises:
        BackendCmdsInvalidRequest
        BackendCmdsInvalidResponse
        BackendNotAvailable
        BackendCmdsBadResponse
    """
    transport.logger.info("Request", cmd=req["cmd"])

    def _shorten_data(data):
        if len(req) > 300:
            return data[:150] + b"[...]" + data[-150:]
        else:
            return data

    try:
        raw_req = serializer.req_dumps(req)

    except ProtocoleError as exc:
        raise BackendCmdsInvalidRequest(exc) from exc

    try:
        await transport.send(raw_req)
        raw_rep = await transport.recv(keepalive)

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

    raise_on_bad_response(rep)

    return rep


###  Backend authenticated cmds  ###


### Events&misc API ###


async def ping(transport: Transport, ping: str) -> str:
    rep = await _send_cmd(transport, ping_serializer, cmd="ping", ping=ping)
    return rep["pong"]


async def events_subscribe(transport: Transport,) -> None:
    await _send_cmd(transport, events_subscribe_serializer, cmd="events_subscribe")


async def events_listen(transport: Transport, wait: bool = True) -> dict:
    rep = await _send_cmd(
        transport, events_listen_serializer, keepalive=wait, cmd="events_listen", wait=wait
    )
    rep.pop("status")
    return rep


### Message API ###


async def message_send(
    transport: Transport, recipient: UserID, timestamp: pendulum.Pendulum, body: bytes
) -> None:
    await _send_cmd(
        transport,
        message_send_serializer,
        cmd="message_send",
        recipient=recipient,
        timestamp=timestamp,
        body=body,
    )


async def message_get(transport: Transport, offset: int) -> List[Tuple[int, DeviceID, bytes]]:
    rep = await _send_cmd(transport, message_get_serializer, cmd="message_get", offset=offset)
    return [
        (item["count"], item["sender"], item["timestamp"], item["body"]) for item in rep["messages"]
    ]


### Vlob API ###


async def vlob_create(
    transport: Transport,
    realm_id: UUID,
    encryption_revision: int,
    vlob_id: UUID,
    timestamp: pendulum.Pendulum,
    blob: bytes,
) -> None:
    await _send_cmd(
        transport,
        vlob_create_serializer,
        cmd="vlob_create",
        realm_id=realm_id,
        encryption_revision=encryption_revision,
        vlob_id=vlob_id,
        timestamp=timestamp,
        blob=blob,
    )


async def vlob_read(
    transport: Transport, encryption_revision: int, vlob_id: UUID, version: int = None
) -> Tuple[DeviceID, pendulum.Pendulum, int, bytes]:
    rep = await _send_cmd(
        transport,
        vlob_read_serializer,
        cmd="vlob_read",
        encryption_revision=encryption_revision,
        vlob_id=vlob_id,
        version=version,
    )
    return rep["author"], rep["timestamp"], rep["version"], rep["blob"]


async def vlob_update(
    transport: Transport,
    encryption_revision: int,
    vlob_id: UUID,
    version: int,
    timestamp: pendulum.Pendulum,
    blob: bytes,
) -> None:
    await _send_cmd(
        transport,
        vlob_update_serializer,
        cmd="vlob_update",
        encryption_revision=encryption_revision,
        vlob_id=vlob_id,
        version=version,
        timestamp=timestamp,
        blob=blob,
    )


async def vlob_group_check(transport: Transport, to_check: list) -> list:
    rep = await _send_cmd(
        transport, vlob_group_check_serializer, cmd="vlob_group_check", to_check=to_check
    )
    return rep["changed"]


async def vlob_poll_changes(
    transport: Transport, realm_id: UUID, last_checkpoint: int
) -> Tuple[int, Dict[UUID, int]]:
    rep = await _send_cmd(
        transport,
        vlob_poll_changes_serializer,
        cmd="vlob_poll_changes",
        realm_id=realm_id,
        last_checkpoint=last_checkpoint,
    )
    return (rep["current_checkpoint"], rep["changes"])


async def vlob_maintenance_get_reencryption_batch(
    transport: Transport, realm_id: UUID, encryption_revision: int, size: int
) -> List[Tuple[EntryID, int, bytes]]:
    rep = await _send_cmd(
        transport,
        vlob_maintenance_get_reencryption_batch_serializer,
        cmd="vlob_maintenance_get_reencryption_batch",
        realm_id=realm_id,
        encryption_revision=encryption_revision,
        size=size,
    )
    return [(x["vlob_id"], x["version"], x["blob"]) for x in rep["batch"]]


async def vlob_maintenance_save_reencryption_batch(
    transport: Transport,
    realm_id: UUID,
    encryption_revision: int,
    batch: List[Tuple[EntryID, int, bytes]],
) -> Tuple[int, int]:
    rep = await _send_cmd(
        transport,
        vlob_maintenance_save_reencryption_batch_serializer,
        cmd="vlob_maintenance_save_reencryption_batch",
        realm_id=realm_id,
        encryption_revision=encryption_revision,
        batch=[{"vlob_id": x[0], "version": x[1], "blob": x[2]} for x in batch],
    )
    return rep["total"], rep["done"]


### Realm API ###


async def realm_status(transport: Transport, realm_id: UUID) -> dict:
    rep = await _send_cmd(transport, realm_status_serializer, cmd="realm_status", realm_id=realm_id)
    rep.pop("status")
    # TODO: return RealmStatus object ?
    return rep


async def realm_get_roles(transport: Transport, realm_id: UUID) -> Dict[UserID, RealmRole]:
    rep = await _send_cmd(
        transport, realm_get_roles_serializer, cmd="realm_get_roles", realm_id=realm_id
    )
    return rep["users"]


async def realm_update_roles(
    transport: Transport, realm_id: UUID, user: UserID, role: Optional[RealmRole]
) -> None:
    await _send_cmd(
        transport,
        realm_update_roles_serializer,
        cmd="realm_update_roles",
        realm_id=realm_id,
        user=user,
        role=role,
    )


async def realm_start_reencryption_maintenance(
    transport: Transport,
    realm_id: UUID,
    encryption_revision: int,
    timestamp: pendulum.Pendulum,
    per_participant_message: Dict[UserID, bytes],
) -> None:
    await _send_cmd(
        transport,
        realm_start_reencryption_maintenance_serializer,
        cmd="realm_start_reencryption_maintenance",
        realm_id=realm_id,
        encryption_revision=encryption_revision,
        timestamp=timestamp,
        per_participant_message=per_participant_message,
    )


async def realm_finish_reencryption_maintenance(
    transport: Transport, realm_id: UUID, encryption_revision: int
) -> None:
    await _send_cmd(
        transport,
        realm_finish_reencryption_maintenance_serializer,
        cmd="realm_finish_reencryption_maintenance",
        realm_id=realm_id,
        encryption_revision=encryption_revision,
    )


### Block API ###


async def block_create(transport: Transport, block_id: UUID, realm_id: UUID, block: bytes) -> None:
    await _send_cmd(
        transport,
        block_create_serializer,
        cmd="block_create",
        block_id=block_id,
        realm_id=realm_id,
        block=block,
    )


async def block_read(transport: Transport, block_id: UUID) -> bytes:
    rep = await _send_cmd(transport, block_read_serializer, cmd="block_read", block_id=block_id)
    return rep["block"]


### User API ###


async def user_get(
    transport: Transport, user_id: UserID
) -> Tuple[UnverifiedRemoteUser, List[UnverifiedRemoteDevice], List[UnverifiedRemoteDevice]]:
    rep = await _send_cmd(transport, user_get_serializer, cmd="user_get", user_id=user_id)

    user = UnverifiedRemoteUser(user_certificate=rep["user_certificate"])
    devices = [
        UnverifiedRemoteDevice(
            device_certificate=d["device_certificate"],
            revoked_device_certificate=d.get("revoked_device_certificate"),
        )
        for d in rep["devices"]
    ]
    trustchain = [
        UnverifiedRemoteDevice(
            device_certificate=d["device_certificate"],
            revoked_device_certificate=d.get("revoked_device_certificate"),
        )
        for d in rep["trustchain"]
    ]
    return (user, devices, trustchain)


async def user_find(
    transport: Transport,
    query: str = None,
    page: int = 1,
    per_page: int = 100,
    omit_revoked: bool = False,
) -> List[UserID]:
    rep = await _send_cmd(
        transport,
        user_find_serializer,
        cmd="user_find",
        query=query,
        page=page,
        per_page=per_page,
        omit_revoked=omit_revoked,
    )
    return rep["results"]


async def user_invite(transport: Transport, user_id: UserID) -> bytes:
    rep = await _send_cmd(transport, user_invite_serializer, cmd="user_invite", user_id=user_id)
    return rep["encrypted_claim"]


async def user_cancel_invitation(transport: Transport, user_id: UserID) -> None:
    await _send_cmd(
        transport, user_cancel_invitation_serializer, cmd="user_cancel_invitation", user_id=user_id
    )


async def user_create(
    transport: Transport, user_certificate: bytes, device_certificate: bytes
) -> None:
    await _send_cmd(
        transport,
        user_create_serializer,
        cmd="user_create",
        user_certificate=user_certificate,
        device_certificate=device_certificate,
    )


async def device_invite(transport: Transport, invited_device_name: DeviceName) -> bytes:
    rep = await _send_cmd(
        transport,
        device_invite_serializer,
        cmd="device_invite",
        invited_device_name=invited_device_name,
    )
    return rep["encrypted_claim"]


async def device_cancel_invitation(transport: Transport, invited_device_name: DeviceName) -> None:
    await _send_cmd(
        transport,
        device_cancel_invitation_serializer,
        cmd="device_cancel_invitation",
        invited_device_name=invited_device_name,
    )


async def device_create(
    transport: Transport, device_certificate: bytes, encrypted_answer: bytes
) -> None:
    await _send_cmd(
        transport,
        device_create_serializer,
        cmd="device_create",
        device_certificate=device_certificate,
        encrypted_answer=encrypted_answer,
    )


async def device_revoke(
    transport: Transport, revoked_device_certificate: bytes
) -> Optional[pendulum.Pendulum]:
    rep = await _send_cmd(
        transport,
        device_revoke_serializer,
        cmd="device_revoke",
        revoked_device_certificate=revoked_device_certificate,
    )
    return rep["user_revoked_on"]


###  Backend anonymous cmds  ###


# ping already defined in authenticated part


async def organization_create(transport: Transport, organization_id: OrganizationID) -> str:
    rep = await _send_cmd(
        transport,
        organization_create_serializer,
        cmd="organization_create",
        organization_id=organization_id,
    )
    return rep["bootstrap_token"]


async def organization_bootstrap(
    transport: Transport,
    organization_id: OrganizationID,
    bootstrap_token: str,
    root_verify_key: VerifyKey,
    user_certificate: bytes,
    device_certificate: bytes,
) -> None:
    await _send_cmd(
        transport,
        organization_bootstrap_serializer,
        cmd="organization_bootstrap",
        organization_id=organization_id,
        bootstrap_token=bootstrap_token,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
    )


async def user_get_invitation_creator(
    transport: Transport, invited_user_id: UserID
) -> Tuple[UnverifiedRemoteDevice, UnverifiedRemoteUser, List[UnverifiedRemoteDevice]]:
    rep = await _send_cmd(
        transport,
        user_get_invitation_creator_serializer,
        cmd="user_get_invitation_creator",
        invited_user_id=invited_user_id,
    )

    device = UnverifiedRemoteDevice(device_certificate=rep["device_certificate"])
    user = UnverifiedRemoteUser(user_certificate=rep["user_certificate"])
    trustchain = [
        UnverifiedRemoteDevice(
            device_certificate=d["device_certificate"],
            revoked_device_certificate=d.get("revoked_device_certificate"),
        )
        for d in rep["trustchain"]
    ]
    return (device, user, trustchain)


async def user_claim(
    transport: Transport, invited_user_id: UserID, encrypted_claim: bytes
) -> UnverifiedRemoteUser:
    rep = await _send_cmd(
        transport,
        user_claim_serializer,
        cmd="user_claim",
        invited_user_id=invited_user_id,
        encrypted_claim=encrypted_claim,
    )
    return UnverifiedRemoteUser(user_certificate=rep["user_certificate"])


async def device_get_invitation_creator(
    transport: Transport, invited_device_id: DeviceID
) -> Tuple[UnverifiedRemoteDevice, UnverifiedRemoteUser, List[UnverifiedRemoteDevice]]:
    rep = await _send_cmd(
        transport,
        device_get_invitation_creator_serializer,
        cmd="device_get_invitation_creator",
        invited_device_id=invited_device_id,
    )

    device = UnverifiedRemoteDevice(device_certificate=rep["device_certificate"])
    user = UnverifiedRemoteUser(user_certificate=rep["user_certificate"])
    trustchain = [
        UnverifiedRemoteDevice(
            device_certificate=d["device_certificate"],
            revoked_device_certificate=d.get("revoked_device_certificate"),
        )
        for d in rep["trustchain"]
    ]
    return (device, user, trustchain)


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
    return rep["encrypted_answer"]

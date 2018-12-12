from typing import Tuple, List, Dict, Iterable
from structlog import get_logger
from uuid import uuid4, UUID
from async_generator import asynccontextmanager

from parsec.types import DeviceID, UserID
from parsec.crypto import SigningKey
from parsec.api.transport import BaseTransport, TransportError
from parsec.api.protocole import (
    ProtocoleError,
    ping_serializer,
    events_subscribe_serializer,
    events_listen_serializer,
    beacon_read_serializer,
    message_send_serializer,
    message_get_serializer,
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
from parsec.core.backend_connection.exceptions import BackendConnectionError, BackendNotAvailable
from parsec.core.backend_connection.transport import (
    authenticated_transport_factory,
    anonymous_transport_factory,
)


__all__ = (
    "BackendCmdsInvalidRequest",
    "BackendCmdsInvalidResponse",
    "BackendCmdsBadResponse",
    "backend_cmds_factory",
    "backend_anonymous_cmds_factory",
    "BackendCmds",
    "BackendAnonymousCmds",
)


logger = get_logger()
# TODO: exceptions


class BackendCmdsInvalidRequest(BackendConnectionError):
    pass


class BackendCmdsInvalidResponse(BackendConnectionError):
    pass


class BackendCmdsBadResponse(BackendConnectionError):
    pass


async def _send_cmd(transport, serializer, **req):
    def _shorten_data(data):
        if len(req) > 300:
            return data[:150] + b"[...]" + data[-150:]
        else:
            return data

    try:
        raw_req = serializer.req_dump(req)

    except ProtocoleError as exc:
        raise BackendCmdsInvalidRequest() from exc

    transport.log.debug("send req", req=_shorten_data(raw_req))
    try:
        await transport.send(raw_req)
        raw_rep = await transport.recv()

    except TransportError as exc:
        raise BackendNotAvailable() from exc

    transport.log.debug("recv rep", req=_shorten_data(raw_rep))

    try:
        rep = serializer.rep_load(raw_rep)

    except ProtocoleError as exc:
        raise BackendCmdsInvalidResponse() from exc

    if rep["status"] == "invalid_msg_format":
        raise BackendCmdsInvalidRequest(rep)

    return rep


class BackendCmds:
    def __init__(self, transport: BaseTransport, log=None):
        self.transport = transport
        self.log = log or logger

    ### Events&misc API ###

    async def ping(self, ping: str) -> str:
        rep = await _send_cmd(self.transport, ping_serializer, cmd="ping", ping=ping)
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)
        return rep["pong"]

    async def events_subscribe(
        self,
        message_received: bool = False,
        beacon_updated: Iterable[UUID] = (),
        pinged: Iterable[str] = (),
    ) -> None:
        rep = await _send_cmd(
            self.transport,
            events_subscribe_serializer,
            cmd="events_subscribe",
            message_received=message_received,
            beacon_updated=beacon_updated,
            pinged=pinged,
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)

    async def events_listen(self, wait: bool = True) -> dict:
        rep = await _send_cmd(
            self.transport, events_listen_serializer, cmd="events_listen", wait=wait
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)
        rep.pop("status")
        return rep

    # Beacon

    async def beacon_read(self, id: UUID, offset: int) -> List[Tuple[UUID, int]]:
        rep = await _send_cmd(
            self.transport, beacon_read_serializer, cmd="beacon_read", id=id, offset=offset
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)
        return [(item["src_id"], item["src_version"]) for item in rep["items"]]

    # Message

    async def message_send(self, recipient: UserID, body: bytes) -> None:
        rep = await _send_cmd(
            self.transport,
            message_send_serializer,
            cmd="message_send",
            recipient=recipient,
            body=body,
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)
        return rep["version"], rep["blob"]

    async def message_get(self, offset: int) -> List[Tuple[int, DeviceID, bytes]]:
        rep = await _send_cmd(
            self.transport, message_get_serializer, cmd="message_get", offset=offset
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)
        return [(item["count"], item["sender"], item["body"]) for item in rep["messages"]]

    ### Vlob API ###

    async def vlob_create(self, id: UUID, rts: str, wts: str, blob: bytes) -> None:
        rep = await _send_cmd(
            self.transport,
            vlob_create_serializer,
            cmd="vlob_create",
            id=id,
            rts=rts,
            wts=wts,
            blob=blob,
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)

    async def vlob_read(self, id: UUID, rts: str) -> Tuple[int, bytes]:
        rep = await _send_cmd(self.transport, vlob_read_serializer, cmd="vlob_read", id=id, rts=rts)
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)
        return rep["version"], rep["blob"]

    async def vlob_update(self, id: UUID, wts: str, version: int, blob: bytes) -> None:
        rep = await _send_cmd(
            self.transport,
            vlob_update_serializer,
            cmd="vlob_update",
            id=id,
            version=version,
            wts=wts,
            blob=blob,
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)

    ### Blockstore API ###

    async def blockstore_create(self, id: UUID, block: bytes) -> None:
        rep = await _send_cmd(
            self.transport,
            blockstore_create_serializer,
            cmd="blockstore_create",
            id=id,
            block=block,
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)

    async def blockstore_read(self, id: UUID) -> bytes:
        rep = await _send_cmd(
            self.transport, blockstore_read_serializer, cmd="blockstore_read", id=id
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)
        return rep["block"]

    ### User API ###

    async def user_get(self, user_id: UserID) -> Tuple[RemoteUser, Dict[DeviceID, RemoteDevice]]:
        rep = await _send_cmd(self.transport, user_get_serializer, cmd="user_get", user_id=user_id)
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
            revocated_on=rep["revocated_on"],
            certified_revocation=rep["certified_revocation"],
            revocation_certifier=rep["revocation_certifier"],
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
        self, query: str = None, page: int = 1, per_page: int = 100
    ) -> List[UserID]:
        rep = await _send_cmd(
            self.transport,
            user_find_serializer,
            cmd="user_find",
            query=query,
            page=page,
            per_page=per_page,
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)
        return rep["results"]

    async def user_invite(self, user_id: UserID) -> bytes:
        rep = await _send_cmd(
            self.transport, user_invite_serializer, cmd="user_invite", user_id=user_id
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)
        return rep["encrypted_claim"]

    async def user_cancel_invitation(self, user_id: UserID) -> None:
        rep = await _send_cmd(
            self.transport,
            user_cancel_invitation_serializer,
            cmd="user_cancel_invitation",
            user_id=user_id,
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)

    async def user_create(self, certified_user: bytes, certified_device: bytes) -> None:
        rep = await _send_cmd(
            self.transport,
            user_create_serializer,
            cmd="user_create",
            certified_user=certified_user,
            certified_device=certified_device,
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)

    async def device_invite(self, device_id: DeviceID) -> bytes:
        rep = await _send_cmd(
            self.transport, device_invite_serializer, cmd="device_invite", device_id=device_id
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)
        return rep["encrypted_claim"]

    async def device_cancel_invitation(self, device_id: DeviceID) -> None:
        rep = await _send_cmd(
            self.transport,
            device_cancel_invitation_serializer,
            cmd="device_cancel_invitation",
            device_id=device_id,
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)

    async def device_create(self, certified_device: bytes, encrypted_answer: bytes) -> None:
        rep = await _send_cmd(
            self.transport,
            device_create_serializer,
            cmd="device_create",
            certified_device=certified_device,
            encrypted_answer=encrypted_answer,
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)

    async def device_revoke(self, certified_revocation: bytes) -> None:
        rep = await _send_cmd(
            self.transport,
            device_revoke_serializer,
            cmd="device_revoke",
            certified_revocation=certified_revocation,
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)


class BackendAnonymousCmds:
    def __init__(self, transport: BaseTransport, log=None):
        self.transport = transport
        # TODO: use logger...
        self.log = log or logger

    async def ping(self, ping: str):
        rep = await _send_cmd(self.transport, ping_serializer, cmd="ping", ping=ping)
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)
        return rep["pong"]

    async def user_get_invitation_creator(self, invited_user_id: UserID) -> RemoteUser:
        rep = await _send_cmd(
            self.transport,
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

    async def user_claim(self, invited_user_id: UserID, encrypted_claim: bytes) -> None:
        rep = await _send_cmd(
            self.transport,
            user_claim_serializer,
            cmd="user_claim",
            invited_user_id=invited_user_id,
            encrypted_claim=encrypted_claim,
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)

    async def device_get_invitation_creator(self, invited_device_id: DeviceID) -> RemoteUser:
        rep = await _send_cmd(
            self.transport,
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

    async def device_claim(self, invited_device_id: DeviceID, encrypted_claim: bytes) -> bytes:
        rep = await _send_cmd(
            self.transport,
            device_claim_serializer,
            cmd="device_claim",
            invited_device_id=invited_device_id,
            encrypted_claim=encrypted_claim,
        )
        if rep["status"] != "ok":
            raise BackendCmdsBadResponse(rep)
        return rep["encrypted_answer"]


@asynccontextmanager
async def backend_cmds_factory(
    addr: str, device_id: DeviceID, signing_key: SigningKey
) -> BackendCmds:
    """
    Raises:
        parsec.api.protocole.ProtocoleError
        BackendNotAvailable
    """
    async with authenticated_transport_factory(addr, device_id, signing_key) as transport:
        log = logger.bind(addr=addr, auth=device_id, id=uuid4().hex)
        yield BackendCmds(transport, log)


@asynccontextmanager
async def backend_anonymous_cmds_factory(addr: str) -> BackendAnonymousCmds:
    async with anonymous_transport_factory(addr) as transport:
        log = logger.bind(addr=addr, auth="<anonymous>", id=uuid4().hex)
        yield BackendAnonymousCmds(transport, log)

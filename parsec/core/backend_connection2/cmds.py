import attr
from typing import Tuple, List, Dict
from structlog import get_logger

from parsec.types import DeviceID, UserID
from parsec.api.transport import BaseTransport
from parsec.api.protocole import (
    user_get_serializer,
    user_find_serializer,
    user_invite_serializer,
    user_get_invitation_creator_serializer,
    user_claim_serializer,
    user_cancel_invitation_serializer,
    user_create_serializer,
    user_revoke_serializer,
    device_invite_serializer,
    device_get_invitation_creator_serializer,
    device_claim_serializer,
    device_cancel_invitation_serializer,
    device_create_serializer,
    device_revoke_serializer,
)
from parsec.core.types import RemoteDevice, RemoteUser, DevicesMapping


logger = get_logger()
# TODO: exceptions


class BackendCmds:
    def __init__(self, transport: BaseTransport, log=None):
        self.transport = transport
        # TODO: use logger...
        self.log = log or logger

    async def user_get(self, user_id: UserID) -> Tuple[RemoteUser, Dict[DeviceID, RemoteDevice]]:
        raw_req = {"cmd": "user_get", "user_id": user_id}
        req = user_get_serializer.req_dump(raw_req)
        raw_rep = await self.transport.send(req)
        rep = user_get_serializer.rep_load(raw_rep)

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
            devices=DevicesMapping(*devices),
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
        raw_req = {"cmd": "user_find", "query": query, "page": page, "per_page": per_page}
        req = user_find_serializer.req_dump(raw_req)
        raw_rep = await self.transport.send(req)
        return user_find_serializer.rep_load(raw_rep)["results"]

    async def user_invite(self, user_id: UserID) -> bytes:
        raw_req = {"cmd": "user_invite", "user_id": user_id}
        req = user_invite_serializer.req_dump(raw_req)
        raw_rep = await self.transport.send(req)
        return user_invite_serializer.rep_load(raw_rep)["encrypted_claim"]

    async def user_cancel_invitation(self, user_id: UserID) -> None:
        raw_req = {"cmd": "user_cancel_invitation", "user_id": user_id}
        req = user_cancel_invitation_serializer.req_dump(raw_req)
        raw_rep = await self.transport.send(req)
        user_cancel_invitation_serializer.rep_load(raw_rep)

    async def user_create(self, certified_user: bytes, certified_device: bytes) -> None:
        raw_req = {
            "cmd": "user_create",
            "certified_user": certified_user,
            "certified_device": certified_device,
        }
        req = user_create_serializer.req_dump(raw_req)
        raw_rep = await self.transport.send(req)
        user_create_serializer.rep_load(raw_rep)

    async def user_revoke(self, certified_revocation: bytes) -> None:
        raw_req = {"cmd": "user_revoke", "certified_revocation": certified_revocation}
        req = user_revoke_serializer.req_dump(raw_req)
        raw_rep = await self.transport.send(req)
        return user_revoke_serializer.rep_load(raw_rep)

    async def device_invite(self, device_id: DeviceID) -> bytes:
        raw_req = {"cmd": "device_invite", "device_id": device_id}
        req = device_invite_serializer.req_dump(raw_req)
        raw_rep = await self.transport.send(req)
        return device_invite_serializer.rep_load(raw_rep).data["encrypted_claim"]

    async def device_cancel_invitation(self, device_id: DeviceID) -> None:
        raw_req = {"cmd": "device_cancel_invitation", "device_id": device_id}
        req = device_cancel_invitation_serializer.req_dump(raw_req)
        raw_rep = await self.transport.send(req)
        device_cancel_invitation_serializer.rep_load(raw_rep)

    async def device_create(self, certified_device: bytes, encrypted_answer: bytes) -> None:
        raw_req = {
            "cmd": "device_create",
            "certified_device": certified_device,
            "encrypted_answer": encrypted_answer,
        }
        req = device_create_serializer.req_dump(raw_req)
        raw_rep = await self.transport.send(req)
        device_create_serializer.rep_load(raw_rep)

    async def device_revoke(self, certified_revocation: bytes) -> None:
        raw_req = {"cmd": "device_revoke", "certified_revocation": certified_revocation}
        req = device_revoke_serializer.req_dump(raw_req)
        raw_rep = await self.transport.send(req)
        device_revoke_serializer.rep_load(raw_rep)


class BackendAnonymousCmds:
    def __init__(self, transport: BaseTransport, log=None):
        self.transport = transport
        # TODO: use logger...
        self.log = log or logger

    async def user_get_invitation_creator(self, invited_user_id: UserID) -> RemoteDevice:
        raw_req = {"cmd": "user_get_invitation_creator", "invited_user_id": invited_user_id}
        req = user_get_invitation_creator_serializer.req_dump(raw_req)
        raw_rep = await self.transport.send(req)
        rep = user_get_invitation_creator_serializer.rep_load(raw_rep)
        return RemoteDevice(
            device_id=rep["device_id"],
            certified_device=rep["certified_device"],
            device_certifier=rep["device_certifier"],
            created_on=rep["created_on"],
            revocated_on=rep["revocated_on"],
            certified_revocation=rep["certified_revocation"],
            revocation_certifier=rep["revocation_certifier"],
        )

    async def user_claim(self, invited_user_id: UserID, encrypted_claim: bytes) -> None:
        raw_req = {"cmd": "user_claim", "invited_user_id": invited_user_id}
        req = user_claim_serializer.req_dump(raw_req)
        raw_rep = await self.transport.send(req)
        user_claim_serializer.rep_load(raw_rep)

    async def device_get_invitation_creator(self, invited_device_id: DeviceID) -> RemoteDevice:
        raw_req = {"cmd": "device_get_invitation_creator", "invited_device_id": invited_device_id}
        req = device_get_invitation_creator_serializer.req_dump(raw_req)
        raw_rep = await self.transport.send(req)
        rep = device_get_invitation_creator_serializer.rep_load(raw_rep)
        return RemoteDevice(
            device_id=rep["device_id"],
            certified_device=rep["certified_device"],
            device_certifier=rep["device_certifier"],
            created_on=rep["created_on"],
            revocated_on=rep["revocated_on"],
            certified_revocation=rep["certified_revocation"],
            revocation_certifier=rep["revocation_certifier"],
        )

    async def device_claim(self, invited_device_id: DeviceID, encrypted_claim: bytes) -> bytes:
        raw_req = {
            "cmd": "device_claim",
            "invited_device_id": invited_device_id,
            "encrypted_claim": encrypted_claim,
        }
        req = device_claim_serializer.req_dump(raw_req)
        raw_rep = await self.transport.send(req)
        return device_claim_serializer.rep_load(raw_rep).data["encrypted_answer"]

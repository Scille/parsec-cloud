# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import attr
from typing import Dict, List, Optional, Tuple
import pendulum

from parsec.types import UserID, DeviceID, OrganizationID, FrozenDict
from parsec.crypto import VerifyKey, PublicKey
from parsec.trustchain import (
    unsecure_certified_device_extract_verify_key,
    unsecure_certified_user_extract_public_key,
    certified_extract_parts,
    validate_payload_certified_user,
    validate_payload_certified_device,
    validate_payload_certified_device_revocation,
    TrustChainError,
)
from parsec.api.protocole import (
    user_get_serializer,
    user_find_serializer,
    user_get_invitation_creator_serializer,
    user_invite_serializer,
    user_claim_serializer,
    user_cancel_invitation_serializer,
    user_create_serializer,
    device_get_invitation_creator_serializer,
    device_invite_serializer,
    device_claim_serializer,
    device_cancel_invitation_serializer,
    device_create_serializer,
    device_revoke_serializer,
)
from parsec.backend.utils import anonymous_api, catch_protocole_errors


class UserError(Exception):
    pass


class UserNotFoundError(UserError):
    pass


class UserAlreadyExistsError(UserError):
    pass


class UserAlreadyRevokedError(UserError):
    pass


PEER_EVENT_MAX_WAIT = 300
INVITATION_VALIDITY = 3600


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class Device:
    def __repr__(self):
        return f"{self.__class__.__name__}({self.device_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    @property
    def device_name(self):
        return self.device_id.device_name

    @property
    def user_id(self):
        return self.device_id.user_id

    @property
    def verify_key(self) -> VerifyKey:
        return unsecure_certified_device_extract_verify_key(self.certified_device)

    device_id: DeviceID
    certified_device: bytes
    device_certifier: Optional[DeviceID]

    created_on: pendulum.Pendulum = attr.ib(factory=pendulum.now)
    revocated_on: pendulum.Pendulum = None
    certified_revocation: bytes = None
    revocation_certifier: DeviceID = None


class DevicesMapping(FrozenDict):
    def __init__(self, *devices: Device):
        super().__init__({d.device_name: d for d in devices})


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class User:
    def __repr__(self):
        return f"{self.__class__.__name__}({self.user_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    @property
    def public_key(self) -> PublicKey:
        return unsecure_certified_user_extract_public_key(self.certified_user)

    user_id: UserID
    certified_user: bytes
    user_certifier: Optional[DeviceID]
    is_admin: bool = False
    devices: DevicesMapping = attr.ib(factory=DevicesMapping)

    created_on: pendulum.Pendulum = attr.ib(factory=pendulum.now)

    def is_revocated(self) -> bool:
        now = pendulum.now()
        for d in self.devices.values():
            if not d.revocated_on or d.revocated_on > now:
                return False
        return True

    def get_revocated_on(self) -> Optional[pendulum.Pendulum]:
        revocations = [d.revocated_on for d in self.devices.values()]
        if not revocations or None in revocations:
            return None
        else:
            return sorted(revocations)[-1]


def new_user_factory(
    device_id: DeviceID,
    is_admin: bool,
    certifier: Optional[DeviceID],
    certified_user: bytes,
    certified_device: bytes,
    now: pendulum.Pendulum = None,
) -> User:
    now = now or pendulum.now()
    return User(
        user_id=device_id.user_id,
        is_admin=is_admin,
        certified_user=certified_user,
        user_certifier=certifier,
        devices=DevicesMapping(
            Device(
                device_id=device_id,
                certified_device=certified_device,
                device_certifier=certifier,
                created_on=now,
            )
        ),
        created_on=now,
    )


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class UserInvitation:
    def __repr__(self):
        return f"{self.__class__.__name__}({self.user_id})"

    user_id: UserID
    creator: DeviceID
    created_on: pendulum.Pendulum = attr.ib(factory=pendulum.now)

    @property
    def organization_id(self) -> OrganizationID:
        return self.user_id.organization_id

    def is_valid(self) -> bool:
        return (pendulum.now() - self.created_on).total_seconds() < INVITATION_VALIDITY


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class DeviceInvitation:
    def __repr__(self):
        return f"{self.__class__.__name__}({self.device_id})"

    device_id: DeviceID
    creator: DeviceID
    created_on: pendulum.Pendulum = attr.ib(factory=pendulum.now)

    def is_valid(self) -> bool:
        return (pendulum.now() - self.created_on).total_seconds() < INVITATION_VALIDITY


class BaseUserComponent:
    #### Access user API ####

    @catch_protocole_errors
    async def api_user_get(self, client_ctx, msg):
        msg = user_get_serializer.req_load(msg)

        try:
            user, trustchain = await self.get_user_with_trustchain(
                client_ctx.organization_id, msg["user_id"]
            )
        except UserNotFoundError:
            return {"status": "not_found"}

        return user_get_serializer.rep_dump(
            {
                "status": "ok",
                "user_id": user.user_id,
                "is_admin": user.is_admin,
                "created_on": user.created_on,
                "certified_user": user.certified_user,
                "user_certifier": user.user_certifier,
                "devices": user.devices,
                "trustchain": trustchain,
            }
        )

    @catch_protocole_errors
    async def api_user_find(self, client_ctx, msg):
        msg = user_find_serializer.req_load(msg)
        results, total = await self.find(client_ctx.organization_id, **msg)
        return user_find_serializer.rep_dump(
            {
                "status": "ok",
                "results": results,
                "page": msg["page"],
                "per_page": msg["per_page"],
                "total": total,
            }
        )

    #### User creation API ####

    @catch_protocole_errors
    async def api_user_invite(self, client_ctx, msg):
        # is_admin could have changed in db since the creation of the connection
        try:
            user, _ = await self.get_user_with_trustchain(
                client_ctx.organization_id, client_ctx.device_id.user_id
            )

        except UserNotFoundError:
            raise RuntimeError("User `{client_ctx.device_id.user_id}` disappeared !")

        if not user.is_admin:
            return {
                "status": "invalid_role",
                "reason": f"User `{client_ctx.device_id.user_id}` is not admin",
            }

        msg = user_invite_serializer.req_load(msg)

        invitation = UserInvitation(msg["user_id"], client_ctx.device_id)
        try:
            await self.create_user_invitation(client_ctx.organization_id, invitation)

        except UserAlreadyExistsError as exc:
            return {"status": "already_exists", "reason": str(exc)}

        # Wait for invited user to send `user_claim`

        claim_answered = trio.Event()
        _encrypted_claim = None

        def _on_user_claimed(event, organization_id, user_id, encrypted_claim):
            nonlocal _encrypted_claim
            if organization_id == client_ctx.organization_id and user_id == invitation.user_id:
                claim_answered.set()
                _encrypted_claim = encrypted_claim

        self.event_bus.connect("user.claimed", _on_user_claimed, weak=True)
        with trio.move_on_after(PEER_EVENT_MAX_WAIT) as cancel_scope:
            await claim_answered.wait()
        if cancel_scope.cancelled_caught:
            return {
                "status": "timeout",
                "reason": ("Timeout while waiting for new user to be claimed."),
            }
        return user_invite_serializer.rep_dump(
            {"status": "ok", "encrypted_claim": _encrypted_claim}
        )

    @anonymous_api
    @catch_protocole_errors
    async def api_user_get_invitation_creator(self, client_ctx, msg):
        msg = user_get_invitation_creator_serializer.req_load(msg)

        try:
            invitation = await self.get_user_invitation(
                client_ctx.organization_id, msg["invited_user_id"]
            )
            if not invitation.is_valid():
                return {"status": "not_found"}

            user, trustchain = await self.get_user_with_trustchain(
                client_ctx.organization_id, invitation.creator.user_id
            )

        except UserNotFoundError:
            return {"status": "not_found"}

        return user_get_invitation_creator_serializer.rep_dump(
            {
                "status": "ok",
                "user_id": user.user_id,
                "created_on": user.created_on,
                "certified_user": user.certified_user,
                "user_certifier": user.user_certifier,
                "trustchain": trustchain,
            }
        )

    @anonymous_api
    @catch_protocole_errors
    async def api_user_claim(self, client_ctx, msg):
        msg = user_claim_serializer.req_load(msg)

        try:
            invitation = await self.claim_user_invitation(
                client_ctx.organization_id, msg["invited_user_id"], msg["encrypted_claim"]
            )
            if not invitation.is_valid():
                return {"status": "not_found"}

        except UserAlreadyExistsError:
            return {"status": "not_found"}

        except UserNotFoundError:
            return {"status": "not_found"}

        # Wait for creator user to accept (or refuse) our claim

        replied = trio.Event()
        replied_ok = False

        def _on_reply(event, organization_id, user_id):
            nonlocal replied_ok
            if organization_id == client_ctx.organization_id and user_id == invitation.user_id:
                replied_ok = event == "user.created"
                replied.set()

        self.event_bus.connect("user.created", _on_reply, weak=True)
        self.event_bus.connect("user.invitation.cancelled", _on_reply, weak=True)
        with trio.move_on_after(PEER_EVENT_MAX_WAIT) as cancel_scope:
            await replied.wait()
        if cancel_scope.cancelled_caught:
            return {
                "status": "timeout",
                "reason": "Timeout while waiting for invitation creator to answer.",
            }
        if not replied_ok:
            return {"status": "denied", "reason": "Invitation creator rejected us."}
        return user_claim_serializer.rep_dump({"status": "ok"})

    @catch_protocole_errors
    async def api_user_cancel_invitation(self, client_ctx, msg):
        msg = user_cancel_invitation_serializer.req_load(msg)

        await self.cancel_user_invitation(client_ctx.organization_id, msg["user_id"])

        return user_cancel_invitation_serializer.rep_dump({"status": "ok"})

    @catch_protocole_errors
    async def api_user_create(self, client_ctx, msg):
        msg = user_create_serializer.req_load(msg)

        try:
            u_certifier_id, u_payload = certified_extract_parts(msg["certified_user"])
            d_certifier_id, d_payload = certified_extract_parts(msg["certified_device"])

        except TrustChainError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        if u_certifier_id != client_ctx.device_id or d_certifier_id != client_ctx.device_id:
            return {
                "status": "invalid_certification",
                "reason": "Certifier is not the authenticated device.",
            }

        try:
            now = pendulum.now()
            u_data = validate_payload_certified_user(client_ctx.verify_key, u_payload, now)
            d_data = validate_payload_certified_device(client_ctx.verify_key, d_payload, now)

        except TrustChainError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        if u_data["user_id"] != d_data["device_id"].user_id:
            return {
                "status": "invalid_data",
                "reason": "Device and User must have the same user ID.",
            }

        if u_data["timestamp"] != d_data["timestamp"]:
            return {
                "status": "invalid_data",
                "reason": "Device and User must have the same timestamp.",
            }

        try:
            user = User(
                user_id=u_data["user_id"],
                is_admin=msg["is_admin"],
                certified_user=msg["certified_user"],
                user_certifier=u_certifier_id,
                devices=DevicesMapping(
                    Device(
                        device_id=d_data["device_id"],
                        certified_device=msg["certified_device"],
                        device_certifier=d_certifier_id,
                        created_on=d_data["timestamp"],
                    )
                ),
                created_on=u_data["timestamp"],
            )
            await self.create_user(client_ctx.organization_id, user)

        except UserAlreadyExistsError as exc:
            return {"status": "already_exists", "reason": str(exc)}

        return user_create_serializer.rep_dump({"status": "ok"})

    #### Device creation API ####

    @catch_protocole_errors
    async def api_device_invite(self, client_ctx, msg):
        msg = device_invite_serializer.req_load(msg)

        invited_device_id = DeviceID(f"{client_ctx.device_id.user_id}@{msg['invited_device_name']}")
        invitation = DeviceInvitation(invited_device_id, client_ctx.device_id)
        try:
            await self.create_device_invitation(client_ctx.organization_id, invitation)

        except UserAlreadyExistsError as exc:
            return {"status": "already_exists", "reason": str(exc)}

        # Wait for invited user to send `user_claim`

        claim_answered = trio.Event()
        _encrypted_claim = None

        def _on_device_claimed(event, organization_id, device_id, encrypted_claim):
            nonlocal _encrypted_claim
            if organization_id == client_ctx.organization_id and device_id == invitation.device_id:
                claim_answered.set()
                _encrypted_claim = encrypted_claim

        self.event_bus.connect("device.claimed", _on_device_claimed, weak=True)
        with trio.move_on_after(PEER_EVENT_MAX_WAIT) as cancel_scope:
            await claim_answered.wait()
        if cancel_scope.cancelled_caught:
            return {
                "status": "timeout",
                "reason": ("Timeout while waiting for new device to be claimed."),
            }
        return device_invite_serializer.rep_dump(
            {"status": "ok", "encrypted_claim": _encrypted_claim}
        )

    @anonymous_api
    @catch_protocole_errors
    async def api_device_get_invitation_creator(self, client_ctx, msg):
        msg = device_get_invitation_creator_serializer.req_load(msg)

        try:
            invitation = await self.get_device_invitation(
                client_ctx.organization_id, msg["invited_device_id"]
            )
            if not invitation.is_valid():
                return {"status": "not_found"}

            user, trustchain = await self.get_user_with_trustchain(
                client_ctx.organization_id, invitation.creator.user_id
            )

        except UserNotFoundError:
            return {"status": "not_found"}

        return device_get_invitation_creator_serializer.rep_dump(
            {
                "status": "ok",
                "user_id": user.user_id,
                "created_on": user.created_on,
                "certified_user": user.certified_user,
                "user_certifier": user.user_certifier,
                "trustchain": trustchain,
            }
        )

    @anonymous_api
    @catch_protocole_errors
    async def api_device_claim(self, client_ctx, msg):
        msg = device_claim_serializer.req_load(msg)

        try:
            invitation = await self.claim_device_invitation(
                client_ctx.organization_id, msg["invited_device_id"], msg["encrypted_claim"]
            )
            if not invitation.is_valid():
                return {"status": "not_found"}

        except UserAlreadyExistsError:
            return {"status": "not_found"}

        except UserNotFoundError:
            return {"status": "not_found"}

        # Wait for creator device to accept (or refuse) our claim

        replied = trio.Event()
        replied_ok = False
        replied_encrypted_answer = None

        def _on_reply(event, organization_id, device_id, encrypted_answer=None):
            nonlocal replied_ok, replied_encrypted_answer
            if organization_id == client_ctx.organization_id and device_id == invitation.device_id:
                replied_ok = event == "device.created"
                replied_encrypted_answer = encrypted_answer
                replied.set()

        self.event_bus.connect("device.created", _on_reply, weak=True)
        self.event_bus.connect("device.invitation.cancelled", _on_reply, weak=True)
        with trio.move_on_after(PEER_EVENT_MAX_WAIT) as cancel_scope:
            await replied.wait()
        if cancel_scope.cancelled_caught:
            return {
                "status": "timeout",
                "reason": ("Timeout while waiting for invitation creator to answer."),
            }
        if not replied_ok:
            return {"status": "denied", "reason": ("Invitation creator rejected us.")}

        return device_claim_serializer.rep_dump(
            {"status": "ok", "encrypted_answer": replied_encrypted_answer}
        )

    @catch_protocole_errors
    async def api_device_cancel_invitation(self, client_ctx, msg):
        msg = device_cancel_invitation_serializer.req_load(msg)

        invited_device_id = DeviceID(f"{client_ctx.device_id.user_id}@{msg['invited_device_name']}")

        await self.cancel_device_invitation(client_ctx.organization_id, invited_device_id)

        return device_cancel_invitation_serializer.rep_dump({"status": "ok"})

    @catch_protocole_errors
    async def api_device_create(self, client_ctx, msg):
        msg = device_create_serializer.req_load(msg)

        try:
            certifier_id, payload = certified_extract_parts(msg["certified_device"])
        except TrustChainError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        if certifier_id != client_ctx.device_id:
            return {
                "status": "invalid_certification",
                "reason": "Certifier is not the authenticated device.",
            }

        try:
            data = validate_payload_certified_device(client_ctx.verify_key, payload, pendulum.now())
        except TrustChainError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        if data["device_id"].user_id != client_ctx.user_id:
            return {"status": "bad_user_id", "reason": "Device must be handled by it own user."}

        try:
            device = Device(
                device_id=data["device_id"],
                certified_device=msg["certified_device"],
                device_certifier=certifier_id,
                created_on=data["timestamp"],
            )
            await self.create_device(
                client_ctx.organization_id, device, encrypted_answer=msg["encrypted_answer"]
            )
        except UserAlreadyExistsError as exc:
            return {"status": "already_exists", "reason": str(exc)}

        return device_create_serializer.rep_dump({"status": "ok"})

    @catch_protocole_errors
    async def api_device_revoke(self, client_ctx, msg):
        msg = device_revoke_serializer.req_load(msg)

        try:
            certifier_id, payload = certified_extract_parts(msg["certified_revocation"])
        except TrustChainError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        if certifier_id != client_ctx.device_id:
            return {
                "status": "invalid_certification",
                "reason": "Certifier is not the authenticated device.",
            }

        try:
            data = validate_payload_certified_device_revocation(
                client_ctx.verify_key, payload, pendulum.now()
            )
        except TrustChainError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        if client_ctx.device_id.user_id != data["device_id"].user_id:
            try:
                user, _ = await self.get_user_with_trustchain(
                    client_ctx.organization_id, client_ctx.device_id.user_id
                )

            except UserNotFoundError:
                raise RuntimeError("User `{client_ctx.device_id.user_id}` disappeared !")

            if not user.is_admin:
                return {
                    "status": "invalid_role",
                    "reason": f"User `{client_ctx.device_id.user_id}` is not admin",
                }

        try:
            user_revocated_on = await self.revoke_device(
                client_ctx.organization_id,
                data["device_id"],
                msg["certified_revocation"],
                certifier_id,
            )

        except UserNotFoundError:
            return {"status": "not_found"}

        except UserAlreadyRevokedError:
            return {
                "status": "already_revoked",
                "reason": f"Device `{data['device_id']}` already revoked",
            }

        return device_revoke_serializer.rep_dump(
            {"status": "ok", "user_revocated_on": user_revocated_on}
        )

    #### Virtual methods ####

    async def set_user_admin(
        self, organization_id: OrganizationID, user_id: UserID, is_admin: bool
    ) -> None:
        """
        Raises:
            UserNotFoundError
        """
        raise NotImplementedError()

    async def create_user(self, organization_id: OrganizationID, user: User) -> None:
        """
        Raises:
            UserAlreadyExistsError
        """
        raise NotImplementedError()

    async def create_device(
        self, organization_id: OrganizationID, device: Device, encrypted_answer: bytes = b""
    ) -> None:
        """
        Raises:
            UserAlreadyExistsError
        """
        raise NotImplementedError()

    async def get_user(self, organization_id: OrganizationID, user_id: UserID) -> User:
        """
        Raises:
            UserNotFoundError
        """
        raise NotImplementedError()

    async def get_user_with_trustchain(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> Tuple[User, Dict[DeviceID, Device]]:
        """
        Raises:
            UserNotFoundError
        """
        raise NotImplementedError()

    async def get_device(self, organization_id: OrganizationID, device_id: DeviceID) -> Device:
        """
        Raises:
            UserNotFoundError
        """
        raise NotImplementedError()

    async def get_device_with_trustchain(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> Tuple[Device, Dict[DeviceID, Device]]:
        """
        Raises:
            UserNotFoundError
        """
        raise NotImplementedError()

    async def find(
        self,
        organization_id: OrganizationID,
        query: str = None,
        page: int = 1,
        per_page: int = 100,
        omit_revocated: bool = False,
    ) -> Tuple[List[UserID], int]:
        raise NotImplementedError()

    async def create_user_invitation(
        self, organization_id: OrganizationID, invitation: UserInvitation
    ) -> None:
        """
        Raises:
            UserAlreadyExistsError
        """
        raise NotImplementedError()

    async def get_user_invitation(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> UserInvitation:
        """
        Raises:
            UserAlreadyExistsError
            UserNotFoundError
        """
        raise NotImplementedError()

    async def claim_user_invitation(
        self, organization_id: OrganizationID, user_id: UserID, encrypted_claim: bytes = b""
    ) -> UserInvitation:
        """
        Raises:
            UserAlreadyExistsError
            UserNotFoundError
        """
        raise NotImplementedError()

    async def cancel_user_invitation(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> None:
        """
        Raises: Nothing
        """
        raise NotImplementedError()

    async def create_device_invitation(
        self, organization_id: OrganizationID, invitation: DeviceInvitation
    ) -> None:
        """
        Raises:
            UserAlreadyExistsError
            UserNotFoundError
        """
        raise NotImplementedError()

    async def get_device_invitation(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> DeviceInvitation:
        """
        Raises:
            UserAlreadyExistsError
            UserNotFoundError
        """
        raise NotImplementedError()

    async def claim_device_invitation(
        self, organization_id: OrganizationID, device_id: DeviceID, encrypted_claim: bytes = b""
    ) -> UserInvitation:
        """
        Raises:
            UserAlreadyExistsError
            UserNotFoundError
        """
        raise NotImplementedError()

    async def cancel_device_invitation(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> None:
        """
        Raises: Nothing
        """
        raise NotImplementedError()

    async def revoke_device(
        self,
        organization_id: OrganizationID,
        device_id: DeviceID,
        certified_revocation: bytes,
        revocation_certifier: DeviceID,
    ) -> Optional[pendulum.Pendulum]:
        """
        Raises:
            UserNotFoundError
            UserAlreadyRevokedError
        Returns: User revoked date if any
        """
        raise NotImplementedError()

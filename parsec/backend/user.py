# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import attr
from typing import List, Optional, Tuple
import pendulum

from parsec.types import UserID, DeviceID, OrganizationID
from parsec.crypto import (
    CryptoError,
    VerifyKey,
    PublicKey,
    verify_user_certificate,
    verify_device_certificate,
    verify_revoked_device_certificate,
    unsecure_read_device_certificate,
    unsecure_read_user_certificate,
    timestamps_in_the_ballpark,
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
        return unsecure_read_device_certificate(self.device_certificate).verify_key

    device_id: DeviceID
    device_certificate: bytes
    device_certifier: Optional[DeviceID]

    created_on: pendulum.Pendulum = attr.ib(factory=pendulum.now)
    revoked_on: pendulum.Pendulum = None
    revoked_device_certificate: bytes = None
    revoked_device_certifier: DeviceID = None


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class User:
    def __repr__(self):
        return f"{self.__class__.__name__}({self.user_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    @property
    def public_key(self) -> PublicKey:
        return unsecure_read_user_certificate(self.user_certificate).public_key

    user_id: UserID
    user_certificate: bytes
    user_certifier: Optional[DeviceID]
    is_admin: bool = False
    created_on: pendulum.Pendulum = attr.ib(factory=pendulum.now)


def user_is_revoked(devices: List[Device]) -> bool:
    now = pendulum.now()
    for d in devices:
        if not d.revoked_on or d.revoked_on > now:
            return False
    return True


def user_get_revoked_on(devices: List[Device]) -> Optional[pendulum.Pendulum]:
    revocations = [d.revoked_on for d in devices]
    if not revocations or None in revocations:
        return None
    else:
        return sorted(revocations)[-1]


def new_user_factory(
    device_id: DeviceID,
    is_admin: bool,
    certifier: Optional[DeviceID],
    user_certificate: bytes,
    device_certificate: bytes,
    now: pendulum.Pendulum = None,
) -> Tuple[User, Device]:
    now = now or pendulum.now()
    user = User(
        user_id=device_id.user_id,
        is_admin=is_admin,
        user_certificate=user_certificate,
        user_certifier=certifier,
        created_on=now,
    )
    first_device = Device(
        device_id=device_id,
        device_certificate=device_certificate,
        device_certifier=certifier,
        created_on=now,
    )
    return user, first_device


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
            user, devices, trustchain = await self.get_user_with_devices_and_trustchain(
                client_ctx.organization_id, msg["user_id"]
            )
        except UserNotFoundError:
            return {"status": "not_found"}

        return user_get_serializer.rep_dump(
            {
                "status": "ok",
                "user_id": user.user_id,
                "user_certificate": user.user_certificate,
                "devices": devices,
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
        if not client_ctx.is_admin:
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

        def _filter_on_user_claimed(event, organization_id, user_id, encrypted_claim):
            return organization_id == client_ctx.organization_id and user_id == invitation.user_id

        with self.event_bus.waiter_on("user.claimed", filter=_filter_on_user_claimed) as waiter:
            with trio.move_on_after(PEER_EVENT_MAX_WAIT) as cancel_scope:
                _, event_data = await waiter.wait()

        if cancel_scope.cancelled_caught:
            return {
                "status": "timeout",
                "reason": ("Timeout while waiting for new user to be claimed."),
            }

        else:
            return user_invite_serializer.rep_dump(
                {"status": "ok", "encrypted_claim": event_data["encrypted_claim"]}
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

            creator_user, creator_device = await self.get_user_with_device(
                client_ctx.organization_id, invitation.creator
            )

            user, trustchain = await self.get_user_with_trustchain(
                client_ctx.organization_id, invitation.creator.user_id
            )

        except UserNotFoundError:
            return {"status": "not_found"}

        return user_get_invitation_creator_serializer.rep_dump(
            {
                "status": "ok",
                "device_certificate": creator_device.device_certificate,
                "user_certificate": user.user_certificate,
                "trustchain": trustchain,
            }
        )

    @anonymous_api
    @catch_protocole_errors
    async def api_user_claim(self, client_ctx, msg):
        msg = user_claim_serializer.req_load(msg)

        # Setting the cancel scope here instead of just were we are waiting
        # for the event make testing easier.
        with trio.move_on_after(PEER_EVENT_MAX_WAIT) as cancel_scope:
            rep = await self._api_user_claim(client_ctx, msg)

        if cancel_scope.cancelled_caught:
            rep = {
                "status": "timeout",
                "reason": "Timeout while waiting for invitation creator to answer.",
            }

        return user_claim_serializer.rep_dump(rep)

    async def _api_user_claim(self, client_ctx, msg):
        replied_ok = False

        # Must start listening events before calling `claim_user_invitation`
        # given this function will send the `user.claimed` event the creator
        # is waiting for.

        send_channel, recv_channel = trio.open_memory_channel(1000)

        def _on_organization_events(event, organization_id, user_id, first_device_id=None):
            if organization_id == client_ctx.organization_id:
                send_channel.send_nowait((event, user_id))

        with self.event_bus.connect_in_context(
            ("user.created", _on_organization_events),
            ("user.invitation.cancelled", _on_organization_events),
        ):
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
            async for event, user_id in recv_channel:
                if user_id == invitation.user_id:
                    replied_ok = event == "user.created"
                    user = await self.get_user(client_ctx.organization_id, invitation.user_id)
                    user_certificate = user.user_certificate
                    break

        if not replied_ok:
            return {"status": "denied", "reason": "Invitation creator rejected us."}

        else:
            return {"status": "ok", "user_certificate": user_certificate}

    @catch_protocole_errors
    async def api_user_cancel_invitation(self, client_ctx, msg):
        msg = user_cancel_invitation_serializer.req_load(msg)

        await self.cancel_user_invitation(client_ctx.organization_id, msg["user_id"])

        return user_cancel_invitation_serializer.rep_dump({"status": "ok"})

    @catch_protocole_errors
    async def api_user_create(self, client_ctx, msg):
        msg = user_create_serializer.req_load(msg)

        try:
            d_data = verify_device_certificate(
                msg["device_certificate"], client_ctx.device_id, client_ctx.verify_key
            )
            u_data = verify_user_certificate(
                msg["user_certificate"], client_ctx.device_id, client_ctx.verify_key
            )

        except CryptoError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        if u_data.certified_on != d_data.certified_on:
            return {
                "status": "invalid_data",
                "reason": "Device and User certifications must have the same timestamp.",
            }

        now = pendulum.now()
        if not timestamps_in_the_ballpark(u_data.certified_on, now):
            return {
                "status": "invalid_certification",
                "reason": f"Invalid timestamp in certification.",
            }

        if u_data.user_id != d_data.device_id.user_id:
            return {
                "status": "invalid_data",
                "reason": "Device and User must have the same user ID.",
            }

        try:
            user = User(
                user_id=u_data.user_id,
                is_admin=u_data.is_admin,
                user_certificate=msg["user_certificate"],
                user_certifier=u_data.certified_by,
                created_on=u_data.certified_on,
            )
            first_devices = Device(
                device_id=d_data.device_id,
                device_certificate=msg["device_certificate"],
                device_certifier=d_data.certified_by,
                created_on=d_data.certified_on,
            )
            await self.create_user(client_ctx.organization_id, user, first_devices)

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

        def _filter_on_device_claimed(event, organization_id, device_id, encrypted_claim):
            return (
                organization_id == client_ctx.organization_id and device_id == invitation.device_id
            )

        with self.event_bus.waiter_on("device.claimed", filter=_filter_on_device_claimed) as waiter:
            with trio.move_on_after(PEER_EVENT_MAX_WAIT) as cancel_scope:
                _, event_data = await waiter.wait()

        if cancel_scope.cancelled_caught:
            return {
                "status": "timeout",
                "reason": ("Timeout while waiting for new device to be claimed."),
            }

        else:
            return device_invite_serializer.rep_dump(
                {"status": "ok", "encrypted_claim": event_data["encrypted_claim"]}
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

            creator_user, creator_device = await self.get_user_with_device(
                client_ctx.organization_id, invitation.creator
            )

            user, trustchain = await self.get_user_with_trustchain(
                client_ctx.organization_id, invitation.creator.user_id
            )

        except UserNotFoundError:
            return {"status": "not_found"}

        return device_get_invitation_creator_serializer.rep_dump(
            {
                "status": "ok",
                "device_certificate": creator_device.device_certificate,
                "user_certificate": user.user_certificate,
                "trustchain": trustchain,
            }
        )

    @anonymous_api
    @catch_protocole_errors
    async def api_device_claim(self, client_ctx, msg):
        msg = device_claim_serializer.req_load(msg)

        # Setting the cancel scope here instead of just were we are waiting
        # for the event make testing easier.
        with trio.move_on_after(PEER_EVENT_MAX_WAIT) as cancel_scope:
            rep = await self._api_device_claim(client_ctx, msg)

        if cancel_scope.cancelled_caught:
            rep = {
                "status": "timeout",
                "reason": "Timeout while waiting for invitation creator to answer.",
            }

        return device_claim_serializer.rep_dump(rep)

    async def _api_device_claim(self, client_ctx, msg):
        replied_ok = False

        # Must start listening events before calling `claim_device_invitation`
        # given this function will send the `device.claimed` event the creator
        # is waiting for.

        send_channel, recv_channel = trio.open_memory_channel(1000)

        def _on_organization_events(event, organization_id, device_id, encrypted_answer=None):
            if organization_id == client_ctx.organization_id:
                send_channel.send_nowait((event, device_id, encrypted_answer))

        with self.event_bus.connect_in_context(
            ("device.created", _on_organization_events),
            ("device.invitation.cancelled", _on_organization_events),
        ):
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
            async for event, device_id, encrypted_answer in recv_channel:
                if device_id == invitation.device_id:
                    replied_ok = event == "device.created"
                    break

        if not replied_ok:
            return {"status": "denied", "reason": ("Invitation creator rejected us.")}

        else:
            return device_claim_serializer.rep_dump(
                {"status": "ok", "encrypted_answer": encrypted_answer}
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
            data = verify_device_certificate(
                msg["device_certificate"], client_ctx.device_id, client_ctx.verify_key
            )

        except CryptoError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        if not timestamps_in_the_ballpark(data.certified_on, pendulum.now()):
            return {
                "status": "invalid_certification",
                "reason": f"Invalid timestamp in certification.",
            }

        if data.device_id.user_id != client_ctx.user_id:
            return {"status": "bad_user_id", "reason": "Device must be handled by it own user."}

        try:
            device = Device(
                device_id=data.device_id,
                device_certificate=msg["device_certificate"],
                device_certifier=data.certified_by,
                created_on=data.certified_on,
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
            data = verify_revoked_device_certificate(
                msg["revoked_device_certificate"], client_ctx.device_id, client_ctx.verify_key
            )

        except CryptoError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        if not timestamps_in_the_ballpark(data.certified_on, pendulum.now()):
            return {
                "status": "invalid_certification",
                "reason": f"Invalid timestamp in certification.",
            }

        if client_ctx.device_id.user_id != data.device_id.user_id:
            try:
                user = await self.get_user(client_ctx.organization_id, client_ctx.device_id.user_id)

            except UserNotFoundError as exc:
                raise RuntimeError("User `{client_ctx.device_id.user_id}` disappeared !") from exc

            if not user.is_admin:
                return {
                    "status": "invalid_role",
                    "reason": f"User `{client_ctx.device_id.user_id}` is not admin",
                }

        try:
            user_revoked_on = await self.revoke_device(
                client_ctx.organization_id,
                data.device_id,
                msg["revoked_device_certificate"],
                data.certified_by,
                data.certified_on,
            )

        except UserNotFoundError:
            return {"status": "not_found"}

        except UserAlreadyRevokedError:
            return {
                "status": "already_revoked",
                "reason": f"Device `{data.device_id}` already revoked",
            }

        return device_revoke_serializer.rep_dump(
            {"status": "ok", "user_revoked_on": user_revoked_on}
        )

    #### Virtual methods ####

    async def create_user(
        self, organization_id: OrganizationID, user: User, first_device: Device
    ) -> None:
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
    ) -> Tuple[User, Tuple[Device]]:
        """
        Raises:
            UserNotFoundError
        """
        raise NotImplementedError()

    async def get_user_with_devices_and_trustchain(
        self, organization_id: OrganizationID, user_id: UserID
    ) -> Tuple[User, Tuple[Device], Tuple[Device]]:
        """
        Raises:
            UserNotFoundError
        """
        raise NotImplementedError()

    async def get_user_with_device(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> Tuple[User, Device]:
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
        omit_revoked: bool = False,
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
        revoked_device_certificate: bytes,
        revoked_device_certifier: DeviceID,
        revoked_on: pendulum.Pendulum = None,
    ) -> Optional[pendulum.Pendulum]:
        """
        Raises:
            UserNotFoundError
            UserAlreadyRevokedError
        Returns: User revoked date if any
        """
        raise NotImplementedError()

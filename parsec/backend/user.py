import trio
import attr
import random
import string
from typing import List, Tuple
import pendulum

from parsec.types import UserID, DeviceID
from parsec.event_bus import EventBus
from parsec.crypto import VerifyKey
from parsec.trustchain import (
    unsecure_certified_device_extract_verify_key,
    unsecure_certified_user_extract_public_key,
    certified_extract_parts,
    validate_payload_certified_user,
    validate_payload_certified_device,
    validate_payload_certified_user_revocation,
    validate_payload_certified_device_revocation,
    TrustChainError,
)
from parsec.api.user import (
    user_get_req_schema,
    user_get_rep_schema,
    user_find_req_schema,
    user_find_rep_schema,
    user_get_invitation_creator_req_schema,
    user_get_invitation_creator_rep_schema,
    user_invite_req_schema,
    user_claim_req_schema,
    user_cancel_invitation_req_schema,
    user_create_req_schema,
    user_revoke_req_schema,
    device_get_invitation_creator_req_schema,
    device_get_invitation_creator_rep_schema,
    device_invite_req_schema,
    device_claim_req_schema,
    device_claim_rep_schema,
    device_cancel_invitation_req_schema,
    device_create_req_schema,
    device_revoke_req_schema,
)
from parsec.backend.utils import anonymous_api
from parsec.backend.exceptions import NotFoundError, AlreadyExistsError, AlreadyRevokedError


PEER_EVENT_MAX_WAIT = 300
INVITATION_VALIDITY = 3600


@attr.s(slots=True, frozen=True, repr=False)
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
    def verify_key(self):
        return unsecure_certified_device_extract_verify_key(self.certified_device)

    device_id = attr.ib()
    certified_device = attr.ib()
    device_certifier = attr.ib()

    created_on = attr.ib(factory=pendulum.now)
    revocated_on = attr.ib(default=None)
    certified_revocation = attr.ib(default=None)
    revocation_certifier = attr.ib(default=None)


class DevicesMapping:
    """
    Basically a frozen dict.
    """

    __slots__ = ("_read_only_mapping",)

    def __init__(self, *devices: Device):
        self._read_only_mapping = {d.device_name: d for d in devices}

    def __repr__(self):
        return f"{self.__class__.__name__}({self._read_only_mapping!r})"

    def __getitem__(self, key):
        return self._read_only_mapping[key]

    def items(self):
        return self._read_only_mapping.items()

    def keys(self):
        return self._read_only_mapping.keys()

    def values(self):
        return self._read_only_mapping.values()

    def __iter__(self):
        return self._read_only_mapping.__iter__()

    def __in__(self, key):
        return self._read_only_mapping.__in__(key)


@attr.s(slots=True, frozen=True, repr=False)
class User:
    def __repr__(self):
        return f"{self.__class__.__name__}({self.user_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    @property
    def public_key(self):
        return unsecure_certified_user_extract_public_key(self.certified_user)

    user_id = attr.ib()
    certified_user = attr.ib()
    user_certifier = attr.ib()
    devices = attr.ib(factory=DevicesMapping)

    created_on = attr.ib(factory=pendulum.now)
    revocated_on = attr.ib(default=None)
    certified_revocation = attr.ib(default=None)
    revocation_certifier = attr.ib(default=None)


@attr.s(slots=True, frozen=True, repr=False)
class UserInvitation:
    def __repr__(self):
        return f"{self.__class__.__name__}({self.user_id})"

    user_id = attr.ib()
    creator = attr.ib()
    created_on = attr.ib(factory=pendulum.now)

    def is_valid(self) -> bool:
        return (pendulum.now() - self.created_on).total_seconds() < INVITATION_VALIDITY


@attr.s(slots=True, frozen=True, repr=False)
class DeviceInvitation:
    def __repr__(self):
        return f"{self.__class__.__name__}({self.device_id})"

    device_id = attr.ib()
    creator = attr.ib()
    created_on = attr.ib(factory=pendulum.now)

    def is_valid(self) -> bool:
        return (pendulum.now() - self.created_on).total_seconds() < INVITATION_VALIDITY


def _generate_token():
    return "".join([random.choice(string.digits) for _ in range(6)])


class BaseUserComponent:
    def __init__(self, root_verify_key: VerifyKey, event_bus: EventBus):
        self.root_verify_key = root_verify_key
        self.event_bus = event_bus

    #### Access user API ####

    async def api_user_get(self, client_ctx, msg):
        msg = user_get_req_schema.load_or_abort(msg)
        try:
            user = await self.get_user(msg["user_id"])
        except NotFoundError as exc:
            return {"status": "not_found"}

        data, errors = user_get_rep_schema.dump(user)
        if errors:
            raise RuntimeError(f"Dump error with {user!r}: {errors}")
        return data

    async def api_user_find(self, client_ctx, msg):
        msg = user_find_req_schema.load_or_abort(msg)
        results, total = await self.find(**msg)
        return user_find_rep_schema.dump(
            {
                "status": "ok",
                "results": results,
                "page": msg["page"],
                "per_page": msg["per_page"],
                "total": total,
            }
        ).data

    #### User creation API ####

    async def api_user_invite(self, client_ctx, msg):
        msg = user_invite_req_schema.load_or_abort(msg)
        invitation = UserInvitation(msg["user_id"], client_ctx.device_id)
        try:
            await self.create_user_invitation(invitation)

        except AlreadyExistsError as exc:
            return {"status": "already_exists", "reason": str(exc)}

        # Wait for invited user to send `user_claim`

        claim_answered = trio.Event()
        _encrypted_claim = None

        def _on_user_claimed(event, user_id, encrypted_claim):
            nonlocal _encrypted_claim
            if user_id == invitation.user_id:
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
        return {"status": "ok", "encrypted_claim": _encrypted_claim}

    @anonymous_api
    async def api_user_get_invitation_creator(self, client_ctx, msg):
        msg = user_get_invitation_creator_req_schema.load_or_abort(msg)
        try:
            invitation = await self.get_user_invitation(msg["invited_user_id"])
            if not invitation.is_valid():
                return {"status": "not_found"}
            certifier = await self.get_device(invitation.creator)

        except NotFoundError:
            return {"status": "not_found"}

        data, errors = user_get_invitation_creator_rep_schema.dump(certifier)

        if errors:
            raise RuntimeError(f"Dump error with {certifier!r}: {errors}")
        return data

    @anonymous_api
    async def api_user_claim(self, client_ctx, msg):
        msg = user_claim_req_schema.load_or_abort(msg)

        try:
            invitation = await self.get_user_invitation(msg["invited_user_id"])
            if not invitation.is_valid():
                return {"status": "not_found"}

        except NotFoundError:
            return {"status": "not_found"}

        self.event_bus.send(
            "user.claimed", user_id=invitation.user_id, encrypted_claim=msg["encrypted_claim"]
        )

        # Wait for creator user to accept (or refuse) our claim

        replied = trio.Event()
        replied_ok = False

        def _on_reply(event, user_id):
            nonlocal replied_ok
            if user_id == invitation.user_id:
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
        return {"status": "ok"}

    async def api_user_cancel_invitation(self, client_ctx, msg):
        msg = user_cancel_invitation_req_schema.load_or_abort(msg)

        await self.user_cancel_invitation(msg["user_id"])

        self.event_bus.send("user.invitation.cancelled", user_id=msg["user_id"])

        return {"status": "ok"}

    async def api_user_create(self, client_ctx, msg):
        msg = user_create_req_schema.load_or_abort(msg)

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
            u_data = validate_payload_certified_user(
                client_ctx.verify_key, u_payload, pendulum.now()
            )
            d_data = validate_payload_certified_device(
                client_ctx.verify_key, d_payload, pendulum.now()
            )
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
            await self.create_user(user)
        except AlreadyExistsError as exc:
            return {"status": "already_exists", "reason": str(exc)}

        self.event_bus.send("user.created", user_id=user.user_id)
        return {"status": "ok"}

    async def api_user_revoke(self, client_ctx, msg):
        msg = user_revoke_req_schema.load_or_abort(msg)

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
            data = validate_payload_certified_user_revocation(
                client_ctx.verify_key, payload, pendulum.now()
            )
        except TrustChainError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        try:
            await self.revoke_user(data["user_id"], msg["certified_revocation"], certifier_id)

        except NotFoundError:
            return {"status": "not_found"}

        except AlreadyRevokedError:
            return {
                "status": "already_revoked",
                "reason": f"User `{data['user_id']}` already revoked",
            }

        return {"status": "ok"}

    #### Device creation API ####

    async def api_device_invite(self, client_ctx, msg):
        msg = device_invite_req_schema.load_or_abort(msg)
        if msg["device_id"].user_id != client_ctx.user_id:
            return {"status": "bad_user_id", "reason": "Device must be handled by it own user."}

        invitation = DeviceInvitation(msg["device_id"], client_ctx.device_id)
        try:
            await self.create_device_invitation(invitation)

        except AlreadyExistsError as exc:
            return {"status": "already_exists", "reason": str(exc)}

        # Wait for invited user to send `user_claim`

        claim_answered = trio.Event()
        _encrypted_claim = None

        def _on_device_claimed(event, device_id, encrypted_claim):
            nonlocal _encrypted_claim
            if device_id == invitation.device_id:
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
        return {"status": "ok", "encrypted_claim": _encrypted_claim}

    @anonymous_api
    async def api_device_get_invitation_creator(self, client_ctx, msg):
        msg = device_get_invitation_creator_req_schema.load_or_abort(msg)
        try:
            invitation = await self.get_device_invitation(msg["invited_device_id"])
            if not invitation.is_valid():
                return {"status": "not_found"}
            certifier = await self.get_device(invitation.creator)

        except NotFoundError:
            return {"status": "not_found"}

        data, errors = device_get_invitation_creator_rep_schema.dump(certifier)

        if errors:
            raise RuntimeError(f"Dump error with {certifier!r}: {errors}")
        return data

    @anonymous_api
    async def api_device_claim(self, client_ctx, msg):
        msg = device_claim_req_schema.load_or_abort(msg)

        try:
            invitation = await self.get_device_invitation(msg["invited_device_id"])
            if not invitation.is_valid():
                return {"status": "not_found"}

        except NotFoundError:
            return {"status": "not_found"}

        self.event_bus.send(
            "device.claimed", device_id=invitation.device_id, encrypted_claim=msg["encrypted_claim"]
        )

        # Wait for creator device to accept (or refuse) our claim

        replied = trio.Event()
        replied_ok = False
        replied_encrypted_answer = None

        def _on_reply(event, device_id, encrypted_answer=None):
            nonlocal replied_ok, replied_encrypted_answer
            if device_id == invitation.device_id:
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

        raw_data = {"status": "ok", "encrypted_answer": replied_encrypted_answer}
        data, errors = device_claim_rep_schema.dump(raw_data)

        if errors:
            raise RuntimeError(f"Dump error with {raw_data!r}: {errors}")
        return data

    async def api_device_cancel_invitation(self, client_ctx, msg):
        msg = device_cancel_invitation_req_schema.load_or_abort(msg)
        if msg["device_id"].user_id != client_ctx.user_id:
            return {"status": "bad_user_id", "reason": "Device must be handled by it own user."}

        await self.device_cancel_invitation(msg["device_id"])

        self.event_bus.send("device.invitation.cancelled", device_id=msg["device_id"])

        return {"status": "ok"}

    async def api_device_create(self, client_ctx, msg):
        msg = device_create_req_schema.load_or_abort(msg)

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
            await self.create_device(device)
        except AlreadyExistsError as exc:
            return {"status": "already_exists", "reason": str(exc)}

        self.event_bus.send(
            "device.created", device_id=device.device_id, encrypted_answer=msg["encrypted_answer"]
        )
        return {"status": "ok"}

    async def api_device_revoke(self, client_ctx, msg):
        msg = device_revoke_req_schema.load_or_abort(msg)

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

        try:
            await self.revoke_device(data["device_id"], msg["certified_revocation"], certifier_id)

        except NotFoundError:
            return {"status": "not_found"}

        except AlreadyRevokedError:
            return {
                "status": "already_revoked",
                "reason": f"Device `{data['device_id']}` already revoked",
            }

        return {"status": "ok"}

    #### Virtual methods ####

    async def create_user(self, user: User) -> None:
        """
        Raises:
            AlreadyExistsError
        """
        raise NotImplementedError()

    async def create_device(self, device: Device) -> None:
        """
        Raises:
            AlreadyExistsError
        """
        raise NotImplementedError()

    async def get_user(self, user_id: UserID) -> User:
        """
        Raises:
            NotFoundError
        """
        raise NotImplementedError()

    async def get_device(self, device_id: DeviceID) -> Device:
        """
        Raises:
            NotFoundError
        """
        raise NotImplementedError()

    async def find(
        self, query: str = None, page: int = 1, per_page: int = 100
    ) -> Tuple[List[UserID], int]:
        raise NotImplementedError()

    async def create_user_invitation(self, invitation: UserInvitation) -> None:
        """
        Raises:
            AlreadyExistsError
        """
        raise NotImplementedError()

    async def get_user_invitation(self, user_id: UserID) -> UserInvitation:
        """
        Raises:
            NotFoundError
        """

        raise NotImplementedError()

    async def user_cancel_invitation(self, user_id: UserID) -> None:
        """
        Raises: Nothing
        """
        raise NotImplementedError()

    async def create_device_invitation(self, invitation: DeviceInvitation) -> None:
        """
        Raises:
            AlreadyExistsError
            NotFoundError
        """
        raise NotImplementedError()

    async def get_device_invitation(self, user_id: UserID) -> DeviceInvitation:
        """
        Raises:
            NotFoundError
        """

        raise NotImplementedError()

    async def device_cancel_invitation(self, device_id: DeviceID) -> None:
        """
        Raises: Nothing
        """
        raise NotImplementedError()

    async def revokeuser(
        self, user_id: UserID, certified_revocation: bytes, revocation_certifier: DeviceID
    ) -> None:
        raise NotImplementedError()

    async def revokedevice(
        self, device_id: DeviceID, certified_revocation: bytes, revocation_certifier: DeviceID
    ) -> None:
        raise NotImplementedError()

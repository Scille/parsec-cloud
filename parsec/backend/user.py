# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

import attr
from typing import List, Optional, Tuple
import pendulum

from parsec.utils import timestamps_in_the_ballpark
from parsec.crypto import VerifyKey, PublicKey
from parsec.event_bus import EventBus
from parsec.api.data import (
    UserProfile,
    UserCertificateContent,
    DeviceCertificateContent,
    RevokedUserCertificateContent,
    DataError,
)
from parsec.api.protocol import (
    OrganizationID,
    UserID,
    DeviceID,
    HumanHandle,
    HandshakeType,
    user_get_serializer,
    human_find_serializer,
    user_create_serializer,
    user_revoke_serializer,
    device_create_serializer,
)
from parsec.backend.utils import catch_protocol_errors, api


class UserError(Exception):
    pass


class UserNotFoundError(UserError):
    pass


class UserAlreadyExistsError(UserError):
    pass


class UserAlreadyRevokedError(UserError):
    pass


class UserActiveUsersLimitReached(UserError):
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
        return DeviceCertificateContent.unsecure_load(self.device_certificate).verify_key

    device_id: DeviceID
    device_label: Optional[str]
    device_certificate: bytes
    redacted_device_certificate: bytes
    device_certifier: Optional[DeviceID]
    created_on: pendulum.DateTime = attr.ib(factory=pendulum.now)


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class User:
    def __repr__(self):
        return f"{self.__class__.__name__}({self.user_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    def is_revoked(self):
        return self.revoked_on and self.revoked_on <= pendulum.now()

    @property
    def public_key(self) -> PublicKey:
        return UserCertificateContent.unsecure_load(self.user_certificate).public_key

    user_id: UserID
    human_handle: Optional[HumanHandle]
    user_certificate: bytes
    redacted_user_certificate: bytes
    user_certifier: Optional[DeviceID]
    profile: UserProfile = UserProfile.STANDARD
    created_on: pendulum.DateTime = attr.ib(factory=pendulum.now)
    revoked_on: Optional[pendulum.DateTime] = None
    revoked_user_certificate: Optional[bytes] = None
    revoked_user_certifier: Optional[DeviceID] = None


@attr.s(slots=True, frozen=True, auto_attribs=True)
class Trustchain:
    users: Tuple[bytes, ...]
    revoked_users: Tuple[bytes, ...]
    devices: Tuple[bytes, ...]


@attr.s(slots=True, auto_attribs=True)
class GetUserAndDevicesResult:
    user_certificate: bytes
    device_certificates: Tuple[bytes, ...]
    revoked_user_certificate: Optional[bytes]
    trustchain_user_certificates: Tuple[bytes, ...]
    trustchain_device_certificates: Tuple[bytes, ...]
    trustchain_revoked_user_certificates: Tuple[bytes, ...]


@attr.s(slots=True, frozen=True, auto_attribs=True)
class HumanFindResultItem:
    user_id: UserID
    revoked: bool
    human_handle: Optional[HumanHandle] = None


class BaseUserComponent:
    def __init__(self, event_bus: EventBus):
        self._event_bus = event_bus

    #### Access user API ####

    @api("user_get")
    @catch_protocol_errors
    async def api_user_get(self, client_ctx, msg):
        msg = user_get_serializer.req_load(msg)
        need_redacted = client_ctx.profile == UserProfile.OUTSIDER

        try:
            result = await self.get_user_with_devices_and_trustchain(
                client_ctx.organization_id, msg["user_id"], redacted=need_redacted
            )
        except UserNotFoundError:
            return {"status": "not_found"}

        return user_get_serializer.rep_dump(
            {
                "status": "ok",
                "user_certificate": result.user_certificate,
                "revoked_user_certificate": result.revoked_user_certificate,
                "device_certificates": result.device_certificates,
                "trustchain": {
                    "devices": result.trustchain_device_certificates,
                    "users": result.trustchain_user_certificates,
                    "revoked_users": result.trustchain_revoked_user_certificates,
                },
            }
        )

    @api("human_find", handshake_types=[HandshakeType.AUTHENTICATED])
    @catch_protocol_errors
    async def api_human_find(self, client_ctx, msg):
        if client_ctx.profile == UserProfile.OUTSIDER:
            return {
                "status": "not_allowed",
                "reason": "Not allowed for user with OUTSIDER profile.",
            }

        msg = human_find_serializer.req_load(msg)
        results, total = await self.find_humans(client_ctx.organization_id, **msg)
        return human_find_serializer.rep_dump(
            {
                "status": "ok",
                "results": results,
                "page": msg["page"],
                "per_page": msg["per_page"],
                "total": total,
            }
        )

    @api("user_create", handshake_types=[HandshakeType.AUTHENTICATED])
    @catch_protocol_errors
    async def api_user_create(self, client_ctx, msg):
        if client_ctx.profile != UserProfile.ADMIN:
            return {
                "status": "not_allowed",
                "reason": f"User `{client_ctx.device_id.user_id}` is not admin",
            }
        msg = user_create_serializer.req_load(msg)
        rep = await self._api_user_create(client_ctx, msg)
        return user_create_serializer.rep_dump(rep)

    async def _api_user_create(self, client_ctx, msg):
        try:
            d_data = DeviceCertificateContent.verify_and_load(
                msg["device_certificate"],
                author_verify_key=client_ctx.verify_key,
                expected_author=client_ctx.device_id,
            )
            u_data = UserCertificateContent.verify_and_load(
                msg["user_certificate"],
                author_verify_key=client_ctx.verify_key,
                expected_author=client_ctx.device_id,
            )
            ru_data = UserCertificateContent.verify_and_load(
                msg["redacted_user_certificate"],
                author_verify_key=client_ctx.verify_key,
                expected_author=client_ctx.device_id,
            )
            rd_data = DeviceCertificateContent.verify_and_load(
                msg["redacted_device_certificate"],
                author_verify_key=client_ctx.verify_key,
                expected_author=client_ctx.device_id,
            )

        except DataError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        if u_data.timestamp != d_data.timestamp:
            return {
                "status": "invalid_data",
                "reason": "Device and User certificates must have the same timestamp.",
            }

        now = pendulum.now()
        if not timestamps_in_the_ballpark(u_data.timestamp, now):
            return {
                "status": "invalid_certification",
                "reason": f"Invalid timestamp in certificate.",
            }

        if u_data.user_id != d_data.device_id.user_id:
            return {
                "status": "invalid_data",
                "reason": "Device and User must have the same user ID.",
            }

        if ru_data.evolve(human_handle=u_data.human_handle) != u_data:
            return {
                "status": "invalid_data",
                "reason": "Redacted User certificate differs from User certificate.",
            }
        if ru_data.human_handle:
            return {
                "status": "invalid_data",
                "reason": "Redacted User certificate must not contain a human_handle field.",
            }

        if rd_data.evolve(device_label=d_data.device_label) != d_data:
            return {
                "status": "invalid_data",
                "reason": "Redacted Device certificate differs from Device certificate.",
            }
        if rd_data.device_label:
            return {
                "status": "invalid_data",
                "reason": "Redacted Device certificate must not contain a device_label field.",
            }

        try:
            user = User(
                user_id=u_data.user_id,
                human_handle=u_data.human_handle,
                profile=u_data.profile,
                user_certificate=msg["user_certificate"],
                redacted_user_certificate=msg["redacted_user_certificate"]
                or msg["user_certificate"],
                user_certifier=u_data.author,
                created_on=u_data.timestamp,
            )
            first_device = Device(
                device_id=d_data.device_id,
                device_label=d_data.device_label,
                device_certificate=msg["device_certificate"],
                redacted_device_certificate=msg["redacted_device_certificate"]
                or msg["device_certificate"],
                device_certifier=d_data.author,
                created_on=d_data.timestamp,
            )
            await self.create_user(client_ctx.organization_id, user, first_device)

        except UserAlreadyExistsError as exc:
            return {"status": "already_exists", "reason": str(exc)}
        except UserActiveUsersLimitReached:
            return {"status": "active_users_limit_reached"}

        return {"status": "ok"}

    @api("user_revoke")
    @catch_protocol_errors
    async def api_user_revoke(self, client_ctx, msg):
        if client_ctx.profile != UserProfile.ADMIN:
            return {
                "status": "not_allowed",
                "reason": f"User `{client_ctx.device_id.user_id}` is not admin",
            }

        msg = user_revoke_serializer.req_load(msg)

        try:
            data = RevokedUserCertificateContent.verify_and_load(
                msg["revoked_user_certificate"],
                author_verify_key=client_ctx.verify_key,
                expected_author=client_ctx.device_id,
            )

        except DataError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        if not timestamps_in_the_ballpark(data.timestamp, pendulum.now()):
            return {
                "status": "invalid_certification",
                "reason": f"Invalid timestamp in certification.",
            }

        if data.user_id == client_ctx.user_id:
            return {"status": "not_allowed", "reason": "Cannot do self-revocation"}

        try:
            await self.revoke_user(
                organization_id=client_ctx.organization_id,
                user_id=data.user_id,
                revoked_user_certificate=msg["revoked_user_certificate"],
                revoked_user_certifier=data.author,
                revoked_on=data.timestamp,
            )

        except UserNotFoundError:
            return {"status": "not_found"}

        except UserAlreadyRevokedError:
            return {"status": "already_revoked", "reason": f"User `{data.user_id}` already revoked"}

        return user_revoke_serializer.rep_dump({"status": "ok"})

    @api("device_create", handshake_types=[HandshakeType.AUTHENTICATED])
    @catch_protocol_errors
    async def api_device_create(self, client_ctx, msg):
        msg = device_create_serializer.req_load(msg)

        try:
            data = DeviceCertificateContent.verify_and_load(
                msg["device_certificate"],
                author_verify_key=client_ctx.verify_key,
                expected_author=client_ctx.device_id,
            )

            redacted_data = None
            if msg["redacted_device_certificate"]:
                redacted_data = DeviceCertificateContent.verify_and_load(
                    msg["redacted_device_certificate"],
                    author_verify_key=client_ctx.verify_key,
                    expected_author=client_ctx.device_id,
                )

        except DataError as exc:
            return {
                "status": "invalid_certification",
                "reason": f"Invalid certification data ({exc}).",
            }

        if not timestamps_in_the_ballpark(data.timestamp, pendulum.now()):
            return {
                "status": "invalid_certification",
                "reason": f"Invalid timestamp in certification.",
            }

        if data.device_id.user_id != client_ctx.user_id:
            return {"status": "bad_user_id", "reason": "Device must be handled by it own user."}

        if redacted_data:
            if redacted_data.evolve(device_label=data.device_label) != data:
                return {
                    "status": "invalid_data",
                    "reason": "Redacted Device certificate differs from Device certificate.",
                }
            if redacted_data.device_label:
                return {
                    "status": "invalid_data",
                    "reason": "Redacted Device certificate must not contain a device_label field.",
                }

        try:
            device = Device(
                device_id=data.device_id,
                device_label=data.device_label,
                device_certificate=msg["device_certificate"],
                redacted_device_certificate=msg["redacted_device_certificate"]
                or msg["device_certificate"],
                device_certifier=data.author,
                created_on=data.timestamp,
            )
            await self.create_device(client_ctx.organization_id, device)
        except UserAlreadyExistsError as exc:
            return {"status": "already_exists", "reason": str(exc)}

        return device_create_serializer.rep_dump({"status": "ok"})

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

    async def revoke_user(
        self,
        organization_id: OrganizationID,
        user_id: UserID,
        revoked_user_certificate: bytes,
        revoked_user_certifier: DeviceID,
        revoked_on: Optional[pendulum.DateTime] = None,
    ) -> None:
        """
        Raises:
            UserNotFoundError
            UserAlreadyRevokedError
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
    ) -> Tuple[User, Trustchain]:
        """
        Raises:
            UserNotFoundError
        """
        raise NotImplementedError()

    async def get_user_with_device_and_trustchain(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> Tuple[User, Device, Trustchain]:
        """
        Raises:
            UserNotFoundError
        """
        raise NotImplementedError()

    async def get_user_with_devices_and_trustchain(
        self, organization_id: OrganizationID, user_id: UserID, redacted: bool = False
    ) -> GetUserAndDevicesResult:
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

    async def find_humans(
        self,
        organization_id: OrganizationID,
        query: Optional[str] = None,
        page: int = 1,
        per_page: int = 100,
        omit_revoked: bool = False,
        omit_non_human: bool = False,
    ) -> Tuple[List[HumanFindResultItem], int]:
        raise NotImplementedError()

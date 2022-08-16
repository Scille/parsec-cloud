# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import attr
from typing import List, Optional, Tuple
from parsec._parsec import DateTime

from parsec.utils import timestamps_in_the_ballpark
from parsec.event_bus import EventBus
from parsec.api.data import UserProfile, RevokedUserCertificateContent, DataError
from parsec.api.protocol import (
    OrganizationID,
    UserID,
    DeviceID,
    HumanHandle,
    user_get_serializer,
    human_find_serializer,
    user_create_serializer,
    user_revoke_serializer,
    device_create_serializer,
)
from parsec.backend.utils import catch_protocol_errors, api
from parsec.backend.user_type import (
    User,
    Device,
    validate_new_user_certificates,
    validate_new_device_certificate,
    CertificateValidationError,
)


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


class UserCertifValidationError(UserError):
    pass


class UserInvalidCertificationError(UserCertifValidationError):
    status = "invalid_certification"


class UserInvalidDataError(UserCertifValidationError):
    status = "invalid_data"


PEER_EVENT_MAX_WAIT = 300
INVITATION_VALIDITY = 3600


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

    @api("human_find")
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

    @api("user_create")
    @catch_protocol_errors
    async def api_user_create(self, client_ctx, msg):
        if client_ctx.profile != UserProfile.ADMIN:
            return user_create_serializer.rep_dump(
                {
                    "status": "not_allowed",
                    "reason": f"User `{client_ctx.device_id.user_id}` is not admin",
                }
            )
        msg = user_create_serializer.req_load(msg)

        try:
            user, first_device = validate_new_user_certificates(
                expected_author=client_ctx.device_id,
                author_verify_key=client_ctx.verify_key,
                device_certificate=msg["device_certificate"],
                user_certificate=msg["user_certificate"],
                redacted_user_certificate=msg["redacted_user_certificate"],
                redacted_device_certificate=msg["redacted_device_certificate"],
            )
            await self.create_user(client_ctx.organization_id, user, first_device)
            rep = {"status": "ok"}

        except CertificateValidationError as exc:
            rep = {"status": exc.status, "reason": exc.reason}

        except UserAlreadyExistsError as exc:
            rep = {"status": "already_exists", "reason": str(exc)}

        except UserActiveUsersLimitReached:
            rep = {"status": "active_users_limit_reached"}

        return user_create_serializer.rep_dump(rep)

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

        if not timestamps_in_the_ballpark(data.timestamp, DateTime.now()):
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

    @api("device_create")
    @catch_protocol_errors
    async def api_device_create(self, client_ctx, msg):
        msg = device_create_serializer.req_load(msg)

        try:
            device = validate_new_device_certificate(
                expected_author=client_ctx.device_id,
                author_verify_key=client_ctx.verify_key,
                device_certificate=msg["device_certificate"],
                redacted_device_certificate=msg["redacted_device_certificate"],
            )
            await self.create_device(client_ctx.organization_id, device)
            rep = {"status": "ok"}

        except CertificateValidationError as exc:
            rep = {"status": exc.status, "reason": exc.reason}

        except UserAlreadyExistsError as exc:
            rep = {"status": "already_exists", "reason": str(exc)}

        return device_create_serializer.rep_dump(rep)

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
        revoked_on: Optional[DateTime] = None,
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
        """
        Raises: Nothing !
        """
        raise NotImplementedError()

    async def dump_users(self, organization_id: OrganizationID) -> Tuple[List[User], List[Device]]:
        """
        Raises: Nothing !
        """
        raise NotImplementedError()

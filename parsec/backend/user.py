# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import List, Tuple

import attr

from parsec._parsec import (
    DateTime,
    DeviceCreateRep,
    DeviceCreateRepAlreadyExists,
    DeviceCreateRepBadUserId,
    DeviceCreateRepInvalidCertification,
    DeviceCreateRepInvalidData,
    DeviceCreateRepOk,
    DeviceCreateReq,
    HumanFindRep,
    HumanFindRepNotAllowed,
    HumanFindRepOk,
    HumanFindReq,
    HumanFindResultItem,
    Trustchain,
    UserCreateRep,
    UserCreateRepActiveUsersLimitReached,
    UserCreateRepAlreadyExists,
    UserCreateRepInvalidCertification,
    UserCreateRepInvalidData,
    UserCreateRepNotAllowed,
    UserCreateRepOk,
    UserCreateReq,
    UserGetRep,
    UserGetRepNotFound,
    UserGetRepOk,
    UserGetReq,
    UserRevokeRep,
    UserRevokeRepAlreadyRevoked,
    UserRevokeRepInvalidCertification,
    UserRevokeRepNotAllowed,
    UserRevokeRepNotFound,
    UserRevokeRepOk,
    UserRevokeReq,
)
from parsec.api.data import DataError, RevokedUserCertificate
from parsec.api.protocol import DeviceID, OrganizationID, UserID, UserProfile
from parsec.backend.client_context import AuthenticatedClientContext
from parsec.backend.user_type import (
    CertificateValidationError,
    Device,
    User,
    validate_new_device_certificate,
    validate_new_user_certificates,
)
from parsec.backend.utils import api, api_typed_msg_adapter, catch_protocol_errors
from parsec.event_bus import EventBus
from parsec.utils import timestamps_in_the_ballpark


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


@attr.s(slots=True, auto_attribs=True)
class GetUserAndDevicesResult:
    user_certificate: bytes
    device_certificates: Tuple[bytes, ...]
    revoked_user_certificate: bytes | None
    trustchain_user_certificates: Tuple[bytes, ...]
    trustchain_device_certificates: Tuple[bytes, ...]
    trustchain_revoked_user_certificates: Tuple[bytes, ...]


class BaseUserComponent:
    def __init__(self, event_bus: EventBus):
        self._event_bus = event_bus

    #### Access user API ####

    @api("user_get")
    @catch_protocol_errors
    @api_typed_msg_adapter(UserGetReq, UserGetRep)
    async def api_user_get(
        self, client_ctx: AuthenticatedClientContext, req: UserGetReq
    ) -> UserGetRep:
        need_redacted = client_ctx.profile == UserProfile.OUTSIDER

        try:
            result = await self.get_user_with_devices_and_trustchain(
                client_ctx.organization_id, req.user_id, redacted=need_redacted
            )
        except UserNotFoundError:
            return UserGetRepNotFound()

        return UserGetRepOk(
            user_certificate=result.user_certificate,
            revoked_user_certificate=result.revoked_user_certificate,
            device_certificates=list(result.device_certificates),
            trustchain=Trustchain(
                devices=list(result.trustchain_device_certificates),
                users=list(result.trustchain_user_certificates),
                revoked_users=list(result.trustchain_revoked_user_certificates),
            ),
        )

    @api("human_find")
    @catch_protocol_errors
    @api_typed_msg_adapter(HumanFindReq, HumanFindRep)
    async def api_human_find(
        self, client_ctx: AuthenticatedClientContext, req: HumanFindReq
    ) -> HumanFindRep:
        if client_ctx.profile == UserProfile.OUTSIDER:
            return HumanFindRepNotAllowed(None)
        results, total = await self.find_humans(
            client_ctx.organization_id,
            omit_non_human=req.omit_non_human,
            omit_revoked=req.omit_revoked,
            page=req.page,
            per_page=req.per_page,
            query=req.query,
        )
        return HumanFindRepOk(
            results=results,
            page=req.page,
            per_page=req.per_page,
            total=total,
        )

    @api("user_create")
    @catch_protocol_errors
    @api_typed_msg_adapter(UserCreateReq, UserCreateRep)
    async def api_user_create(
        self, client_ctx: AuthenticatedClientContext, req: UserCreateReq
    ) -> UserCreateRep:
        if client_ctx.profile != UserProfile.ADMIN:
            return UserCreateRepNotAllowed(None)

        try:
            user, first_device = validate_new_user_certificates(
                expected_author=client_ctx.device_id,
                author_verify_key=client_ctx.verify_key,
                device_certificate=req.device_certificate,
                user_certificate=req.user_certificate,
                redacted_user_certificate=req.redacted_user_certificate,
                redacted_device_certificate=req.redacted_device_certificate,
            )
            await self.create_user(client_ctx.organization_id, user, first_device)

        except CertificateValidationError as exc:
            if exc.status == "invalid_certification":
                return UserCreateRepInvalidCertification(None)
            elif exc.status == "invalid_data":
                return UserCreateRepInvalidData(None)
            elif exc.status == "not_allowed":
                return UserCreateRepNotAllowed(None)

        except UserAlreadyExistsError:
            return UserCreateRepAlreadyExists(None)

        except UserActiveUsersLimitReached:
            return UserCreateRepActiveUsersLimitReached(None)

        return UserCreateRepOk()

    @api("user_revoke")
    @catch_protocol_errors
    @api_typed_msg_adapter(UserRevokeReq, UserRevokeRep)
    async def api_user_revoke(
        self, client_ctx: AuthenticatedClientContext, req: UserRevokeReq
    ) -> UserRevokeRep:
        if client_ctx.profile != UserProfile.ADMIN:
            return UserRevokeRepNotAllowed(None)

        try:
            data = RevokedUserCertificate.verify_and_load(
                signed=req.revoked_user_certificate,
                author_verify_key=client_ctx.verify_key,
                expected_author=client_ctx.device_id,
            )

        except DataError:
            return UserRevokeRepInvalidCertification(None)

        if not timestamps_in_the_ballpark(data.timestamp, DateTime.now()):
            return UserRevokeRepInvalidCertification(None)

        if data.user_id == client_ctx.user_id:
            return UserRevokeRepNotAllowed(None)

        try:
            await self.revoke_user(
                organization_id=client_ctx.organization_id,
                user_id=data.user_id,
                revoked_user_certificate=req.revoked_user_certificate,
                revoked_user_certifier=data.author,
                revoked_on=data.timestamp,
            )

        except UserNotFoundError:
            return UserRevokeRepNotFound()

        except UserAlreadyRevokedError:
            return UserRevokeRepAlreadyRevoked(None)

        return UserRevokeRepOk()

    @api("device_create")
    @catch_protocol_errors
    @api_typed_msg_adapter(DeviceCreateReq, DeviceCreateRep)
    async def api_device_create(
        self, client_ctx: AuthenticatedClientContext, req: DeviceCreateReq
    ) -> DeviceCreateRep:
        try:
            device = validate_new_device_certificate(
                expected_author=client_ctx.device_id,
                author_verify_key=client_ctx.verify_key,
                device_certificate=req.device_certificate,
                redacted_device_certificate=req.redacted_device_certificate,
            )
            await self.create_device(client_ctx.organization_id, device)

        except CertificateValidationError as exc:
            if exc.status == "bad_user_id":
                return DeviceCreateRepBadUserId(None)
            elif exc.status == "invalid_certification":
                return DeviceCreateRepInvalidCertification(None)
            elif exc.status == "invalid_data":
                return DeviceCreateRepInvalidData(None)

        except UserAlreadyExistsError:
            return DeviceCreateRepAlreadyExists(None)

        return DeviceCreateRepOk()

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
        self,
        organization_id: OrganizationID,
        device: Device,
        encrypted_answer: bytes = b"",
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
        revoked_on: DateTime | None = None,
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
        query: str | None = None,
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

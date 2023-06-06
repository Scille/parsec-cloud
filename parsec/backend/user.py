# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from __future__ import annotations

from typing import List, Tuple

import attr

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    UserID,
    UserProfile,
    authenticated_cmds,
)
from parsec.api.data import DataError, RevokedUserCertificate, UserUpdateCertificate
from parsec.backend.client_context import AuthenticatedClientContext
from parsec.backend.user_type import (
    CertificateValidationError,
    Device,
    User,
    validate_new_device_certificate,
    validate_new_user_certificates,
)
from parsec.backend.utils import api
from parsec.event_bus import EventBus
from parsec.utils import (
    BALLPARK_CLIENT_EARLY_OFFSET,
    BALLPARK_CLIENT_LATE_OFFSET,
    timestamps_in_the_ballpark,
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


class UserRequireGreaterTimestampError(UserError):
    @property
    def strictly_greater_than(self) -> DateTime:
        return self.args[0]


class UserCertifValidationError(UserError):
    pass


class UserInvalidCertificationError(UserCertifValidationError):
    status = "invalid_certification"


class UserInvalidDataError(UserCertifValidationError):
    status = "invalid_data"


HumanFindResultItem = authenticated_cmds.v3.human_find.HumanFindResultItem
Trustchain = authenticated_cmds.v3.user_get.Trustchain


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

    @api
    async def api_certificate_get(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.certificate_get.Req,
    ) -> authenticated_cmds.latest.certificate_get.Rep:
        need_redacted = client_ctx.profile == UserProfile.OUTSIDER
        certifs = await self.get_certificates(
            client_ctx.organization_id, offset=req.offset, redacted=need_redacted
        )

        return authenticated_cmds.latest.certificate_get.RepOk(certificates=certifs)

    @api
    async def api_user_update(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.user_update.Req
    ) -> authenticated_cmds.latest.user_update.Rep:
        if client_ctx.profile != UserProfile.ADMIN:
            return authenticated_cmds.latest.user_update.RepNotAllowed()

        certif = req.user_update_certificate
        try:
            data = UserUpdateCertificate.verify_and_load(
                signed=certif,
                author_verify_key=client_ctx.verify_key,
                expected_author=client_ctx.device_id,
            )

        except DataError:
            return authenticated_cmds.latest.user_update.RepInvalidCertification()

        now = DateTime.now()
        if not timestamps_in_the_ballpark(data.timestamp, now):
            return authenticated_cmds.latest.user_update.RepBadTimestamp(
                ballpark_client_early_offset=BALLPARK_CLIENT_EARLY_OFFSET,
                ballpark_client_late_offset=BALLPARK_CLIENT_LATE_OFFSET,
                backend_timestamp=now,
                client_timestamp=data.timestamp,
            )

        if data.user_id == client_ctx.user_id:
            return authenticated_cmds.latest.user_update.RepNotAllowed()

        # TODO: user that get there role update should get disconnected to force update
        # the need_redacted

        try:
            await self.update_user(
                organization_id=client_ctx.organization_id,
                user_id=data.user_id,
                new_profile=data.new_profile,
                user_update_certificate=certif,
                user_update_certifier=client_ctx.device_id,
                updated_on=data.timestamp,
            )

        except UserAlreadyExistsError:
            return authenticated_cmds.latest.user_update.RepAlreadyExists()

        except UserRequireGreaterTimestampError as exc:
            return authenticated_cmds.latest.user_update.RepRequireGreaterTimestamp(
                exc.strictly_greater_than
            )

        return authenticated_cmds.latest.user_update.RepOk()

    @api
    async def apiv2v3_user_get(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.v3.user_get.Req
    ) -> authenticated_cmds.v3.user_get.Rep:
        need_redacted = client_ctx.profile == UserProfile.OUTSIDER

        try:
            result = await self.get_user_with_devices_and_trustchain(
                client_ctx.organization_id, req.user_id, redacted=need_redacted
            )
        except UserNotFoundError:
            return authenticated_cmds.v3.user_get.RepNotFound()

        return authenticated_cmds.v3.user_get.RepOk(
            user_certificate=result.user_certificate,
            revoked_user_certificate=result.revoked_user_certificate,
            device_certificates=list(result.device_certificates),
            trustchain=authenticated_cmds.v3.user_get.Trustchain(
                devices=list(result.trustchain_device_certificates),
                users=list(result.trustchain_user_certificates),
                revoked_users=list(result.trustchain_revoked_user_certificates),
            ),
        )

    @api
    async def apiv2v3_human_find(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.v3.human_find.Req
    ) -> authenticated_cmds.v3.human_find.Rep:
        if client_ctx.profile == UserProfile.OUTSIDER:
            return authenticated_cmds.v3.human_find.RepNotAllowed(None)
        results, total = await self.find_humans(
            client_ctx.organization_id,
            omit_non_human=req.omit_non_human,
            omit_revoked=req.omit_revoked,
            page=req.page,
            per_page=req.per_page,
            query=req.query,
        )
        return authenticated_cmds.v3.human_find.RepOk(
            results=results,
            page=req.page,
            per_page=req.per_page,
            total=total,
        )

    @api
    async def api_user_create(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.user_create.Req
    ) -> authenticated_cmds.latest.user_create.Rep:
        if client_ctx.profile != UserProfile.ADMIN:
            return authenticated_cmds.latest.user_create.RepNotAllowed(None)

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
                return authenticated_cmds.latest.user_create.RepInvalidCertification(None)
            elif exc.status == "invalid_data":
                return authenticated_cmds.latest.user_create.RepInvalidData(None)
            elif exc.status == "not_allowed":
                return authenticated_cmds.latest.user_create.RepNotAllowed(None)

        except UserAlreadyExistsError:
            return authenticated_cmds.latest.user_create.RepAlreadyExists(None)

        except UserActiveUsersLimitReached:
            return authenticated_cmds.latest.user_create.RepActiveUsersLimitReached(None)

        return authenticated_cmds.latest.user_create.RepOk()

    @api
    async def api_user_revoke(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.user_revoke.Req
    ) -> authenticated_cmds.latest.user_revoke.Rep:
        if client_ctx.profile != UserProfile.ADMIN:
            return authenticated_cmds.latest.user_revoke.RepNotAllowed(None)

        try:
            data = RevokedUserCertificate.verify_and_load(
                signed=req.revoked_user_certificate,
                author_verify_key=client_ctx.verify_key,
                expected_author=client_ctx.device_id,
            )

        except DataError:
            return authenticated_cmds.latest.user_revoke.RepInvalidCertification(None)

        if not timestamps_in_the_ballpark(data.timestamp, DateTime.now()):
            return authenticated_cmds.latest.user_revoke.RepInvalidCertification(None)

        if data.user_id == client_ctx.user_id:
            return authenticated_cmds.latest.user_revoke.RepNotAllowed(None)

        try:
            await self.revoke_user(
                organization_id=client_ctx.organization_id,
                user_id=data.user_id,
                revoked_user_certificate=req.revoked_user_certificate,
                revoked_user_certifier=data.author,
                revoked_on=data.timestamp,
            )

        except UserNotFoundError:
            return authenticated_cmds.latest.user_revoke.RepNotFound()

        except UserAlreadyRevokedError:
            return authenticated_cmds.latest.user_revoke.RepAlreadyRevoked(None)

        return authenticated_cmds.latest.user_revoke.RepOk()

    @api
    async def api_device_create(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.device_create.Req,
    ) -> authenticated_cmds.latest.device_create.Rep:
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
                return authenticated_cmds.latest.device_create.RepBadUserId(None)
            elif exc.status == "invalid_certification":
                return authenticated_cmds.latest.device_create.RepInvalidCertification(None)
            elif exc.status == "invalid_data":
                return authenticated_cmds.latest.device_create.RepInvalidData(None)

        except UserAlreadyExistsError:
            return authenticated_cmds.latest.device_create.RepAlreadyExists(None)

        return authenticated_cmds.latest.device_create.RepOk()

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

    async def update_user(
        self,
        organization_id: OrganizationID,
        user_id: UserID,
        new_profile: UserProfile,
        user_update_certificate: bytes,
        user_update_certifier: DeviceID,
        updated_on: DateTime | None = None,
    ) -> None:
        """
        Raises:
            UserNotFoundError
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

    async def get_certificates(
        self, organization_id: OrganizationID, offset: int, redacted: bool
    ) -> list[bytes]:
        """
        Raises: Nothing !
        """
        raise NotImplementedError()

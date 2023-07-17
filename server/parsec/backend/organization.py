# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from secrets import token_hex
from typing import Any, Union

import attr

from parsec._parsec import (
    ActiveUsersLimit,
    DateTime,
    OrganizationID,
    OrganizationStats,
    SequesterVerifyKeyDer,
    UserProfile,
    VerifyKey,
    anonymous_cmds,
    authenticated_cmds,
)
from parsec.api.data import (
    DataError,
    DeviceCertificate,
    SequesterAuthorityCertificate,
    UserCertificate,
)
from parsec.backend.client_context import AnonymousClientContext, AuthenticatedClientContext
from parsec.backend.config import BackendConfig
from parsec.backend.user import Device, User
from parsec.backend.utils import Unset, UnsetType, api
from parsec.backend.webhooks import WebhooksComponent
from parsec.utils import (
    BALLPARK_CLIENT_EARLY_OFFSET,
    BALLPARK_CLIENT_LATE_OFFSET,
    timestamps_in_the_ballpark,
)


class OrganizationError(Exception):
    pass


class OrganizationAlreadyExistsError(OrganizationError):
    pass


class OrganizationAlreadyBootstrappedError(OrganizationError):
    pass


class OrganizationNotFoundError(OrganizationError):
    pass


class OrganizationInvalidBootstrapTokenError(OrganizationError):
    pass


class OrganizationFirstUserCreationError(OrganizationError):
    pass


class OrganizationExpiredError(OrganizationError):
    pass


@attr.s(slots=True, frozen=True, auto_attribs=True)
class SequesterAuthority:
    certificate: bytes
    verify_key_der: SequesterVerifyKeyDer


@attr.s(slots=True, frozen=True, auto_attribs=True)
class Organization:
    organization_id: OrganizationID
    bootstrap_token: str
    is_expired: bool
    created_on: DateTime
    bootstrapped_on: DateTime | None
    root_verify_key: VerifyKey | None
    user_profile_outsider_allowed: bool
    active_users_limit: ActiveUsersLimit
    sequester_authority: SequesterAuthority | None
    sequester_services_certificates: tuple[bytes, ...] | None

    def is_bootstrapped(self) -> bool:
        return self.root_verify_key is not None

    def evolve(self, **kwargs: Any) -> Organization:
        return attr.evolve(self, **kwargs)


def generate_bootstrap_token() -> str:
    return token_hex(32)


class BaseOrganizationComponent:
    def __init__(self, webhooks: WebhooksComponent, config: BackendConfig):
        self.webhooks = webhooks
        self._config = config

    @api
    async def api_authenticated_organization_config(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.organization_config.Req,
    ) -> authenticated_cmds.latest.organization_config.Rep:
        organization_id = client_ctx.organization_id
        try:
            organization = await self.get(organization_id)

        except OrganizationNotFoundError:
            return authenticated_cmds.latest.organization_config.RepNotFound()

        if organization.sequester_authority:
            sequester_authority_certificate: bytes | None = (
                organization.sequester_authority.certificate
            )
            sequester_services_certificates: list[bytes] | None = (
                list(organization.sequester_services_certificates)
                if organization.sequester_services_certificates
                else []
            )
        else:
            sequester_authority_certificate = None
            sequester_services_certificates = None

        return authenticated_cmds.latest.organization_config.RepOk(
            user_profile_outsider_allowed=organization.user_profile_outsider_allowed,
            active_users_limit=organization.active_users_limit,
            sequester_authority_certificate=sequester_authority_certificate,
            sequester_services_certificates=sequester_services_certificates,
        )

    @api
    async def api_authenticated_organization_stats(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.organization_stats.Req,
    ) -> authenticated_cmds.latest.organization_stats.Rep:
        if client_ctx.profile != UserProfile.ADMIN:
            return authenticated_cmds.latest.organization_stats.RepNotAllowed(None)
        # Get organization of the user
        organization_id = client_ctx.organization_id
        try:
            stats = await self.stats(organization_id)

        except OrganizationNotFoundError:
            return authenticated_cmds.latest.organization_stats.RepNotFound()

        return authenticated_cmds.latest.organization_stats.RepOk(
            data_size=stats.data_size,
            metadata_size=stats.metadata_size,
            realms=stats.realms,
            users=stats.users,
            active_users=stats.active_users,
            users_per_profile_detail=list(stats.users_per_profile_detail),
        )

    @api
    async def apiv3_organization_bootstrap(
        self,
        client_ctx: AnonymousClientContext,
        req: anonymous_cmds.v3.organization_bootstrap.Req,
    ) -> anonymous_cmds.v3.organization_bootstrap.Rep:
        # `organization_bootstrap` command is strictly similar between APIv3 and v4+
        # (and old client using APIv3 may send a request with a missing field, but
        # this is automatically handled in the deserialization)
        return await self.api_organization_bootstrap(client_ctx, req)  # type: ignore

    @api
    async def api_organization_bootstrap(
        self,
        client_ctx: AnonymousClientContext,
        req: anonymous_cmds.latest.organization_bootstrap.Req,
    ) -> anonymous_cmds.latest.organization_bootstrap.Rep:
        bootstrap_token = req.bootstrap_token
        root_verify_key = req.root_verify_key

        try:
            u_data = UserCertificate.verify_and_load(
                req.user_certificate,
                author_verify_key=root_verify_key,
                expected_author=None,
            )
            d_data = DeviceCertificate.verify_and_load(
                req.device_certificate,
                author_verify_key=root_verify_key,
                expected_author=None,
            )

            ru_data = UserCertificate.verify_and_load(
                req.redacted_user_certificate,
                author_verify_key=root_verify_key,
                expected_author=None,
            )
            rd_data = DeviceCertificate.verify_and_load(
                req.redacted_device_certificate,
                author_verify_key=root_verify_key,
                expected_author=None,
            )

        except DataError:
            return anonymous_cmds.latest.organization_bootstrap.RepInvalidCertification(None)
        if u_data.profile != UserProfile.ADMIN:
            return anonymous_cmds.latest.organization_bootstrap.RepInvalidData(None)

        if u_data.timestamp != d_data.timestamp:
            return anonymous_cmds.latest.organization_bootstrap.RepInvalidData(None)

        if u_data.user_id != d_data.device_id.user_id:
            return anonymous_cmds.latest.organization_bootstrap.RepInvalidData(None)

        now = DateTime.now()
        if not timestamps_in_the_ballpark(u_data.timestamp, now):
            return anonymous_cmds.latest.organization_bootstrap.RepBadTimestamp(
                reason=None,
                ballpark_client_early_offset=BALLPARK_CLIENT_EARLY_OFFSET,
                ballpark_client_late_offset=BALLPARK_CLIENT_LATE_OFFSET,
                backend_timestamp=now,
                client_timestamp=u_data.timestamp,
            )

        if ru_data.evolve(human_handle=u_data.human_handle) != u_data:
            return anonymous_cmds.latest.organization_bootstrap.RepInvalidData(None)
        if ru_data.human_handle:
            return anonymous_cmds.latest.organization_bootstrap.RepInvalidData(None)

        if rd_data.evolve(device_label=d_data.device_label) != d_data:
            return anonymous_cmds.latest.organization_bootstrap.RepInvalidData(None)
        if rd_data.device_label:
            return anonymous_cmds.latest.organization_bootstrap.RepInvalidData(None)

        sequester_authority_certificate = req.sequester_authority_certificate
        if sequester_authority_certificate is None:
            sequester_authority = None

        else:
            try:
                sequester_authority_certif_data = SequesterAuthorityCertificate.verify_and_load(
                    sequester_authority_certificate,
                    author_verify_key=root_verify_key,
                )

            except DataError:
                return anonymous_cmds.latest.organization_bootstrap.RepInvalidData(None)

            if not timestamps_in_the_ballpark(sequester_authority_certif_data.timestamp, now):
                return anonymous_cmds.latest.organization_bootstrap.RepBadTimestamp(
                    reason=None,
                    ballpark_client_early_offset=BALLPARK_CLIENT_EARLY_OFFSET,
                    ballpark_client_late_offset=BALLPARK_CLIENT_LATE_OFFSET,
                    backend_timestamp=now,
                    client_timestamp=sequester_authority_certif_data.timestamp,
                )

            if sequester_authority_certif_data.timestamp != u_data.timestamp:
                return anonymous_cmds.latest.organization_bootstrap.RepInvalidData(None)

            sequester_authority = SequesterAuthority(
                certificate=sequester_authority_certificate,
                verify_key_der=sequester_authority_certif_data.verify_key_der,
            )

        user = User(
            user_id=u_data.user_id,
            human_handle=u_data.human_handle,
            initial_profile=u_data.profile,
            user_certificate=req.user_certificate,
            redacted_user_certificate=req.redacted_user_certificate,
            user_certifier=u_data.author,
            created_on=u_data.timestamp,
        )
        first_device = Device(
            device_id=d_data.device_id,
            device_label=d_data.device_label,
            device_certificate=req.device_certificate,
            redacted_device_certificate=req.redacted_device_certificate,
            device_certifier=d_data.author,
            created_on=d_data.timestamp,
        )
        try:
            await self.bootstrap(
                id=client_ctx.organization_id,
                user=user,
                first_device=first_device,
                bootstrap_token=bootstrap_token,
                root_verify_key=root_verify_key,
                bootstrapped_on=now,
                sequester_authority=sequester_authority,
            )

        except OrganizationAlreadyBootstrappedError:
            return anonymous_cmds.latest.organization_bootstrap.RepAlreadyBootstrapped()

        except (OrganizationNotFoundError, OrganizationInvalidBootstrapTokenError):
            return anonymous_cmds.latest.organization_bootstrap.RepNotFound()

        # Note: we let OrganizationFirstUserCreationError bubbles up given
        # it should not occurs under normal circumstances

        # Finally notify webhook
        await self.webhooks.on_organization_bootstrap(
            organization_id=client_ctx.organization_id,
            device_id=first_device.device_id,
            device_label=first_device.device_label,
            human_email=user.human_handle.email if user.human_handle else None,
            human_label=user.human_handle.label if user.human_handle else None,
        )

        return anonymous_cmds.latest.organization_bootstrap.RepOk()

    async def create(
        self,
        id: OrganizationID,
        bootstrap_token: str,
        # `None` is a valid value for some of those params, hence it cannot be used
        # as "param not set" marker and we use a custom `Unset` singleton instead.
        # `None` stands for "no limit"
        active_users_limit: Union[UnsetType, ActiveUsersLimit] = Unset,
        user_profile_outsider_allowed: Union[UnsetType, bool] = Unset,
        created_on: DateTime | None = None,
    ) -> None:
        """
        Raises:
            OrganizationAlreadyExistsError
        """
        raise NotImplementedError()

    async def get(self, id: OrganizationID) -> Organization:
        """
        Raises:
            OrganizationNotFoundError
        """
        raise NotImplementedError()

    async def bootstrap(
        self,
        id: OrganizationID,
        user: User,
        first_device: Device,
        bootstrap_token: str,
        root_verify_key: VerifyKey,
        bootstrapped_on: DateTime | None = None,
        sequester_authority: SequesterAuthority | None = None,
    ) -> None:
        """
        Raises:
            OrganizationNotFoundError
            OrganizationAlreadyBootstrappedError
            OrganizationInvalidBootstrapTokenError
            OrganizationFirstUserCreationError
        """
        raise NotImplementedError()

    async def stats(
        self,
        id: OrganizationID,
        at: DateTime | None = None,
    ) -> OrganizationStats:
        """
        Raises:
            OrganizationNotFoundError

        Note: also raises `OrganizationNotFoundError` if the organization is
        present in the database but has been created after `at` datetime.
        """
        raise NotImplementedError()

    async def server_stats(
        self, at: DateTime | None = None
    ) -> dict[OrganizationID, OrganizationStats]:
        """
        Raises: Nothing !
        """
        raise NotImplementedError()

    async def update(
        self,
        id: OrganizationID,
        # `None` is a valid value for some of those params, hence it cannot be used
        # as "param not set" marker and we use a custom `Unset` singleton instead.
        is_expired: Union[UnsetType, bool] = Unset,
        # `None` stands for "no limit"
        active_users_limit: Union[UnsetType, ActiveUsersLimit] = Unset,
        user_profile_outsider_allowed: Union[UnsetType, bool] = Unset,
    ) -> None:
        """
        Raises:
            OrganizationNotFoundError
        """
        raise NotImplementedError()

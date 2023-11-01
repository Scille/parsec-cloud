# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from typing import Literal, assert_never

from parsec._parsec import (
    ActiveUsersLimit,
    BootstrapToken,
    DateTime,
    DeviceCertificate,
    OrganizationID,
    SequesterAuthorityCertificate,
    SequesterVerifyKeyDer,
    UserCertificate,
    UserID,
    UserProfile,
    VerifyKey,
    anonymous_cmds,
)
from parsec.api import api
from parsec.ballpark import TimestampOutOfBallpark, timestamps_in_the_ballpark
from parsec.client_context import AnonymousClientContext
from parsec.config import BackendConfig
from parsec.types import Unset
from parsec.webhooks import WebhooksComponent


@dataclass(slots=True)
class OrganizationStatsProfileDetailItem:
    profile: UserProfile
    active: int
    revoked: int


@dataclass(slots=True)
class OrganizationStats:
    data_size: int
    metadata_size: int
    realms: int
    users: int
    active_users: int
    users_per_profile_detail: tuple[OrganizationStatsProfileDetailItem, ...]


@dataclass(slots=True)
class OrganizationDump:
    organization_id: OrganizationID
    is_bootstrapped: bool
    is_expired: bool
    active_users_limit: ActiveUsersLimit
    user_profile_outsider_allowed: bool


OrganizationBootstrapValidateBadOutcome = Enum(
    "OrganizationBootstrapValidateBadOutcome",
    (
        "INVALID_CERTIFICATE",
        "TIMESTAMP_MISMATCH",
        "INVALID_USER_PROFILE",
        "USER_ID_MISMATCH",
        "INVALID_REDACTED",
        "REDACTED_MISMATCH",
    ),
)


def organization_bootstrap_validate(
    now: DateTime,
    root_verify_key: VerifyKey,
    user_certificate: bytes,
    device_certificate: bytes,
    redacted_user_certificate: bytes,
    redacted_device_certificate: bytes,
    sequester_authority_certificate: bytes | None,
) -> (
    tuple[UserCertificate, DeviceCertificate, SequesterAuthorityCertificate | None]
    | TimestampOutOfBallpark
    | OrganizationBootstrapValidateBadOutcome
):
    try:
        u_data = UserCertificate.verify_and_load(
            user_certificate,
            author_verify_key=root_verify_key,
            expected_author=None,
        )
        d_data = DeviceCertificate.verify_and_load(
            device_certificate,
            author_verify_key=root_verify_key,
            expected_author=None,
        )
        ru_data = UserCertificate.verify_and_load(
            redacted_user_certificate,
            author_verify_key=root_verify_key,
            expected_author=None,
        )
        rd_data = DeviceCertificate.verify_and_load(
            redacted_device_certificate,
            author_verify_key=root_verify_key,
            expected_author=None,
        )

    except ValueError:
        return OrganizationBootstrapValidateBadOutcome.INVALID_CERTIFICATE

    if u_data.profile != UserProfile.ADMIN:
        return OrganizationBootstrapValidateBadOutcome.INVALID_USER_PROFILE

    if u_data.timestamp != d_data.timestamp:
        return OrganizationBootstrapValidateBadOutcome.TIMESTAMP_MISMATCH

    if u_data.user_id != d_data.device_id.user_id:
        return OrganizationBootstrapValidateBadOutcome.USER_ID_MISMATCH

    match timestamps_in_the_ballpark(u_data.timestamp, now):
        case TimestampOutOfBallpark() as error:
            return error

    if not ru_data.is_redacted:
        return OrganizationBootstrapValidateBadOutcome.INVALID_REDACTED
    if not u_data.redacted_compare(ru_data):
        return OrganizationBootstrapValidateBadOutcome.REDACTED_MISMATCH

    if not rd_data.is_redacted:
        return OrganizationBootstrapValidateBadOutcome.INVALID_REDACTED
    if not d_data.redacted_compare(rd_data):
        return OrganizationBootstrapValidateBadOutcome.REDACTED_MISMATCH

    if sequester_authority_certificate is None:
        s_data = None

    else:
        try:
            s_data = SequesterAuthorityCertificate.verify_and_load(
                sequester_authority_certificate,
                author_verify_key=root_verify_key,
            )

        except ValueError:
            return OrganizationBootstrapValidateBadOutcome.INVALID_CERTIFICATE

        match timestamps_in_the_ballpark(s_data.timestamp, now):
            case TimestampOutOfBallpark() as error:
                return error

        if s_data.timestamp != u_data.timestamp:
            return OrganizationBootstrapValidateBadOutcome.TIMESTAMP_MISMATCH

    return u_data, d_data, s_data


@dataclass(slots=True)
class Organization:
    organization_id: OrganizationID
    bootstrap_token: BootstrapToken | None
    is_expired: bool
    created_on: DateTime
    bootstrapped_on: DateTime | None
    root_verify_key: VerifyKey | None
    user_profile_outsider_allowed: bool
    active_users_limit: ActiveUsersLimit
    sequester_authority_certificate: bytes | None
    sequester_authority_verify_key_der: SequesterVerifyKeyDer | None
    sequester_services_certificates: tuple[bytes, ...] | None

    @property
    def is_bootstrapped(self) -> bool:
        return self.root_verify_key is not None

    @property
    def is_sequestered(self) -> bool:
        return self.sequester_authority_certificate is not None


OrganizationCreateBadOutcome = Enum(
    "OrganizationCreateBadOutcome", ("ORGANIZATION_ALREADY_EXISTS",)
)
OrganizationGetBadOutcome = Enum("OrganizationGetBadOutcome", ("ORGANIZATION_NOT_FOUND",))
OrganizationBootstrapStoreBadOutcome = Enum(
    "OrganizationBootstrapStoreBadOutcome",
    (
        "ORGANIZATION_NOT_FOUND",
        "ORGANIZATION_EXPIRED",
        "ORGANIZATION_ALREADY_BOOTSTRAPPED",
        "INVALID_BOOTSTRAP_TOKEN",
    ),
)
OrganizationStatsAsUserBadOutcome = Enum(
    "OrganizationStatsAsUserBadOutcome",
    (
        "ORGANIZATION_NOT_FOUND",
        "ORGANIZATION_EXPIRED",
        "AUTHOR_NOT_FOUND",
        "AUTHOR_REVOKED",
        "AUTHOR_NOT_ALLOWED",
    ),
)
OrganizationStatsBadOutcome = Enum("OrganizationStatsBadOutcome", ("ORGANIZATION_NOT_FOUND",))
OrganizationUpdateBadOutcome = Enum("OrganizationUpdateBadOutcome", ("ORGANIZATION_NOT_FOUND",))


class BaseOrganizationComponent:
    def __init__(self, webhooks: WebhooksComponent, config: BackendConfig):
        self.webhooks = webhooks
        self._config = config

    #
    # Public methods
    #

    async def create(
        self,
        now: DateTime,
        id: OrganizationID,
        # `None` is a valid value for some of those params, hence it cannot be used
        # as "param not set" marker and we use a custom `Unset` singleton instead.
        # `None` stands for "no limit"
        active_users_limit: Literal[Unset] | ActiveUsersLimit = Unset,
        user_profile_outsider_allowed: Literal[Unset] | bool = Unset,
        force_bootstrap_token: BootstrapToken | None = None,
    ) -> BootstrapToken | OrganizationCreateBadOutcome:
        raise NotImplementedError

    async def get(self, id: OrganizationID) -> Organization | OrganizationGetBadOutcome:
        raise NotImplementedError

    async def bootstrap(
        self,
        id: OrganizationID,
        now: DateTime,
        bootstrap_token: BootstrapToken | None,
        root_verify_key: VerifyKey,
        user_certificate: bytes,
        device_certificate: bytes,
        redacted_user_certificate: bytes,
        redacted_device_certificate: bytes,
        sequester_authority_certificate: bytes | None,
    ) -> (
        tuple[UserCertificate, DeviceCertificate, SequesterAuthorityCertificate | None]
        | OrganizationBootstrapValidateBadOutcome
        | OrganizationBootstrapStoreBadOutcome
        | TimestampOutOfBallpark
    ):
        raise NotImplementedError

    async def stats_as_user(
        self,
        organization_id: OrganizationID,
        author: UserID,
        at: DateTime | None = None,
    ) -> OrganizationStats | OrganizationStatsAsUserBadOutcome:
        raise NotImplementedError

    async def organization_stats(
        self,
        organization_id: OrganizationID,
    ) -> OrganizationStats | OrganizationStatsBadOutcome:
        raise NotImplementedError

    async def server_stats(
        self, at: DateTime | None = None
    ) -> dict[OrganizationID, OrganizationStats]:
        raise NotImplementedError

    async def update(
        self,
        id: OrganizationID,
        # `None` is a valid value for some of those params, hence it cannot be used
        # as "param not set" marker and we use a custom `Unset` singleton instead.
        is_expired: Literal[Unset] | bool = Unset,
        # `None` stands for "no limit"
        active_users_limit: Literal[Unset] | ActiveUsersLimit = Unset,
        user_profile_outsider_allowed: Literal[Unset] | bool = Unset,
    ) -> None | OrganizationUpdateBadOutcome:
        raise NotImplementedError

    async def test_dump_organizations(
        self, skip_templates: bool = True
    ) -> dict[OrganizationID, OrganizationDump]:
        raise NotImplementedError

    #
    # API commands
    #

    @api
    async def api_organization_bootstrap(
        self,
        client_ctx: AnonymousClientContext,
        req: anonymous_cmds.latest.organization_bootstrap.Req,
    ) -> anonymous_cmds.latest.organization_bootstrap.Rep:
        outcome = await self.bootstrap(
            id=client_ctx.organization_id,
            now=DateTime.now(),
            bootstrap_token=req.bootstrap_token,
            root_verify_key=req.root_verify_key,
            user_certificate=req.user_certificate,
            device_certificate=req.device_certificate,
            redacted_user_certificate=req.redacted_user_certificate,
            redacted_device_certificate=req.redacted_device_certificate,
            sequester_authority_certificate=req.sequester_authority_certificate,
        )
        match outcome:
            case (UserCertificate() as user, DeviceCertificate() as first_device, _):
                # TODO: replace this by a background task listening for event
                # Finally notify webhook
                await self.webhooks.on_organization_bootstrap(
                    organization_id=client_ctx.organization_id,
                    device_id=first_device.device_id,
                    device_label=first_device.device_label,
                    human_email=user.human_handle.email,
                    human_label=user.human_handle.label,
                )

                return anonymous_cmds.latest.organization_bootstrap.RepOk()

            case OrganizationBootstrapStoreBadOutcome.ORGANIZATION_ALREADY_BOOTSTRAPPED:
                return anonymous_cmds.latest.organization_bootstrap.RepOrganizationAlreadyBootstrapped()

            case OrganizationBootstrapStoreBadOutcome.INVALID_BOOTSTRAP_TOKEN:
                return anonymous_cmds.latest.organization_bootstrap.RepInvalidBootstrapToken()

            case TimestampOutOfBallpark() as error:
                return anonymous_cmds.latest.organization_bootstrap.RepTimestampOutOfBallpark(
                    ballpark_client_early_offset=error.ballpark_client_early_offset,
                    ballpark_client_late_offset=error.ballpark_client_late_offset,
                    server_timestamp=error.server_timestamp,
                    client_timestamp=error.client_timestamp,
                )

            case OrganizationBootstrapValidateBadOutcome():
                return anonymous_cmds.latest.organization_bootstrap.RepInvalidCertificate()

            case OrganizationBootstrapStoreBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()

            case OrganizationBootstrapStoreBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()

            case unknown:
                assert_never(unknown)

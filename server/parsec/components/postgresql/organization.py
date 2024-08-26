# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import Literal, override

from parsec._parsec import (
    ActiveUsersLimit,
    BootstrapToken,
    DateTime,
    DeviceCertificate,
    DeviceID,
    OrganizationID,
    SequesterAuthorityCertificate,
    SequesterVerifyKeyDer,
    UserCertificate,
    VerifyKey,
)
from parsec.ballpark import TimestampOutOfBallpark
from parsec.components.events import EventBus
from parsec.components.organization import (
    BaseOrganizationComponent,
    Organization,
    OrganizationBootstrapStoreBadOutcome,
    OrganizationBootstrapValidateBadOutcome,
    OrganizationCreateBadOutcome,
    OrganizationDump,
    OrganizationDumpTopics,
    OrganizationGetBadOutcome,
    OrganizationStats,
    OrganizationStatsAsUserBadOutcome,
    OrganizationStatsBadOutcome,
    OrganizationUpdateBadOutcome,
)
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.organization_bootstrap import organization_bootstrap
from parsec.components.postgresql.organization_create import organization_create
from parsec.components.postgresql.organization_stats import (
    organization_server_stats,
    organization_stats,
)
from parsec.components.postgresql.organization_test_dump_organizations import (
    organization_test_dump_organizations,
)
from parsec.components.postgresql.organization_test_dump_topics import organization_test_dump_topics
from parsec.components.postgresql.organization_update import organization_update
from parsec.components.postgresql.test_queries import (
    q_test_drop_organization,
    q_test_duplicate_organization,
)
from parsec.components.postgresql.user import PGUserComponent
from parsec.components.postgresql.utils import (
    Q,
    no_transaction,
    q_organization_internal_id,
    transaction,
)
from parsec.config import BackendConfig
from parsec.types import Unset, UnsetType
from parsec.webhooks import WebhooksComponent


def _make_q_get_organization(for_update: bool = False) -> Q:
    return Q(f"""
SELECT
    bootstrap_token,
    root_verify_key,
    is_expired,
    _bootstrapped_on as bootstrapped_on,
    _created_on as created_on,
    active_users_limit,
    user_profile_outsider_allowed,
    sequester_authority_certificate,
    sequester_authority_verify_key_der,
    minimum_archiving_period
FROM organization
WHERE organization_id = $organization_id
{"FOR UPDATE" if for_update else ""}
""")


_q_get_organization = _make_q_get_organization()
_q_get_organization_for_update = _make_q_get_organization(for_update=True)

_q_get_enabled_service_certificates_for_organization = Q(
    f"""
    SELECT service_certificate
    FROM sequester_service
    WHERE
        organization={ q_organization_internal_id("$organization_id") }
        AND disabled_on IS NULL
    ORDER BY _id
    """
)


class PGOrganizationComponent(BaseOrganizationComponent):
    def __init__(
        self,
        pool: AsyncpgPool,
        webhooks: WebhooksComponent,
        config: BackendConfig,
        event_bus: EventBus,
    ) -> None:
        super().__init__(webhooks, config)
        self.pool = pool
        self.event_bus = event_bus
        self.user: PGUserComponent

    def register_components(self, user: PGUserComponent, **kwargs) -> None:
        self.user = user

    @override
    @transaction
    async def create(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        id: OrganizationID,
        # `None` is a valid value for some of those params, hence it cannot be used
        # as "param not set" marker and we use a custom `Unset` singleton instead.
        # `None` stands for "no limit"
        active_users_limit: Literal[UnsetType.Unset] | ActiveUsersLimit = Unset,
        user_profile_outsider_allowed: Literal[UnsetType.Unset] | bool = Unset,
        minimum_archiving_period: UnsetType | int = Unset,
        force_bootstrap_token: BootstrapToken | None = None,
    ) -> BootstrapToken | OrganizationCreateBadOutcome:
        bootstrap_token = force_bootstrap_token or BootstrapToken.new()
        if active_users_limit is Unset:
            active_users_limit = self._config.organization_initial_active_users_limit
        if user_profile_outsider_allowed is Unset:
            user_profile_outsider_allowed = (
                self._config.organization_initial_user_profile_outsider_allowed
            )
        if minimum_archiving_period is Unset:
            minimum_archiving_period = self._config.organization_initial_minimum_archiving_period

        outcome = await organization_create(
            conn,
            now,
            id,
            active_users_limit,
            user_profile_outsider_allowed,
            minimum_archiving_period,
            bootstrap_token,
        )
        match outcome:
            case int():
                return bootstrap_token
            case error:
                return error

    @override
    @transaction
    async def get(
        self, conn: AsyncpgConnection, id: OrganizationID
    ) -> Organization | OrganizationGetBadOutcome:
        return await self._get(conn, id)

    @staticmethod
    async def _get(
        conn: AsyncpgConnection, id: OrganizationID, for_update: bool = False
    ) -> Organization | OrganizationGetBadOutcome:
        if for_update:
            row = await conn.fetchrow(*_q_get_organization_for_update(organization_id=id.str))
        else:
            row = await conn.fetchrow(*_q_get_organization(organization_id=id.str))
        if not row:
            return OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND

        rvk = VerifyKey(row["root_verify_key"]) if row["root_verify_key"] else None

        # Sequester services certificates is None if sequester is not enabled
        sequester_authority_certificate = None
        sequester_services_certificates = None
        sequester_authority_verify_key_der = None

        if row["sequester_authority_certificate"]:
            sequester_authority_verify_key_der = SequesterVerifyKeyDer(
                row["sequester_authority_verify_key_der"]
            )
            sequester_authority_certificate = row["sequester_authority_certificate"]
            services = await conn.fetch(
                *_q_get_enabled_service_certificates_for_organization(organization_id=id.str)
            )
            sequester_services_certificates = tuple(
                service["service_certificate"] for service in services
            )

        raw_bootstrap_token = row["bootstrap_token"]
        bootstrap_token = (
            None if raw_bootstrap_token is None else BootstrapToken.from_hex(raw_bootstrap_token)
        )

        return Organization(
            organization_id=id,
            bootstrap_token=bootstrap_token,
            root_verify_key=rvk,
            is_expired=row["is_expired"],
            created_on=row["created_on"],
            bootstrapped_on=row["bootstrapped_on"],
            active_users_limit=ActiveUsersLimit.from_maybe_int(row["active_users_limit"]),
            user_profile_outsider_allowed=row["user_profile_outsider_allowed"],
            sequester_authority_certificate=sequester_authority_certificate,
            sequester_authority_verify_key_der=sequester_authority_verify_key_der,
            sequester_services_certificates=sequester_services_certificates,
            minimum_archiving_period=row["minimum_archiving_period"],
        )

    @override
    @transaction
    async def bootstrap(
        self,
        conn: AsyncpgConnection,
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
        return await organization_bootstrap(
            conn,
            id,
            now,
            bootstrap_token,
            root_verify_key,
            user_certificate,
            device_certificate,
            redacted_user_certificate,
            redacted_device_certificate,
            sequester_authority_certificate,
        )

    @override
    @transaction
    async def stats(
        self,
        connection: AsyncpgConnection,
        organization_id: OrganizationID,
        author: DeviceID,
        at: DateTime | None = None,
    ) -> OrganizationStats | OrganizationStatsAsUserBadOutcome:
        # TODO: This is intended for organization admins but is not currently exposed/used
        raise NotImplementedError

    @override
    @no_transaction
    async def organization_stats(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
    ) -> OrganizationStats | OrganizationStatsBadOutcome:
        return await organization_stats(conn, organization_id)

    @override
    @no_transaction
    async def server_stats(
        self, conn: AsyncpgConnection, at: DateTime | None = None
    ) -> dict[OrganizationID, OrganizationStats]:
        return await organization_server_stats(conn, at)

    @override
    @transaction
    async def update(
        self,
        conn: AsyncpgConnection,
        id: OrganizationID,
        is_expired: Literal[UnsetType.Unset] | bool = Unset,
        active_users_limit: Literal[UnsetType.Unset] | ActiveUsersLimit = Unset,
        user_profile_outsider_allowed: Literal[UnsetType.Unset] | bool = Unset,
        minimum_archiving_period: Literal[UnsetType.Unset] | int = Unset,
    ) -> None | OrganizationUpdateBadOutcome:
        return await organization_update(
            self.event_bus,
            conn,
            id,
            is_expired,
            active_users_limit,
            user_profile_outsider_allowed,
            minimum_archiving_period,
        )

    @override
    @no_transaction
    async def test_dump_organizations(
        self, conn: AsyncpgConnection, skip_templates: bool = True
    ) -> dict[OrganizationID, OrganizationDump]:
        return await organization_test_dump_organizations(conn, skip_templates)

    @override
    @no_transaction
    async def test_dump_topics(
        self, conn: AsyncpgConnection, id: OrganizationID
    ) -> OrganizationDumpTopics:
        return await organization_test_dump_topics(conn, id)

    @override
    @transaction
    async def test_drop_organization(self, conn: AsyncpgConnection, id: OrganizationID) -> None:
        await conn.execute(*q_test_drop_organization(organization_id=id.str))

    @override
    @transaction
    async def test_duplicate_organization(
        self, conn: AsyncpgConnection, source_id: OrganizationID, target_id: OrganizationID
    ) -> None:
        row = await conn.fetchrow(*_q_get_organization(organization_id=source_id.str))
        assert row is not None, f"The organization {source_id} doesn't exist"
        row = await conn.fetchrow(*_q_get_organization(organization_id=target_id.str))
        assert row is None, f"The organization {target_id} already exists"
        await conn.execute(
            *q_test_duplicate_organization(source_id=source_id.str, target_id=target_id.str)
        )
        row = await conn.fetchrow(*_q_get_organization(organization_id=target_id.str))
        assert row is not None, f"The organization {target_id} hasn't been duplicated"

# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

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
from parsec.components.organization import (
    BaseOrganizationComponent,
    Organization,
    OrganizationBootstrapStoreBadOutcome,
    OrganizationBootstrapValidateBadOutcome,
    OrganizationCreateBadOutcome,
    OrganizationDump,
    OrganizationDumpTopics,
    OrganizationGetBadOutcome,
    OrganizationGetTosBadOutcome,
    OrganizationStats,
    OrganizationStatsAsUserBadOutcome,
    OrganizationStatsBadOutcome,
    OrganizationUpdateBadOutcome,
    TermsOfService,
    TosLocale,
    TosUrl,
)
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.organization_bootstrap import organization_bootstrap
from parsec.components.postgresql.organization_create import organization_create
from parsec.components.postgresql.organization_get_tos import organization_get_tos
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
from parsec.config import AccountVaultStrategy, AllowedClientAgent, BackendConfig
from parsec.types import Unset, UnsetType
from parsec.webhooks import WebhooksComponent


def _make_q_get_organization(for_update: bool = False) -> Q:
    return Q(f"""
SELECT
    bootstrap_token,
    root_verify_key,
    is_expired,
    _bootstrapped_on AS bootstrapped_on,
    _created_on AS created_on,
    active_users_limit,
    user_profile_outsider_allowed,
    sequester_authority_certificate,
    sequester_authority_verify_key_der,
    minimum_archiving_period,
    tos_updated_on,
    tos_per_locale_urls,
    allowed_client_agent,
    account_vault_strategy
FROM organization
WHERE organization_id = $organization_id{''' FOR UPDATE''' if for_update else ""}
""")


_q_get_organization = _make_q_get_organization()
_q_get_organization_for_update = _make_q_get_organization(for_update=True)

_q_get_enabled_service_certificates_for_organization = Q(f"""
SELECT service_certificate
FROM sequester_service
WHERE
    organization = {q_organization_internal_id("$organization_id")}  -- noqa: LT14
    AND disabled_on IS NULL
ORDER BY _id
""")


class PGOrganizationComponent(BaseOrganizationComponent):
    def __init__(
        self,
        pool: AsyncpgPool,
        webhooks: WebhooksComponent,
        config: BackendConfig,
    ) -> None:
        super().__init__(webhooks, config)
        self.pool = pool
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
        active_users_limit: UnsetType | ActiveUsersLimit = Unset,
        user_profile_outsider_allowed: UnsetType | bool = Unset,
        minimum_archiving_period: UnsetType | int = Unset,
        tos: UnsetType | dict[TosLocale, TosUrl] = Unset,
        allowed_client_agent: UnsetType | AllowedClientAgent = Unset,
        account_vault_strategy: UnsetType | AccountVaultStrategy = Unset,
        force_bootstrap_token: BootstrapToken | None = None,
    ) -> BootstrapToken | OrganizationCreateBadOutcome:
        if minimum_archiving_period is not Unset:
            assert minimum_archiving_period >= 0  # Sanity check

        bootstrap_token = force_bootstrap_token or BootstrapToken.new()
        if active_users_limit is Unset:
            active_users_limit = self._config.organization_initial_active_users_limit
        if user_profile_outsider_allowed is Unset:
            user_profile_outsider_allowed = (
                self._config.organization_initial_user_profile_outsider_allowed
            )
        if minimum_archiving_period is Unset:
            minimum_archiving_period = self._config.organization_initial_minimum_archiving_period
        optional_tos = self._config.organization_initial_tos if tos is Unset else tos
        if allowed_client_agent is Unset:
            allowed_client_agent = self._config.organization_initial_allowed_client_agent
        if account_vault_strategy is Unset:
            account_vault_strategy = self._config.organization_initial_account_vault_strategy

        outcome = await organization_create(
            conn,
            now,
            id,
            active_users_limit,
            user_profile_outsider_allowed,
            minimum_archiving_period,
            optional_tos,
            allowed_client_agent,
            account_vault_strategy,
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

    # TODO: Remove me once `server/parsec/components/postgresql/invite.py` no longer depends on it
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

        match row["root_verify_key"]:
            case None:
                rvk = None
            case bytes() as root_verify_key:
                rvk = VerifyKey(root_verify_key)
            case _:
                assert False, row

        match (row["sequester_authority_certificate"], row["sequester_authority_verify_key_der"]):
            # Sequester services certificates is None if sequester is not enabled
            case (None, None):
                sequester_authority_certificate = None
                sequester_services_certificates = None
                sequester_authority_verify_key_der = None

            case (
                bytes() as sequester_authority_certificate,
                bytes() as raw_sequester_authority_verify_key_der,
            ):
                sequester_authority_verify_key_der = SequesterVerifyKeyDer(
                    raw_sequester_authority_verify_key_der
                )

                services = await conn.fetch(
                    *_q_get_enabled_service_certificates_for_organization(organization_id=id.str)
                )
                sequester_services_certificates = tuple(
                    service["service_certificate"] for service in services
                )

            case _:
                assert False, row

        raw_bootstrap_token = row["bootstrap_token"]
        bootstrap_token = (
            None if raw_bootstrap_token is None else BootstrapToken.from_hex(raw_bootstrap_token)
        )

        match (row["tos_updated_on"], row["tos_per_locale_urls"]):
            case (None, None):
                tos = None

            case (DateTime() as tos_updated_on, dict() as tos_per_locale_urls):
                # Sanity check to ensure the JSON on database have the expected
                # {<locale>: <url>} format.
                for key, value in tos_per_locale_urls.items():
                    assert isinstance(key, str) and key, tos_per_locale_urls
                    assert isinstance(value, str) and value, tos_per_locale_urls

                tos = TermsOfService(
                    updated_on=tos_updated_on,
                    per_locale_urls=tos_per_locale_urls,
                )

            case _:
                assert False, row

        match row["allowed_client_agent"]:
            case str() as allowed_client_agent_raw:
                allowed_client_agent = AllowedClientAgent(allowed_client_agent_raw)
            case _:
                assert False, row

        match row["account_vault_strategy"]:
            case str() as account_vault_strategy_raw:
                account_vault_strategy = AccountVaultStrategy(account_vault_strategy_raw)
            case _:
                assert False, row

        match row["minimum_archiving_period"]:
            case int() as minimum_archiving_period if minimum_archiving_period >= 0:
                pass
            case _:
                assert False, row

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
            minimum_archiving_period=minimum_archiving_period,
            tos=tos,
            allowed_client_agent=allowed_client_agent,
            account_vault_strategy=account_vault_strategy,
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
        now: DateTime,
        id: OrganizationID,
        is_expired: UnsetType | bool = Unset,
        active_users_limit: UnsetType | ActiveUsersLimit = Unset,
        user_profile_outsider_allowed: UnsetType | bool = Unset,
        minimum_archiving_period: UnsetType | int = Unset,
        tos: UnsetType | None | dict[TosLocale, TosUrl] = Unset,
        allowed_client_agent: UnsetType | AllowedClientAgent = Unset,
        account_vault_strategy: UnsetType | AccountVaultStrategy = Unset,
    ) -> None | OrganizationUpdateBadOutcome:
        if minimum_archiving_period is not Unset:
            assert minimum_archiving_period >= 0  # Sanity check

        return await organization_update(
            conn,
            now,
            id,
            is_expired,
            active_users_limit,
            user_profile_outsider_allowed,
            minimum_archiving_period,
            tos,
            allowed_client_agent,
            account_vault_strategy,
        )

    @override
    @no_transaction
    async def get_tos(
        self, conn: AsyncpgConnection, id: OrganizationID
    ) -> TermsOfService | None | OrganizationGetTosBadOutcome:
        return await organization_get_tos(conn, id)

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
        await conn.execute(
            *q_test_duplicate_organization(source_id=source_id.str, target_id=target_id.str)
        )

# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from functools import lru_cache
from typing import Literal, assert_never, override

import asyncpg
from asyncpg import UniqueViolationError

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
    OrganizationGetBadOutcome,
    OrganizationStats,
    OrganizationStatsAsUserBadOutcome,
    OrganizationStatsBadOutcome,
    OrganizationStatsProfileDetailItem,
    OrganizationUpdateBadOutcome,
    organization_bootstrap_validate,
)
from parsec.components.postgresql.test_queries import (
    q_test_drop_organization_from_device_table,
    q_test_drop_organization_from_human_table,
    q_test_drop_organization_from_invitation_table,
    q_test_drop_organization_from_organization_table,
    q_test_drop_organization_from_realm_table,
    q_test_drop_organization_from_realm_user_role_table,
    q_test_drop_organization_from_user_table,
    q_test_duplicate_organization_from_device_table,
    q_test_duplicate_organization_from_human_table,
    q_test_duplicate_organization_from_invitation_table,
    q_test_duplicate_organization_from_organization_table,
    q_test_duplicate_organization_from_realm_table,
    q_test_duplicate_organization_from_realm_user_role_table,
    q_test_duplicate_organization_from_user_table,
)
from parsec.components.postgresql.user_queries.create import q_create_user
from parsec.components.postgresql.utils import Q, q_organization_internal_id, transaction
from parsec.components.user import UserCreateDeviceStoreBadOutcome, UserCreateUserStoreBadOutcome
from parsec.config import BackendConfig
from parsec.events import EventOrganizationExpired
from parsec.types import Unset, UnsetType
from parsec.webhooks import WebhooksComponent

_q_insert_organization = Q(
    """
INSERT INTO organization (
    organization_id,
    bootstrap_token,
    active_users_limit,
    user_profile_outsider_allowed,
    _created_on,
    _bootstrapped_on,
    is_expired,
    _expired_on,
    minimum_archiving_period
)
VALUES (
    $organization_id,
    $bootstrap_token,
    $active_users_limit,
    $user_profile_outsider_allowed,
    $created_on,
    NULL,
    FALSE,
    NULL,
    $minimum_archiving_period
)
ON CONFLICT (organization_id) DO
    UPDATE SET
        bootstrap_token = EXCLUDED.bootstrap_token,
        active_users_limit = EXCLUDED.active_users_limit,
        user_profile_outsider_allowed = EXCLUDED.user_profile_outsider_allowed,
        _created_on = EXCLUDED._created_on,
        is_expired = EXCLUDED.is_expired,
        _expired_on = EXCLUDED._expired_on,
        minimum_archiving_period = EXCLUDED.minimum_archiving_period
    WHERE organization.root_verify_key is NULL
"""
)


_q_get_organization = Q(
    """
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
"""
)

_q_get_organization_enabled_services_certificates = Q(
    f"""
    SELECT service_certificate
    FROM sequester_service
    WHERE
        organization={ q_organization_internal_id("$organization_id") }
        AND disabled_on is NULL
    ORDER BY _id
    """
)

_q_get_organization_for_update = Q(
    """
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
FOR UPDATE
"""
)


_q_bootstrap_organization = Q(
    """
UPDATE organization
SET
    root_verify_key = $root_verify_key,
    sequester_authority_certificate=$sequester_authority_certificate,
    sequester_authority_verify_key_der=$sequester_authority_verify_key_der,
    _bootstrapped_on = $bootstrapped_on
WHERE
    organization_id = $organization_id
    AND bootstrap_token = $bootstrap_token
    AND root_verify_key IS NULL
"""
)

# Note the `profile::text` casting here, this is a limitation of asyncpg which doesn't support
# enum within an anonymous record (see https://github.com/MagicStack/asyncpg/issues/360)
_q_get_stats = Q(
    f"""
SELECT
    (
        SELECT COALESCE(_created_on <= $at, false)
        FROM organization
        WHERE organization_id = $organization_id
    ) exist,
    (
        SELECT ARRAY(
            SELECT (revoked_on, profile::text)
            FROM user_
            WHERE organization = { q_organization_internal_id("$organization_id") }
            AND created_on <= $at
        )
    ) users,
    (
        SELECT COUNT(DISTINCT(realm._id))
        FROM realm
        LEFT JOIN realm_user_role
        ON realm_user_role.realm = realm._id
        WHERE realm.organization = { q_organization_internal_id("$organization_id") }
        AND realm_user_role.certified_on <= $at
    ) realms,
    (
        SELECT COALESCE(SUM(size), 0)
        FROM vlob_atom
        WHERE
            organization = { q_organization_internal_id("$organization_id") }
            AND created_on <= $at
            AND (deleted_on IS NULL OR deleted_on > $at)
    ) metadata_size,
    (
        SELECT COALESCE(SUM(size), 0)
        FROM block
        WHERE
            organization = { q_organization_internal_id("$organization_id") }
            AND created_on <= $at
            AND (deleted_on IS NULL OR deleted_on > $at)
    ) data_size
"""
)

_q_get_organizations = Q("SELECT organization_id AS id from organization ORDER BY id")

# There's no `created_on` or similar field for realm. So we get an estimation by
# taking the oldest certification in the `realm_user_role`
_q_get_average_realm_creation_date = Q(
    """
    SELECT realm, MIN(certified_on) AS creation_date
    FROM realm AS r
        JOIN realm_user_role AS rur ON r._id = rur.realm
        JOIN organization AS o ON o._id = r.organization
    WHERE o.organization_id = $organization_id
    GROUP BY realm
"""
)


@lru_cache()
def _q_update_factory(
    with_is_expired: bool,
    with_active_users_limit: bool,
    with_user_profile_outsider_allowed: bool,
    with_minimum_archiving_period: bool,
) -> Q:
    fields = []
    if with_is_expired:
        fields.append("is_expired = $is_expired")
        fields.append("_expired_on = (CASE WHEN $is_expired THEN NOW() ELSE NULL END)")
    if with_active_users_limit:
        fields.append("active_users_limit = $active_users_limit")
    if with_user_profile_outsider_allowed:
        fields.append("user_profile_outsider_allowed = $user_profile_outsider_allowed")
    if with_minimum_archiving_period:
        fields.append("minimum_archiving_period = $minimum_archiving_period")

    return Q(
        f"""
            UPDATE organization
            SET
            { ", ".join(fields) }
            WHERE organization_id = $organization_id
        """
    )


class PGOrganizationComponent(BaseOrganizationComponent):
    def __init__(
        self,
        pool: asyncpg.Pool,
        webhooks: WebhooksComponent,
        config: BackendConfig,
        event_bus: EventBus,
    ) -> None:
        super().__init__(webhooks, config)
        self.pool = pool
        self.event_bus = event_bus

    @override
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
        minimum_archiving_period: UnsetType | int = Unset,
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

        async with self.pool.acquire() as conn:
            try:
                result = await conn.execute(
                    *_q_insert_organization(
                        organization_id=id.str,
                        bootstrap_token=bootstrap_token.hex,
                        active_users_limit=active_users_limit
                        if active_users_limit is not ActiveUsersLimit.NO_LIMIT
                        else None,
                        user_profile_outsider_allowed=user_profile_outsider_allowed,
                        created_on=now,
                        minimum_archiving_period=minimum_archiving_period,
                    )
                )
            except UniqueViolationError:
                return OrganizationCreateBadOutcome.ORGANIZATION_ALREADY_EXISTS
            if result != "INSERT 0 1":
                assert False, f"Insertion error: {result}"
            return bootstrap_token

    @override
    @transaction
    async def get(
        self, conn: asyncpg.Connection, id: OrganizationID
    ) -> Organization | OrganizationGetBadOutcome:
        return await self._get(conn, id)

    @staticmethod
    async def _get(
        conn: asyncpg.Connection, id: OrganizationID, for_update: bool = False
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
                *_q_get_organization_enabled_services_certificates(organization_id=id.str)
            )
            sequester_services_certificates = tuple(
                service["service_certificate"] for service in services
            )

        return Organization(
            organization_id=id,
            bootstrap_token=BootstrapToken.from_hex(row["bootstrap_token"]),
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
        conn: asyncpg.Connection,
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
        # The FOR UPDATE in the query ensure the line is locked in the
        # organization table until the end of the transaction. Hence
        # preventing concurrent bootstrap operation from going any further.
        match await self._get(conn, id, for_update=True):
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return OrganizationBootstrapStoreBadOutcome.ORGANIZATION_NOT_FOUND
            case organization:
                pass

        if organization.is_expired:
            return OrganizationBootstrapStoreBadOutcome.ORGANIZATION_EXPIRED

        if organization.is_bootstrapped:
            return OrganizationBootstrapStoreBadOutcome.ORGANIZATION_ALREADY_BOOTSTRAPPED

        if organization.bootstrap_token != bootstrap_token:
            return OrganizationBootstrapStoreBadOutcome.INVALID_BOOTSTRAP_TOKEN

        match organization_bootstrap_validate(
            now=now,
            root_verify_key=root_verify_key,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
            sequester_authority_certificate=sequester_authority_certificate,
        ):
            case (u_certif, d_certif, s_certif):
                pass
            case error:
                return error

        # All checks are good, now we do the actual insertion

        match await q_create_user(
            conn,
            id,
            u_certif,
            user_certificate,
            redacted_device_certificate,
            d_certif,
            device_certificate,
            redacted_device_certificate,
        ):
            case UserCreateUserStoreBadOutcome():
                assert False, "The organization is empty, user creation should always succeed"
            case UserCreateDeviceStoreBadOutcome():
                assert False, "The organization is empty, device creation should always succeed"

        sequester_authority_verify_key_der = (
            None if s_certif is None else s_certif.verify_key_der.dump()
        )
        bootstrap_token_string = bootstrap_token.hex if bootstrap_token is not None else ""
        result = await conn.execute(
            *_q_bootstrap_organization(
                organization_id=id.str,
                bootstrap_token=bootstrap_token_string,
                bootstrapped_on=now,
                root_verify_key=root_verify_key.encode(),
                sequester_authority_certificate=s_certif,
                sequester_authority_verify_key_der=sequester_authority_verify_key_der,
            )
        )
        assert result == "UPDATE 1"

        return u_certif, d_certif, s_certif

    @override
    @transaction
    async def stats_as_user(
        self,
        connection: asyncpg.Connection,
        organization_id: OrganizationID,
        author: UserID,
        at: DateTime | None = None,
    ) -> OrganizationStats | OrganizationStatsAsUserBadOutcome:
        raise NotImplementedError

    async def _get_organization_stats(
        self,
        connection: asyncpg.Connection,
        organization: OrganizationID,
        at: DateTime | None = None,
    ) -> OrganizationStats | OrganizationStatsBadOutcome:
        at = at or DateTime.now()
        result = await connection.fetchrow(*_q_get_stats(organization_id=organization.str, at=at))
        if result is None or not result["exist"]:
            return OrganizationStatsBadOutcome.ORGANIZATION_NOT_FOUND

        users = 0
        active_users = 0
        users_per_profile_detail = {p: {"active": 0, "revoked": 0} for p in UserProfile.VALUES}
        for u in result["users"]:
            is_revoked, profile = u
            users += 1
            if is_revoked:
                users_per_profile_detail[UserProfile.from_str(profile)]["revoked"] += 1
            else:
                active_users += 1
                users_per_profile_detail[UserProfile.from_str(profile)]["active"] += 1

        users_per_profile_detail = tuple(
            OrganizationStatsProfileDetailItem(profile=profile, **data)
            for profile, data in users_per_profile_detail.items()
        )

        return OrganizationStats(
            data_size=result["data_size"],
            metadata_size=result["metadata_size"],
            realms=result["realms"],
            users=users,
            active_users=active_users,
            users_per_profile_detail=users_per_profile_detail,
        )

    @override
    @transaction
    async def organization_stats(
        self,
        conn: asyncpg.Connection,
        organization_id: OrganizationID,
    ) -> OrganizationStats | OrganizationStatsBadOutcome:
        return await self._get_organization_stats(conn, organization_id)

    @override
    @transaction
    async def server_stats(
        self, connection: asyncpg.Connection, at: DateTime | None = None
    ) -> dict[OrganizationID, OrganizationStats]:
        raise NotImplementedError
        # for org in await conn.fetch(*_q_get_organizations()):
        #     org_id = OrganizationID(org["id"])
        #     org_stats = await _organization_stats(conn, org_id, at)
        #     if org_stats:
        #         results[org_id] = org_stats
        # return results

    @override
    @transaction
    async def update(
        self,
        conn: asyncpg.Connection,
        id: OrganizationID,
        is_expired: Literal[Unset] | bool = Unset,
        active_users_limit: Literal[Unset] | ActiveUsersLimit = Unset,
        user_profile_outsider_allowed: Literal[Unset] | bool = Unset,
        minimum_archiving_period: Literal[Unset] | int = Unset,
    ) -> None | OrganizationUpdateBadOutcome:
        with_is_expired = is_expired is not Unset
        with_active_users_limit = active_users_limit is not Unset
        with_user_profile_outsider_allowed = user_profile_outsider_allowed is not Unset
        with_minimum_archiving_period = minimum_archiving_period is not Unset

        match await self._get(conn, id):
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return OrganizationUpdateBadOutcome.ORGANIZATION_NOT_FOUND
            case Organization() as organization:
                pass
            case unkown:
                assert_never(unkown)

        if (
            not with_is_expired
            and not with_active_users_limit
            and not with_user_profile_outsider_allowed
            and not with_minimum_archiving_period
        ):
            # Nothing to update
            return

        fields: dict[str, object] = {}
        if with_is_expired:
            fields["is_expired"] = is_expired
        if with_active_users_limit:
            assert isinstance(active_users_limit, ActiveUsersLimit)
            fields["active_users_limit"] = active_users_limit.to_maybe_int()
        if with_user_profile_outsider_allowed:
            fields["user_profile_outsider_allowed"] = user_profile_outsider_allowed
        if with_minimum_archiving_period:
            fields["minimum_archiving_period"] = minimum_archiving_period

        q = _q_update_factory(
            with_is_expired=with_is_expired,
            with_active_users_limit=with_active_users_limit,
            with_user_profile_outsider_allowed=with_user_profile_outsider_allowed,
            with_minimum_archiving_period=with_minimum_archiving_period,
        )

        result = await conn.execute(*q(organization_id=id.str, **fields))
        assert result == "UPDATE 1"

        # TODO: the event is triggered even if the orga was already expired, is this okay ?
        if organization.is_expired:
            await self.event_bus.send(EventOrganizationExpired(organization_id=id))

    # async def archiving_config(
    #     self,
    #     id: OrganizationID,
    #     user: UserID,
    # ) -> list[RealmArchivingStatus]:
    #     async with self.dbh.pool.acquire() as conn:
    #         await self._get(conn, id)
    #         mapping = await realm_queries.query_get_realms_for_user(conn, id, user)
    #         realm_ids = {
    #             realm_id: internal_realm_id
    #             for (realm_id, (internal_realm_id, _)) in mapping.items()
    #         }
    #         result = []
    #         for (
    #             realm_id,
    #             configuration,
    #             configured_on,
    #             configured_by,
    #         ) in await realm_queries.query_get_archiving_configurations(
    #             conn,
    #             id,
    #             realm_ids,
    #         ):
    #             result.append(
    #                 RealmArchivingStatus(
    #                     realm_id=realm_id,
    #                     configured_on=configured_on,
    #                     configured_by=configured_by,
    #                     configuration=configuration,
    #                 )
    #             )
    #         return result

    @transaction
    async def test_dump_organizations(
        self, conn: asyncpg.Connection, skip_templates: bool = True
    ) -> dict[OrganizationID, OrganizationDump]:
        items = {}
        for org in await conn.fetch(*_q_get_organizations()):
            match await self._get(conn, OrganizationID(org["id"])):
                case OrganizationGetBadOutcome():
                    continue
                case Organization() as org:
                    pass
                case unkown:
                    assert_never(unkown)

            if org.organization_id.str.endswith("Template") and skip_templates:
                continue

            org.active_users_limit
            items[org.organization_id] = OrganizationDump(
                organization_id=org.organization_id,
                is_bootstrapped=org.is_bootstrapped,
                is_expired=org.is_expired,
                active_users_limit=org.active_users_limit,
                user_profile_outsider_allowed=org.user_profile_outsider_allowed,
            )

        return items

    async def test_drop_organization(self, id: OrganizationID) -> None:
        async with self.pool.acquire() as conn:
            await conn.execute(
                *q_test_drop_organization_from_realm_user_role_table(organization_id=id.str)
            )
            await conn.execute(*q_test_drop_organization_from_realm_table(organization_id=id.str))
            await conn.execute(
                *q_test_drop_organization_from_invitation_table(organization_id=id.str)
            )
            await conn.execute(*q_test_drop_organization_from_device_table(organization_id=id.str))
            await conn.execute(*q_test_drop_organization_from_user_table(organization_id=id.str))
            await conn.execute(*q_test_drop_organization_from_human_table(organization_id=id.str))
            await conn.execute(
                *q_test_drop_organization_from_organization_table(organization_id=id.str)
            )

    async def test_duplicate_organization(
        self, source_id: OrganizationID, target_id: OrganizationID
    ) -> None:
        async with self.pool.acquire() as conn:
            await conn.execute(
                *q_test_duplicate_organization_from_organization_table(
                    source_id=source_id.str, target_id=target_id.str
                )
            )
            await conn.execute(
                *q_test_duplicate_organization_from_human_table(
                    source_id=source_id.str, target_id=target_id.str
                )
            )
            await conn.execute(
                *q_test_duplicate_organization_from_user_table(
                    source_id=source_id.str, target_id=target_id.str
                )
            )
            await conn.execute(
                *q_test_duplicate_organization_from_device_table(
                    source_id=source_id.str, target_id=target_id.str
                )
            )
            await conn.execute(
                *q_test_duplicate_organization_from_invitation_table(
                    source_id=source_id.str, target_id=target_id.str
                )
            )
            await conn.execute(
                *q_test_duplicate_organization_from_realm_table(
                    source_id=source_id.str, target_id=target_id.str
                )
            )
            await conn.execute(
                *q_test_duplicate_organization_from_realm_user_role_table(
                    source_id=source_id.str, target_id=target_id.str
                )
            )

# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from functools import lru_cache
from typing import Literal, override

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
    VerifyKey,
)
from parsec.ballpark import TimestampOutOfBallpark
from parsec.components.organization import (
    BaseOrganizationComponent,
    Organization,
    OrganizationBootstrapStoreBadOutcome,
    OrganizationBootstrapValidateBadOutcome,
    OrganizationCreateBadOutcome,
    OrganizationGetBadOutcome,
    OrganizationStats,
    OrganizationStatsAsUserBadOutcome,
    OrganizationStatsBadOutcome,
    organization_bootstrap_validate,
)
from parsec.components.postgresql.test_queries import (
    q_test_drop_organization_from_device_table,
    q_test_drop_organization_from_human_table,
    q_test_drop_organization_from_invitation_table,
    q_test_drop_organization_from_organization_table,
    q_test_drop_organization_from_user_table,
    q_test_duplicate_organization_from_device_table,
    q_test_duplicate_organization_from_human_table,
    q_test_duplicate_organization_from_invitation_table,
    q_test_duplicate_organization_from_organization_table,
    q_test_duplicate_organization_from_user_table,
)
from parsec.components.postgresql.user_queries.create import q_create_user
from parsec.components.postgresql.utils import Q, q_organization_internal_id, transaction
from parsec.components.user import UserCreateDeviceStoreBadOutcome, UserCreateUserStoreBadOutcome
from parsec.config import BackendConfig
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
    _expired_on
)
VALUES (
    $organization_id,
    $bootstrap_token,
    $active_users_limit,
    $user_profile_outsider_allowed,
    $created_on,
    NULL,
    FALSE,
    NULL
)
ON CONFLICT (organization_id) DO
    UPDATE SET
        bootstrap_token = EXCLUDED.bootstrap_token,
        active_users_limit = EXCLUDED.active_users_limit,
        user_profile_outsider_allowed = EXCLUDED.user_profile_outsider_allowed,
        _created_on = EXCLUDED._created_on,
        is_expired = EXCLUDED.is_expired,
        _expired_on = EXCLUDED._expired_on
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
    sequester_authority_verify_key_der
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
    sequester_authority_verify_key_der
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
    with_is_expired: bool, with_active_users_limit: bool, with_user_profile_outsider_allowed: bool
) -> Q:
    fields = []
    if with_is_expired:
        fields.append("is_expired = $is_expired")
        fields.append("_expired_on = (CASE WHEN $is_expired THEN NOW() ELSE NULL END)")
    if with_active_users_limit:
        fields.append("active_users_limit = $active_users_limit")
    if with_user_profile_outsider_allowed:
        fields.append("user_profile_outsider_allowed = $user_profile_outsider_allowed")

    return Q(
        f"""
            UPDATE organization
            SET
            { ", ".join(fields) }
            WHERE organization_id = $organization_id
        """
    )


async def _organization_stats(
    conn: asyncpg.Connection,
    id: OrganizationID,
    at: DateTime,
) -> OrganizationStats | None:
    raise NotImplementedError
    # result = await conn.fetchrow(*_q_get_stats(organization_id=id.str, at=at))
    # if not result["exist"]:
    #     return None

    # users = 0
    # active_users = 0
    # users_per_profile_detail = {p: {"active": 0, "revoked": 0} for p in UserProfile.VALUES}
    # for u in result["users"]:
    #     is_revoked, profile = u
    #     users += 1
    #     if is_revoked:
    #         users_per_profile_detail[UserProfile.from_str(profile)]["revoked"] += 1
    #     else:
    #         active_users += 1
    #         users_per_profile_detail[UserProfile.from_str(profile)]["active"] += 1

    # users_per_profile_detail = tuple(
    #     UsersPerProfileDetailItem(profile=profile, **data)
    #     for profile, data in users_per_profile_detail.items()
    # )

    # return OrganizationStats(
    #     data_size=result["data_size"],
    #     metadata_size=result["metadata_size"],
    #     realms=result["realms"],
    #     users=users,
    #     active_users=active_users,
    #     users_per_profile_detail=users_per_profile_detail,
    # )


class PGOrganizationComponent(BaseOrganizationComponent):
    def __init__(
        self, pool: asyncpg.Pool, webhooks: WebhooksComponent, config: BackendConfig
    ) -> None:
        super().__init__(webhooks, config)
        self.pool = pool

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
    ) -> BootstrapToken | OrganizationCreateBadOutcome:
        bootstrap_token = force_bootstrap_token or BootstrapToken.new()
        if active_users_limit is Unset:
            active_users_limit = self._config.organization_initial_active_users_limit
        if user_profile_outsider_allowed is Unset:
            user_profile_outsider_allowed = (
                self._config.organization_initial_user_profile_outsider_allowed
            )
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

        # sequester_authority_certificate = None
        # sequester_authority_verify_key_der = None
        # if sequester_authority:
        #     sequester_authority_certificate = sequester_authority.certificate
        #     sequester_authority_verify_key_der = sequester_authority.verify_key_der.dump()
        # result = await conn.execute(
        #     *_q_bootstrap_organization(
        #         organization_id=id.str,
        #         bootstrap_token=bootstrap_token,
        #         bootstrapped_on=bootstrapped_on,
        #         root_verify_key=root_verify_key.encode(),
        #         sequester_authority_certificate=sequester_authority_certificate,
        #         sequester_authority_verify_key_der=sequester_authority_verify_key_der,
        #     )
        # )
        # if result != "UPDATE 1":
        #     raise OrganizationError(f"Update error: {result}")

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

    @override
    @transaction
    async def organization_stats(
        self,
        connection: asyncpg.Connection,
        organization_id: OrganizationID,
    ) -> OrganizationStats | OrganizationStatsBadOutcome:
        raise NotImplementedError
        # at = at or DateTime.now()
        # stats = await _organization_stats(conn, id, at)
        #     if not stats:
        #         raise OrganizationNotFoundError()
        #     return stats

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
    async def update(
        self,
        id: OrganizationID,
        is_expired: UnsetType | bool = Unset,
        active_users_limit: UnsetType | ActiveUsersLimit = Unset,
        user_profile_outsider_allowed: UnsetType | bool = Unset,
    ) -> None:
        raise NotImplementedError
        # fields: dict[str, Any] = {}

        # with_is_expired = is_expired is not Unset
        # with_active_users_limit = active_users_limit is not Unset
        # with_user_profile_outsider_allowed = user_profile_outsider_allowed is not Unset

        # if (
        #     not with_is_expired
        #     and not with_active_users_limit
        #     and not with_user_profile_outsider_allowed
        # ):
        #     # Nothing to update, just make sure the organization exists and
        #     # pretent we actually did an update
        #     await self.get(id=id)
        #     return

        # if with_is_expired:
        #     fields["is_expired"] = is_expired
        # if with_active_users_limit:
        #     assert isinstance(active_users_limit, ActiveUsersLimit)
        #     fields["active_users_limit"] = active_users_limit.to_int()
        # if with_user_profile_outsider_allowed:
        #     fields["user_profile_outsider_allowed"] = user_profile_outsider_allowed

        # q = _q_update_factory(
        #     with_is_expired=with_is_expired,
        #     with_active_users_limit=with_active_users_limit,
        #     with_user_profile_outsider_allowed=with_user_profile_outsider_allowed,
        # )

        # async with self.dbh.pool.acquire() as conn, conn.transaction():
        #     result = await conn.execute(*q(organization_id=id.str, **fields))

        #     if result == "UPDATE 0":
        #         raise OrganizationNotFoundError

        #     if result != "UPDATE 1":
        #         raise OrganizationError(f"Update error: {result}")

        #     if with_is_expired and is_expired:
        #         await send_signal(conn, BackendEventOrganizationExpired(organization_id=id))

    async def test_drop_organization(self, id: OrganizationID) -> None:
        async with self.pool.acquire() as conn:
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

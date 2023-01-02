# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from functools import lru_cache
from typing import Any, Union

import triopg
from triopg import UniqueViolationError

from parsec._parsec import (
    DateTime,
    OrganizationStats,
    SequesterVerifyKeyDer,
    UsersPerProfileDetailItem,
    VerifyKey,
)
from parsec.api.protocol import OrganizationID, UserProfile
from parsec.backend.events import BackendEvent
from parsec.backend.organization import (
    BaseOrganizationComponent,
    Organization,
    OrganizationAlreadyBootstrappedError,
    OrganizationAlreadyExistsError,
    OrganizationError,
    OrganizationFirstUserCreationError,
    OrganizationInvalidBootstrapTokenError,
    OrganizationNotFoundError,
    SequesterAuthority,
)
from parsec.backend.postgresql.handler import PGHandler, send_signal
from parsec.backend.postgresql.user_queries.create import q_create_user
from parsec.backend.postgresql.utils import Q, q_organization_internal_id
from parsec.backend.user import Device, User, UserError
from parsec.backend.utils import Unset, UnsetType

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
    conn: triopg._triopg.TrioConnectionProxy,
    id: OrganizationID,
    at: DateTime,
) -> OrganizationStats | None:
    result = await conn.fetchrow(*_q_get_stats(organization_id=id.str, at=at))
    if not result["exist"]:
        return None

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
        UsersPerProfileDetailItem(profile=profile, **data)
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


class PGOrganizationComponent(BaseOrganizationComponent):
    def __init__(self, dbh: PGHandler, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)
        self.dbh = dbh

    async def create(
        self,
        id: OrganizationID,
        bootstrap_token: str,
        active_users_limit: Union[UnsetType, int | None] = Unset,
        user_profile_outsider_allowed: Union[UnsetType, bool] = Unset,
        created_on: DateTime | None = None,
    ) -> None:
        created_on = created_on or DateTime.now()
        if active_users_limit is Unset:
            active_users_limit = self._config.organization_initial_active_users_limit
        if user_profile_outsider_allowed is Unset:
            user_profile_outsider_allowed = (
                self._config.organization_initial_user_profile_outsider_allowed
            )
        async with self.dbh.pool.acquire() as conn:
            try:
                result = await conn.execute(
                    *_q_insert_organization(
                        organization_id=id.str,
                        bootstrap_token=bootstrap_token,
                        active_users_limit=active_users_limit,
                        user_profile_outsider_allowed=user_profile_outsider_allowed,
                        created_on=created_on,
                    )
                )
            except UniqueViolationError:
                raise OrganizationAlreadyExistsError()

            if result != "INSERT 0 1":
                raise OrganizationAlreadyExistsError()

    async def get(self, id: OrganizationID) -> Organization:
        async with self.dbh.pool.acquire() as conn:
            return await self._get(conn, id)

    @staticmethod
    async def _get(
        conn: triopg._triopg.TrioConnectionProxy, id: OrganizationID, for_update: bool = False
    ) -> Organization:
        if for_update:
            row = await conn.fetchrow(*_q_get_organization_for_update(organization_id=id.str))
        else:
            row = await conn.fetchrow(*_q_get_organization(organization_id=id.str))
        if not row:
            raise OrganizationNotFoundError()

        rvk = VerifyKey(row["root_verify_key"]) if row["root_verify_key"] else None

        sequester_authority = None
        # Sequester services certificates is None if sequester is not enabled
        sequester_services_certificates = None

        if row["sequester_authority_certificate"]:
            verify_key_der = SequesterVerifyKeyDer(row["sequester_authority_verify_key_der"])
            sequester_authority = SequesterAuthority(
                certificate=row["sequester_authority_certificate"], verify_key_der=verify_key_der
            )
            services = await conn.fetch(
                *_q_get_organization_enabled_services_certificates(organization_id=id.str)
            )
            sequester_services_certificates = tuple(
                service["service_certificate"] for service in services
            )
            sequester_services_certificates = sequester_services_certificates

        return Organization(
            organization_id=id,
            bootstrap_token=row["bootstrap_token"],
            root_verify_key=rvk,
            is_expired=row["is_expired"],
            created_on=row["created_on"],
            bootstrapped_on=row["bootstrapped_on"],
            active_users_limit=row["active_users_limit"],
            user_profile_outsider_allowed=row["user_profile_outsider_allowed"],
            sequester_authority=sequester_authority,
            sequester_services_certificates=sequester_services_certificates,
        )

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
        bootstrapped_on = bootstrapped_on or DateTime.now()
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            # The FOR UPDATE in the query ensure the line is locked in the
            # organization table until the end of the transaction. Hence
            # preventing concurrent bootstrap operation form going any further.
            organization = await self._get(conn, id, for_update=True)

            if organization.is_bootstrapped():
                raise OrganizationAlreadyBootstrappedError()

            if organization.bootstrap_token != bootstrap_token:
                raise OrganizationInvalidBootstrapTokenError()

            try:
                await q_create_user(conn, id, user, first_device)
            except UserError as exc:
                raise OrganizationFirstUserCreationError(exc) from exc

            sequester_authority_certificate = None
            sequester_authority_verify_key_der = None
            if sequester_authority:
                sequester_authority_certificate = sequester_authority.certificate
                sequester_authority_verify_key_der = sequester_authority.verify_key_der.dump()
            result = await conn.execute(
                *_q_bootstrap_organization(
                    organization_id=id.str,
                    bootstrap_token=bootstrap_token,
                    bootstrapped_on=bootstrapped_on,
                    root_verify_key=root_verify_key.encode(),
                    sequester_authority_certificate=sequester_authority_certificate,
                    sequester_authority_verify_key_der=sequester_authority_verify_key_der,
                )
            )
            if result != "UPDATE 1":
                raise OrganizationError(f"Update error: {result}")

    async def stats(
        self,
        id: OrganizationID,
        at: DateTime | None = None,
    ) -> OrganizationStats:
        at = at or DateTime.now()
        async with self.dbh.pool.acquire() as conn:
            stats = await _organization_stats(conn, id, at)
            if not stats:
                raise OrganizationNotFoundError()
            return stats

    async def server_stats(
        self, at: DateTime | None = None
    ) -> dict[OrganizationID, OrganizationStats]:
        at = at or DateTime.now()
        results = {}

        async with self.dbh.pool.acquire() as conn, conn.transaction():
            for org in await conn.fetch(*_q_get_organizations()):
                org_id = OrganizationID(org["id"])
                org_stats = await _organization_stats(conn, org_id, at)
                if org_stats:
                    results[org_id] = org_stats

        return results

    async def update(
        self,
        id: OrganizationID,
        is_expired: Union[UnsetType, bool] = Unset,
        active_users_limit: Union[UnsetType, int | None] = Unset,
        user_profile_outsider_allowed: Union[UnsetType, bool] = Unset,
    ) -> None:
        """
        Raises:
            OrganizationNotFoundError
            OrganizationError
        """
        fields: dict[str, Any] = {}

        with_is_expired = is_expired is not Unset
        with_active_users_limit = active_users_limit is not Unset
        with_user_profile_outsider_allowed = user_profile_outsider_allowed is not Unset

        if (
            not with_is_expired
            and not with_active_users_limit
            and not with_user_profile_outsider_allowed
        ):
            # Nothing to update, just make sure the organization exists and
            # pretent we actually did an update
            await self.get(id=id)
            return

        if with_is_expired:
            fields["is_expired"] = is_expired
        if with_active_users_limit:
            fields["active_users_limit"] = active_users_limit
        if with_user_profile_outsider_allowed:
            fields["user_profile_outsider_allowed"] = user_profile_outsider_allowed

        q = _q_update_factory(
            with_is_expired=with_is_expired,
            with_active_users_limit=with_active_users_limit,
            with_user_profile_outsider_allowed=with_user_profile_outsider_allowed,
        )

        async with self.dbh.pool.acquire() as conn, conn.transaction():
            result = await conn.execute(*q(organization_id=id.str, **fields))

            if result == "UPDATE 0":
                raise OrganizationNotFoundError

            if result != "UPDATE 1":
                raise OrganizationError(f"Update error: {result}")

            if with_is_expired and is_expired:
                await send_signal(conn, BackendEvent.ORGANIZATION_EXPIRED, organization_id=id)

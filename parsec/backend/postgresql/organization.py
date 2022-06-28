# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from typing import Optional, Union
from functools import lru_cache
from triopg import UniqueViolationError

from parsec.api.protocol import OrganizationID, UserProfile
from parsec.crypto import VerifyKey
from parsec.backend.events import BackendEvent
from parsec.backend.user import UserError, User, Device
from parsec.backend.utils import UnsetType, Unset
from parsec.backend.organization import (
    BaseOrganizationComponent,
    SequesterAuthority,
    OrganizationStats,
    Organization,
    OrganizationError,
    OrganizationAlreadyExistsError,
    OrganizationInvalidBootstrapTokenError,
    OrganizationAlreadyBootstrappedError,
    OrganizationNotFoundError,
    OrganizationFirstUserCreationError,
    UsersPerProfileDetailItem,
)
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.postgresql.user_queries.create import q_create_user
from parsec.backend.postgresql.utils import Q, q_organization_internal_id
from parsec.backend.postgresql.handler import send_signal


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
    NOW(),
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
SELECT bootstrap_token, root_verify_key, is_expired, active_users_limit, user_profile_outsider_allowed, sequester_authority_certificate
FROM organization
WHERE organization_id = $organization_id
"""
)


_q_get_organization_for_update = Q(
    """
SELECT bootstrap_token, root_verify_key, is_expired, active_users_limit, user_profile_outsider_allowed, sequester_authority_certificate
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
    sequester_authority_certificate= $sequester_authority_certificate
    _bootstrapped_on = NOW()
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
        EXISTS(
            SELECT 1 FROM organization WHERE _id = { q_organization_internal_id("$organization_id") }
        )
    ) exist,
    (
        SELECT ARRAY(
            SELECT (revoked_on, profile::text)
            FROM user_
            WHERE organization = { q_organization_internal_id("$organization_id") }
        )
    ) users,
    (
        SELECT COUNT(*)
        FROM realm
        WHERE realm.organization = { q_organization_internal_id("$organization_id") }
    ) realms,
    (
        SELECT COALESCE(SUM(size), 0)
        FROM vlob_atom
        WHERE
            organization = { q_organization_internal_id("$organization_id") }
    ) metadata_size,
    (
        SELECT COALESCE(SUM(size), 0)
        FROM block
        WHERE
            organization = { q_organization_internal_id("$organization_id") }
    ) data_size
"""
)


@lru_cache()
def _q_update_factory(
    with_is_expired: bool, with_active_users_limit: bool, with_user_profile_outsider_allowed: bool
):
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


class PGOrganizationComponent(BaseOrganizationComponent):
    def __init__(self, dbh: PGHandler, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.dbh = dbh

    async def create(
        self,
        id: OrganizationID,
        bootstrap_token: str,
        active_users_limit: Union[UnsetType, Optional[int]] = Unset,
        user_profile_outsider_allowed: Union[UnsetType, bool] = Unset,
    ) -> None:
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
    async def _get(conn, id: OrganizationID, for_update: bool = False) -> Organization:
        if for_update:
            data = await conn.fetchrow(*_q_get_organization_for_update(organization_id=id.str))
        else:
            data = await conn.fetchrow(*_q_get_organization(organization_id=id.str))
        if not data:
            raise OrganizationNotFoundError()

        rvk = VerifyKey(data[1]) if data[1] else None
        return Organization(
            organization_id=id,
            bootstrap_token=data[0],
            root_verify_key=rvk,
            is_expired=data[2],
            active_users_limit=data[3],
            user_profile_outsider_allowed=data[4],
            sequester_authority=data[5],
            sequester_services_certificates=None,  # TODO: implement it in postgresql version
        )

    async def bootstrap(
        self,
        id: OrganizationID,
        user: User,
        first_device: Device,
        bootstrap_token: str,
        root_verify_key: VerifyKey,
        sequester_authority: Optional[SequesterAuthority] = None,
    ) -> None:
        # TODO
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

            result = await conn.execute(
                *_q_bootstrap_organization(
                    organization_id=id.str,
                    bootstrap_token=bootstrap_token,
                    root_verify_key=root_verify_key.encode(),
                    sequester_authority_certificate=sequester_authority,
                )
            )

            if result != "UPDATE 1":
                raise OrganizationError(f"Update error: {result}")

    async def stats(self, id: OrganizationID) -> OrganizationStats:
        async with self.dbh.pool.acquire() as conn, conn.transaction():
            result = await conn.fetchrow(*_q_get_stats(organization_id=id.str))
            if not result["exist"]:
                raise OrganizationNotFoundError()

            users = 0
            active_users = 0
            users_per_profile_detail = {p: {"active": 0, "revoked": 0} for p in UserProfile}
            for u in result["users"]:
                is_revoked, profile = u
                users += 1
                if is_revoked:
                    users_per_profile_detail[UserProfile[profile]]["revoked"] += 1
                else:
                    active_users += 1
                    users_per_profile_detail[UserProfile[profile]]["active"] += 1

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

    async def update(
        self,
        id: OrganizationID,
        is_expired: Union[UnsetType, bool] = Unset,
        active_users_limit: Union[UnsetType, Optional[int]] = Unset,
        user_profile_outsider_allowed: Union[UnsetType, bool] = Unset,
    ) -> None:
        """
        Raises:
            OrganizationNotFoundError
            OrganizationError
        """
        fields: dict = {}

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

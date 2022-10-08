# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import Optional, Union, Tuple
from functools import lru_cache
from triopg import UniqueViolationError

from parsec.api.protocol import OrganizationID, UserProfile
from parsec.crypto import VerifyKey
from parsec.sequester_crypto import SequesterVerifyKeyDer
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
from parsec.backend.sequester import BaseSequesterService, WebhookSequesterService
from parsec.backend.postgresql.handler import PGHandler, send_signal
from parsec.backend.postgresql.user_queries.create import q_create_user
from parsec.backend.postgresql.utils import Q, q_organization_internal_id
from parsec.backend.postgresql.sequester import _q_create_sequester_service


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
SELECT
    bootstrap_token,
    root_verify_key,
    is_expired,
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
        sequester_authority: Optional[SequesterAuthority] = None,
        sequester_initial_services: Optional[Tuple[BaseSequesterService, ...]] = None,
    ) -> None:
        assert sequester_initial_services is None or sequester_authority is not None

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
                    root_verify_key=root_verify_key.encode(),
                    sequester_authority_certificate=sequester_authority_certificate,
                    sequester_authority_verify_key_der=sequester_authority_verify_key_der,
                )
            )
            if result != "UPDATE 1":
                raise OrganizationError(f"Update error: {result}")

            if sequester_initial_services:
                # Sanity check, the caller is responsible to provide unique service IDs
                assert len({x.service_id for x in sequester_initial_services}) == len(sequester_initial_services)

                for service in sequester_initial_services:
                    webhook_url: Optional[str]
                    if isinstance(service, WebhookSequesterService):
                        webhook_url = service.webhook_url
                    else:
                        webhook_url = None
                    result = await conn.execute(
                        *_q_create_sequester_service(
                            organization_id=id.str,
                            service_id=service.service_id,
                            service_label=service.service_label,
                            service_certificate=service.service_certificate,
                            created_on=service.created_on,
                            service_type=service.service_type.value.upper(),
                            webhook_url=webhook_url,
                        )
                    )
                    if result != "INSERT 0 1":
                        raise OrganizationError(f"Insertion Error: {result}")

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

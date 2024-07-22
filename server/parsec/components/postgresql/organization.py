# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from functools import lru_cache
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
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.test_queries import (
    q_test_drop_organization,
    q_test_duplicate_organization,
)
from parsec.components.postgresql.user import PGUserComponent
from parsec.components.postgresql.utils import Q, q_organization_internal_id, transaction
from parsec.components.user import UserCreateDeviceStoreBadOutcome, UserCreateUserStoreBadOutcome
from parsec.config import BackendConfig
from parsec.events import EventOrganizationExpired
from parsec.types import Unset, UnsetType
from parsec.webhooks import WebhooksComponent

_q_insert_organization = Q(
    """
WITH new_organization AS (
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
        WHERE organization.root_verify_key IS NULL
    RETURNING _id
)
INSERT INTO common_topic (organization, last_timestamp)
SELECT _id, $created_on FROM new_organization
ON CONFLICT (organization) DO NOTHING
"""
)


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
    AND bootstrap_token IS NOT DISTINCT FROM $bootstrap_token
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
    ) AS found,
    (
        SELECT ARRAY(
            SELECT (
                revoked_on,
                COALESCE (
                    (
                        SELECT profile.profile::text
                        FROM profile
                        WHERE profile.user_ = user_._id
                        ORDER BY profile.certified_on DESC LIMIT 1
                    ),
                    user_.initial_profile::text
                )
            )
            FROM user_
            WHERE organization = { q_organization_internal_id("$organization_id") }
            AND created_on <= $at
        )
    ) AS users,
    (
        SELECT COUNT(DISTINCT(realm._id))
        FROM realm
        LEFT JOIN realm_user_role
        ON realm_user_role.realm = realm._id
        WHERE realm.organization = { q_organization_internal_id("$organization_id") }
        AND realm_user_role.certified_on <= $at
    ) AS realms,
    (
        SELECT COALESCE(SUM(size), 0)
        FROM vlob_atom
        LEFT JOIN realm
        ON vlob_atom.realm = realm._id
        WHERE
            realm.organization = { q_organization_internal_id("$organization_id") }
            AND vlob_atom.created_on <= $at
            AND (vlob_atom.deleted_on IS NULL OR vlob_atom.deleted_on > $at)
    ) AS metadata_size,
    (
        SELECT COALESCE(SUM(size), 0)
        FROM block
        WHERE
            organization = { q_organization_internal_id("$organization_id") }
            AND created_on <= $at
            AND (deleted_on IS NULL OR deleted_on > $at)
    ) AS data_size
"""
)

_q_get_organizations = Q("SELECT organization_id AS id FROM organization ORDER BY id")


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
        if result == "INSERT 0 0":
            return OrganizationCreateBadOutcome.ORGANIZATION_ALREADY_EXISTS
        if result != "INSERT 0 1":
            assert False, f"Insertion error: {result}"
        return bootstrap_token

    async def spontaneous_create(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        now: DateTime,
    ) -> None:
        active_users_limit = self._config.organization_initial_active_users_limit
        user_profile_outsider_allowed = (
            self._config.organization_initial_user_profile_outsider_allowed
        )
        minimum_archiving_period = self._config.organization_initial_minimum_archiving_period
        result = await conn.execute(
            *_q_insert_organization(
                organization_id=organization_id.str,
                bootstrap_token=None,
                active_users_limit=active_users_limit
                if active_users_limit is not ActiveUsersLimit.NO_LIMIT
                else None,
                user_profile_outsider_allowed=user_profile_outsider_allowed,
                created_on=now,
                minimum_archiving_period=minimum_archiving_period,
            )
        )
        if result != "INSERT 0 1":
            assert False, f"Insertion error: {result}"

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
        # The FOR UPDATE in the query ensure the line is locked in the
        # organization table until the end of the transaction. Hence
        # preventing concurrent bootstrap operation from going any further.
        match await self._get(conn, id, for_update=True):
            case Organization() as organization:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return OrganizationBootstrapStoreBadOutcome.ORGANIZATION_NOT_FOUND

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
            case (user_certificate_cooked, device_certificate_cooked, s_certif):
                pass
            case error:
                return error

        # All checks are good, now we do the actual insertion
        match await self.user._create_user(
            conn,
            id,
            user_certificate_cooked,
            user_certificate,
            redacted_user_certificate,
        ):
            case None:
                pass
            case UserCreateUserStoreBadOutcome() as err:
                assert (
                    False
                ), f"The organization is empty, user creation should always succeed (got {err})"

        match await self.user._create_device(
            conn,
            id,
            device_certificate_cooked,
            device_certificate,
            redacted_device_certificate,
            first_device=True,
        ):
            case None:
                pass
            case UserCreateDeviceStoreBadOutcome() as err:
                assert (
                    False
                ), f"The organization is empty, device creation should always succeed (got {err})"

        sequester_authority_verify_key_der = (
            None if s_certif is None else s_certif.verify_key_der.dump()
        )
        result = await conn.execute(
            *_q_bootstrap_organization(
                organization_id=id.str,
                bootstrap_token=None if bootstrap_token is None else bootstrap_token.hex,
                bootstrapped_on=now,
                root_verify_key=root_verify_key.encode(),
                sequester_authority_certificate=s_certif,
                sequester_authority_verify_key_der=sequester_authority_verify_key_der,
            )
        )
        assert result == "UPDATE 1", result

        return user_certificate_cooked, device_certificate_cooked, s_certif

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

    async def _get_organization_stats(
        self,
        connection: AsyncpgConnection,
        organization: OrganizationID,
        at: DateTime | None = None,
    ) -> OrganizationStats | OrganizationStatsBadOutcome:
        at = at or DateTime.now()
        result = await connection.fetchrow(*_q_get_stats(organization_id=organization.str, at=at))
        if result is None or not result["found"]:
            return OrganizationStatsBadOutcome.ORGANIZATION_NOT_FOUND

        users = 0
        active_users = 0
        users_per_profile = {profile: {"active": 0, "revoked": 0} for profile in UserProfile.VALUES}
        for u in result["users"]:
            revoked_on, profile = u
            users += 1
            user_profile = UserProfile.from_str(profile)
            if revoked_on:
                users_per_profile[user_profile]["revoked"] += 1
            else:
                active_users += 1
                users_per_profile[user_profile]["active"] += 1

        users_per_profile_detail = tuple(
            OrganizationStatsProfileDetailItem(profile=profile, **data)
            for profile, data in users_per_profile.items()
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
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
    ) -> OrganizationStats | OrganizationStatsBadOutcome:
        return await self._get_organization_stats(conn, organization_id)

    @override
    @transaction
    async def server_stats(
        self, conn: AsyncpgConnection, at: DateTime | None = None
    ) -> dict[OrganizationID, OrganizationStats]:
        results = {}
        for org in await conn.fetch(*_q_get_organizations()):
            org_id = OrganizationID(org["id"])
            org_stats = await self._get_organization_stats(conn, org_id, at=at)
            if org_stats:
                results[org_id] = org_stats
        return results

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
        with_is_expired = is_expired is not Unset
        with_active_users_limit = active_users_limit is not Unset
        with_user_profile_outsider_allowed = user_profile_outsider_allowed is not Unset
        with_minimum_archiving_period = minimum_archiving_period is not Unset

        match await self._get(conn, id):
            case Organization() as organization:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return OrganizationUpdateBadOutcome.ORGANIZATION_NOT_FOUND

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

    @transaction
    async def test_dump_organizations(
        self, conn: AsyncpgConnection, skip_templates: bool = True
    ) -> dict[OrganizationID, OrganizationDump]:
        items = {}
        for org in await conn.fetch(*_q_get_organizations()):
            match await self._get(conn, OrganizationID(org["id"])):
                case Organization() as org:
                    pass
                case OrganizationGetBadOutcome():
                    continue

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

    @transaction
    async def test_drop_organization(self, conn: AsyncpgConnection, id: OrganizationID) -> None:
        await conn.execute(*q_test_drop_organization(organization_id=id.str))

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

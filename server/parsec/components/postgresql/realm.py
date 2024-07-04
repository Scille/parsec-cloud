# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmKeyRotationCertificate,
    RealmNameCertificate,
    RealmRole,
    RealmRoleCertificate,
    UserID,
    UserProfile,
    VerifyKey,
    VlobID,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.events import EventBus
from parsec.components.organization import Organization, OrganizationGetBadOutcome
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.user import PGUserComponent
from parsec.components.postgresql.utils import (
    Q,
    q_device_internal_id,
    q_organization_internal_id,
    q_realm,
    q_realm_internal_id,
    q_user,
    q_user_internal_id,
    transaction,
)
from parsec.components.realm import (
    BadKeyIndex,
    BaseRealmComponent,
    CertificateBasedActionIdempotentOutcome,
    KeyIndex,
    KeysBundle,
    RealmCheckBadOutcome,
    RealmCreateStoreBadOutcome,
    RealmCreateValidateBadOutcome,
    RealmGetCurrentRealmsForUserBadOutcome,
    RealmGetKeysBundleBadOutcome,
    RealmRenameStoreBadOutcome,
    RealmRenameValidateBadOutcome,
    RealmRotateKeyStoreBadOutcome,
    RealmRotateKeyValidateBadOutcome,
    RealmShareStoreBadOutcome,
    RealmShareValidateBadOutcome,
    RealmUnshareStoreBadOutcome,
    RealmUnshareValidateBadOutcome,
    realm_create_validate,
    realm_rename_validate,
    realm_rotate_key_validate,
    realm_share_validate,
    realm_unshare_validate,
)
from parsec.components.user import CheckDeviceBadOutcome, CheckUserBadOutcome
from parsec.events import EventRealmCertificate


def _make_q_lock_realm(for_update: bool = False, for_share=False) -> Q:
    assert for_update ^ for_share
    share_or_update = "SHARE" if for_share else "UPDATE"
    return Q(f"""
WITH
selected_realm AS (
    SELECT _id
    FROM realm
    WHERE
        organization = { q_organization_internal_id("$organization_id") }
        AND realm_id = $realm_id
),
locked_realms AS (
    SELECT selected_realm._id, last_timestamp
    FROM realm_topic
    INNER JOIN selected_realm ON realm_topic.realm = selected_realm._id
    FOR {share_or_update}
)
SELECT
    (
        SELECT realm_user_role.role
        FROM realm_user_role
        WHERE
            realm_user_role.realm = realm._id
            AND realm_user_role.user_ = { q_user_internal_id(organization="realm.organization", user_id="$user_id") }
        ORDER BY certified_on DESC
        LIMIT 1
    ) AS role,
    key_index,
    last_timestamp
FROM realm
INNER JOIN locked_realms USING (_id)
""")


_q_check_realm_topic = _make_q_lock_realm(for_share=True)
_q_lock_realm_topic = _make_q_lock_realm(for_update=True)

_q_get_current_roles = Q(
    f"""
SELECT DISTINCT ON(user_)
    { q_user(_id="realm_user_role.user_", select="user_id") },
    role
FROM  realm_user_role
WHERE realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
ORDER BY user_, certified_on DESC
"""
)

_q_insert_keys_bundle = Q(
    f"""
INSERT INTO realm_keys_bundle (
    realm,
    key_index,
    realm_key_rotation_certificate,
    certified_by,
    certified_on,
    key_canary,
    keys_bundle
) VALUES (
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    $key_index,
    $realm_key_rotation_certificate,
    { q_device_internal_id(organization_id="$organization_id", device_id="$certified_by") },
    $certified_on,
    $key_canary,
    $keys_bundle
)
RETURNING _id
"""
)

_q_insert_keys_bundle_access = Q(
    f"""
INSERT INTO realm_keys_bundle_access (
    realm,
    user_,
    realm_keys_bundle,
    access
) VALUES (
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") },
    $realm_keys_bundle_internal_id,
    $access
)
"""
)

_q_update_key_index = Q(
    f"""
UPDATE realm
SET key_index = $key_index
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND realm_id = $realm_id
"""
)

_q_get_keys_bundle = Q(
    f"""
SELECT
    realm_keys_bundle._id,
    realm_keys_bundle.keys_bundle
FROM realm_keys_bundle
INNER JOIN realm ON realm_keys_bundle.realm = realm._id
WHERE
    realm.organization = { q_organization_internal_id("$organization_id") }
    AND realm.realm_id = $realm_id
    AND realm_keys_bundle.key_index = $key_index
"""
)

_q_get_keys_bundle_access = Q(
    """
SELECT realm_keys_bundle_access.access
FROM realm_keys_bundle_access
INNER JOIN user_ ON realm_keys_bundle_access.user_ = user_._id
WHERE realm_keys_bundle = $realm_keys_bundle_internal_id
AND user_.user_id = $user_id
"""
)

_q_get_realms_for_user = Q(
    f"""
SELECT DISTINCT ON(realm)
    { q_realm(_id="realm_user_role.realm", select="realm_id") } as realm_id,
    role,
    certified_on
FROM realm_user_role
WHERE user_ = { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
ORDER BY realm, certified_on DESC
"""
)

_q_get_realm_role_certificates = Q(
    f"""
SELECT
    realm_user_role.certified_on,
    realm_user_role.certificate
FROM realm_user_role
INNER JOIN realm ON realm_user_role.realm = realm._id
WHERE
    realm.organization = { q_organization_internal_id("$organization_id") }
    AND realm.realm_id = $realm_id
    AND COALESCE(realm_user_role.certified_on > $after, TRUE)
    AND COALESCE(realm_user_role.certified_on <= $before, TRUE)
"""
)

_q_get_key_rotation_certificates = Q(
    f"""
SELECT
    realm_keys_bundle.certified_on,
    realm_keys_bundle.realm_key_rotation_certificate
FROM realm_keys_bundle
INNER JOIN realm ON realm_keys_bundle.realm = realm._id
WHERE
    realm.organization = { q_organization_internal_id("$organization_id") }
    AND realm.realm_id = $realm_id
    AND COALESCE(realm_keys_bundle.certified_on > $after, TRUE)
    AND COALESCE(realm_keys_bundle.certified_on <= $before, TRUE)
"""
)

_q_get_realm_name_certificates = Q(
    f"""
SELECT
    realm_name.certified_on,
    realm_name.realm_name_certificate
FROM realm_name
INNER JOIN realm ON realm_name.realm = realm._id
WHERE
    realm.organization = { q_organization_internal_id("$organization_id") }
    AND realm.realm_id = $realm_id
    AND COALESCE(realm_name.certified_on > $after, TRUE)
    AND COALESCE(realm_name.certified_on <= $before, TRUE)
"""
)

_q_rename_realm = Q(
    f"""
INSERT INTO realm_name (
    realm,
    realm_name_certificate,
    certified_by,
    certified_on
) VALUES (
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    $realm_name_certificate,
    { q_device_internal_id(organization_id="$organization_id", device_id="$certified_by") },
    $certified_on
)
"""
)

_q_insert_realm_user_role = Q(
    f"""
INSERT INTO realm_user_role (
    realm,
    user_,
    role,
    certificate,
    certified_by,
    certified_on
) VALUES (
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") },
    $role,
    $certificate,
    { q_device_internal_id(organization_id="$organization_id", device_id="$certified_by") },
    $certified_on
)
"""
)

_q_insert_recipient_keys_bundle_access = Q(
    f"""
INSERT INTO realm_keys_bundle_access (
    realm,
    user_,
    realm_keys_bundle,
    access
) VALUES (
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") },
    (
        SELECT _id
        FROM realm_keys_bundle
        WHERE
            realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
            AND key_index = $key_index
    ),
    $access
)
ON CONFLICT (realm, user_, realm_keys_bundle) DO NOTHING
"""
)

_q_update_realm_topic = Q(
    f"""
UPDATE realm_topic
SET last_timestamp = $timestamp
WHERE realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
"""
)

_q_create_realm = Q(
    f"""
WITH new_realm_id AS (
    INSERT INTO realm (
        organization,
        realm_id,
        created_on,
        key_index
    ) VALUES (
        { q_organization_internal_id("$organization_id") },
        $realm_id,
        $timestamp,
        0
    )
    ON CONFLICT (organization, realm_id) DO NOTHING
    RETURNING _id
),
new_realm_user_role AS (
    INSERT INTO realm_user_role (
        realm,
        user_,
        role,
        certificate,
        certified_by,
        certified_on
    )
    SELECT
        _id,
        { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") },
        'OWNER',
        $certificate,
        { q_device_internal_id(organization_id="$organization_id", device_id="$certified_by") },
        $timestamp
    FROM new_realm_id
),
new_timestamp AS (
    INSERT INTO realm_topic (
        organization,
        realm,
        last_timestamp
    )
    SELECT
        { q_organization_internal_id("$organization_id") },
        _id,
        $timestamp
    FROM new_realm_id
    RETURNING last_timestamp
)
SELECT true AS inserted, last_timestamp FROM new_timestamp
UNION
SELECT false AS inserted, last_timestamp
FROM realm_topic
WHERE realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
LIMIT 1
"""
)


async def query_get_realms_for_user(
    conn: AsyncpgConnection, organization_id: OrganizationID, user: UserID
) -> dict[VlobID, RealmRole]:
    rep = await conn.fetch(
        *_q_get_realms_for_user(organization_id=organization_id.str, user_id=user)
    )
    return {
        VlobID.from_hex(row["realm_id"]): RealmRole.from_str(row["role"])
        for row in rep
        if row["role"] is not None
    }


class PGRealmComponent(BaseRealmComponent):
    def __init__(self, pool: AsyncpgPool, event_bus: EventBus):
        super().__init__()
        self.pool = pool
        self.event_bus = event_bus

    def register_components(
        self, organization: PGOrganizationComponent, user: PGUserComponent, **kwargs
    ) -> None:
        self.organization = organization
        self.user = user

    async def _check_realm_topic(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        realm_id: VlobID,
        author_user_id: UserID,
    ) -> tuple[RealmRole, KeyIndex, DateTime] | RealmCheckBadOutcome:
        row = await conn.fetchrow(
            *_q_check_realm_topic(
                organization_id=organization_id.str,
                realm_id=realm_id,
                user_id=author_user_id,
            )
        )
        if not row:
            return RealmCheckBadOutcome.REALM_NOT_FOUND
        if row["role"] is None:
            return RealmCheckBadOutcome.USER_NOT_IN_REALM
        return RealmRole.from_str(row["role"]), int(row["key_index"]), row["last_timestamp"]

    async def _lock_realm_topic(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        realm_id: VlobID,
        author_user_id: UserID,
    ) -> tuple[RealmRole, KeyIndex, DateTime] | RealmCheckBadOutcome:
        row = await conn.fetchrow(
            *_q_lock_realm_topic(
                organization_id=organization_id.str,
                realm_id=realm_id,
                user_id=author_user_id,
            )
        )
        if not row:
            return RealmCheckBadOutcome.REALM_NOT_FOUND
        if row["role"] is None:
            return RealmCheckBadOutcome.USER_NOT_IN_REALM
        return RealmRole.from_str(row["role"]), int(row["key_index"]), row["last_timestamp"]

    async def _get_current_role_for_user(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        realm_id: VlobID,
        user_id: UserID,
    ) -> RealmRole | None:
        row = await conn.fetchrow(
            *_q_check_realm_topic(
                organization_id=organization_id.str,
                realm_id=realm_id,
                user_id=user_id,
            )
        )
        if not row or row["role"] is None:
            return None
        return RealmRole.from_str(row["role"])

    async def _get_realms_for_user(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, user_id: UserID
    ) -> dict[VlobID, tuple[RealmRole | None, DateTime]]:
        rows = await conn.fetch(
            *_q_get_realms_for_user(organization_id=organization_id.str, user_id=user_id)
        )
        realms = {}
        for row in rows:
            realm_id = VlobID.from_hex(row["realm_id"])
            role = RealmRole.from_str(row["role"]) if row["role"] is not None else None
            realms[realm_id] = (role, row["certified_on"])

        return realms

    async def _get_realm_ids_for_user(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, user_id: UserID
    ) -> dict[VlobID, RealmRole]:
        realms = await self._get_realms_for_user(conn, organization_id, user_id)
        return {realm_id: role for realm_id, (role, _) in realms.items() if role is not None}

    async def _get_realm_certificates_for_user(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        user: UserID,
        after: dict[VlobID, DateTime],
    ) -> dict[VlobID, list[bytes]]:
        result = {}
        realms = await self._get_realms_for_user(conn, organization_id, user)
        for realm_id, (role, certified_on) in realms.items():
            before = certified_on if role is None else None
            realm_certificates = await self._get_realm_certificates_for_realm(
                conn,
                organization_id,
                realm_id,
                after.get(realm_id),
                before,
            )
            if realm_certificates:
                result[realm_id] = realm_certificates
        return result

    async def _get_realm_certificates_for_realm(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        realm_id: VlobID,
        after: DateTime | None,
        before: DateTime | None,
    ) -> list[bytes]:
        realm_items = []
        realm_role_priority = 0
        key_rotation_priority = 1
        realm_name_priority = 2
        rows = await conn.fetch(
            *_q_get_realm_role_certificates(
                organization_id=organization_id.str, realm_id=realm_id, after=after, before=before
            )
        )
        for row in rows:
            realm_item = (
                row["certified_on"],
                realm_role_priority,
                row["certificate"],
            )
            realm_items.append(realm_item)
        rows = await conn.fetch(
            *_q_get_key_rotation_certificates(
                organization_id=organization_id.str, realm_id=realm_id, after=after, before=before
            )
        )
        for row in rows:
            realm_item = (
                row["certified_on"],
                key_rotation_priority,
                row["realm_key_rotation_certificate"],
            )
            realm_items.append(realm_item)
        rows = await conn.fetch(
            *_q_get_realm_name_certificates(
                organization_id=organization_id.str, realm_id=realm_id, after=after, before=before
            )
        )
        for row in rows:
            realm_item = (
                row["certified_on"],
                realm_name_priority,
                row["realm_name_certificate"],
            )
            realm_items.append(realm_item)

        realm_items.sort()
        return [certificate for _, _, certificate in realm_items]

    async def _has_realm_name_certificate(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, realm_id: VlobID
    ) -> bool:
        rows = await conn.fetchrow(
            *_q_get_realm_name_certificates(
                organization_id=organization_id.str,
                realm_id=realm_id,
                after=None,
                before=None,
            )
        )
        return rows is not None

    @override
    @transaction
    async def create(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        realm_role_certificate: bytes,
    ) -> (
        RealmRoleCertificate
        | CertificateBasedActionIdempotentOutcome
        | RealmCreateValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmCreateStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return RealmCreateStoreBadOutcome.ORGANIZATION_NOT_FOUND

        if org.is_expired:
            return RealmCreateStoreBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, last_common_certificate_timestamp):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return RealmCreateStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return RealmCreateStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return RealmCreateStoreBadOutcome.AUTHOR_REVOKED

        match realm_create_validate(
            now=now,
            expected_author_device_id=author,
            expected_author_user_id=author_user_id,
            author_verify_key=author_verify_key,
            realm_role_certificate=realm_role_certificate,
        ):
            case RealmRoleCertificate() as certif:
                pass
            case error:
                return error
        assert certif.role == RealmRole.OWNER

        if last_common_certificate_timestamp >= certif.timestamp:
            return RequireGreaterTimestamp(strictly_greater_than=last_common_certificate_timestamp)

        result = await conn.fetchrow(
            *_q_create_realm(
                organization_id=organization_id.str,
                realm_id=certif.realm_id,
                timestamp=certif.timestamp,
                user_id=certif.user_id,
                certificate=realm_role_certificate,
                certified_by=certif.author,
            )
        )
        assert result is not None
        inserted, topic_timestamp = result
        if not inserted:
            return CertificateBasedActionIdempotentOutcome(certificate_timestamp=topic_timestamp)

        await self.event_bus.send(
            EventRealmCertificate(
                organization_id=organization_id,
                timestamp=certif.timestamp,
                realm_id=certif.realm_id,
                user_id=certif.user_id,
                role_removed=certif.role is None,
            )
        )

        return certif

    @override
    @transaction
    async def share(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        realm_role_certificate: bytes,
        recipient_keys_bundle_access: bytes,
        key_index: int,
    ) -> (
        RealmRoleCertificate
        | BadKeyIndex
        | CertificateBasedActionIdempotentOutcome
        | RealmShareValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmShareStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return RealmShareStoreBadOutcome.ORGANIZATION_NOT_FOUND

        if org.is_expired:
            return RealmShareStoreBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, _):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return RealmShareStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return RealmShareStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return RealmShareStoreBadOutcome.AUTHOR_REVOKED

        match realm_share_validate(
            now=now,
            expected_author_user_id=author_user_id,
            expected_author_device_id=author,
            author_verify_key=author_verify_key,
            realm_role_certificate=realm_role_certificate,
        ):
            case RealmRoleCertificate() as certif:
                assert certif.role is not None
            case error:
                return error

        match await self.user._check_user(conn, organization_id, certif.user_id):
            case (target_current_profile, _):
                pass
            case CheckUserBadOutcome.USER_NOT_FOUND:
                return RealmShareStoreBadOutcome.RECIPIENT_NOT_FOUND
            case CheckUserBadOutcome.USER_REVOKED:
                return RealmShareStoreBadOutcome.RECIPIENT_REVOKED

        if target_current_profile == UserProfile.OUTSIDER and certif.role in (
            RealmRole.MANAGER,
            RealmRole.OWNER,
        ):
            return RealmShareStoreBadOutcome.ROLE_INCOMPATIBLE_WITH_OUTSIDER

        match await self._lock_realm_topic(conn, organization_id, certif.realm_id, author_user_id):
            case (current_author_role, realm_key_index, last_realm_certificate_timestamp):
                if current_author_role not in (RealmRole.OWNER, RealmRole.MANAGER):
                    return RealmShareStoreBadOutcome.AUTHOR_NOT_ALLOWED
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                return RealmShareStoreBadOutcome.REALM_NOT_FOUND
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return RealmShareStoreBadOutcome.AUTHOR_NOT_ALLOWED

        existing_target_role = await self._get_current_role_for_user(
            conn, organization_id, certif.realm_id, certif.user_id
        )
        owner_or_manager = (RealmRole.OWNER, RealmRole.MANAGER)
        requires_owner_role = (
            existing_target_role in owner_or_manager or certif.role in owner_or_manager
        )
        if requires_owner_role and current_author_role != RealmRole.OWNER:
            return RealmShareStoreBadOutcome.AUTHOR_NOT_ALLOWED

        if existing_target_role == certif.role:
            return CertificateBasedActionIdempotentOutcome(
                certificate_timestamp=last_realm_certificate_timestamp
            )

        if key_index != realm_key_index:
            return BadKeyIndex(last_realm_certificate_timestamp=last_realm_certificate_timestamp)

        # Ensure certificate consistency: our certificate must be the newest thing on the server.
        #
        # Strictly speaking consistency only requires the certificate to be more recent than
        # the the certificates involving the realm and/or the recipient user; and, similarly,
        # the vlobs created/updated by the recipient.
        #
        # However doing such precise checks is complex and error prone, so we take a simpler
        # approach by considering certificates don't change often so it's no big deal to
        # have a much more coarse approach.

        if certif.timestamp <= last_realm_certificate_timestamp:
            return RequireGreaterTimestamp(strictly_greater_than=last_realm_certificate_timestamp)

        # TODO: compare with vlob timestamps too

        # All checks are good, now we do the actual insertion
        ret = await conn.execute(
            *_q_insert_realm_user_role(
                organization_id=organization_id.str,
                realm_id=certif.realm_id,
                user_id=certif.user_id,
                role=certif.role.str,
                certificate=realm_role_certificate,
                certified_by=author,
                certified_on=certif.timestamp,
            )
        )
        assert ret == "INSERT 0 1", f"Unexpected return value: {ret}"

        ret = await conn.execute(
            *_q_insert_recipient_keys_bundle_access(
                organization_id=organization_id.str,
                realm_id=certif.realm_id,
                user_id=certif.user_id,
                access=recipient_keys_bundle_access,
                key_index=key_index,
            )
        )
        # The access might already be there, so make the insertion is idempotent
        assert ret in ("INSERT 0 1", "INSERT 0 0"), f"Unexpected return value: {ret}"

        ret = await conn.execute(
            *_q_update_realm_topic(
                organization_id=organization_id.str,
                realm_id=certif.realm_id,
                timestamp=certif.timestamp,
            )
        )
        assert ret == "UPDATE 1", f"Unexpected return value: {ret}"

        await self.event_bus.send(
            EventRealmCertificate(
                organization_id=organization_id,
                timestamp=certif.timestamp,
                realm_id=certif.realm_id,
                user_id=certif.user_id,
                role_removed=False,
            )
        )

        return certif

    @override
    @transaction
    async def unshare(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        realm_role_certificate: bytes,
    ) -> (
        RealmRoleCertificate
        | CertificateBasedActionIdempotentOutcome
        | RealmUnshareValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmUnshareStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return RealmUnshareStoreBadOutcome.ORGANIZATION_NOT_FOUND

        if org.is_expired:
            return RealmUnshareStoreBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, _):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return RealmUnshareStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return RealmUnshareStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return RealmUnshareStoreBadOutcome.AUTHOR_REVOKED

        match realm_unshare_validate(
            now=now,
            expected_author_user_id=author_user_id,
            expected_author_device_id=author,
            author_verify_key=author_verify_key,
            realm_role_certificate=realm_role_certificate,
        ):
            case RealmRoleCertificate() as certif:
                assert certif.role is None
            case error:
                return error

        match await self.user._check_user(conn, organization_id, certif.user_id):
            case tuple():
                pass
            case CheckUserBadOutcome.USER_NOT_FOUND:
                return RealmUnshareStoreBadOutcome.RECIPIENT_NOT_FOUND
            case CheckUserBadOutcome.USER_REVOKED:
                # It is allowed to unshare with a revoked user. This allows for client
                # to only check for unshare event to detect when key rotation is needed.
                pass

        match await self._lock_realm_topic(conn, organization_id, certif.realm_id, author_user_id):
            case (role, _, last_realm_certificate_timestamp):
                if role not in (RealmRole.OWNER, RealmRole.MANAGER):
                    return RealmUnshareStoreBadOutcome.AUTHOR_NOT_ALLOWED
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                return RealmUnshareStoreBadOutcome.REALM_NOT_FOUND
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return RealmUnshareStoreBadOutcome.AUTHOR_NOT_ALLOWED

        existing_target_role = await self._get_current_role_for_user(
            conn, organization_id, certif.realm_id, certif.user_id
        )
        requires_owner_role = existing_target_role in (RealmRole.OWNER, RealmRole.MANAGER)
        if requires_owner_role and role != RealmRole.OWNER:
            return RealmUnshareStoreBadOutcome.AUTHOR_NOT_ALLOWED

        if existing_target_role is None:
            return CertificateBasedActionIdempotentOutcome(
                certificate_timestamp=last_realm_certificate_timestamp
            )

        # Ensure certificate consistency: our certificate must be the newest thing on the server.
        #
        # Strictly speaking consistency only requires the certificate to be more recent than
        # the the certificates involving the realm and/or the recipient user; and, similarly,
        # the vlobs created/updated by the recipient.
        #
        # However doing such precise checks is complex and error prone, so we take a simpler
        # approach by considering certificates don't change often so it's no big deal to
        # have a much more coarse approach.

        if certif.timestamp <= last_realm_certificate_timestamp:
            return RequireGreaterTimestamp(strictly_greater_than=last_realm_certificate_timestamp)

        # TODO: compare with vlob timestamps too

        # All checks are good, now we do the actual insertion
        ret = await conn.execute(
            *_q_insert_realm_user_role(
                organization_id=organization_id.str,
                realm_id=certif.realm_id,
                user_id=certif.user_id,
                role=None,
                certificate=realm_role_certificate,
                certified_by=author,
                certified_on=certif.timestamp,
            )
        )
        assert ret == "INSERT 0 1", f"Unexpected return value: {ret}"

        ret = await conn.execute(
            *_q_update_realm_topic(
                organization_id=organization_id.str,
                realm_id=certif.realm_id,
                timestamp=certif.timestamp,
            )
        )
        assert ret == "UPDATE 1", f"Unexpected return value: {ret}"

        await self.event_bus.send(
            EventRealmCertificate(
                organization_id=organization_id,
                timestamp=certif.timestamp,
                realm_id=certif.realm_id,
                user_id=certif.user_id,
                role_removed=True,
            )
        )

        return certif

    @override
    @transaction
    async def rename(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        realm_name_certificate: bytes,
        initial_name_or_fail: bool,
    ) -> (
        RealmNameCertificate
        | BadKeyIndex
        | CertificateBasedActionIdempotentOutcome
        | RealmRenameValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmRenameStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return RealmRenameStoreBadOutcome.ORGANIZATION_NOT_FOUND

        if org.is_expired:
            return RealmRenameStoreBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, _):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return RealmRenameStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return RealmRenameStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return RealmRenameStoreBadOutcome.AUTHOR_REVOKED

        match realm_rename_validate(
            now=now,
            expected_author=author,
            author_verify_key=author_verify_key,
            realm_name_certificate=realm_name_certificate,
        ):
            case RealmNameCertificate() as certif:
                pass
            case error:
                return error

        match await self._lock_realm_topic(conn, organization_id, certif.realm_id, author_user_id):
            case (role, realm_key_index, last_realm_certificate_timestamp):
                if role != RealmRole.OWNER:
                    return RealmRenameStoreBadOutcome.AUTHOR_NOT_ALLOWED
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                return RealmRenameStoreBadOutcome.REALM_NOT_FOUND
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return RealmRenameStoreBadOutcome.AUTHOR_NOT_ALLOWED

        if realm_key_index != certif.key_index:
            return BadKeyIndex(last_realm_certificate_timestamp=last_realm_certificate_timestamp)

        if initial_name_or_fail and await self._has_realm_name_certificate(
            conn, organization_id, certif.realm_id
        ):
            return CertificateBasedActionIdempotentOutcome(
                certificate_timestamp=last_realm_certificate_timestamp
            )

        ret = await conn.execute(
            *_q_rename_realm(
                organization_id=organization_id.str,
                realm_id=certif.realm_id,
                realm_name_certificate=realm_name_certificate,
                certified_by=author,
                certified_on=certif.timestamp,
            )
        )
        assert ret == "INSERT 0 1", f"Unexpected return value: {ret}"

        ret = await conn.execute(
            *_q_update_realm_topic(
                organization_id=organization_id.str,
                realm_id=certif.realm_id,
                timestamp=certif.timestamp,
            )
        )
        assert ret == "UPDATE 1", f"Unexpected return value: {ret}"

        await self.event_bus.send(
            EventRealmCertificate(
                organization_id=organization_id,
                timestamp=certif.timestamp,
                realm_id=certif.realm_id,
                user_id=author_user_id,
                role_removed=False,
            )
        )

        return certif

    @override
    @transaction
    async def rotate_key(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        realm_key_rotation_certificate: bytes,
        per_participant_keys_bundle_access: dict[UserID, bytes],
        keys_bundle: bytes,
    ) -> (
        RealmKeyRotationCertificate
        | BadKeyIndex
        | RealmRotateKeyValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmRotateKeyStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return RealmRotateKeyStoreBadOutcome.ORGANIZATION_NOT_FOUND

        if org.is_expired:
            return RealmRotateKeyStoreBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, _):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return RealmRotateKeyStoreBadOutcome.AUTHOR_REVOKED

        match realm_rotate_key_validate(
            now=now,
            expected_author=author,
            author_verify_key=author_verify_key,
            realm_key_rotation_certificate=realm_key_rotation_certificate,
        ):
            case RealmKeyRotationCertificate() as certif:
                pass
            case error:
                return error

        match await self._lock_realm_topic(conn, organization_id, certif.realm_id, author_user_id):
            case (role, realm_key_index, last_realm_certificate_timestamp):
                if role not in (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR):
                    return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_ALLOWED
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                return RealmRotateKeyStoreBadOutcome.REALM_NOT_FOUND
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_ALLOWED

        if certif.key_index != realm_key_index + 1:
            return BadKeyIndex(
                last_realm_certificate_timestamp=last_realm_certificate_timestamp,
            )

        participants = await self._get_users_in_realm(conn, organization_id, certif.realm_id)
        if per_participant_keys_bundle_access.keys() != participants:
            return RealmRotateKeyStoreBadOutcome.PARTICIPANT_MISMATCH

        await self._add_realm_key_rotation_certificate(
            conn=conn,
            organization_id=organization_id,
            realm_key_rotation_certificate_cooked=certif,
            realm_key_rotation_certificate=realm_key_rotation_certificate,
            per_participant_keys_bundle_access=per_participant_keys_bundle_access,
            keys_bundle=keys_bundle,
        )

        await self.event_bus.send(
            EventRealmCertificate(
                organization_id=organization_id,
                timestamp=certif.timestamp,
                realm_id=certif.realm_id,
                user_id=author_user_id,
                role_removed=False,
            )
        )

        return certif

    @override
    @transaction
    async def get_keys_bundle(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        key_index: int | None,
    ) -> KeysBundle | RealmGetKeysBundleBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization() as org:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return RealmGetKeysBundleBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return RealmGetKeysBundleBadOutcome.ORGANIZATION_EXPIRED

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, _):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return RealmGetKeysBundleBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return RealmGetKeysBundleBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return RealmGetKeysBundleBadOutcome.AUTHOR_REVOKED

        match await self._check_realm_topic(conn, organization_id, realm_id, author_user_id):
            case (_, current_key_index, _):
                pass
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                return RealmGetKeysBundleBadOutcome.REALM_NOT_FOUND
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return RealmGetKeysBundleBadOutcome.AUTHOR_NOT_ALLOWED

        # `key_index` starts at 1, but the array starts at 0
        match key_index:
            case 0:
                return RealmGetKeysBundleBadOutcome.BAD_KEY_INDEX
            case None:
                key_index = current_key_index
            case int():
                pass

        # Get keys bundle
        row = await conn.fetchrow(
            *_q_get_keys_bundle(
                organization_id=organization_id.str,
                realm_id=realm_id,
                key_index=key_index,
            )
        )
        if row is None:
            return RealmGetKeysBundleBadOutcome.BAD_KEY_INDEX
        keys_bundle = row["keys_bundle"]
        realm_keys_bundle_internal_id = row["_id"]

        # Get key bundle access
        row = await conn.fetchrow(
            *_q_get_keys_bundle_access(
                realm_keys_bundle_internal_id=realm_keys_bundle_internal_id,
                user_id=author_user_id,
            )
        )
        if row is None:
            return RealmGetKeysBundleBadOutcome.ACCESS_NOT_AVAILABLE_FOR_AUTHOR
        keys_bundle_access = row["access"]

        return KeysBundle(
            key_index=key_index,
            keys_bundle_access=keys_bundle_access,
            keys_bundle=keys_bundle,
        )

    @override
    @transaction
    async def get_current_realms_for_user(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, user: UserID
    ) -> dict[VlobID, RealmRole] | RealmGetCurrentRealmsForUserBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization():
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return RealmGetCurrentRealmsForUserBadOutcome.ORGANIZATION_NOT_FOUND

        match await self.user._check_user(conn, organization_id, user):
            case tuple():
                pass
            case CheckUserBadOutcome.USER_NOT_FOUND:
                return RealmGetCurrentRealmsForUserBadOutcome.USER_NOT_FOUND
            case CheckUserBadOutcome.USER_REVOKED:
                pass

        rows = await conn.fetch(
            *_q_get_realms_for_user(organization_id=organization_id.str, user_id=user)
        )
        user_realms = {}
        for row in rows:
            if row["role"] is None:
                continue
            role = RealmRole.from_str(row["role"])
            realm_id = VlobID.from_hex(row["realm_id"])
            user_realms[realm_id] = role

        return user_realms

    async def _get_users_in_realm(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, realm_id: VlobID
    ) -> set[UserID]:
        rows = await conn.fetch(
            *_q_get_current_roles(organization_id=organization_id.str, realm_id=realm_id)
        )
        return {UserID.from_hex(row["user_id"]) for row in rows if row["role"] is not None}

    async def _add_realm_key_rotation_certificate(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        realm_key_rotation_certificate_cooked: RealmKeyRotationCertificate,
        realm_key_rotation_certificate: bytes,
        per_participant_keys_bundle_access: dict[UserID, bytes],
        keys_bundle: bytes,
    ) -> None:
        assert realm_key_rotation_certificate_cooked.author is not None
        keys_bundle_internal_id = await conn.fetchval(
            *_q_insert_keys_bundle(
                organization_id=organization_id.str,
                realm_id=realm_key_rotation_certificate_cooked.realm_id,
                key_index=realm_key_rotation_certificate_cooked.key_index,
                realm_key_rotation_certificate=realm_key_rotation_certificate,
                certified_by=realm_key_rotation_certificate_cooked.author,
                certified_on=realm_key_rotation_certificate_cooked.timestamp,
                key_canary=realm_key_rotation_certificate_cooked.key_canary,
                keys_bundle=keys_bundle,
            )
        )
        assert isinstance(keys_bundle_internal_id, int)

        def arg_gen():
            for user_id, access in per_participant_keys_bundle_access.items():
                x = _q_insert_keys_bundle_access.arg_only(
                    organization_id=organization_id.str,
                    realm_id=realm_key_rotation_certificate_cooked.realm_id,
                    realm_keys_bundle_internal_id=keys_bundle_internal_id,
                    user_id=user_id,
                    access=access,
                )
                yield x

        await conn.executemany(
            _q_insert_keys_bundle_access.sql,
            arg_gen(),
        )
        await conn.execute(
            *_q_update_key_index(
                organization_id=organization_id.str,
                realm_id=realm_key_rotation_certificate_cooked.realm_id,
                key_index=realm_key_rotation_certificate_cooked.key_index,
            )
        )

        ret = await conn.execute(
            *_q_update_realm_topic(
                organization_id=organization_id.str,
                realm_id=realm_key_rotation_certificate_cooked.realm_id,
                timestamp=realm_key_rotation_certificate_cooked.timestamp,
            )
        )
        assert ret == "UPDATE 1", f"Unexpected return value: {ret}"

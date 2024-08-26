# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    SequesterServiceID,
    VlobID,
)
from parsec.ballpark import (
    RequireGreaterTimestamp,
    TimestampOutOfBallpark,
    timestamps_in_the_ballpark,
)
from parsec.components.events import EventBus
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
    RetryNeeded,
)
from parsec.components.realm import BadKeyIndex
from parsec.components.vlob import (
    RejectedBySequesterService,
    SequesterInconsistency,
    SequesterServiceNotAvailable,
    VlobCreateBadOutcome,
)
from parsec.events import EVENT_VLOB_MAX_BLOB_SIZE, EventVlob

_q_create_fetch_data_and_lock_topics = Q(
    """
WITH my_organization AS (
    SELECT
        _id,
        is_expired
    FROM organization
    WHERE
        organization_id = $organization_id
        -- Only consider bootstrapped organizations
        AND root_verify_key IS NOT NULL
    LIMIT 1
),

-- Common topic lock must occur ASAP
my_locked_common_topic AS (
    SELECT last_timestamp
    FROM common_topic
    WHERE organization = (SELECT _id FROM my_organization)
    LIMIT 1
    FOR SHARE
),

my_realm AS (
    SELECT
        realm._id,
        key_index
    FROM realm
    INNER JOIN my_organization ON realm.organization = my_organization._id
    WHERE realm_id = $realm_id
    LIMIT 1
),

-- Realm topic lock must occur ASAP
my_locked_realm_topic AS (
    SELECT last_timestamp
    FROM realm_topic
    WHERE
        organization = (SELECT _id FROM my_organization)
        AND realm = (SELECT _id FROM my_realm)
    LIMIT 1
    FOR SHARE
),

my_device AS (
    SELECT
        device._id,
        device.user_
    FROM device
    INNER JOIN my_organization ON device.organization = my_organization._id
    WHERE device.device_id = $device_id
    LIMIT 1
),

my_user AS (
    SELECT
        user_._id,
        user_.frozen,
        (user_.revoked_on IS NOT NULL) AS revoked
    FROM user_
    INNER JOIN my_device ON user_._id = my_device.user_
    LIMIT 1
)

SELECT
    (SELECT _id FROM my_organization) AS organization_internal_id,
    (SELECT is_expired FROM my_organization) AS organization_is_expired,
    (SELECT _id FROM my_device) AS device_internal_id,
    (SELECT frozen FROM my_user) AS user_is_frozen,
    (SELECT revoked FROM my_user) AS user_is_revoked,
    (SELECT last_timestamp FROM my_locked_common_topic) AS last_common_certificate_timestamp,
    (SELECT last_timestamp FROM my_locked_realm_topic) AS last_realm_certificate_timestamp,
    (SELECT _id FROM my_realm) AS realm_internal_id,
    (SELECT key_index FROM my_realm) AS realm_key_index,
    COALESCE(
        (
            SELECT role IN ('CONTRIBUTOR', 'MANAGER', 'OWNER')
            FROM realm_user_role
            WHERE
                realm_user_role.user_ = (SELECT _id FROM my_user)
                AND realm_user_role.realm = (SELECT _id FROM my_realm)
            ORDER BY certified_on DESC
            LIMIT 1
        ),
        False
    ) AS user_can_write,
    EXISTS(
        SELECT True
        FROM vlob_atom
        WHERE
            vlob_atom.realm = (SELECT _id FROM my_realm)
            AND vlob_atom.vlob_id = $vlob_id
        LIMIT 1
    ) AS vlob_already_exists
"""
)


_q_insert_vlob = Q(
    """
WITH new_vlob AS (
    INSERT INTO vlob_atom (
        realm,
        key_index,
        vlob_id,
        version,
        blob,
        size,
        author,
        created_on
    )
    SELECT
        $realm_internal_id,
        $key_index,
        $vlob_id,
        $version,
        $blob,
        $blob_len,
        $author_internal_id,
        $timestamp
    ON CONFLICT (realm, vlob_id, version) DO NOTHING
    RETURNING _id, realm
),

new_realm_vlob_update AS (
    INSERT INTO realm_vlob_update (
        realm,
        index,
        vlob_atom
    ) (
        SELECT
            realm,
            (
                SELECT COALESCE(MAX(index), 0) + 1 AS checkpoint
                FROM realm_vlob_update
                WHERE realm_vlob_update.realm = new_vlob.realm
            ),
            _id
        FROM new_vlob
    )
    ON CONFLICT (realm, index) DO NOTHING
    RETURNING index
)

SELECT
    (SELECT _id FROM new_vlob) AS new_vlob_internal_id,
    (SELECT index FROM new_realm_vlob_update) AS new_checkpoint
"""
)


async def vlob_create(
    event_bus: EventBus,
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: VlobID,
    vlob_id: VlobID,
    key_index: int,
    timestamp: DateTime,
    blob: bytes,
    sequester_blob: dict[SequesterServiceID, bytes] | None = None,
) -> (
    None
    | BadKeyIndex
    | VlobCreateBadOutcome
    | TimestampOutOfBallpark
    | RequireGreaterTimestamp
    | RejectedBySequesterService
    | SequesterServiceNotAvailable
    | SequesterInconsistency
):
    # 1) Query the database to get all info about org/device/user/realm/vlob
    #    and lock the common & realm topics

    row = await conn.fetchrow(
        *_q_create_fetch_data_and_lock_topics(
            organization_id=organization_id.str,
            device_id=author,
            realm_id=realm_id,
            vlob_id=vlob_id,
        )
    )
    assert row is not None

    # 1.1) Check organization

    match row["organization_internal_id"]:
        case int():
            pass
        case None:
            return VlobCreateBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["organization_is_expired"]:
        case False:
            pass
        case True:
            return VlobCreateBadOutcome.ORGANIZATION_EXPIRED
        case unknown:
            assert False, repr(unknown)

    # 1.2) Check device & user

    match row["device_internal_id"]:
        case int() as author_internal_id:
            pass
        case None:
            return VlobCreateBadOutcome.AUTHOR_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    # Since device exists, it corresponding user must also exist

    match row["user_is_frozen"]:
        case False:
            pass
        case True:
            return VlobCreateBadOutcome.AUTHOR_REVOKED
        case unknown:
            assert False, repr(unknown)

    match row["user_is_revoked"]:
        case False:
            pass
        case True:
            return VlobCreateBadOutcome.AUTHOR_REVOKED
        case unknown:
            assert False, repr(unknown)

    # 1.3) Check topics

    match row["last_common_certificate_timestamp"]:
        case DateTime() as last_common_certificate_timestamp:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["last_realm_certificate_timestamp"]:
        case DateTime() as last_realm_certificate_timestamp:
            pass
        case None:
            return VlobCreateBadOutcome.REALM_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    # 1.4) Check realm
    # (Note since realm's topic exists, the realm itself must also exist)

    match row["realm_internal_id"]:
        case int() as realm_internal_id:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["user_can_write"]:
        case True:
            pass
        case False:
            return VlobCreateBadOutcome.AUTHOR_NOT_ALLOWED
        case unknown:
            assert False, repr(unknown)

    match row["realm_key_index"]:
        case int() as realm_current_key_index:
            if realm_current_key_index != key_index:
                return BadKeyIndex(
                    last_realm_certificate_timestamp=last_realm_certificate_timestamp,
                )
            pass
        case unknown:
            assert False, repr(unknown)

    # 2) Timestamp checks

    if (err := timestamps_in_the_ballpark(timestamp, now)) is not None:
        return err

    if timestamp <= last_common_certificate_timestamp:
        return RequireGreaterTimestamp(strictly_greater_than=last_common_certificate_timestamp)

    if timestamp <= last_realm_certificate_timestamp:
        return RequireGreaterTimestamp(strictly_greater_than=last_realm_certificate_timestamp)

    # 3) Sequester checks

    # TODO: Sequester behavior has changed since Parsec v2, this `sequester_blob`
    #       field is in fact no longer needed (so we will most likely drop in
    #       when implementing the support of sequester...)
    if sequester_blob is not None:
        return VlobCreateBadOutcome.ORGANIZATION_NOT_SEQUESTERED

    # 4) All checks are good except for the vlob existence, now we do the actual insertion
    #    and rely on the database constraint to be informed if the vlob already exists.

    # Note no unique violation exception can be raised here since all the insertion
    # in this query uses `ON CONFLICT DO NOTHING` (we then know if a conflict occured
    # if the returned columns are NULL).
    row = await conn.fetchrow(
        *_q_insert_vlob(
            realm_internal_id=realm_internal_id,
            author_internal_id=author_internal_id,
            key_index=key_index,
            vlob_id=vlob_id,
            blob=blob,
            blob_len=len(blob),
            timestamp=timestamp,
            version=1,
        )
    )
    assert row is not None

    match row["new_vlob_internal_id"]:
        case int():
            pass
        case None:
            # Given concurrent vlob creation is allowed, unique violation may occur !
            return VlobCreateBadOutcome.VLOB_ALREADY_EXISTS
        case unknown:
            assert False, repr(unknown)

    match row["new_checkpoint"]:
        case int() as new_checkpoint:
            assert new_checkpoint >= 1, new_checkpoint
        case None:
            # An unrelated vlob update in the realm has updated the current checkpoint
            # right between our `SELECT` that found the next checkpoint and our
            # `INSERT` that updated the current checkpoint with this value.
            # The solution here is simply to retry the query from the start.
            raise RetryNeeded
        case unknown:
            assert False, repr(unknown)

    event = EventVlob(
        organization_id=organization_id,
        author=author,
        realm_id=realm_id,
        timestamp=timestamp,
        vlob_id=vlob_id,
        version=1,
        blob=blob if len(blob) < EVENT_VLOB_MAX_BLOB_SIZE else None,
        last_common_certificate_timestamp=last_common_certificate_timestamp,
        last_realm_certificate_timestamp=last_realm_certificate_timestamp,
    )
    await event_bus.send(event)

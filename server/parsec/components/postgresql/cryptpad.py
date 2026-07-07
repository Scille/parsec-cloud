# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import DateTime, DeviceID, OrganizationID, RealmRole, VlobID
from parsec.ballpark import TimestampOutOfBallpark, timestamps_in_the_ballpark
from parsec.components.cryptpad import (
    BaseCryptpadComponent,
    CryptpadRegisterSessionBadOutcome,
    CryptpadSession,
)
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.queries import (
    AuthAndLockCommonOnlyBadOutcome,
    AuthAndLockCommonOnlyData,
    LockRealmWriteRealmBadOutcome,
    LockRealmWriteRealmData,
    auth_and_lock_common_read,
    lock_realm_read,
)
from parsec.components.postgresql.utils import (
    Q,
    q_device,
    transaction,
)
from parsec.components.realm import BadKeyIndex
from parsec.config import BackendConfig

_q_upsert_session = Q(f"""
WITH
-- The row as it was *before* this query (i.e. before `my_upsert` below runs).
-- This is what we return when `my_upsert` turns out to insert/update nothing.
my_existing_session AS (
    SELECT
        key_index,
        author,
        created_on,
        encrypted_view_key,
        encrypted_edit_key
    FROM cryptpad_session
    WHERE
        organization = $organization_internal_id
        AND document_id = $document_id
),

-- Only overwrite an existing session if we have write access and the current
-- session doesn't support edit yet. Otherwise 0 rows are affected and nothing
-- is returned here, so `my_existing_session` is used instead (see final SELECT).
my_upsert AS (
    INSERT INTO cryptpad_session (
        organization,
        document_id,
        key_index,
        author,
        created_on,
        encrypted_view_key,
        encrypted_edit_key
    ) VALUES (
        $organization_internal_id,
        $document_id,
        $key_index,
        $author_internal_id,
        $created_on,
        $encrypted_view_key,
        $encrypted_edit_key
    )
    ON CONFLICT (organization, document_id) DO UPDATE
        SET
            key_index = excluded.key_index,
            author = excluded.author,
            created_on = excluded.created_on,
            encrypted_view_key = excluded.encrypted_view_key,
            encrypted_edit_key = excluded.encrypted_edit_key
        WHERE
        $edit_allowed
        AND cryptpad_session.encrypted_edit_key IS NULL
    RETURNING key_index, author, created_on, encrypted_view_key, encrypted_edit_key
),

my_session AS (
    SELECT
        COALESCE(
            (SELECT key_index FROM my_upsert),
            (SELECT key_index FROM my_existing_session)
        ) AS key_index,
        COALESCE(
            (SELECT author FROM my_upsert),
            (SELECT author FROM my_existing_session)
        ) AS author,
        COALESCE(
            (SELECT created_on FROM my_upsert),
            (SELECT created_on FROM my_existing_session)
        ) AS created_on,
        COALESCE(
            (SELECT encrypted_view_key FROM my_upsert),
            (SELECT encrypted_view_key FROM my_existing_session)
        ) AS encrypted_view_key,
        COALESCE(
            (SELECT encrypted_edit_key FROM my_upsert),
            (SELECT encrypted_edit_key FROM my_existing_session)
        ) AS encrypted_edit_key
)

SELECT
    my_session.key_index AS session_key_index,
    my_session.created_on AS session_created_on,
    my_session.encrypted_view_key AS session_encrypted_view_key,
    my_session.encrypted_edit_key AS session_encrypted_edit_key,
    {q_device(select="device_id", _id="my_session.author")} AS session_author  -- noqa: LT14
FROM my_session
""")


class PGCryptpadComponent(BaseCryptpadComponent):
    def __init__(self, pool: AsyncpgPool, config: BackendConfig) -> None:
        super().__init__(config)
        self.pool = pool

    @override
    @transaction
    async def register_session(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        document_id: VlobID,
        key_index: int,
        timestamp: DateTime,
        encrypted_candidate_view_key: bytes,
        encrypted_candidate_edit_key: bytes | None,
    ) -> CryptpadSession | BadKeyIndex | CryptpadRegisterSessionBadOutcome | TimestampOutOfBallpark:
        # 1) Read lock common topic

        match await auth_and_lock_common_read(conn, organization_id, author):
            case AuthAndLockCommonOnlyData() as db_common:
                pass
            case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
                return CryptpadRegisterSessionBadOutcome.ORGANIZATION_NOT_FOUND
            case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
                return CryptpadRegisterSessionBadOutcome.ORGANIZATION_EXPIRED
            case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
                return CryptpadRegisterSessionBadOutcome.AUTHOR_NOT_FOUND
            case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
                return CryptpadRegisterSessionBadOutcome.AUTHOR_REVOKED

        # 2) Read lock realm topic

        match await lock_realm_read(
            conn,
            db_common.organization_internal_id,
            db_common.user_internal_id,
            realm_id,
        ):
            case LockRealmWriteRealmData() as db_realm:
                pass
            case LockRealmWriteRealmBadOutcome.REALM_NOT_FOUND:
                return CryptpadRegisterSessionBadOutcome.REALM_NOT_FOUND
            case LockRealmWriteRealmBadOutcome.REALM_DELETED:
                return CryptpadRegisterSessionBadOutcome.REALM_DELETED
            case LockRealmWriteRealmBadOutcome.USER_NOT_IN_REALM:
                return CryptpadRegisterSessionBadOutcome.AUTHOR_NOT_ALLOWED

        # 3) Check author's role and key index

        match db_realm.realm_user_current_role:
            case RealmRole.READER:
                if encrypted_candidate_edit_key is not None:
                    return CryptpadRegisterSessionBadOutcome.AUTHOR_NOT_ALLOWED
                edit_allowed = False

            case RealmRole.OWNER | RealmRole.MANAGER | RealmRole.CONTRIBUTOR:
                if encrypted_candidate_edit_key is None:
                    return CryptpadRegisterSessionBadOutcome.AUTHOR_NOT_ALLOWED
                edit_allowed = True

            case unknown:
                # TODO: Implement `Enum` on `RealmRole` so we can use `assert_never` here
                # (see https://github.com/Scille/parsec-cloud/issues/12725)
                assert False, unknown

        # We only accept the last key
        if db_realm.realm_key_index != key_index:
            return BadKeyIndex(
                last_realm_certificate_timestamp=db_realm.last_realm_certificate_timestamp
            )

        maybe_error = timestamps_in_the_ballpark(timestamp, now)
        if maybe_error is not None:
            return maybe_error

        # 4) Create the session if it doesn't exist yet, or replace it if it
        #    doesn't support edit and we have write access.

        row = await conn.fetchrow(
            *_q_upsert_session(
                organization_internal_id=db_common.organization_internal_id,
                document_id=document_id,
                key_index=key_index,
                author_internal_id=db_common.device_internal_id,
                created_on=timestamp,
                encrypted_view_key=encrypted_candidate_view_key,
                encrypted_edit_key=(encrypted_candidate_edit_key if edit_allowed else None),
                edit_allowed=edit_allowed,
            )
        )
        assert row is not None

        match row["session_key_index"]:
            case int() as session_key_index:
                pass
            case _:
                assert False, row

        match row["session_author"]:
            case str() as raw_session_author:
                session_author = DeviceID.from_hex(raw_session_author)
            case _:
                assert False, row

        match row["session_created_on"]:
            case DateTime() as session_created_on:
                pass
            case _:
                assert False, row

        match row["session_encrypted_view_key"]:
            case bytes() as session_encrypted_view_key:
                pass
            case _:
                assert False, row

        match row["session_encrypted_edit_key"]:
            case bytes() | None as session_encrypted_edit_key:
                pass
            case _:
                assert False, row

        return CryptpadSession(
            author=session_author,
            timestamp=session_created_on,
            key_index=session_key_index,
            encrypted_view_key=session_encrypted_view_key,
            encrypted_edit_key=(session_encrypted_edit_key if edit_allowed else None),
            needed_common_certificate_timestamp=db_common.last_common_certificate_timestamp,
            needed_realm_certificate_timestamp=db_realm.last_realm_certificate_timestamp,
        )

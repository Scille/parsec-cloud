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
from parsec.components.memory.datamodel import MemoryCryptpadSession, MemoryDatamodel
from parsec.components.realm import BadKeyIndex
from parsec.config import BackendConfig


class MemoryCryptpadComponent(BaseCryptpadComponent):
    def __init__(self, data: MemoryDatamodel, config: BackendConfig) -> None:
        super().__init__(config)
        self._data = data

    @override
    async def register_session(
        self,
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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return CryptpadRegisterSessionBadOutcome.ORGANIZATION_NOT_FOUND

        if org.is_expired:
            return CryptpadRegisterSessionBadOutcome.ORGANIZATION_EXPIRED

        async with org.topics_lock(read=["common", ("realm", realm_id)]) as (
            common_topic_last_timestamp,
            realm_topic_last_timestamp,
        ):
            try:
                author_device = org.devices[author]
            except KeyError:
                return CryptpadRegisterSessionBadOutcome.AUTHOR_NOT_FOUND

            author_user_id = author_device.cooked.user_id

            try:
                author_user = org.users[author_user_id]
            except KeyError:
                return CryptpadRegisterSessionBadOutcome.AUTHOR_NOT_FOUND

            if author_user.is_revoked:
                return CryptpadRegisterSessionBadOutcome.AUTHOR_REVOKED

            try:
                realm = org.realms[realm_id]
            except KeyError:
                return CryptpadRegisterSessionBadOutcome.REALM_NOT_FOUND

            if realm.is_deleted:
                return CryptpadRegisterSessionBadOutcome.REALM_DELETED

            match realm.get_current_role_for(author_user_id):
                case None:
                    return CryptpadRegisterSessionBadOutcome.AUTHOR_NOT_ALLOWED

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
            if len(realm.key_rotations) != key_index:
                return BadKeyIndex(last_realm_certificate_timestamp=realm_topic_last_timestamp)

            maybe_error = timestamps_in_the_ballpark(timestamp, now)
            if maybe_error is not None:
                return maybe_error

            session = org.cryptpad_sessions.get(document_id)
            # Create the session if it doesn't exist, or replace the session if doesn't
            # support edit and we have write access.
            if session is None or (edit_allowed and session.encrypted_edit_key is None):
                session = MemoryCryptpadSession(
                    document_id=document_id,
                    key_index=key_index,
                    encrypted_edit_key=(encrypted_candidate_edit_key if edit_allowed else None),
                    encrypted_view_key=encrypted_candidate_view_key,
                    author=author,
                    timestamp=timestamp,
                )
                org.cryptpad_sessions[document_id] = session

            return CryptpadSession(
                author=session.author,
                timestamp=session.timestamp,
                key_index=session.key_index,
                encrypted_view_key=session.encrypted_view_key,
                encrypted_edit_key=(session.encrypted_edit_key if edit_allowed else None),
                needed_common_certificate_timestamp=common_topic_last_timestamp,
                needed_realm_certificate_timestamp=realm_topic_last_timestamp,
            )

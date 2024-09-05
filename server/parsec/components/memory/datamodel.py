# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from asyncio import Event
from collections import defaultdict
from contextlib import asynccontextmanager
from copy import deepcopy
from dataclasses import dataclass, field
from enum import Enum, auto
from itertools import chain
from typing import AsyncIterator, Iterable, Literal

from parsec._parsec import (
    ActiveUsersLimit,
    BlockID,
    BootstrapToken,
    CancelledGreetingAttemptReason,
    DateTime,
    DeviceCertificate,
    DeviceID,
    EnrollmentID,
    GreeterOrClaimer,
    GreetingAttemptID,
    InvitationToken,
    InvitationType,
    OrganizationID,
    RealmArchivingCertificate,
    RealmKeyRotationCertificate,
    RealmNameCertificate,
    RealmRole,
    RealmRoleCertificate,
    RevokedUserCertificate,
    SequesterAuthorityCertificate,
    SequesterRevokedServiceCertificate,
    SequesterServiceCertificate,
    SequesterServiceID,
    ShamirRecoveryBriefCertificate,
    UserCertificate,
    UserID,
    UserProfile,
    UserUpdateCertificate,
    VerifyKey,
    VlobID,
)
from parsec.components.sequester import SequesterServiceType


class AdvisoryLock(Enum):
    """
    Advisory lock must be taken for certain operations to avoid concurrency issue.

    - Invitation creation: Only one active invitation is allowed per email, this is something
      that cannot be enforced purely in PostgreSQL with unique constraint (given previous
      invitations merely got a deleted flag set).
    """

    InvitationCreation = auto()


TopicAndDiscriminant = (
    Literal["common"]
    | Literal["sequester"]
    | tuple[Literal["realm"], VlobID]
    | Literal["shamir_recovery"]
    # Not an actual topic, but it is convenient to implement advisory lock this
    # way since in practice it works similarly.
    | tuple[Literal["__advisory_lock"], AdvisoryLock]
)


@dataclass(slots=True)
class MemoryDatamodel:
    organizations: dict[OrganizationID, MemoryOrganization] = field(default_factory=dict)


@dataclass(slots=True)
class MemoryOrganization:
    organization_id: OrganizationID
    bootstrap_token: BootstrapToken | None
    created_on: DateTime
    active_users_limit: ActiveUsersLimit
    user_profile_outsider_allowed: bool
    bootstrapped_on: DateTime | None = None
    # None for non-sequestered organization
    sequester_authority_certificate: bytes | None = field(default=None, repr=False)
    cooked_sequester_authority: SequesterAuthorityCertificate | None = None
    root_verify_key: VerifyKey | None = field(default=None, repr=False)
    is_expired: bool = False
    minimum_archiving_period: int = 2592000  # 30 days

    # None for non-sequestered organization
    sequester_services: dict[SequesterServiceID, MemorySequesterService] | None = None
    users: dict[UserID, MemoryUser] = field(default_factory=dict)
    devices: dict[DeviceID, MemoryDevice] = field(default_factory=dict)
    invitations: dict[InvitationToken, MemoryInvitation] = field(default_factory=dict)
    greeting_attempts: dict[GreetingAttemptID, MemoryGreetingAttempt] = field(default_factory=dict)
    pki_enrollments: dict[EnrollmentID, MemoryPkiEnrollment] = field(default_factory=dict)
    realms: dict[VlobID, MemoryRealm] = field(default_factory=dict)
    vlobs: dict[VlobID, list[MemoryVlobAtom]] = field(default_factory=dict)
    blocks: dict[BlockID, MemoryBlock] = field(default_factory=dict)
    block_store: dict[BlockID, bytes] = field(default_factory=dict, repr=False)
    # The user id is the author of the shamir recovery process
    # TODO after https://github.com/Scille/parsec-cloud/issues/7364
    # keep previous setups dict[UserID, List[MemoryShamirSetup | MemoryShamirRemoval]]
    # see https://github.com/Scille/parsec-cloud/pull/7324#discussion_r1616803899
    shamir_setup: dict[UserID, MemoryShamirSetup] = field(default_factory=dict)
    per_topic_last_timestamp: dict[TopicAndDiscriminant, DateTime] = field(default_factory=dict)

    # Stores topic name and discriminant (or `None`)
    _topic_write_locked: set[TopicAndDiscriminant] = field(default_factory=set)
    # Stores topic name and discriminant (or `None`) as key, and the number
    # of concurrent read operations currently taking the lock as key
    _topic_read_locked: dict[TopicAndDiscriminant, int] = field(
        default_factory=lambda: defaultdict(lambda: 0)
    )
    _notify_me_on_topic_lock_release: Event | None = None

    @asynccontextmanager
    async def advisory_lock_exclusive(self, lock: AdvisoryLock) -> AsyncIterator[None]:
        """
        Equivalent to `SELECT pg_advisory_xact_lock(<lock ID>, _id) FROM organization WHERE organization_id = <org>`
        """
        async with self.topics_lock(write=[("__advisory_lock", lock)]):
            yield

    @asynccontextmanager
    async def advisory_lock_shared(self, lock: AdvisoryLock) -> AsyncIterator[None]:
        """
        Equivalent to `SELECT pg_advisory_xact_lock_shared(<lock ID>, _id) FROM organization WHERE organization_id = <org>`
        """
        async with self.topics_lock(write=[("__advisory_lock", lock)]):
            yield

    @asynccontextmanager
    async def topics_lock(
        self, read: Iterable[TopicAndDiscriminant] = (), write: Iterable[TopicAndDiscriminant] = ()
    ) -> AsyncIterator[tuple[DateTime, ...]]:
        """
        Read is equivalent to `SELECT last_timestamp FROM <topic table> WHERE organization_id = <org> FOR SHARE`

        Write is equivalent to `SELECT last_timestamp FROM <topic table> WHERE organization_id = <org> FOR UPDATE`
        """
        while True:
            # 1) Check the locks we want aren't already taken in write mode
            if any(
                topic_and_discriminant in self._topic_write_locked
                for topic_and_discriminant in chain(read, write)
            ):
                if self._notify_me_on_topic_lock_release is None:
                    self._notify_me_on_topic_lock_release = Event()
                await self._notify_me_on_topic_lock_release.wait()
                # The event we waited on doesn't specify which lock has been
                # released, so we must retry from the beginning.
                continue

            # 2) Check the write locks we want aren't already taken in read mode
            if any(
                self._topic_read_locked.get(topic_and_discriminant, 0) != 0
                for topic_and_discriminant in write
            ):
                if self._notify_me_on_topic_lock_release is None:
                    self._notify_me_on_topic_lock_release = Event()
                await self._notify_me_on_topic_lock_release.wait()
                # The event we waited on doesn't specify which lock has been
                # released, so we must retry from the beginning.
                continue

            # 3) All checks are good, we can now take the locks !

            self._topic_write_locked.update(write)
            for topic_and_discriminant in read:
                self._topic_read_locked[topic_and_discriminant] += 1

            # Default to epoch (1970-01-01) to spare the caller from having to deal
            # with corner-cases (e.g. when creating realm or bootstrapping org)
            default_last_timestamp = DateTime.from_timestamp_seconds(0)
            per_topic_last_timestamps = tuple(
                self.per_topic_last_timestamp.get(topic_and_discriminant, default_last_timestamp)
                for topic_and_discriminant in chain(read, write)
            )
            try:
                yield per_topic_last_timestamps

            finally:
                # 4) Release our locks and leave

                self._topic_write_locked.difference_update(write)
                for topic_and_discriminant in read:
                    self._topic_read_locked[topic_and_discriminant] -= 1

                if self._notify_me_on_topic_lock_release is not None:
                    self._notify_me_on_topic_lock_release.set()
                    self._notify_me_on_topic_lock_release = Event()

            return

    def clone_as(self, new_organization_id: OrganizationID) -> MemoryOrganization:
        cloned = deepcopy(self)
        cloned.organization_id = new_organization_id
        cloned._topic_write_locked.clear()
        cloned._topic_read_locked.clear()
        cloned._notify_me_on_topic_lock_release = None

        return cloned

    def active_users(self) -> Iterable[MemoryUser]:
        for user in self.users.values():
            if not user.is_revoked:
                yield user

    def active_user_limit_reached(self) -> bool:
        active_users = sum(0 if u.is_revoked else 1 for u in self.users.values())
        return self.active_users_limit <= ActiveUsersLimit.limited_to(active_users)

    @property
    def is_sequestered(self) -> bool:
        return self.sequester_authority_certificate is not None

    @property
    def is_bootstrapped(self) -> bool:
        return self.bootstrapped_on is not None


@dataclass(slots=True)
class MemorySequesterService:
    cooked: SequesterServiceCertificate
    sequester_service_certificate: bytes = field(repr=False)
    service_type: SequesterServiceType
    webhook_url: str | None
    # None if not yet revoked
    cooked_revoked: SequesterRevokedServiceCertificate | None = None
    sequester_revoked_service_certificate: bytes | None = field(default=None, repr=False)

    @property
    def is_revoked(self) -> bool:
        return self.sequester_revoked_service_certificate is not None


@dataclass(slots=True)
class MemoryUserProfileUpdate:
    cooked: UserUpdateCertificate
    user_update_certificate: bytes = field(repr=False)


@dataclass(slots=True)
class MemoryUser:
    cooked: UserCertificate
    user_certificate: bytes = field(repr=False)
    redacted_user_certificate: bytes = field(repr=False)
    profile_updates: list[MemoryUserProfileUpdate] = field(default_factory=list)
    # None if not yet revoked
    cooked_revoked: RevokedUserCertificate | None = None
    revoked_user_certificate: bytes | None = field(default=None, repr=False)
    # Should be updated each time a new vlob is created/updated
    last_vlob_operation_timestamp: DateTime | None = None
    is_frozen: bool = False

    @property
    def current_profile(self) -> UserProfile:
        try:
            return self.profile_updates[-1].cooked.new_profile
        except IndexError:
            return self.cooked.profile

    @property
    def is_revoked(self) -> bool:
        return self.revoked_user_certificate is not None


@dataclass(slots=True)
class MemoryDevice:
    cooked: DeviceCertificate
    device_certificate: bytes = field(repr=False)
    redacted_device_certificate: bytes = field(repr=False)


class MemoryInvitationDeletedReason(Enum):
    FINISHED = auto()
    CANCELLED = auto()


@dataclass(slots=True)
class MemoryInvitation:
    token: InvitationToken
    type: InvitationType
    created_by_user_id: UserID
    created_by_device_id: DeviceID
    # Required for when type=USER
    claimer_email: str | None
    created_on: DateTime
    deleted_on: DateTime | None = None
    deleted_reason: MemoryInvitationDeletedReason | None = None

    # New fields for the new invitation system
    # TODO: remove the old fields once the new system is fully deployed
    greeting_sessions: dict[UserID, MemoryGreetingSession] = field(default_factory=dict)

    @property
    def is_deleted(self) -> bool:
        return self.deleted_on is not None

    @property
    def is_completed(self) -> bool:
        return self.deleted_reason == MemoryInvitationDeletedReason.FINISHED

    @property
    def is_cancelled(self) -> bool:
        return self.deleted_reason == MemoryInvitationDeletedReason.CANCELLED

    def get_greeting_session(self, user_id: UserID) -> MemoryGreetingSession:
        try:
            return self.greeting_sessions[user_id]
        except KeyError:
            greeting_session = MemoryGreetingSession(
                token=self.token,
                greeter_id=user_id,
            )
            return self.greeting_sessions.setdefault(user_id, greeting_session)


@dataclass(slots=True)
class MemoryGreetingSession:
    # Immutable properties
    token: InvitationToken
    greeter_id: UserID

    # Mutable properties
    greeting_attempts: list[GreetingAttemptID] = field(default_factory=list)

    def get_active_greeting_attempt(self, org: MemoryOrganization) -> MemoryGreetingAttempt:
        for attempt_id in self.greeting_attempts:
            attempt = org.greeting_attempts[attempt_id]
            if attempt.is_active():
                return attempt
        attempt = MemoryGreetingAttempt(
            greeting_attempt=GreetingAttemptID.new(),
            token=self.token,
            greeter_id=self.greeter_id,
        )
        org.greeting_attempts[attempt.greeting_attempt] = attempt
        self.greeting_attempts.append(attempt.greeting_attempt)
        return attempt

    def new_attempt_for_greeter(
        self, org: MemoryOrganization, now: DateTime
    ) -> MemoryGreetingAttempt:
        current_attempt = self.get_active_greeting_attempt(org)
        current_attempt.greeter_join_or_cancel(now)
        if current_attempt.is_active():
            return current_attempt
        current_attempt = self.get_active_greeting_attempt(org)
        current_attempt.greeter_join_or_cancel(now)
        assert current_attempt.is_active()
        return current_attempt

    def new_attempt_for_claimer(
        self, org: MemoryOrganization, now: DateTime
    ) -> MemoryGreetingAttempt:
        current_attempt = self.get_active_greeting_attempt(org)
        current_attempt.claimer_join_or_cancel(now)
        if current_attempt.is_active():
            return current_attempt
        current_attempt = self.get_active_greeting_attempt(org)
        current_attempt.claimer_join_or_cancel(now)
        assert current_attempt.is_active()
        return current_attempt


@dataclass(slots=True)
class MemoryGreetingAttempt:
    # Immutable properties
    greeting_attempt: GreetingAttemptID
    token: InvitationToken
    greeter_id: UserID

    # Mutable properties
    claimer_joined: None | DateTime = None
    greeter_joined: None | DateTime = None
    cancelled_reason: None | tuple[GreeterOrClaimer, CancelledGreetingAttemptReason, DateTime] = (
        None
    )
    greeter_steps: list[bytes] = field(default_factory=list)
    claimer_steps: list[bytes] = field(default_factory=list)

    class StepOutcome(Enum):
        MISMATCH = auto()
        NOT_READY = auto()
        TOO_ADVANCED = auto()

    def is_active(self) -> bool:
        return self.cancelled_reason is None

    def greeter_cancel(
        self,
        now: DateTime,
        reason: CancelledGreetingAttemptReason = CancelledGreetingAttemptReason.AUTOMATICALLY_CANCELLED,
    ):
        self.cancelled_reason = (GreeterOrClaimer.GREETER, reason, now)

    def claimer_cancel(
        self,
        now: DateTime,
        reason: CancelledGreetingAttemptReason = CancelledGreetingAttemptReason.AUTOMATICALLY_CANCELLED,
    ):
        self.cancelled_reason = (GreeterOrClaimer.CLAIMER, reason, now)

    def greeter_join_or_cancel(self, now: DateTime):
        match self.greeter_joined:
            case None:
                self.greeter_joined = now
            case DateTime():
                self.greeter_cancel(now)

    def claimer_join_or_cancel(self, now: DateTime):
        match self.claimer_joined:
            case None:
                self.claimer_joined = now
            case DateTime():
                self.claimer_cancel(now)

    def greeter_step(self, index: int, payload: bytes) -> bytes | StepOutcome:
        if index < len(self.greeter_steps) and self.greeter_steps[index] != payload:
            return self.StepOutcome.MISMATCH
        if index > len(self.greeter_steps) or index > len(self.claimer_steps):
            return self.StepOutcome.TOO_ADVANCED
        if index == len(self.greeter_steps):
            self.greeter_steps.append(payload)
        if index >= len(self.claimer_steps):
            return self.StepOutcome.NOT_READY
        return self.claimer_steps[index]

    def claimer_step(self, index: int, payload: bytes) -> bytes | StepOutcome:
        if index < len(self.claimer_steps) and self.claimer_steps[index] != payload:
            return self.StepOutcome.MISMATCH
        if index > len(self.greeter_steps) or index > len(self.claimer_steps):
            return self.StepOutcome.TOO_ADVANCED
        if index == len(self.claimer_steps):
            self.claimer_steps.append(payload)
        if index >= len(self.greeter_steps):
            return self.StepOutcome.NOT_READY
        return self.greeter_steps[index]


class MemoryPkiEnrollmentState(Enum):
    SUBMITTED = auto()
    ACCEPTED = auto()
    REJECTED = auto()
    CANCELLED = auto()


@dataclass(slots=True)
class MemoryPkiEnrollmentInfoAccepted:
    accepted_on: DateTime
    accepter_der_x509_certificate: bytes = field(repr=False)
    accept_payload_signature: bytes = field(repr=False)
    accept_payload: bytes = field(repr=False)


@dataclass(slots=True)
class MemoryPkiEnrollmentInfoRejected:
    rejected_on: DateTime


@dataclass(slots=True)
class MemoryPkiEnrollmentInfoCancelled:
    cancelled_on: DateTime


@dataclass(slots=True)
class MemoryPkiEnrollment:
    enrollment_id: EnrollmentID
    submitter_der_x509_certificate: bytes = field(repr=False)
    submitter_der_x509_certificate_sha1: bytes = field(repr=False)

    submit_payload_signature: bytes = field(repr=False)
    submit_payload: bytes = field(repr=False)
    submitted_on: DateTime

    accepter: DeviceID | None = None
    submitter_accepted_user_id: UserID | None = None
    submitter_accepted_device_id: DeviceID | None = None

    enrollment_state: MemoryPkiEnrollmentState = MemoryPkiEnrollmentState.SUBMITTED
    info_accepted: MemoryPkiEnrollmentInfoAccepted | None = None
    info_rejected: MemoryPkiEnrollmentInfoRejected | None = None
    info_cancelled: MemoryPkiEnrollmentInfoCancelled | None = None


@dataclass(slots=True)
class MemoryRealm:
    realm_id: VlobID
    created_on: DateTime
    roles: list[MemoryRealmUserRole]
    vlob_updates: list[MemoryRealmVlobUpdate] = field(default_factory=list)
    key_rotations: list[MemoryRealmKeyRotation] = field(default_factory=list)
    renames: list[MemoryRealmRename] = field(default_factory=list)
    archivings: list[MemoryRealmArchiving] = field(default_factory=list)
    last_vlob_timestamp: DateTime | None = None

    def get_current_role_for(self, user_id: UserID) -> RealmRole | None:
        for role in reversed(self.roles):
            if role.cooked.user_id == user_id:
                return role.cooked.role
        return None

    def is_personal(self) -> bool:
        """
        A personal realm is a realm that has never been shared.
        """
        # If the realm is not shared, then only a single user is part of it.
        # And given it is not possible to change his own role, then there must
        # only be a single certificate !
        return len(self.roles) == 1


@dataclass(slots=True)
class MemoryRealmVlobUpdate:
    index: int
    vlob_atom: MemoryVlobAtom


@dataclass(slots=True)
class MemoryRealmKeyRotation:
    cooked: RealmKeyRotationCertificate
    realm_key_rotation_certificate: bytes = field(repr=False)
    per_participant_keys_bundle_access: dict[UserID, bytes] = field(repr=False)
    keys_bundle: bytes = field(repr=False)


@dataclass(slots=True)
class MemoryRealmRename:
    cooked: RealmNameCertificate
    realm_name_certificate: bytes = field(repr=False)


@dataclass(slots=True)
class MemoryRealmArchiving:
    cooked: RealmArchivingCertificate
    realm_archiving_certificate: bytes = field(repr=False)


@dataclass(slots=True)
class MemoryRealmUserRole:
    cooked: RealmRoleCertificate
    realm_role_certificate: bytes = field(repr=False)


@dataclass(slots=True)
class MemoryRealmUserChange:
    user: UserID
    # The last time this user changed the role of another user
    last_role_change: DateTime | None
    # The last time this user updated a vlob
    last_vlob_update: DateTime | None


@dataclass(slots=True)
class MemoryVlobAtom:
    realm_id: VlobID
    vlob_id: VlobID
    key_index: int
    version: int
    blob: bytes = field(repr=False)
    author: DeviceID
    created_on: DateTime
    # None for non-sequestered organization
    blob_for_storage_sequester_services: dict[SequesterServiceID, bytes] | None = field(repr=False)
    # None if not deleted
    deleted_on: DateTime | None = None


@dataclass(slots=True)
class MemoryBlock:
    realm_id: VlobID
    block_id: BlockID
    key_index: int
    author: DeviceID
    block_size: int
    created_on: DateTime
    # None if not deleted
    deleted_on: DateTime | None = None


@dataclass(slots=True)
class MemoryShamirSetup:
    # The actual data we want to recover.
    # It is encrypted with `data_key` that is itself split into shares.
    # This should contains a serialized `LocalDevice`
    ciphered_data: bytes
    # The token the claimer should provide to get access to `ciphered_data`.
    # This token is split into shares, hence it acts as a proof the claimer
    # asking for the `ciphered_data` had it identity confirmed by the recipients.
    reveal_token: InvitationToken
    # The Shamir recovery setup provided as a `ShamirRecoveryBriefCertificate`.
    # It contains the threshold for the quorum and the shares recipients.
    # This field has a certain level of duplication with the "shares" below,
    # but they are used for different things (we provide the encrypted share
    # data only when needed)
    brief: ShamirRecoveryBriefCertificate
    # The shares provided as a `ShamirRecoveryShareCertificate` since
    # each share is aimed at a specific recipient.
    shares: dict[UserID, bytes]
    # keep raw brief
    brief_bytes: bytes

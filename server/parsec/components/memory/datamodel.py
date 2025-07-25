# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from asyncio import Event, Lock
from collections import defaultdict
from collections.abc import AsyncIterator, Iterable, Iterator
from contextlib import asynccontextmanager
from copy import deepcopy
from dataclasses import dataclass, field
from enum import Enum, auto
from itertools import chain
from typing import Literal

from parsec._parsec import (
    AccountAuthMethodID,
    ActiveUsersLimit,
    BlockID,
    BootstrapToken,
    CancelledGreetingAttemptReason,
    DateTime,
    DeviceCertificate,
    DeviceID,
    EmailAddress,
    EnrollmentID,
    GreeterOrClaimer,
    GreetingAttemptID,
    HashDigest,
    HumanHandle,
    InvitationStatus,
    InvitationToken,
    InvitationType,
    OrganizationID,
    RealmArchivingCertificate,
    RealmKeyRotationCertificate,
    RealmNameCertificate,
    RealmRole,
    RealmRoleCertificate,
    RevokedUserCertificate,
    SecretKey,
    SequesterAuthorityCertificate,
    SequesterRevokedServiceCertificate,
    SequesterServiceCertificate,
    SequesterServiceID,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryDeletionCertificate,
    ShamirRecoveryShareCertificate,
    UntrustedPasswordAlgorithm,
    UserCertificate,
    UserID,
    UserProfile,
    UserUpdateCertificate,
    VerifyKey,
    VlobID,
)
from parsec.components.account import ValidationCodeInfo
from parsec.components.invite import InvitationCreatedBy
from parsec.components.organization import TermsOfService
from parsec.components.sequester import SequesterServiceType
from parsec.config import AccountVaultStrategy, AllowedClientAgent

type CommonTopicCertificate = (
    UserCertificate
    | DeviceCertificate
    | UserUpdateCertificate
    | SequesterServiceCertificate
    | RevokedUserCertificate
)
type SequesterTopicCertificate = (
    SequesterAuthorityCertificate | SequesterServiceCertificate | SequesterRevokedServiceCertificate
)
type RealmTopicCertificate = (
    RealmRoleCertificate
    | RealmKeyRotationCertificate
    | RealmNameCertificate
    | RealmArchivingCertificate
)


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
    # accounts are not associated to one organization
    # (email, account)
    accounts: dict[EmailAddress, MemoryAccount] = field(default_factory=dict)
    account_create_validation_emails: dict[EmailAddress, ValidationCodeInfo] = field(
        default_factory=dict
    )
    account_delete_validation_emails: dict[EmailAddress, ValidationCodeInfo] = field(
        default_factory=dict
    )
    account_recover_validation_emails: dict[EmailAddress, ValidationCodeInfo] = field(
        default_factory=dict
    )
    # Single big lock used for all account creation steps (i.e. sending validation mail,
    # checking validation code, creating account)
    account_creation_lock: Lock = field(default_factory=Lock)

    def get_account_from_active_auth_method(
        self, auth_method_id: AccountAuthMethodID
    ) -> tuple[MemoryAccount, MemoryAuthenticationMethod] | None:
        for account in self.accounts.values():
            try:
                auth_method = account.current_vault.authentication_methods[auth_method_id]
            except KeyError:
                continue

            if auth_method.disabled_on:
                return None
            # Sanity check since auth methods gets disabled when the account is deleted
            assert account.deleted_on is None

            return (account, auth_method)

    def get_account_from_any_auth_method(
        self, auth_method_id: AccountAuthMethodID
    ) -> tuple[MemoryAccount, MemoryAuthenticationMethod] | None:
        for account in self.accounts.values():
            for vault in (account.current_vault, *account.previous_vaults):
                try:
                    return (account, vault.authentication_methods[auth_method_id])
                except KeyError:
                    pass


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
    tos: TermsOfService | None = None
    allowed_client_agent: AllowedClientAgent = AllowedClientAgent.NATIVE_OR_WEB
    account_vault_strategy: AccountVaultStrategy = AccountVaultStrategy.ALLOWED

    # None for non-sequestered organization
    sequester_services: dict[SequesterServiceID, MemorySequesterService] | None = None
    users: dict[UserID, MemoryUser] = field(default_factory=dict)
    devices: dict[DeviceID, MemoryDevice] = field(default_factory=dict)
    invitations: dict[InvitationToken, MemoryInvitation] = field(default_factory=dict)
    greeting_attempts: dict[GreetingAttemptID, MemoryGreetingAttempt] = field(default_factory=dict)
    pki_enrollments: dict[EnrollmentID, MemoryPkiEnrollment] = field(default_factory=dict)
    realms: dict[VlobID, MemoryRealm] = field(default_factory=dict)
    blocks: dict[BlockID, MemoryBlock] = field(default_factory=dict)
    block_store: dict[BlockID, bytes] = field(default_factory=dict, repr=False)
    # The user id is the author of the shamir recovery
    shamir_recoveries: dict[UserID, list[MemoryShamirRecovery]] = field(
        default_factory=lambda: defaultdict(list)
    )
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

    def active_sequester_services(self) -> Iterable[MemorySequesterService]:
        services = self.sequester_services.values() if self.sequester_services else ()
        return (service for service in services if not service.is_revoked)

    @property
    def is_bootstrapped(self) -> bool:
        return self.bootstrapped_on is not None

    def ordered_common_certificates(
        self, redacted: bool = False
    ) -> Iterator[tuple[DateTime, bytes, CommonTopicCertificate]]:
        """
        Return all common certificates ordered by timestamp.
        """
        # Certificates must be returned ordered by timestamp, however there is a trick
        # for the common certificates: when a new user is created, the corresponding
        # user and device certificates have the same timestamp, but we must return
        # the user certificate first (given device references the user).
        # So to achieve this we use a tuple (timestamp, priority, certificate) where
        # only the first two field should be used for sorting (the priority field
        # handling the case where user and device have the same timestamp).

        common_certificates_unordered: list[
            tuple[DateTime, int, bytes, CommonTopicCertificate]
        ] = []
        for user in self.users.values():
            if redacted:
                common_certificates_unordered.append(
                    (user.cooked.timestamp, 0, user.redacted_user_certificate, user.cooked)
                )
            else:
                common_certificates_unordered.append(
                    (user.cooked.timestamp, 0, user.user_certificate, user.cooked)
                )

            if user.is_revoked:
                assert user.cooked_revoked is not None
                assert user.revoked_user_certificate is not None
                common_certificates_unordered.append(
                    (
                        user.cooked_revoked.timestamp,
                        1,
                        user.revoked_user_certificate,
                        user.cooked_revoked,
                    )
                )

            # user's profile update certificates
            common_certificates_unordered.extend(
                [
                    (
                        profile_update.cooked.timestamp,
                        1,
                        profile_update.user_update_certificate,
                        profile_update.cooked,
                    )
                    for profile_update in user.profile_updates
                ]
            )

        for device in self.devices.values():
            if redacted:
                common_certificates_unordered.append(
                    (device.cooked.timestamp, 1, device.redacted_device_certificate, device.cooked)
                )
            else:
                common_certificates_unordered.append(
                    (device.cooked.timestamp, 1, device.device_certificate, device.cooked)
                )

        for ts, _, raw, cooked in sorted(common_certificates_unordered, key=lambda x: (x[0], x[1])):
            yield (ts, raw, cooked)

    def ordered_sequester_certificates(
        self,
    ) -> Iterator[tuple[DateTime, bytes, SequesterTopicCertificate]]:
        """
        Return all sequester certificates ordered by timestamp.
        """
        if self.sequester_authority_certificate is None:
            return

        assert self.cooked_sequester_authority is not None
        assert self.sequester_services is not None

        yield (
            self.cooked_sequester_authority.timestamp,
            self.sequester_authority_certificate,
            self.cooked_sequester_authority,
        )

        sequester_services_unordered: list[tuple[DateTime, bytes, SequesterTopicCertificate]] = []
        for service in self.sequester_services.values():
            sequester_services_unordered.append(
                (service.cooked.timestamp, service.sequester_service_certificate, service.cooked)
            )
            if service.cooked_revoked:
                assert service.sequester_revoked_service_certificate is not None
                sequester_services_unordered.append(
                    (
                        service.cooked_revoked.timestamp,
                        service.sequester_revoked_service_certificate,
                        service.cooked_revoked,
                    )
                )

        yield from sorted(sequester_services_unordered, key=lambda x: x[0])

    def ordered_realm_certificates(
        self, realm_id: VlobID
    ) -> Iterator[tuple[DateTime, bytes, RealmTopicCertificate]]:
        """
        Return all realm certificates ordered by timestamp.
        """
        realm = self.realms[realm_id]

        # Collect all the certificates related to the realm
        realm_certificates_unordered = []
        realm_certificates_unordered += [
            (role.cooked.timestamp, role.realm_role_certificate, role.cooked)
            for role in realm.roles
        ]
        realm_certificates_unordered += [
            (role.cooked.timestamp, role.realm_key_rotation_certificate, role.cooked)
            for role in realm.key_rotations
        ]
        realm_certificates_unordered += [
            (role.cooked.timestamp, role.realm_name_certificate, role.cooked)
            for role in realm.renames
        ]
        # TODO #6092: support archiving here !

        yield from sorted(realm_certificates_unordered, key=lambda x: x[0])

    def simulate_postgresql_block_table(self) -> Iterable[tuple[int, MemoryBlock]]:
        """
        Simulate the PostgreSQL table the blocks are supposed to be stored into.

        This is useful for the realm export feature, since it relies on the table
        sequential primary key to determine which row should be exported.

        This returns a list of blocks with their sequential primary key.
        """

        # Here we simulate the sequential primary key of the blocks table in PostgreSQL:
        # - Blocks are stored in a dict, in Python a dict is guaranteed to be ordered
        #   according to insertion order.
        # - We never remove blocks from the dict.
        # - From the above two points, we can reliably use the index of the block in the
        #   dict as the primary key.

        # PostgreSQL sequential index starts at 1, however here we skip a bunch of
        # indexes as a poor man's way to simulate the fact there can be holes in the
        # indexes (e.g. when a transaction is rolled back).
        # This should be enough to detect typical improper use of the primary key as
        # a list offset.
        return enumerate(self.blocks.values(), start=100)

    def simulate_postgresql_vlob_atom_table(self) -> Iterable[tuple[int, MemoryVlobAtom]]:
        """
        Simulate the PostgreSQL table the vlob atoms are supposed to be stored into.

        This is useful for the realm export feature, since it relies on the table
        sequential primary key to determine which row should be exported.

        This returns a list of vlob atoms with their sequential primary key.
        """

        # Simulating the sequential primary key of the vlobs table in PostgreSQL is a bit
        # more tricky than for blocks: we cannot directly rely on the dict since it itself
        # contains a list of vlob atoms (i.e. a vlob is composed of multiple versions
        # called atoms).
        # So we first hove to re-create a list of all vlob atoms across all realms,
        # sort them by creation date, which is basically equivalent of what the vlobs
        # table in PostgreSQL is.

        all_vlob_atoms: list[MemoryVlobAtom] = []
        for realm in self.realms.values():
            for vlob in realm.vlobs.values():
                all_vlob_atoms.extend(vlob)

        # Note we also order by vlob ID to ensure a stable order in case of same creation date
        all_vlob_atoms.sort(key=lambda vlob_atom: (vlob_atom.created_on, vlob_atom.vlob_id))

        # PostgreSQL sequential index starts at 1, however here we skip a bunch of
        # indexes as a poor man's way to simulate the fact there can be holes in the
        # indexes (e.g. when a transaction is rolled back).
        # This should be enough to detect typical improper use of the primary key as
        # a list offset.
        return enumerate(all_vlob_atoms, start=200)


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
    # None if not yet accepted (or nothing to accept)
    tos_accepted_on: DateTime | None = None

    @property
    def current_profile(self) -> UserProfile:
        try:
            return self.profile_updates[-1].cooked.new_profile
        except IndexError:
            return self.cooked.profile

    @property
    def is_revoked(self) -> bool:
        return self.revoked_user_certificate is not None

    @property
    def revoked_on(self) -> DateTime | None:
        return self.cooked_revoked.timestamp if self.cooked_revoked else None


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
    created_by: InvitationCreatedBy

    # Required when type=USER or type=SHAMIR_RECOVERY
    claimer_email: EmailAddress | None

    # Required when type=DEVICE or type=SHAMIR_RECOVERY
    claimer_user_id: UserID | None

    # Required when type=SHAMIR_RECOVERY
    shamir_recovery_index: int | None

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

    @property
    def invitation_status(self) -> InvitationStatus:
        if self.deleted_reason is not None:
            match self.deleted_reason:
                case MemoryInvitationDeletedReason.CANCELLED:
                    return InvitationStatus.CANCELLED
                case MemoryInvitationDeletedReason.FINISHED:
                    return InvitationStatus.FINISHED
        return InvitationStatus.PENDING

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
    vlobs: dict[VlobID, list[MemoryVlobAtom]] = field(default_factory=dict)

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
    per_participant_keys_bundle_accesses: dict[UserID, list[tuple[DateTime, bytes]]] = field(
        repr=False
    )
    keys_bundle: bytes = field(repr=False)
    # None for non-sequestered organization
    per_sequester_service_keys_bundle_access: dict[SequesterServiceID, bytes] | None = field(
        repr=False
    )


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
class MemoryShamirRecovery:
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
    cooked_brief: ShamirRecoveryBriefCertificate
    shamir_recovery_brief_certificate: bytes
    # The shares provided as a `ShamirRecoveryShareCertificate` since
    # each share is aimed at a specific recipient.
    shares: dict[UserID, MemoryShamirShare]
    cooked_deletion: ShamirRecoveryDeletionCertificate | None = None
    shamir_recovery_deletion_certificate: bytes | None = None

    @property
    def deleted_on(self) -> DateTime | None:
        return self.cooked_deletion.timestamp if self.cooked_deletion else None

    @property
    def is_deleted(self) -> bool:
        return self.shamir_recovery_deletion_certificate is not None


@dataclass(slots=True)
class MemoryShamirShare:
    cooked: ShamirRecoveryShareCertificate
    shamir_recovery_share_certificates: bytes


@dataclass(slots=True)
class MemoryAccount:
    # Main identifier for the account.
    account_email: EmailAddress
    # Not used by Parsec Account but works as a quality-of-life feature
    # to allow pre-filling human handle during enrollment.
    human_handle: HumanHandle
    current_vault: MemoryAccountVault
    # Current vault is not part of previous vaults
    previous_vaults: list[MemoryAccountVault] = field(default_factory=list)
    # Note that any active auth methods gets disabled when the account is deleted
    deleted_on: DateTime | None = None


@dataclass(slots=True)
class MemoryAccountVault:
    items: dict[HashDigest, bytes]
    # `authentication_methods` is guaranteed to have at least 1 element
    authentication_methods: dict[AccountAuthMethodID, MemoryAuthenticationMethod]

    def __post_init__(self):
        assert len(self.authentication_methods) > 0  # Sanity check

    @property
    def active_authentication_methods(self) -> Iterable[MemoryAuthenticationMethod]:
        for auth_method in self.authentication_methods.values():
            if auth_method.disabled_on is None:
                yield auth_method


@dataclass(slots=True)
class MemoryAuthenticationMethod:
    id: AccountAuthMethodID
    created_on: DateTime
    # IP address of the HTTP request that created the authentication method
    # (either by account creation, vault key rotation or account recovery)
    # Can be unknown (i.e. empty string) since this information is optional in
    # ASGI (see
    # https://asgi.readthedocs.io/en/latest/specs/www.html#http-connection-scope).
    created_by_ip: str | Literal[""]
    # User agent header of the HTTP request that created the vault.
    created_by_user_agent: str
    # Secret key used for HMAC based authentication with the server
    mac_key: SecretKey
    # Vault key encrypted with the `auth_method_secret_key` see rfc 1014
    vault_key_access: bytes
    # Auth method can be of two types:
    # - ClientProvided, for which the client is able to store
    #   `auth_method_master_secret` all by itself.
    # - Password, for which the client must obtain some configuration
    #   (i.e. this field !) from the server in order to know how
    #   to turn the password into `auth_method_master_secret`.
    password_algorithm: UntrustedPasswordAlgorithm | None
    # Note that only the current vault contains auth methods that are not disabled
    disabled_on: DateTime | None = None

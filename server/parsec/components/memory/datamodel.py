# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from copy import deepcopy
from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Iterable

from parsec._parsec import (
    ActiveUsersLimit,
    BlockID,
    BootstrapToken,
    DateTime,
    DeviceCertificate,
    DeviceID,
    EnrollmentID,
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
from parsec.components.invite import ConduitState
from parsec.components.sequester import SequesterServiceType


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

    @property
    def last_sequester_certificate_timestamp(self) -> DateTime:
        """
        Raises ValueError if the organization is not sequestered !
        """
        if self.cooked_sequester_authority is None:
            raise ValueError("Not a sequestered organization !")
        assert self.sequester_services is not None
        last_timestamp = self.cooked_sequester_authority.timestamp
        for service in self.sequester_services.values():
            last_timestamp = max(last_timestamp, service.cooked.timestamp)
            if service.cooked_revoked is not None:
                last_timestamp = max(last_timestamp, service.cooked_revoked.timestamp)
        return last_timestamp

    @property
    def last_common_certificate_timestamp(self) -> DateTime:
        """
        Raises ValueError if the organization is not bootstrapped !
        """
        return max(
            (
                # User certificates
                *(u.cooked.timestamp for u in self.users.values()),
                # Revoked user certificates
                *(
                    u.cooked_revoked.timestamp
                    for u in self.users.values()
                    if u.cooked_revoked is not None
                ),
                # User update certificates
                *(p.cooked.timestamp for u in self.users.values() for p in u.profile_updates),
                # Device certificates
                *(d.cooked.timestamp for d in self.devices.values()),
            )
        )

    @property
    def last_certificate_timestamp(self) -> DateTime:
        """
        Raises ValueError if the organization is not bootstrapped !
        """
        if self.is_sequestered:
            return max(
                self.last_common_certificate_timestamp,
                self.last_sequester_certificate_timestamp,
                *(r.last_realm_certificate_timestamp for r in self.realms.values()),
            )
        else:
            # Must pass an iterator to max in case there is no realms (see `max` signature)
            return max(
                (
                    self.last_common_certificate_timestamp,
                    *(r.last_realm_certificate_timestamp for r in self.realms.values()),
                )
            )

    @property
    def last_certificate_or_vlob_timestamp(self) -> DateTime:
        """
        Raises ValueError if the organization is not bootstrapped !
        """
        # Must pass an iterator to max in case there is no realms (see `max` signature)
        return max(
            (
                self.last_certificate_timestamp,
                *(ts for r in self.realms.values() if (ts := r.last_vlob_timestamp) is not None),
            )
        )

    @property
    def last_shamir_certificate_timestamp(self) -> DateTime | None:
        if len(self.shamir_setup) == 0:
            return None
        else:
            return max(setup.brief.timestamp for setup in self.shamir_setup.values())

    def clone_as(self, new_organization_id: OrganizationID) -> MemoryOrganization:
        cloned = deepcopy(self)
        cloned.organization_id = new_organization_id
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
    conduit_state: ConduitState = ConduitState.STATE_1_WAIT_PEERS
    conduit_is_last_exchange: bool = False
    conduit_greeter_payload: bytes | None = field(default=None, repr=False)
    conduit_claimer_payload: bytes | None = field(default=None, repr=False)

    @property
    def is_deleted(self) -> bool:
        return self.deleted_on is not None


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

    @property
    def last_realm_certificate_timestamp(self) -> DateTime:
        # TODO: don't forget to update this once realm archiving is implemented !
        last_timestamp = self.roles[-1].cooked.timestamp
        if self.renames:
            last_timestamp = max(last_timestamp, self.renames[-1].cooked.timestamp)
        if self.key_rotations:
            last_timestamp = max(last_timestamp, self.key_rotations[-1].cooked.timestamp)
        if self.archivings:
            last_timestamp = max(last_timestamp, self.archivings[-1].cooked.timestamp)
        return last_timestamp

    @property
    def last_vlob_timestamp(self) -> DateTime | None:
        if not self.vlob_updates:
            return None
        else:
            return self.vlob_updates[-1].vlob_atom.created_on


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

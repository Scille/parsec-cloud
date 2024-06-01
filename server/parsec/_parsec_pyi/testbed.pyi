# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec import (
    BlockID,
    DateTime,
    DeviceCertificate,
    DeviceID,
    DeviceLabel,
    HumanHandle,
    InvitationToken,
    PrivateKey,
    RealmArchivingCertificate,
    RealmKeyRotationCertificate,
    RealmNameCertificate,
    RealmRole,
    RealmRoleCertificate,
    RevokedUserCertificate,
    SecretKey,
    SequesterAuthorityCertificate,
    SequesterPrivateKeyDer,
    SequesterPublicKeyDer,
    SequesterServiceCertificate,
    SequesterServiceID,
    SequesterSigningKeyDer,
    SequesterVerifyKeyDer,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryShareCertificate,
    SigningKey,
    UserCertificate,
    UserID,
    UserProfile,
    UserUpdateCertificate,
    VlobID,
)

def test_get_testbed_template(id: str) -> TestbedTemplateContent | None: ...
def test_load_testbed_customization(
    testbed: TestbedTemplateContent, customization: bytes
) -> list[TestbedEvent]: ...

class TestbedTemplateContent:
    id: str
    events: list["TestbedEvent"]

    def compute_crc(self) -> int: ...

class TestbedEventBootstrapOrganization:
    timestamp: DateTime
    root_signing_key: SigningKey
    sequester_authority_signing_key: SequesterSigningKeyDer | None
    sequester_authority_verify_key: SequesterVerifyKeyDer | None
    first_user_id: UserID
    first_user_human_handle: HumanHandle
    first_user_private_key: PrivateKey
    first_user_first_device_id: DeviceID
    first_user_first_device_label: DeviceLabel
    first_user_first_device_signing_key: SigningKey
    first_user_user_realm_id: VlobID
    first_user_user_realm_key: SecretKey
    first_user_local_symkey: SecretKey
    first_user_local_password: str

    sequester_authority_certificate: SequesterAuthorityCertificate | None
    sequester_authority_raw_certificate: bytes | None

    first_user_certificate: UserCertificate
    first_user_raw_certificate: bytes
    first_user_raw_redacted_certificate: bytes

    first_user_first_device_certificate: DeviceCertificate
    first_user_first_device_raw_certificate: bytes
    first_user_first_device_raw_redacted_certificate: bytes

class TestbedEventNewSequesterService:
    timestamp: DateTime
    id: SequesterServiceID
    label: str
    encryption_private_key: SequesterPrivateKeyDer
    encryption_public_key: SequesterPublicKeyDer

    certificate: SequesterServiceCertificate
    raw_redacted_certificate: bytes
    raw_certificate: bytes

class TestbedEventRevokeSequesterService:
    timestamp: DateTime
    id: SequesterServiceID

    certificate: SequesterServiceCertificate
    raw_redacted_certificate: bytes
    raw_certificate: bytes

class TestbedEventNewUser:
    timestamp: DateTime
    author: DeviceID
    user_id: UserID
    human_handle: HumanHandle
    private_key: PrivateKey
    first_device_id: DeviceID
    first_device_label: DeviceLabel
    first_device_signing_key: SigningKey
    initial_profile: UserProfile
    user_realm_id: VlobID
    user_realm_key: SecretKey
    local_symkey: SecretKey
    local_password: str

    user_certificate: UserCertificate
    user_raw_redacted_certificate: bytes
    user_raw_certificate: bytes

    first_device_certificate: DeviceCertificate
    first_device_raw_redacted_certificate: bytes
    first_device_raw_certificate: bytes

class TestbedEventNewDevice:
    timestamp: DateTime
    author: DeviceID
    user_id: UserID
    device_id: DeviceID
    device_label: DeviceLabel
    signing_key: SigningKey
    local_symkey: SecretKey
    local_password: str

    certificate: DeviceCertificate
    raw_redacted_certificate: bytes
    raw_certificate: bytes

class TestbedEventUpdateUserProfile:
    timestamp: DateTime
    author: DeviceID
    user: UserID
    profile: UserProfile

    certificate: UserUpdateCertificate
    raw_redacted_certificate: bytes
    raw_certificate: bytes

class TestbedEventRevokeUser:
    timestamp: DateTime
    author: DeviceID
    user: UserID

    certificate: RevokedUserCertificate
    raw_redacted_certificate: bytes
    raw_certificate: bytes

class TestbedEventNewUserInvitation:
    claimer_email: str
    created_by: DeviceID
    created_on: DateTime
    token: InvitationToken

class TestbedEventNewDeviceInvitation:
    created_by: DeviceID
    created_on: DateTime
    token: InvitationToken

class TestbedEventNewRealm:
    timestamp: DateTime
    author: DeviceID
    realm_id: VlobID
    realm_key: SecretKey

    certificate: RealmRoleCertificate
    raw_redacted_certificate: bytes
    raw_certificate: bytes

class TestbedEventShareRealm:
    timestamp: DateTime
    author: DeviceID
    realm: VlobID
    user: UserID
    role: RealmRole | None
    key_index: int | None  # None if role is None
    recipient_keys_bundle_access: bytes | None  # None if role is None

    certificate: RealmRoleCertificate
    raw_certificate: bytes

class TestbedEventRenameRealm:
    timestamp: DateTime
    author: DeviceID
    realm: VlobID

    certificate: RealmNameCertificate
    raw_certificate: bytes

class TestbedEventRotateKeyRealm:
    timestamp: DateTime
    author: DeviceID
    realm: VlobID
    key_index: int
    per_participant_keys_bundle_access: dict[UserID, bytes]
    keys_bundle: bytes

    certificate: RealmKeyRotationCertificate
    raw_certificate: bytes

class TestbedEventArchiveRealm:
    timestamp: DateTime
    author: DeviceID
    realm: VlobID

    certificate: RealmArchivingCertificate
    raw_redacted_certificate: bytes
    raw_certificate: bytes

class TestbedEventNewShamirRecovery:
    timestamp: DateTime
    author: DeviceID
    threshold: int
    per_recipient_shares: dict[UserID, int]

    brief_certificate: ShamirRecoveryBriefCertificate
    raw_brief_certificate: bytes
    shares_certificates: list[ShamirRecoveryShareCertificate]
    raw_shares_certificates: list[bytes]

class TestbedEventCreateOrUpdateOpaqueVlob:
    timestamp: DateTime
    author: DeviceID
    realm: VlobID
    vlob_id: VlobID
    key_index: int
    version: int
    encrypted: bytes
    sequestered: dict[SequesterServiceID, bytes] | None

class TestbedEventCreateBlock:
    timestamp: DateTime
    author: DeviceID
    realm: VlobID
    block_id: BlockID
    key_index: int
    cleartext: bytes

    encrypted: bytes

class TestbedEventCreateOpaqueBlock:
    timestamp: DateTime
    author: DeviceID
    realm: VlobID
    block_id: BlockID
    key_index: int
    encrypted: bytes

TestbedEvent = (
    TestbedEventBootstrapOrganization
    | TestbedEventNewSequesterService
    | TestbedEventRevokeSequesterService
    | TestbedEventNewUser
    | TestbedEventNewDevice
    | TestbedEventUpdateUserProfile
    | TestbedEventRevokeUser
    | TestbedEventNewRealm
    | TestbedEventShareRealm
    | TestbedEventRenameRealm
    | TestbedEventRotateKeyRealm
    | TestbedEventArchiveRealm
    | TestbedEventNewShamirRecovery
    | TestbedEventCreateOrUpdateOpaqueVlob
    | TestbedEventCreateOpaqueBlock
)

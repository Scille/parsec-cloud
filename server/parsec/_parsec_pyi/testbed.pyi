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
    SigningKey,
    UserCertificate,
    UserID,
    UserProfile,
    UserUpdateCertificate,
    VlobID,
)

def test_get_testbed_template(id: str) -> TestbedTemplateContent | None: ...

class TestbedTemplateContent:
    id: str
    events: list["TestbedEvent"]
    certificates: list[
        UserCertificate
        | DeviceCertificate
        | RevokedUserCertificate
        | UserUpdateCertificate
        | RealmRoleCertificate
        | SequesterAuthorityCertificate
        | SequesterServiceCertificate
    ]

    def compute_crc(self) -> int: ...

class TestbedEventBootstrapOrganization:
    timestamp: DateTime
    root_signing_key: SigningKey
    sequester_authority_signing_key: SequesterSigningKeyDer | None
    sequester_authority_verify_key: SequesterVerifyKeyDer | None
    first_user_device_id: DeviceID
    first_user_human_handle: HumanHandle
    first_user_private_key: PrivateKey
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

class TestbedEventNewUser:
    timestamp: DateTime
    author: DeviceID
    device_id: DeviceID
    human_handle: HumanHandle
    private_key: PrivateKey
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
    greeter_user_id: UserID
    created_on: DateTime
    token: InvitationToken

class TestbedEventNewDeviceInvitation:
    greeter_user_id: UserID
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
    recipient_message: bytes | None

    certificate: RealmRoleCertificate
    raw_redacted_certificate: bytes
    raw_certificate: bytes

class TestbedEventStartRealmReencryption:
    timestamp: DateTime
    author: DeviceID
    realm: VlobID
    encryption_revision: int
    per_participant_message: list[tuple[UserID, bytes]]

class TestbedEventFinishRealmReencryption:
    timestamp: DateTime
    author: DeviceID
    realm: VlobID
    encryption_revision: int

class TestbedEventCreateOrUpdateOpaqueVlob:
    timestamp: DateTime
    author: DeviceID
    realm: VlobID
    encryption_revision: int
    vlob_id: VlobID
    version: int
    encrypted: bytes
    sequestered: dict[SequesterServiceID, bytes] | None

class TestbedEventCreateOpaqueBlock:
    timestamp: DateTime
    author: DeviceID
    realm: VlobID
    block_id: BlockID
    encrypted: bytes

TestbedEvent = (
    TestbedEventBootstrapOrganization
    | TestbedEventNewSequesterService
    | TestbedEventNewUser
    | TestbedEventNewDevice
    | TestbedEventUpdateUserProfile
    | TestbedEventRevokeUser
    | TestbedEventNewRealm
    | TestbedEventShareRealm
    | TestbedEventStartRealmReencryption
    | TestbedEventFinishRealmReencryption
    | TestbedEventCreateOrUpdateOpaqueVlob
    | TestbedEventCreateOpaqueBlock
)

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::*;

/// Returns:new ... certificate:
/// - user
/// - redacted user
/// - device
/// - redacted device
pub fn create_user_and_device_certificates(
    author: &LocalDevice,
    device_label: DeviceLabel,
    human_handle: HumanHandle,
    profile: UserProfile,
    public_key: PublicKey,
    verify_key: VerifyKey,
    timestamp: DateTime,
) -> (UserID, DeviceID, Bytes, Bytes, Bytes, Bytes) {
    let user_id = UserID::default();
    let device_id = DeviceID::default();

    let user_certificate = UserCertificate {
        author: CertificateSigner::User(author.device_id),
        timestamp,
        user_id,
        human_handle: MaybeRedacted::Real(human_handle),
        public_key: public_key.clone(),
        algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
        profile,
    };

    let redacted_user_certificate = UserCertificate {
        author: CertificateSigner::User(author.device_id),
        timestamp,
        user_id,
        human_handle: MaybeRedacted::Redacted(HumanHandle::new_redacted(user_id)),
        public_key,
        algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
        profile,
    };

    let device_certificate = DeviceCertificate {
        author: CertificateSigner::User(author.device_id),
        timestamp,
        purpose: DevicePurpose::Standard,
        user_id,
        device_id,
        device_label: MaybeRedacted::Real(device_label),
        verify_key: verify_key.clone(),
        algorithm: SigningKeyAlgorithm::Ed25519,
    };

    let redacted_device_certificate = DeviceCertificate {
        author: CertificateSigner::User(author.device_id),
        timestamp,
        purpose: DevicePurpose::Standard,
        user_id,
        device_id,
        device_label: MaybeRedacted::Redacted(DeviceLabel::new_redacted(device_id)),
        verify_key,
        algorithm: SigningKeyAlgorithm::Ed25519,
    };

    let user_certificate_bytes = user_certificate.dump_and_sign(&author.signing_key);
    let redacted_user_certificate_bytes =
        redacted_user_certificate.dump_and_sign(&author.signing_key);
    let device_certificate_bytes = device_certificate.dump_and_sign(&author.signing_key);
    let redacted_device_certificate_bytes =
        redacted_device_certificate.dump_and_sign(&author.signing_key);

    (
        user_id,
        device_id,
        user_certificate_bytes.into(),
        redacted_user_certificate_bytes.into(),
        device_certificate_bytes.into(),
        redacted_device_certificate_bytes.into(),
    )
}

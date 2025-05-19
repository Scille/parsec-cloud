// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// We accept `unwrap` here since this code is only for tests
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use crate::{Device, alice, bob, mallory};

#[fixture]
#[once]
pub fn alice_user_certif(alice: &Device) -> UserCertificate {
    UserCertificate {
        author: CertificateSigner::Root,
        timestamp: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap(),
        user_id: alice.user_id,
        human_handle: MaybeRedacted::Real(alice.human_handle.clone()),
        public_key: alice.public_key(),
        algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
        profile: alice.profile,
    }
}

#[fixture]
#[once]
pub fn bob_user_certif(bob: &Device) -> UserCertificate {
    UserCertificate {
        author: CertificateSigner::Root,
        timestamp: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap(),
        user_id: bob.user_id,
        human_handle: MaybeRedacted::Real(bob.human_handle.clone()),
        public_key: bob.public_key(),
        algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
        profile: bob.profile,
    }
}

#[fixture]
#[once]
pub fn mallory_user_certif(mallory: &Device) -> UserCertificate {
    UserCertificate {
        author: CertificateSigner::Root,
        timestamp: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap(),
        user_id: mallory.user_id,
        human_handle: MaybeRedacted::Real(mallory.human_handle.clone()),
        public_key: mallory.public_key(),
        algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
        profile: mallory.profile,
    }
}

#[fixture]
#[once]
pub fn alice_device_certif(alice: &Device) -> DeviceCertificate {
    DeviceCertificate {
        author: CertificateSigner::Root,
        timestamp: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap(),
        purpose: DevicePurpose::Standard,
        user_id: alice.user_id,
        device_id: alice.device_id,
        device_label: MaybeRedacted::Real(alice.device_label.clone()),
        verify_key: alice.verify_key(),
        algorithm: SigningKeyAlgorithm::Ed25519,
    }
}

#[fixture]
#[once]
pub fn bob_device_certif(bob: &Device) -> DeviceCertificate {
    DeviceCertificate {
        author: CertificateSigner::Root,
        timestamp: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap(),
        purpose: DevicePurpose::Standard,
        user_id: bob.user_id,
        device_id: bob.device_id,
        device_label: MaybeRedacted::Real(bob.device_label.clone()),
        verify_key: bob.verify_key(),
        algorithm: SigningKeyAlgorithm::Ed25519,
    }
}

#[fixture]
#[once]
pub fn mallory_device_certif(mallory: &Device) -> DeviceCertificate {
    DeviceCertificate {
        author: CertificateSigner::Root,
        timestamp: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap(),
        purpose: DevicePurpose::Standard,
        user_id: mallory.user_id,
        device_id: mallory.device_id,
        device_label: MaybeRedacted::Real(mallory.device_label.clone()),
        verify_key: mallory.verify_key(),
        algorithm: SigningKeyAlgorithm::Ed25519,
    }
}

#[fixture]
#[once]
pub fn alice_revoked_user_certif(alice: &Device, bob: &Device) -> RevokedUserCertificate {
    RevokedUserCertificate {
        author: bob.device_id,
        timestamp: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap(),
        user_id: alice.user_id,
    }
}

#[fixture]
#[once]
pub fn bob_revoked_user_certif(alice: &Device, bob: &Device) -> RevokedUserCertificate {
    RevokedUserCertificate {
        author: alice.device_id,
        timestamp: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap(),
        user_id: bob.user_id,
    }
}

#[fixture]
#[once]
pub fn mallory_revoked_user_certif(alice: &Device, mallory: &Device) -> RevokedUserCertificate {
    RevokedUserCertificate {
        author: alice.device_id,
        timestamp: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap(),
        user_id: mallory.user_id,
    }
}

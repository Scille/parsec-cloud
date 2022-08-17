// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use chrono::{TimeZone, Utc};
use rstest::fixture;

use libparsec_types::{
    CertificateSignerOwned, DateTime, DeviceCertificate, RevokedUserCertificate, UserCertificate,
};

use crate::{alice, bob, mallory, Device};

#[fixture]
#[once]
pub fn alice_user_certif(alice: &Device) -> UserCertificate {
    UserCertificate {
        author: CertificateSignerOwned::Root,
        timestamp: DateTime::from(Utc.ymd(2000, 1, 1).and_hms(0, 0, 0)),
        user_id: alice.user_id().clone(),
        human_handle: alice.human_handle.clone(),
        public_key: alice.public_key(),
        profile: alice.profile,
    }
}

#[fixture]
#[once]
pub fn bob_user_certif(bob: &Device) -> UserCertificate {
    UserCertificate {
        author: CertificateSignerOwned::Root,
        timestamp: DateTime::from(Utc.ymd(2000, 1, 1).and_hms(0, 0, 0)),
        user_id: bob.user_id().clone(),
        human_handle: bob.human_handle.clone(),
        public_key: bob.public_key(),
        profile: bob.profile,
    }
}

#[fixture]
#[once]
pub fn mallory_user_certif(mallory: &Device) -> UserCertificate {
    UserCertificate {
        author: CertificateSignerOwned::Root,
        timestamp: DateTime::from(Utc.ymd(2000, 1, 1).and_hms(0, 0, 0)),
        user_id: mallory.user_id().clone(),
        human_handle: mallory.human_handle.clone(),
        public_key: mallory.public_key(),
        profile: mallory.profile,
    }
}

#[fixture]
#[once]
pub fn alice_device_certif(alice: &Device) -> DeviceCertificate {
    DeviceCertificate {
        author: CertificateSignerOwned::Root,
        timestamp: DateTime::from(Utc.ymd(2000, 1, 1).and_hms(0, 0, 0)),
        device_id: alice.device_id.clone(),
        device_label: alice.device_label.clone(),
        verify_key: alice.verify_key(),
    }
}

#[fixture]
#[once]
pub fn bob_device_certif(bob: &Device) -> DeviceCertificate {
    DeviceCertificate {
        author: CertificateSignerOwned::Root,
        timestamp: DateTime::from(Utc.ymd(2000, 1, 1).and_hms(0, 0, 0)),
        device_id: bob.device_id.clone(),
        device_label: bob.device_label.clone(),
        verify_key: bob.verify_key(),
    }
}

#[fixture]
#[once]
pub fn mallory_device_certif(mallory: &Device) -> DeviceCertificate {
    DeviceCertificate {
        author: CertificateSignerOwned::Root,
        timestamp: DateTime::from(Utc.ymd(2000, 1, 1).and_hms(0, 0, 0)),
        device_id: mallory.device_id.clone(),
        device_label: mallory.device_label.clone(),
        verify_key: mallory.verify_key(),
    }
}

#[fixture]
#[once]
pub fn alice_revoked_user_certif(alice: &Device, bob: &Device) -> RevokedUserCertificate {
    RevokedUserCertificate {
        author: bob.device_id.clone(),
        timestamp: DateTime::from(Utc.ymd(2000, 1, 1).and_hms(0, 0, 0)),
        user_id: alice.user_id().clone(),
    }
}

#[fixture]
#[once]
pub fn bob_revoked_user_certif(alice: &Device, bob: &Device) -> RevokedUserCertificate {
    RevokedUserCertificate {
        author: alice.device_id.clone(),
        timestamp: DateTime::from(Utc.ymd(2000, 1, 1).and_hms(0, 0, 0)),
        user_id: bob.user_id().clone(),
    }
}

#[fixture]
#[once]
pub fn mallory_revoked_user_certif(alice: &Device, mallory: &Device) -> RevokedUserCertificate {
    RevokedUserCertificate {
        author: alice.device_id.clone(),
        timestamp: DateTime::from(Utc.ymd(2000, 1, 1).and_hms(0, 0, 0)),
        user_id: mallory.user_id().clone(),
    }
}

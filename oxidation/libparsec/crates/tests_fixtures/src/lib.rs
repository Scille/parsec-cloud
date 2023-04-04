// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod testbed;
mod trustchain;

pub use testbed::*;
pub use trustchain::*;

// Reexport
pub use env_logger;
pub use libparsec_tests_macros::parsec_test;
pub use libparsec_tests_types::{alice, bob, coolorg, rstest, timestamp, Device, Organization};

use rstest::fixture;
use std::path::PathBuf;
use uuid::Uuid;

use libparsec_crypto::prelude::*;
use libparsec_types::*;

#[fixture]
#[once]
pub fn mallory(coolorg: &Organization) -> Device {
    Device {
        organization_addr: coolorg.addr_with_prefixed_host("mallory_dev1"),
        device_id: "mallory@dev1".parse().unwrap(),
        device_label: None,
        human_handle: Some(
            HumanHandle::new("mallory@example.com", "Mallory McMalloryFace").unwrap(),
        ),
        signing_key: SigningKey::generate(),
        private_key: PrivateKey::generate(),
        profile: UserProfile::Standard,
        user_manifest_id: EntryID::default(),
        user_manifest_key: SecretKey::generate(),
        local_symkey: SecretKey::generate(),
        time_provider: TimeProvider::default(),
    }
}

#[fixture]
#[once]
pub fn user_certificate(alice: &Device, bob: &Device, timestamp: DateTime) -> Vec<u8> {
    UserCertificate {
        author: CertificateSignerOwned::User(alice.device_id.clone()),
        timestamp,
        user_id: bob.user_id().clone(),
        human_handle: bob.human_handle.clone(),
        public_key: bob.public_key(),
        profile: UserProfile::Standard,
    }
    .dump_and_sign(&alice.signing_key)
}

#[fixture]
#[once]
pub fn redacted_user_certificate(alice: &Device, bob: &Device, timestamp: DateTime) -> Vec<u8> {
    UserCertificate {
        author: CertificateSignerOwned::User(alice.device_id.clone()),
        timestamp,
        user_id: bob.user_id().clone(),
        human_handle: None,
        public_key: bob.public_key(),
        profile: UserProfile::Standard,
    }
    .dump_and_sign(&alice.signing_key)
}

#[fixture]
#[once]
pub fn device_certificate(alice: &Device, bob: &Device, timestamp: DateTime) -> Vec<u8> {
    DeviceCertificate {
        author: CertificateSignerOwned::User(alice.device_id.clone()),
        timestamp,
        device_id: bob.device_id.clone(),
        device_label: bob.device_label.clone(),
        verify_key: bob.verify_key(),
    }
    .dump_and_sign(&alice.signing_key)
}

#[fixture]
#[once]
pub fn redacted_device_certificate(alice: &Device, bob: &Device, timestamp: DateTime) -> Vec<u8> {
    DeviceCertificate {
        author: CertificateSignerOwned::User(alice.device_id.clone()),
        timestamp,
        device_id: bob.device_id.clone(),
        device_label: None,
        verify_key: bob.verify_key(),
    }
    .dump_and_sign(&alice.signing_key)
}

/// A temporary path that will be removed on drop.
pub struct TmpPath(PathBuf);

impl std::ops::Deref for TmpPath {
    type Target = PathBuf;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for TmpPath {
    fn drop(&mut self) {
        if let Err(err) = std::fs::remove_dir_all(&self.0) {
            // Cannot remove the directory :'(
            // If we are on Windows, it most likely means a file in the directory
            // is still opened. Typically a SQLite database is still opened because
            // the SQLiteExecutor's drop doesn't wait
            let content = {
                match std::fs::read_dir(&self.0) {
                    Ok(items) => items
                        .into_iter()
                        .map(|item| match item {
                            Ok(item) => {
                                format!("{}", item.path().strip_prefix(&self.0).unwrap().display())
                            }
                            Err(err) => format!("<error: {:?}>", err),
                        })
                        .collect(),
                    Err(_) => vec!["<empty>".to_owned()],
                }
                .join(" ")
            };
            panic!(
                "Cannot remove {:?}: {}\n\
                Content: {}\n\
                Have you done a gracious close of resources in your test ?",
                &self.0, &err, content
            );
        }
    }
}

#[fixture]
pub fn tmp_path() -> TmpPath {
    let mut path = std::env::temp_dir();

    path.extend(["parsec-tests", &Uuid::new_v4().to_string()]);

    std::fs::create_dir_all(&path).expect("Cannot create tmp_path dir");

    TmpPath(path)
}

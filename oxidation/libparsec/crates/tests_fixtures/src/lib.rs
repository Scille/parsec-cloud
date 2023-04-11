// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
use std::path::PathBuf;

use rstest::fixture;
use uuid::Uuid;

pub use libparsec_tests_macros::parsec_test;

mod trustchain;

pub use trustchain::*;

// Reexport
pub use env_logger;
pub use rstest;

pub use libparsec_types::{
    fixtures::{
        alice, bob, coolorg, device_certificate, mallory, redacted_device_certificate,
        redacted_user_certificate, timestamp, user_certificate, Device, Organization,
    },
    DateTime,
};

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

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;
use uuid::Uuid;

/// A temporary path that will be removed on drop.
///
/// ## For web environment
///
/// In web environment, we cannot use the `std::fs` api (since we do not have access to the
/// filesystem). So we will neither create nor remove the tmp directory.
///
/// In this case the temp path could be used as a discriminant since it's value is random.
pub struct TmpPath(PathBuf);

impl TmpPath {
    pub fn create() -> Self {
        let mut path = if cfg!(target_arch = "wasm32") {
            PathBuf::default()
        } else {
            std::env::temp_dir()
        };

        path.extend(["parsec-tests", &Uuid::new_v4().to_string()]);

        #[cfg(not(target_arch = "wasm32"))]
        std::fs::create_dir_all(&*path).expect("Cannot create tmp_path dir");

        TmpPath(path)
    }
}

impl std::ops::Deref for TmpPath {
    type Target = PathBuf;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::TmpPath;

    impl Drop for TmpPath {
        fn drop(&mut self) {
            if let Err(err) = std::fs::remove_dir_all(&self.0) {
                if matches!(err.kind(), std::io::ErrorKind::NotFound) {
                    // TmpPath was already removed
                    return;
                }
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
                                format!("{}", item.path().strip_prefix(&self.0).expect("The item paths are the children of the inner path, they always have it as a prefix").display())
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
}

#[libparsec_tests_lite::rstest::fixture]
pub fn tmp_path() -> TmpPath {
    TmpPath::create()
}

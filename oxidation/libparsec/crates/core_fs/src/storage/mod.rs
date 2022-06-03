// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

mod chunk_storage;
mod local_database;
mod manifest_storage;
// Not used for the moment
#[allow(dead_code)]
mod user_storage;
mod version;
mod workspace_storage;

pub use manifest_storage::ChunkOrBlockID;
pub use workspace_storage::*;

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::path::PathBuf;
    use tests_fixtures::{manifest_dir, tmp, TempPath};

    use crate::storage::local_database::SqlitePool;

    #[rstest]
    fn test_schema(manifest_dir: &PathBuf, tmp: TempPath) {
        let schema = std::fs::read_to_string(manifest_dir.join("src/schema.rs")).unwrap();
        let db_path = tmp.generate("schema.sqlite");

        // Create the file
        SqlitePool::new(&db_path).unwrap();

        std::process::Command::new("diesel")
            .arg("database")
            .arg("reset")
            .arg("--database-url")
            .arg(&db_path)
            .current_dir(manifest_dir)
            .output()
            .unwrap();

        let current_schema = String::from_utf8(
            std::process::Command::new("diesel")
                .arg("print-schema")
                .arg("--database-url")
                .arg(&db_path)
                .current_dir(manifest_dir)
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();

        assert!(schema.contains(&current_schema))
    }
}

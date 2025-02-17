use std::{fs::File, io::Write, path::Path, sync::Arc};

use libparsec::{tmp_path, TmpPath};
use libparsec_client::WorkspaceHistoryRealmExportDecryptor;
use libparsec_tests_fixtures::{TestbedEnv, TestbedEvent};

use crate::{integration_tests::spawn_interactive_command, std_cmd};

#[rstest::rstest]
#[tokio::test]
async fn mount_realm_export(tmp_path: TmpPath) {
    let exe_path = std::env::current_exe().unwrap();
    let mut path: &Path = exe_path.as_ref();
    let original_export_db_path = loop {
        if path.ends_with("target") {
            break path
                .join("../server/tests/realm_export/sequestered_export.sqlite")
                .canonicalize()
                .unwrap();
        }
        match path.parent() {
            Some(parent) => path = parent,
            None => {
                panic!("Cannot find the realm export dump");
            }
        }
    };

    // Copy the realm export database since SQLite modifies the file in place
    // even when only doing read operations.

    let export_db_path = tmp_path.join(original_export_db_path.file_name().unwrap());
    std::fs::copy(original_export_db_path, &export_db_path).unwrap();

    libparsec_tests_fixtures::TestbedScope::run("sequestered", |env: Arc<TestbedEnv>| async move {
        let env = env.as_ref();

        // We use `sequestered` testbed template, which contains two sequesters services
        // `sequester_service_1` & `sequester_service_2` with a key rotation on `wksp1`
        // done after `sequester_service_1` got revoked.
        let decryptors: Vec<_> = env
            .template
            .events
            .iter()
            .filter_map(|e| match e {
                TestbedEvent::NewSequesterService(x) => {
                    Some(WorkspaceHistoryRealmExportDecryptor::SequesterService {
                        sequester_service_id: x.id,
                        private_key: Box::new(x.encryption_private_key.clone()),
                    })
                }
                _ => None,
            })
            .collect();
        let (key, seq_id) = match &decryptors[0] {
            WorkspaceHistoryRealmExportDecryptor::SequesterService {
                private_key,
                sequester_service_id,
            } => (private_key, sequester_service_id),
            WorkspaceHistoryRealmExportDecryptor::User { .. } => unreachable!(), // see filter map above
        };

        let key_file_path = tmp_path.join("priv_key.key");
        let mut file = File::create(key_file_path.clone()).unwrap();
        file.write_all(&key.dump()).unwrap();

        let cmd = std_cmd!(
            "mount-realm-export",
            export_db_path.to_str().unwrap(),
            key_file_path.to_str().unwrap(),
            "mounted_workspace",
            &format!("{seq_id}")
        );

        let mut p = spawn_interactive_command(cmd, Some(1500)).unwrap();

        p.exp_string("Mounted workspace at mounted_workspace")
            .unwrap();
    })
    .await;
}

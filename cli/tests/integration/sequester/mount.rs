use std::path::{Path, PathBuf};

use libparsec::{tmp_path, SequesterServiceID, TmpPath};
use libparsec_tests_fixtures::{p_assert_eq, p_assert_matches};

async fn populate_realm_export_db(temp_dir: &Path) -> PathBuf {
    // Retrieve the path of the realm export dump

    let exe_path = std::env::current_exe().unwrap();
    let mut path: &Path = exe_path.as_ref();
    let original_export_db_path = loop {
        if path.ends_with("target") {
            break path.join("../server/tests/realm_export/sequestered_export.sqlite");
        }
        match path.parent() {
            Some(parent) => path = parent,
            None => panic!("Cannot find the realm export dump"),
        }
    };

    // Copy the realm export database since SQLite modifies the file in place
    // even when only doing read operations.
    let export_db_path = temp_dir.join(original_export_db_path.file_name().unwrap());
    tokio::fs::copy(original_export_db_path, &export_db_path)
        .await
        .unwrap();

    export_db_path
}

#[rstest::rstest]
#[tokio::test]
async fn sequester_decryptor(tmp_path: TmpPath) {
    let export_db_path = populate_realm_export_db(&tmp_path).await;

    libparsec_tests_fixtures::TestbedScope::run(
        "sequestered",
        |env: std::sync::Arc<libparsec_tests_fixtures::TestbedEnv>| async move {
            let sequester_id: SequesterServiceID =
                *env.template.get_stuff("sequester_service_1_id");
            let sequester_private_key = env
                .template
                .events
                .iter()
                .rev()
                .find_map(|e| match e {
                    libparsec_tests_fixtures::TestbedEvent::NewSequesterService(e)
                        if e.id == sequester_id =>
                    {
                        Some(e.encryption_private_key.clone())
                    }
                    _ => None,
                })
                .unwrap();
            let sequester_private_key_path = tmp_path.join("sequester_private_key.pem");
            tokio::fs::write(
                &sequester_private_key_path,
                &sequester_private_key.dump_pem(),
            )
            .await
            .unwrap();

            let base_mountpoint_path = tmp_path.join("mountpoint");
            tokio::fs::create_dir_all(&base_mountpoint_path)
                .await
                .unwrap();

            let mut cmd = crate::std_cmd!(
                "sequester",
                "mount",
                "--decryptor",
                &format!(
                    "sequester:{}:{}",
                    sequester_id,
                    sequester_private_key_path.display()
                ),
                "--timestamp",
                "2000-01-16T00:00:00Z",
                &export_db_path
            );
            cmd.current_dir(&base_mountpoint_path);
            let mut p = crate::spawn_interactive_command(cmd, Some(1500)).unwrap();

            let (_unread, matched) = p
                .exp_regex("Mounted at .*")
                .expect("Command to not inform mounting the workspace");

            let path_str = regex::regex!("^Mounted at (.*), send SIGINT")
                .captures_iter(&matched)
                .next()
                .expect("Regex did not match")
                .get(1) // index 0 represent the full match
                .expect("Regex did not captured something")
                .as_str();
            let mountpoint_path = AsRef::<Path>::as_ref(path_str);

            p_assert_eq!(
                mountpoint_path.parent(),
                Some(base_mountpoint_path.as_path())
            );

            let bar_txt_path = mountpoint_path.join("bar2.txt");

            std::assert_matches!(tokio::fs::try_exists(&bar_txt_path).await, Ok(true));

            p_assert_eq!(tokio::fs::read(&bar_txt_path).await.unwrap(), b"Hello v2");

            let status = p
                .process_mut()
                .kill(rexpect::process::Signal::SIGINT)
                .unwrap(); // Indicate the command to stop
            p_assert_matches!(status, rexpect::process::WaitStatus::Exited(_, 0));
        },
    )
    .await;
    println!("finishing...");
}

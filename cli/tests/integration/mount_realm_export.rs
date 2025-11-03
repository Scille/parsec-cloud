use std::path::{Path, PathBuf};

use libparsec::{tmp_path, SequesterServiceID, TmpPath};
use libparsec_tests_fixtures::p_assert_eq;

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

            // Ensure parsec-cli is built (since `assert_cmd::cargo::cargo_bin` just return
            // a path to the binary without doing such check)
            tokio::task::spawn_blocking(|| {
                crate::assert_cmd!("mount-realm-export", "--help")
                    .ok()
                    .unwrap();
            })
            .await
            .unwrap();

            let base_mountpoint_path = tmp_path.join("mountpoint");
            tokio::fs::create_dir_all(&base_mountpoint_path)
                .await
                .unwrap();

            let cli_process = crate::std_cmd!(
                "mount-realm-export",
                "--decryptor",
                &format!(
                    "sequester:{}:{}",
                    sequester_id,
                    sequester_private_key_path.to_string_lossy()
                ),
                "--timestamp",
                "2000-01-16T00:00:00Z",
                &export_db_path.to_string_lossy()
            )
            .current_dir(&base_mountpoint_path)
            .spawn()
            .unwrap();

            struct KillOnDrop(std::process::Child);
            impl Drop for KillOnDrop {
                fn drop(&mut self) {
                    // Cannot use `Child::kill` because it sends a SIGKILL, which would leave
                    // the mountpoint in a dirty state. Instead we want to use a SIGTERM.
                    // Must send a SIGTERM
                    std::process::Command::new("kill")
                        .args(["-s", "TERM", &self.0.id().to_string()])
                        .output()
                        .unwrap();
                    // No we can wait for the child to finish
                    self.0.wait().unwrap();
                }
            }
            let _kill_guard = KillOnDrop(cli_process);

            macro_rules! retry {
                ($body:block) => {{
                    let mut attempt = 0;
                    loop {
                        let outcome: Option<_> = $body;
                        match outcome {
                            Some(ok) => break ok,
                            None => {
                                attempt += 1;
                                assert!(attempt < 100, "Mountpoint not created after 100 attempts");
                                tokio::time::sleep(std::time::Duration::from_millis(30)).await;
                                continue;
                            }
                        }
                    }
                }};
            }

            let mountpoint_path = retry!({
                let children = {
                    let mut reader = tokio::fs::read_dir(&base_mountpoint_path).await.unwrap();
                    let mut children = vec![];
                    while let Some(entry) = reader.next_entry().await.unwrap() {
                        children.push(entry);
                    }
                    children
                };
                match children.len() {
                    0 => None,
                    1 => Some(children[0].path()),
                    _ => panic!("Too many entries in the base mountpoint directory: {children:?}"),
                }
            });

            let bar_txt_path = mountpoint_path.join("bar2.txt");

            // Even if the mountpoint directory exist, it is possible the mountpoint
            // is not quiet ready yet, so poll until we see its content just to be sure.
            retry!({
                tokio::fs::try_exists(&bar_txt_path)
                    .await
                    .unwrap()
                    .then_some(())
            });

            p_assert_eq!(tokio::fs::read(&bar_txt_path).await.unwrap(), b"Hello v2");
        },
    )
    .await;
    println!("finishing...");
}

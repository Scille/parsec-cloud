use std::{env::current_dir, io::Read};

use libparsec::{tmp_path, SequesterKeySize, SequesterSigningKeyDer, TmpPath};

use crate::{
    integration_tests::{bootstrap_cli_test_with_sequester, spawn_interactive_command},
    std_cmd,
    testenv_utils::TestOrganization,
    utils::start_client,
};

#[rstest::rstest]
#[tokio::test]
async fn mount_realm_export(tmp_path: TmpPath) {
    let (seq_sign_key, seq_verification_key) =
        SequesterSigningKeyDer::generate_pair(SequesterKeySize::_1024Bits);

    let (_, TestOrganization { alice, .. }, org_id) =
        bootstrap_cli_test_with_sequester(&tmp_path, seq_verification_key)
            .await
            .unwrap();
    let client = start_client(alice.clone()).await.unwrap();

    // Create the workspace used to copy the file to
    let wid = client
        .create_workspace("new-workspace".parse().unwrap())
        .await
        .unwrap();
    client.ensure_workspaces_bootstrapped().await.unwrap();

    // TODO update when realm export available from here

    let mut parsec_dir = current_dir().unwrap();
    assert!(parsec_dir.pop()); // got to parsec-cloud
    let parsec_dir = dbg!(parsec_dir.join("server"));
    let output = std::process::Command::new("poetry")
        .current_dir(parsec_dir)
        .arg("run")
        .arg("parsec")
        .arg("export_realm")
        .arg(format!("--organization={org_id}"))
        .arg(format!("--realm={wid}"))
        .arg("--db=MOCKED")
        .arg("--blockstore=MOCKED")
        .spawn()
        .unwrap();

    let mut buf = String::new();
    dbg!(output)
        .stdout
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();
    let db_export = buf.lines().next().unwrap();
    assert_eq!("Creating ", &db_export[0..9]);
    let db_export_path = dbg!(&db_export[9..]);

    let cmd = std_cmd!(
        "mount-realm-export",
        db_export_path,
        &format!("{seq_sign_key:?}"),
        "mounted_workspace"
    );

    let mut p = spawn_interactive_command(cmd, Some(1500)).unwrap();

    p.exp_string("Mounted workspace at mounted_workspace")
        .unwrap();
}

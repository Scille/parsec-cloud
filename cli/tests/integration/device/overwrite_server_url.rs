use libparsec::{tmp_path, ParsecAddr, TmpPath};
use libparsec_tests_fixtures::prelude::*;

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};

#[rstest::rstest]
#[tokio::test]
async fn ok(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let cmd = crate::std_cmd!(
        "device",
        "overwrite-server-url",
        "--device",
        &alice.device_id.hex(),
        "--server-url",
        "https://new.invalid:123",
        "--password-stdin"
    );
    let mut p = crate::spawn_interactive_command(cmd, Some(1500)).unwrap();

    p.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();

    let old_server_addr: ParsecAddr = alice.organization_addr.clone().into();
    p.exp_string(&format!("Current server URL: {old_server_addr}"))
        .unwrap();

    p.exp_regex(".*New server URL: parsec3://new.invalid:123.*")
        .unwrap();
    p.exp_regex(".*Are you sure?.*").unwrap();

    p.send_line("y").unwrap();

    p.exp_regex("Device updated successfully").unwrap();
    p.exp_eof().unwrap();

    // TODO: Inspect the device content once `device info` CLI command is available
}

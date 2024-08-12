use std::sync::Arc;

use libparsec::{tmp_path, LocalDevice, TmpPath};
use predicates::prelude::PredicateBooleanExt;

use super::bootstrap_cli_test;
use crate::{
    testenv_utils::TestOrganization,
    utils::{RESET, YELLOW},
};

#[rstest::rstest]
#[tokio::test]
async fn device_not_found(tmp_path: TmpPath) {
    const A_DEVICE_THAT_DOES_NOT_EXIST: &str = "0000";
    let (
        _,
        TestOrganization {
            alice,
            bob,
            other_alice,
            toto,
        },
        _,
    ) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let mut devices = [alice, other_alice, bob, toto];
    devices.sort_by_cached_key(|d| d.device_id.hex());

    // Ensure that A_DEVICE_THAT_DOES_NOT_EXIST is not the prefix of one of the known devices.
    devices.iter().for_each(|dev| {
        assert!(!dev
            .device_id
            .hex()
            .starts_with(A_DEVICE_THAT_DOES_NOT_EXIST));
    });

    let mut available_devices_string_list = String::new();
    format_device::<4>(&devices, &mut available_devices_string_list).unwrap();

    crate::assert_cmd_failure!(
        with_password = "a password",
        "list-users",
        "--device",
        A_DEVICE_THAT_DOES_NOT_EXIST
    )
    .stderr(
        predicates::str::contains("Error: Device `0000` not found, available devices:")
            .and(predicates::str::contains(&available_devices_string_list)),
    );
}

fn format_device<const SHORT_ID_LEN: usize>(
    devices: &[Arc<LocalDevice>],
    mut writer: impl std::fmt::Write,
) -> std::fmt::Result {
    for device in devices {
        let short_id = &device.device_id.hex()[..SHORT_ID_LEN];
        let organization_id = &device.organization_id();
        let human_handle = &device.human_handle;
        let device_label = &device.device_label;
        writeln!(
            writer,
            "{YELLOW}{short_id}{RESET} - {organization_id}: {human_handle} @ {device_label}"
        )?;
    }
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn missing_option(tmp_path: TmpPath) {
    let (
        _,
        TestOrganization {
            alice,
            bob,
            other_alice,
            toto,
        },
        _,
    ) = bootstrap_cli_test(&tmp_path).await.unwrap();

    let mut devices = [alice, other_alice, bob, toto];
    devices.sort_by_cached_key(|d| d.device_id.hex());

    let mut available_devices_string_list = String::new();
    format_device::<4>(&devices, &mut available_devices_string_list).unwrap();

    crate::assert_cmd_failure!("list-users").stderr(
        predicates::str::contains("Error: Missing option '--device'")
            .and(predicates::str::contains("Available devices:"))
            .and(predicates::str::contains(&available_devices_string_list)),
    );
}

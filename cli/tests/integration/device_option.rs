use std::sync::Arc;

use libparsec::{tmp_path, HumanHandle, LocalDevice, ParsecAddr, TmpPath};
use predicates::prelude::PredicateBooleanExt;

use super::{
    bootstrap_cli_test,
    testenv_utils::{create_new_device, create_new_user, TestOrganization},
};
use parsec_cli::utils::{get_minimal_short_id_size, RESET, YELLOW};

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
    format_devices(&devices, &mut available_devices_string_list).unwrap();

    crate::assert_cmd_failure!(
        with_password = "a password",
        "user",
        "list",
        "--device",
        A_DEVICE_THAT_DOES_NOT_EXIST
    )
    .stderr(
        predicates::str::contains("Error: Device `0000` not found, available devices:")
            .and(predicates::str::contains(&available_devices_string_list)),
    );
}

fn format_devices(
    devices: &[Arc<LocalDevice>],
    mut writer: impl std::fmt::Write,
) -> std::fmt::Result {
    let short_id_size = get_minimal_short_id_size(devices.iter().map(|d| &d.device_id));

    for device in devices {
        let short_id = &device.device_id.hex()[..short_id_size];
        let organization_id = &device.organization_id();
        let human_handle = &device.human_handle;
        let device_label = &device.device_label;
        let server_url = ParsecAddr::from(&device.organization_addr).to_http_url(None);

        writeln!(
            writer,
            "{YELLOW}{short_id}{RESET} - {organization_id}: {human_handle} @ {device_label} ({server_url})"
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
    format_devices(&devices, &mut available_devices_string_list).unwrap();

    crate::assert_cmd_failure!("user", "list").stderr(
        predicates::str::contains("Error: Missing option '--device'")
            .and(predicates::str::contains("Available devices:"))
            .and(predicates::str::contains(&available_devices_string_list)),
    );
}

#[rstest::rstest]
#[tokio::test]
async fn multiple_device_found(tmp_path: TmpPath) {
    let config = super::get_testenv_config();
    let url = super::new_environment(&tmp_path, None, config, false)
        .await
        .unwrap()
        .unwrap();

    let org_id = super::unique_org_id();
    let org_addr = parsec_cli::commands::organization::create::create_organization_req(
        &org_id,
        &url,
        crate::testenv_utils::DEFAULT_ADMINISTRATION_TOKEN,
    )
    .await
    .unwrap();

    let client_config = libparsec::ClientConfig::default();
    let first_available_device =
        parsec_cli::commands::organization::bootstrap::bootstrap_organization_req(
            client_config.clone(),
            org_addr,
            "dev1".parse().unwrap(),
            libparsec::HumanHandle::from_raw("first@dev1.com", "First").unwrap(),
            crate::testenv_utils::DEFAULT_DEVICE_PASSWORD
                .to_string()
                .into(),
            None,
        )
        .await
        .unwrap();

    let first_device = libparsec::load_device(
        &client_config.config_dir,
        &libparsec_client::DeviceAccessStrategy::Password {
            key_file: libparsec::get_default_key_file(
                &client_config.config_dir,
                first_available_device.device_id,
            ),
            password: crate::testenv_utils::DEFAULT_DEVICE_PASSWORD
                .to_string()
                .into(),
        },
    )
    .await
    .unwrap();

    let first_auth_cmds = libparsec::AuthenticatedCmds::new(
        &client_config.config_dir,
        first_device.clone(),
        libparsec::ProxyConfig::default(),
    )
    .unwrap();

    let first_dev_id_prefix = &first_device.device_id.hex()[..8];

    let second_device = create_second_device(
        &first_auth_cmds,
        &first_device,
        client_config,
        first_dev_id_prefix,
    )
    .await
    .unwrap();

    let mut devices = [first_device, second_device];
    devices.sort_by_cached_key(|d| d.device_id.hex());

    let mut available_devices_string_list = String::new();
    format_devices(&devices, &mut available_devices_string_list).unwrap();

    crate::assert_cmd_failure!("user", "list", "--device", first_dev_id_prefix).stderr(
        predicates::str::contains(format!(
            "Error: Multiple devices found for `{first_dev_id_prefix}`:"
        ))
        .and(predicates::str::contains(available_devices_string_list)),
    );
}

async fn create_second_device(
    cmds: &libparsec::AuthenticatedCmds,
    author: &Arc<LocalDevice>,
    client_config: libparsec::ClientConfig,
    dev_id_prefix: &str,
) -> anyhow::Result<Arc<LocalDevice>> {
    // Create a device id with the same prefix as the first device
    let second_device_id = {
        let new_id_hex = libparsec::DeviceID::default().hex();
        libparsec::DeviceID::from_hex(&format!(
            "{}{}",
            &dev_id_prefix,
            &new_id_hex[dev_id_prefix.len()..]
        ))
        .map_err(anyhow::Error::msg)
    }?;

    let second_device = Arc::new(LocalDevice::generate_new_device(
        cmds.addr().clone(),
        libparsec::UserProfile::Standard,
        HumanHandle::from_raw("second@dev2.com", "Second").map_err(anyhow::Error::msg)?,
        "second".parse().map_err(anyhow::Error::msg)?,
        None,
        Some(second_device_id),
        None,
        None,
        None,
        None,
        None,
    ));

    let now = author.now();
    let (user_cert, redacted_user_cert) = create_new_user(
        second_device.clone(),
        author.clone(),
        libparsec::UserProfile::Standard,
        now,
    );
    let (dev_cert, redacted_dev_cert) =
        create_new_device(second_device.clone(), author.clone(), now);

    match cmds
        .send(libparsec::authenticated_cmds::latest::user_create::Req {
            device_certificate: dev_cert,
            redacted_device_certificate: redacted_dev_cert,
            redacted_user_certificate: redacted_user_cert,
            user_certificate: user_cert,
        })
        .await?
    {
        libparsec::authenticated_cmds::latest::user_create::UserCreateRep::Ok => (),
        invalid_rep => panic!("Cannot create user: {invalid_rep:?}"),
    }

    let key_file =
        libparsec::get_default_key_file(&client_config.config_dir, second_device.device_id);

    let second_dev_access = libparsec_client::DeviceSaveStrategy::Password {
        password: crate::testenv_utils::DEFAULT_DEVICE_PASSWORD
            .to_string()
            .into(),
    };

    libparsec::save_device(
        std::path::Path::new(""),
        &second_dev_access,
        &second_device,
        key_file,
    )
    .await?;

    Ok(second_device)
}

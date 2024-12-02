// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ProxyConfig;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    bootstrap_organization, test_organization_bootstrap_finalize_ctx_factory,
    BootstrapOrganizationError, Client, ClientConfig, EventBus, MountpointMountStrategy,
    WorkspaceStorageCacheSize,
};

// should be the same as bootstrap token defined in server/parsec/backend.py L91 as TEST_BOOTSTRAP_TOKEN
const TEST_BOOTSTRAP_TOKEN_STR: &str = "672bc6ba9c43455da28344e975dc72b7";

fn test_bootstrap_token() -> BootstrapToken {
    BootstrapToken::from_hex(TEST_BOOTSTRAP_TOKEN_STR).unwrap()
}

fn make_config(env: &TestbedEnv) -> Arc<ClientConfig> {
    Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        with_monitors: false,
        prevent_sync_pattern: PreventSyncPattern::from_regex_str(r"\.tmp$").unwrap(),
    })
}

#[parsec_test(testbed = "empty", with_server)]
#[case::regular(false)]
#[case::sequestered(true)]
async fn ok(#[case] sequestered: bool, env: &TestbedEnv) {
    let human_handle: HumanHandle = "John Doe <john.doe@example.com>".parse().unwrap();
    let device_label: DeviceLabel = "My Machine".parse().unwrap();
    let sequester_authority_verify_key = if sequestered {
        let (_, verify_key) = SequesterSigningKeyDer::generate_pair(SequesterKeySize::_1024Bits);
        Some(verify_key)
    } else {
        None
    };

    let bootstrap_addr = ParsecOrganizationBootstrapAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        Some(test_bootstrap_token()),
    );
    let config = make_config(env);
    let event_bus = EventBus::default();
    let finalize_ctx = bootstrap_organization(
        config.clone(),
        event_bus.clone(),
        bootstrap_addr.clone(),
        human_handle.clone(),
        device_label.clone(),
        sequester_authority_verify_key,
    )
    .await
    .unwrap();

    let device = finalize_ctx.new_local_device;

    assert_eq!(device.human_handle, human_handle);
    assert_eq!(device.device_label, device_label);
    assert_eq!(device.initial_profile, UserProfile::Admin);

    // Ensure the device can be used, and the server now accept it commands
    let client = Client::start(config.clone(), event_bus.clone(), device)
        .await
        .unwrap();

    client.poll_server_for_new_certificates().await.unwrap();

    client.stop().await;

    // Finally try to re-use the token

    let outcome = bootstrap_organization(
        config,
        event_bus,
        bootstrap_addr,
        human_handle,
        device_label,
        None,
    )
    .await;

    p_assert_matches!(outcome, Err(BootstrapOrganizationError::AlreadyUsedToken));
}

#[parsec_test(testbed = "minimal")]
async fn finalize(tmp_path: TmpPath, env: &TestbedEnv) {
    let config = make_config(env);
    let access = DeviceAccessStrategy::Password {
        key_file: tmp_path.join("alice.keys"),
        password: "P@ssw0rd.".to_owned().into(),
    };

    let finalize_ctx = test_organization_bootstrap_finalize_ctx_factory(
        config.clone(),
        env.local_device("alice@dev1"),
    );
    let device = finalize_ctx.new_local_device.clone();
    finalize_ctx.save_local_device(&access).await.unwrap();

    // Round-trip check
    let reloaded = libparsec_platform_device_loader::load_device(&config.config_dir, &access)
        .await
        .unwrap();
    p_assert_eq!(*reloaded, *device);
}

enum BadFinalizeKind {
    DeviceSaveError,
    UserStorageInitError,
}

#[parsec_test(testbed = "minimal")]
#[case::device_save_error(BadFinalizeKind::DeviceSaveError)]
#[case::user_storage_init_error(BadFinalizeKind::UserStorageInitError)]
async fn bad_finalize(tmp_path: TmpPath, #[case] kind: BadFinalizeKind, env: &TestbedEnv) {
    let device = env.local_device("alice@dev1");
    let config = {
        let mut config = make_config(env);
        let p = Arc::make_mut(&mut config);
        p.config_dir = tmp_path.join("config");
        p.data_base_dir = tmp_path.join("data");
        config
    };
    let access = DeviceAccessStrategy::Password {
        key_file: config.config_dir.join("alice.keys"),
        password: "P@ssw0rd.".to_owned().into(),
    };

    // Simple way to make the folder unwritable
    match kind {
        BadFinalizeKind::DeviceSaveError => {
            std::fs::write(&config.config_dir, b"").unwrap();
        }
        BadFinalizeKind::UserStorageInitError => {
            std::fs::write(&config.data_base_dir, b"").unwrap();
        }
    }

    let finalize_ctx = test_organization_bootstrap_finalize_ctx_factory(config, device);
    p_assert_matches!(
        finalize_ctx.save_local_device(&access).await,
        Err(anyhow::Error { .. })
    );
}

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let bootstrap_addr = ParsecOrganizationBootstrapAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        Some(test_bootstrap_token()),
    );
    let config = make_config(env);
    let event_bus = EventBus::default();
    let human_handle: HumanHandle = "John Doe <john.doe@example.com>".parse().unwrap();
    let device_label: DeviceLabel = "My Machine".parse().unwrap();
    let outcome = bootstrap_organization(
        config,
        event_bus,
        bootstrap_addr,
        human_handle,
        device_label,
        None,
    )
    .await;

    p_assert_matches!(outcome, Err(BootstrapOrganizationError::Offline));
}

#[parsec_test(testbed = "empty", with_server)]
async fn bad_token(env: &TestbedEnv) {
    let bootstrap_addr = ParsecOrganizationBootstrapAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        // not the bootstrap token defined in server/parsec/backend.py L91 as TEST_BOOTSTRAP_TOKEN
        Some(BootstrapToken::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap()),
    );
    let config = make_config(env);
    let event_bus = EventBus::default();
    let human_handle: HumanHandle = "John Doe <john.doe@example.com>".parse().unwrap();
    let device_label: DeviceLabel = "My Machine".parse().unwrap();
    let outcome = bootstrap_organization(
        config,
        event_bus,
        bootstrap_addr,
        human_handle,
        device_label,
        None,
    )
    .await;

    p_assert_matches!(outcome, Err(BootstrapOrganizationError::InvalidToken));
}

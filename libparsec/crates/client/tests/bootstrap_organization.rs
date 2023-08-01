// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client::{
    bootstrap_organization, test_organization_bootstrap_finalize_ctx_factory,
    BootstrapOrganizationError, Client, ClientConfig, EventBus,
};
use libparsec_client_connection::ProxyConfig;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

fn make_config(env: &TestbedEnv) -> Arc<ClientConfig> {
    Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_base_dir: env.discriminant_dir.clone(),
        workspace_storage_cache_size: libparsec_client::WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
    })
}

#[parsec_test(testbed = "empty", with_server)]
#[case::full_none(false, false, false)]
#[case::human_handle(true, false, false)]
#[case::device_label(false, true, false)]
#[case::sequestered(false, false, true)]
async fn ok(
    #[case] with_human_handle: bool,
    #[case] with_device_label: bool,
    #[case] sequestered: bool,
    env: &TestbedEnv,
) {
    let human_handle = if with_human_handle {
        Some("John Doe <john.doe@example.com>".parse().unwrap())
    } else {
        None
    };
    let device_label = if with_device_label {
        Some("My Machine".parse().unwrap())
    } else {
        None
    };
    let sequester_authority_verify_key = if sequestered {
        let (_, verify_key) = SequesterSigningKeyDer::generate_pair(SequesterKeySize::_1024Bits);
        Some(verify_key)
    } else {
        None
    };

    let bootstrap_addr = BackendOrganizationBootstrapAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        None,
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

    client
        .certificates_ops
        .poll_server_for_new_certificates(None)
        .await
        .unwrap();

    client.stop().await;

    // Finally try to re-use the token

    let outcome = bootstrap_organization(config, event_bus, bootstrap_addr, None, None, None).await;

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
    let bootstrap_addr = BackendOrganizationBootstrapAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        None,
    );
    let config = make_config(env);
    let event_bus = EventBus::default();
    let outcome = bootstrap_organization(config, event_bus, bootstrap_addr, None, None, None).await;

    p_assert_matches!(outcome, Err(BootstrapOrganizationError::Offline));
}

#[parsec_test(testbed = "empty", with_server)]
async fn bad_token(env: &TestbedEnv) {
    let bootstrap_addr = BackendOrganizationBootstrapAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        Some("<invalid token>".to_owned()),
    );
    let config = make_config(env);
    let event_bus = EventBus::default();
    let outcome = bootstrap_organization(config, event_bus, bootstrap_addr, None, None, None).await;

    p_assert_matches!(outcome, Err(BootstrapOrganizationError::InvalidToken));
}

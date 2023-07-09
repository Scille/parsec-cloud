// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client::{BootstrapOrganizationError, Client, ClientConfig, EventBus};
use libparsec_client_connection::ProxyConfig;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

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
    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_base_dir: env.discriminant_dir.clone(),
        preferred_org_creation_backend_addr: "parsec://example.com".parse().unwrap(),
        workspace_storage_cache_size: libparsec_client::WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
    });
    let event_bus = EventBus::default();
    let device = libparsec_client::invite::bootstrap_organization(
        &config,
        event_bus.clone(),
        bootstrap_addr.clone(),
        human_handle.clone(),
        device_label.clone(),
        sequester_authority_verify_key,
    )
    .await
    .unwrap();

    assert_eq!(device.human_handle, human_handle);
    assert_eq!(device.device_label, device_label);
    assert_eq!(device.initial_profile, UserProfile::Admin);

    // Ensure the device can be used, and the server knows accept it commands
    let client = Client::start(
        config.clone(),
        event_bus.clone(),
        std::sync::Arc::new(device),
    )
    .await
    .unwrap();

    client
        .certificates_ops
        .poll_server_for_new_certificates(None)
        .await
        .unwrap();

    client.stop().await;

    // Finally try to re-use the token

    let outcome = libparsec_client::invite::bootstrap_organization(
        &config,
        event_bus,
        bootstrap_addr,
        None,
        None,
        None,
    )
    .await;

    p_assert_matches!(outcome, Err(BootstrapOrganizationError::AlreadyUsedToken));
}

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let bootstrap_addr = BackendOrganizationBootstrapAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        None,
    );
    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_base_dir: env.discriminant_dir.clone(),
        preferred_org_creation_backend_addr: "parsec://example.com".parse().unwrap(),
        workspace_storage_cache_size: libparsec_client::WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
    });
    let event_bus = EventBus::default();
    let outcome = libparsec_client::invite::bootstrap_organization(
        &config,
        event_bus,
        bootstrap_addr,
        None,
        None,
        None,
    )
    .await;

    p_assert_matches!(outcome, Err(BootstrapOrganizationError::Offline));
}

#[parsec_test(testbed = "empty", with_server)]
async fn bad_token(env: &TestbedEnv) {
    let bootstrap_addr = BackendOrganizationBootstrapAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        Some("<invalid token>".to_owned()),
    );
    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_base_dir: env.discriminant_dir.clone(),
        preferred_org_creation_backend_addr: "parsec://example.com".parse().unwrap(),
        workspace_storage_cache_size: libparsec_client::WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
    });
    let event_bus = EventBus::default();
    let outcome = libparsec_client::invite::bootstrap_organization(
        &config,
        event_bus,
        bootstrap_addr,
        None,
        None,
        None,
    )
    .await;

    p_assert_matches!(outcome, Err(BootstrapOrganizationError::InvalidToken));
}

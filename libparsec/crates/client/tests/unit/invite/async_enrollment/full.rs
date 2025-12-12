// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, sync::Arc};

use super::utils::client_factory;
use libparsec_platform_async::future;
use libparsec_platform_device_loader::{AvailableDeviceType, DeviceSaveStrategy};
use libparsec_protocol::invited_cmds::latest::invite_info::ShamirRecoveryRecipient;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    claimer_retrieve_info, submit_async_enrollment, AnyClaimRetrievedInfoCtx,
    ClaimerRetrieveInfoError, ClientConfig, MountpointMountStrategy, ProxyConfig,
    ShamirRecoveryClaimAddShareError, ShamirRecoveryClaimMaybeFinalizeCtx,
    ShamirRecoveryClaimMaybeRecoverDeviceCtx, ShamirRecoveryClaimPickRecipientError,
    ShamirRecoveryClaimShare, WorkspaceStorageCacheSize,
};

fn make_config(env: &TestbedEnv) -> Arc<ClientConfig> {
    Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        with_monitors: false,
        prevent_sync_pattern: PreventSyncPattern::from_regex(r"\.tmp$").unwrap(),
    })
}

#[parsec_test(testbed = "minimalorg", with_server)]
async fn submit_and_cancel(tmp_path: TmpPath, env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    submit_async_enrollment(
        config,
        async_enrollment_addr,
        false,
        "PC".parse().unwrap(),
        identity_strategy,
    )
    .await
    .unwrap();

    // Re-submit is

    submit_async_enrollment(
        config,
        async_enrollment_addr,
        false,
        "PC".parse().unwrap(),
        identity_strategy,
    )
    .await
    .unwrap();
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

async fn submit_offline(tmp_path: TmpPath) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    submit_async_enrollment(
        config,
        async_enrollment_addr,
        false,
        "PC".parse().unwrap(),
        identity_strategy,
    )
    .await
    .unwrap();

    p_assert_matches!(submit_async_enrollment(&tmp_path,).await, Err());
}

#[parsec_test(testbed = "minimalorg", with_server)]
async fn full_enrollment(tmp_path: TmpPath, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let alice_client = client_factory(&env.discriminant_dir, alice.clone()).await;

    let enrollements = alice_client.list_async_enrollments().await.unwrap();
    // alice_client.accept_async_enrollment(UserProfile::Outsider, enrollment_id, identity_strategy).await.unwrap();
    todo!()
}

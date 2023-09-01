// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use libparsec_client::{
    certificates_ops::CertificatesOps,
    workspace_ops::{EntryInfo, WorkspaceOps},
    ClientConfig, EventBus, WorkspaceStorageCacheSize,
};
use libparsec_client_connection::{AuthenticatedCmds, ProxyConfig};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

pub(crate) async fn workspace_ops_factory(
    env: &TestbedEnv,
    device: &Arc<LocalDevice>,
    realm_id: RealmID,
) -> WorkspaceOps {
    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_base_dir: env.discriminant_dir.clone(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
    });
    let event_bus = EventBus::default();
    let cmds = Arc::new(
        AuthenticatedCmds::new(&config.config_dir, device.clone(), config.proxy.clone()).unwrap(),
    );
    let certificates_ops = Arc::new(
        CertificatesOps::start(
            config.clone(),
            device.clone(),
            event_bus.clone(),
            cmds.clone(),
        )
        .await
        .unwrap(),
    );
    WorkspaceOps::start(
        config.clone(),
        device.clone(),
        cmds,
        certificates_ops,
        event_bus,
        realm_id,
    )
    .await
    .unwrap()
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn entry_info(env: &TestbedEnv) {
    let realm_id = RealmID::from_hex("f0000000000000000000000000000002").unwrap();
    let foo_id = EntryID::from_hex("f0000000000000000000000000000003").unwrap();
    let bar_txt_id = EntryID::from_hex("f0000000000000000000000000000005").unwrap();

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(env, &alice, realm_id).await;

    // Workspace

    let info = ops.entry_info(&"/".parse().unwrap()).await.unwrap();
    p_assert_matches!(
        info,
        EntryInfo::Folder{
            confinement_point,
            id,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            children,
        }
        if confinement_point.is_none() &&
        id == realm_id.into() &&
        created == "2000-01-07T00:00:00Z".parse().unwrap() &&
        updated == "2000-01-07T00:00:00Z".parse().unwrap() &&
        base_version == 1 &&
        !is_placeholder &&
        !need_sync &&
        children == ["bar.txt".parse().unwrap(), "foo".parse().unwrap()]
    );

    // Folder

    let info = ops.entry_info(&"/foo".parse().unwrap()).await.unwrap();
    p_assert_matches!(
        info,
        EntryInfo::Folder{
            confinement_point,
            id,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            children,
        }
        if confinement_point.is_none() &&
        id == foo_id &&
        created == "2000-01-04T00:00:00Z".parse().unwrap() &&
        updated == "2000-01-04T00:00:00Z".parse().unwrap() &&
        base_version == 1 &&
        !is_placeholder &&
        !need_sync &&
        children.is_empty()
    );

    // File

    let info = ops.entry_info(&"/bar.txt".parse().unwrap()).await.unwrap();
    p_assert_matches!(
        info,
        EntryInfo::File{
            confinement_point,
            id,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            size,
        }
        if confinement_point.is_none() &&
        id == bar_txt_id &&
        created == "2000-01-06T00:00:00Z".parse().unwrap() &&
        updated == "2000-01-06T00:00:00Z".parse().unwrap() &&
        base_version == 1 &&
        !is_placeholder &&
        !need_sync &&
        size == 11  // Contains "hello world"
    );
}

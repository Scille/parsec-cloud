// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_send_hook, test_register_sequence_of_send_hooks,
    test_send_hook_realm_get_keys_bundle,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::workspace::WorkspaceHistoryGetWorkspaceManifestV1TimestampError;

use super::super::utils::workspace_ops_factory;

#[parsec_test(testbed = "minimal_client_ready")]
async fn v1_exists(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Get back last workspace manifest version synced in server
    let (wksp1_v1_remote_manifest, wksp1_v1_encrypted) = env
        .template
        .events
        .iter()
        .find_map(|e| match e {
            TestbedEvent::CreateOrUpdateFolderManifestVlob(e)
                if e.manifest.id == wksp1_id && e.manifest.version == 1 =>
            {
                Some((e.manifest.clone(), e.encrypted(&env.template)))
            }
            _ => None,
        })
        .unwrap();
    let wksp1_v1_timestamp = wksp1_v1_remote_manifest.timestamp;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Read the workspace manifest's vlob v1
        {
            let last_realm_certificate_timestamp =
                env.get_last_realm_certificate_timestamp(wksp1_id);
            let last_common_certificate_timestamp = env.get_last_common_certificate_timestamp();
            move |req: authenticated_cmds::latest::vlob_read_versions::Req| {
                p_assert_eq!(req.realm_id, wksp1_id);
                p_assert_eq!(req.items, [(wksp1_id, 1)]);
                authenticated_cmds::latest::vlob_read_versions::Rep::Ok {
                    items: vec![(
                        wksp1_id,
                        1,
                        wksp1_v1_remote_manifest.author,
                        wksp1_v1_remote_manifest.version,
                        wksp1_v1_remote_manifest.timestamp,
                        wksp1_v1_encrypted,
                    )],
                    needed_common_certificate_timestamp: last_common_certificate_timestamp,
                    needed_realm_certificate_timestamp: last_realm_certificate_timestamp,
                }
            }
        },
        // 2) Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
    );

    p_assert_eq!(
        ops.history
            .get_workspace_manifest_v1_timestamp()
            .await
            .unwrap(),
        Some(wksp1_v1_timestamp)
    );

    // The result is in cache so no more request should be needed now
    p_assert_eq!(
        ops.history
            .get_workspace_manifest_v1_timestamp()
            .await
            .unwrap(),
        Some(wksp1_v1_timestamp)
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn still_in_v0(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    test_register_send_hook(&env.discriminant_dir, {
        let last_realm_certificate_timestamp = env.get_last_realm_certificate_timestamp(wksp1_id);
        let last_common_certificate_timestamp = env.get_last_common_certificate_timestamp();
        move |req: authenticated_cmds::latest::vlob_read_versions::Req| {
            p_assert_eq!(req.realm_id, wksp1_id);
            p_assert_eq!(req.items, [(wksp1_id, 1)]);
            authenticated_cmds::latest::vlob_read_versions::Rep::Ok {
                items: vec![],
                needed_common_certificate_timestamp: last_common_certificate_timestamp,
                needed_realm_certificate_timestamp: last_realm_certificate_timestamp,
            }
        }
    });

    p_assert_eq!(
        ops.history
            .get_workspace_manifest_v1_timestamp()
            .await
            .unwrap(),
        None
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn offline(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    p_assert_matches!(
        ops.history
            .get_workspace_manifest_v1_timestamp()
            .await
            .unwrap_err(),
        WorkspaceHistoryGetWorkspaceManifestV1TimestampError::Offline(_)
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stopped(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    ops.certificates_ops.stop().await.unwrap();

    // Get back last workspace manifest version synced in server
    let (wksp1_v1_remote_manifest, wksp1_v1_encrypted) = env
        .template
        .events
        .iter()
        .find_map(|e| match e {
            TestbedEvent::CreateOrUpdateFolderManifestVlob(e)
                if e.manifest.id == wksp1_id && e.manifest.version == 1 =>
            {
                Some((e.manifest.clone(), e.encrypted(&env.template)))
            }
            _ => None,
        })
        .unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Get back the workspace manifest's vlob v1...
        {
            let last_realm_certificate_timestamp =
                env.get_last_realm_certificate_timestamp(wksp1_id);
            let last_common_certificate_timestamp = env.get_last_common_certificate_timestamp();
            move |req: authenticated_cmds::latest::vlob_read_versions::Req| {
                p_assert_eq!(req.realm_id, wksp1_id);
                p_assert_eq!(req.items, [(wksp1_id, 1)]);
                authenticated_cmds::latest::vlob_read_versions::Rep::Ok {
                    items: vec![(
                        wksp1_id,
                        1,
                        wksp1_v1_remote_manifest.author,
                        wksp1_v1_remote_manifest.version,
                        wksp1_v1_remote_manifest.timestamp,
                        wksp1_v1_encrypted,
                    )],
                    needed_common_certificate_timestamp: last_common_certificate_timestamp,
                    needed_realm_certificate_timestamp: last_realm_certificate_timestamp,
                }
            }
        },
        // ...should be checking the workspace manifest, but the certificate ops
        // is stopped so nothing more happened !
    );

    p_assert_matches!(
        ops.history
            .get_workspace_manifest_v1_timestamp()
            .await
            .unwrap_err(),
        WorkspaceHistoryGetWorkspaceManifestV1TimestampError::Stopped
    );
}

#[parsec_test(testbed = "coolorg")]
async fn no_realm_access(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let mallory = env.local_device("mallory@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &mallory, wksp1_id).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::vlob_read_versions::Req| {
            p_assert_eq!(req.realm_id, wksp1_id);
            p_assert_eq!(req.items, [(wksp1_id, 1)]);
            authenticated_cmds::latest::vlob_read_versions::Rep::AuthorNotAllowed
        }
    );

    p_assert_matches!(
        ops.history
            .get_workspace_manifest_v1_timestamp()
            .await
            .unwrap_err(),
        WorkspaceHistoryGetWorkspaceManifestV1TimestampError::NoRealmAccess
    );
}

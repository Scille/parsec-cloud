// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_realm_get_keys_bundle,
};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;
use crate::{
    ClientArchiveWorkspaceError, EventNewCertificates, EventWorkspacesSelfListChanged,
    WorkspaceInfo,
};

#[parsec_test(testbed = "coolorg")]
#[case::archived(RealmArchivingConfiguration::Archived)]
#[case::available(RealmArchivingConfiguration::Available)]
#[case::deletion_planned(RealmArchivingConfiguration::DeletionPlanned {
    deletion_date: DateTime::from_ymd_hms_us(2030, 1, 1, 0, 0, 0, 0).unwrap()
})]
async fn ok(#[case] configuration: RealmArchivingConfiguration, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let alice_user_id = alice.user_id;
    let client = client_factory(&env.discriminant_dir, alice).await;

    // Mock requests to server
    let new_realm_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Archive
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |req: authenticated_cmds::latest::realm_update_archiving::Req| {
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.archiving_certificate);
                authenticated_cmds::latest::realm_update_archiving::Rep::Ok
            }
        },
        // 2) Fetch new certificates
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |_req: authenticated_cmds::latest::certificate_get::Req| {
                authenticated_cmds::latest::certificate_get::Rep::Ok {
                    common_certificates: vec![],
                    realm_certificates: HashMap::from_iter([(
                        wksp1_id,
                        new_realm_certificates.lock().unwrap().clone(),
                    )]),
                    sequester_certificates: vec![],
                    shamir_recovery_certificates: vec![],
                }
            }
        },
        // 3) Refresh workspace list fetches realm keys bundle to decrypt the name
        test_send_hook_realm_get_keys_bundle!(env, alice_user_id, wksp1_id),
    );

    let mut spy = client.event_bus.spy.start_expecting();

    client
        .archive_workspace(wksp1_id, configuration)
        .await
        .unwrap();

    spy.assert_next(|_event: &EventNewCertificates| {});
    spy.assert_next(|_event: &EventWorkspacesSelfListChanged| {});

    let mut workspaces = client.list_workspaces().await;
    p_assert_eq!(workspaces.len(), 1, "{:?}", workspaces);

    let wksp1_info = workspaces.pop().unwrap();

    let WorkspaceInfo {
        id,
        is_started,
        is_bootstrapped,
        name,
        name_origin,
        self_role,
        self_role_origin,
        archiving_configuration,
        archiving_configuration_origin,
    } = wksp1_info;
    p_assert_eq!(id, wksp1_id);
    p_assert_eq!(is_bootstrapped, true);
    p_assert_eq!(is_started, false);
    p_assert_eq!(name, "wksp1".parse().unwrap());
    p_assert_eq!(
        name_origin,
        CertificateBasedInfoOrigin::Certificate {
            timestamp: "2000-01-11T00:00:00Z".parse().unwrap()
        }
    );
    p_assert_eq!(self_role, RealmRole::Owner);
    p_assert_eq!(
        self_role_origin,
        CertificateBasedInfoOrigin::Certificate {
            timestamp: "2000-01-09T00:00:00Z".parse().unwrap()
        }
    );
    p_assert_eq!(archiving_configuration, configuration);
    p_assert_matches!(
        archiving_configuration_origin,
        CertificateBasedInfoOrigin::Certificate { .. }
    );
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok_require_bootstrap_before_archive(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    // Create a workspace but don't sync it...
    let wid = client
        .create_workspace("wksp2".parse().unwrap())
        .await
        .unwrap();

    client
        .archive_workspace(wid, RealmArchivingConfiguration::Archived)
        .await
        .unwrap();
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok_placeholder_workspace(env: &TestbedEnv) {
    let wksp2_id = env
        .customize(|builder| {
            let wksp2_id = builder.new_realm("alice").map(|e| e.realm_id);
            builder.certificates_storage_fetch_certificates("alice@dev1");
            builder
                .user_storage_local_update("alice@dev1")
                .update_local_workspaces_with_fetched_certificates();
            wksp2_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    client
        .archive_workspace(wksp2_id, RealmArchivingConfiguration::Archived)
        .await
        .unwrap();
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn to_unknown_workspace(env: &TestbedEnv) {
    let dummy_id: VlobID = VlobID::default();

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .archive_workspace(dummy_id, RealmArchivingConfiguration::Archived)
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientArchiveWorkspaceError::WorkspaceNotFound);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn not_allowed(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    let err = client
        .archive_workspace(wksp1_id, RealmArchivingConfiguration::Archived)
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientArchiveWorkspaceError::AuthorNotAllowed);
}

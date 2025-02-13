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
use crate::{ClientProcessWorkspacesNeedsError, RealmNeeds};

#[parsec_test(testbed = "minimal_client_ready")]
async fn noop(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    p_assert_eq!(
        client
            .certificates_ops
            .get_realm_needs(wksp1_id)
            .await
            .unwrap(),
        RealmNeeds::Nothing,
    );

    client.process_workspaces_needs().await.unwrap();
}

#[parsec_test(testbed = "minimal")]
async fn local_only_realm_noop(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let realm_id = client
        .create_workspace("my realm".parse().unwrap())
        .await
        .unwrap();

    p_assert_eq!(
        client
            .certificates_ops
            .get_realm_needs(realm_id)
            .await
            .unwrap(),
        RealmNeeds::Nothing,
    );

    client.process_workspaces_needs().await.unwrap();
}

#[parsec_test(testbed = "sequestered")]
async fn need_key_rotation_only(
    #[values(
        "user_unshared",
        "sequester_service_created",
        "sequester_service_revoked"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let sequester_service_2_id: SequesterServiceID =
        *env.template.get_stuff("sequester_service_2_id");
    let (expected_participants, expected_sequester_services) = env
        .customize(|builder| {
            let (mut expected_participants, mut expected_sequester_services) = match kind {
                "user_unshared" => {
                    builder.share_realm(wksp1_id, "bob", None);
                    builder.certificates_storage_fetch_certificates("alice@dev1");
                    (vec!["alice".parse().unwrap()], vec![sequester_service_2_id])
                }
                "sequester_service_created" => {
                    let sequester_service_3_id = builder.new_sequester_service().map(|e| e.id);
                    builder.certificates_storage_fetch_certificates("alice@dev1");
                    (
                        vec!["alice".parse().unwrap(), "bob".parse().unwrap()],
                        vec![sequester_service_2_id, sequester_service_3_id],
                    )
                }
                "sequester_service_revoked" => {
                    builder.revoke_sequester_service(sequester_service_2_id);
                    builder.certificates_storage_fetch_certificates("alice@dev1");
                    (
                        vec!["alice".parse().unwrap(), "bob".parse().unwrap()],
                        vec![],
                    )
                }
                unknown => panic!("Unknown kind: {}", unknown),
            };
            expected_participants.sort();
            expected_sequester_services.sort();
            (expected_participants, expected_sequester_services)
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    // Sanity check
    p_assert_eq!(
        client
            .certificates_ops
            .get_realm_needs(wksp1_id)
            .await
            .unwrap(),
        RealmNeeds::KeyRotationOnly {
            current_key_index: Some(3)
        },
    );

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, "alice".parse().unwrap(), wksp1_id),
        {
            move |req: authenticated_cmds::latest::realm_rotate_key::Req| {
                let mut participants: Vec<_> = req
                    .per_participant_keys_bundle_access
                    .keys()
                    .copied()
                    .collect();
                participants.sort();
                p_assert_eq!(participants, expected_participants);

                let sequester_services = req.per_sequester_service_keys_bundle_access.map(|e| {
                    let mut sequester_services = e.keys().copied().collect::<Vec<_>>();
                    sequester_services.sort();
                    sequester_services
                });
                p_assert_eq!(sequester_services, Some(expected_sequester_services));

                authenticated_cmds::latest::realm_rotate_key::Rep::Ok
            }
        },
    );

    client.process_workspaces_needs().await.unwrap();
}

#[parsec_test(testbed = "coolorg")]
async fn need_unshare_then_key_rotation(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    env.customize(|builder| {
        builder.revoke_user("bob");
        builder.certificates_storage_fetch_certificates("alice@dev1");
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    // Sanity check
    p_assert_eq!(
        client
            .certificates_ops
            .get_realm_needs(wksp1_id)
            .await
            .unwrap(),
        RealmNeeds::UnshareThenKeyRotation {
            current_key_index: Some(1),
            revoked_users: vec!["bob".parse().unwrap()]
        }
    );

    let new_realm_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |req: authenticated_cmds::latest::realm_unshare::Req| {
                let certif =
                    RealmRoleCertificate::unsecure_load(req.realm_role_certificate.clone())
                        .unwrap()
                        .skip_validation(UnsecureSkipValidationReason::Test);
                p_assert_eq!(certif.user_id, "bob".parse().unwrap());
                p_assert_eq!(certif.role, None);
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_role_certificate);

                authenticated_cmds::latest::realm_unshare::Rep::Ok
            }
        },
        // Fetch back the unsharing certificates
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
        test_send_hook_realm_get_keys_bundle!(env, "alice".parse().unwrap(), wksp1_id),
        {
            move |req: authenticated_cmds::latest::realm_rotate_key::Req| {
                let participants: Vec<_> = req
                    .per_participant_keys_bundle_access
                    .keys()
                    .copied()
                    .collect();
                p_assert_eq!(participants, vec!["alice".parse().unwrap()]);

                authenticated_cmds::latest::realm_rotate_key::Rep::Ok
            }
        },
    );

    client.process_workspaces_needs().await.unwrap();
}

#[parsec_test(testbed = "coolorg")]
async fn needs_but_user_not_owner(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    env.customize(|builder| {
        builder.share_realm(wksp1_id, "bob", None);
        builder.share_realm(wksp1_id, "mallory", RealmRole::Contributor);
        builder.certificates_storage_fetch_certificates("mallory@dev1");
    })
    .await;

    let mallory = env.local_device("mallory@dev1");
    let client = client_factory(&env.discriminant_dir, mallory).await;

    // Sanity check
    p_assert_eq!(
        client
            .certificates_ops
            .get_realm_needs(wksp1_id)
            .await
            .unwrap(),
        RealmNeeds::KeyRotationOnly {
            current_key_index: Some(1)
        }
    );

    client.process_workspaces_needs().await.unwrap();
}

#[parsec_test(testbed = "minimal", with_server)]
async fn multiple_realms_and_needs(env: &TestbedEnv) {
    let (realm1_id, realm2_id, realm3_id) = env
        .customize(|builder| {
            builder.new_user("bob");
            builder.new_user("mallory");
            let realm1_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|e| e.realm);
            let realm2_id = builder
                .new_realm("bob")
                .then_do_initial_key_rotation()
                .map(|e| e.realm);
            let realm3_id = builder
                .new_realm("mallory")
                .then_do_initial_key_rotation()
                .map(|e| e.realm);
            builder.share_realm(realm1_id, "mallory", RealmRole::Contributor);
            builder.share_realm(realm2_id, "alice", RealmRole::Owner);
            builder.share_realm(realm2_id, "mallory", RealmRole::Owner);
            builder.share_realm(realm3_id, "alice", RealmRole::Owner);
            builder.revoke_user("bob");
            builder.share_realm(realm3_id, "mallory", None);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            (realm1_id, realm2_id, realm3_id)
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    client.process_workspaces_needs().await.unwrap();

    client.poll_server_for_new_certificates().await.unwrap();

    macro_rules! list_workspaces_users {
        ($client: expr, $realm_id: expr) => {{
            let mut users = $client
                .list_workspace_users($realm_id)
                .await
                .unwrap()
                .into_iter()
                .map(|e| e.user_id)
                .collect::<Vec<_>>();
            users.sort();
            users
        }};
    }
    p_assert_eq!(
        list_workspaces_users!(client, realm1_id),
        vec!["mallory".parse().unwrap(), "alice".parse().unwrap()],
    );
    p_assert_eq!(
        list_workspaces_users!(client, realm2_id),
        vec!["mallory".parse().unwrap(), "alice".parse().unwrap()],
    );
    p_assert_eq!(
        list_workspaces_users!(client, realm3_id),
        vec!["alice".parse().unwrap()],
    );
}

#[parsec_test(testbed = "coolorg")]
async fn offline(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    env.customize(|builder| {
        builder.share_realm(wksp1_id, "bob", None);
        builder.certificates_storage_fetch_certificates("alice@dev1");
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client.process_workspaces_needs().await.unwrap_err();
    p_assert_matches!(err, ClientProcessWorkspacesNeedsError::Offline(_));
}

#[parsec_test(testbed = "coolorg")]
async fn stopped(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    env.customize(|builder| {
        builder.share_realm(wksp1_id, "bob", None);
        builder.certificates_storage_fetch_certificates("alice@dev1");
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    client.certificates_ops.stop().await.unwrap();

    let err = client.process_workspaces_needs().await.unwrap_err();
    p_assert_matches!(err, ClientProcessWorkspacesNeedsError::Stopped);
}

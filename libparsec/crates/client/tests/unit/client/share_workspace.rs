// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::{test_register_send_hook, test_register_sequence_of_send_hooks};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;
use crate::{ClientShareWorkspaceError, WorkspaceUserAccessInfo};

#[parsec_test(testbed = "coolorg", with_server)]
async fn maxou(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    env.customize(|builder| {
        // Bob starts as a READER, switch it to MANAGER...
        builder
            .share_realm(wksp1_id, "bob", RealmRole::Manager)
            .customize(|e| {
                e.author = "alice@dev1".parse().unwrap();
            });
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;
    client
        .share_workspace(wksp1_id, "bob".parse().unwrap(), Some(RealmRole::Manager))
        .await
        .unwrap();

    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    client
        .share_workspace(
            wksp1_id,
            "mallory".parse().unwrap(),
            Some(RealmRole::Reader),
        )
        .await
        .unwrap();
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok(#[values("author_is_owner", "author_is_manager")] kind: &str, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let mallory = env.local_device("mallory@dev1");

    let author = match kind {
        "author_is_owner" => alice.clone(),
        "author_is_manager" => bob.clone(),
        unknown => panic!("Unknown kind: {}", unknown),
    };
    let client = client_factory(&env.discriminant_dir, author.clone()).await;

    let mut expected_wksp1_access_info = vec![
        WorkspaceUserAccessInfo {
            user_id: alice.user_id().to_owned(),
            human_handle: alice.human_handle.clone(),
            current_profile: UserProfile::Admin,
            current_role: RealmRole::Owner,
        },
        WorkspaceUserAccessInfo {
            user_id: bob.user_id().to_owned(),
            human_handle: bob.human_handle.clone(),
            current_profile: UserProfile::Standard,
            current_role: RealmRole::Reader,
        },
    ];

    // 1) Give initial access

    client
        .share_workspace(
            wksp1_id,
            "mallory".parse().unwrap(),
            Some(RealmRole::Manager),
        )
        .await
        .unwrap();

    let wksp1_access_info = client.list_workspace_users(wksp1_id).await.unwrap();
    p_assert_eq!(
        wksp1_access_info,
        [
            WorkspaceUserAccessInfo {
                user_id: alice.user_id().to_owned(),
                human_handle: alice.human_handle.clone(),
                current_profile: UserProfile::Admin,
                current_role: RealmRole::Owner,
            },
            WorkspaceUserAccessInfo {
                user_id: mallory.user_id().to_owned(),
                human_handle: mallory.human_handle.clone(),
                current_profile: UserProfile::Standard,
                current_role: RealmRole::Manager,
            }
        ]
    );

    // Change again

    client
        .share_workspace(
            wksp1_id,
            "mallory".parse().unwrap(),
            Some(RealmRole::Reader),
        )
        .await
        .unwrap();

    let wksp1_access_info = client.list_workspace_users(wksp1_id).await.unwrap();
    p_assert_eq!(
        wksp1_access_info,
        [
            WorkspaceUserAccessInfo {
                user_id: alice.user_id().to_owned(),
                human_handle: alice.human_handle.clone(),
                current_profile: UserProfile::Admin,
                current_role: RealmRole::Owner,
            },
            WorkspaceUserAccessInfo {
                user_id: mallory.user_id().to_owned(),
                human_handle: mallory.human_handle.clone(),
                current_profile: UserProfile::Standard,
                current_role: RealmRole::Reader,
            }
        ]
    );

    // Finally unshare

    client
        .share_workspace(wksp1_id, "mallory".parse().unwrap(), None)
        .await
        .unwrap();

    let wksp1_access_info = client.list_workspace_users(wksp1_id).await.unwrap();
    p_assert_eq!(
        wksp1_access_info,
        [WorkspaceUserAccessInfo {
            user_id: alice.user_id().to_owned(),
            human_handle: alice.human_handle.clone(),
            current_profile: UserProfile::Admin,
            current_role: RealmRole::Owner,
        }]
    );
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok_placeholder_workspace(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    // Create a workspace but don't sync it...
    let wid = client
        .create_workspace("wksp2".parse().unwrap())
        .await
        .unwrap();

    client
        .share_workspace(wid, "bob".parse().unwrap(), Some(RealmRole::Reader))
        .await
        .unwrap();
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn to_self(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .share_workspace(wksp1_id, "alice".parse().unwrap(), Some(RealmRole::Reader))
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::RecipientIsSelf);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn to_unknown_user(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .share_workspace(wksp1_id, "dummy".parse().unwrap(), Some(RealmRole::Reader))
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::RecipientNotFound);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn to_unknown_workspace(env: &TestbedEnv) {
    let dummy_id: VlobID = VlobID::default();

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .share_workspace(dummy_id, "mallory".parse().unwrap(), Some(RealmRole::Owner))
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::WorkspaceNotFound);
}

#[parsec_test(testbed = "coolorg")]
async fn cannot_share_with_revoked_recipient(
    #[values("from_client", "from_server")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    match kind {
        "from_client" => {
            env.customize(|builder| {
                builder.revoke_user("mallory");
                builder.certificates_storage_fetch_certificates("alice@dev1");
            })
            .await;
        }
        "from_server" => {
            env.customize(|builder| {
                builder.revoke_user("mallory");
            })
            .await;

            test_register_send_hook(
                &env.discriminant_dir,
                |_req: authenticated_cmds::latest::realm_share::Req| {
                    authenticated_cmds::latest::realm_share::Rep::RecipientRevoked
                },
            );
        }
        unknown => panic!("Unknown kind: {}", unknown),
    }

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .share_workspace(
            wksp1_id,
            "mallory".parse().unwrap(),
            Some(RealmRole::Reader),
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::RecipientRevoked);
}

#[parsec_test(testbed = "coolorg")]
async fn can_unshare_with_revoked_recipient(
    #[values("from_client", "from_server")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    match kind {
        "from_client" => {
            env.customize(|builder| {
                builder.revoke_user("bob");
                builder.certificates_storage_fetch_certificates("alice@dev1");
            })
            .await;
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                |_req: authenticated_cmds::latest::realm_share::Req| {
                    authenticated_cmds::latest::realm_share::Rep::Ok
                },
            );
        }
        "from_server" => {
            env.customize(|builder| {
                builder.revoke_user("bob");
            })
            .await;

            test_register_send_hook(
                &env.discriminant_dir,
                |_req: authenticated_cmds::latest::realm_share::Req| {
                    authenticated_cmds::latest::realm_share::Rep::Ok
                },
            );
        }
        unknown => panic!("Unknown kind: {}", unknown),
    }

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    client
        .share_workspace(wksp1_id, "bob".parse().unwrap(), Some(RealmRole::Reader))
        .await
        .unwrap();

    let wksp1_access_info = client.list_workspace_users(wksp1_id).await.unwrap();
    p_assert_eq!(
        wksp1_access_info,
        [WorkspaceUserAccessInfo {
            user_id: alice.user_id().to_owned(),
            human_handle: alice.human_handle.clone(),
            current_profile: UserProfile::Admin,
            current_role: RealmRole::Owner,
        }]
    );
}

#[parsec_test(testbed = "coolorg")]
async fn author_not_allowed(#[values("from_client", "from_server")] kind: &str, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    match kind {
        "from_client" => (),
        "from_server" => {
            env.customize(|builder| {
                // Bob's client thinks he can share the workspace...
                builder.share_realm(wksp1_id, "bob", RealmRole::Manager);
                builder.certificates_storage_fetch_certificates("bob@dev1");
                // ...but his access has been removed in the meantime !
                builder.share_realm(wksp1_id, "bob", None);
            })
            .await;
        }
        unknown => panic!("Unknown kind: {}", unknown),
    };

    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    let err = client
        .share_workspace(
            wksp1_id,
            "mallory".parse().unwrap(),
            Some(RealmRole::Reader),
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::AuthorNotAllowed);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn role_incompatible_with_outsider(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .share_workspace(wksp1_id, "mallory".parse().unwrap(), Some(RealmRole::Owner))
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::RoleIncompatibleWithOutsider);
}

#[parsec_test(testbed = "coolorg")]
async fn offline(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .share_workspace(
            wksp1_id,
            "mallory".parse().unwrap(),
            Some(RealmRole::Reader),
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::Offline);
}

#[parsec_test(testbed = "coolorg")]
async fn timestamp_out_of_ballpark(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        |_req: authenticated_cmds::latest::realm_share::Req| {
            authenticated_cmds::latest::realm_share::Rep::TimestampOutOfBallpark {
                ballpark_client_early_offset: 100.0,
                ballpark_client_late_offset: 200.0,
                client_timestamp: "2024-05-01T00:00:00Z".parse().unwrap(),
                server_timestamp: "2024-05-01T01:00:00Z".parse().unwrap(),
            }
        },
    );

    let err = client
        .share_workspace(
            wksp1_id,
            "mallory".parse().unwrap(),
            Some(RealmRole::Reader),
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::TimestampOutOfBallpark {
        ballpark_client_early_offset,
        ballpark_client_late_offset,
        client_timestamp,
        server_timestamp,
    } if ballpark_client_early_offset == 100.0 &&
         ballpark_client_late_offset == 200.0 &&
         client_timestamp == "2024-05-01T00:00:00Z".parse().unwrap() &&
         server_timestamp == "2024-05-01T01:00:00Z".parse().unwrap()
    );
}

// #[parsec_test(testbed = "minimal_client_ready")]
// async fn simple(env: &TestbedEnv) {
//     env.customize(|builder: &mut TestbedTemplateBuilder| {
//         builder.new_user("bob");
//         // Fetch bob certificate in alice@dev1
//         builder.certificates_storage_fetch_certificates("alice@dev1");
//     }).await;
//     let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
//     let alice = env.local_device("alice@dev1");
//     let bob = env.local_device("bob@dev1");
//     let user_ops = user_ops_factory(&env, &alice).await;
//     load_realm_keys_bundle(&env, &user_ops, wksp1_id).await;

//     // Mock server command `realm_share`
//     test_register_send_hook(&env.discriminant_dir, {
//         let env = env.clone();
//         move |req: authenticated_cmds::latest::realm_share::Req| {
//             p_assert_eq!(
//                 req.key_index,
//                 env.get_last_realm_keys_bundle_index(wksp1_id)
//             );

//             let decrypted_keys_bundle_access = bob
//                 .private_key
//                 .decrypt_from_self(&req.recipient_keys_bundle_access)
//                 .unwrap();
//             let keys_bundle_access =
//                 RealmKeysBundleAccess::load(&decrypted_keys_bundle_access).unwrap();
//             assert_eq!(
//                 &keys_bundle_access.keys_bundle_key,
//                 env.get_last_realm_keys_bundle_access_key(wksp1_id)
//             );

//             let certif = RealmRoleCertificate::verify_and_load(
//                 &req.realm_role_certificate,
//                 &alice.verify_key(),
//                 CertificateSignerRef::User(&alice.device_id),
//                 Some(wksp1_id),
//                 Some(bob.user_id()),
//             )
//             .unwrap();
//             p_assert_matches!(certif.role, Some(RealmRole::Contributor));

//             authenticated_cmds::latest::realm_share::Rep::Ok {}
//         }
//     });

//     user_ops
//         .share_workspace(
//             wksp1_id,
//             "bob".parse().unwrap(),
//             Some(RealmRole::Contributor),
//         )
//         .await
//         .unwrap();
// }

// #[parsec_test(testbed = "minimal_client_ready")]
// async fn placeholder_workspace(env: &TestbedEnv) {
//     env.customize(|builder: &mut TestbedTemplateBuilder| {
//         builder.new_user("bob");
//         // Fetch bob certificate in alice@dev1
//         builder.certificates_storage_fetch_certificates("alice@dev1");
//     }).await;
//     let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
//     let alice = env.local_device("alice@dev1");
//     let bob = env.local_device("bob@dev1");
//     let user_ops = user_ops_factory(env, &alice).await;
//     load_realm_keys_bundle(&env, &user_ops, wksp1_id).await;

//     // Create a workspace but don't sync it...
//     let wid = user_ops
//         .create_workspace("wksp2".parse().unwrap())
//         .await
//         .unwrap();

//     // Mock server
//     test_register_sequence_of_send_hooks!(
//         &env.discriminant_dir,
//         // 1) `realm_create` for the shared workspace
//         {
//             let alice = alice.clone();
//             move |req: authenticated_cmds::latest::realm_create::Req| {
//                 let certif = RealmRoleCertificate::verify_and_load(
//                     &req.realm_role_certificate,
//                     &alice.verify_key(),
//                     CertificateSignerRef::User(&alice.device_id),
//                     Some(wid),
//                     Some(alice.user_id()),
//                 )
//                 .unwrap();
//                 p_assert_matches!(certif.role, Some(RealmRole::Owner));

//                 authenticated_cmds::latest::realm_create::Rep::Ok {}
//             }
//         },
//         // 2) `realm_share`
//         {
//             let env = env.clone();
//             move |req: authenticated_cmds::latest::realm_share::Req| {
//                 p_assert_eq!(
//                     req.key_index,
//                     env.get_last_realm_keys_bundle_index(wksp1_id)
//                 );

//                 let decrypted_keys_bundle_access = bob
//                     .private_key
//                     .decrypt_from_self(&req.recipient_keys_bundle_access)
//                     .unwrap();
//                 let keys_bundle_access =
//                     RealmKeysBundleAccess::load(&decrypted_keys_bundle_access).unwrap();
//                 assert_eq!(
//                     &keys_bundle_access.keys_bundle_key,
//                     env.get_last_realm_keys_bundle_access_key(wksp1_id)
//                 );

//                 let certif = RealmRoleCertificate::verify_and_load(
//                     &req.realm_role_certificate,
//                     &alice.verify_key(),
//                     CertificateSignerRef::User(&alice.device_id),
//                     Some(wksp1_id),
//                     Some(bob.user_id()),
//                 )
//                 .unwrap();
//                 p_assert_matches!(certif.role, Some(RealmRole::Contributor));

//                 authenticated_cmds::latest::realm_share::Rep::Ok {}
//             }
//         }
//     );

//     // ...and share it, which should trigger its sync before anything else
//     user_ops
//         .share_workspace(wid, "bob".parse().unwrap(), Some(RealmRole::Contributor))
//         .await
//         .unwrap();
// }

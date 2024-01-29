// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// TODO: share with self is not allowed
// TODO: share newly (not sync) created workspace

// use libparsec_client_connection::{
//     protocol::authenticated_cmds, test_register_send_hook, test_register_sequence_of_send_hooks,
// };
// use libparsec_tests_fixtures::prelude::*;
// use libparsec_types::prelude::*;

// use super::utils::{load_realm_keys_bundle, user_ops_factory};
// use crate::user::ShareWorkspaceError;

// #[parsec_test(testbed = "minimal_client_ready")]
// async fn to_self(env: &TestbedEnv) {
//     let alice = env.local_device("alice@dev1");
//     let user_ops = user_ops_factory(env, &alice).await;

//     let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

//     let outcome = user_ops
//         .share_workspace(
//             wksp1_id,
//             alice.user_id().to_owned(),
//             Some(RealmRole::Contributor),
//         )
//         .await;
//     p_assert_matches!(outcome, Err(ShareWorkspaceError::ShareToSelf));
// }

// #[parsec_test(testbed = "minimal_client_ready")]
// async fn to_unknown_user(env: &TestbedEnv) {
//     let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

//     // Note Bob doesn't exist in this env !
//     let alice = env.local_device("alice@dev1");

//     let user_ops = user_ops_factory(env, &alice).await;
//     load_realm_keys_bundle(env, &user_ops, wksp1_id).await;

//     let outcome = user_ops
//         .share_workspace(
//             wksp1_id,
//             "bob".parse().unwrap(),
//             Some(RealmRole::Contributor),
//         )
//         .await;
//     p_assert_matches!(outcome, Err(ShareWorkspaceError::UserNotFound));
// }

// #[parsec_test(testbed = "minimal_client_ready")]
// async fn simple(env: &TestbedEnv) {
//     let env = env.customize(|builder: &mut TestbedTemplateBuilder| {
//         builder.new_user("bob");
//         // Fetch bob certificate in alice@dev1
//         builder.certificates_storage_fetch_certificates("alice@dev1");
//     });
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
//     let env = &env.customize(|builder: &mut TestbedTemplateBuilder| {
//         builder.new_user("bob");
//         // Fetch bob certificate in alice@dev1
//         builder.certificates_storage_fetch_certificates("alice@dev1");
//     });
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

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{protocol::authenticated_cmds, test_register_send_hook};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::user_ops_factory;
use crate::user_ops::WorkspaceShareError;

#[parsec_test(testbed = "minimal_client_ready")]
async fn to_unknown_user(env: &TestbedEnv) {
    // Note Bob doesn't exist in this env !
    let alice = env.local_device("alice@dev1");
    let user_ops = user_ops_factory(env, &alice).await;

    let wid = user_ops.list_workspaces()[0].0;

    let outcome = user_ops
        .workspace_share(wid, &"bob".parse().unwrap(), Some(RealmRole::Contributor))
        .await;
    p_assert_matches!(outcome, Err(WorkspaceShareError::UnknownRecipient));

    user_ops.stop().await;
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn simple(env: &TestbedEnv) {
    let env = &env.customize(|builder: &mut TestbedTemplateBuilder| {
        builder.new_user("bob");
        // Fetch bob certificate in alice@dev1
        builder.certificates_storage_fetch_certificates("alice@dev1");
    });
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let user_ops = user_ops_factory(env, &alice).await;

    let (w_id, w_name, w_key) = {
        let user_manifest = user_ops.test_get_user_manifest();
        let w_entry = &user_manifest.workspaces[0];
        (w_entry.id, w_entry.name.clone(), w_entry.key.clone())
    };

    // Mock server command `vlob_update`
    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::realm_update_roles::Req| {
            let certif = RealmRoleCertificate::verify_and_load(
                &req.role_certificate,
                &alice.verify_key(),
                CertificateSignerRef::User(&alice.device_id),
                Some(w_id),
                Some(bob.user_id()),
            )
            .unwrap();
            p_assert_matches!(certif.role, Some(RealmRole::Contributor));

            match &req.recipient_message {
                Some(recipient_message) => {
                    let msg = MessageContent::decrypt_verify_and_load_for(
                        recipient_message,
                        &bob.private_key,
                        &alice.verify_key(),
                        &alice.device_id,
                        certif.timestamp,
                    )
                    .unwrap();
                    match msg {
                        MessageContent::SharingGranted {
                            author: _,
                            timestamp: _,
                            name,
                            id: _,
                            encryption_revision: _,
                            encrypted_on: _,
                            key,
                        } => {
                            p_assert_eq!(name, w_name);
                            p_assert_eq!(key, w_key);
                        }
                        msg => panic!("Invalid type for message content: {:?}", msg),
                    }
                }
                None => panic!("Missing recipient message !"),
            }

            authenticated_cmds::latest::realm_update_roles::Rep::Ok {}
        },
    );

    user_ops
        .workspace_share(w_id, &"bob".parse().unwrap(), Some(RealmRole::Contributor))
        .await
        .unwrap();

    user_ops.stop().await;
}

// TODO: workspace minimal sync is not implemented yet !

// #[parsec_test(testbed = "minimal_client_ready")]
// async fn placeholder_workspace(env: &TestbedEnv) {
//     let env = &env.customize(|builder: &mut TestbedTemplateBuilder| {
//         builder.new_user("bob");
//         // Fetch bob certificate in alice@dev1
//         builder.certificates_storage_fetch_certificates("alice@dev1");
//     });
//     let alice = env.local_device("alice@dev1");
//     let bob = env.local_device("bob@dev1");
//     let user_ops = user_ops_factory(env, &alice).await;

//     // Create a workspace but don't sync it...
//     let wid = user_ops
//         .workspace_create("wksp2".parse().unwrap())
//         .await
//         .unwrap();

//     let (w_name, w_key) = {
//         let user_manifest = user_ops.get_user_manifest();
//         let w_entry = &user_manifest.workspaces.iter().find(|x| x.id == wid).unwrap();
//         (w_entry.name.clone(), w_entry.key.clone())
//     };

//     // Mock server: 1) `realm_create`
//     test_register_sequence_of_send_hooks!(&env.discriminant_dir,
//         // 1) `realm_create`
//         {
//             let alice = alice.clone();
//             move |req: authenticated_cmds::latest::realm_create::Req| {
//                 let certif = RealmRoleCertificate::verify_and_load(
//                     &req.role_certificate, &alice.verify_key(), CertificateSignerRef::User(&alice.device_id),
//                     Some(wid.into()), Some(alice.user_id())
//                 ).unwrap();
//                 p_assert_matches!(certif.role, Some(RealmRole::Owner));

//                 authenticated_cmds::latest::realm_create::Rep::Ok {}
//             }
//         },
//         // 2) `vlob_create`
//         move |req: authenticated_cmds::latest::vlob_create::Req| {
//             p_assert_eq!(req.realm_id, wid.into());
//             p_assert_eq!(req.vlob_id, wid.into());

//             authenticated_cmds::latest::vlob_create::Rep::Ok {}
//         },
//         // 3) `realm_update_roles`
//         {
//             let alice = alice.clone();
//             move |req: authenticated_cmds::latest::realm_update_roles::Req| {
//                 let certif = RealmRoleCertificate::verify_and_load(
//                     &req.role_certificate,
//                     &alice.verify_key(),
//                     CertificateSignerRef::User(&alice.device_id),
//                     Some(wid.into()),
//                     Some(bob.user_id()),
//                 ).unwrap();
//                 p_assert_matches!(certif.role, Some(RealmRole::Contributor));

//                 match &req.recipient_message {
//                     Some(recipient_message) => {
//                         let msg = MessageContent::decrypt_verify_and_load_for(&recipient_message, &bob.private_key, &alice.verify_key(), &alice.device_id, certif.timestamp).unwrap();
//                         match msg {
//                             MessageContent::SharingGranted { author: _, timestamp: _, name, id: _, encryption_revision: _, encrypted_on: _, key } => {
//                                 p_assert_eq!(name, w_name);
//                                 p_assert_eq!(key, w_key);
//                             }
//                             msg => panic!("Invalid type for message content: {:?}", msg)
//                         }
//                     },
//                     None => panic!("Missing recipient message !"),
//                 }

//                 authenticated_cmds::latest::realm_update_roles::Rep::Ok {}
//             }
//         },
//     );

//     // ...and share it, which should trigger it sync before anything else
//     user_ops.workspace_share(wid, &"bob".parse().unwrap(), Some(RealmRole::Contributor)).await.unwrap();

//     user_ops.stop().await;
// }

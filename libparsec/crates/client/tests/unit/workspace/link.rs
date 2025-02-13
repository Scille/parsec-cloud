// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
    test_send_hook_realm_get_keys_bundle,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::workspace::WorkspaceDecryptPathAddrError;

use super::utils::workspace_ops_factory;

#[parsec_test(testbed = "minimal_client_ready")]
async fn generate(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let target_path = "/foo/bar.txt".parse().unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
    );

    let link = ops.generate_path_addr(&target_path).await.unwrap();

    let (key_derivation, expected_key_index) = env.get_last_realm_key(wksp1_id);
    let key = key_derivation.derive_secret_key_from_uuid(PATH_URL_KEY_DERIVATION_UUID);

    p_assert_eq!(link.workspace_id(), wksp1_id);
    p_assert_eq!(link.key_index(), expected_key_index);

    let cleartext = key.decrypt(link.encrypted_path()).unwrap();
    p_assert_eq!(cleartext, b"/foo/bar.txt");
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn decrypt_path(#[values(true, false)] key_in_storage: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    env.customize(|builder| {
        if !key_in_storage {
            // The link uses a key our client doesn't know about
            builder.rotate_key_realm(wksp1_id);
        }
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let link = {
        let (key_derivation, key_index) = env.get_last_realm_key(wksp1_id);
        let key = key_derivation.derive_secret_key_from_uuid(PATH_URL_KEY_DERIVATION_UUID);
        let encrypted_path = key.encrypt(b"/foo/bar.txt");
        ParsecWorkspacePathAddr::new(
            alice.organization_addr.clone(),
            alice.organization_id().to_owned(),
            wksp1_id,
            key_index,
            encrypted_path,
        )
    };

    if key_in_storage {
        p_assert_eq!(link.key_index(), 1);
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            // Fetch workspace keys bundle to decrypt the vlob
            test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        );
    } else {
        p_assert_eq!(link.key_index(), 2);
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            // 1) Client fetches the keys bundle it knows about (i.e. index 1)...
            test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id, 1),
            // 2) ..then it realizes it is missing a key rotation and hence poll for certificates...
            {
                let mut realm_certificates = env.get_realms_certificates_signed();
                let new_certif = realm_certificates.remove(&wksp1_id).unwrap().pop().unwrap();
                move |_req: authenticated_cmds::latest::certificate_get::Req| {
                    authenticated_cmds::latest::certificate_get::Rep::Ok {
                        common_certificates: vec![],
                        realm_certificates: std::collections::HashMap::from_iter([(
                            wksp1_id,
                            vec![new_certif],
                        )]),
                        sequester_certificates: vec![],
                        shamir_recovery_certificates: vec![],
                    }
                }
            },
            // 3) ...and finally it fetches the actual last keys bundle
            test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        );
    }

    let path = ops.decrypt_path_addr(&link).await.unwrap();
    p_assert_eq!(path, "/foo/bar.txt".parse().unwrap());
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn decrypt_missing_key_and_offline(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    env.customize(|builder| {
        // The link uses a key our client doesn't know about
        builder.rotate_key_realm(wksp1_id);
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let link = {
        let (key_derivation, key_index) = env.get_last_realm_key(wksp1_id);
        let key = key_derivation.derive_secret_key_from_uuid(PATH_URL_KEY_DERIVATION_UUID);
        let encrypted_path = key.encrypt(b"/foo/bar.txt");
        ParsecWorkspacePathAddr::new(
            alice.organization_addr.clone(),
            alice.organization_id().to_owned(),
            wksp1_id,
            key_index,
            encrypted_path,
        )
    };

    let err = ops.decrypt_path_addr(&link).await.unwrap_err();
    p_assert_matches!(err, WorkspaceDecryptPathAddrError::Offline(_));
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn bad_decrypt_realm_unknown_key_index(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let link = {
        let (_, key_index) = env.get_last_realm_key(wksp1_id);
        let encrypted_path = b"<whatever>".to_vec();
        ParsecWorkspacePathAddr::new(
            alice.organization_addr.clone(),
            alice.organization_id().to_owned(),
            wksp1_id,
            key_index + 1,
            encrypted_path,
        )
    };

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Client fetches the keys bundle it knows about...
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // 2) ...since the key index is not in the current keys bundle, client tries to get more certifs...
        move |_req: authenticated_cmds::latest::certificate_get::Req| {
            authenticated_cmds::latest::certificate_get::Rep::Ok {
                common_certificates: vec![],
                realm_certificates: std::collections::HashMap::new(),
                sequester_certificates: vec![],
                shamir_recovery_certificates: vec![],
            }
        },
        // 3) ...but there is none, so the client has not new keys bundle to fetch
    );

    let err = ops.decrypt_path_addr(&link).await.unwrap_err();
    p_assert_matches!(err, WorkspaceDecryptPathAddrError::KeyNotFound);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn decrypt_bad_encryption(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let link = {
        let (_, key_index) = env.get_last_realm_key(wksp1_id);
        let encrypted_path = b"<dummy>".to_vec();
        ParsecWorkspacePathAddr::new(
            alice.organization_addr.clone(),
            alice.organization_id().to_owned(),
            wksp1_id,
            key_index,
            encrypted_path,
        )
    };

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // Fetch workspace keys bundle to decrypt the vlob
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
    );

    let err = ops.decrypt_path_addr(&link).await.unwrap_err();
    p_assert_matches!(err, WorkspaceDecryptPathAddrError::CorruptedData);
}

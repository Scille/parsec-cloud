// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::workspace::WorkspaceDecryptFileLinkPathError;

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
        {
            let keys_bundle = env.get_last_realm_keys_bundle(wksp1_id);
            let keys_bundle_access =
                env.get_last_realm_keys_bundle_access_for(wksp1_id, alice.user_id());
            move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                p_assert_eq!(req.realm_id, wksp1_id);
                p_assert_eq!(req.key_index, 1);
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle,
                    keys_bundle_access,
                }
            }
        },
    );

    let link = ops.generate_file_link(&target_path).await.unwrap();

    let (key, expected_key_index) = env.get_last_realm_key(wksp1_id);

    p_assert_eq!(link.workspace_id(), wksp1_id);
    p_assert_eq!(link.key_index(), expected_key_index);

    let cleartext = key.decrypt(link.encrypted_path()).unwrap();
    p_assert_eq!(cleartext, b"/foo/bar.txt");
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn decrypt_path(#[values(true, false)] key_in_storage: bool, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let keys_bundle_1 = env.get_last_realm_keys_bundle(wksp1_id);
    let keys_bundle_1_access =
        env.get_last_realm_keys_bundle_access_for(wksp1_id, &"alice".parse().unwrap());
    let env = env.customize(|builder| {
        if !key_in_storage {
            // The link uses a key our client doesn't know about
            builder.rotate_key_realm(wksp1_id);
        }
    });

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let link = {
        let (key, key_index) = env.get_last_realm_key(wksp1_id);
        let encrypted_path = key.encrypt(b"/foo/bar.txt");
        ParsecOrganizationFileLinkAddr::new(
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
            move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                p_assert_eq!(req.realm_id, wksp1_id);
                p_assert_eq!(req.key_index, 1);
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle: keys_bundle_1,
                    keys_bundle_access: keys_bundle_1_access,
                }
            }
        );
    } else {
        p_assert_eq!(link.key_index(), 2);
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            // 1) Client fetches the keys bundle it knows about...
            move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                p_assert_eq!(req.realm_id, wksp1_id);
                p_assert_eq!(req.key_index, 1);
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle: keys_bundle_1,
                    keys_bundle_access: keys_bundle_1_access,
                }
            },
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
            {
                let keys_bundle = env.get_last_realm_keys_bundle(wksp1_id);
                let keys_bundle_access =
                    env.get_last_realm_keys_bundle_access_for(wksp1_id, alice.user_id());
                move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                    p_assert_eq!(req.realm_id, wksp1_id);
                    p_assert_eq!(req.key_index, 2);
                    authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                        keys_bundle,
                        keys_bundle_access,
                    }
                }
            },
        );
    }

    let path = ops.decrypt_file_link_path(&link).await.unwrap();
    p_assert_eq!(path, "/foo/bar.txt".parse().unwrap());
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn decrypt_missing_key_and_offline(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let env = env.customize(|builder| {
        // The link uses a key our client doesn't know about
        builder.rotate_key_realm(wksp1_id);
    });

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let link = {
        let (key, key_index) = env.get_last_realm_key(wksp1_id);
        let encrypted_path = key.encrypt(b"/foo/bar.txt");
        ParsecOrganizationFileLinkAddr::new(
            alice.organization_addr.clone(),
            alice.organization_id().to_owned(),
            wksp1_id,
            key_index,
            encrypted_path,
        )
    };

    let err = ops.decrypt_file_link_path(&link).await.unwrap_err();
    p_assert_matches!(err, WorkspaceDecryptFileLinkPathError::Offline);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn bad_decrypt_realm_unknown_key_index(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let link = {
        let (_, key_index) = env.get_last_realm_key(wksp1_id);
        let encrypted_path = b"<whatever>".to_vec();
        ParsecOrganizationFileLinkAddr::new(
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
        {
            let keys_bundle = env.get_last_realm_keys_bundle(wksp1_id);
            let keys_bundle_access =
                env.get_last_realm_keys_bundle_access_for(wksp1_id, alice.user_id());
            move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                p_assert_eq!(req.realm_id, wksp1_id);
                p_assert_eq!(req.key_index, 1);
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle,
                    keys_bundle_access,
                }
            }
        },
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

    let err = ops.decrypt_file_link_path(&link).await.unwrap_err();
    p_assert_matches!(err, WorkspaceDecryptFileLinkPathError::KeyNotFound);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn decrypt_bad_encryption(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let link = {
        let (_, key_index) = env.get_last_realm_key(wksp1_id);
        let encrypted_path = b"<dummy>".to_vec();
        ParsecOrganizationFileLinkAddr::new(
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
        {
            let keys_bundle = env.get_last_realm_keys_bundle(wksp1_id);
            let keys_bundle_access =
                env.get_last_realm_keys_bundle_access_for(wksp1_id, alice.user_id());
            move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                p_assert_eq!(req.realm_id, wksp1_id);
                p_assert_eq!(req.key_index, 1);
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle,
                    keys_bundle_access,
                }
            }
        },
    );

    let err = ops.decrypt_file_link_path(&link).await.unwrap_err();
    p_assert_matches!(err, WorkspaceDecryptFileLinkPathError::CorruptedData);
}

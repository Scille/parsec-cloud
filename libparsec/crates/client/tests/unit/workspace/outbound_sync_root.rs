// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::utils::workspace_ops_factory;

enum Modification {
    Nothing,
    Create,
    Remove,
    Rename,
    // A given entry name is overwritten by a new entry ID
    Replace,
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn non_placeholder(
    #[values(
        Modification::Nothing,
        Modification::Create,
        Modification::Remove,
        Modification::Rename,
        Modification::Replace
    )]
    modification: Modification,
    #[values(false, true)] remote_change: bool,
    env: &TestbedEnv,
) {
    // In case of no modification, the client won't query the server, hence the
    // remote modification is useless !
    if matches!(modification, Modification::Nothing) && remote_change {
        // Meaningless case, just skip it
        return;
    }

    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    // 1) Customize testbed

    let env = env.customize(|builder| {
        builder.new_device("alice"); // alice@dev2
        builder.certificates_storage_fetch_certificates("alice@dev1");

        match modification {
            Modification::Nothing => (),
            Modification::Create => {
                let local_new_id = builder
                    .workspace_data_storage_local_file_manifest_create_or_update(
                        "alice@dev1",
                        wksp1_id,
                        None,
                    )
                    .map(|e| e.local_manifest.base.id);
                builder.store_stuff("wksp1_local_new_id", &local_new_id);
                builder
                    .workspace_data_storage_local_workspace_manifest_update("alice@dev1", wksp1_id)
                    .customize_children([("new", Some(local_new_id))].into_iter());
            }
            Modification::Remove => {
                builder
                    .workspace_data_storage_local_workspace_manifest_update("alice@dev1", wksp1_id)
                    .customize_children([("foo", None)].into_iter());
            }
            Modification::Rename => {
                builder
                    .workspace_data_storage_local_workspace_manifest_update("alice@dev1", wksp1_id)
                    .customize_children(
                        [("foo", None), ("foo_renamed", Some(wksp1_foo_id))].into_iter(),
                    );
            }
            Modification::Replace => {
                let local_foo_replaced_id = builder
                    .workspace_data_storage_local_file_manifest_create_or_update(
                        "alice@dev1",
                        wksp1_id,
                        None,
                    )
                    .map(|e| e.local_manifest.base.id);
                builder.store_stuff("wksp1_local_foo_replaced_id", &local_foo_replaced_id);
                builder
                    .workspace_data_storage_local_workspace_manifest_update("alice@dev1", wksp1_id)
                    .customize_children([("foo", Some(local_foo_replaced_id))].into_iter());
            }
        }

        if remote_change {
            let dont_mind_me_id = builder
                .create_or_update_folder_manifest_vlob("alice@dev2", wksp1_id, None)
                .map(|e| e.manifest.id);
            builder.store_stuff("wksp1_dont_mind_me_id", &dont_mind_me_id);
            builder
                .create_or_update_workspace_manifest_vlob("alice@dev2", wksp1_id)
                .customize_children([("dont_mind_me", Some(dont_mind_me_id))].into_iter());
        }
    });

    // Get back last workspace manifest version synced in server
    let (wksp1_last_remote_manifest, wksp1_last_encrypted) = env
        .template
        .events
        .iter()
        .rev()
        .find_map(|e| match e {
            TestbedEvent::CreateOrUpdateWorkspaceManifestVlob(e) if e.manifest.id == wksp1_id => {
                Some((e.manifest.clone(), e.encrypted(&env.template)))
            }
            _ => None,
        })
        .unwrap();

    // 2) Start workspace ops

    let alice = env.local_device("alice@dev1");
    let wksp1_ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    // 3) Actual sync operation

    match (&modification, remote_change) {
        // No modification means no need to go disturb the server !
        (Modification::Nothing, _) => (),

        // Simple case: modification only on client side
        (_, false) => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                // 1) Fetch last workspace keys bundle to encrypt the new manifest
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
                // 2) `vlob_update` succeed on first try !
                move |req: authenticated_cmds::latest::vlob_update::Req| {
                    p_assert_eq!(req.key_index, 1);
                    p_assert_eq!(req.vlob_id, wksp1_id);
                    p_assert_eq!(req.version, 2);
                    assert!(req.sequester_blob.is_none());
                    authenticated_cmds::latest::vlob_update::Rep::Ok {}
                },
            );
        }

        // Modification on both client and server, a merge and retry will be needed
        (_, true) => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                // 1) Fetch last workspace keys bundle to encrypt the new manifest
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
                // 2) Fail to `vlob_create` due to new remote version
                move |req: authenticated_cmds::latest::vlob_update::Req| {
                    p_assert_eq!(req.key_index, 1);
                    p_assert_eq!(req.vlob_id, wksp1_id);
                    assert!(req.sequester_blob.is_none());
                    authenticated_cmds::latest::vlob_update::Rep::BadVlobVersion {}
                },
                // 3) `vlob_get` to fetch the remote change
                {
                    let wksp1_last_remote_manifest = wksp1_last_remote_manifest.clone();
                    let last_realm_certificate_timestamp =
                        env.get_last_realm_certificate_timestamp(wksp1_id);
                    let last_common_certificate_timestamp =
                        env.get_last_common_certificate_timestamp();
                    move |req: authenticated_cmds::latest::vlob_read_batch::Req| {
                        p_assert_eq!(req.at, None);
                        p_assert_eq!(req.realm_id, wksp1_id);
                        p_assert_eq!(req.vlobs, [wksp1_id]);
                        authenticated_cmds::latest::vlob_read_batch::Rep::Ok {
                            items: vec![(
                                wksp1_id,
                                1,
                                wksp1_last_remote_manifest.author.clone(),
                                wksp1_last_remote_manifest.version,
                                wksp1_last_remote_manifest.timestamp,
                                wksp1_last_encrypted,
                            )],
                            needed_common_certificate_timestamp: last_common_certificate_timestamp,
                            needed_realm_certificate_timestamp: last_realm_certificate_timestamp,
                        }
                    }
                },
                // 4) `vlob_update` again to upload the merged version
                move |req: authenticated_cmds::latest::vlob_update::Req| {
                    p_assert_eq!(req.key_index, 1);
                    p_assert_eq!(req.vlob_id, wksp1_id);
                    assert!(req.sequester_blob.is_none());
                    authenticated_cmds::latest::vlob_update::Rep::Ok {}
                },
            );
        }
    }

    wksp1_ops.outbound_sync(wksp1_id).await.unwrap();

    // 4) Check the outcome

    let (expected_base_version, expected_children) = {
        let mut children = vec![];
        let mut base_version = 1;

        let get_id = |raw| *env.template.get_stuff(raw);

        match &modification {
            Modification::Nothing => {
                children.push(("bar.txt", get_id("wksp1_bar_txt_id")));
                children.push(("foo", get_id("wksp1_foo_id")));
            }
            Modification::Create => {
                base_version += 1;
                children.push(("bar.txt", get_id("wksp1_bar_txt_id")));
                children.push(("foo", get_id("wksp1_foo_id")));
                children.push(("new", get_id("wksp1_local_new_id")));
            }
            Modification::Remove => {
                base_version += 1;
                children.push(("bar.txt", get_id("wksp1_bar_txt_id")));
            }
            Modification::Rename => {
                base_version += 1;
                children.push(("bar.txt", get_id("wksp1_bar_txt_id")));
                children.push(("foo_renamed", get_id("wksp1_foo_id")));
            }
            Modification::Replace => {
                base_version += 1;
                children.push(("bar.txt", get_id("wksp1_bar_txt_id")));
                children.push(("foo", get_id("wksp1_local_foo_replaced_id")));
            }
        }
        if remote_change {
            base_version += 1;
            children.push(("dont_mind_me", get_id("wksp1_dont_mind_me_id")));
        }
        (
            base_version,
            children
                .into_iter()
                .map(|(k, v)| (k.parse().unwrap(), v))
                .collect::<std::collections::HashMap<EntryName, VlobID>>(),
        )
    };

    // Check the user manifest is not longer need sync
    let workspace_manifest = wksp1_ops.data_storage.get_workspace_manifest();
    p_assert_eq!(workspace_manifest.need_sync, false);
    p_assert_eq!(workspace_manifest.speculative, false);
    p_assert_eq!(workspace_manifest.children, expected_children);
    p_assert_eq!(workspace_manifest.base.children, expected_children);
    p_assert_eq!(workspace_manifest.base.version, expected_base_version);

    wksp1_ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal")]
async fn placeholder(#[values(true, false)] is_speculative: bool, env: &TestbedEnv) {
    let (env, wksp1_id) = env.customize_with_map(|builder| {
        // Alice has access to a workspace partially bootstrapped: only the realm creation has been done
        let wksp1_id = builder.new_realm("alice").map(|e| e.realm_id);

        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder
            .user_storage_local_update("alice@dev1")
            .update_local_workspaces_with_fetched_certificates();
        builder.user_storage_fetch_realm_checkpoint("alice@dev1");

        // new_realm_event.then_add_workspace_entry_to_user_manifest_vlob();
        // builder.store_stuff("wksp1_id", &wksp1_id);
        // builder.store_stuff("wksp1_key", &wksp1_key);

        // builder.user_storage_fetch_user_vlob("alice@dev1");
        builder
            .workspace_data_storage_local_workspace_manifest_update("alice@dev1", wksp1_id)
            .customize(|e| {
                let manifest = std::sync::Arc::make_mut(&mut e.local_manifest);
                manifest.speculative = is_speculative;
            });

        wksp1_id
    });

    let alice = env.local_device("alice@dev1");
    let wksp1_ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    // Check the workspace manifest is need sync
    let workspace_manifest = wksp1_ops.data_storage.get_workspace_manifest();
    p_assert_eq!(workspace_manifest.need_sync, true);
    p_assert_eq!(workspace_manifest.speculative, is_speculative);
    p_assert_eq!(workspace_manifest.base.version, 0);
    assert!(workspace_manifest.children.is_empty());
    assert!(workspace_manifest.base.children.is_empty());

    let new_realm_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle: Arc<Mutex<Option<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle_access: Arc<Mutex<Option<Bytes>>> = Arc::default();
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 0) Workspace bootstrap: realm creation has already been done

        // 1) Workspace bootstrap: Initial key rotation
        {
            let alice = alice.clone();
            let new_realm_certificates = new_realm_certificates.clone();
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_access = new_realm_initial_keys_bundle_access.clone();
            move |req: authenticated_cmds::latest::realm_rotate_key::Req| {
                RealmKeyRotationCertificate::verify_and_load(
                    &req.realm_key_rotation_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
                    Some(wksp1_id),
                )
                .unwrap();
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_key_rotation_certificate);
                *new_realm_initial_keys_bundle.lock().unwrap() = Some(req.keys_bundle);
                let access = req
                    .per_participant_keys_bundle_access
                    .get(&"alice".parse().unwrap())
                    .unwrap()
                    .to_owned();
                *new_realm_initial_keys_bundle_access.lock().unwrap() = Some(access);
                authenticated_cmds::latest::realm_rotate_key::Rep::Ok
            }
        },
        // 2) Workspace bootstrap: Fetch new realm certificates (required for initial rename)
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
        // 3) Workspace bootstrap: Fetch keys bundle (required for initial rename)
        {
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_access = new_realm_initial_keys_bundle_access.clone();
            move |_req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle: new_realm_initial_keys_bundle
                        .lock()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .clone(),
                    keys_bundle_access: new_realm_initial_keys_bundle_access
                        .lock()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .clone(),
                }
            }
        },
        // 4) Workspace bootstrap: Actual rename
        {
            let alice = alice.clone();
            let new_realm_certificates = new_realm_certificates.clone();
            move |req: authenticated_cmds::latest::realm_rename::Req| {
                RealmNameCertificate::verify_and_load(
                    &req.realm_name_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
                    Some(wksp1_id),
                )
                .unwrap();
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_name_certificate);
                authenticated_cmds::latest::realm_rename::Rep::Ok
            }
        },
        // 5) `vlob_create`
        move |req: authenticated_cmds::latest::vlob_create::Req| {
            p_assert_eq!(req.realm_id, wksp1_id);
            p_assert_eq!(req.vlob_id, wksp1_id);
            assert!(req.sequester_blob.is_none());
            authenticated_cmds::latest::vlob_create::Rep::Ok {}
        },
    );

    wksp1_ops.outbound_sync(wksp1_id).await.unwrap();

    // Check the workspace manifest is no longer need sync
    let workspace_manifest = wksp1_ops.data_storage.get_workspace_manifest();
    p_assert_eq!(workspace_manifest.need_sync, false);
    p_assert_eq!(workspace_manifest.speculative, false);
    p_assert_eq!(workspace_manifest.base.version, 1);
    assert!(workspace_manifest.children.is_empty());
    assert!(workspace_manifest.base.children.is_empty());

    wksp1_ops.stop().await.unwrap();
}

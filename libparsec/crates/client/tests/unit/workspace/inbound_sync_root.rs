// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;

enum RemoteModification {
    Nothing,
    Create,
    Remove,
    Rename,
    // A given entry name is overwritten by a new entry ID
    Replace,
}

enum LocalModification {
    Nothing,
    NotConflicting,
    Conflicting,
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn non_placeholder(
    #[values(
        RemoteModification::Nothing,
        RemoteModification::Create,
        RemoteModification::Remove,
        RemoteModification::Rename,
        RemoteModification::Replace
    )]
    remote_modification: RemoteModification,
    #[values(
        LocalModification::Nothing,
        LocalModification::NotConflicting,
        LocalModification::Conflicting
    )]
    local_modification: LocalModification,
    env: &TestbedEnv,
) {
    if matches!(
        (&local_modification, &remote_modification),
        (LocalModification::Conflicting, RemoteModification::Nothing)
    ) {
        // Meaningless case, just skip it
        return;
    }

    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    // 1) Customize testbed

    let env = env.customize(|builder| {
        builder.new_device("alice"); // alice@dev2
        builder.certificates_storage_fetch_certificates("alice@dev1");

        match remote_modification {
            RemoteModification::Nothing => (),
            RemoteModification::Create => {
                let entry_id = builder
                    .create_or_update_folder_manifest_vlob("alice@dev2", wksp1_id, None)
                    .map(|e| e.manifest.id);
                builder.store_stuff("wksp1_new_id", &entry_id);
                builder
                    .create_or_update_workspace_manifest_vlob("alice@dev2", wksp1_id)
                    .customize_children([("new", Some(entry_id))].into_iter());
            }
            RemoteModification::Remove => {
                builder
                    .create_or_update_workspace_manifest_vlob("alice@dev2", wksp1_id)
                    .customize_children([("foo", None)].into_iter());
            }
            RemoteModification::Rename => {
                builder
                    .create_or_update_workspace_manifest_vlob("alice@dev2", wksp1_id)
                    .customize_children(
                        [("foo", None), ("foo_renamed", Some(wksp1_foo_id))].into_iter(),
                    );
            }
            RemoteModification::Replace => {
                let entry_id = builder
                    .create_or_update_folder_manifest_vlob("alice@dev2", wksp1_id, None)
                    .map(|e| e.manifest.id);
                builder.store_stuff("wksp1_foo_replaced_id", &entry_id);
                builder
                    .create_or_update_workspace_manifest_vlob("alice@dev2", wksp1_id)
                    .customize_children([("foo", Some(entry_id))].into_iter());
            }
        };

        match (&local_modification, &remote_modification) {
            (LocalModification::Nothing, _) => (),
            (LocalModification::NotConflicting, _) => {
                // Add a single file that is not referenced in the remote
                let id = builder
                    .workspace_data_storage_local_file_manifest_create_or_update(
                        "alice@dev1",
                        wksp1_id,
                        None,
                    )
                    .map(|e| e.local_manifest.base.id);
                builder.store_stuff("wksp1_local_dont_mind_me_txt_id", &id);
                builder
                    .workspace_data_storage_local_workspace_manifest_update("alice@dev1", wksp1_id)
                    .customize_children([("dont_mind_me.txt", Some(id))].into_iter());
            }
            (LocalModification::Conflicting, RemoteModification::Create) => {
                // Use the same entry name for a local change
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
            (LocalModification::Conflicting, RemoteModification::Remove) => {
                // The entry is rename in local and removed on remote !
                builder
                    .workspace_data_storage_local_workspace_manifest_update("alice@dev1", wksp1_id)
                    .customize_children(
                        [("foo", None), ("foo_renamed", Some(wksp1_foo_id))].into_iter(),
                    );
            }
            (LocalModification::Conflicting, RemoteModification::Rename) => {
                // Use the same entry name for a local change
                let local_foo_renamed_id = builder
                    .workspace_data_storage_local_file_manifest_create_or_update(
                        "alice@dev1",
                        wksp1_id,
                        None,
                    )
                    .map(|e| e.local_manifest.base.id);
                builder.store_stuff("wksp1_local_foo_renamed_id", &local_foo_renamed_id);
                builder
                    .workspace_data_storage_local_workspace_manifest_update("alice@dev1", wksp1_id)
                    .customize_children([("foo_renamed", Some(local_foo_renamed_id))].into_iter());
            }
            (LocalModification::Conflicting, RemoteModification::Replace) => {
                // Use the same entry name for a local change
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
            (LocalModification::Conflicting, RemoteModification::Nothing) => {
                unreachable!()
            }
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

    // Mock server command `vlob_read_batch` fetch the last version (i.e. v1 for
    // `RemoteModification::Nothing`, v2 else) of the workspace manifest

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Read the workspace manifest's vlob
        {
            let wksp1_last_remote_manifest = wksp1_last_remote_manifest.clone();
            let last_realm_certificate_timestamp =
                env.get_last_realm_certificate_timestamp(wksp1_id);
            let last_common_certificate_timestamp = env.get_last_common_certificate_timestamp();
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
        // 2) Fetch workspace keys bundle to decrypt the vlob
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

    wksp1_ops.inbound_sync(wksp1_id).await.unwrap();
    let workspace_manifest = wksp1_ops.data_storage.get_workspace_manifest();

    // 4) Check the outcome

    let expected_need_sync = !matches!(&local_modification, LocalModification::Nothing);
    let expected_children = {
        let mut children = vec![];

        let get_id = |raw| *env.template.get_stuff(raw);

        match &remote_modification {
            RemoteModification::Nothing => {
                children.push(("bar.txt", get_id("wksp1_bar_txt_id")));
                children.push(("foo", get_id("wksp1_foo_id")));
            }
            RemoteModification::Create => {
                children.push(("bar.txt", get_id("wksp1_bar_txt_id")));
                children.push(("foo", get_id("wksp1_foo_id")));
                children.push(("new", get_id("wksp1_new_id")));
            }
            RemoteModification::Remove => {
                children.push(("bar.txt", get_id("wksp1_bar_txt_id")));
            }
            RemoteModification::Rename => {
                children.push(("bar.txt", get_id("wksp1_bar_txt_id")));
                children.push(("foo_renamed", get_id("wksp1_foo_id")));
            }
            RemoteModification::Replace => {
                children.push(("bar.txt", get_id("wksp1_bar_txt_id")));
                children.push(("foo", get_id("wksp1_foo_replaced_id")));
            }
        }

        match (&local_modification, &remote_modification) {
            (LocalModification::Nothing, _) => {}
            (LocalModification::NotConflicting, _) => {
                children.push((
                    "dont_mind_me.txt",
                    get_id("wksp1_local_dont_mind_me_txt_id"),
                ));
            }

            (LocalModification::Conflicting, RemoteModification::Create) => {
                children.push(("new (Parsec - name conflict)", get_id("wksp1_local_new_id")));
            }
            (LocalModification::Conflicting, RemoteModification::Remove) => {
                children.push(("foo_renamed", get_id("wksp1_foo_id")));
            }
            (LocalModification::Conflicting, RemoteModification::Rename) => {
                children.push((
                    "foo_renamed (Parsec - name conflict)",
                    get_id("wksp1_local_foo_renamed_id"),
                ));
            }
            (LocalModification::Conflicting, RemoteModification::Replace) => {
                children.push((
                    "foo (Parsec - name conflict)",
                    get_id("wksp1_local_foo_replaced_id"),
                ));
            }

            (LocalModification::Conflicting, RemoteModification::Nothing) => unreachable!(),
        }

        children
            .into_iter()
            .map(|(k, v)| (k.parse().unwrap(), v))
            .collect::<std::collections::HashMap<EntryName, VlobID>>()
    };
    p_assert_eq!(workspace_manifest.speculative, false);
    p_assert_eq!(workspace_manifest.base, *wksp1_last_remote_manifest);
    p_assert_eq!(workspace_manifest.children, expected_children);
    p_assert_eq!(workspace_manifest.need_sync, expected_need_sync);

    wksp1_ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal")]
async fn placeholder(
    #[values(false, true)] is_speculative: bool,
    #[values(false, true)] local_change: bool,
    env: &TestbedEnv,
) {
    let (env, (wksp1_id, wksp1_foo_id)) = env.customize_with_map(|builder| {
        builder.new_device("alice"); // alice@dev2

        // Alice has access to a realm, alice@dev2 has synchronized the initial workspace
        // manifest while alice@dev1 is still using a placeholder.

        let wksp1_id = builder.new_realm("alice").map(|e| e.realm_id);
        builder.rotate_key_realm(wksp1_id);
        // We skip the initial rename part of the workspace bootstrap as it is not needed here
        builder.create_or_update_workspace_manifest_vlob("alice@dev2", wksp1_id);

        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder
            .user_storage_local_update("alice@dev1")
            .update_local_workspaces_with_fetched_certificates();
        builder.user_storage_fetch_realm_checkpoint("alice@dev1");
        // Note alice@dev1 doesn't fetch anything related to wksp1 here !

        let foo_id = builder
            .workspace_data_storage_local_file_manifest_create_or_update(
                "alice@dev1",
                wksp1_id,
                None,
            )
            .map(|e| e.local_manifest.base.id);
        builder
            .workspace_data_storage_local_workspace_manifest_update("alice@dev1", wksp1_id)
            .customize(|e| {
                let manifest = std::sync::Arc::make_mut(&mut e.local_manifest);
                manifest.speculative = is_speculative;
                if local_change {
                    manifest.need_sync = true;
                    manifest.children.insert("foo".parse().unwrap(), foo_id);
                }
            });

        (wksp1_id, foo_id)
    });

    let alice = env.local_device("alice@dev1");
    let wksp1_ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

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

    // Mock server command `vlob_read` fetch the last version
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Read the workspace manifest's vlob
        {
            let wksp1_last_remote_manifest = wksp1_last_remote_manifest.clone();
            let last_realm_certificate_timestamp =
                env.get_last_realm_certificate_timestamp(wksp1_id);
            let last_common_certificate_timestamp = env.get_last_common_certificate_timestamp();
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
        // 2) Fetch workspace keys bundle to decrypt the vlob
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

    wksp1_ops.inbound_sync(wksp1_id).await.unwrap();

    let workspace_manifest = wksp1_ops.data_storage.get_workspace_manifest();
    p_assert_eq!(workspace_manifest.speculative, false);
    p_assert_eq!(workspace_manifest.base, *wksp1_last_remote_manifest);
    if local_change {
        p_assert_eq!(workspace_manifest.need_sync, true);
        let mut expected_children = wksp1_last_remote_manifest.children.clone();
        expected_children.insert("foo".parse().unwrap(), wksp1_foo_id);
        p_assert_eq!(workspace_manifest.children, expected_children);
    } else {
        p_assert_eq!(workspace_manifest.need_sync, false);
        p_assert_eq!(
            workspace_manifest.children,
            wksp1_last_remote_manifest.children
        );
    }

    wksp1_ops.stop().await.unwrap();
}

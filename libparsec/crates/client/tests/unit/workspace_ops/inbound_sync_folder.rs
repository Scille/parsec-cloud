// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{protocol::authenticated_cmds, test_register_send_hook};
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
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let wksp1_key: &SecretKey = env.template.get_stuff("wksp1_key");

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
                builder.store_stuff("wksp1_foo_new_id", &entry_id);
                builder
                    .create_or_update_folder_manifest_vlob("alice@dev2", wksp1_id, wksp1_foo_id)
                    .customize_children([("new", Some(entry_id))].into_iter());
            }
            RemoteModification::Remove => {
                builder
                    .create_or_update_folder_manifest_vlob("alice@dev2", wksp1_id, wksp1_foo_id)
                    .customize_children([("spam", None)].into_iter());
            }
            RemoteModification::Rename => {
                builder
                    .create_or_update_folder_manifest_vlob("alice@dev2", wksp1_id, wksp1_foo_id)
                    .customize_children(
                        [("spam", None), ("spam_renamed", Some(wksp1_foo_spam_id))].into_iter(),
                    );
            }
            RemoteModification::Replace => {
                let entry_id = builder
                    .create_or_update_folder_manifest_vlob("alice@dev2", wksp1_id, None)
                    .map(|e| e.manifest.id);
                builder.store_stuff("wksp1_foo_spam_replaced_id", &entry_id);
                builder
                    .create_or_update_folder_manifest_vlob("alice@dev2", wksp1_id, wksp1_foo_id)
                    .customize_children([("spam", Some(entry_id))].into_iter());
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
                builder.store_stuff("wksp1_local_foo_dont_mind_me_txt_id", &id);
                builder
                    .workspace_data_storage_local_folder_manifest_create_or_update(
                        "alice@dev1",
                        wksp1_id,
                        wksp1_foo_id,
                    )
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
                builder.store_stuff("wksp1_local_foo_new_id", &local_new_id);
                builder
                    .workspace_data_storage_local_folder_manifest_create_or_update(
                        "alice@dev1",
                        wksp1_id,
                        wksp1_foo_id,
                    )
                    .customize_children([("new", Some(local_new_id))].into_iter());
            }
            (LocalModification::Conflicting, RemoteModification::Remove) => {
                // The entry is rename in local and removed on remote !
                builder
                    .workspace_data_storage_local_folder_manifest_create_or_update(
                        "alice@dev1",
                        wksp1_id,
                        wksp1_foo_id,
                    )
                    .customize_children(
                        [("spam", None), ("spam_renamed", Some(wksp1_foo_spam_id))].into_iter(),
                    );
            }
            (LocalModification::Conflicting, RemoteModification::Rename) => {
                // Use the same entry name for a local change
                let local_foo_spam_renamed_id = builder
                    .workspace_data_storage_local_file_manifest_create_or_update(
                        "alice@dev1",
                        wksp1_id,
                        None,
                    )
                    .map(|e| e.local_manifest.base.id);
                builder.store_stuff(
                    "wksp1_local_foo_spam_renamed_id",
                    &local_foo_spam_renamed_id,
                );
                builder
                    .workspace_data_storage_local_folder_manifest_create_or_update(
                        "alice@dev1",
                        wksp1_id,
                        wksp1_foo_id,
                    )
                    .customize_children(
                        [("spam_renamed", Some(local_foo_spam_renamed_id))].into_iter(),
                    );
            }
            (LocalModification::Conflicting, RemoteModification::Replace) => {
                // Use the same entry name for a local change
                let local_foo_spam_replaced_id = builder
                    .workspace_data_storage_local_file_manifest_create_or_update(
                        "alice@dev1",
                        wksp1_id,
                        None,
                    )
                    .map(|e| e.local_manifest.base.id);
                builder.store_stuff(
                    "wksp1_local_foo_spam_replaced_id",
                    &local_foo_spam_replaced_id,
                );
                builder
                    .workspace_data_storage_local_folder_manifest_create_or_update(
                        "alice@dev1",
                        wksp1_id,
                        wksp1_foo_id,
                    )
                    .customize_children([("spam", Some(local_foo_spam_replaced_id))].into_iter());
            }
            (LocalModification::Conflicting, RemoteModification::Nothing) => {
                unreachable!()
            }
        }
    });

    // Get back last workspace manifest version synced in server
    let (wksp1_foo_last_remote_manifest, wksp1_foo_last_encrypted) = env
        .template
        .events
        .iter()
        .rev()
        .find_map(|e| match e {
            TestbedEvent::CreateOrUpdateFolderManifestVlob(e) if e.manifest.id == wksp1_foo_id => {
                Some((e.manifest.clone(), e.encrypted(&env.template)))
            }
            _ => None,
        })
        .unwrap();

    // 2) Start workspace ops

    let alice = env.local_device("alice@dev1");
    let wksp1_ops = workspace_ops_factory(
        &env.discriminant_dir,
        &alice,
        wksp1_id,
        wksp1_key.to_owned(),
    )
    .await;

    // 3) Actual sync operation

    // Mock server command `vlob_read` fetch the last version (i.e. v1 for
    // `RemoteModification::Nothing`, v2 else)of the workspace manifest

    test_register_send_hook(&env.discriminant_dir, {
        let wksp1_foo_last_remote_manifest = wksp1_foo_last_remote_manifest.clone();
        let env = env.clone();
        move |req: authenticated_cmds::latest::vlob_read::Req| {
            p_assert_eq!(req.encryption_revision, 1);
            p_assert_eq!(req.vlob_id, wksp1_foo_id);
            p_assert_eq!(req.version, None);
            p_assert_eq!(req.timestamp, None);
            authenticated_cmds::latest::vlob_read::Rep::Ok {
                author: "alice@dev2".parse().unwrap(),
                certificate_index: env.get_last_certificate_index(),
                timestamp: wksp1_foo_last_remote_manifest.timestamp,
                version: wksp1_foo_last_remote_manifest.version,
                blob: wksp1_foo_last_encrypted,
            }
        }
    });

    wksp1_ops.inbound_sync(wksp1_foo_id).await.unwrap();
    let foo_manifest = match wksp1_ops
        .data_storage
        .get_child_manifest(wksp1_foo_id)
        .await
        .unwrap()
    {
        ArcLocalChildManifest::Folder(m) => m,
        m => panic!(
            "Invalid manifest type for `/foo`, expecting folder and got: {:?}",
            m
        ),
    };

    // 4) Check the outcome

    let expected_need_sync = !matches!(&local_modification, LocalModification::Nothing);
    let expected_children = {
        let mut children = vec![];

        let get_id = |raw| *env.template.get_stuff(raw);

        match &remote_modification {
            RemoteModification::Nothing => {
                children.push(("egg.txt", get_id("wksp1_foo_egg_txt_id")));
                children.push(("spam", get_id("wksp1_foo_spam_id")));
            }
            RemoteModification::Create => {
                children.push(("egg.txt", get_id("wksp1_foo_egg_txt_id")));
                children.push(("spam", get_id("wksp1_foo_spam_id")));
                children.push(("new", get_id("wksp1_foo_new_id")));
            }
            RemoteModification::Remove => {
                children.push(("egg.txt", get_id("wksp1_foo_egg_txt_id")));
            }
            RemoteModification::Rename => {
                children.push(("egg.txt", get_id("wksp1_foo_egg_txt_id")));
                children.push(("spam_renamed", get_id("wksp1_foo_spam_id")));
            }
            RemoteModification::Replace => {
                children.push(("egg.txt", get_id("wksp1_foo_egg_txt_id")));
                children.push(("spam", get_id("wksp1_foo_spam_replaced_id")));
            }
        }

        match (&local_modification, &remote_modification) {
            (LocalModification::Nothing, _) => {}
            (LocalModification::NotConflicting, _) => {
                children.push((
                    "dont_mind_me.txt",
                    get_id("wksp1_local_foo_dont_mind_me_txt_id"),
                ));
            }

            (LocalModification::Conflicting, RemoteModification::Create) => {
                children.push((
                    "new (Parsec - name conflict)",
                    get_id("wksp1_local_foo_new_id"),
                ));
            }
            (LocalModification::Conflicting, RemoteModification::Remove) => {
                children.push(("spam_renamed", get_id("wksp1_foo_spam_id")));
            }
            (LocalModification::Conflicting, RemoteModification::Rename) => {
                children.push((
                    "spam_renamed (Parsec - name conflict)",
                    get_id("wksp1_local_foo_spam_renamed_id"),
                ));
            }
            (LocalModification::Conflicting, RemoteModification::Replace) => {
                children.push((
                    "spam (Parsec - name conflict)",
                    get_id("wksp1_local_foo_spam_replaced_id"),
                ));
            }

            (LocalModification::Conflicting, RemoteModification::Nothing) => unreachable!(),
        }

        children
            .into_iter()
            .map(|(k, v)| (k.parse().unwrap(), v))
            .collect::<std::collections::HashMap<EntryName, VlobID>>()
    };
    p_assert_eq!(foo_manifest.base, *wksp1_foo_last_remote_manifest);
    p_assert_eq!(foo_manifest.children, expected_children);
    p_assert_eq!(foo_manifest.need_sync, expected_need_sync);

    wksp1_ops.stop().await.unwrap();
}

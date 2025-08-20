// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::workspace::{
    store::{PathConfinementPoint, RetrievePathFromIDEntry, RetrievePathFromIDError},
    MoveEntryMode,
};

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    // Resolve root
    p_assert_matches!(
        ops.store.retrieve_path_from_id(wksp1_id).await.unwrap(),
        RetrievePathFromIDEntry::Reachable {
            ref manifest,
            path,
            confinement_point,
            entry_chain
        } if confinement_point == PathConfinementPoint::NotConfined
            && path == "/".parse().unwrap()
            && matches!(manifest, ArcLocalChildManifest::Folder(manifest) if manifest.base.id == wksp1_id)
            && entry_chain == vec![wksp1_id]
    );

    // Resolve child folder
    p_assert_matches!(
        ops.store.retrieve_path_from_id(wksp1_foo_id).await.unwrap(),
        RetrievePathFromIDEntry::Reachable {
            ref manifest,
            path,
            confinement_point,
            entry_chain
    } if confinement_point == PathConfinementPoint::NotConfined
    && path == "/foo".parse().unwrap()
    && matches!(manifest, ArcLocalChildManifest::Folder(manifest) if manifest.base.id == wksp1_foo_id)
    && entry_chain == vec![wksp1_id, wksp1_foo_id]
    );

    // Resolve child file
    p_assert_matches!(
    ops.store.retrieve_path_from_id(wksp1_foo_egg_txt_id).await.unwrap(),
        RetrievePathFromIDEntry::Reachable {
            ref manifest,
            path,
            confinement_point,
            entry_chain
    } if confinement_point == PathConfinementPoint::NotConfined
    && path == "/foo/egg.txt".parse().unwrap()
    && matches!(manifest, ArcLocalChildManifest::File(manifest) if manifest.base.id == wksp1_foo_egg_txt_id)
    && entry_chain == vec![wksp1_id, wksp1_foo_id, wksp1_foo_egg_txt_id]
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_with_confinement(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;
    ops.rename_entry_by_id(
        wksp1_id,
        "foo".parse().unwrap(),
        "foo.tmp".parse().unwrap(),
        MoveEntryMode::NoReplace,
    )
    .await
    .unwrap();
    ops.rename_entry_by_id(
        wksp1_foo_id,
        "spam".parse().unwrap(),
        "spam.tmp".parse().unwrap(),
        MoveEntryMode::NoReplace,
    )
    .await
    .unwrap();
    let wksp1_foo_spam_egg_id = ops
        .create_file("/foo.tmp/spam.tmp/egg.txt".parse().unwrap())
        .await
        .unwrap();

    // Resolve root
    p_assert_matches!(
        ops.store.retrieve_path_from_id(wksp1_id).await.unwrap(),
        RetrievePathFromIDEntry::Reachable { ref manifest, path, confinement_point, entry_chain }
        if confinement_point == PathConfinementPoint::NotConfined
        && path == "/".parse().unwrap()
        && matches!(manifest, ArcLocalChildManifest::Folder(manifest) if manifest.base.id == wksp1_id)
        && entry_chain == vec![wksp1_id]
    );

    // Resolve foo folder
    p_assert_matches!(
        ops.store.retrieve_path_from_id(wksp1_foo_id).await.unwrap(),
        RetrievePathFromIDEntry::Reachable { ref manifest, path, confinement_point, entry_chain }
        if confinement_point == PathConfinementPoint::Confined(wksp1_id)
        && path == "/foo.tmp".parse().unwrap()
        && matches!(manifest, ArcLocalChildManifest::Folder(manifest) if manifest.base.id == wksp1_foo_id)
        && entry_chain == vec![wksp1_id, wksp1_foo_id]
    );

    // Resolve spam folder
    p_assert_matches!(
        ops.store.retrieve_path_from_id(wksp1_foo_spam_id).await.unwrap(),
        RetrievePathFromIDEntry::Reachable { ref manifest, path, confinement_point, entry_chain }
        if confinement_point == PathConfinementPoint::Confined(wksp1_id)
        && path == "/foo.tmp/spam.tmp".parse().unwrap()
        && matches!(manifest, ArcLocalChildManifest::Folder(manifest) if manifest.base.id == wksp1_foo_spam_id)
        && entry_chain == vec![wksp1_id, wksp1_foo_id, wksp1_foo_spam_id]
    );

    // Resolve egg file
    p_assert_matches!(
        ops.store.retrieve_path_from_id(wksp1_foo_spam_egg_id).await.unwrap(),
        RetrievePathFromIDEntry::Reachable { ref manifest, path, confinement_point, entry_chain }
        if confinement_point == PathConfinementPoint::Confined(wksp1_id)
        && path == "/foo.tmp/spam.tmp/egg.txt".parse().unwrap()
        && matches!(manifest, ArcLocalChildManifest::File(manifest) if manifest.base.id == wksp1_foo_spam_egg_id)
        && entry_chain == vec![wksp1_id, wksp1_foo_id, wksp1_foo_spam_id, wksp1_foo_spam_egg_id]
    );
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn inconsistent_path_unknown_child_id(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let unknown_child_id = VlobID::default();

    env.customize(|builder| {
        builder
            .workspace_data_storage_local_folder_manifest_create_or_update(
                "alice@dev1",
                wksp1_id,
                wksp1_foo_id,
                None,
            )
            .customize(|x| {
                let manifest = Arc::make_mut(&mut x.local_manifest);
                manifest
                    .children
                    .insert("unknown.txt".parse().unwrap(), unknown_child_id);
            });
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    p_assert_matches!(
        ops.store
            .retrieve_path_from_id(unknown_child_id)
            .await
            .unwrap(),
        RetrievePathFromIDEntry::Missing
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn base_path_mismatch_is_ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    env.customize(|builder| {
        let patched_parent_id = wksp1_foo_id;
        builder
            .workspace_data_storage_local_file_manifest_create_or_update(
                "alice@dev1",
                wksp1_id,
                wksp1_bar_txt_id,
                None,
            )
            .customize(|x| {
                let manifest = Arc::make_mut(&mut x.local_manifest);
                manifest.base.parent = patched_parent_id;
            });
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    p_assert_matches!(
        ops.store.retrieve_path_from_id(wksp1_bar_txt_id).await.unwrap(),
        RetrievePathFromIDEntry::Reachable { ref manifest, path, confinement_point, entry_chain }
        if confinement_point == PathConfinementPoint::NotConfined
        && path == "/bar.txt".parse().unwrap()
        && matches!(manifest, ArcLocalChildManifest::File(manifest) if manifest.base.id == wksp1_bar_txt_id)
        && entry_chain == vec![wksp1_id, wksp1_bar_txt_id]
    );
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn inconsistent_path_parent_mismatch_other_entry(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    env.customize(|builder| {
        builder
            .workspace_data_storage_local_folder_manifest_create_or_update(
                "alice@dev1",
                wksp1_id,
                wksp1_foo_id,
                None,
            )
            .customize(|x| {
                let manifest = Arc::make_mut(&mut x.local_manifest);
                manifest.parent = wksp1_bar_txt_id;
            });
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    p_assert_matches!(
        ops.store.retrieve_path_from_id(wksp1_foo_id).await.unwrap(),
        RetrievePathFromIDEntry::Unreachable { ref manifest }
        if matches!(manifest, ArcLocalChildManifest::Folder(manifest) if manifest.base.id == wksp1_foo_id)
    );
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn inconsistent_path_parent_mismatch_self_referencing(env: &TestbedEnv) {
    // Self-referencing parent is not allowed on a child folder manifest, this
    // is checked at deserialization time so our local storage should never
    // contain such a thing.
    // It would be acceptable to omit this test considering it represent an invalid
    // internal state for the local storage, however the author was full of joy
    // and motivation so here we are ;-)

    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    env.customize(|builder| {
        builder
            .workspace_data_storage_local_folder_manifest_create_or_update(
                "alice@dev1",
                wksp1_id,
                wksp1_foo_id,
                None,
            )
            .customize(|x| {
                let manifest = Arc::make_mut(&mut x.local_manifest);
                manifest.parent = wksp1_foo_id;
            });
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let err = ops
        .store
        .retrieve_path_from_id(wksp1_foo_id)
        .await
        .unwrap_err();

    let expected_err = DataError::DataIntegrity {
        data_type: "libparsec_types::local_manifest::folder::LocalFolderManifest",
        invariant: "id and parent are different for child manifest",
    };
    p_assert_matches!(err, RetrievePathFromIDError::Internal(anyhow_err) if anyhow_err.root_cause().to_string().contains(&expected_err.to_string()))
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn inconsistent_path_recursive_by_children(
    #[values("root", "child")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let (recursive_target_id, expected_path, expected_entry_chain) = match kind {
        "root" => (wksp1_id, "/".parse().unwrap(), vec![wksp1_id]),
        "child" => (
            wksp1_foo_id,
            "/foo".parse().unwrap(),
            vec![wksp1_id, wksp1_foo_id],
        ),
        unknown => panic!("Unknown kind: {unknown}"),
    };

    env.customize(|builder| {
        builder
            .workspace_data_storage_local_folder_manifest_create_or_update(
                "alice@dev1",
                wksp1_id,
                wksp1_foo_spam_id,
                None,
            )
            .customize(|x| {
                let manifest = Arc::make_mut(&mut x.local_manifest);
                manifest
                    .children
                    .insert("recursive".parse().unwrap(), recursive_target_id);
            });
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    p_assert_matches!(
        ops.store.retrieve_path_from_id(recursive_target_id).await.unwrap(),
        RetrievePathFromIDEntry::Reachable { ref manifest, path, confinement_point, entry_chain }
        if confinement_point == PathConfinementPoint::NotConfined
        && path == expected_path
        && matches!(manifest, ArcLocalChildManifest::Folder(manifest) if manifest.base.id == recursive_target_id)
        && entry_chain == expected_entry_chain
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn inconsistent_path_recursive_by_parent(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");

    env.customize(|builder| {
        builder
            .workspace_data_storage_local_folder_manifest_create_or_update(
                "alice@dev1",
                wksp1_id,
                wksp1_foo_id,
                None,
            )
            .customize(|x| {
                let manifest = Arc::make_mut(&mut x.local_manifest);
                manifest.parent = wksp1_foo_spam_id;
            });
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    p_assert_matches!(
        ops.store.retrieve_path_from_id(wksp1_foo_spam_id).await.unwrap(),
        RetrievePathFromIDEntry::Unreachable { ref manifest }
        if matches!(manifest, ArcLocalChildManifest::Folder(manifest) if manifest.base.id == wksp1_foo_spam_id)
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn inconsistent_path_recursive_by_parent_and_children(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");

    env.customize(|builder| {
        builder
            .workspace_data_storage_local_folder_manifest_create_or_update(
                "alice@dev1",
                wksp1_id,
                wksp1_foo_id,
                None,
            )
            .customize(|x| {
                let manifest = Arc::make_mut(&mut x.local_manifest);
                manifest.parent = wksp1_foo_spam_id;
            });
        builder
            .workspace_data_storage_local_folder_manifest_create_or_update(
                "alice@dev1",
                wksp1_id,
                wksp1_foo_spam_id,
                None,
            )
            .customize(|x| {
                let manifest = Arc::make_mut(&mut x.local_manifest);
                manifest
                    .children
                    .insert("recursive".parse().unwrap(), wksp1_foo_id);
            });
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    p_assert_matches!(
        ops.store.retrieve_path_from_id(wksp1_foo_spam_id).await.unwrap(),
        RetrievePathFromIDEntry::Unreachable { ref manifest }
        if matches!(manifest, ArcLocalChildManifest::Folder(manifest) if manifest.base.id == wksp1_foo_spam_id)
    );
}

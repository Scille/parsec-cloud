// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::workspace::{
    store::{PathConfinementPoint, ResolvePathError},
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
    let (manifest, confinement) = ops.store.resolve_path(&"/".parse().unwrap()).await.unwrap();
    p_assert_eq!(confinement, PathConfinementPoint::NotConfined);
    p_assert_matches!(manifest, ArcLocalChildManifest::Folder(m) if m.base.id == wksp1_id);

    // Resolve child folder
    let (manifest, confinement) = ops
        .store
        .resolve_path(&"/foo".parse().unwrap())
        .await
        .unwrap();
    p_assert_eq!(confinement, PathConfinementPoint::NotConfined);
    p_assert_matches!(manifest, ArcLocalChildManifest::Folder(m) if m.base.id == wksp1_foo_id);

    // Resolve child file
    let (manifest, confinement) = ops
        .store
        .resolve_path(&"/foo/egg.txt".parse().unwrap())
        .await
        .unwrap();
    p_assert_eq!(confinement, PathConfinementPoint::NotConfined);
    p_assert_matches!(manifest, ArcLocalChildManifest::File(m) if m.base.id == wksp1_foo_egg_txt_id);
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
    let (manifest, confinement) = ops.store.resolve_path(&"/".parse().unwrap()).await.unwrap();
    p_assert_eq!(confinement, PathConfinementPoint::NotConfined);
    p_assert_matches!(manifest, ArcLocalChildManifest::Folder(m) if m.base.id == wksp1_id);

    // Resolve foo folder
    let (manifest, confinement) = ops
        .store
        .resolve_path(&"/foo.tmp".parse().unwrap())
        .await
        .unwrap();
    p_assert_eq!(confinement, PathConfinementPoint::Confined(wksp1_id));
    p_assert_matches!(manifest, ArcLocalChildManifest::Folder(m) if m.base.id == wksp1_foo_id);

    // Resolve spam folder
    let (manifest, confinement) = ops
        .store
        .resolve_path(&"/foo.tmp/spam.tmp".parse().unwrap())
        .await
        .unwrap();
    p_assert_eq!(confinement, PathConfinementPoint::Confined(wksp1_id));
    p_assert_matches!(manifest, ArcLocalChildManifest::Folder(m) if m.base.id == wksp1_foo_spam_id);

    // Resolve egg file
    let (manifest, confinement) = ops
        .store
        .resolve_path(&"/foo.tmp/spam.tmp/egg.txt".parse().unwrap())
        .await
        .unwrap();
    p_assert_eq!(confinement, PathConfinementPoint::Confined(wksp1_id));
    p_assert_matches!(manifest, ArcLocalChildManifest::File(m) if m.base.id == wksp1_foo_spam_egg_id);
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn inconsistent_path_unknown_child_id(env: &TestbedEnv) {
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
                manifest
                    .children
                    .insert("unknown.txt".parse().unwrap(), VlobID::default());
            });
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let err = ops
        .store
        .resolve_path(&"/foo/unknown.txt".parse().unwrap())
        .await
        .unwrap_err();
    p_assert_matches!(err, ResolvePathError::EntryNotFound);
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

    let (manifest, confinement) = ops
        .store
        .resolve_path(&"/bar.txt".parse().unwrap())
        .await
        .unwrap();
    p_assert_eq!(confinement, PathConfinementPoint::NotConfined);
    p_assert_matches!(manifest, ArcLocalChildManifest::File(m) if m.base.id == wksp1_bar_txt_id);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn inconsistent_path_parent_mismatch(
    #[values("other_entry", "self_referencing")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let (patched_parent_id, expected_error) = match kind {
        "other_entry" => (wksp1_bar_txt_id, ResolvePathError::EntryNotFound),
        "self_referencing" => (
            wksp1_foo_id,
            ResolvePathError::Internal(
                DataError::DataIntegrity {
                    data_type: "libparsec_types::local_manifest::folder::LocalFolderManifest",
                    invariant: "id and parent are different for child manifest",
                }
                .into(),
            ),
        ),
        unknown => panic!("Unknown kind: {unknown}"),
    };
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
                manifest.parent = patched_parent_id;
            });
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let err = ops
        .store
        .resolve_path(&"/foo".parse().unwrap())
        .await
        .unwrap_err();

    if let ResolvePathError::Internal(x) = &expected_error {
        println!("Expected error: {:?}", x.to_string());
    }
    match expected_error {
        ResolvePathError::EntryNotFound => p_assert_matches!(err, ResolvePathError::EntryNotFound),
        ResolvePathError::Internal(expected) => {
            p_assert_matches!(err, ResolvePathError::Internal(anyhow_err) if anyhow_err.root_cause().to_string().contains(&expected.to_string()))
        }
        _ => unreachable!(),
    }
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn inconsistent_path_recursive(#[values("root", "child")] kind: &str, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    env.customize(|builder| {
        let recursive_target_id = match kind {
            "root" => wksp1_id,
            "child" => wksp1_foo_id,
            unknown => panic!("Unknown kind: {unknown}"),
        };
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

    let err = ops
        .store
        .resolve_path(&"/foo/spam/recursive".parse().unwrap())
        .await
        .unwrap_err();
    p_assert_matches!(err, ResolvePathError::EntryNotFound);
}

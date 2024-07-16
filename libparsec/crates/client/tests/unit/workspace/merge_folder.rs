// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::workspace::merge::{merge_local_folder_manifest, MergeLocalFolderManifestOutcome};

#[parsec_test(testbed = "minimal_client_ready")]
async fn no_remote_change(
    #[values("same_version", "older_version", "same_version_with_local_change")] kind: &str,
    env: &TestbedEnv,
) {
    let local_author = "alice@dev1".parse().unwrap();
    let timestamp = "2021-01-10T00:00:00Z".parse().unwrap();
    let vlob_id = VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap();
    let parent_id = VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap();

    let mut remote = FolderManifest {
        author: "bob@dev1".parse().unwrap(),
        timestamp: "2021-01-03T00:00:00Z".parse().unwrap(),
        id: vlob_id,
        parent: parent_id,
        version: 2,
        created: "2021-01-01T00:00:00Z".parse().unwrap(),
        updated: "2021-01-02T00:00:00Z".parse().unwrap(),
        children: HashMap::new(),
    };
    let mut local = LocalFolderManifest {
        base: remote.clone(),
        parent: parent_id,
        need_sync: false,
        updated: "2021-01-02T00:00:00Z".parse().unwrap(),
        children: HashMap::new(),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };
    match kind {
        "same_version" => (),
        "older_version" => {
            remote.version = 1;
            // Changes in the remote are ignored since it's an old version
            remote.updated = "2021-01-01T00:00:00Z".parse().unwrap();
            remote
                .children
                .insert("child.txt".parse().unwrap(), VlobID::default());
        }
        "same_version_with_local_change" => {
            local.need_sync = true;
            local.updated = "2021-01-03T00:00:00Z".parse().unwrap();
            local
                .children
                .insert("child.txt".parse().unwrap(), VlobID::default());
        }
        unknown => panic!("Unknown kind: {}", unknown),
    }

    let outcome = merge_local_folder_manifest(
        local_author,
        timestamp,
        // TODO: Pass prevent sync pattern
        None,
        &local,
        remote,
    );
    p_assert_eq!(outcome, MergeLocalFolderManifestOutcome::NoChange);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn remote_only_change(env: &TestbedEnv) {
    let local_author = "alice@dev1".parse().unwrap();
    let timestamp = "2021-01-10T00:00:00Z".parse().unwrap();
    let vlob_id = VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap();
    let parent_id = VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap();

    let mut remote = FolderManifest {
        author: "bob@dev1".parse().unwrap(),
        timestamp: "2021-01-03T00:00:00Z".parse().unwrap(),
        id: vlob_id,
        parent: parent_id,
        version: 1,
        created: "2021-01-01T00:00:00Z".parse().unwrap(),
        updated: "2021-01-02T00:00:00Z".parse().unwrap(),
        children: HashMap::new(),
    };
    let local = LocalFolderManifest {
        base: remote.clone(),
        parent: parent_id,
        need_sync: false,
        updated: "2021-01-02T00:00:00Z".parse().unwrap(),
        children: HashMap::new(),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };

    let new_parent_id = VlobID::from_hex("b95472b9c6d9415fa65297835d1feca5").unwrap();
    let new_child_id = VlobID::from_hex("1040c4845fd1451b9c243c93991d9a5e").unwrap();
    remote.version = 2;
    remote.parent = new_parent_id;
    remote.updated = "2021-01-03T00:00:00Z".parse().unwrap();
    remote
        .children
        .insert("child.txt".parse().unwrap(), new_child_id);

    let expected = LocalFolderManifest {
        base: FolderManifest {
            author: "bob@dev1".parse().unwrap(),
            timestamp: "2021-01-03T00:00:00Z".parse().unwrap(),
            id: vlob_id,
            parent: new_parent_id,
            version: 2,
            created: "2021-01-01T00:00:00Z".parse().unwrap(),
            updated: "2021-01-03T00:00:00Z".parse().unwrap(),
            children: HashMap::from_iter([("child.txt".parse().unwrap(), new_child_id)]),
        },
        parent: new_parent_id,
        need_sync: false,
        updated: "2021-01-03T00:00:00Z".parse().unwrap(),
        children: HashMap::from_iter([("child.txt".parse().unwrap(), new_child_id)]),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };
    let outcome = merge_local_folder_manifest(
        local_author,
        timestamp,
        // TODO: Pass prevent sync pattern
        None,
        &local,
        remote,
    );
    p_assert_eq!(
        outcome,
        MergeLocalFolderManifestOutcome::Merged(Arc::new(expected))
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
#[case::update_field_only(|remote: &mut FolderManifest, local: &mut LocalFolderManifest, _: DateTime, _: DeviceID| {
    remote.version = 2;
    remote.updated = "2021-01-03T00:00:00Z".parse().unwrap();

    local.need_sync = true;
    local.updated = "2021-01-04T00:00:00Z".parse().unwrap();

    let mut expected = local.clone();
    expected.need_sync = false;
    expected.updated = remote.updated;
    expected.base = remote.clone();

    expected
})]
#[case::children_no_conflict(|remote: &mut FolderManifest, local: &mut LocalFolderManifest, timestamp: DateTime, _: DeviceID| {
    let new_remote_child_id = VlobID::from_hex("1040c4845fd1451b9c243c93991d9a5e").unwrap();
    let new_local_child_id = VlobID::from_hex("df2edbe0d1c647bf9cea980f58dac4dc").unwrap();

    remote.version = 2;
    remote.updated = "2021-01-03T00:00:00Z".parse().unwrap();
    remote
        .children
        .insert("remote_child.txt".parse().unwrap(), new_remote_child_id);

    local.need_sync = true;
    local.updated = "2021-01-04T00:00:00Z".parse().unwrap();
    local
        .children
        .insert("local_child.txt".parse().unwrap(), new_local_child_id);

    let mut expected = local.clone();
    expected.updated = timestamp;
    expected.base = remote.clone();
    expected
        .children
        .insert("remote_child.txt".parse().unwrap(), new_remote_child_id);
    expected
        .children
        .insert("local_child.txt".parse().unwrap(), new_local_child_id);

    expected
})]
#[case::children_same_id_and_name(|remote: &mut FolderManifest, local: &mut LocalFolderManifest, _: DateTime, _: DeviceID| {
    let new_child_id = VlobID::from_hex("1040c4845fd1451b9c243c93991d9a5e").unwrap();

    remote.version = 2;
    remote.updated = "2021-01-03T00:00:00Z".parse().unwrap();
    remote
        .children
        .insert("child.txt".parse().unwrap(), new_child_id);

    local.need_sync = true;
    local.updated = "2021-01-04T00:00:00Z".parse().unwrap();
    local
        .children
        .insert("child.txt".parse().unwrap(), new_child_id);

    let mut expected = local.clone();
    expected.need_sync = false;
    expected.updated = remote.updated;
    expected.base = remote.clone();
    expected
        .children
        .insert("child.txt".parse().unwrap(), new_child_id);

    expected
})]
#[case::children_conflict(|remote: &mut FolderManifest, local: &mut LocalFolderManifest, timestamp: DateTime, _: DeviceID| {
    let new_remote_child_id = VlobID::from_hex("1040c4845fd1451b9c243c93991d9a5e").unwrap();
    let new_local_child_id = VlobID::from_hex("df2edbe0d1c647bf9cea980f58dac4dc").unwrap();

    remote.version = 2;
    remote.updated = "2021-01-03T00:00:00Z".parse().unwrap();
    remote
        .children
        .insert("child.txt".parse().unwrap(), new_remote_child_id);

    local.need_sync = true;
    local.updated = "2021-01-04T00:00:00Z".parse().unwrap();
    local
        .children
        .insert("child.txt".parse().unwrap(), new_local_child_id);

    let mut expected = local.clone();
    expected.updated = timestamp;
    expected.base = remote.clone();
    expected
        .children
        .insert("child.txt".parse().unwrap(), new_remote_child_id);
    expected.children.insert(
        "child (Parsec - name conflict).txt".parse().unwrap(),
        new_local_child_id,
    );

    expected
})]
#[case::parent_modified_on_local(|remote: &mut FolderManifest, local: &mut LocalFolderManifest, timestamp: DateTime, _: DeviceID| {
    let new_parent_id = VlobID::from_hex("1040c4845fd1451b9c243c93991d9a5e").unwrap();

    remote.version = 2;
    remote.updated = "2021-01-03T00:00:00Z".parse().unwrap();

    local.need_sync = true;
    local.updated = "2021-01-04T00:00:00Z".parse().unwrap();
    local.parent = new_parent_id;

    let mut expected = local.clone();
    expected.updated = timestamp;
    expected.base = remote.clone();
    expected.parent = new_parent_id;

    expected
})]
#[case::parent_modified_on_remote(|remote: &mut FolderManifest, local: &mut LocalFolderManifest, _: DateTime, _: DeviceID| {
    let new_parent_id = VlobID::from_hex("1040c4845fd1451b9c243c93991d9a5e").unwrap();

    remote.version = 2;
    remote.updated = "2021-01-03T00:00:00Z".parse().unwrap();
    remote.parent = new_parent_id;

    local.need_sync = true;
    local.updated = "2021-01-04T00:00:00Z".parse().unwrap();

    let mut expected = local.clone();
    expected.need_sync = false;
    expected.updated = remote.updated;
    expected.base = remote.clone();
    expected.parent = new_parent_id;

    expected
})]
#[case::parent_no_conflict(|remote: &mut FolderManifest, local: &mut LocalFolderManifest, _: DateTime, _: DeviceID| {
    let new_parent_id = VlobID::from_hex("1040c4845fd1451b9c243c93991d9a5e").unwrap();

    remote.version = 2;
    remote.updated = "2021-01-03T00:00:00Z".parse().unwrap();
    remote.parent = new_parent_id;

    local.need_sync = true;
    local.updated = "2021-01-04T00:00:00Z".parse().unwrap();
    local.parent = new_parent_id;

    let mut expected = local.clone();
    expected.need_sync = false;
    expected.base = remote.clone();
    expected.updated = remote.updated;
    expected.parent = new_parent_id;

    expected
})]
#[case::parent_conflict(|remote: &mut FolderManifest, local: &mut LocalFolderManifest, _: DateTime, _: DeviceID| {
    let new_remote_parent_id =
        VlobID::from_hex("1040c4845fd1451b9c243c93991d9a5e").unwrap();
    let new_local_parent_id = VlobID::from_hex("df2edbe0d1c647bf9cea980f58dac4dc").unwrap();

    remote.version = 2;
    remote.updated = "2021-01-03T00:00:00Z".parse().unwrap();
    remote.parent = new_remote_parent_id;

    local.need_sync = true;
    local.updated = "2021-01-04T00:00:00Z".parse().unwrap();
    local.parent = new_local_parent_id;

    let mut expected = local.clone();
    expected.need_sync = false;
    expected.base = remote.clone();
    // Parent conflict is simply resolved by siding with remote
    expected.updated = remote.updated;
    expected.parent = new_remote_parent_id;

    expected
})]
#[case::remote_parent_change_are_ours(|remote: &mut FolderManifest, local: &mut LocalFolderManifest, _: DateTime, local_author: DeviceID| {
    remote.author = local_author;

    let new_remote_parent_id =
        VlobID::from_hex("1040c4845fd1451b9c243c93991d9a5e").unwrap();
    let new_local_parent_id = VlobID::from_hex("df2edbe0d1c647bf9cea980f58dac4dc").unwrap();

    remote.version = 2;
    remote.updated = "2021-01-03T00:00:00Z".parse().unwrap();
    remote.parent = new_remote_parent_id;

    local.need_sync = true;
    local.updated = "2021-01-04T00:00:00Z".parse().unwrap();
    local.parent = new_local_parent_id;

    let mut expected = local.clone();
    expected.base = remote.clone();

    expected
})]
#[case::remote_children_changes_are_ours(|remote: &mut FolderManifest, local: &mut LocalFolderManifest, _: DateTime, local_author: DeviceID| {
    remote.author = local_author;

    let new_remote_child_id = VlobID::from_hex("1040c4845fd1451b9c243c93991d9a5e").unwrap();
    let new_local_child_id = VlobID::from_hex("df2edbe0d1c647bf9cea980f58dac4dc").unwrap();

    remote.version = 2;
    remote.updated = "2021-01-03T00:00:00Z".parse().unwrap();
    remote
        .children
        .insert("remote_child.txt".parse().unwrap(), new_remote_child_id);

    local.need_sync = true;
    local.updated = "2021-01-04T00:00:00Z".parse().unwrap();
    local
        .children
        .insert("local_child.txt".parse().unwrap(), new_local_child_id);

    let mut expected = local.clone();
    expected.base = remote.clone();

    expected
})]
#[case::remote_changes_are_ours_but_speculative(|remote: &mut FolderManifest, local: &mut LocalFolderManifest, timestamp: DateTime, local_author: DeviceID| {
    remote.author = local_author;
    local.speculative = true;

    let new_remote_child_id = VlobID::from_hex("1040c4845fd1451b9c243c93991d9a5e").unwrap();
    let new_local_child_id = VlobID::from_hex("df2edbe0d1c647bf9cea980f58dac4dc").unwrap();

    remote.version = 2;
    remote.updated = "2021-01-03T00:00:00Z".parse().unwrap();
    remote
        .children
        .insert("remote_child.txt".parse().unwrap(), new_remote_child_id);

    local.need_sync = true;
    local.speculative = true;
    local.updated = "2021-01-04T00:00:00Z".parse().unwrap();
    local
        .children
        .insert("local_child.txt".parse().unwrap(), new_local_child_id);

    let mut expected = local.clone();
    expected.updated = timestamp;
    expected.base = remote.clone();
    expected.speculative = false;
    // Given `local` was a speculative manifest, it is considered the client wasn't
    // aware of `remote`, and hence a regular merge is done instead of considering
    // `remote_child.txt` was willingly locally removed while uploading the remote.
    expected
        .children
        .insert("remote_child.txt".parse().unwrap(), new_remote_child_id);
    expected
        .children
        .insert("local_child.txt".parse().unwrap(), new_local_child_id);

    expected
})]
#[case::remote_with_confined_children(|remote: &mut FolderManifest, _: &mut LocalFolderManifest, _: DateTime, _: DeviceID| {
    remote.version = 2;
    remote.children.insert("a.txt".parse().unwrap(), VlobID::default());
    remote.children.insert("b.txt".parse().unwrap(), VlobID::default());
    let c_file_vid = VlobID::default();
    let c_file_entry: EntryName = "c.txt.tmp".parse().unwrap();
    remote.children.insert(c_file_entry.clone(), c_file_vid); // This one match the prevent sync pattern

    let mut expected = LocalFolderManifest::from_remote(remote.clone(), None);
    expected.children.remove(&c_file_entry);
    expected.remote_confinement_points.insert(c_file_vid);

    expected
})]
#[case::local_with_confined_children(|remote: &mut FolderManifest, local: &mut LocalFolderManifest, timestamp: DateTime, _: DeviceID| {
    let d_name: EntryName = "d.txt".parse().unwrap();
    let d_vid = VlobID::default();
    remote.children.insert(d_name.clone(), d_vid);
    remote.version = 2;

    local.need_sync = true;
    local.children.insert("a.txt".parse().unwrap(), VlobID::default());
    local.children.insert("b.txt".parse().unwrap(), VlobID::default());
    let c_file_vid = VlobID::default();
    let c_file_entry: EntryName = "c.txt.tmp".parse().unwrap();
    local.children.insert(c_file_entry.clone(), c_file_vid); // This one match the prevent sync pattern
    local.local_confinement_points.insert(c_file_vid);

    let mut expected = local.clone();
    expected.base = remote.clone();
    expected.children.insert(d_name, d_vid);
    expected.updated = timestamp;
    expected
})]
#[case::both_remote_local_with_confined_children(|remote: &mut FolderManifest, local: &mut LocalFolderManifest, timestamp: DateTime, _: DeviceID| {
    let d_name: EntryName = "d.txt".parse().unwrap();
    let d_vid = VlobID::default();
    remote.children.insert(d_name.clone(), d_vid);
    let e_name: EntryName = "e.txt.tmp".parse().unwrap();
    let e_vid = VlobID::default();
    remote.children.insert(e_name.clone(), e_vid); // This one match the version sync pattern
    remote.version = 2;

    local.need_sync = true;
    local.children.insert("a.txt".parse().unwrap(), VlobID::default());
    local.children.insert("b.txt".parse().unwrap(), VlobID::default());
    let c_file_vid = VlobID::default();
    let c_file_entry: EntryName = "c.txt.tmp".parse().unwrap();
    local.children.insert(c_file_entry.clone(), c_file_vid); // This one match the prevent sync pattern
    local.local_confinement_points.insert(c_file_vid);

    let mut expected = local.clone();
    expected.base = remote.clone();
    expected.children.insert(d_name, d_vid);
    expected.updated = timestamp;
    expected.remote_confinement_points.insert(e_vid);
    expected
})]
async fn local_and_remote_changes(
    #[case] prepare: impl FnOnce(
        &mut FolderManifest,
        &mut LocalFolderManifest,
        DateTime,
        DeviceID,
    ) -> LocalFolderManifest,
    env: &TestbedEnv,
) {
    let local_author: DeviceID = "alice@dev1".parse().unwrap();
    let timestamp = "2021-01-10T00:00:00Z".parse().unwrap();
    let vlob_id = VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap();
    let parent_id = VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap();
    let prevent_sync_pattern = Regex::from_glob_pattern("*.tmp").unwrap();

    let mut remote = FolderManifest {
        author: "bob@dev1".parse().unwrap(),
        timestamp: "2021-01-03T00:00:00Z".parse().unwrap(),
        id: vlob_id,
        parent: parent_id,
        version: 1,
        created: "2021-01-01T00:00:00Z".parse().unwrap(),
        updated: "2021-01-02T00:00:00Z".parse().unwrap(),
        children: HashMap::new(),
    };
    let mut local = LocalFolderManifest {
        base: remote.clone(),
        speculative: false,
        need_sync: false,
        updated: remote.updated,
        parent: remote.parent,
        children: remote.children.clone(),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
    };

    let expected = prepare(&mut remote, &mut local, timestamp, local_author);

    let outcome = merge_local_folder_manifest(
        local_author,
        timestamp,
        Some(&prevent_sync_pattern),
        &local,
        remote,
    );
    p_assert_eq!(
        outcome,
        MergeLocalFolderManifestOutcome::Merged(Arc::new(expected))
    );
}

// TODO: test prevent sync pattern !

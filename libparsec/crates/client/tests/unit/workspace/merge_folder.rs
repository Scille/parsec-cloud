// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::{HashMap, HashSet};

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::workspace::merge::merge_local_folder_manifests;

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

    let outcome = merge_local_folder_manifests(&local_author, timestamp, &local, remote);
    p_assert_eq!(outcome, None);
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
    let outcome = merge_local_folder_manifests(&local_author, timestamp, &local, remote);
    p_assert_eq!(outcome, Some(expected));
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn local_and_remote_changes(
    #[values(
        "updated_field_only",
        "children_no_conflict",
        "children_same_id_and_name",
        "children_conflict",
        "parent_modified_on_local",
        "parent_modified_on_remote",
        "parent_no_conflict",
        "parent_conflict",
        "remote_parent_change_are_ours",
        "remote_children_changes_are_ours",
        "remote_changes_are_ours_but_speculative"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let local_author: DeviceID = "alice@dev1".parse().unwrap();
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

    let expected = match kind {
        "updated_field_only" => {
            remote.version = 2;
            remote.updated = "2021-01-03T00:00:00Z".parse().unwrap();

            local.need_sync = true;
            local.updated = "2021-01-04T00:00:00Z".parse().unwrap();

            let mut expected = local.clone();
            expected.need_sync = false;
            expected.updated = remote.updated;
            expected.base = remote.clone();

            expected
        }

        "children_no_conflict" => {
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
        }

        "children_same_id_and_name" => {
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
        }

        "children_conflict" => {
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
        }

        "parent_modified_on_local" => {
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
        }

        "parent_modified_on_remote" => {
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
        }

        "parent_no_conflict" => {
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
        }

        "parent_conflict" => {
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
        }

        "remote_parent_change_are_ours" => {
            remote.author = local_author.clone();

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
        }

        "remote_children_changes_are_ours" => {
            remote.author = local_author.clone();

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
        }

        "remote_changes_are_ours_but_speculative" => {
            remote.author = local_author.clone();
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
        }

        unknown => panic!("Unknown kind: {}", unknown),
    };

    let outcome = merge_local_folder_manifests(&local_author, timestamp, &local, remote);
    p_assert_eq!(outcome, Some(expected));
}

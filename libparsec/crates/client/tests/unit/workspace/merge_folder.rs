// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::workspace::merge::{merge_local_folder_manifest, MergeLocalFolderManifestOutcome};

#[parsec_test(testbed = "minimal_client_ready")]
async fn no_remote_change(
    #[values(
        "same_version",
        "older_version",
        "same_version_with_local_change",
        "same_version_with_local_confinement",
        "same_version_with_remote_confinement"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let prevent_sync_pattern = PreventSyncPattern::from_glob("*.tmp").unwrap();
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
        "same_version_with_local_confinement" => {
            let child_id = VlobID::default();
            local
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            local.local_confinement_points.insert(child_id);
        }
        "same_version_with_remote_confinement" => {
            let child_id = VlobID::default();
            remote
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            local
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            local.remote_confinement_points.insert(child_id);
        }
        unknown => panic!("Unknown kind: {}", unknown),
    }

    let outcome = merge_local_folder_manifest(
        local_author,
        timestamp,
        &prevent_sync_pattern,
        &local,
        remote,
    );
    p_assert_eq!(outcome, MergeLocalFolderManifestOutcome::NoChange);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn no_remote_change_but_local_uses_outdated_prevent_sync_pattern(
    #[values(
        "local_entry_matching_outdated_pattern",
        "remote_entry_matching_outdated_pattern",
        "local_entry_matching_new_pattern",
        "remote_entry_matching_new_pattern"
    )]
    kind: &str,
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
        "local_entry_matching_outdated_pattern" => {
            // An entry is currently confined locally, but the prevent sync pattern
            // has changed so after the merge there should no longer be any confinement
            let child_id = VlobID::from_hex("9D1E5C787E014D5382B800A566CFA29D").unwrap();
            local
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);
            // Pretent the outdated prevent sync pattern is `.tmp~`
            local.local_confinement_points.insert(child_id);
        }
        "remote_entry_matching_outdated_pattern" => {
            // An entry is currently confined remotely, but the prevent sync pattern
            // has changed so after the merge there should no longer be any confinement
            let child_id = VlobID::from_hex("9D1E5C787E014D5382B800A566CFA29D").unwrap();
            remote
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);
            local
                .base
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);
            // Pretent the outdated prevent sync pattern is `.tmp~`
            local.remote_confinement_points.insert(child_id);
        }
        "local_entry_matching_new_pattern" => {
            let child_id = VlobID::from_hex("9D1E5C787E014D5382B800A566CFA29D").unwrap();
            local
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
        }
        "remote_entry_matching_new_pattern" => {
            let child_id = VlobID::from_hex("9D1E5C787E014D5382B800A566CFA29D").unwrap();
            remote
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            local
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
        }
        unknown => panic!("Unknown kind: {}", unknown),
    }

    let new_prevent_sync_pattern = PreventSyncPattern::from_glob("*.tmp").unwrap();
    let outcome = merge_local_folder_manifest(
        local_author,
        timestamp,
        &new_prevent_sync_pattern,
        &local,
        remote,
    );
    // Plot twist: no matter what, the merge algorithm should first detect no merge is needed !
    p_assert_eq!(outcome, MergeLocalFolderManifestOutcome::NoChange);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn remote_only_change(
    #[values(
        "only_updated_field_modified",
        "parent_field_modified",
        "new_entry_added",
        "new_entry_added_overwriting_existing_entry",
        "entry_renamed",
        "entry_renamed_overwriting_existing_entry",
        "entry_removed",
        "existing_confined_entry_then_new_non_confined_entry_added",
        "confined_entry_renamed_so_no_longer_confined",
        "confined_entry_renamed_but_still_confined",
        "confined_entry_removed",
        "new_confined_entry_added",
        "non_confined_entry_renamed_into_confined",
        "outdated_prevent_sync_pattern_non_confined_becomes_confined",
        "outdated_prevent_sync_pattern_confined_becomes_non_confined"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let local_author = "alice@dev1".parse().unwrap();
    let merge_timestamp = "2021-01-10T00:00:00Z".parse().unwrap();
    let vlob_id = VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap();
    let parent_id = VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap();

    // Start by creating `local` and `remote` manifests with minimal changes:
    // `remote` is just version n+1 with `updated` field set to a new timestamp.
    // Then this base will be customized in the following `match kind` statement.

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

    remote.version = 2;
    remote.updated = "2021-01-04T00:00:00Z".parse().unwrap();
    remote.timestamp = "2021-01-05T00:00:00Z".parse().unwrap();

    let mut expected = LocalFolderManifest {
        base: FolderManifest {
            author: "bob@dev1".parse().unwrap(),
            timestamp: "2021-01-05T00:00:00Z".parse().unwrap(),
            id: vlob_id,
            parent: parent_id,
            version: 2,
            created: "2021-01-01T00:00:00Z".parse().unwrap(),
            updated: "2021-01-04T00:00:00Z".parse().unwrap(),
            children: HashMap::new(), // Set in the match kind below
        },
        parent: parent_id,
        need_sync: false,
        updated: "2021-01-04T00:00:00Z".parse().unwrap(),
        children: HashMap::new(), // Set in the match kind below
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };

    let prevent_sync_pattern = PreventSyncPattern::from_glob("*.tmp").unwrap();
    let child_id = VlobID::from_hex("1040c4845fd1451b9c243c93991d9a5e").unwrap();
    let confined_id = VlobID::from_hex("9100fa0bfca94e4d96077dd274a243c0").unwrap();
    match kind {
        "only_updated_field_modified" => (),
        "parent_field_modified" => {
            let new_parent_id = VlobID::from_hex("b95472b9c6d9415fa65297835d1feca5").unwrap();
            remote.parent = new_parent_id;
            expected.base.parent = new_parent_id;
            expected.parent = new_parent_id;
        }
        // And entry is added in the remote
        "new_entry_added" => {
            remote
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            expected
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            expected
                .children
                .insert("child.txt".parse().unwrap(), child_id);
        }
        "new_entry_added_overwriting_existing_entry" => {
            let new_child_id = VlobID::from_hex("f023096c9b774a67bb6c35b82a4ed71f").unwrap();
            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            remote
                .children
                .insert("child.txt".parse().unwrap(), new_child_id);
            expected
                .base
                .children
                .insert("child.txt".parse().unwrap(), new_child_id);
            expected
                .children
                .insert("child.txt".parse().unwrap(), new_child_id);
        }
        // And entry is renamed in the remote
        "entry_renamed" => {
            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            local
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            remote
                .children
                .insert("child-renamed.txt".parse().unwrap(), child_id);
            expected
                .base
                .children
                .insert("child-renamed.txt".parse().unwrap(), child_id);
            expected
                .children
                .insert("child-renamed.txt".parse().unwrap(), child_id);
        }
        "entry_renamed_overwriting_existing_entry" => {
            let child2_id = VlobID::from_hex("f023096c9b774a67bb6c35b82a4ed71f").unwrap();
            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            local
                .base
                .children
                .insert("child2.txt".parse().unwrap(), child2_id);
            remote
                .children
                .insert("child.txt".parse().unwrap(), child2_id);
            expected
                .base
                .children
                .insert("child.txt".parse().unwrap(), child2_id);
            expected
                .children
                .insert("child.txt".parse().unwrap(), child2_id);
        }
        // And entry is removed in the remote
        "entry_removed" => {
            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            local
                .children
                .insert("child.txt".parse().unwrap(), child_id);
        }
        // A remote confined entry already exists in local, then the remote
        // manifest introduces a new unrelated entry.
        "existing_confined_entry_then_new_non_confined_entry_added" => {
            local
                .base
                .children
                .insert("confined.tmp".parse().unwrap(), confined_id);
            local.remote_confinement_points.insert(confined_id);
            remote
                .children
                .insert("confined.tmp".parse().unwrap(), confined_id);
            remote
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            expected
                .base
                .children
                .insert("confined.tmp".parse().unwrap(), confined_id);
            expected
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            expected.remote_confinement_points.insert(confined_id);
            expected
                .children
                .insert("child.txt".parse().unwrap(), child_id);
        }
        // A remote confined entry already exists in local, then the remote
        // rename this entry with a name not matching the prevent sync pattern
        "confined_entry_renamed_so_no_longer_confined" => {
            local
                .base
                .children
                .insert("confined.tmp".parse().unwrap(), confined_id);
            local.remote_confinement_points.insert(confined_id);
            remote
                .children
                .insert("confined-renamed.txt".parse().unwrap(), confined_id);
            expected
                .base
                .children
                .insert("confined-renamed.txt".parse().unwrap(), confined_id);
            expected
                .children
                .insert("confined-renamed.txt".parse().unwrap(), confined_id);
        }
        // A remote confined entry already exists in local, then the remote
        // renames this entry with a name still matching the prevent sync pattern
        "confined_entry_renamed_but_still_confined" => {
            local
                .base
                .children
                .insert("confined.tmp".parse().unwrap(), confined_id);
            local.remote_confinement_points.insert(confined_id);
            remote
                .children
                .insert("confined-renamed.tmp".parse().unwrap(), confined_id);
            expected
                .base
                .children
                .insert("confined-renamed.tmp".parse().unwrap(), confined_id);
            expected.remote_confinement_points.insert(confined_id);
        }
        // A remote confined entry already exists in local, then the remote
        // remove this entry
        "confined_entry_removed" => {
            local
                .base
                .children
                .insert("confined.tmp".parse().unwrap(), confined_id);
            local.remote_confinement_points.insert(confined_id);
        }
        // The remote manifest brings a new entry which name matches the prevent sync pattern
        "new_confined_entry_added" => {
            remote
                .children
                .insert("confined.tmp".parse().unwrap(), confined_id);
            expected
                .base
                .children
                .insert("confined.tmp".parse().unwrap(), confined_id);
            expected.remote_confinement_points.insert(confined_id);
        }
        // An entry already exists in local and is not confined, then the remote
        // renames this entry with a name matching the prevent sync pattern
        "non_confined_entry_renamed_into_confined" => {
            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            remote
                .children
                .insert("child-renamed.tmp".parse().unwrap(), child_id);
            expected
                .base
                .children
                .insert("child-renamed.tmp".parse().unwrap(), child_id);
            expected.remote_confinement_points.insert(child_id);
        }
        // The local manifest has it confinement points build with an outdated prevent
        // sync pattern (i.e. not `.tmp`), when the merge occurs with the remote manifest
        // the new prevent sync pattern is applied and an entry that was not confined
        // becomes confined.
        "outdated_prevent_sync_pattern_non_confined_becomes_confined" => {
            local
                .base
                .children
                // Not at this point `child.tmp` doesn't match the prevent sync pattern
                // used to build `local`, and hence is present among `local.children`
                .insert("child.tmp".parse().unwrap(), child_id);
            local
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            remote
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            expected
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            expected.remote_confinement_points.insert(child_id);
        }
        // The local manifest has it confinement points build with an outdated prevent
        // sync pattern (i.e. not `.tmp`), when the merge occurs with the remote manifest
        // the new prevent sync pattern is applied and an entry that was confined becomes
        // not confined.
        "outdated_prevent_sync_pattern_confined_becomes_non_confined" => {
            local
                .base
                .children
                // Not at this point `confined.tmp~` matches the prevent sync pattern
                // used to build `local`, and hence is not among `local.children`
                .insert("confined.tmp~".parse().unwrap(), confined_id);
            local.remote_confinement_points.insert(confined_id);
            remote
                .children
                .insert("confined.tmp~".parse().unwrap(), confined_id);
            expected
                .base
                .children
                .insert("confined.tmp~".parse().unwrap(), confined_id);
            expected
                .children
                .insert("confined.tmp~".parse().unwrap(), confined_id);
        }
        unknown => panic!("Unknown kind: {}", unknown),
    }

    let outcome = merge_local_folder_manifest(
        local_author,
        merge_timestamp,
        &prevent_sync_pattern,
        &local,
        remote,
    );
    p_assert_eq!(
        outcome,
        MergeLocalFolderManifestOutcome::Merged(Arc::new(expected))
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn local_and_remote_changes(
    #[values(
        // 1) Tests without confined entries

        "only_updated_field_modified",
        "parent_modified_in_remote_and_only_update_field_modified_in_local",
        "parent_modified_in_remote_and_unrelated_child_change_in_local",
        "parent_modified_in_local_and_only_update_field_modified_in_remote",
        "parent_modified_in_local_and_unrelated_child_change_in_remote",
        "parent_modified_in_both_with_different_value",
        "parent_modified_in_both_with_same_value",
        "parent_modified_in_both_with_unrelated_child_change_in_local",
        "parent_modified_in_both_with_remote_from_ourself",
        "children_modified_in_both_with_remote_from_ourself",
        // TODO: this test is flaky and often leads to an invalid entry name:
        // "child (Parsec - name conflict) (Parsec - name conflict).txt"
        // This is most likely due to an iteration on the children (given
        // hashmap iteration is not stable).
        // "conflicting_children_then_conflict_name_already_taken",
        "children_modified_in_local_or_remote",
        "child_added_in_both_with_same_id_and_name",
        "child_added_in_both_with_same_id_different_name",
        "children_added_in_both_with_same_name_different_id",
        "child_renamed_in_both_with_different_name",
        "child_renamed_in_both_with_same_name",
        "different_entries_renamed_into_same_name",
        "child_removed_in_both",
        "child_removed_in_local_and_renamed_in_remote",
        "child_removed_in_remote_and_renamed_in_local",
        "child_removed_in_local_and_name_taken_by_add_in_remote",
        "child_removed_in_remote_and_name_taken_by_add_in_local",
        "child_renamed_in_local_and_previous_name_taken_by_add_in_remote",
        "child_renamed_in_remote_and_previous_name_taken_by_add_in_local",
        "child_removed_in_local_and_name_taken_by_rename_in_remote",
        "child_removed_in_remote_and_name_taken_by_rename_in_local",
        "children_swapping_name_by_local_and_remote_renames",
        "speculative_local_with_no_modifications",
        "speculative_local_with_modifications",
        "speculative_local_with_child_added_in_remote",
        "speculative_local_with_children_modified_in_both_with_remote_from_ourself",

        // 2) Test with confined entries

        "local_confined_child_and_unrelated_remote_changes",
        "added_remote_confined_child_and_unrelated_local_changes",
        "existing_remote_confined_child_then_unrelated_remote_changes",
        "confined_child_added_in_both_with_same_name",
        "child_renamed_in_remote_becomes_confined",
        "child_renamed_in_both_becomes_confined",
        "child_already_renamed_into_confined_in_local",
        "child_renamed_in_local_becomes_confined_and_removed_in_remote",
        "child_renamed_in_remote_becomes_confined_and_removed_in_local",
        "child_renamed_in_local_becomes_confined_and_renamed_in_remote",
        "child_renamed_in_remote_becomes_confined_and_renamed_in_local",
        "confined_child_renamed_in_remote_still_confined",
        "confined_child_renamed_in_remote_becomes_non_confined",
        "confined_child_renamed_in_both_becomes_non_confined",
        "remote_confined_child_renamed_in_local_becomes_non_confined",
        "remote_confined_child_renamed_in_local_stays_confined",
        "remote_confined_child_removed_in_remote",
        "children_modified_in_both_with_confined_entries_and_remote_from_ourself",

        // 3) Test with outdated prevent sync pattern in local and confined entries
        // Outdated prevent sync pattern means the local manifest has been created
        // with a different prevent sync pattern (we use `.tmp~` here) than the
        // one that will be used in the merge (`.tmp` here).

        // Note we call it "psp" instead of "prevent_sync_pattern" to save some
        // space, otherwise some test names becomes similar in `cargo nextest`
        // given a test name is limited in size.

        "outdated_psp_local_child_becomes_non_confined",
        "outdated_psp_remote_child_becomes_non_confined",
        "outdated_psp_local_child_matches_new_pattern",
        "outdated_psp_remote_child_matches_new_pattern",
        "outdated_psp_remote_confined_entry_local_rename_then_remote_also_rename_with_confined_name",
        "outdated_psp_remote_confined_entry_local_rename_with_confined_name_then_remote_also_rename",
        "outdated_psp_remote_confined_entry_rename_in_both_with_confined_name",
        "outdated_psp_remote_child_becomes_non_confined_with_remote_from_ourself",
        "outdated_psp_local_child_becomes_non_confined_with_remote_from_ourself",
        "outdated_psp_remote_child_becomes_confined_with_remote_from_ourself",
        "outdated_psp_local_child_becomes_confined_with_remote_from_ourself",
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let local_author = "alice@dev1".parse().unwrap();
    let merge_timestamp = "2021-01-10T00:00:00Z".parse().unwrap();
    let vlob_id = VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap();
    let parent_id = VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap();

    // Start by creating `local` and `remote` manifests with minimal changes:
    // - `local` has just its `updated` field set to a new timestamp.
    // - `remote` is just version n+1 with `updated` field set to a new timestamp.
    // Then this base will be customized in the following `match kind` statement.

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
        need_sync: true,
        updated: "2021-01-10T00:00:00Z".parse().unwrap(),
        children: HashMap::new(),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };

    remote.version = 2;
    remote.timestamp = "2021-01-05T00:00:00Z".parse().unwrap();
    remote.updated = "2021-01-04T00:00:00Z".parse().unwrap();

    let mut expected = LocalFolderManifest {
        base: FolderManifest {
            author: "bob@dev1".parse().unwrap(),
            timestamp: "2021-01-05T00:00:00Z".parse().unwrap(),
            id: vlob_id,
            parent: parent_id,
            version: 2,
            created: "2021-01-01T00:00:00Z".parse().unwrap(),
            updated: "2021-01-04T00:00:00Z".parse().unwrap(),
            children: HashMap::new(), // Set in the match kind below
        },
        parent: parent_id,
        need_sync: true,
        updated: "2021-01-10T00:00:00Z".parse().unwrap(),
        children: HashMap::new(), // Set in the match kind below
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };

    let prevent_sync_pattern = PreventSyncPattern::from_glob("*.tmp").unwrap();
    match kind {
        "only_updated_field_modified" => {
            // Since only `updated` has been modified on local, then
            // it is overwritten by the remote value, then the merge
            // determine there is nothing more to sync here
            expected.updated = remote.updated;
            expected.need_sync = false;
        }
        "parent_modified_in_remote_and_only_update_field_modified_in_local" => {
            let new_parent_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            remote.parent = new_parent_id;

            expected.base.parent = new_parent_id;
            expected.parent = new_parent_id;
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "parent_modified_in_remote_and_unrelated_child_change_in_local" => {
            let new_parent_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            remote.parent = new_parent_id;
            // Also add an unrelated change on local side
            let child_a_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            local
                .children
                .insert("childA.txt".parse().unwrap(), child_a_id);

            expected.base.parent = new_parent_id;
            expected.parent = new_parent_id;
            expected
                .children
                .insert("childA.txt".parse().unwrap(), child_a_id);
        }
        "parent_modified_in_local_and_only_update_field_modified_in_remote" => {
            let new_parent_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            local.parent = new_parent_id;

            expected.parent = new_parent_id;
        }
        "parent_modified_in_local_and_unrelated_child_change_in_remote" => {
            let new_parent_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            local.parent = new_parent_id;
            // Also add an unrelated change on remote side
            let child1_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            remote
                .children
                .insert("child1.txt".parse().unwrap(), child1_id);

            expected.parent = new_parent_id;
            expected
                .children
                .insert("child1.txt".parse().unwrap(), child1_id);
            expected
                .base
                .children
                .insert("child1.txt".parse().unwrap(), child1_id);
        }
        "parent_modified_in_both_with_different_value" => {
            let new_local_parent_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let new_remote_parent_id =
                VlobID::from_hex("44a80da765bd41ed984fdee9e7b0fd0b").unwrap();
            local.parent = new_local_parent_id;
            remote.parent = new_remote_parent_id;

            // Parent conflict is simply resolved by siding with remote.
            // The only change in local was the re-parenting, which got overwritten.
            // So the local is no longer need sync now.
            expected.need_sync = false;
            expected.updated = remote.updated;
            expected.parent = new_remote_parent_id;
            expected.base.parent = new_remote_parent_id;
        }
        "parent_modified_in_both_with_same_value" => {
            let new_parent_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            local.parent = new_parent_id;
            remote.parent = new_parent_id;

            expected.need_sync = false;
            expected.updated = remote.updated;
            expected.parent = new_parent_id;
            expected.base.parent = new_parent_id;
        }
        "parent_modified_in_both_with_unrelated_child_change_in_local" => {
            let new_local_parent_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let new_remote_parent_id =
                VlobID::from_hex("44a80da765bd41ed984fdee9e7b0fd0b").unwrap();
            local.parent = new_local_parent_id;
            remote.parent = new_remote_parent_id;
            // Also add an unrelated change on local side
            let child_a_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            local
                .children
                .insert("childA.txt".parse().unwrap(), child_a_id);

            // The re-parent in local got overwritten, but there is still changes
            // in children that require sync.
            expected.parent = new_remote_parent_id;
            expected.base.parent = new_remote_parent_id;
            expected
                .children
                .insert("childA.txt".parse().unwrap(), child_a_id);
        }
        "parent_modified_in_both_with_remote_from_ourself" => {
            let new_local_parent_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let new_remote_parent_id =
                VlobID::from_hex("44a80da765bd41ed984fdee9e7b0fd0b").unwrap();
            local.parent = new_local_parent_id;
            remote.parent = new_remote_parent_id;
            remote.author = local_author;

            // Merge should detect the remote is from ourself, and hence the changes
            // in local are new ones that shouldn't be overwritten.
            expected.base.author = local_author;
            expected.parent = new_local_parent_id;
            expected.base.parent = new_remote_parent_id;
        }
        "children_modified_in_both_with_remote_from_ourself" => {
            let local_child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let remote_child_id = VlobID::from_hex("9a20331879744a149f55bc3ba16e8225").unwrap();

            // The same name is used in both remote and local to create a new entry,
            // this should lead to a conflict...
            remote
                .children
                .insert("child.txt".parse().unwrap(), remote_child_id);
            local
                .children
                .insert("child.txt".parse().unwrap(), local_child_id);
            // ...but the remote change are from ourself, hence instead of conflict
            // we should just acknowledge the remote and keep the local changes.
            remote.author = local_author;

            expected.base.author = local_author;
            expected
                .base
                .children
                .insert("child.txt".parse().unwrap(), remote_child_id);
            expected
                .children
                .insert("child.txt".parse().unwrap(), local_child_id);
        }
        "conflicting_children_then_conflict_name_already_taken" => {
            let local_child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let local_child2_id = VlobID::from_hex("87dfd188ff2f417da8417cafec9d10b5").unwrap();
            let remote_child_id = VlobID::from_hex("9a20331879744a149f55bc3ba16e8225").unwrap();

            // The same name is used in both remote and local to create a new entry,
            // this lead to a conflict...
            remote
                .children
                .insert("child.txt".parse().unwrap(), remote_child_id);
            local
                .children
                .insert("child.txt".parse().unwrap(), local_child_id);
            // ...conflict which is supposed to be resolved by renaming the local
            // entry, but the name normally used for this is already taken !
            local.children.insert(
                "child (Parsec - name conflict).txt".parse().unwrap(),
                local_child2_id,
            );

            expected
                .base
                .children
                .insert("child.txt".parse().unwrap(), remote_child_id);
            expected
                .children
                .insert("child.txt".parse().unwrap(), remote_child_id);
            expected.children.insert(
                "child (Parsec - name conflict).txt".parse().unwrap(),
                local_child2_id,
            );
            expected.children.insert(
                "child (Parsec - name conflict 2).txt".parse().unwrap(),
                local_child_id,
            );
        }
        "children_modified_in_local_or_remote" => {
            // This test contains all the possible entry modifications (add, rename, remove)
            // made in local or remote (in this test, no entry is modified by both local
            // and remote, hence no conflict is possible).
            //
            // Previous remote has 5 children: `child0.txt`, `child1.txt`, `child2.txt`, `child3.txt`, `child4.txt`
            // Local manifest has 3 local changes:
            // - `child1.txt` renamed into `child1-renamed.txt`
            // - `child2.txt` removed
            // - `childB.txt` which is a new entry
            // Then new remote has 3 changes:
            // - `child3.txt` renamed into `child3-renamed.txt`
            // - `child4.txt` removed
            // - a new `child5.txt`

            let child0_id = VlobID::from_hex("beb059a4121d4ee996fef65cda3667db").unwrap();
            let child1_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let child2_id = VlobID::from_hex("65277afff09548f885fa7ed7bd65de33").unwrap();
            let child3_id = VlobID::from_hex("2998bd200ebd4f0e87bd977b763459ca").unwrap();
            let child4_id = VlobID::from_hex("d3cf9aa5350f480f8e49a62aadc77027").unwrap();
            let child5_id = VlobID::from_hex("15a4186b8a6f4eeebc15067da5a6c0b6").unwrap();
            let child_b_id = VlobID::from_hex("9a20331879744a149f55bc3ba16e8225").unwrap();

            local
                .base
                .children
                .insert("child0.txt".parse().unwrap(), child0_id);
            local
                .base
                .children
                .insert("child1.txt".parse().unwrap(), child1_id);
            local
                .base
                .children
                .insert("child2.txt".parse().unwrap(), child2_id);
            local
                .base
                .children
                .insert("child3.txt".parse().unwrap(), child3_id);
            local
                .base
                .children
                .insert("child4.txt".parse().unwrap(), child4_id);
            local
                .children
                .insert("child0.txt".parse().unwrap(), child0_id);
            local
                .children
                .insert("child1-renamed.txt".parse().unwrap(), child1_id);
            local
                .children
                .insert("childB.txt".parse().unwrap(), child_b_id);
            local
                .children
                .insert("child3.txt".parse().unwrap(), child3_id);
            local
                .children
                .insert("child4.txt".parse().unwrap(), child4_id);

            remote
                .children
                .insert("child0.txt".parse().unwrap(), child0_id);
            remote
                .children
                .insert("child1.txt".parse().unwrap(), child1_id);
            remote
                .children
                .insert("child2.txt".parse().unwrap(), child2_id);
            remote
                .children
                .insert("child3-renamed.txt".parse().unwrap(), child3_id);
            remote
                .children
                .insert("child5.txt".parse().unwrap(), child5_id);

            expected
                .base
                .children
                .insert("child0.txt".parse().unwrap(), child0_id);
            expected
                .base
                .children
                .insert("child1.txt".parse().unwrap(), child1_id);
            expected
                .base
                .children
                .insert("child2.txt".parse().unwrap(), child2_id);
            expected
                .base
                .children
                .insert("child3-renamed.txt".parse().unwrap(), child3_id);
            expected
                .base
                .children
                .insert("child5.txt".parse().unwrap(), child5_id);
            expected
                .children
                .insert("child0.txt".parse().unwrap(), child0_id);
            expected
                .children
                .insert("child1-renamed.txt".parse().unwrap(), child1_id);
            expected
                .children
                .insert("childB.txt".parse().unwrap(), child_b_id);
            expected
                .children
                .insert("child3-renamed.txt".parse().unwrap(), child3_id);
            expected
                .children
                .insert("child5.txt".parse().unwrap(), child5_id);
        }
        "child_added_in_both_with_same_id_and_name" => {
            // Note this case is not supposed to happen in reality (as two separated
            // devices shouldn't be able to generate the same VlobID)
            let child_id = VlobID::from_hex("beb059a4121d4ee996fef65cda3667db").unwrap();
            local
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            remote
                .children
                .insert("child.txt".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            expected
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "child_added_in_both_with_same_id_different_name" => {
            // Note this case is not supposed to happen in reality (as two separated
            // devices shouldn't be able to generate the same VlobID)
            let child_id = VlobID::from_hex("beb059a4121d4ee996fef65cda3667db").unwrap();
            local
                .children
                .insert("childA.txt".parse().unwrap(), child_id);
            remote
                .children
                .insert("child1.txt".parse().unwrap(), child_id);

            // The merge algorithm simply choose to overwrite the local changes, this
            // is an acceptable outcome.
            // Also note that if the merge algorithm is modified and the new outcome
            // is to keep the local changes, this is also an acceptable (again we
            // are dealing with an exotic edge case here !).
            expected
                .base
                .children
                .insert("child1.txt".parse().unwrap(), child_id);
            expected
                .children
                .insert("child1.txt".parse().unwrap(), child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "children_added_in_both_with_same_name_different_id" => {
            let local_child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let remote_child_id = VlobID::from_hex("9a20331879744a149f55bc3ba16e8225").unwrap();

            remote
                .children
                .insert("child.txt".parse().unwrap(), remote_child_id);
            local
                .children
                .insert("child.txt".parse().unwrap(), local_child_id);

            expected
                .base
                .children
                .insert("child.txt".parse().unwrap(), remote_child_id);
            expected
                .children
                .insert("child.txt".parse().unwrap(), remote_child_id);
            expected.children.insert(
                "child (Parsec - name conflict).txt".parse().unwrap(),
                local_child_id,
            );
        }
        "child_renamed_in_both_with_different_name" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            local
                .children
                .insert("child-local-rename.txt".parse().unwrap(), child_id);
            remote
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);
            // Conflict is simply resolved by siding with remote.
            expected
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "child_renamed_in_both_with_same_name" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            local
                .children
                .insert("child-rename.txt".parse().unwrap(), child_id);
            remote
                .children
                .insert("child-rename.txt".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child-rename.txt".parse().unwrap(), child_id);
            expected
                .children
                .insert("child-rename.txt".parse().unwrap(), child_id);
            // Both local and remote agree on the change so there is no conflict
            // and no need for any sync !
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "different_entries_renamed_into_same_name" => {
            let child1_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let child2_id = VlobID::from_hex("f023096c9b774a67bb6c35b82a4ed71f").unwrap();

            // We start with two entries: child 1 & 2
            local
                .base
                .children
                .insert("child1.txt".parse().unwrap(), child1_id);
            local
                .base
                .children
                .insert("child2.txt".parse().unwrap(), child2_id);
            // Local renames child 1
            local
                .children
                .insert("child-rename.txt".parse().unwrap(), child1_id);
            local
                .children
                .insert("child2.txt".parse().unwrap(), child2_id);
            // Remote renames child 2
            remote
                .children
                .insert("child1.txt".parse().unwrap(), child1_id);
            remote
                .children
                .insert("child-rename.txt".parse().unwrap(), child2_id);

            expected
                .base
                .children
                .insert("child1.txt".parse().unwrap(), child1_id);
            expected
                .base
                .children
                .insert("child-rename.txt".parse().unwrap(), child2_id);
            expected.children.insert(
                "child-rename (Parsec - name conflict).txt".parse().unwrap(),
                child1_id,
            );
            expected
                .children
                .insert("child-rename.txt".parse().unwrap(), child2_id);
        }
        "child_removed_in_both" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);

            // Both local and remote agree on the change so there is no conflict
            // and no need for any sync !
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "child_removed_in_local_and_renamed_in_remote" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            remote
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);
            // Merge give priority to rename over remove
            expected
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "child_removed_in_remote_and_renamed_in_local" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            local
                .children
                .insert("child-local-rename.txt".parse().unwrap(), child_id);

            // Merge give priority to rename over remove
            expected
                .children
                .insert("child-local-rename.txt".parse().unwrap(), child_id);
        }
        "child_removed_in_local_and_name_taken_by_add_in_remote" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let new_child_id = VlobID::from_hex("d7c1206cf1eb4331a0508ee5687fd53a").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            remote
                .children
                .insert("child.txt".parse().unwrap(), new_child_id);

            expected
                .base
                .children
                .insert("child.txt".parse().unwrap(), new_child_id);
            expected
                .children
                .insert("child.txt".parse().unwrap(), new_child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "child_removed_in_remote_and_name_taken_by_add_in_local" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let new_child_id = VlobID::from_hex("d7c1206cf1eb4331a0508ee5687fd53a").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            local
                .children
                .insert("child.txt".parse().unwrap(), new_child_id);

            expected
                .children
                .insert("child.txt".parse().unwrap(), new_child_id);
        }
        "child_renamed_in_local_and_previous_name_taken_by_add_in_remote" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let new_child_id = VlobID::from_hex("d7c1206cf1eb4331a0508ee5687fd53a").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            local
                .children
                .insert("child-rename.txt".parse().unwrap(), child_id);
            remote
                .children
                .insert("child.txt".parse().unwrap(), new_child_id);

            expected
                .base
                .children
                .insert("child.txt".parse().unwrap(), new_child_id);
            expected
                .children
                .insert("child.txt".parse().unwrap(), new_child_id);
            expected
                .children
                .insert("child-rename.txt".parse().unwrap(), child_id);
        }
        "child_renamed_in_remote_and_previous_name_taken_by_add_in_local" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let new_child_id = VlobID::from_hex("d7c1206cf1eb4331a0508ee5687fd53a").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            local
                .children
                .insert("child.txt".parse().unwrap(), new_child_id);
            remote
                .children
                .insert("child-rename.txt".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child-rename.txt".parse().unwrap(), child_id);
            expected
                .children
                .insert("child-rename.txt".parse().unwrap(), child_id);
            expected
                .children
                .insert("child.txt".parse().unwrap(), new_child_id);
        }
        "child_removed_in_local_and_name_taken_by_rename_in_remote" => {
            let child1_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let child2_id = VlobID::from_hex("d7c1206cf1eb4331a0508ee5687fd53a").unwrap();

            local
                .base
                .children
                .insert("child1.txt".parse().unwrap(), child1_id);
            local
                .base
                .children
                .insert("child2.txt".parse().unwrap(), child2_id);
            local
                .children
                .insert("child2.txt".parse().unwrap(), child2_id);
            remote
                .children
                .insert("child1.txt".parse().unwrap(), child2_id);

            expected
                .base
                .children
                .insert("child1.txt".parse().unwrap(), child2_id);
            expected
                .children
                .insert("child1.txt".parse().unwrap(), child2_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "child_removed_in_remote_and_name_taken_by_rename_in_local" => {
            let child1_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let child2_id = VlobID::from_hex("d7c1206cf1eb4331a0508ee5687fd53a").unwrap();

            local
                .base
                .children
                .insert("child1.txt".parse().unwrap(), child1_id);
            local
                .base
                .children
                .insert("child2.txt".parse().unwrap(), child2_id);
            local
                .children
                .insert("child1.txt".parse().unwrap(), child2_id);
            remote
                .children
                .insert("child2.txt".parse().unwrap(), child2_id);

            expected
                .base
                .children
                .insert("child2.txt".parse().unwrap(), child2_id);
            expected
                .children
                .insert("child1.txt".parse().unwrap(), child2_id);
        }
        "children_swapping_name_by_local_and_remote_renames" => {
            let child1_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let child2_id = VlobID::from_hex("d7c1206cf1eb4331a0508ee5687fd53a").unwrap();

            local
                .base
                .children
                .insert("child1.txt".parse().unwrap(), child1_id);
            local
                .base
                .children
                .insert("child2.txt".parse().unwrap(), child2_id);
            local
                .children
                .insert("child1.txt".parse().unwrap(), child2_id);
            remote
                .children
                .insert("child2.txt".parse().unwrap(), child1_id);

            expected
                .base
                .children
                .insert("child2.txt".parse().unwrap(), child1_id);
            expected
                .children
                .insert("child1.txt".parse().unwrap(), child2_id);
            expected
                .children
                .insert("child2.txt".parse().unwrap(), child1_id);
        }
        "speculative_local_with_no_modifications" => {
            local = LocalFolderManifest::new_root(
                local_author,
                local.base.id,
                // timestamp is more recent than the remote but should get overwritten
                "2024-01-01T00:00:00Z".parse().unwrap(),
                true,
            );
            // Speculative manifest is only allowed for root manifest which must
            // have parent pointing on itself.
            remote.parent = local.base.id;
            expected.parent = local.base.id;
            expected.base.parent = local.base.id;
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "speculative_local_with_modifications" => {
            local = LocalFolderManifest::new_root(
                local_author,
                local.base.id,
                // timestamp is more recent than the remote but should get overwritten
                "2024-01-01T00:00:00Z".parse().unwrap(),
                true,
            );
            // Speculative manifest is only allowed for root manifest which must
            // have parent pointing on itself.
            remote.parent = local.base.id;
            expected.parent = local.base.id;
            expected.base.parent = local.base.id;

            // Add modification to local than should be preserved by the merge
            let child_b_id = VlobID::from_hex("9a20331879744a149f55bc3ba16e8225").unwrap();
            local
                .children
                .insert("childB.txt".parse().unwrap(), child_b_id);
            expected
                .children
                .insert("childB.txt".parse().unwrap(), child_b_id);
        }
        "speculative_local_with_child_added_in_remote" => {
            local = LocalFolderManifest::new_root(
                local_author,
                local.base.id,
                // timestamp is more recent than the remote but should get overwritten
                "2024-01-01T00:00:00Z".parse().unwrap(),
                true,
            );
            // Speculative manifest is only allowed for root manifest which must
            // have parent pointing on itself.
            remote.parent = local.base.id;
            expected.parent = local.base.id;
            expected.base.parent = local.base.id;

            // Add modification to remote than should be preserved by the merge,
            // this is a specific behavior for speculative manifest given in this
            // case we cannot assume missing entries in the speculative manifest
            // means it was known and has been removed (like the merge normally does).
            let child_id = VlobID::from_hex("9a20331879744a149f55bc3ba16e8225").unwrap();
            remote
                .children
                .insert("child.txt".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            expected
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "speculative_local_with_children_modified_in_both_with_remote_from_ourself" => {
            local = LocalFolderManifest::new_root(
                local_author,
                local.base.id,
                // timestamp is more recent than the remote but should get overwritten
                "2024-01-01T00:00:00Z".parse().unwrap(),
                true,
            );
            // Speculative manifest is only allowed for root manifest which must
            // have parent pointing on itself.
            remote.parent = local.base.id;
            expected.parent = local.base.id;
            expected.base.parent = local.base.id;

            let local_child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let remote_child_id = VlobID::from_hex("9a20331879744a149f55bc3ba16e8225").unwrap();

            // The same name is used in both remote and local to create a new entry,
            // this should lead to a conflict...
            remote
                .children
                .insert("child.txt".parse().unwrap(), remote_child_id);
            local
                .children
                .insert("child.txt".parse().unwrap(), local_child_id);
            // ...but the remote change are from ourself so instead the remote
            // manifest should just be acknowledge...
            // ...but but but ! Having a local speculative manifest means we
            // cannot assume missing entries in the speculative manifest means
            // it was known and has been removed (like the merge normally does).
            // So in the end we must do a merge between local and remote changes,
            // which leads to a conflict !
            remote.author = local_author;

            expected.base.author = local_author;
            expected
                .base
                .children
                .insert("child.txt".parse().unwrap(), remote_child_id);
            expected
                .children
                .insert("child.txt".parse().unwrap(), remote_child_id);
            expected.children.insert(
                "child (Parsec - name conflict).txt".parse().unwrap(),
                local_child_id,
            );
        }
        "local_confined_child_and_unrelated_remote_changes" => {
            let remote_child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let local_child_id = VlobID::from_hex("9a20331879744a149f55bc3ba16e8225").unwrap();

            local
                .children
                .insert("local_child.tmp".parse().unwrap(), local_child_id);
            local.local_confinement_points.insert(local_child_id);
            remote
                .children
                .insert("remote_child.txt".parse().unwrap(), remote_child_id);

            expected
                .base
                .children
                .insert("remote_child.txt".parse().unwrap(), remote_child_id);
            expected
                .children
                .insert("remote_child.txt".parse().unwrap(), remote_child_id);
            expected
                .children
                .insert("local_child.tmp".parse().unwrap(), local_child_id);
            expected.local_confinement_points.insert(local_child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "added_remote_confined_child_and_unrelated_local_changes" => {
            let remote_child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let local_child_id = VlobID::from_hex("9a20331879744a149f55bc3ba16e8225").unwrap();

            local
                .children
                .insert("local_child.txt".parse().unwrap(), local_child_id);
            remote
                .children
                .insert("remote_child.tmp".parse().unwrap(), remote_child_id);

            expected
                .base
                .children
                .insert("remote_child.tmp".parse().unwrap(), remote_child_id);
            expected
                .children
                .insert("local_child.txt".parse().unwrap(), local_child_id);
            expected.remote_confinement_points.insert(remote_child_id);
        }
        "existing_remote_confined_child_then_unrelated_remote_changes" => {
            let child1_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let child2_id = VlobID::from_hex("9a20331879744a149f55bc3ba16e8225").unwrap();

            local
                .base
                .children
                .insert("child1.tmp".parse().unwrap(), child1_id);
            local.remote_confinement_points.insert(child1_id);
            remote
                .children
                .insert("child1.tmp".parse().unwrap(), child1_id);
            remote
                .children
                .insert("child2.txt".parse().unwrap(), child2_id);

            expected
                .base
                .children
                .insert("child1.tmp".parse().unwrap(), child1_id);
            expected
                .base
                .children
                .insert("child2.txt".parse().unwrap(), child2_id);
            expected
                .children
                .insert("child2.txt".parse().unwrap(), child2_id);
            expected.remote_confinement_points.insert(child1_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "confined_child_added_in_both_with_same_name" => {
            let remote_child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let local_child_id = VlobID::from_hex("9a20331879744a149f55bc3ba16e8225").unwrap();

            local
                .children
                .insert("child.tmp".parse().unwrap(), local_child_id);
            local.local_confinement_points.insert(local_child_id);
            remote
                .children
                .insert("child.tmp".parse().unwrap(), remote_child_id);

            expected
                .base
                .children
                .insert("child.tmp".parse().unwrap(), remote_child_id);
            expected
                .children
                .insert("child.tmp".parse().unwrap(), local_child_id);
            expected.remote_confinement_points.insert(remote_child_id);
            expected.local_confinement_points.insert(local_child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "child_renamed_in_remote_becomes_confined" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            local
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            remote
                .children
                .insert("child.tmp".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            expected.remote_confinement_points.insert(child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "child_renamed_in_both_becomes_confined" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            local
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            local.local_confinement_points.insert(child_id);
            remote
                .children
                .insert("child.tmp".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            expected
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            expected.remote_confinement_points.insert(child_id);
            expected.local_confinement_points.insert(child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "child_already_renamed_into_confined_in_local" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            local
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            remote
                .children
                .insert("child.txt".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            expected
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            expected.local_confinement_points.insert(child_id);
            // We expect a need sync, because the renaming to a confined name
            // is analogous to a removal, and this removal should be synced
            expected.need_sync = true;
            expected.updated = merge_timestamp;
        }
        "child_renamed_in_local_becomes_confined_and_removed_in_remote" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            local
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            local.local_confinement_points.insert(child_id);

            expected
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            expected.local_confinement_points.insert(child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "child_renamed_in_remote_becomes_confined_and_removed_in_local" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            remote
                .children
                .insert("child.tmp".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            expected.remote_confinement_points.insert(child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "child_renamed_in_local_becomes_confined_and_renamed_in_remote" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            local
                .children
                .insert("child-local-rename.tmp".parse().unwrap(), child_id);
            local.local_confinement_points.insert(child_id);
            remote
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);
            // Renaming to a confined name is analogous to a removal, and removal
            // get priority over rename, so the remote rename is ignored.
            expected
                .children
                .insert("child-local-rename.tmp".parse().unwrap(), child_id);
            expected.local_confinement_points.insert(child_id);
            expected.need_sync = true;
            expected.updated = merge_timestamp;
        }
        "child_renamed_in_remote_becomes_confined_and_renamed_in_local" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.txt".parse().unwrap(), child_id);
            local
                .children
                .insert("child-local-rename.txt".parse().unwrap(), child_id);
            remote
                .children
                .insert("child-remote-rename.tmp".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child-remote-rename.tmp".parse().unwrap(), child_id);
            // Here the file is removed locally. This is good, because this id is
            // now remotely confined, so it would be weird to keep using it locally
            // under a non-confined name.
            expected.need_sync = false;
            expected.updated = remote.updated;
            expected.remote_confinement_points.insert(child_id);
        }
        "confined_child_renamed_in_remote_still_confined" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            local.remote_confinement_points.insert(child_id);
            remote
                .children
                .insert("child-remote-rename.tmp".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child-remote-rename.tmp".parse().unwrap(), child_id);
            expected.remote_confinement_points.insert(child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "confined_child_renamed_in_remote_becomes_non_confined" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            local.remote_confinement_points.insert(child_id);
            remote
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);
            expected
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "confined_child_renamed_in_both_becomes_non_confined" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            // This is a weird case given local is normally not able to rename
            // this child given it was confined (but this may occur if the entry
            // has not always been confined).
            local
                .children
                .insert("child-local-rename.txt".parse().unwrap(), child_id);
            local.remote_confinement_points.insert(child_id);
            remote
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);
            expected
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "remote_confined_child_renamed_in_local_becomes_non_confined" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            // This is a weird case given local is normally not able to rename
            // this child given it was confined (but this may occur if the entry
            // has not always been confined).
            local
                .children
                .insert("child-local-rename.txt".parse().unwrap(), child_id);
            local.remote_confinement_points.insert(child_id);
            remote
                .children
                .insert("child.tmp".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            expected
                .children
                .insert("child-local-rename.txt".parse().unwrap(), child_id);
            expected.remote_confinement_points.insert(child_id);
        }
        "remote_confined_child_renamed_in_local_stays_confined" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            // This is a weird case given local is normally not able to rename
            // this child given it was confined (but this may occur if the entry
            // has not always been confined).
            local
                .children
                .insert("child-local-rename.tmp".parse().unwrap(), child_id);
            local.local_confinement_points.insert(child_id);
            local.remote_confinement_points.insert(child_id);
            remote
                .children
                .insert("child.tmp".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            expected
                .children
                .insert("child-local-rename.tmp".parse().unwrap(), child_id);
            expected.local_confinement_points.insert(child_id);
            expected.remote_confinement_points.insert(child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "remote_confined_child_removed_in_remote" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            local.remote_confinement_points.insert(child_id);

            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "children_modified_in_both_with_confined_entries_and_remote_from_ourself" => {
            let initial_child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let initial_confined_child_id = VlobID::from_hex("fec4e512c3304019b01ba81619ddf563").unwrap();
            let new_shared_child_id = VlobID::from_hex("68a698cc8f7e40b884fac9a4dd152459").unwrap();
            let new_shared_confined_child_id = VlobID::from_hex("71994e1d17fd498cbd90e3319e823b97").unwrap();
            let new_local_child_id = VlobID::from_hex("51d73d55ca8c4de19a6295d7c0445648").unwrap();
            let new_local_confined_child_id = VlobID::from_hex("bb1e9b16ccd349e4bf2d6bfdea8e5f9a").unwrap();
            let new_remote_confined_child_id = VlobID::from_hex("766c1021232f41eaa258f820865637d5").unwrap();
            let new_remote_child_id = VlobID::from_hex("c75621ee01824e5ebdde4fcdc84e47e2").unwrap();

            local
                .base
                .children
                .insert("initial.txt".parse().unwrap(), initial_child_id);
            local
                .base
                .children
                .insert("initial_confined.tmp".parse().unwrap(), initial_confined_child_id);
            local.remote_confinement_points.insert(initial_confined_child_id);

            local
                .children
                .insert("initial.txt".parse().unwrap(), initial_child_id);
            local
                .children
                .insert("new_local.txt".parse().unwrap(), new_local_child_id);
            local
                .children
                .insert("new_local_confined.tmp".parse().unwrap(), new_local_confined_child_id);
            local.local_confinement_points.insert(new_local_confined_child_id);
            local
                .children
                .insert("new_shared.txt".parse().unwrap(), new_shared_child_id);
            local
                .children
                .insert("new_shared_confined.tmp".parse().unwrap(), new_shared_confined_child_id);
            local.local_confinement_points.insert(new_shared_confined_child_id);

            remote
                .children
                .insert("initial.txt".parse().unwrap(), initial_child_id);
            remote
                .children
                .insert("initial_confined.tmp".parse().unwrap(), initial_confined_child_id);
            remote
                .children
                .insert("new_remote.txt".parse().unwrap(), new_remote_child_id);
            remote
                .children
                .insert("new_remote_confined.tmp".parse().unwrap(), new_remote_confined_child_id);
            remote
                .children
                .insert("new_shared.txt".parse().unwrap(), new_shared_child_id);
            remote
                .children
                .insert("new_shared_confined.tmp".parse().unwrap(), new_shared_confined_child_id);
            remote.author = local_author;

            // Modifications from entries are considered already known (given we are the
            // author of those modification), and has since been reverted in local.
            // So the merge should acknowledge the remote manifest, but not changing
            // the local children.

            expected.base.author = local_author;
            expected
                .base
                .children
                .insert("initial.txt".parse().unwrap(), initial_child_id);
            expected
                .base
                .children
                .insert("initial_confined.tmp".parse().unwrap(), initial_confined_child_id);
            expected
                .base
                .children
                .insert("new_remote.txt".parse().unwrap(), new_remote_child_id);
            expected
                .base
                .children
                .insert("new_remote_confined.tmp".parse().unwrap(), new_remote_confined_child_id);
            expected
                .base
                .children
                .insert("new_shared.txt".parse().unwrap(), new_shared_child_id);
            expected
                .base
                .children
                .insert("new_shared_confined.tmp".parse().unwrap(), new_shared_confined_child_id);

            expected
                .children
                .insert("initial.txt".parse().unwrap(), initial_child_id);
            expected
                .children
                .insert("new_local.txt".parse().unwrap(), new_local_child_id);
            expected
                .children
                .insert("new_local_confined.tmp".parse().unwrap(), new_local_confined_child_id);
            expected
                .children
                .insert("new_shared.txt".parse().unwrap(), new_shared_child_id);
            expected
                .children
                .insert("new_shared_confined.tmp".parse().unwrap(), new_shared_confined_child_id);

            expected.remote_confinement_points.insert(initial_confined_child_id);
            expected.remote_confinement_points.insert(new_remote_confined_child_id);
            expected.remote_confinement_points.insert(new_shared_confined_child_id);
            expected.local_confinement_points.insert(new_local_confined_child_id);
            expected.local_confinement_points.insert(new_shared_confined_child_id);
        }
        "outdated_psp_local_child_becomes_non_confined" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);
            local.local_confinement_points.insert(child_id);

            expected
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);
        }
        "outdated_psp_remote_child_becomes_non_confined" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            local
                .base
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);
            local.remote_confinement_points.insert(child_id);
            remote
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);
            expected
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "outdated_psp_local_child_matches_new_pattern" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            // At this point, the prevent sync pattern is not `.tmp`, so `child.tmp`
            // is just a non-confined un-synced regular file.
            local
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            local.need_sync = true;

            expected
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            expected.local_confinement_points.insert(child_id);

            // There is no need to sync at this point.
            // However, the manifest was already marked as need-sync, and the merging
            // also detected a change due to the prevent sync pattern not being up-to-date.
            // It is similar to those situations where a file is added then removed, and
            // the prevent_sync_pattern stays to true. In any case, uploading a non-changing
            // manifest is not a big deal if it only happens in rare cases.
            expected.need_sync = true;
            expected.updated = merge_timestamp;
        }
        "outdated_psp_remote_child_matches_new_pattern" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            // At this point, the prevent sync pattern is not `.tmp`, so `child.tmp`
            // is just a non confined regular file.
            local
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            local
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            remote
                .children
                .insert("child.tmp".parse().unwrap(), child_id);

            expected
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            expected.remote_confinement_points.insert(child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "outdated_psp_remote_confined_entry_local_rename_then_remote_also_rename_with_confined_name" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            // The entry is initially remotely confined...
            local
                .base
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);
            local.remote_confinement_points.insert(child_id);
            // ...then gets renamed in both local and remote, with remote name
            // matching the new prevent sync pattern.
            //
            // This is a weird case given local is normally not able to rename
            // this child given it was confined (but this may occur if the entry
            // has not always been confined).
            local
                .children
                .insert("child-local-rename.txt".parse().unwrap(), child_id);
            local.need_sync = true;
            remote
                .children
                .insert("child-remote-rename.tmp".parse().unwrap(), child_id);

            // Here the file is removed locally. This is good, because this id is
            // now remotely confined, so it would be weird to keep using it locally
            // under a non-confined name.
            expected
                .base
                .children
                .insert("child-remote-rename.tmp".parse().unwrap(), child_id);
            expected.remote_confinement_points.insert(child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "outdated_psp_remote_confined_entry_local_rename_with_confined_name_then_remote_also_rename" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            // The entry is initially confined...
            local
                .base
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);
            local.remote_confinement_points.insert(child_id);
            // ...then gets renamed in both local and remote, with local names
            // matching the new prevent sync pattern.
            //
            // This is a weird case given local is normally not able to rename
            // this child given it was confined (but this may occur if the entry
            // has not always been confined).
            local
                .children
                .insert("child-local-rename.tmp".parse().unwrap(), child_id);
            local.need_sync = true;
            remote
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);

            expected
            .base
            .children
            .insert("child-remote-rename.txt".parse().unwrap(), child_id);
            // Here the file is renamed to the remote name, which is consistent
            // with the typical merge priority (the remote rename wins over the
            // local rename).
            expected
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "outdated_psp_remote_confined_entry_rename_in_both_with_confined_name" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            // The entry is initially confined...
            local
                .base
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);
            local.remote_confinement_points.insert(child_id);
            // ...then gets renamed in both local and remote, with names matching
            // the new prevent sync pattern.
            //
            // This is a weird case given local is normally not able to rename
            // this child given it was confined (but this may occur if the entry
            // has not always been confined).
            local
                .children
                .insert("child-local-rename.tmp".parse().unwrap(), child_id);
            local.need_sync = true;
            remote
                .children
                .insert("child-remote-rename.tmp".parse().unwrap(), child_id);

            // Surprisingly, the local file gets removed although it would have
            // been possible to keep it given the id had both a confined name locally
            // and remotely. This is just a side-effect of the prevent sync pattern
            // being outdated: it is similar to performing the merge with the old
            // pattern, and then applying the new one.
            expected
                .base
                .children
                .insert("child-remote-rename.tmp".parse().unwrap(), child_id);
            expected.remote_confinement_points.insert(child_id);
            expected.need_sync = false;
            expected.updated = remote.updated;
        }
        "outdated_psp_remote_child_becomes_non_confined_with_remote_from_ourself" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            // The entry is initially confined...
            local
                .base
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);
            local.remote_confinement_points.insert(child_id);
            // ...then gets renamed in remote, with a name not matching the new
            // prevent sync pattern.
            remote
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);
            remote.author = local_author;

            // Since `tmp~` is no longer a prevent sync pattern, `child.tmp~` is now
            // considered a regular local file
            // Given the remote is from ourself, the merge considers we already know
            // about it and hence acknowledges it and preserve the local children.
            // Hence the entry is considered renamed from `child-remote-rename.txt`
            // to `child.tmp~` locally.
            // This somehow swaps temporality (since the `child.tmp~` name was older
            // than the `child-remote-rename.txt` name), but this is acceptable given
            // that this should simply not happen: how have we been able to rename
            // a previously confined entry to a non-confined name?
            expected.base.author = local_author;
            expected
                .base
                .children
                .insert("child-remote-rename.txt".parse().unwrap(), child_id);
            expected
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);
        }
        "outdated_psp_local_child_becomes_non_confined_with_remote_from_ourself" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            // The entry is initially confined...
            local
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);
            local.local_confinement_points.insert(child_id);
            local.need_sync = false;
            local.updated = local.base.updated;
            // ...the remote hasn't anything important to merge, but this should
            // refresh the confinement in local with the new prevent sync pattern.
           remote.author = local_author;

            // Given the remote is from ourself, the merge considers we already know
            // about it and hence acknowledges it and preserve the local children.
            // However the new prevent sync pattern means the local child should
            // now be synchronized.
            expected.base.author = local_author;
            expected
                .children
                .insert("child.tmp~".parse().unwrap(), child_id);
        }
        "outdated_psp_remote_child_becomes_confined_with_remote_from_ourself" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            // The entry is initially not confined...
            local
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            // ...then an unrelated changed (only `updated` field) occurs in the
            // remote, which will trigger the refresh of the confined entries.
            remote
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
           remote.author = local_author;

            expected.base.author = local_author;
            expected
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            expected.remote_confinement_points.insert(child_id);
        }
        "outdated_psp_local_child_becomes_confined_with_remote_from_ourself" => {
            let child_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();

            // A `child.tmp` entry is added and initially not confined
            local
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            local.need_sync = true;
            // A corresponding remote manifest is produced and acknowledged by the server
            remote.children.insert("child.tmp".parse().unwrap(), child_id);
            remote.author = local_author;

            // Here `child.tmp` is removed, which is probably a bug to fix
            // TODO: add a test for applying a prevent sync pattern to a synchronized
            // entry containing a `.tmp` suffix.
            expected.base.author = local_author;
            expected
                .base
                .children
                .insert("child.tmp".parse().unwrap(), child_id);
            expected.remote_confinement_points.insert(child_id);
            expected.need_sync = true;
            expected.updated = merge_timestamp;
        }
        unknown => panic!("Unknown kind: {}", unknown),
    }

    let outcome = merge_local_folder_manifest(
        local_author,
        merge_timestamp,
        &prevent_sync_pattern,
        &local,
        remote,
    );
    p_assert_eq!(
        outcome,
        MergeLocalFolderManifestOutcome::Merged(Arc::new(expected))
    );
}

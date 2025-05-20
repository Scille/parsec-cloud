// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::num::NonZeroU64;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::workspace::merge::{MergeLocalFileManifestOutcome, merge_local_file_manifest};

#[parsec_test(testbed = "minimal_client_ready")]
async fn no_remote_change(
    #[values("same_version", "older_version", "same_version_with_local_change")] kind: &str,
    env: &TestbedEnv,
) {
    let local_author = "alice@dev1".parse().unwrap();
    let timestamp = "2021-01-10T00:00:00Z".parse().unwrap();
    let vlob_id = VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap();
    let parent_id = VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap();

    let mut remote = FileManifest {
        author: "bob@dev1".parse().unwrap(),
        timestamp: "2021-01-03T00:00:00Z".parse().unwrap(),
        id: vlob_id,
        parent: parent_id,
        version: 2,
        created: "2021-01-01T00:00:00Z".parse().unwrap(),
        updated: "2021-01-02T00:00:00Z".parse().unwrap(),
        size: 10,
        blocksize: Blocksize::try_from(512 * 1024).unwrap(),
        blocks: vec![BlockAccess {
            id: BlockID::from_hex("7b2d00afd9f242b5a362eeb85e1be31a").unwrap(),
            offset: 0,
            size: NonZeroU64::new(10).unwrap(),
            digest: HashDigest::from(hex!(
                "7d486915b914332bb5730fd772223e8b276919e51edca2de0f82c5fc1bce7eb5"
            )),
        }],
    };
    let mut local = LocalFileManifest {
        base: remote.clone(),
        parent: parent_id,
        need_sync: false,
        updated: "2021-01-02T00:00:00Z".parse().unwrap(),
        size: 10,
        blocksize: Blocksize::try_from(512 * 1024).unwrap(),
        blocks: vec![vec![ChunkView {
            id: remote.blocks[0].id.into(),
            start: 0,
            stop: NonZeroU64::new(10).unwrap(),
            raw_offset: 0,
            raw_size: NonZeroU64::new(10).unwrap(),
            access: Some(remote.blocks[0].clone()),
        }]],
    };
    match kind {
        "same_version" => (),
        "older_version" => {
            remote.version = 1;
            // Changes in the remote are ignored since it's an old version
            remote.updated = "2021-01-01T00:00:00Z".parse().unwrap();
            remote.size = 15;
            remote.parent = VlobID::from_hex("1a79300d1f62450ca303122480a13ec2").unwrap();
            remote.blocks.push(BlockAccess {
                id: BlockID::from_hex("6b0bbd6756114a7ea66271b15bf9938e").unwrap(),
                offset: 0,
                size: NonZeroU64::new(10).unwrap(),
                digest: HashDigest::from(hex!(
                    "3d66ba5747c74614850dab7c14cbe7b303ddb1823998491f87a261dcadd978d9"
                )),
            });
        }
        "same_version_with_local_change" => {
            local.need_sync = true;
            local.updated = "2021-01-03T00:00:00Z".parse().unwrap();
            local.parent = VlobID::from_hex("1a79300d1f62450ca303122480a13ec2").unwrap();
            local.blocksize = Blocksize::try_from(1024 * 1024).unwrap();
            local.size = 15;
            local.blocks[0].push(ChunkView {
                id: remote.blocks[0].id.into(),
                start: 10,
                stop: NonZeroU64::new(15).unwrap(),
                raw_offset: 10,
                raw_size: NonZeroU64::new(15).unwrap(),
                access: None,
            });
        }
        unknown => panic!("Unknown kind: {}", unknown),
    }

    let outcome = merge_local_file_manifest(local_author, timestamp, &local, remote);
    p_assert_eq!(outcome, MergeLocalFileManifestOutcome::NoChange);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn remote_only_change(
    #[values(
        "only_updated_field_modified",
        "parent_field_modified",
        "blocksize_changed",
        "new_block_added",
        "block_replaced"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let local_author = "alice@dev1".parse().unwrap();
    let timestamp = "2021-01-10T00:00:00Z".parse().unwrap();
    let vlob_id = VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap();
    let parent_id = VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap();

    // Start by creating `local` and `remote` manifests with minimal changes:
    // `remote` is just version n+1 with `updated` field set to a new timestamp.
    // Then this base will be customized in the following `match kind` statement.

    let mut remote = FileManifest {
        author: "bob@dev1".parse().unwrap(),
        timestamp: "2021-01-03T00:00:00Z".parse().unwrap(),
        id: vlob_id,
        parent: parent_id,
        version: 1,
        created: "2021-01-01T00:00:00Z".parse().unwrap(),
        updated: "2021-01-02T00:00:00Z".parse().unwrap(),
        size: 0,
        blocksize: Blocksize::try_from(512 * 1024).unwrap(),
        blocks: vec![],
    };
    let mut local = LocalFileManifest {
        base: remote.clone(),
        parent: remote.parent,
        need_sync: false,
        updated: remote.updated,
        size: remote.size,
        blocksize: remote.blocksize,
        blocks: vec![],
    };

    remote.version = 2;
    remote.updated = "2021-01-04T00:00:00Z".parse().unwrap();
    remote.timestamp = "2021-01-05T00:00:00Z".parse().unwrap();

    let mut expected = LocalFileManifest {
        base: FileManifest {
            author: "bob@dev1".parse().unwrap(),
            timestamp: "2021-01-05T00:00:00Z".parse().unwrap(),
            id: vlob_id,
            parent: parent_id,
            version: 2,
            created: "2021-01-01T00:00:00Z".parse().unwrap(),
            updated: "2021-01-04T00:00:00Z".parse().unwrap(),
            size: 0,
            blocksize: Blocksize::try_from(512 * 1024).unwrap(),
            blocks: vec![],
        },
        parent: parent_id,
        need_sync: false,
        updated: "2021-01-04T00:00:00Z".parse().unwrap(),
        size: 0,
        blocksize: Blocksize::try_from(512 * 1024).unwrap(),
        blocks: vec![],
    };

    match kind {
        "only_updated_field_modified" => (),
        "parent_field_modified" => {
            let new_parent_id = VlobID::from_hex("b95472b9c6d9415fa65297835d1feca5").unwrap();
            remote.parent = new_parent_id;
            expected.base.parent = new_parent_id;
            expected.parent = new_parent_id;
        }
        "blocksize_changed" => {
            remote.blocksize = Blocksize::try_from(1024 * 1024).unwrap();
            expected.blocksize = remote.blocksize;
            expected.base.blocksize = remote.blocksize;
        }
        "new_block_added" => {
            let chunk_id = BlockID::from_hex("adbeac70e3cd4990ae6396a277e91ca4").unwrap();
            remote.size = 10;
            remote.blocks.push(BlockAccess {
                id: chunk_id,
                offset: 0,
                size: NonZeroU64::new(10).unwrap(),
                digest: HashDigest::from(hex!(
                    "3d66ba5747c74614850dab7c14cbe7b303ddb1823998491f87a261dcadd978d9"
                )),
            });

            expected.base.size = remote.size;
            expected.base.blocks.clone_from(&remote.blocks);
            expected.size = 10;
            expected.blocks.push(vec![ChunkView {
                id: chunk_id.into(),
                start: 0,
                stop: NonZeroU64::new(10).unwrap(),
                raw_offset: 0,
                raw_size: NonZeroU64::new(10).unwrap(),
                access: Some(remote.blocks[0].clone()),
            }]);
        }
        "block_replaced" => {
            let old_chunk_id = BlockID::from_hex("adbeac70e3cd4990ae6396a277e91ca4").unwrap();
            let new_chunk_id = BlockID::from_hex("c2d3d962f7b749049ad9e5b10f610c70").unwrap();

            local.base.size = 10;
            local.base.blocks.push(BlockAccess {
                id: old_chunk_id,
                offset: 0,
                size: NonZeroU64::new(10).unwrap(),
                digest: HashDigest::from(hex!(
                    "3d66ba5747c74614850dab7c14cbe7b303ddb1823998491f87a261dcadd978d9"
                )),
            });
            local.size = 10;
            local.blocks.push(vec![ChunkView {
                id: old_chunk_id.into(),
                start: 0,
                stop: NonZeroU64::new(10).unwrap(),
                raw_offset: 0,
                raw_size: NonZeroU64::new(10).unwrap(),
                access: Some(local.base.blocks[0].clone()),
            }]);

            remote.size = 15;
            remote.blocks.push(BlockAccess {
                id: new_chunk_id,
                offset: 0,
                size: NonZeroU64::new(15).unwrap(),
                digest: HashDigest::from(hex!(
                    "957d1ffaa047479bb2e21416949182b33897fb6bfe674a439dd2b682e327dbe3"
                )),
            });

            expected.base.size = remote.size;
            expected.base.blocks.clone_from(&remote.blocks);
            expected.size = remote.size;
            expected.blocks.push(vec![ChunkView {
                id: new_chunk_id.into(),
                start: 0,
                stop: NonZeroU64::new(15).unwrap(),
                raw_offset: 0,
                raw_size: NonZeroU64::new(15).unwrap(),
                access: Some(remote.blocks[0].clone()),
            }]);
        }
        unknown => panic!("Unknown kind: {}", unknown),
    }

    let outcome = merge_local_file_manifest(local_author, timestamp, &local, remote);
    p_assert_eq!(outcome, MergeLocalFileManifestOutcome::Merged(expected));
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn local_and_remote_changes(
    #[values(
        "only_updated_field_modified",
        "parent_modified_in_remote_and_only_update_field_modified_in_local",
        "parent_modified_in_remote_and_unrelated_block_change_in_local",
        "parent_modified_in_local_and_only_update_field_modified_in_remote",
        "parent_modified_in_local_and_unrelated_block_change_in_remote",
        "parent_modified_in_both_with_different_value",
        "parent_modified_in_both_with_same_value",
        "parent_modified_in_both_with_unrelated_block_change_in_local",
        "parent_modified_in_both_with_remote_from_ourself",
        "blocksize_modified_in_both_with_remote_from_ourself",
        "blocks_modified_in_both_with_remote_from_ourself",
        "size_and_blocks_modified_in_both_with_remote_from_ourself",
        "blocksize_modified_in_both",
        "blocks_modified_in_both",
        "size_and_blocks_modified_in_both"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let local_author = "alice@dev1".parse().unwrap();
    let timestamp = "2021-01-10T00:00:00Z".parse().unwrap();
    let vlob_id = VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap();
    let parent_id = VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap();
    let block_id = BlockID::from_hex("7c76251f55b14f63a837d63567becf34").unwrap();

    // Start by creating `local` and `remote` manifests with minimal changes:
    // `remote` is just version n+1 with `updated` field set to a new timestamp.
    // Then this base will be customized in the following `match kind` statement.

    let mut remote = FileManifest {
        author: "bob@dev1".parse().unwrap(),
        timestamp: "2021-01-03T00:00:00Z".parse().unwrap(),
        id: vlob_id,
        parent: parent_id,
        version: 1,
        created: "2021-01-01T00:00:00Z".parse().unwrap(),
        updated: "2021-01-02T00:00:00Z".parse().unwrap(),
        size: 10,
        blocksize: Blocksize::try_from(10).unwrap(),
        blocks: vec![BlockAccess {
            id: block_id,
            offset: 0,
            size: NonZeroU64::new(10).unwrap(),
            digest: HashDigest::from(hex!(
                "26d623b082f145d88927a4de50c162b59b2aaa1202ae4415b18e26a67c7a43a7"
            )),
        }],
    };
    let mut local = LocalFileManifest {
        base: remote.clone(),
        parent: remote.parent,
        need_sync: true,
        updated: "2021-01-10T00:00:00Z".parse().unwrap(),
        size: remote.size,
        blocksize: remote.blocksize,
        blocks: vec![vec![ChunkView {
            id: block_id.into(),
            start: 0,
            stop: NonZeroU64::new(10).unwrap(),
            raw_offset: 0,
            raw_size: NonZeroU64::new(10).unwrap(),
            access: Some(remote.blocks[0].clone()),
        }]],
    };

    remote.version = 2;
    remote.updated = "2021-01-04T00:00:00Z".parse().unwrap();
    remote.timestamp = "2021-01-05T00:00:00Z".parse().unwrap();

    let mut merged = LocalFileManifest {
        base: remote.clone(),
        parent: parent_id,
        need_sync: true,
        updated: "2021-01-10T00:00:00Z".parse().unwrap(),
        size: 10,
        blocksize: Blocksize::try_from(10).unwrap(),
        blocks: vec![vec![ChunkView {
            id: block_id.into(),
            start: 0,
            stop: NonZeroU64::new(10).unwrap(),
            raw_offset: 0,
            raw_size: NonZeroU64::new(10).unwrap(),
            access: Some(remote.blocks[0].clone()),
        }]],
    };

    let expected = match kind {
        "only_updated_field_modified" => {
            // Since only `updated` has been modified on local, then
            // it is overwritten by the remote value, then the merge
            // determine there is nothing more to sync here
            merged.updated = remote.updated;
            merged.need_sync = false;

            MergeLocalFileManifestOutcome::Merged(merged)
        }
        "parent_modified_in_remote_and_only_update_field_modified_in_local" => {
            let new_parent_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            remote.parent = new_parent_id;

            merged.base.parent = new_parent_id;
            merged.parent = new_parent_id;
            merged.need_sync = false;
            merged.updated = remote.updated;

            MergeLocalFileManifestOutcome::Merged(merged)
        }
        "parent_modified_in_remote_and_unrelated_block_change_in_local" => {
            let new_parent_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            remote.parent = new_parent_id;
            // Also add an unrelated change on local side
            let new_chunk_id = ChunkID::from_hex("6d06c7044f7d4275b5269aa1b6ae137e").unwrap();
            local.blocks[0][0].id = new_chunk_id;
            local.blocks[0][0].access = None;

            merged.base.parent = new_parent_id;
            merged.parent = new_parent_id;
            merged.blocks.clone_from(&local.blocks);

            MergeLocalFileManifestOutcome::Merged(merged)
        }
        "parent_modified_in_local_and_only_update_field_modified_in_remote" => {
            let new_parent_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            local.parent = new_parent_id;

            merged.parent = new_parent_id;

            MergeLocalFileManifestOutcome::Merged(merged)
        }
        "parent_modified_in_local_and_unrelated_block_change_in_remote" => {
            let new_parent_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            local.parent = new_parent_id;
            // Also add an unrelated change on remote side
            let new_block_id = BlockID::from_hex("c4bd6179df134cd49ec50c70d09a1bc7").unwrap();
            remote.blocks[0].id = new_block_id;

            merged.parent = new_parent_id;
            merged.blocks[0][0].id = new_block_id.into();
            merged.blocks[0][0].access.as_mut().unwrap().id = new_block_id;
            merged.base.blocks[0].id = new_block_id;

            MergeLocalFileManifestOutcome::Merged(merged)
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
            merged.need_sync = false;
            merged.updated = remote.updated;
            merged.parent = new_remote_parent_id;
            merged.base.parent = new_remote_parent_id;

            MergeLocalFileManifestOutcome::Merged(merged)
        }
        "parent_modified_in_both_with_same_value" => {
            let new_parent_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            local.parent = new_parent_id;
            remote.parent = new_parent_id;

            merged.need_sync = false;
            merged.updated = remote.updated;
            merged.parent = new_parent_id;
            merged.base.parent = new_parent_id;

            MergeLocalFileManifestOutcome::Merged(merged)
        }
        "parent_modified_in_both_with_unrelated_block_change_in_local" => {
            let new_local_parent_id = VlobID::from_hex("a1d7229d7e44418a8a4e4fd821003fd3").unwrap();
            let new_remote_parent_id =
                VlobID::from_hex("44a80da765bd41ed984fdee9e7b0fd0b").unwrap();
            local.parent = new_local_parent_id;
            remote.parent = new_remote_parent_id;
            // Also add an unrelated change on local side
            let new_chunk_id = ChunkID::from_hex("6d06c7044f7d4275b5269aa1b6ae137e").unwrap();
            local.blocks[0][0].id = new_chunk_id;
            local.blocks[0][0].access = None;

            // The re-parent in local got overwritten, but there is still changes
            // in blocks that require sync.
            merged.parent = new_remote_parent_id;
            merged.base.parent = new_remote_parent_id;
            merged.size = local.size;
            merged.blocks.clone_from(&local.blocks);

            MergeLocalFileManifestOutcome::Merged(merged)
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
            merged.base.author = local_author;
            merged.parent = new_local_parent_id;
            merged.base.parent = new_remote_parent_id;

            MergeLocalFileManifestOutcome::Merged(merged)
        }
        "blocks_modified_in_both_with_remote_from_ourself" => {
            let new_block_id = BlockID::from_hex("c4bd6179df134cd49ec50c70d09a1bc7").unwrap();
            remote.blocks[0].id = new_block_id;

            let new_chunk_id = ChunkID::from_hex("6d06c7044f7d4275b5269aa1b6ae137e").unwrap();
            local.blocks[0][0].id = new_chunk_id;
            local.blocks[0][0].access = None;

            // The remote change are from ourself, hence instead of conflict
            // we should just acknowledge the remote and keep the local changes.
            remote.author = local_author;

            merged.base.author = local_author;

            merged.base.blocks.clone_from(&remote.blocks);
            merged.blocks.clone_from(&local.blocks);

            MergeLocalFileManifestOutcome::Merged(merged)
        }
        "size_and_blocks_modified_in_both_with_remote_from_ourself" => {
            remote.size = 5;
            remote.blocks[0].size = NonZeroU64::new(5).unwrap();
            local.size = 2;
            local.blocks[0][0].stop = NonZeroU64::new(2).unwrap();

            // The remote change are from ourself, hence instead of conflict
            // we should just acknowledge the remote and keep the local changes.
            remote.author = local_author;

            merged.base.author = local_author;

            merged.base.size = remote.size;
            merged.base.blocks.clone_from(&remote.blocks);
            merged.size = local.size;
            merged.blocks.clone_from(&local.blocks);

            MergeLocalFileManifestOutcome::Merged(merged)
        }
        "blocksize_modified_in_both_with_remote_from_ourself" => {
            remote.blocksize = Blocksize::try_from(1024 * 1024).unwrap();
            local.blocksize = Blocksize::try_from(2048 * 1024).unwrap();

            // The remote change are from ourself, hence instead of conflict
            // we should just acknowledge the remote and keep the local changes.
            remote.author = local_author;

            merged.base.author = local_author;

            merged.base.blocksize = remote.blocksize;
            merged.blocksize = local.blocksize;

            MergeLocalFileManifestOutcome::Merged(merged)
        }
        "blocksize_modified_in_both" => {
            remote.blocksize = Blocksize::try_from(1024 * 1024).unwrap();
            local.blocksize = Blocksize::try_from(2048 * 1024).unwrap();

            MergeLocalFileManifestOutcome::Conflict(remote.clone())
        }
        "blocks_modified_in_both" => {
            let new_block_id = BlockID::from_hex("c4bd6179df134cd49ec50c70d09a1bc7").unwrap();
            remote.blocks[0].id = new_block_id;

            let new_chunk_id = ChunkID::from_hex("6d06c7044f7d4275b5269aa1b6ae137e").unwrap();
            local.blocks[0][0].id = new_chunk_id;
            local.blocks[0][0].access = None;

            MergeLocalFileManifestOutcome::Conflict(remote.clone())
        }
        "size_and_blocks_modified_in_both" => {
            remote.size = 5;
            remote.blocks[0].size = NonZeroU64::new(5).unwrap();
            local.size = 2;
            local.blocks[0][0].stop = NonZeroU64::new(2).unwrap();

            MergeLocalFileManifestOutcome::Conflict(remote.clone())
        }
        unknown => panic!("Unknown kind: {}", unknown),
    };

    let outcome = merge_local_file_manifest(local_author, timestamp, &local, remote);
    p_assert_eq!(outcome, expected);
}

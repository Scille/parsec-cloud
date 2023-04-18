// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashSet;

use libparsec_types::prelude::*;

#[derive(Default)]
pub struct ChangesAfterSync {
    pub added_blocks: HashSet<BlockAccess>,
    pub removed_blocks: HashSet<BlockAccess>,
    pub added_entries: HashSet<EntryID>,
    pub removed_entries: HashSet<EntryID>,
}

impl From<(&FileManifest, &FileManifest)> for ChangesAfterSync {
    fn from((old_manifest, new_manifest): (&FileManifest, &FileManifest)) -> Self {
        let old_blocks = old_manifest
            .blocks
            .clone()
            .into_iter()
            .collect::<HashSet<_>>();
        let new_blocks = new_manifest
            .blocks
            .clone()
            .into_iter()
            .collect::<HashSet<_>>();
        Self {
            added_blocks: &new_blocks - &old_blocks,
            removed_blocks: &old_blocks - &new_blocks,
            ..Default::default()
        }
    }
}

impl From<(&FolderManifest, &FolderManifest)> for ChangesAfterSync {
    fn from((old_manifest, new_manifest): (&FolderManifest, &FolderManifest)) -> Self {
        let old_entries = old_manifest
            .children
            .clone()
            .into_values()
            .collect::<HashSet<_>>();
        let new_entries = new_manifest
            .children
            .clone()
            .into_values()
            .collect::<HashSet<_>>();
        Self {
            added_entries: &new_entries - &old_entries,
            removed_entries: &old_entries - &new_entries,
            ..Default::default()
        }
    }
}

impl From<(&WorkspaceManifest, &WorkspaceManifest)> for ChangesAfterSync {
    fn from((old_manifest, new_manifest): (&WorkspaceManifest, &WorkspaceManifest)) -> Self {
        let old_entries = old_manifest
            .children
            .clone()
            .into_values()
            .collect::<HashSet<_>>();
        let new_entries = new_manifest
            .children
            .clone()
            .into_values()
            .collect::<HashSet<_>>();
        Self {
            added_entries: &new_entries - &old_entries,
            removed_entries: &old_entries - &new_entries,
            ..Default::default()
        }
    }
}

impl From<(FileManifest, FileManifest)> for ChangesAfterSync {
    fn from((old_manifest, new_manifest): (FileManifest, FileManifest)) -> Self {
        let old_blocks = old_manifest.blocks.into_iter().collect::<HashSet<_>>();
        let new_blocks = new_manifest.blocks.into_iter().collect::<HashSet<_>>();
        Self {
            added_blocks: &new_blocks - &old_blocks,
            removed_blocks: &old_blocks - &new_blocks,
            ..Default::default()
        }
    }
}

impl From<(FolderManifest, FolderManifest)> for ChangesAfterSync {
    fn from((old_manifest, new_manifest): (FolderManifest, FolderManifest)) -> Self {
        let old_entries = old_manifest.children.into_values().collect::<HashSet<_>>();
        let new_entries = new_manifest.children.into_values().collect::<HashSet<_>>();
        Self {
            added_entries: &new_entries - &old_entries,
            removed_entries: &old_entries - &new_entries,
            ..Default::default()
        }
    }
}

impl From<(WorkspaceManifest, WorkspaceManifest)> for ChangesAfterSync {
    fn from((old_manifest, new_manifest): (WorkspaceManifest, WorkspaceManifest)) -> Self {
        let old_entries = old_manifest.children.into_values().collect::<HashSet<_>>();
        let new_entries = new_manifest.children.into_values().collect::<HashSet<_>>();
        Self {
            added_entries: &new_entries - &old_entries,
            removed_entries: &old_entries - &new_entries,
            ..Default::default()
        }
    }
}

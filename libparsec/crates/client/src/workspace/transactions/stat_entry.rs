// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::{super::WorkspaceOps, get_child_manifest, resolve_path, FsOperationError};

#[derive(Debug, Clone)]
pub enum EntryStat {
    File {
        /// The confinement point corresponds to the entry id of the parent folderish
        /// manifest that contains a child with a confined name in the path leading
        /// to our entry.
        confinement_point: Option<VlobID>,
        id: VlobID,
        created: DateTime,
        updated: DateTime,
        base_version: VersionInt,
        is_placeholder: bool,
        need_sync: bool,
        size: SizeInt,
    },
    // Here Folder can also be the root of the workspace (i.e. WorkspaceManifest)
    Folder {
        /// The confinement point corresponds to the entry id of the parent folderish
        /// manifest that contains a child with a confined name in the path leading
        /// to our entry.
        confinement_point: Option<VlobID>,
        id: VlobID,
        created: DateTime,
        updated: DateTime,
        base_version: VersionInt,
        is_placeholder: bool,
        need_sync: bool,
        children: Vec<EntryName>,
    },
}

pub(crate) async fn stat_entry(
    ops: &WorkspaceOps,
    path: &FsPath,
) -> Result<EntryStat, FsOperationError> {
    // Special case for /
    if path.is_root() {
        let manifest = ops.data_storage.get_workspace_manifest();
        // Ensure children are sorted alphabetically to simplify testing
        let children = {
            let mut children: Vec<_> = manifest
                .children
                .keys()
                .map(|name| name.to_owned())
                .collect();
            children.sort_unstable_by(|a, b| a.as_ref().cmp(b.as_ref()));
            children
        };

        let info = EntryStat::Folder {
            // Root has no parent, hence confinement_point is never possible
            confinement_point: None,
            id: manifest.base.id,
            created: manifest.base.created,
            updated: manifest.updated,
            base_version: manifest.base.version,
            is_placeholder: manifest.base.version == 0,
            need_sync: manifest.need_sync,
            children,
        };

        Ok(info)
    } else {
        let resolution = resolve_path(ops, path).await?;
        let manifest = get_child_manifest(ops, resolution.entry_id).await?;

        let info = match manifest {
            ArcLocalChildManifest::File(m) => EntryStat::File {
                confinement_point: resolution.confinement_point,
                id: m.base.id,
                created: m.base.created,
                updated: m.updated,
                base_version: m.base.version,
                is_placeholder: m.base.version == 0,
                need_sync: m.need_sync,
                size: m.size,
            },
            ArcLocalChildManifest::Folder(m) => {
                // Ensure children are sorted alphabetically to simplify testing
                let children = {
                    let mut children: Vec<_> =
                        m.children.keys().map(|name| name.to_owned()).collect();
                    children.sort_unstable_by(|a, b| a.as_ref().cmp(b.as_ref()));
                    children
                };

                EntryStat::Folder {
                    confinement_point: resolution.confinement_point,
                    id: m.base.id,
                    created: m.base.created,
                    updated: m.updated,
                    base_version: m.base.version,
                    is_placeholder: m.base.version == 0,
                    need_sync: m.need_sync,
                    children,
                }
            }
        };

        Ok(info)
    }
}

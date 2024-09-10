// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
use std::sync::Arc;

use libparsec_platform_storage::workspace::{UpdateManifestData, WorkspaceStorage};
use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum ApplyPreventSyncPatternError {
    #[error(transparent)]
    Internal(anyhow::Error),
}

pub(super) async fn ensure_prevent_sync_pattern_applied_to_wksp(
    storage: &mut WorkspaceStorage,
    device: Arc<LocalDevice>,
    pattern: &Regex,
) -> Result<(), ApplyPreventSyncPatternError> {
    const PAGE_SIZE: u32 = 1000;

    let fully_applied = storage
        .set_prevent_sync_pattern(pattern)
        .await
        .map_err(ApplyPreventSyncPatternError::Internal)?;

    if fully_applied {
        return Ok(());
    }

    let mut offset = 0;
    let mut updated_manifest = Vec::new();
    loop {
        let manifests = storage
            .list_manifests(offset, PAGE_SIZE)
            .await
            .map_err(ApplyPreventSyncPatternError::Internal)?;

        for encoded_manifest in &manifests {
            // Only the prevent sync pattern could be applied to a folder type manifest.
            // We assume that workspace manifest and folder manifest could both be deserialize as folder manifest.
            if let Some(folder) = decode_folder(encoded_manifest, &device.local_symkey)? {
                let new_folder =
                    folder.apply_prevent_sync_pattern(pattern, device.time_provider.now());
                if new_folder != folder {
                    updated_manifest.push(UpdateManifestData {
                        entry_id: new_folder.base.id,
                        encrypted: new_folder.dump_and_encrypt(&device.local_symkey),
                        need_sync: new_folder.need_sync,
                        base_version: new_folder.base.version,
                    })
                }
            }
        }

        storage
            .update_manifests(updated_manifest.drain(..))
            .await
            .map_err(ApplyPreventSyncPatternError::Internal)?;

        // The manifests list is not filled to the page size,
        // We consider that another call will result in an empty list, so we can stop here.
        if manifests.len() < PAGE_SIZE as usize {
            break;
        }
        updated_manifest.clear();
        offset += PAGE_SIZE;
    }
    storage
        .mark_prevent_sync_pattern_fully_applied(pattern)
        .await
        .map_err(|e| ApplyPreventSyncPatternError::Internal(e.into()))?;
    Ok(())
}

fn decode_folder(
    encoded: &[u8],
    symkey: &SecretKey,
) -> Result<Option<LocalFolderManifest>, ApplyPreventSyncPatternError> {
    match LocalFolderManifest::decrypt_and_load(encoded, symkey) {
        Ok(manifest) => Ok(Some(manifest)),
        // We consider that BadSerialization indicate that the manifest is not of folder type.
        Err(DataError::BadSerialization { .. }) => Ok(None),
        Err(e) => Err(ApplyPreventSyncPatternError::Internal(anyhow::anyhow!(
            "Local database contains invalid data: {e}"
        ))),
    }
}

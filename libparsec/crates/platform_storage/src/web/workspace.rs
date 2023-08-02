// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::path::Path;

use libparsec_types::prelude::*;

pub async fn workspace_storage_non_speculative_init(
    _data_base_dir: &Path,
    _device: &LocalDevice,
    _workspace_id: EntryID,
) -> anyhow::Result<()> {
    todo!()
}

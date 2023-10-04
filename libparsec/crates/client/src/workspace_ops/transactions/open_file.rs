// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::{super::WorkspaceOps, FsOperationError};

pub struct OpenedFile {}

pub async fn open_file(
    _ops: &WorkspaceOps,
    _path: &FsPath,
) -> Result<OpenedFile, FsOperationError> {
    todo!()
}

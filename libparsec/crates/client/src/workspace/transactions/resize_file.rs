// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::{super::WorkspaceOps, check_write_access, FsOperationError};

pub(crate) async fn resize_file(
    ops: &WorkspaceOps,
    _path: &FsPath,
    _size: usize,
) -> Result<(), FsOperationError> {
    check_write_access(ops)?;

    todo!()
}

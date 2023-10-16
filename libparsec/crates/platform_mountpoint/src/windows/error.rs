// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use winfsp_wrs::{
    NTSTATUS, STATUS_ACCESS_DENIED, STATUS_DIRECTORY_NOT_EMPTY, STATUS_END_OF_FILE,
    STATUS_NAME_TOO_LONG, STATUS_OBJECT_NAME_COLLISION, STATUS_OBJECT_NAME_INVALID,
    STATUS_OBJECT_NAME_NOT_FOUND,
};

use crate::MountpointError;

impl From<MountpointError> for NTSTATUS {
    fn from(value: MountpointError) -> Self {
        match value {
            MountpointError::AccessDenied => STATUS_ACCESS_DENIED,
            MountpointError::DirNotEmpty => STATUS_DIRECTORY_NOT_EMPTY,
            MountpointError::EndOfFile => STATUS_END_OF_FILE,
            MountpointError::NameCollision => STATUS_OBJECT_NAME_COLLISION,
            MountpointError::NameTooLong => STATUS_NAME_TOO_LONG,
            MountpointError::InvalidName => STATUS_OBJECT_NAME_INVALID,
            MountpointError::NotFound => STATUS_OBJECT_NAME_NOT_FOUND,
        }
    }
}

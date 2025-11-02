// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use libparsec_types::prelude::*;
use web_sys::wasm_bindgen::JsValue;

error_set::error_set! {
    DomExceptionError := {
        #[display("DOM exception: {exception:?}")]
        DomException {
            exception: web_sys::DomException
        }
    }
    CastError := {
        #[display("Failed to cast to {ty} ({value:?}")]
        Cast {
            ty: &'static str,
            value: JsValue
        }
    }
    AwaitPromiseError := {
        #[display("Failed to await JS promise ({error:?})")]
        Promise {
            error: JsValue
        }
    }
    JsPromiseError := CastError || AwaitPromiseError
    NotFoundError := {
        #[display("No such file or directory at {}", path.display())]
        NotFound {
            path: PathBuf,
        }
    }
    GetRootDirectoryError := AwaitPromiseError || CastError || DomExceptionError || {
        #[display("Storage not available ({exception:?})")]
        StorageNotAvailable { exception: web_sys::DomException }
    }
    GetDirectoryHandleError := CastError || AwaitPromiseError || DomExceptionError || NotFoundError || {
        #[display("Item at {} is not a directory", path.display())]
        NotADirectory {
            path: PathBuf,
        },
    }
    GetFileHandleError := CastError || AwaitPromiseError || DomExceptionError || NotFoundError || GetDirectoryHandleError || {
        #[display("Item at {} is not a directory", path.display())]
        NotAFile {
            path: PathBuf
        }
    }
    ReadToEndError := DomExceptionError
        || NotFoundError
        || CastError
        || AwaitPromiseError
        || {
        #[display("Failed to get file object ({error:?})")]
        GetFile { error: JsValue },
    }
    WriteAllError := CastError || NotFoundError || DomExceptionError || {
        #[display("Failed to create writable stream ({error:?})")]
        CreateWritable { error: JsValue },
        #[display("Cannot edit resource `{}` ({exception:?})", path.display())]
        CannotEdit {
            path: PathBuf,
            exception: web_sys::DomException
        },
        #[display("No space left ({exception:?})")]
        NoSpaceLeft { exception: web_sys::DomException },
        #[display("Write error ({error:?})")]
        Write {
            error: JsValue
        },
        #[display("Close error ({error:?})")]
        Close {
            error: JsValue
        }
    }
    NewStorageError := GetRootDirectoryError
    ListFileEntriesError := GetDirectoryHandleError
    ListAvailableDevicesError := ListFileEntriesError
        || LoadAvailableDeviceError
        || AwaitPromiseError
        || {
            ReadToEnd(ReadToEndError)
        }
    ListPkiLocalPendingError := GetDirectoryHandleError
        || LoadPkiLocalPendingError
        || AwaitPromiseError
        || {
            ReadToEnd(ReadToEndError)
        }
    LoadPkiLocalPendingError := {
        ReadToEnd(ReadToEndError),
        DataError(libparsec_types::DataError)
    }
    DeviceMissingError := NotFoundError
    RmpDecodeError := {
        RmpDecode(libparsec_types::RmpDecodeError)
    }
    LoadAvailableDeviceError := {
        ReadToEnd(ReadToEndError),
        RmpDecode(libparsec_types::RmpDecodeError)
    }
    ReadFile := {
        GetFile(GetFileHandleError),
        ReadFile(ReadToEndError),
    }
    SaveDeviceError := SaveRawDataError || {
        RemoteOpaqueKeyUploadFailed(anyhow::Error),
        RemoteOpaqueKeyUploadOffline(anyhow::Error),
        Internal(anyhow::Error),
    }
    SaveRawDataError := GetFileHandleError || WriteAllError
    RemoveEntryError := NotFoundError || DomExceptionError || AwaitPromiseError || GetDirectoryHandleError
    ArchiveDeviceError := RemoveEntryError || {
        GetDeviceToArchive(GetFileHandleError),
        ReadDeviceToArchive(ReadToEndError),
        CreateArchiveDevice(GetFileHandleError),
        WriteArchiveDevice(WriteAllError),
    }
    RemoveDeviceError := SaveRawDataError
}

macro_rules! impl_from_new_storage_error {
    ($for:ty) => {
        impl From<NewStorageError> for $for {
            fn from(value: NewStorageError) -> Self {
                Self::Internal(anyhow::anyhow!("{value}"))
            }
        }
    };
    ($($for:ty),*) => {
        $(impl_from_new_storage_error!($for);)*
    };
}

impl_from_new_storage_error!(
    crate::SaveDeviceError,
    crate::UpdateDeviceError,
    crate::ArchiveDeviceError,
    crate::RemoveDeviceError
);

impl From<SaveDeviceError> for crate::SaveDeviceError {
    fn from(value: SaveDeviceError) -> Self {
        match value {
            SaveDeviceError::NotAFile { .. } => Self::InvalidPath(anyhow::anyhow!("{value}")),
            SaveDeviceError::NotADirectory { .. } => Self::InvalidPath(anyhow::anyhow!("{value}")),
            SaveDeviceError::NotFound { .. } => Self::InvalidPath(anyhow::anyhow!("{value}")),
            SaveDeviceError::CreateWritable { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::CannotEdit { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::Write { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::Close { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::DomException { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::Cast { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::Promise { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::NoSpaceLeft { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::RemoteOpaqueKeyUploadFailed(err) => {
                Self::RemoteOpaqueKeyUploadFailed(err)
            }
            SaveDeviceError::RemoteOpaqueKeyUploadOffline(err) => {
                Self::RemoteOpaqueKeyUploadOffline(err)
            }
            SaveDeviceError::Internal(err) => Self::Internal(err),
        }
    }
}

impl From<SaveDeviceError> for crate::UpdateDeviceError {
    fn from(value: SaveDeviceError) -> Self {
        match value {
            SaveDeviceError::NotAFile { .. } => Self::InvalidPath(anyhow::anyhow!("{value}")),
            SaveDeviceError::NotADirectory { .. } => Self::InvalidPath(anyhow::anyhow!("{value}")),
            SaveDeviceError::NotFound { .. } => Self::InvalidPath(anyhow::anyhow!("{value}")),
            SaveDeviceError::CreateWritable { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::CannotEdit { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::Write { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::Close { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::DomException { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::Cast { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::Promise { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::NoSpaceLeft { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveDeviceError::RemoteOpaqueKeyUploadFailed(err) => {
                Self::RemoteOpaqueKeyOperationFailed(err)
            }
            SaveDeviceError::RemoteOpaqueKeyUploadOffline(err) => {
                Self::RemoteOpaqueKeyOperationOffline(err)
            }
            SaveDeviceError::Internal(err) => Self::Internal(err),
        }
    }
}

impl From<RemoveDeviceError> for crate::UpdateDeviceError {
    fn from(value: RemoveDeviceError) -> Self {
        match value {
            RemoveDeviceError::NotAFile { .. } => Self::InvalidPath(anyhow::anyhow!("{value}")),
            RemoveDeviceError::NotADirectory { .. } => {
                Self::InvalidPath(anyhow::anyhow!("{value}"))
            }
            RemoveDeviceError::Cast { .. } => Self::InvalidPath(anyhow::anyhow!("{value}")),
            RemoveDeviceError::Promise { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            RemoveDeviceError::DomException { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            RemoveDeviceError::NotFound { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            RemoveDeviceError::CreateWritable { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            RemoveDeviceError::CannotEdit { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            RemoveDeviceError::NoSpaceLeft { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            RemoveDeviceError::Write { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            RemoveDeviceError::Close { .. } => Self::Internal(anyhow::anyhow!("{value}")),
        }
    }
}

impl From<RemoveDeviceError> for crate::RemoveDeviceError {
    fn from(value: RemoveDeviceError) -> Self {
        match value {
            RemoveDeviceError::NotFound { .. } => Self::NotFound,
            _ => Self::Internal(anyhow::anyhow!("{value}")),
        }
    }
}

impl From<ArchiveDeviceError> for crate::ArchiveDeviceError {
    fn from(value: ArchiveDeviceError) -> Self {
        Self::Internal(anyhow::anyhow!("{value}"))
    }
}

impl From<SaveRawDataError> for crate::SavePkiLocalPendingError {
    fn from(value: SaveRawDataError) -> Self {
        match value {
            SaveRawDataError::NotAFile { .. } => Self::InvalidPath(anyhow::anyhow!("{value}")),
            SaveRawDataError::NotADirectory { .. } => Self::InvalidPath(anyhow::anyhow!("{value}")),
            SaveRawDataError::NotFound { .. } => Self::InvalidPath(anyhow::anyhow!("{value}")),
            SaveRawDataError::CreateWritable { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveRawDataError::CannotEdit { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveRawDataError::Write { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveRawDataError::Close { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveRawDataError::DomException { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveRawDataError::Cast { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveRawDataError::Promise { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            SaveRawDataError::NoSpaceLeft { .. } => Self::Internal(anyhow::anyhow!("{value}")),
        }
    }
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use libparsec_types::prelude::*;
use web_sys::wasm_bindgen::JsValue;

error_set::error_set! {
    DomExceptionError = {
        DomException {
            exception: web_sys::DomException
        }
    };
    CastError = {
        #[display("Failed to cast to {ty} ({value:?}")]
        Cast {
            ty: &'static str,
            value: JsValue
        }
    };
    AwaitPromiseError = {
        #[display("Failed to await JS promise ({error:?})")]
        Promise {
            error: JsValue
        }
    };
    JsPromiseError = CastError || AwaitPromiseError;
    NotFoundError = {
        #[display("No such file or directory at {}", path.display())]
        NotFound {
            path: PathBuf,
        }
    };
    GetRootDirectoryError = AwaitPromiseError || CastError || DomExceptionError || {
        #[display("Storage not available")]
        StorageNotAvailable { exception: web_sys::DomException }
    };
    GetDirectoryHandleError = CastError || AwaitPromiseError || DomExceptionError || NotFoundError || {
        #[display("Item at {} is not a directory", path.display())]
        NotADirectory {
            path: PathBuf,
        },
    };
    GetFileHandleError = CastError || AwaitPromiseError || DomExceptionError || NotFoundError || GetDirectoryHandleError || {
        #[display("Item at {} is not a directory", path.display())]
        NotAFile {
            path: PathBuf
        }
    };
    ReadToEndError = DomExceptionError
        || NotFoundError
        || CastError
        || AwaitPromiseError
        || {
        #[display("Failed to get file object")]
        GetFile { error: JsValue },
    };
    WriteAllError = CastError || NotFoundError || DomExceptionError || {
        CreateWritable { error: JsValue },
        CannotEdit {
            path: PathBuf,
            exception: web_sys::DomException
        },
        NoSpaceLeft { exception: web_sys::DomException },
        Write {
            error: JsValue
        },
        Close {
            error: JsValue
        }
    };
    NewStorageError = GetRootDirectoryError;
    ListAvailableDevicesError = GetDirectoryHandleError
        || AwaitPromiseError
        || {
            ReadToEnd(ReadToEndError)
        };
    DeviceMissingError = NotFoundError;
    RmpDecodeError = {
        RmpDecode(libparsec_types::RmpDecodeError)
    };
    LoadAvailableDeviceError = {
        ReadToEnd(ReadToEndError),
        RmpDecode(libparsec_types::RmpDecodeError)
    };
    InvalidPathError = {
        #[display("Invalid path {}", path.display())]
        InvalidPath {
            path: PathBuf
        }
    };
    ReadFile = {
        GetFile(GetFileHandleError),
        ReadFile(ReadToEndError),
    };
    SaveDeviceError = SaveDeviceFileError;
    SaveDeviceFileError = GetFileHandleError || WriteAllError;
    RemoveEntryError = NotFoundError || DomExceptionError || AwaitPromiseError || GetDirectoryHandleError;
    ArchiveDeviceError = RemoveEntryError || {
        GetDeviceToArchive(GetFileHandleError),
        ReadDeviceToArchive(ReadToEndError),
        CreateArchiveDevice(GetFileHandleError),
        WriteArchiveDevice(WriteAllError),
    };
    RemoveDeviceError = SaveDeviceError;
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
        Self::Internal(anyhow::anyhow!("{value}"))
    }
}

impl From<ArchiveDeviceError> for crate::ArchiveDeviceError {
    fn from(value: ArchiveDeviceError) -> Self {
        Self::Internal(anyhow::anyhow!("{value}"))
    }
}

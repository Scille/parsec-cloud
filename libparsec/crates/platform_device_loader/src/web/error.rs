// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::anyhow;
use web_sys::wasm_bindgen::JsValue;

#[derive(Debug, thiserror::Error)]
pub enum NewStorageError {
    #[error("No window context available, are we in a browser?")]
    NoWindow,
    #[error("No local storage available")]
    NoLocalStorage,
    #[error("Failed to access local storage ({:?})", .0)]
    WindowError(JsValue),
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
    crate::LoadDeviceError,
    crate::SaveDeviceError,
    crate::ChangeAuthenticationError,
    crate::ArchiveDeviceError,
    crate::RemoveDeviceError
);

error_set::error_set! {
    GetItemStorageError = {
        #[display("Failed to get item {key} from storage ({error:?})")]
        GetItemStorage {
            key: String,
            error: JsValue
        }
    };
    SetItemStorageError = {
        #[display("Failed to set item {key} to storage ({error:?})")]
        SetItemStorage {
            key: String,
            error: JsValue
        }
    };
    RemoveItemStorageError = {
        #[display("Failed to remove item {key} from storage ({error:?})")]
        RemoveItemStorage {
            key: String,
            error: JsValue
        }
    };
    JsonDeserializationError = {
        JsonDecode(serde_json::Error)
    };
    RmpDecodeError = {
        RmpDecode(libparsec_types::RmpDecodeError)
    };
    Base64DecodeError = {
        B64Decode(data_encoding::DecodeError)
    };
    ListAvailableDevicesError = GetItemStorageError
        || JsonDeserializationError
        || LoadAvailableDeviceError;
    DeviceMissingError = {
        #[display("No device for '{key}'")]
        Missing {
            key: String
        },
    };
    GetRawDeviceError = GetItemStorageError
        || Base64DecodeError
        || DeviceMissingError;
    LoadAvailableDeviceError = GetRawDeviceError || RmpDecodeError;
    InvalidPathError = {
        #[display("Invalid path {}", path.display())]
        InvalidPath {
            path: std::path::PathBuf
        }
    };
    LoadDeviceError = GetRawDeviceError
        || RmpDecodeError
        || {
            InvalidFileType,
            GetSecretKey(libparsec_crypto::CryptoError),
            DecryptAndLoad(crate::DecryptDeviceFileError),
        };
    SaveDeviceError = SaveDeviceFileError;
    SaveDeviceFileError = SetItemStorageError;
    ArchiveDeviceError = GetItemStorageError
        || SetItemStorageError
        || DeviceMissingError
        || RemoveItemStorageError
        || JsonDeserializationError;
    RemoveDeviceError = GetItemStorageError
        || RemoveItemStorageError
        || JsonDeserializationError;
}

impl From<LoadDeviceError> for crate::LoadDeviceError {
    fn from(value: LoadDeviceError) -> Self {
        match value {
            LoadDeviceError::InvalidFileType => Self::InvalidData,
            LoadDeviceError::GetSecretKey(_) => Self::DecryptionFailed,
            LoadDeviceError::DecryptAndLoad(e) => e.into(),
            LoadDeviceError::Missing { .. } => Self::InvalidPath(anyhow::anyhow!("{value}")),
            LoadDeviceError::GetItemStorage { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            LoadDeviceError::RmpDecode(_) => Self::InvalidData,
            LoadDeviceError::B64Decode(_) => Self::InvalidData,
        }
    }
}

impl From<SaveDeviceError> for crate::SaveDeviceError {
    fn from(value: SaveDeviceError) -> Self {
        match value {
            SaveDeviceError::SetItemStorage { .. } => Self::Internal(anyhow::anyhow!("{value}")),
        }
    }
}

impl From<LoadDeviceError> for crate::ChangeAuthenticationError {
    fn from(value: LoadDeviceError) -> Self {
        match value {
            LoadDeviceError::InvalidFileType => Self::InvalidData,
            LoadDeviceError::GetSecretKey(_) => Self::DecryptionFailed,
            LoadDeviceError::DecryptAndLoad(e) => e.into(),
            LoadDeviceError::Missing { .. } => Self::InvalidPath(anyhow::anyhow!("{value}")),
            LoadDeviceError::GetItemStorage { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            LoadDeviceError::RmpDecode(_) => Self::InvalidData,
            LoadDeviceError::B64Decode(_) => Self::InvalidData,
        }
    }
}

impl From<SaveDeviceError> for crate::ChangeAuthenticationError {
    fn from(value: SaveDeviceError) -> Self {
        match value {
            SaveDeviceError::SetItemStorage { .. } => Self::Internal(anyhow::anyhow!("{value}")),
        }
    }
}

impl From<RemoveDeviceError> for crate::ChangeAuthenticationError {
    fn from(value: RemoveDeviceError) -> Self {
        match value {
            RemoveDeviceError::GetItemStorage { .. } => Self::Internal(anyhow::anyhow!("{value}")),
            RemoveDeviceError::JsonDecode(_) => Self::InvalidData,
            RemoveDeviceError::RemoveItemStorage { .. } => {
                Self::Internal(anyhow::anyhow!("{value}"))
            }
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

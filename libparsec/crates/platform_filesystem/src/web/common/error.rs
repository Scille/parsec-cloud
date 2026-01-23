// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::ListFilesError;
use crate::LoadFileError;
use crate::SaveContentError;
use std::path::PathBuf;
use web_sys::wasm_bindgen::JsValue;

#[derive(Debug, thiserror::Error)]
pub enum CastError {
    #[error("Failed to cast to {ty} ({value:?}")]
    Cast { ty: &'static str, value: JsValue },
}

#[derive(Debug, thiserror::Error)]
pub enum GetDirectoryHandleError {
    #[error("Failed to cast to {ty} ({value:?}")]
    Cast { ty: &'static str, value: JsValue },
    #[error("Failed to await JS promise ({0:?})")]
    Promise(JsValue),
    #[error("DOM exception: {0:?}")]
    DomException(web_sys::DomException),
    #[error("Item at {0:?} is not a directory")]
    NotADirectory(PathBuf),
    #[error("No such file or directory at {0:?}")]
    NotFound(PathBuf),
}

pub type ListFileEntriesError = GetDirectoryHandleError;

#[derive(Debug, thiserror::Error)]
pub enum GetFileHandleError {
    #[error("Failed to cast to {ty} ({value:?}")]
    Cast { ty: &'static str, value: JsValue },
    #[error("Failed to await JS promise ({0:?})")]
    Promise(JsValue),
    #[error("DOM exception: {0:?}")]
    DomException(web_sys::DomException),
    #[error("Item at {0:?} is not a directory")]
    NotAFile(PathBuf),
    #[error("No such file or directory at {0:?}")]
    NotFound(PathBuf),
    #[error("Item at {0:?} is not a directory")]
    NotADirectory(PathBuf),
}

#[derive(Debug, thiserror::Error)]
pub enum GetRootDirectoryError {
    #[error("Failed to cast to {ty} ({value:?}")]
    Cast { ty: &'static str, value: JsValue },
    #[error("DOM exception: {0:?}")]
    DomException(web_sys::DomException),
    #[error("Storage not available ({0:?})")]
    StorageNotAvailable(web_sys::DomException),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadToEndError {
    #[error("Failed to cast to {ty} ({value:?}")]
    Cast { ty: &'static str, value: JsValue },
    #[error("Failed to await JS promise ({0:?})")]
    Promise(JsValue),
    #[error("Failed to get file object ({0:?})")]
    GetFile(JsValue),
    #[error("DOM exception: {0:?}")]
    DomException(web_sys::DomException),
    #[error("No such file or directory at {0:?}")]
    NotFound(PathBuf),
}

#[derive(Debug, thiserror::Error)]
pub enum RemoveEntryError {
    #[error("Failed to cast to {ty} ({value:?}")]
    Cast { ty: &'static str, value: JsValue },
    #[error("Failed to await JS promise ({0:?})")]
    Promise(JsValue),
    #[error("DOM exception: {0:?}")]
    DomException(web_sys::DomException),
    #[error("Item at {0:?} is not a directory")]
    NotADirectory(PathBuf),
    #[error("No such file or directory at {0:?}")]
    NotFound(PathBuf),
}

#[derive(Debug, thiserror::Error)]
pub enum WriteAllError {
    #[error("Close error ({0:?})")]
    Close(JsValue),
    #[error("Write error ({0:?})")]
    Write(JsValue),
    #[error("DOM exception: {0:?}")]
    DomException(web_sys::DomException),
    #[error("Failed to cast to {ty} ({value:?}")]
    Cast { ty: &'static str, value: JsValue },
    #[error("Failed to create writable stream ({0:?})")]
    CreateWritable(JsValue),
    #[error("No space left ({0:?})")]
    NoSpaceLeft(web_sys::DomException),
    #[error("Cannot edit resource `{path:?}` ({exception:?})")]
    CannotEdit {
        path: PathBuf,
        exception: web_sys::DomException,
    },
    #[error("No such file or directory at {0:?}")]
    NotFound(PathBuf),
}

pub type NewStorageError = GetRootDirectoryError;

#[derive(Debug, thiserror::Error)]
pub enum ReadFileError {
    #[error("Failed to cast to {ty} ({value:?}")]
    Cast { ty: &'static str, value: JsValue },
    #[error("Failed to await JS promise ({0:?})")]
    Promise(JsValue),
    #[error("DOM exception: {0:?}")]
    DomException(web_sys::DomException),
    #[error("Item at {0:?} is not a directory")]
    NotAFile(PathBuf),
    #[error("No such file or directory at {0:?}")]
    NotFound(PathBuf),
    #[error("Item at {0:?} is not a directory")]
    NotADirectory(PathBuf),
    #[error("Failed to get file object ({0:?})")]
    GetFile(JsValue),
}

impl From<ReadFileError> for LoadFileError {
    fn from(value: ReadFileError) -> Self {
        match value {
            ReadFileError::Cast { .. }
            | ReadFileError::DomException(..)
            | ReadFileError::Promise(..)
            | ReadFileError::GetFile(..) => LoadFileError::Internal(anyhow::anyhow!("{value}")),
            ReadFileError::NotAFile(..) => LoadFileError::NotAFile,
            ReadFileError::NotFound(..) => LoadFileError::NotFound,
            ReadFileError::NotADirectory(..) => LoadFileError::InvalidParent,
        }
    }
}

impl From<ReadToEndError> for ReadFileError {
    fn from(value: ReadToEndError) -> Self {
        match value {
            ReadToEndError::Cast { ty, value } => ReadFileError::Cast { ty, value },
            ReadToEndError::Promise(e) => ReadFileError::Promise(e),
            ReadToEndError::DomException(e) => ReadFileError::DomException(e),
            ReadToEndError::NotFound(e) => ReadFileError::NotFound(e),
            ReadToEndError::GetFile(e) => ReadFileError::GetFile(e),
        }
    }
}

impl From<GetFileHandleError> for ReadFileError {
    fn from(value: GetFileHandleError) -> Self {
        match value {
            GetFileHandleError::Cast { ty, value } => ReadFileError::Cast { ty, value },
            GetFileHandleError::Promise(e) => ReadFileError::Promise(e),
            GetFileHandleError::DomException(e) => ReadFileError::DomException(e),
            GetFileHandleError::NotADirectory(e) => ReadFileError::NotADirectory(e),
            GetFileHandleError::NotFound(e) => ReadFileError::NotFound(e),
            GetFileHandleError::NotAFile(e) => ReadFileError::NotAFile(e),
        }
    }
}

impl From<GetDirectoryHandleError> for GetFileHandleError {
    fn from(e: GetDirectoryHandleError) -> Self {
        match e {
            GetDirectoryHandleError::Cast { ty, value } => GetFileHandleError::Cast { ty, value },
            GetDirectoryHandleError::Promise(e) => GetFileHandleError::Promise(e),
            GetDirectoryHandleError::DomException(e) => GetFileHandleError::DomException(e),
            GetDirectoryHandleError::NotADirectory(e) => GetFileHandleError::NotADirectory(e),
            GetDirectoryHandleError::NotFound(e) => GetFileHandleError::NotFound(e),
        }
    }
}
impl From<GetDirectoryHandleError> for RemoveEntryError {
    fn from(e: GetDirectoryHandleError) -> Self {
        match e {
            GetDirectoryHandleError::Cast { ty, value } => RemoveEntryError::Cast { ty, value },
            GetDirectoryHandleError::Promise(e) => RemoveEntryError::Promise(e),
            GetDirectoryHandleError::DomException(e) => RemoveEntryError::DomException(e),
            GetDirectoryHandleError::NotADirectory(e) => RemoveEntryError::NotADirectory(e),
            GetDirectoryHandleError::NotFound(e) => RemoveEntryError::NotFound(e),
        }
    }
}

impl From<WriteAllError> for SaveContentError {
    fn from(e: WriteAllError) -> Self {
        match e {
            WriteAllError::Cast { .. }
            | WriteAllError::DomException(..)
            | WriteAllError::Close(..)
            | WriteAllError::Write(..)
            | WriteAllError::CreateWritable(..) => {
                SaveContentError::Internal(anyhow::anyhow!("{e}"))
            }
            WriteAllError::NoSpaceLeft(..) => SaveContentError::NoSpaceLeft,
            WriteAllError::CannotEdit { .. } => SaveContentError::CannotEdit,
            WriteAllError::NotFound(..) => unreachable!("file should be created if not existing ?"),
        }
    }
}

impl From<GetFileHandleError> for SaveContentError {
    fn from(e: GetFileHandleError) -> Self {
        match e {
            GetFileHandleError::Cast { .. }
            | GetFileHandleError::DomException(..)
            | GetFileHandleError::Promise(..) => SaveContentError::Internal(anyhow::anyhow!("{e}")),
            GetFileHandleError::NotAFile(..) => SaveContentError::InvalidPath,
            GetFileHandleError::NotFound(..) => {
                unreachable!("file should be created if not existing ?")
            }
            GetFileHandleError::NotADirectory(..) => SaveContentError::InvalidParent,
        }
    }
}

impl From<GetDirectoryHandleError> for SaveContentError {
    fn from(e: GetDirectoryHandleError) -> Self {
        match e {
            GetDirectoryHandleError::Cast { .. }
            | GetDirectoryHandleError::DomException(..)
            | GetDirectoryHandleError::Promise(..) => {
                SaveContentError::Internal(anyhow::anyhow!("{e}"))
            }
            GetDirectoryHandleError::NotADirectory(..) => SaveContentError::InvalidParent,
            GetDirectoryHandleError::NotFound(..) => SaveContentError::ParentNotFound,
        }
    }
}

impl From<GetRootDirectoryError> for SaveContentError {
    fn from(e: GetRootDirectoryError) -> Self {
        match e {
            GetRootDirectoryError::Cast { .. } | GetRootDirectoryError::DomException(..) => {
                SaveContentError::Internal(anyhow::anyhow!("{e}"))
            }
            GetRootDirectoryError::StorageNotAvailable { .. } => {
                SaveContentError::StorageNotAvailable
            }
        }
    }
}

impl From<ListFileEntriesError> for ListFilesError {
    fn from(e: GetDirectoryHandleError) -> Self {
        match e {
            GetDirectoryHandleError::Cast { .. }
            | GetDirectoryHandleError::DomException(..)
            | GetDirectoryHandleError::Promise(..)
            | GetDirectoryHandleError::NotFound(..) => {
                ListFilesError::Internal(anyhow::anyhow!("{e}"))
            }
            GetDirectoryHandleError::NotADirectory(..) => ListFilesError::InvalidParent,
        }
    }
}

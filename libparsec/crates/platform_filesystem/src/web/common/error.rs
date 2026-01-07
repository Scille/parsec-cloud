// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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

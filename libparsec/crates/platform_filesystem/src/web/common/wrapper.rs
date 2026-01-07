// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    ffi::OsStr,
    ops::Deref,
    path::{Path, PathBuf},
};

use libparsec_platform_async::{
    future::{self, FutureExt},
    stream::{Stream, StreamExt},
};
use wasm_bindgen_futures::{stream::JsStream, JsFuture};
use web_sys::{js_sys, wasm_bindgen::JsCast, DomException};

use super::error::{
    CastError, GetDirectoryHandleError, GetFileHandleError, GetRootDirectoryError, ReadToEndError,
    RemoveEntryError, WriteAllError,
};

#[derive(Clone)]
pub struct Directory {
    pub path: PathBuf,
    pub handle: web_sys::FileSystemDirectoryHandle,
}

impl Directory {
    pub async fn get_root() -> Result<Self, GetRootDirectoryError> {
        let storage_manager: web_sys::StorageManager = if let Some(window) = web_sys::window() {
            Ok(window.navigator().storage())
        } else {
            // `window` is not available, at that point we consider that we are in a shared
            // worker.

            // `WorkerGlobalScope` is available as a global
            // https://github.com/rustwasm/wasm-bindgen/issues/1003
            js_sys::global()
                .dyn_into::<web_sys::WorkerGlobalScope>()
                .map(|scope| scope.navigator().storage())
                .map_err(|e| GetRootDirectoryError::Cast {
                    ty: stringify!(web_sys::WorkerGlobalScope),
                    value: e.into(),
                })
        }?;
        JsFuture::from(storage_manager.get_directory())
            .await
            .map_err(|e| {
                e.dyn_into::<web_sys::DomException>()
                    .map(|e| {
                        if e.code() == web_sys::DomException::SECURITY_ERR {
                            GetRootDirectoryError::StorageNotAvailable(e)
                        } else {
                            GetRootDirectoryError::DomException(e)
                        }
                    })
                    .unwrap_or_else(|e| GetRootDirectoryError::Cast {
                        ty: stringify!(web_sys::DomException),
                        value: e,
                    })
            })
            .and_then(|raw| {
                raw.dyn_into::<web_sys::FileSystemDirectoryHandle>()
                    .map_err(|e| GetRootDirectoryError::Cast {
                        ty: stringify!(web_sys::FileSystemDirectoryHandle),
                        value: e,
                    })
            })
            .map(|dir| Self::new(dir, "/".as_ref()))
    }

    fn new(handle: web_sys::FileSystemDirectoryHandle, parent: &Path) -> Self {
        Self {
            path: parent.join(handle.name()),
            handle,
        }
    }

    pub async fn get_directory_from_path(
        &self,
        path: &Path,
        option: Option<OpenOptions>,
    ) -> Result<Self, GetDirectoryHandleError> {
        log::trace!("Get directory handle {} from path", path.display());
        debug_assert!(path.has_root());
        let Some(dirname) = path.file_name().and_then(std::ffi::OsStr::to_str) else {
            // no filename mean `/`, so we return self.
            return Ok(self.clone());
        };
        let dir = self
            .get_parent_directory(path.parent(), option.clone())
            .await?;
        dir.as_ref()
            .unwrap_or(self)
            .get_directory(dirname, option)
            .await
    }

    async fn get_parent_directory(
        &self,
        parent: Option<&Path>,
        option: Option<OpenOptions>,
    ) -> Result<Option<Self>, GetDirectoryHandleError> {
        if let Some(parent) = parent {
            let dir = self
                .get_directory_from_path(parent, option.clone())
                .boxed_local()
                .await?;
            Ok(Some(dir))
        } else {
            Ok(None)
        }
    }

    pub async fn create_dir_all(&self, path: &Path) -> Result<Self, GetDirectoryHandleError> {
        self.get_directory_from_path(path, Some(OpenOptions::create()))
            .await
    }

    pub async fn get_directory(
        &self,
        dirname: &str,
        option: Option<OpenOptions>,
    ) -> Result<Directory, GetDirectoryHandleError> {
        log::trace!(
            "Get directory handle `{dirname}` in {}",
            self.path.display()
        );
        let promise = if let Some(option) = option {
            self.handle
                .get_directory_handle_with_options(dirname, &option.into())
        } else {
            self.handle.get_directory_handle(dirname)
        };
        JsFuture::from(promise)
            .await
            .map_err(|e| match e.dyn_into::<web_sys::DomException>() {
                Ok(exception) => match exception.code() {
                    web_sys::DomException::NOT_FOUND_ERR => {
                        GetDirectoryHandleError::NotFound(self.path.join(dirname))
                    }
                    web_sys::DomException::TYPE_MISMATCH_ERR => {
                        GetDirectoryHandleError::NotADirectory(self.path.join(dirname))
                    }

                    _ => GetDirectoryHandleError::DomException(exception),
                },
                Err(e) => GetDirectoryHandleError::Promise(e),
            })
            .and_then(|raw| {
                raw.dyn_into::<web_sys::FileSystemDirectoryHandle>()
                    .map_err(|e| GetDirectoryHandleError::Cast {
                        ty: stringify!(web_sys::FileSystemDirectoryHandle),
                        value: e,
                    })
            })
            .map(|v| Self::new(v, &self.path))
            .inspect(|_v| log::trace!("Found handle `{dirname}` in {}", self.path.display()))
    }

    pub async fn get_file_from_path(
        &self,
        path: &Path,
        option: Option<OpenOptions>,
    ) -> Result<File, GetFileHandleError> {
        log::trace!("Get file handle from path {}", path.display());
        debug_assert!(path.has_root());
        let filename = path
            .file_name()
            .and_then(OsStr::to_str)
            .expect("Missing filename");
        let dir = self.get_parent_directory(path.parent(), None).await?;
        dir.as_ref()
            .unwrap_or(self)
            .get_file(filename, option)
            .await
    }

    pub async fn get_file(
        &self,
        filename: &str,
        option: Option<OpenOptions>,
    ) -> Result<File, GetFileHandleError> {
        log::trace!("Get file handle `{filename}` in {}", self.path.display());
        let promise = if let Some(option) = option {
            self.handle
                .get_file_handle_with_options(filename, &option.into())
        } else {
            self.handle.get_file_handle(filename)
        };
        JsFuture::from(promise)
            .await
            .map_err(|e| match e.dyn_into::<web_sys::DomException>() {
                Ok(exception) => match exception.code() {
                    web_sys::DomException::NOT_FOUND_ERR => {
                        GetFileHandleError::NotFound(self.path.join(filename))
                    }
                    web_sys::DomException::TYPE_MISMATCH_ERR => {
                        GetFileHandleError::NotAFile(self.path.join(filename))
                    }
                    _ => GetFileHandleError::DomException(exception),
                },
                Err(e) => GetFileHandleError::Promise(e),
            })
            .and_then(|raw| {
                raw.dyn_into::<web_sys::FileSystemFileHandle>()
                    .map_err(|e| GetFileHandleError::Cast {
                        ty: stringify!(web_sys::FileSystemFileHandle),
                        value: e,
                    })
            })
            .map(|v| File::new(v, &self.path))
            .inspect(|_v| log::trace!("Found handle `{filename}` in {}", self.path.display()))
    }

    pub fn entries(&self) -> impl Stream<Item = DirEntry> + use<'_> {
        log::trace!("Listing entries at {}", self.path.display());
        JsStream::from(self.handle.values()).filter_map(|v| {
            log::trace!("Entries {v:?}");
            future::ready(
                v.ok()
                    .and_then(|v| {
                        v.dyn_into::<web_sys::FileSystemFileHandle>()
                            .map(DirOrFileHandle::File)
                            .or_else(|v| {
                                v.dyn_into::<web_sys::FileSystemDirectoryHandle>()
                                    .map(DirOrFileHandle::Dir)
                            })
                            .inspect_err(|e| {
                                log::warn!("{e:?} is neither a file or directory handle")
                            })
                            .ok()
                    })
                    .map(|v| DirEntry::new(v, &self.path)),
            )
        })
    }

    pub async fn remove_entry_from_path(&self, path: &Path) -> Result<(), RemoveEntryError> {
        log::trace!("Remove entry `{}` from path", path.display());
        debug_assert!(path.has_root());
        let entryname = path
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or(RemoveEntryError::NotFound(path.to_path_buf()))?;
        let dir = self.get_parent_directory(path.parent(), None).await?;
        dir.as_ref().unwrap_or(self).remove_entry(entryname).await
    }

    pub async fn remove_entry(&self, entryname: &str) -> Result<(), RemoveEntryError> {
        JsFuture::from(self.handle.remove_entry(entryname))
            .await
            .map_err(|e| match e.dyn_into::<web_sys::DomException>() {
                Ok(exception) => match exception.code() {
                    web_sys::DomException::NOT_FOUND_ERR => {
                        RemoveEntryError::NotFound(entryname.into())
                    }

                    _ => RemoveEntryError::DomException(exception),
                },
                Err(e) => RemoveEntryError::Promise(e),
            })
            .and(Ok(()))
    }
}

#[derive(Debug, Clone, Default)]
pub struct OpenOptions {
    create: bool,
}

impl OpenOptions {
    pub const fn create() -> Self {
        Self { create: true }
    }
}

impl From<OpenOptions> for web_sys::FileSystemGetDirectoryOptions {
    fn from(value: OpenOptions) -> Self {
        let v = Self::new();
        v.set_create(value.create);
        v
    }
}

impl From<OpenOptions> for web_sys::FileSystemGetFileOptions {
    fn from(value: OpenOptions) -> Self {
        let v = Self::new();
        v.set_create(value.create);
        v
    }
}

pub struct DirEntry {
    pub path: PathBuf,
    pub handle: DirOrFileHandle,
}

impl DirEntry {
    fn new(handle: DirOrFileHandle, parent: &Path) -> Self {
        Self {
            path: parent.join(handle.name()),
            handle,
        }
    }
}

impl TryFrom<DirEntry> for File {
    type Error = CastError;

    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        match value.handle {
            DirOrFileHandle::File(handle) => Ok(Self {
                path: value.path,
                handle,
            }),
            DirOrFileHandle::Dir(d) => Err(CastError::Cast {
                ty: stringify!(web_sys::FileSystemFileHandle),
                value: d.into(),
            }),
        }
    }
}

pub enum DirOrFileHandle {
    Dir(web_sys::FileSystemDirectoryHandle),
    File(web_sys::FileSystemFileHandle),
}

impl DirOrFileHandle {
    pub fn name(&self) -> String {
        match self {
            Self::Dir(v) => v.name(),
            Self::File(v) => v.name(),
        }
    }
}

pub struct File {
    pub path: PathBuf,
    pub handle: web_sys::FileSystemFileHandle,
}

impl File {
    fn new(handle: web_sys::FileSystemFileHandle, parent: &Path) -> Self {
        Self {
            path: parent.join(handle.name()),
            handle,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub async fn read_to_end(&self) -> Result<Vec<u8>, ReadToEndError> {
        let file = JsFuture::from(self.handle.get_file())
            .await
            .map_err(|e| match e.dyn_into::<web_sys::DomException>() {
                Ok(except) if except.code() == web_sys::DomException::NOT_FOUND_ERR => {
                    ReadToEndError::NotFound(self.path.clone())
                }

                Ok(exception) => ReadToEndError::DomException(exception),
                Err(e) => ReadToEndError::GetFile(e),
            })
            .and_then(|v| {
                v.dyn_into::<web_sys::File>()
                    .map_err(|e| ReadToEndError::Cast {
                        ty: stringify!(web_sys::File),
                        value: e,
                    })
            })?;
        let blob = file.deref();
        let array_buffer = JsFuture::from(blob.array_buffer())
            .await
            .map_err(|e| ReadToEndError::Promise(e))
            .inspect_err(|e| log::error!("Failed to get blob array buffer: {e:?}"))
            .and_then(|v| {
                v.dyn_into::<js_sys::ArrayBuffer>()
                    .map_err(|e| ReadToEndError::Cast {
                        ty: stringify!(js_sys::ArrayBuffer),
                        value: e,
                    })
            })?;
        let u8array = js_sys::Uint8Array::new(array_buffer.as_ref());
        Ok(u8array.to_vec())
    }

    pub async fn write_all(&self, data: &[u8]) -> Result<(), WriteAllError> {
        let stream = JsFuture::from(self.handle.create_writable())
            .await
            .map_err(|e| match e.dyn_into::<web_sys::DomException>() {
                Ok(exception) => match exception.code() {
                    web_sys::DomException::NOT_FOUND_ERR => {
                        WriteAllError::NotFound(self.path.clone())
                    }
                    web_sys::DomException::NO_MODIFICATION_ALLOWED_ERR => {
                        WriteAllError::CannotEdit {
                            path: self.path.clone(),
                            exception,
                        }
                    }
                    _ => WriteAllError::DomException(exception),
                },
                Err(e) => WriteAllError::CreateWritable(e),
            })
            .and_then(|v| {
                v.dyn_into::<web_sys::FileSystemWritableFileStream>()
                    .map_err(|e| WriteAllError::Cast {
                        ty: stringify!(web_sys::FileSystemWritableFileStream),
                        value: e,
                    })
            })?;
        stream
            .write_with_u8_array(data)
            .map(wasm_bindgen_futures::JsFuture::from)
            .map_err(|e| match e.dyn_into::<DomException>() {
                Ok(exception) => match exception.code() {
                    web_sys::DomException::QUOTA_EXCEEDED_ERR => {
                        WriteAllError::NoSpaceLeft(exception)
                    }
                    _ => WriteAllError::DomException(exception),
                },
                Err(e) => WriteAllError::Write(e),
            })?
            .await
            .map_err(|e| WriteAllError::Write(e))?;
        wasm_bindgen_futures::JsFuture::from(stream.close())
            .await
            .map_err(|e| WriteAllError::Close(e))
            .and(Ok(()))
    }
}

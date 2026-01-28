// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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

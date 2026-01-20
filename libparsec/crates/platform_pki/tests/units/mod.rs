// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(target_os = "windows")] // TODO: libparsec_platform_pki only supports Windows so far
mod encrypt;
#[cfg(target_os = "windows")] // TODO: libparsec_platform_pki only supports Windows so far
mod list;
mod shared;
#[cfg(target_os = "windows")] // TODO: libparsec_platform_pki only supports Windows so far
mod sign;
mod utils;
mod x509;

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod encrypt;
#[cfg(target_os = "windows")] // TODO: libparsec_platform_pki only supports Windows so far
mod list;
mod shared;
mod sign;
#[cfg(feature = "test-with-testbed")]
mod testbed;
#[cfg(windows)] // TODO: libparsec_platform_pki only supports Windows so far
mod uri;
mod utils;
mod x509;

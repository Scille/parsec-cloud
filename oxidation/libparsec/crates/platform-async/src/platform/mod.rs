// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

#[cfg(not(target_arch = "wasm32"))]
mod default;
#[cfg(not(target_arch = "wasm32"))]
use default as platform;

pub use platform::{spawn, Task};

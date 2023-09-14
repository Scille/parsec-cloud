// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub enum Platform {
    Windows,
    Linux,
    MacOS,
    Android,
    Web,
}

pub const fn get_platform() -> Platform {
    match std::env::consts::OS.as_bytes() {
        b"linux" => Platform::Linux,
        b"macos" => Platform::MacOS,
        b"windows" => Platform::Windows,
        b"android" => Platform::Android,
        _ => {
            #[cfg(target_arch = "wasm32")]
            return Platform::Web;
            #[cfg(not(target_arch = "wasm32"))]
            panic!("Unknown platform");
        }
    }
}

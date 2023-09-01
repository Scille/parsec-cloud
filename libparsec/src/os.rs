// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub enum OS {
    Windows,
    Linux,
    MacOs,
    Android,
}

pub const fn get_os() -> OS {
    match std::env::consts::OS.as_bytes() {
        b"linux" => OS::Linux,
        b"macos" => OS::MacOs,
        b"windows" => OS::Windows,
        b"android" => OS::Android,
        _ => {
            #[cfg(target_arch = "wasm32")]
            panic!("Cannot detect current OS in web");
            #[cfg(not(target_arch = "wasm32"))]
            panic!("Unknown os");
        }
    }
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#[cfg(target_arch = "wasm32")]
libparsec_tests_lite::platform::wasm_bindgen_test_configure!(run_in_browser run_in_shared_worker);

const CONTENT: &[u8] =  b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed porta augue ante. Morbi molestie sapien eget nisi aliquet, ut commodo turpis venenatis. Maecenas porttitor mauris sapien, at gravida dui euismod et.";

mod list;
mod load;
mod remove;
mod rename;
mod save;

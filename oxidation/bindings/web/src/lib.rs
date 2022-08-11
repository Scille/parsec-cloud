// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use libparsec_bindings_common::Cmd;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Deserialize)]
struct Input {
    cmd: String,
    payload: String,
}

#[derive(Serialize)]
struct Output {
    value: String,
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn submitJob(input: JsValue) -> Promise {
    let options = input.into_serde::<Input>().unwrap();
    let cmd = options.cmd;
    let payload = options.payload;

    let res = Cmd::decode(&cmd, &payload).map(Cmd::execute);

    match res {
        Ok(Ok(data)) => Promise::resolve(&JsValue::from_serde(&Output { value: data }).unwrap()),
        Ok(Err(err)) | Err(err) => {
            Promise::reject(&JsValue::from_serde(&Output { value: err }).unwrap())
        }
    }
}

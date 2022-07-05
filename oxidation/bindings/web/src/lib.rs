// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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

    let res = libparsec_bindings_common::decode_and_execute(&cmd, &payload);

    match res {
        Ok(data) => Promise::resolve(&JsValue::from_serde(&Output { value: data }).unwrap()),
        Err(err) => Promise::reject(&JsValue::from_serde(&Output { value: err }).unwrap()),
    }
}

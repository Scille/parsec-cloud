// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use neon::prelude::*;
use std::sync::Mutex;

mod meths;

lazy_static::lazy_static! {
    static ref TOKIO_RUNTIME: Mutex<tokio::runtime::Runtime> =
        Mutex::new(tokio::runtime::Runtime::new().expect("Cannot start tokio runtime for libparsec"));
}

#[neon::main]
pub fn main(mut cx: ModuleContext) -> NeonResult<()> {
    meths::register_meths(&mut cx)
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use neon::prelude::*;
use std::sync::Mutex;

use libparsec::TokioRuntime;

mod meths;

lazy_static::lazy_static! {
    static ref TOKIO_RUNTIME: Mutex<TokioRuntime> =
        Mutex::new(TokioRuntime::new().expect("Cannot start tokio runtime for libparsec"));
}

#[neon::main]
pub fn main(mut cx: ModuleContext) -> NeonResult<()> {
    meths::register_meths(&mut cx)
}

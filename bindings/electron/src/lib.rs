// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use neon::prelude::*;
use std::{backtrace::Backtrace, sync::Mutex};

mod meths;

lazy_static::lazy_static! {
    static ref TOKIO_RUNTIME: Mutex<tokio::runtime::Runtime> =
        Mutex::new(tokio::runtime::Runtime::new().expect("Cannot start tokio runtime for libparsec"));
}

static BACKTRACE: Mutex<Option<Backtrace>> = Mutex::new(None);

macro_rules! capture_backtrace {
    ($tx: ident, $($tt: tt)*) => {
        {
            std::panic::set_hook(Box::new(|_| {
                *crate::BACKTRACE.lock().expect("Mutex is poisoned") = Some(std::backtrace::Backtrace::capture());
            }));

            let ret = $($tt)*;

            let _ = std::panic::take_hook();

            let _ = $tx.send(());

            ret
        }
    };
}

macro_rules! handle_backtrace {
    ($rx: ident, $cx: ident) => {
        let _ = $rx.recv();

        if let Some(backtrace) = crate::BACKTRACE.lock().expect("Mutex is poisoned").take() {
            return $cx.throw_error(format!("Program panicked:\n{backtrace}"));
        }
    };
}

pub(crate) use capture_backtrace;
pub(crate) use handle_backtrace;

#[neon::main]
pub fn main(mut cx: ModuleContext) -> NeonResult<()> {
    meths::register_meths(&mut cx)
}

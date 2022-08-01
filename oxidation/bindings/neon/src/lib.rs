// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use neon::prelude::*;
use std::sync::Mutex;

use libparsec_bindings_common::Cmd;

lazy_static::lazy_static! {
    static ref LIBPARSEC_CTX: Mutex<libparsec_bindings_common::RuntimeContext> =
        Mutex::new(libparsec_bindings_common::create_context());
}

/// Interface between Web, Rust and Native
/// Cordova bridge API for Android requests the inputs and output to be passed as a JS Object
/// hence we have to comply with this ourself to provide the same API accross plateforms
///
/// Input:
///   - cmd: String
///   - payload: Base64 String separated by ':'
///
/// Output:
///   - value: Base64 String
fn submit_job(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let options = cx.argument::<JsObject>(0)?;
    let cmd = options
        .get::<JsString, _, _>(&mut cx, "cmd")?
        .value(&mut cx);
    let payload = options
        .get::<JsString, _, _>(&mut cx, "payload")?
        .value(&mut cx);

    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    let submitted = LIBPARSEC_CTX.lock().unwrap().submit_job(Box::new(move || {
        // The callback is being called from the libparsec thread, so
        // we send closure as a task to be executed by the JavaScript event
        // loop. This _will_ block the event loop while executing.
        let res = Cmd::decode(&cmd, &payload).map(Cmd::execute);

        channel.send(move |mut cx| {
            match res {
                Ok(Ok(data)) => {
                    let arg = cx.empty_object();
                    let arg_value = cx.string(data);
                    arg.set(&mut cx, "value", arg_value)?;
                    deferred.resolve(&mut cx, arg);
                }
                Ok(Err(err)) | Err(err) => {
                    let arg = cx.empty_object();
                    let arg_value = cx.string(err);
                    arg.set(&mut cx, "value", arg_value)?;
                    deferred.reject(&mut cx, arg);
                }
            };

            Ok(())
        });
    }));

    match submitted {
        Ok(_) => Ok(promise),
        Err(err) => cx.throw_error(err),
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("submitJob", submit_job)?;
    Ok(())
}

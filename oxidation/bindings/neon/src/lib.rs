#[macro_use]
extern crate lazy_static;

use neon::{prelude::*};
use std::sync::Mutex;

fn submit_job(mut cx: FunctionContext) -> JsResult<JsPromise> {
    // Cordova bridge API for Android requests the params to be passed as a JS Object,
    // hence we have to comply with this ourself to provide the same API accross plateforms.
    let options = cx.argument::<JsObject>(0)?;
    let cmd: Handle<JsString> = options.get(&mut cx, "cmd")?;
    let cmd = cmd.value(&mut cx);
    let payload: Handle<JsString> = options.get(&mut cx, "payload")?;
    let payload = payload.value(&mut cx);

    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    lazy_static! {
        static ref LIBPARSEC_CTX: Mutex<libparsec_bindings_common::RuntimeContext> = Mutex::new(libparsec_bindings_common::create_context());
    }

    let submitted = LIBPARSEC_CTX.lock().unwrap().submit_job(Box::new(move || {
        // The callback is being called from the libparsec thread, so
        // we send closure as a task to be executed by the JavaScript event
        // loop. This _will_ block the event loop while executing.
        let res = libparsec_bindings_common::decode_and_execute(&cmd, &payload);

        channel.send(move |mut cx| {

            // Cordova bridge API for Android requests the return value to
            // be a JS Object, hence we have to comply with this ourself to
            // provide the same API accross plateforms.
            fn _build_arg<'a>(cx: &mut TaskContext<'a>, value: &str) -> JsResult<'a, JsObject> {
                let arg = cx.empty_object();
                let arg_value = cx.string(value);
                arg.set(cx, "value", arg_value)?;
                Ok(arg)
            }

            match res {
                Ok(data) => {
                    let arg = _build_arg(&mut cx, &data)?;
                    deferred.resolve(&mut cx, arg);
                },
                Err(err) => {
                    let arg = _build_arg(&mut cx, &err)?;
                    deferred.reject(&mut cx, arg);
                },
            };

            Ok(())
        });

    }));

    match submitted {
        Ok(_) => Ok(promise),
        Err(err) => cx.throw_error(err)
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("submitJob", submit_job)?;
    Ok(())
}

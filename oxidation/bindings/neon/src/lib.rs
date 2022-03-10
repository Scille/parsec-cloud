#[macro_use]
extern crate lazy_static;

use neon::prelude::*;
use std::sync::Mutex;

fn submit_job(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let success = cx.argument::<JsFunction>(0)?.root(&mut cx);
    let error = cx.argument::<JsFunction>(1)?.root(&mut cx);
    let cmd = cx.argument::<JsString>(2)?.value(&mut cx).to_owned();
    let payload = cx.argument::<JsString>(3)?.value(&mut cx).to_owned();
    let channel = cx.channel();

    lazy_static! {
        static ref LIBPARSEC_CTX: Mutex<libparsec_bindings_common::RuntimeContext> = Mutex::new(libparsec_bindings_common::create_context());
    }

    let submitted = LIBPARSEC_CTX.lock().unwrap().submit_job(Box::new(move || {
        // The callback is being called from the libparsec thread, so
        // we send closure as a task to be executed by the JavaScript event
        // loop. This _will_ block the event loop while executing.
        let res = libparsec_bindings_common::decode_and_execute(&cmd, &payload);

        channel.send(move |mut cx| {
            let (callback, arg) = match res {
                Ok(data) => (success, data),
                Err(err) => (error, err),
            };
            let callback = callback.into_inner(&mut cx);
            let this = cx.undefined();
            let args = vec![cx.string(arg).upcast::<JsValue>()];
            callback.call(&mut cx, this, args)?;
            Ok(())
        });

    }));

    match submitted {
        Ok(_) => Ok(cx.undefined()),
        Err(err) => cx.throw_error(err)
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("submitJob", submit_job)?;
    Ok(())
}

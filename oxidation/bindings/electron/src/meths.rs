// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

#[allow(unused_imports)]
use neon::{prelude::*, types::buffer::TypedArray};

// HelloError

#[allow(dead_code)]
fn variant_helloerror_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::HelloError> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "EmptySubject" => Ok(libparsec::HelloError::EmptySubject {}),
        "YouAreADog" => {
            let hello = {
                let js_val: Handle<JsString> = obj.get(cx, "hello")?;
                js_val.value(cx)
            };
            Ok(libparsec::HelloError::YouAreADog { hello })
        }
        _ => cx.throw_type_error("Object is not a HelloError"),
    }
}

#[allow(dead_code)]
fn variant_helloerror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::HelloError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::HelloError::EmptySubject {} => {
            let js_tag = JsString::try_new(cx, "EmptySubject").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::HelloError::YouAreADog { hello } => {
            let js_tag = JsString::try_new(cx, "YouAreADog").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_hello = JsString::try_new(cx, hello).or_throw(cx)?;
            js_obj.set(cx, "hello", js_hello)?;
        }
    }
    Ok(js_obj)
}

// hello_world
fn hello_world(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let subject = {
        let js_val = cx.argument::<JsString>(0)?;
        js_val.value(&mut cx)
    };
    let ret = libparsec::hello_world(&subject);
    let js_ret = match ret {
        Ok(ok) => {
            let js_obj = JsObject::new(&mut cx);
            let js_tag = JsBoolean::new(&mut cx, true);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_value = JsString::try_new(&mut cx, ok).or_throw(&mut cx)?;
            js_obj.set(&mut cx, "value", js_value)?;
            js_obj
        }
        Err(err) => {
            let js_obj = (&mut cx).empty_object();
            let js_tag = JsBoolean::new(&mut cx, false);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_err = variant_helloerror_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

pub fn register_meths(cx: &mut ModuleContext) -> NeonResult<()> {
    cx.export_function("helloWorld", hello_world)?;
    Ok(())
}

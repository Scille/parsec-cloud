// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

#[allow(unused_imports)]
use js_sys::*;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
#[allow(unused_imports)]
use wasm_bindgen_futures::*;

// HelloError

#[allow(dead_code)]
fn variant_helloerror_js_to_rs(obj: JsValue) -> Result<libparsec::HelloError, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("EmptySubject") => {
            Ok(libparsec::HelloError::EmptySubject {})
        }
        tag if tag == JsValue::from_str("YouAreADog") => {
            let hello = {
                let js_val = Reflect::get(&obj, &"hello".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            Ok(libparsec::HelloError::YouAreADog { hello })
        }
        _ => Err(JsValue::from(TypeError::new("Object is not a HelloError"))),
    }
}

#[allow(dead_code)]
fn variant_helloerror_rs_to_js(rs_obj: libparsec::HelloError) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::HelloError::EmptySubject {} => {
            Reflect::set(&js_obj, &"tag".into(), &"EmptySubject".into())?;
        }
        libparsec::HelloError::YouAreADog { hello } => {
            Reflect::set(&js_obj, &"tag".into(), &"YouAreADog".into())?;
            let js_hello = hello.into();
            Reflect::set(&js_obj, &"hello".into(), &js_hello)?;
        }
    }
    Ok(js_obj)
}

// hello_world
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn helloWorld(subject: String) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::hello_world(&subject);
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = value.into();
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_helloerror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

#[allow(unused_imports)]
use neon::{prelude::*, types::buffer::TypedArray};

// AvailableDevice

#[allow(dead_code)]
fn struct_availabledevice_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::AvailableDevice> {
    let key_file_path = {
        let js_val: Handle<JsString> = obj.get(cx, "keyFilePath")?;
        match js_val.value(cx).parse() {
            Ok(val) => val,
            Err(err) => return cx.throw_type_error(err),
        }
    };
    let organization_id = {
        let js_val: Handle<JsString> = obj.get(cx, "organizationId")?;
        match js_val.value(cx).parse() {
            Ok(val) => val,
            Err(err) => return cx.throw_type_error(err),
        }
    };
    let device_id = {
        let js_val: Handle<JsString> = obj.get(cx, "deviceId")?;
        match js_val.value(cx).parse() {
            Ok(val) => val,
            Err(err) => return cx.throw_type_error(err),
        }
    };
    let human_handle = {
        let js_val: Handle<JsValue> = obj.get(cx, "humanHandle")?;
        {
            if js_val.is_a::<JsNull, _>(cx) {
                None
            } else {
                let js_val = js_val.downcast_or_throw::<JsString, _>(cx)?;
                Some(match js_val.value(cx).parse() {
                    Ok(val) => val,
                    Err(err) => return cx.throw_type_error(err),
                })
            }
        }
    };
    let device_label = {
        let js_val: Handle<JsValue> = obj.get(cx, "deviceLabel")?;
        {
            if js_val.is_a::<JsNull, _>(cx) {
                None
            } else {
                let js_val = js_val.downcast_or_throw::<JsString, _>(cx)?;
                Some(match js_val.value(cx).parse() {
                    Ok(val) => val,
                    Err(err) => return cx.throw_type_error(err),
                })
            }
        }
    };
    let slug = {
        let js_val: Handle<JsString> = obj.get(cx, "slug")?;
        js_val.value(cx)
    };
    let ty = {
        let js_val: Handle<JsObject> = obj.get(cx, "ty")?;
        variant_devicefiletype_js_to_rs(cx, js_val)?
    };
    Ok(libparsec::AvailableDevice {
        key_file_path,
        organization_id,
        device_id,
        human_handle,
        device_label,
        slug,
        ty,
    })
}

#[allow(dead_code)]
fn struct_availabledevice_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::AvailableDevice,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_key_file_path = JsString::try_new(cx, rs_obj.key_file_path).or_throw(cx)?;
    js_obj.set(cx, "keyFilePath", js_key_file_path)?;
    let js_organization_id = JsString::try_new(cx, rs_obj.organization_id).or_throw(cx)?;
    js_obj.set(cx, "organizationId", js_organization_id)?;
    let js_device_id = JsString::try_new(cx, rs_obj.device_id).or_throw(cx)?;
    js_obj.set(cx, "deviceId", js_device_id)?;
    let js_human_handle = match rs_obj.human_handle {
        Some(elem) => JsString::try_new(cx, elem).or_throw(cx)?.as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "humanHandle", js_human_handle)?;
    let js_device_label = match rs_obj.device_label {
        Some(elem) => JsString::try_new(cx, elem).or_throw(cx)?.as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "deviceLabel", js_device_label)?;
    let js_slug = JsString::try_new(cx, rs_obj.slug).or_throw(cx)?;
    js_obj.set(cx, "slug", js_slug)?;
    let js_ty = variant_devicefiletype_rs_to_js(cx, rs_obj.ty)?;
    js_obj.set(cx, "ty", js_ty)?;
    Ok(js_obj)
}

// DeviceFileType

#[allow(dead_code)]
fn variant_devicefiletype_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceFileType> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "Password" => Ok(libparsec::DeviceFileType::Password {}),
        "Recovery" => Ok(libparsec::DeviceFileType::Recovery {}),
        "Smartcard" => Ok(libparsec::DeviceFileType::Smartcard {}),
        _ => cx.throw_type_error("Object is not a DeviceFileType"),
    }
}

#[allow(dead_code)]
fn variant_devicefiletype_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceFileType,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::DeviceFileType::Password {} => {
            let js_tag = JsString::try_new(cx, "Password").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::DeviceFileType::Recovery {} => {
            let js_tag = JsString::try_new(cx, "Recovery").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::DeviceFileType::Smartcard {} => {
            let js_tag = JsString::try_new(cx, "Smartcard").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// LoggedCoreError

#[allow(dead_code)]
fn variant_loggedcoreerror_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::LoggedCoreError> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "Disconnected" => Ok(libparsec::LoggedCoreError::Disconnected {}),
        "InvalidHandle" => {
            let handle = {
                let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
                (js_val.value(cx) as i32).into()
            };
            Ok(libparsec::LoggedCoreError::InvalidHandle { handle })
        }
        "LoginFailed" => {
            let help = {
                let js_val: Handle<JsString> = obj.get(cx, "help")?;
                js_val.value(cx)
            };
            Ok(libparsec::LoggedCoreError::LoginFailed { help })
        }
        _ => cx.throw_type_error("Object is not a LoggedCoreError"),
    }
}

#[allow(dead_code)]
fn variant_loggedcoreerror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::LoggedCoreError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::LoggedCoreError::Disconnected {} => {
            let js_tag = JsString::try_new(cx, "Disconnected").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::LoggedCoreError::InvalidHandle { handle } => {
            let js_tag = JsString::try_new(cx, "InvalidHandle").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_handle = JsNumber::new(cx, i32::from(handle) as f64);
            js_obj.set(cx, "handle", js_handle)?;
        }
        libparsec::LoggedCoreError::LoginFailed { help } => {
            let js_tag = JsString::try_new(cx, "LoginFailed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_help = JsString::try_new(cx, help).or_throw(cx)?;
            js_obj.set(cx, "help", js_help)?;
        }
    }
    Ok(js_obj)
}

// list_available_devices
fn list_available_devices(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let path = {
        let js_val = cx.argument::<JsString>(0)?;
        match js_val.value(&mut cx).parse() {
            Ok(val) => val,
            Err(err) => return cx.throw_type_error(err),
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::list_available_devices(path).await;

            channel.send(move |mut cx| {
                let js_ret = {
                    // JsArray::new allocates with `undefined` value, that's why we `set` value
                    let js_array = JsArray::new(&mut cx, ret.len() as u32);
                    for (i, elem) in ret.into_iter().enumerate() {
                        let js_elem = struct_availabledevice_rs_to_js(&mut cx, elem)?;
                        js_array.set(&mut cx, i as u32, js_elem)?;
                    }
                    js_array
                };
                deferred.resolve(&mut cx, js_ret);
                Ok(())
            });
        });

    Ok(promise)
}

// login
fn login(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let key = {
        let js_val = cx.argument::<JsString>(0)?;
        js_val.value(&mut cx)
    };
    let password = {
        let js_val = cx.argument::<JsString>(1)?;
        js_val.value(&mut cx)
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::login(&key, &password).await;

            channel.send(move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = JsNumber::new(&mut cx, i32::from(ok) as f64);
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_loggedcoreerror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                deferred.resolve(&mut cx, js_ret);
                Ok(())
            });
        });

    Ok(promise)
}

// logged_core_get_device_id
fn logged_core_get_device_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        (js_val.value(&mut cx) as i32).into()
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::logged_core_get_device_id(handle).await;

            channel.send(move |mut cx| {
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
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_loggedcoreerror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                deferred.resolve(&mut cx, js_ret);
                Ok(())
            });
        });

    Ok(promise)
}

// logged_core_get_device_display
fn logged_core_get_device_display(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        (js_val.value(&mut cx) as i32).into()
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::logged_core_get_device_display(handle).await;

            channel.send(move |mut cx| {
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
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_loggedcoreerror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                deferred.resolve(&mut cx, js_ret);
                Ok(())
            });
        });

    Ok(promise)
}

pub fn register_meths(cx: &mut ModuleContext) -> NeonResult<()> {
    cx.export_function("listAvailableDevices", list_available_devices)?;
    cx.export_function("login", login)?;
    cx.export_function("loggedCoreGetDeviceId", logged_core_get_device_id)?;
    cx.export_function("loggedCoreGetDeviceDisplay", logged_core_get_device_display)?;
    Ok(())
}

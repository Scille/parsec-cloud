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

// AvailableDevice

#[allow(dead_code)]
fn struct_availabledevice_js_to_rs(obj: JsValue) -> Result<libparsec::AvailableDevice, JsValue> {
    let key_file_path = {
        let js_val = Reflect::get(&obj, &"keyFilePath".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
            .parse()
            .map_err(|_| TypeError::new("Not a valid StrPath"))?
    };
    let organization_id = {
        let js_val = Reflect::get(&obj, &"organizationId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
            .parse()
            .map_err(|_| TypeError::new("Not a valid OrganizationID"))?
    };
    let device_id = {
        let js_val = Reflect::get(&obj, &"deviceId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
            .parse()
            .map_err(|_| TypeError::new("Not a valid DeviceID"))?
    };
    let human_handle = {
        let js_val = Reflect::get(&obj, &"humanHandle".into())?;
        if js_val.is_null() {
            None
        } else {
            Some(
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
                    .parse()
                    .map_err(|_| TypeError::new("Not a valid HumanHandle"))?,
            )
        }
    };
    let device_label = {
        let js_val = Reflect::get(&obj, &"deviceLabel".into())?;
        if js_val.is_null() {
            None
        } else {
            Some(
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
                    .parse()
                    .map_err(|_| TypeError::new("Not a valid DeviceLabel"))?,
            )
        }
    };
    let slug = {
        let js_val = Reflect::get(&obj, &"slug".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
    };
    let ty = {
        let js_val = Reflect::get(&obj, &"ty".into())?;
        variant_devicefiletype_js_to_rs(js_val)?
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
fn struct_availabledevice_rs_to_js(rs_obj: libparsec::AvailableDevice) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_key_file_path = JsValue::from_str(rs_obj.key_file_path.as_ref());
    Reflect::set(&js_obj, &"keyFilePath".into(), &js_key_file_path)?;
    let js_organization_id = JsValue::from_str(rs_obj.organization_id.as_ref());
    Reflect::set(&js_obj, &"organizationId".into(), &js_organization_id)?;
    let js_device_id = JsValue::from_str(rs_obj.device_id.as_ref());
    Reflect::set(&js_obj, &"deviceId".into(), &js_device_id)?;
    let js_human_handle = match rs_obj.human_handle {
        Some(val) => JsValue::from_str(val.as_ref()),
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"humanHandle".into(), &js_human_handle)?;
    let js_device_label = match rs_obj.device_label {
        Some(val) => JsValue::from_str(val.as_ref()),
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"deviceLabel".into(), &js_device_label)?;
    let js_slug = rs_obj.slug.into();
    Reflect::set(&js_obj, &"slug".into(), &js_slug)?;
    let js_ty = variant_devicefiletype_rs_to_js(rs_obj.ty)?;
    Reflect::set(&js_obj, &"ty".into(), &js_ty)?;
    Ok(js_obj)
}

// DeviceFileType

#[allow(dead_code)]
fn variant_devicefiletype_js_to_rs(obj: JsValue) -> Result<libparsec::DeviceFileType, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("Password") => Ok(libparsec::DeviceFileType::Password {}),
        tag if tag == JsValue::from_str("Recovery") => Ok(libparsec::DeviceFileType::Recovery {}),
        tag if tag == JsValue::from_str("Smartcard") => Ok(libparsec::DeviceFileType::Smartcard {}),
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a DeviceFileType",
        ))),
    }
}

#[allow(dead_code)]
fn variant_devicefiletype_rs_to_js(rs_obj: libparsec::DeviceFileType) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::DeviceFileType::Password {} => {
            Reflect::set(&js_obj, &"tag".into(), &"Password".into())?;
        }
        libparsec::DeviceFileType::Recovery {} => {
            Reflect::set(&js_obj, &"tag".into(), &"Recovery".into())?;
        }
        libparsec::DeviceFileType::Smartcard {} => {
            Reflect::set(&js_obj, &"tag".into(), &"Smartcard".into())?;
        }
    }
    Ok(js_obj)
}

// LoggedCoreError

#[allow(dead_code)]
fn variant_loggedcoreerror_js_to_rs(obj: JsValue) -> Result<libparsec::LoggedCoreError, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("Disconnected") => {
            Ok(libparsec::LoggedCoreError::Disconnected {})
        }
        tag if tag == JsValue::from_str("InvalidHandle") => {
            let handle = {
                let js_val = Reflect::get(&obj, &"handle".into())?;
                (js_val
                    .dyn_into::<Number>()
                    .map_err(|_| TypeError::new("Not a number"))?
                    .value_of() as i32)
                    .into()
            };
            Ok(libparsec::LoggedCoreError::InvalidHandle { handle })
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a LoggedCoreError",
        ))),
    }
}

#[allow(dead_code)]
fn variant_loggedcoreerror_rs_to_js(
    rs_obj: libparsec::LoggedCoreError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::LoggedCoreError::Disconnected {} => {
            Reflect::set(&js_obj, &"tag".into(), &"Disconnected".into())?;
        }
        libparsec::LoggedCoreError::InvalidHandle { handle } => {
            Reflect::set(&js_obj, &"tag".into(), &"InvalidHandle".into())?;
            let js_handle = JsValue::from(i32::from(handle));
            Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
        }
    }
    Ok(js_obj)
}

// list_available_devices
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn listAvailableDevices(path: String) -> Promise {
    future_to_promise(async move {
        let path = path
            .parse()
            .map_err(|_| JsValue::from(TypeError::new("Not a valid StrPath")))?;

        let ret = libparsec::list_available_devices(path).await;
        Ok({
            // Array::new_with_length allocates with `undefined` value, that's why we `set` value
            let js_array = Array::new_with_length(ret.len() as u32);
            for (i, elem) in ret.into_iter().enumerate() {
                let js_elem = struct_availabledevice_rs_to_js(elem)?;
                js_array.set(i as u32, js_elem);
            }
            js_array.into()
        })
    })
}

// test_gen_default_devices
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn testGenDefaultDevices() -> Promise {
    future_to_promise(async move {
        libparsec::test_gen_default_devices().await;
        Ok(JsValue::NULL)
    })
}

// login
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn login(test_device_id: String) -> Promise {
    future_to_promise(async move {
        let test_device_id = test_device_id
            .parse()
            .map_err(|_| JsValue::from(TypeError::new("Not a valid DeviceID")))?;

        let ret = libparsec::login(test_device_id).await;
        Ok(JsValue::from(i32::from(ret)))
    })
}

// logged_core_get_test_device_id
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn loggedCoreGetTestDeviceId(handle: i32) -> Promise {
    future_to_promise(async move {
        let handle = handle.into();

        let ret = libparsec::logged_core_get_test_device_id(handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from_str(value.as_ref());
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_loggedcoreerror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

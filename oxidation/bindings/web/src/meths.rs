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
        let js_val = Reflect::get(&obj, &"key_file_path".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
            .parse()
            .map_err(|_| TypeError::new("Not a valid StrPath"))?
    };
    let organization_id = {
        let js_val = Reflect::get(&obj, &"organization_id".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
            .parse()
            .map_err(|_| TypeError::new("Not a valid OrganizationID"))?
    };
    let device_id = {
        let js_val = Reflect::get(&obj, &"device_id".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
            .parse()
            .map_err(|_| TypeError::new("Not a valid DeviceID"))?
    };
    let human_handle = {
        let js_val = Reflect::get(&obj, &"human_handle".into())?;
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
        let js_val = Reflect::get(&obj, &"device_label".into())?;
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
    Reflect::set(&js_obj, &"key_file_path".into(), &js_key_file_path)?;
    let js_organization_id = JsValue::from_str(rs_obj.organization_id.as_ref());
    Reflect::set(&js_obj, &"organization_id".into(), &js_organization_id)?;
    let js_device_id = JsValue::from_str(rs_obj.device_id.as_ref());
    Reflect::set(&js_obj, &"device_id".into(), &js_device_id)?;
    let js_human_handle = match rs_obj.human_handle {
        Some(val) => JsValue::from_str(val.as_ref()),
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"human_handle".into(), &js_human_handle)?;
    let js_device_label = match rs_obj.device_label {
        Some(val) => JsValue::from_str(val.as_ref()),
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"device_label".into(), &js_device_label)?;
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

// list_available_devices
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn listAvailableDevices(path: String) -> Promise {
    future_to_promise(async move {
        let path = path
            .parse()
            .map_err(|_| JsValue::from(TypeError::new("Not a valid StrPath")))?;
        let ret = libparsec::list_available_devices(path);
        Ok({
            let js_array = Array::new_with_length(ret.len() as u32);
            for elem in ret {
                let js_elem = struct_availabledevice_rs_to_js(elem)?;
                js_array.push(&js_elem);
            }
            js_array.into()
        })
    })
}

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

// ClientConfig

#[allow(dead_code)]
fn struct_clientconfig_js_to_rs(obj: JsValue) -> Result<libparsec::ClientConfig, JsValue> {
    let config_dir = {
        let js_val = Reflect::get(&obj, &"configDir".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string =
                    |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })
            .map_err(|_| TypeError::new("Not a valid Path"))?
    };
    let data_base_dir = {
        let js_val = Reflect::get(&obj, &"dataBaseDir".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string =
                    |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })
            .map_err(|_| TypeError::new("Not a valid Path"))?
    };
    let mountpoint_base_dir = {
        let js_val = Reflect::get(&obj, &"mountpointBaseDir".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string =
                    |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })
            .map_err(|_| TypeError::new("Not a valid Path"))?
    };
    let workspace_storage_cache_size = {
        let js_val = Reflect::get(&obj, &"workspaceStorageCacheSize".into())?;
        variant_workspacestoragecachesize_js_to_rs(js_val)?
    };
    Ok(libparsec::ClientConfig {
        config_dir,
        data_base_dir,
        mountpoint_base_dir,
        workspace_storage_cache_size,
    })
}

#[allow(dead_code)]
fn struct_clientconfig_rs_to_js(rs_obj: libparsec::ClientConfig) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_config_dir = JsValue::from_str({
        let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        };
        match custom_to_rs_string(rs_obj.config_dir) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"configDir".into(), &js_config_dir)?;
    let js_data_base_dir = JsValue::from_str({
        let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        };
        match custom_to_rs_string(rs_obj.data_base_dir) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"dataBaseDir".into(), &js_data_base_dir)?;
    let js_mountpoint_base_dir = JsValue::from_str({
        let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        };
        match custom_to_rs_string(rs_obj.mountpoint_base_dir) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        }
        .as_ref()
    });
    Reflect::set(
        &js_obj,
        &"mountpointBaseDir".into(),
        &js_mountpoint_base_dir,
    )?;
    let js_workspace_storage_cache_size =
        variant_workspacestoragecachesize_rs_to_js(rs_obj.workspace_storage_cache_size)?;
    Reflect::set(
        &js_obj,
        &"workspaceStorageCacheSize".into(),
        &js_workspace_storage_cache_size,
    )?;
    Ok(js_obj)
}

// AvailableDevice

#[allow(dead_code)]
fn struct_availabledevice_js_to_rs(obj: JsValue) -> Result<libparsec::AvailableDevice, JsValue> {
    let key_file_path = {
        let js_val = Reflect::get(&obj, &"keyFilePath".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string =
                    |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })
            .map_err(|_| TypeError::new("Not a valid Path"))?
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
    let js_key_file_path = JsValue::from_str({
        let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        };
        match custom_to_rs_string(rs_obj.key_file_path) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        }
        .as_ref()
    });
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

// UserClaimInProgress1CtxHandle

#[allow(dead_code)]
fn struct_userclaiminprogress1ctxhandle_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserClaimInProgress1CtxHandle, JsValue> {
    let handle = {
        let js_val = Reflect::get(&obj, &"handle".into())?;
        {
            let v = js_val
                .dyn_into::<Number>()
                .map_err(|_| TypeError::new("Not a number"))?
                .value_of();
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                return Err(JsValue::from(TypeError::new("Not an u32 number")));
            }
            v as u32
        }
    };
    let greeter_sas = {
        let js_val = Reflect::get(&obj, &"greeterSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
            .parse()
            .map_err(|_| TypeError::new("Not a valid SASCode"))?
    };
    let greeter_sas_choices = {
        let js_val = Reflect::get(&obj, &"greeterSasChoices".into())?;
        {
            let js_val = js_val
                .dyn_into::<Array>()
                .map_err(|_| TypeError::new("Not an array"))?;
            let mut converted = Vec::with_capacity(js_val.length() as usize);
            for x in js_val.iter() {
                let x_converted = x
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
                    .parse()
                    .map_err(|_| TypeError::new("Not a valid SASCode"))?;
                converted.push(x_converted);
            }
            converted
        }
    };
    Ok(libparsec::UserClaimInProgress1CtxHandle {
        handle,
        greeter_sas,
        greeter_sas_choices,
    })
}

#[allow(dead_code)]
fn struct_userclaiminprogress1ctxhandle_rs_to_js(
    rs_obj: libparsec::UserClaimInProgress1CtxHandle,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_greeter_sas = JsValue::from_str(rs_obj.greeter_sas.as_ref());
    Reflect::set(&js_obj, &"greeterSas".into(), &js_greeter_sas)?;
    let js_greeter_sas_choices = {
        // Array::new_with_length allocates with `undefined` value, that's why we `set` value
        let js_array = Array::new_with_length(rs_obj.greeter_sas_choices.len() as u32);
        for (i, elem) in rs_obj.greeter_sas_choices.into_iter().enumerate() {
            let js_elem = JsValue::from_str(elem.as_ref());
            js_array.set(i as u32, js_elem);
        }
        js_array.into()
    };
    Reflect::set(
        &js_obj,
        &"greeterSasChoices".into(),
        &js_greeter_sas_choices,
    )?;
    Ok(js_obj)
}

// DeviceClaimInProgress1CtxHandle

#[allow(dead_code)]
fn struct_deviceclaiminprogress1ctxhandle_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceClaimInProgress1CtxHandle, JsValue> {
    let handle = {
        let js_val = Reflect::get(&obj, &"handle".into())?;
        {
            let v = js_val
                .dyn_into::<Number>()
                .map_err(|_| TypeError::new("Not a number"))?
                .value_of();
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                return Err(JsValue::from(TypeError::new("Not an u32 number")));
            }
            v as u32
        }
    };
    let greeter_sas = {
        let js_val = Reflect::get(&obj, &"greeterSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
            .parse()
            .map_err(|_| TypeError::new("Not a valid SASCode"))?
    };
    let greeter_sas_choices = {
        let js_val = Reflect::get(&obj, &"greeterSasChoices".into())?;
        {
            let js_val = js_val
                .dyn_into::<Array>()
                .map_err(|_| TypeError::new("Not an array"))?;
            let mut converted = Vec::with_capacity(js_val.length() as usize);
            for x in js_val.iter() {
                let x_converted = x
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
                    .parse()
                    .map_err(|_| TypeError::new("Not a valid SASCode"))?;
                converted.push(x_converted);
            }
            converted
        }
    };
    Ok(libparsec::DeviceClaimInProgress1CtxHandle {
        handle,
        greeter_sas,
        greeter_sas_choices,
    })
}

#[allow(dead_code)]
fn struct_deviceclaiminprogress1ctxhandle_rs_to_js(
    rs_obj: libparsec::DeviceClaimInProgress1CtxHandle,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_greeter_sas = JsValue::from_str(rs_obj.greeter_sas.as_ref());
    Reflect::set(&js_obj, &"greeterSas".into(), &js_greeter_sas)?;
    let js_greeter_sas_choices = {
        // Array::new_with_length allocates with `undefined` value, that's why we `set` value
        let js_array = Array::new_with_length(rs_obj.greeter_sas_choices.len() as u32);
        for (i, elem) in rs_obj.greeter_sas_choices.into_iter().enumerate() {
            let js_elem = JsValue::from_str(elem.as_ref());
            js_array.set(i as u32, js_elem);
        }
        js_array.into()
    };
    Reflect::set(
        &js_obj,
        &"greeterSasChoices".into(),
        &js_greeter_sas_choices,
    )?;
    Ok(js_obj)
}

// UserClaimInProgress2CtxHandle

#[allow(dead_code)]
fn struct_userclaiminprogress2ctxhandle_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserClaimInProgress2CtxHandle, JsValue> {
    let handle = {
        let js_val = Reflect::get(&obj, &"handle".into())?;
        {
            let v = js_val
                .dyn_into::<Number>()
                .map_err(|_| TypeError::new("Not a number"))?
                .value_of();
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                return Err(JsValue::from(TypeError::new("Not an u32 number")));
            }
            v as u32
        }
    };
    let claimer_sas = {
        let js_val = Reflect::get(&obj, &"claimerSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
            .parse()
            .map_err(|_| TypeError::new("Not a valid SASCode"))?
    };
    Ok(libparsec::UserClaimInProgress2CtxHandle {
        handle,
        claimer_sas,
    })
}

#[allow(dead_code)]
fn struct_userclaiminprogress2ctxhandle_rs_to_js(
    rs_obj: libparsec::UserClaimInProgress2CtxHandle,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_claimer_sas = JsValue::from_str(rs_obj.claimer_sas.as_ref());
    Reflect::set(&js_obj, &"claimerSas".into(), &js_claimer_sas)?;
    Ok(js_obj)
}

// DeviceClaimInProgress2CtxHandle

#[allow(dead_code)]
fn struct_deviceclaiminprogress2ctxhandle_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceClaimInProgress2CtxHandle, JsValue> {
    let handle = {
        let js_val = Reflect::get(&obj, &"handle".into())?;
        {
            let v = js_val
                .dyn_into::<Number>()
                .map_err(|_| TypeError::new("Not a number"))?
                .value_of();
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                return Err(JsValue::from(TypeError::new("Not an u32 number")));
            }
            v as u32
        }
    };
    let claimer_sas = {
        let js_val = Reflect::get(&obj, &"claimerSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
            .parse()
            .map_err(|_| TypeError::new("Not a valid SASCode"))?
    };
    Ok(libparsec::DeviceClaimInProgress2CtxHandle {
        handle,
        claimer_sas,
    })
}

#[allow(dead_code)]
fn struct_deviceclaiminprogress2ctxhandle_rs_to_js(
    rs_obj: libparsec::DeviceClaimInProgress2CtxHandle,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_claimer_sas = JsValue::from_str(rs_obj.claimer_sas.as_ref());
    Reflect::set(&js_obj, &"claimerSas".into(), &js_claimer_sas)?;
    Ok(js_obj)
}

// UserClaimInProgress3CtxHandle

#[allow(dead_code)]
fn struct_userclaiminprogress3ctxhandle_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserClaimInProgress3CtxHandle, JsValue> {
    let handle = {
        let js_val = Reflect::get(&obj, &"handle".into())?;
        {
            let v = js_val
                .dyn_into::<Number>()
                .map_err(|_| TypeError::new("Not a number"))?
                .value_of();
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                return Err(JsValue::from(TypeError::new("Not an u32 number")));
            }
            v as u32
        }
    };
    Ok(libparsec::UserClaimInProgress3CtxHandle { handle })
}

#[allow(dead_code)]
fn struct_userclaiminprogress3ctxhandle_rs_to_js(
    rs_obj: libparsec::UserClaimInProgress3CtxHandle,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// DeviceClaimInProgress3CtxHandle

#[allow(dead_code)]
fn struct_deviceclaiminprogress3ctxhandle_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceClaimInProgress3CtxHandle, JsValue> {
    let handle = {
        let js_val = Reflect::get(&obj, &"handle".into())?;
        {
            let v = js_val
                .dyn_into::<Number>()
                .map_err(|_| TypeError::new("Not a number"))?
                .value_of();
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                return Err(JsValue::from(TypeError::new("Not an u32 number")));
            }
            v as u32
        }
    };
    Ok(libparsec::DeviceClaimInProgress3CtxHandle { handle })
}

#[allow(dead_code)]
fn struct_deviceclaiminprogress3ctxhandle_rs_to_js(
    rs_obj: libparsec::DeviceClaimInProgress3CtxHandle,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// UserClaimFinalizeCtxHandle

#[allow(dead_code)]
fn struct_userclaimfinalizectxhandle_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserClaimFinalizeCtxHandle, JsValue> {
    let handle = {
        let js_val = Reflect::get(&obj, &"handle".into())?;
        {
            let v = js_val
                .dyn_into::<Number>()
                .map_err(|_| TypeError::new("Not a number"))?
                .value_of();
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                return Err(JsValue::from(TypeError::new("Not an u32 number")));
            }
            v as u32
        }
    };
    Ok(libparsec::UserClaimFinalizeCtxHandle { handle })
}

#[allow(dead_code)]
fn struct_userclaimfinalizectxhandle_rs_to_js(
    rs_obj: libparsec::UserClaimFinalizeCtxHandle,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// DeviceClaimFinalizeCtxHandle

#[allow(dead_code)]
fn struct_deviceclaimfinalizectxhandle_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceClaimFinalizeCtxHandle, JsValue> {
    let handle = {
        let js_val = Reflect::get(&obj, &"handle".into())?;
        {
            let v = js_val
                .dyn_into::<Number>()
                .map_err(|_| TypeError::new("Not a number"))?
                .value_of();
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                return Err(JsValue::from(TypeError::new("Not an u32 number")));
            }
            v as u32
        }
    };
    Ok(libparsec::DeviceClaimFinalizeCtxHandle { handle })
}

#[allow(dead_code)]
fn struct_deviceclaimfinalizectxhandle_rs_to_js(
    rs_obj: libparsec::DeviceClaimFinalizeCtxHandle,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// WorkspaceStorageCacheSize

#[allow(dead_code)]
fn variant_workspacestoragecachesize_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::WorkspaceStorageCacheSize, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("Custom") => {
            let size = {
                let js_val = Reflect::get(&obj, &"size".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u32 number")));
                    }
                    v as u32
                }
            };
            Ok(libparsec::WorkspaceStorageCacheSize::Custom { size })
        }
        tag if tag == JsValue::from_str("Default") => {
            Ok(libparsec::WorkspaceStorageCacheSize::Default {})
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a WorkspaceStorageCacheSize",
        ))),
    }
}

#[allow(dead_code)]
fn variant_workspacestoragecachesize_rs_to_js(
    rs_obj: libparsec::WorkspaceStorageCacheSize,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::WorkspaceStorageCacheSize::Custom { size, .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Custom".into())?;
            let js_size = JsValue::from(size);
            Reflect::set(&js_obj, &"size".into(), &js_size)?;
        }
        libparsec::WorkspaceStorageCacheSize::Default { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Default".into())?;
        }
    }
    Ok(js_obj)
}

// ClientEvent

#[allow(dead_code)]
fn variant_clientevent_js_to_rs(obj: JsValue) -> Result<libparsec::ClientEvent, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("Ping") => {
            let ping = {
                let js_val = Reflect::get(&obj, &"ping".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            Ok(libparsec::ClientEvent::Ping { ping })
        }
        _ => Err(JsValue::from(TypeError::new("Object is not a ClientEvent"))),
    }
}

#[allow(dead_code)]
fn variant_clientevent_rs_to_js(rs_obj: libparsec::ClientEvent) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::ClientEvent::Ping { ping, .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Ping".into())?;
            let js_ping = ping.into();
            Reflect::set(&js_obj, &"ping".into(), &js_ping)?;
        }
    }
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
        libparsec::DeviceFileType::Password { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Password".into())?;
        }
        libparsec::DeviceFileType::Recovery { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Recovery".into())?;
        }
        libparsec::DeviceFileType::Smartcard { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Smartcard".into())?;
        }
    }
    Ok(js_obj)
}

// DeviceSaveStrategy

#[allow(dead_code)]
fn variant_devicesavestrategy_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceSaveStrategy, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("Password") => {
            let password = {
                let js_val = Reflect::get(&obj, &"password".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string =
                            |s: String| -> Result<_, String> { Ok(s.into()) };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid Password"))?
            };
            Ok(libparsec::DeviceSaveStrategy::Password { password })
        }
        tag if tag == JsValue::from_str("Smartcard") => {
            Ok(libparsec::DeviceSaveStrategy::Smartcard {})
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a DeviceSaveStrategy",
        ))),
    }
}

#[allow(dead_code)]
fn variant_devicesavestrategy_rs_to_js(
    rs_obj: libparsec::DeviceSaveStrategy,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::DeviceSaveStrategy::Password { password, .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Password".into())?;
            let js_password = JsValue::from_str(password.as_ref());
            Reflect::set(&js_obj, &"password".into(), &js_password)?;
        }
        libparsec::DeviceSaveStrategy::Smartcard { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Smartcard".into())?;
        }
    }
    Ok(js_obj)
}

// BootstrapOrganizationError

#[allow(dead_code)]
fn variant_bootstraporganizationerror_rs_to_js(
    rs_obj: libparsec::BootstrapOrganizationError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::BootstrapOrganizationError::AlreadyUsedToken { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AlreadyUsedToken".into())?;
        }
        libparsec::BootstrapOrganizationError::BadTimestamp {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"BadTimestamp".into())?;
            let js_server_timestamp = JsValue::from_str({
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339().into())
                    };
                match custom_to_rs_string(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"server_timestamp".into(), &js_server_timestamp)?;
            let js_client_timestamp = JsValue::from_str({
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339().into())
                    };
                match custom_to_rs_string(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"client_timestamp".into(), &js_client_timestamp)?;
            let js_ballpark_client_early_offset = ballpark_client_early_offset.into();
            Reflect::set(
                &js_obj,
                &"ballpark_client_early_offset".into(),
                &js_ballpark_client_early_offset,
            )?;
            let js_ballpark_client_late_offset = ballpark_client_late_offset.into();
            Reflect::set(
                &js_obj,
                &"ballpark_client_late_offset".into(),
                &js_ballpark_client_late_offset,
            )?;
        }
        libparsec::BootstrapOrganizationError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::BootstrapOrganizationError::InvalidToken { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"InvalidToken".into())?;
        }
        libparsec::BootstrapOrganizationError::Offline { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Offline".into())?;
        }
        libparsec::BootstrapOrganizationError::SaveDeviceError { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"SaveDeviceError".into())?;
        }
    }
    Ok(js_obj)
}

// ClaimerRetrieveInfoError

#[allow(dead_code)]
fn variant_claimerretrieveinfoerror_rs_to_js(
    rs_obj: libparsec::ClaimerRetrieveInfoError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClaimerRetrieveInfoError::AlreadyUsed { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AlreadyUsed".into())?;
        }
        libparsec::ClaimerRetrieveInfoError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::ClaimerRetrieveInfoError::NotFound { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"NotFound".into())?;
        }
        libparsec::ClaimerRetrieveInfoError::Offline { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Offline".into())?;
        }
    }
    Ok(js_obj)
}

// ClaimInProgressError

#[allow(dead_code)]
fn variant_claiminprogresserror_rs_to_js(
    rs_obj: libparsec::ClaimInProgressError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClaimInProgressError::ActiveUsersLimitReached { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ActiveUsersLimitReached".into())?;
        }
        libparsec::ClaimInProgressError::AlreadyUsed { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AlreadyUsed".into())?;
        }
        libparsec::ClaimInProgressError::CorruptedConfirmation { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"CorruptedConfirmation".into())?;
        }
        libparsec::ClaimInProgressError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::ClaimInProgressError::NotFound { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"NotFound".into())?;
        }
        libparsec::ClaimInProgressError::Offline { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Offline".into())?;
        }
        libparsec::ClaimInProgressError::PeerReset { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"PeerReset".into())?;
        }
    }
    Ok(js_obj)
}

// UserOrDeviceClaimInitialCtxHandle

#[allow(dead_code)]
fn variant_userordeviceclaiminitialctxhandle_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserOrDeviceClaimInitialCtxHandle, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("Device") => {
            let handle = {
                let js_val = Reflect::get(&obj, &"handle".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u32 number")));
                    }
                    v as u32
                }
            };
            let greeter_user_id = {
                let js_val = Reflect::get(&obj, &"greeter_user_id".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
                    .parse()
                    .map_err(|_| TypeError::new("Not a valid UserID"))?
            };
            let greeter_human_handle = {
                let js_val = Reflect::get(&obj, &"greeter_human_handle".into())?;
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
            Ok(libparsec::UserOrDeviceClaimInitialCtxHandle::Device {
                handle,
                greeter_user_id,
                greeter_human_handle,
            })
        }
        tag if tag == JsValue::from_str("User") => {
            let handle = {
                let js_val = Reflect::get(&obj, &"handle".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u32 number")));
                    }
                    v as u32
                }
            };
            let claimer_email = {
                let js_val = Reflect::get(&obj, &"claimer_email".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            let greeter_user_id = {
                let js_val = Reflect::get(&obj, &"greeter_user_id".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
                    .parse()
                    .map_err(|_| TypeError::new("Not a valid UserID"))?
            };
            let greeter_human_handle = {
                let js_val = Reflect::get(&obj, &"greeter_human_handle".into())?;
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
            Ok(libparsec::UserOrDeviceClaimInitialCtxHandle::User {
                handle,
                claimer_email,
                greeter_user_id,
                greeter_human_handle,
            })
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a UserOrDeviceClaimInitialCtxHandle",
        ))),
    }
}

#[allow(dead_code)]
fn variant_userordeviceclaiminitialctxhandle_rs_to_js(
    rs_obj: libparsec::UserOrDeviceClaimInitialCtxHandle,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::UserOrDeviceClaimInitialCtxHandle::Device {
            handle,
            greeter_user_id,
            greeter_human_handle,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"Device".into())?;
            let js_handle = JsValue::from(handle);
            Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
            let js_greeter_user_id = JsValue::from_str(greeter_user_id.as_ref());
            Reflect::set(&js_obj, &"greeter_user_id".into(), &js_greeter_user_id)?;
            let js_greeter_human_handle = match greeter_human_handle {
                Some(val) => JsValue::from_str(val.as_ref()),
                None => JsValue::NULL,
            };
            Reflect::set(
                &js_obj,
                &"greeter_human_handle".into(),
                &js_greeter_human_handle,
            )?;
        }
        libparsec::UserOrDeviceClaimInitialCtxHandle::User {
            handle,
            claimer_email,
            greeter_user_id,
            greeter_human_handle,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"User".into())?;
            let js_handle = JsValue::from(handle);
            Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
            let js_claimer_email = claimer_email.into();
            Reflect::set(&js_obj, &"claimer_email".into(), &js_claimer_email)?;
            let js_greeter_user_id = JsValue::from_str(greeter_user_id.as_ref());
            Reflect::set(&js_obj, &"greeter_user_id".into(), &js_greeter_user_id)?;
            let js_greeter_human_handle = match greeter_human_handle {
                Some(val) => JsValue::from_str(val.as_ref()),
                None => JsValue::NULL,
            };
            Reflect::set(
                &js_obj,
                &"greeter_human_handle".into(),
                &js_greeter_human_handle,
            )?;
        }
    }
    Ok(js_obj)
}

// list_available_devices
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn listAvailableDevices(path: String) -> Promise {
    future_to_promise(async move {
        let path = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::list_available_devices(&path).await;
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

// bootstrap_organization
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn bootstrapOrganization(
    config: Object,
    on_event_callback: Function,
    bootstrap_organization_addr: String,
    save_strategy: Object,
    human_handle: Option<String>,
    device_label: Option<String>,
    sequester_authority_verify_key: Option<Uint8Array>,
) -> Promise {
    future_to_promise(async move {
        let config = config.into();
        let config = struct_clientconfig_js_to_rs(config)?;

        let on_event_callback = std::sync::Arc::new(move |event: libparsec::ClientEvent| {
            // TODO: Better error handling here (log error ?)
            let js_event =
                variant_clientevent_rs_to_js(event).expect("event type conversion error");
            on_event_callback
                .call1(&JsValue::NULL, &js_event)
                .expect("error in event callback");
        }) as std::sync::Arc<dyn Fn(libparsec::ClientEvent)>;

        let bootstrap_organization_addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::BackendOrganizationBootstrapAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(bootstrap_organization_addr)
                .map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let save_strategy = save_strategy.into();
        let save_strategy = variant_devicesavestrategy_js_to_rs(save_strategy)?;

        let human_handle = match human_handle {
            Some(human_handle) => {
                let human_handle = human_handle
                    .parse()
                    .map_err(|_| JsValue::from(TypeError::new("Not a valid HumanHandle")))?;

                Some(human_handle)
            }
            None => None,
        };

        let device_label = match device_label {
            Some(device_label) => {
                let device_label = device_label
                    .parse()
                    .map_err(|_| JsValue::from(TypeError::new("Not a valid DeviceLabel")))?;

                Some(device_label)
            }
            None => None,
        };

        let sequester_authority_verify_key = match sequester_authority_verify_key {
            Some(sequester_authority_verify_key) => {
                let sequester_authority_verify_key = sequester_authority_verify_key
                    .to_vec()
                    .as_slice()
                    .try_into()
                    .map_err(|_| {
                        JsValue::from(TypeError::new("Not a valid SequesterVerifyKeyDer"))
                    })?;

                Some(sequester_authority_verify_key)
            }
            None => None,
        };

        let ret = libparsec::bootstrap_organization(
            config,
            on_event_callback,
            bootstrap_organization_addr,
            save_strategy,
            human_handle,
            device_label,
            sequester_authority_verify_key,
        )
        .await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_availabledevice_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_bootstraporganizationerror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// claimer_retrieve_info
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerRetrieveInfo(config: Object, on_event_callback: Function, addr: String) -> Promise {
    future_to_promise(async move {
        let config = config.into();
        let config = struct_clientconfig_js_to_rs(config)?;

        let on_event_callback = std::sync::Arc::new(move |event: libparsec::ClientEvent| {
            // TODO: Better error handling here (log error ?)
            let js_event =
                variant_clientevent_rs_to_js(event).expect("event type conversion error");
            on_event_callback
                .call1(&JsValue::NULL, &js_event)
                .expect("error in event callback");
        }) as std::sync::Arc<dyn Fn(libparsec::ClientEvent)>;

        let addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::BackendInvitationAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(addr).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::claimer_retrieve_info(config, on_event_callback, addr).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = variant_userordeviceclaiminitialctxhandle_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claimerretrieveinfoerror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// claimer_user_initial_ctx_do_wait_peer
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerUserInitialCtxDoWaitPeer(handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::claimer_user_initial_ctx_do_wait_peer(handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_userclaiminprogress1ctxhandle_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claiminprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// claimer_device_initial_ctx_do_wait_peer
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerDeviceInitialCtxDoWaitPeer(handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::claimer_device_initial_ctx_do_wait_peer(handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_deviceclaiminprogress1ctxhandle_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claiminprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// claimer_user_in_progress_2_do_signify_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerUserInProgress2DoSignifyTrust(handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::claimer_user_in_progress_2_do_signify_trust(handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_userclaiminprogress2ctxhandle_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claiminprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// claimer_device_in_progress_2_do_signify_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerDeviceInProgress2DoSignifyTrust(handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::claimer_device_in_progress_2_do_signify_trust(handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_deviceclaiminprogress2ctxhandle_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claiminprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// claimer_user_in_progress_2_do_wait_peer_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerUserInProgress2DoWaitPeerTrust(handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::claimer_user_in_progress_2_do_wait_peer_trust(handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_userclaiminprogress3ctxhandle_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claiminprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// claimer_device_in_progress_2_do_wait_peer_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerDeviceInProgress2DoWaitPeerTrust(handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::claimer_device_in_progress_2_do_wait_peer_trust(handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_deviceclaiminprogress3ctxhandle_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claiminprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// claimer_user_in_progress_3_do_claim
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerUserInProgress3DoClaim(
    handle: u32,
    requested_device_label: Option<String>,
    requested_human_handle: Option<String>,
) -> Promise {
    future_to_promise(async move {
        let requested_device_label = match requested_device_label {
            Some(requested_device_label) => {
                let requested_device_label = requested_device_label
                    .parse()
                    .map_err(|_| JsValue::from(TypeError::new("Not a valid DeviceLabel")))?;

                Some(requested_device_label)
            }
            None => None,
        };

        let requested_human_handle = match requested_human_handle {
            Some(requested_human_handle) => {
                let requested_human_handle = requested_human_handle
                    .parse()
                    .map_err(|_| JsValue::from(TypeError::new("Not a valid HumanHandle")))?;

                Some(requested_human_handle)
            }
            None => None,
        };

        let ret = libparsec::claimer_user_in_progress_3_do_claim(
            handle,
            requested_device_label,
            requested_human_handle,
        )
        .await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_userclaimfinalizectxhandle_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claiminprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// claimer_device_in_progress_3_do_claim
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerDeviceInProgress3DoClaim(
    handle: u32,
    requested_device_label: Option<String>,
) -> Promise {
    future_to_promise(async move {
        let requested_device_label = match requested_device_label {
            Some(requested_device_label) => {
                let requested_device_label = requested_device_label
                    .parse()
                    .map_err(|_| JsValue::from(TypeError::new("Not a valid DeviceLabel")))?;

                Some(requested_device_label)
            }
            None => None,
        };

        let ret =
            libparsec::claimer_device_in_progress_3_do_claim(handle, requested_device_label).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_deviceclaimfinalizectxhandle_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claiminprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// claimer_user_finalize_save_local_device
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerUserFinalizeSaveLocalDevice(handle: u32, save_strategy: Object) -> Promise {
    future_to_promise(async move {
        let save_strategy = save_strategy.into();
        let save_strategy = variant_devicesavestrategy_js_to_rs(save_strategy)?;

        let ret = libparsec::claimer_user_finalize_save_local_device(handle, save_strategy).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_availabledevice_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claiminprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// claimer_device_finalize_save_local_device
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerDeviceFinalizeSaveLocalDevice(handle: u32, save_strategy: Object) -> Promise {
    future_to_promise(async move {
        let save_strategy = save_strategy.into();
        let save_strategy = variant_devicesavestrategy_js_to_rs(save_strategy)?;

        let ret = libparsec::claimer_device_finalize_save_local_device(handle, save_strategy).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_availabledevice_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claiminprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// test_new_testbed
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn testNewTestbed(template: String, test_server: Option<String>) -> Promise {
    future_to_promise(async move {
        let test_server = match test_server {
            Some(test_server) => {
                let test_server = {
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::BackendAddr::from_any(&s).map_err(|e| e.to_string())
                    };
                    custom_from_rs_string(test_server).map_err(|e| TypeError::new(e.as_ref()))
                }?;

                Some(test_server)
            }
            None => None,
        };

        let ret = libparsec::test_new_testbed(&template, test_server.as_ref()).await;
        Ok(JsValue::from_str({
            let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                path.into_os_string()
                    .into_string()
                    .map_err(|_| "Path contains non-utf8 characters")
            };
            match custom_to_rs_string(ret) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
            }
            .as_ref()
        }))
    })
}

// test_get_testbed_organization_id
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn testGetTestbedOrganizationId(discriminant_dir: String) -> Promise {
    future_to_promise(async move {
        let discriminant_dir = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(discriminant_dir).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::test_get_testbed_organization_id(&discriminant_dir);
        Ok(match ret {
            Some(val) => JsValue::from_str(val.as_ref()),
            None => JsValue::NULL,
        })
    })
}

// test_get_testbed_bootstrap_organization_addr
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn testGetTestbedBootstrapOrganizationAddr(discriminant_dir: String) -> Promise {
    future_to_promise(async move {
        let discriminant_dir = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(discriminant_dir).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::test_get_testbed_bootstrap_organization_addr(&discriminant_dir);
        Ok(match ret {
            Some(val) => JsValue::from_str({
                let custom_to_rs_string = |addr: libparsec::BackendOrganizationBootstrapAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) };
                match custom_to_rs_string(val) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            }),
            None => JsValue::NULL,
        })
    })
}

// test_drop_testbed
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn testDropTestbed(path: String) -> Promise {
    future_to_promise(async move {
        let path = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        libparsec::test_drop_testbed(&path).await;
        Ok(JsValue::NULL)
    })
}

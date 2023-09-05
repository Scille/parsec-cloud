// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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

// UserClaimInProgress1Info

#[allow(dead_code)]
fn struct_userclaiminprogress1info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserClaimInProgress1Info, JsValue> {
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
    Ok(libparsec::UserClaimInProgress1Info {
        handle,
        greeter_sas,
        greeter_sas_choices,
    })
}

#[allow(dead_code)]
fn struct_userclaiminprogress1info_rs_to_js(
    rs_obj: libparsec::UserClaimInProgress1Info,
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

// DeviceClaimInProgress1Info

#[allow(dead_code)]
fn struct_deviceclaiminprogress1info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceClaimInProgress1Info, JsValue> {
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
    Ok(libparsec::DeviceClaimInProgress1Info {
        handle,
        greeter_sas,
        greeter_sas_choices,
    })
}

#[allow(dead_code)]
fn struct_deviceclaiminprogress1info_rs_to_js(
    rs_obj: libparsec::DeviceClaimInProgress1Info,
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

// UserClaimInProgress2Info

#[allow(dead_code)]
fn struct_userclaiminprogress2info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserClaimInProgress2Info, JsValue> {
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
    Ok(libparsec::UserClaimInProgress2Info {
        handle,
        claimer_sas,
    })
}

#[allow(dead_code)]
fn struct_userclaiminprogress2info_rs_to_js(
    rs_obj: libparsec::UserClaimInProgress2Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_claimer_sas = JsValue::from_str(rs_obj.claimer_sas.as_ref());
    Reflect::set(&js_obj, &"claimerSas".into(), &js_claimer_sas)?;
    Ok(js_obj)
}

// DeviceClaimInProgress2Info

#[allow(dead_code)]
fn struct_deviceclaiminprogress2info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceClaimInProgress2Info, JsValue> {
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
    Ok(libparsec::DeviceClaimInProgress2Info {
        handle,
        claimer_sas,
    })
}

#[allow(dead_code)]
fn struct_deviceclaiminprogress2info_rs_to_js(
    rs_obj: libparsec::DeviceClaimInProgress2Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_claimer_sas = JsValue::from_str(rs_obj.claimer_sas.as_ref());
    Reflect::set(&js_obj, &"claimerSas".into(), &js_claimer_sas)?;
    Ok(js_obj)
}

// UserClaimInProgress3Info

#[allow(dead_code)]
fn struct_userclaiminprogress3info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserClaimInProgress3Info, JsValue> {
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
    Ok(libparsec::UserClaimInProgress3Info { handle })
}

#[allow(dead_code)]
fn struct_userclaiminprogress3info_rs_to_js(
    rs_obj: libparsec::UserClaimInProgress3Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// DeviceClaimInProgress3Info

#[allow(dead_code)]
fn struct_deviceclaiminprogress3info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceClaimInProgress3Info, JsValue> {
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
    Ok(libparsec::DeviceClaimInProgress3Info { handle })
}

#[allow(dead_code)]
fn struct_deviceclaiminprogress3info_rs_to_js(
    rs_obj: libparsec::DeviceClaimInProgress3Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// UserClaimFinalizeInfo

#[allow(dead_code)]
fn struct_userclaimfinalizeinfo_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserClaimFinalizeInfo, JsValue> {
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
    Ok(libparsec::UserClaimFinalizeInfo { handle })
}

#[allow(dead_code)]
fn struct_userclaimfinalizeinfo_rs_to_js(
    rs_obj: libparsec::UserClaimFinalizeInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// DeviceClaimFinalizeInfo

#[allow(dead_code)]
fn struct_deviceclaimfinalizeinfo_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceClaimFinalizeInfo, JsValue> {
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
    Ok(libparsec::DeviceClaimFinalizeInfo { handle })
}

#[allow(dead_code)]
fn struct_deviceclaimfinalizeinfo_rs_to_js(
    rs_obj: libparsec::DeviceClaimFinalizeInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// UserGreetInitialInfo

#[allow(dead_code)]
fn struct_usergreetinitialinfo_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserGreetInitialInfo, JsValue> {
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
    Ok(libparsec::UserGreetInitialInfo { handle })
}

#[allow(dead_code)]
fn struct_usergreetinitialinfo_rs_to_js(
    rs_obj: libparsec::UserGreetInitialInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// DeviceGreetInitialInfo

#[allow(dead_code)]
fn struct_devicegreetinitialinfo_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceGreetInitialInfo, JsValue> {
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
    Ok(libparsec::DeviceGreetInitialInfo { handle })
}

#[allow(dead_code)]
fn struct_devicegreetinitialinfo_rs_to_js(
    rs_obj: libparsec::DeviceGreetInitialInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// UserGreetInProgress1Info

#[allow(dead_code)]
fn struct_usergreetinprogress1info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserGreetInProgress1Info, JsValue> {
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
    Ok(libparsec::UserGreetInProgress1Info {
        handle,
        greeter_sas,
    })
}

#[allow(dead_code)]
fn struct_usergreetinprogress1info_rs_to_js(
    rs_obj: libparsec::UserGreetInProgress1Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_greeter_sas = JsValue::from_str(rs_obj.greeter_sas.as_ref());
    Reflect::set(&js_obj, &"greeterSas".into(), &js_greeter_sas)?;
    Ok(js_obj)
}

// DeviceGreetInProgress1Info

#[allow(dead_code)]
fn struct_devicegreetinprogress1info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceGreetInProgress1Info, JsValue> {
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
    Ok(libparsec::DeviceGreetInProgress1Info {
        handle,
        greeter_sas,
    })
}

#[allow(dead_code)]
fn struct_devicegreetinprogress1info_rs_to_js(
    rs_obj: libparsec::DeviceGreetInProgress1Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_greeter_sas = JsValue::from_str(rs_obj.greeter_sas.as_ref());
    Reflect::set(&js_obj, &"greeterSas".into(), &js_greeter_sas)?;
    Ok(js_obj)
}

// UserGreetInProgress2Info

#[allow(dead_code)]
fn struct_usergreetinprogress2info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserGreetInProgress2Info, JsValue> {
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
    let claimer_sas_choices = {
        let js_val = Reflect::get(&obj, &"claimerSasChoices".into())?;
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
    Ok(libparsec::UserGreetInProgress2Info {
        handle,
        claimer_sas,
        claimer_sas_choices,
    })
}

#[allow(dead_code)]
fn struct_usergreetinprogress2info_rs_to_js(
    rs_obj: libparsec::UserGreetInProgress2Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_claimer_sas = JsValue::from_str(rs_obj.claimer_sas.as_ref());
    Reflect::set(&js_obj, &"claimerSas".into(), &js_claimer_sas)?;
    let js_claimer_sas_choices = {
        // Array::new_with_length allocates with `undefined` value, that's why we `set` value
        let js_array = Array::new_with_length(rs_obj.claimer_sas_choices.len() as u32);
        for (i, elem) in rs_obj.claimer_sas_choices.into_iter().enumerate() {
            let js_elem = JsValue::from_str(elem.as_ref());
            js_array.set(i as u32, js_elem);
        }
        js_array.into()
    };
    Reflect::set(
        &js_obj,
        &"claimerSasChoices".into(),
        &js_claimer_sas_choices,
    )?;
    Ok(js_obj)
}

// DeviceGreetInProgress2Info

#[allow(dead_code)]
fn struct_devicegreetinprogress2info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceGreetInProgress2Info, JsValue> {
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
    let claimer_sas_choices = {
        let js_val = Reflect::get(&obj, &"claimerSasChoices".into())?;
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
    Ok(libparsec::DeviceGreetInProgress2Info {
        handle,
        claimer_sas,
        claimer_sas_choices,
    })
}

#[allow(dead_code)]
fn struct_devicegreetinprogress2info_rs_to_js(
    rs_obj: libparsec::DeviceGreetInProgress2Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_claimer_sas = JsValue::from_str(rs_obj.claimer_sas.as_ref());
    Reflect::set(&js_obj, &"claimerSas".into(), &js_claimer_sas)?;
    let js_claimer_sas_choices = {
        // Array::new_with_length allocates with `undefined` value, that's why we `set` value
        let js_array = Array::new_with_length(rs_obj.claimer_sas_choices.len() as u32);
        for (i, elem) in rs_obj.claimer_sas_choices.into_iter().enumerate() {
            let js_elem = JsValue::from_str(elem.as_ref());
            js_array.set(i as u32, js_elem);
        }
        js_array.into()
    };
    Reflect::set(
        &js_obj,
        &"claimerSasChoices".into(),
        &js_claimer_sas_choices,
    )?;
    Ok(js_obj)
}

// UserGreetInProgress3Info

#[allow(dead_code)]
fn struct_usergreetinprogress3info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserGreetInProgress3Info, JsValue> {
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
    Ok(libparsec::UserGreetInProgress3Info { handle })
}

#[allow(dead_code)]
fn struct_usergreetinprogress3info_rs_to_js(
    rs_obj: libparsec::UserGreetInProgress3Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// DeviceGreetInProgress3Info

#[allow(dead_code)]
fn struct_devicegreetinprogress3info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceGreetInProgress3Info, JsValue> {
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
    Ok(libparsec::DeviceGreetInProgress3Info { handle })
}

#[allow(dead_code)]
fn struct_devicegreetinprogress3info_rs_to_js(
    rs_obj: libparsec::DeviceGreetInProgress3Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// UserGreetInProgress4Info

#[allow(dead_code)]
fn struct_usergreetinprogress4info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserGreetInProgress4Info, JsValue> {
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
    let requested_human_handle = {
        let js_val = Reflect::get(&obj, &"requestedHumanHandle".into())?;
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
    let requested_device_label = {
        let js_val = Reflect::get(&obj, &"requestedDeviceLabel".into())?;
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
    Ok(libparsec::UserGreetInProgress4Info {
        handle,
        requested_human_handle,
        requested_device_label,
    })
}

#[allow(dead_code)]
fn struct_usergreetinprogress4info_rs_to_js(
    rs_obj: libparsec::UserGreetInProgress4Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_requested_human_handle = match rs_obj.requested_human_handle {
        Some(val) => JsValue::from_str(val.as_ref()),
        None => JsValue::NULL,
    };
    Reflect::set(
        &js_obj,
        &"requestedHumanHandle".into(),
        &js_requested_human_handle,
    )?;
    let js_requested_device_label = match rs_obj.requested_device_label {
        Some(val) => JsValue::from_str(val.as_ref()),
        None => JsValue::NULL,
    };
    Reflect::set(
        &js_obj,
        &"requestedDeviceLabel".into(),
        &js_requested_device_label,
    )?;
    Ok(js_obj)
}

// DeviceGreetInProgress4Info

#[allow(dead_code)]
fn struct_devicegreetinprogress4info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceGreetInProgress4Info, JsValue> {
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
    let requested_device_label = {
        let js_val = Reflect::get(&obj, &"requestedDeviceLabel".into())?;
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
    Ok(libparsec::DeviceGreetInProgress4Info {
        handle,
        requested_device_label,
    })
}

#[allow(dead_code)]
fn struct_devicegreetinprogress4info_rs_to_js(
    rs_obj: libparsec::DeviceGreetInProgress4Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_requested_device_label = match rs_obj.requested_device_label {
        Some(val) => JsValue::from_str(val.as_ref()),
        None => JsValue::NULL,
    };
    Reflect::set(
        &js_obj,
        &"requestedDeviceLabel".into(),
        &js_requested_device_label,
    )?;
    Ok(js_obj)
}

// CancelError

#[allow(dead_code)]
fn variant_cancelerror_rs_to_js(rs_obj: libparsec::CancelError) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::CancelError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::CancelError::NotBinded { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"NotBinded".into())?;
        }
    }
    Ok(js_obj)
}

// RealmRole

#[allow(dead_code)]
fn variant_realmrole_js_to_rs(obj: JsValue) -> Result<libparsec::RealmRole, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("Contributor") => Ok(libparsec::RealmRole::Contributor {}),
        tag if tag == JsValue::from_str("Manager") => Ok(libparsec::RealmRole::Manager {}),
        tag if tag == JsValue::from_str("Owner") => Ok(libparsec::RealmRole::Owner {}),
        tag if tag == JsValue::from_str("Reader") => Ok(libparsec::RealmRole::Reader {}),
        _ => Err(JsValue::from(TypeError::new("Object is not a RealmRole"))),
    }
}

#[allow(dead_code)]
fn variant_realmrole_rs_to_js(rs_obj: libparsec::RealmRole) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::RealmRole::Contributor { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Contributor".into())?;
        }
        libparsec::RealmRole::Manager { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Manager".into())?;
        }
        libparsec::RealmRole::Owner { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Owner".into())?;
        }
        libparsec::RealmRole::Reader { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Reader".into())?;
        }
    }
    Ok(js_obj)
}

// DeviceAccessStrategy

#[allow(dead_code)]
fn variant_deviceaccessstrategy_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceAccessStrategy, JsValue> {
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
            let key_file = {
                let js_val = Reflect::get(&obj, &"keyFile".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, &'static str> {
                            Ok(std::path::PathBuf::from(s))
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid Path"))?
            };
            Ok(libparsec::DeviceAccessStrategy::Password { password, key_file })
        }
        tag if tag == JsValue::from_str("Smartcard") => {
            let key_file = {
                let js_val = Reflect::get(&obj, &"keyFile".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, &'static str> {
                            Ok(std::path::PathBuf::from(s))
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid Path"))?
            };
            Ok(libparsec::DeviceAccessStrategy::Smartcard { key_file })
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a DeviceAccessStrategy",
        ))),
    }
}

#[allow(dead_code)]
fn variant_deviceaccessstrategy_rs_to_js(
    rs_obj: libparsec::DeviceAccessStrategy,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::DeviceAccessStrategy::Password {
            password, key_file, ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"Password".into())?;
            let js_password = JsValue::from_str(password.as_ref());
            Reflect::set(&js_obj, &"password".into(), &js_password)?;
            let js_key_file = JsValue::from_str({
                let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                };
                match custom_to_rs_string(key_file) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"keyFile".into(), &js_key_file)?;
        }
        libparsec::DeviceAccessStrategy::Smartcard { key_file, .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Smartcard".into())?;
            let js_key_file = JsValue::from_str({
                let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                };
                match custom_to_rs_string(key_file) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"keyFile".into(), &js_key_file)?;
        }
    }
    Ok(js_obj)
}

// ClientStartError

#[allow(dead_code)]
fn variant_clientstarterror_rs_to_js(
    rs_obj: libparsec::ClientStartError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientStartError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::ClientStartError::LoadDeviceDecryptionFailed { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"LoadDeviceDecryptionFailed".into())?;
        }
        libparsec::ClientStartError::LoadDeviceInvalidData { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"LoadDeviceInvalidData".into())?;
        }
        libparsec::ClientStartError::LoadDeviceInvalidPath { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"LoadDeviceInvalidPath".into())?;
        }
    }
    Ok(js_obj)
}

// ClientStopError

#[allow(dead_code)]
fn variant_clientstoperror_rs_to_js(
    rs_obj: libparsec::ClientStopError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientStopError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
    }
    Ok(js_obj)
}

// ClientListWorkspacesError

#[allow(dead_code)]
fn variant_clientlistworkspaceserror_rs_to_js(
    rs_obj: libparsec::ClientListWorkspacesError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientListWorkspacesError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
    }
    Ok(js_obj)
}

// ClientWorkspaceCreateError

#[allow(dead_code)]
fn variant_clientworkspacecreateerror_rs_to_js(
    rs_obj: libparsec::ClientWorkspaceCreateError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientWorkspaceCreateError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
    }
    Ok(js_obj)
}

// ClientWorkspaceRenameError

#[allow(dead_code)]
fn variant_clientworkspacerenameerror_rs_to_js(
    rs_obj: libparsec::ClientWorkspaceRenameError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientWorkspaceRenameError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::ClientWorkspaceRenameError::UnknownWorkspace { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"UnknownWorkspace".into())?;
        }
    }
    Ok(js_obj)
}

// ClientWorkspaceShareError

#[allow(dead_code)]
fn variant_clientworkspaceshareerror_rs_to_js(
    rs_obj: libparsec::ClientWorkspaceShareError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientWorkspaceShareError::BadTimestamp {
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
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"serverTimestamp".into(), &js_server_timestamp)?;
            let js_client_timestamp = JsValue::from_str({
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"clientTimestamp".into(), &js_client_timestamp)?;
            let js_ballpark_client_early_offset = ballpark_client_early_offset.into();
            Reflect::set(
                &js_obj,
                &"ballparkClientEarlyOffset".into(),
                &js_ballpark_client_early_offset,
            )?;
            let js_ballpark_client_late_offset = ballpark_client_late_offset.into();
            Reflect::set(
                &js_obj,
                &"ballparkClientLateOffset".into(),
                &js_ballpark_client_late_offset,
            )?;
        }
        libparsec::ClientWorkspaceShareError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::ClientWorkspaceShareError::NotAllowed { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"NotAllowed".into())?;
        }
        libparsec::ClientWorkspaceShareError::Offline { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Offline".into())?;
        }
        libparsec::ClientWorkspaceShareError::OutsiderCannotBeManagerOrOwner { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"OutsiderCannotBeManagerOrOwner".into(),
            )?;
        }
        libparsec::ClientWorkspaceShareError::RevokedRecipient { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"RevokedRecipient".into())?;
        }
        libparsec::ClientWorkspaceShareError::ShareToSelf { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ShareToSelf".into())?;
        }
        libparsec::ClientWorkspaceShareError::UnknownRecipient { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"UnknownRecipient".into())?;
        }
        libparsec::ClientWorkspaceShareError::UnknownRecipientOrWorkspace { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"UnknownRecipientOrWorkspace".into(),
            )?;
        }
        libparsec::ClientWorkspaceShareError::UnknownWorkspace { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"UnknownWorkspace".into())?;
        }
        libparsec::ClientWorkspaceShareError::WorkspaceInMaintenance { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"WorkspaceInMaintenance".into())?;
        }
    }
    Ok(js_obj)
}

// UserProfile

#[allow(dead_code)]
fn variant_userprofile_js_to_rs(obj: JsValue) -> Result<libparsec::UserProfile, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("Admin") => Ok(libparsec::UserProfile::Admin {}),
        tag if tag == JsValue::from_str("Outsider") => Ok(libparsec::UserProfile::Outsider {}),
        tag if tag == JsValue::from_str("Standard") => Ok(libparsec::UserProfile::Standard {}),
        _ => Err(JsValue::from(TypeError::new("Object is not a UserProfile"))),
    }
}

#[allow(dead_code)]
fn variant_userprofile_rs_to_js(rs_obj: libparsec::UserProfile) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::UserProfile::Admin { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Admin".into())?;
        }
        libparsec::UserProfile::Outsider { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Outsider".into())?;
        }
        libparsec::UserProfile::Standard { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Standard".into())?;
        }
    }
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

// ClaimerGreeterAbortOperationError

#[allow(dead_code)]
fn variant_claimergreeterabortoperationerror_rs_to_js(
    rs_obj: libparsec::ClaimerGreeterAbortOperationError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClaimerGreeterAbortOperationError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
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
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"serverTimestamp".into(), &js_server_timestamp)?;
            let js_client_timestamp = JsValue::from_str({
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"clientTimestamp".into(), &js_client_timestamp)?;
            let js_ballpark_client_early_offset = ballpark_client_early_offset.into();
            Reflect::set(
                &js_obj,
                &"ballparkClientEarlyOffset".into(),
                &js_ballpark_client_early_offset,
            )?;
            let js_ballpark_client_late_offset = ballpark_client_late_offset.into();
            Reflect::set(
                &js_obj,
                &"ballparkClientLateOffset".into(),
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
        libparsec::ClaimInProgressError::Cancelled { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Cancelled".into())?;
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

// UserOrDeviceClaimInitialInfo

#[allow(dead_code)]
fn variant_userordeviceclaiminitialinfo_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserOrDeviceClaimInitialInfo, JsValue> {
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
                let js_val = Reflect::get(&obj, &"greeterUserId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
                    .parse()
                    .map_err(|_| TypeError::new("Not a valid UserID"))?
            };
            let greeter_human_handle = {
                let js_val = Reflect::get(&obj, &"greeterHumanHandle".into())?;
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
            Ok(libparsec::UserOrDeviceClaimInitialInfo::Device {
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
                let js_val = Reflect::get(&obj, &"claimerEmail".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            let greeter_user_id = {
                let js_val = Reflect::get(&obj, &"greeterUserId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
                    .parse()
                    .map_err(|_| TypeError::new("Not a valid UserID"))?
            };
            let greeter_human_handle = {
                let js_val = Reflect::get(&obj, &"greeterHumanHandle".into())?;
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
            Ok(libparsec::UserOrDeviceClaimInitialInfo::User {
                handle,
                claimer_email,
                greeter_user_id,
                greeter_human_handle,
            })
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a UserOrDeviceClaimInitialInfo",
        ))),
    }
}

#[allow(dead_code)]
fn variant_userordeviceclaiminitialinfo_rs_to_js(
    rs_obj: libparsec::UserOrDeviceClaimInitialInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::UserOrDeviceClaimInitialInfo::Device {
            handle,
            greeter_user_id,
            greeter_human_handle,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"Device".into())?;
            let js_handle = JsValue::from(handle);
            Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
            let js_greeter_user_id = JsValue::from_str(greeter_user_id.as_ref());
            Reflect::set(&js_obj, &"greeterUserId".into(), &js_greeter_user_id)?;
            let js_greeter_human_handle = match greeter_human_handle {
                Some(val) => JsValue::from_str(val.as_ref()),
                None => JsValue::NULL,
            };
            Reflect::set(
                &js_obj,
                &"greeterHumanHandle".into(),
                &js_greeter_human_handle,
            )?;
        }
        libparsec::UserOrDeviceClaimInitialInfo::User {
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
            Reflect::set(&js_obj, &"claimerEmail".into(), &js_claimer_email)?;
            let js_greeter_user_id = JsValue::from_str(greeter_user_id.as_ref());
            Reflect::set(&js_obj, &"greeterUserId".into(), &js_greeter_user_id)?;
            let js_greeter_human_handle = match greeter_human_handle {
                Some(val) => JsValue::from_str(val.as_ref()),
                None => JsValue::NULL,
            };
            Reflect::set(
                &js_obj,
                &"greeterHumanHandle".into(),
                &js_greeter_human_handle,
            )?;
        }
    }
    Ok(js_obj)
}

// InvitationStatus

#[allow(dead_code)]
fn variant_invitationstatus_js_to_rs(obj: JsValue) -> Result<libparsec::InvitationStatus, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("Deleted") => Ok(libparsec::InvitationStatus::Deleted {}),
        tag if tag == JsValue::from_str("Idle") => Ok(libparsec::InvitationStatus::Idle {}),
        tag if tag == JsValue::from_str("Ready") => Ok(libparsec::InvitationStatus::Ready {}),
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a InvitationStatus",
        ))),
    }
}

#[allow(dead_code)]
fn variant_invitationstatus_rs_to_js(
    rs_obj: libparsec::InvitationStatus,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::InvitationStatus::Deleted { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Deleted".into())?;
        }
        libparsec::InvitationStatus::Idle { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Idle".into())?;
        }
        libparsec::InvitationStatus::Ready { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Ready".into())?;
        }
    }
    Ok(js_obj)
}

// InvitationEmailSentStatus

#[allow(dead_code)]
fn variant_invitationemailsentstatus_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::InvitationEmailSentStatus, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("BadRecipient") => {
            Ok(libparsec::InvitationEmailSentStatus::BadRecipient {})
        }
        tag if tag == JsValue::from_str("NotAvailable") => {
            Ok(libparsec::InvitationEmailSentStatus::NotAvailable {})
        }
        tag if tag == JsValue::from_str("Success") => {
            Ok(libparsec::InvitationEmailSentStatus::Success {})
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a InvitationEmailSentStatus",
        ))),
    }
}

#[allow(dead_code)]
fn variant_invitationemailsentstatus_rs_to_js(
    rs_obj: libparsec::InvitationEmailSentStatus,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::InvitationEmailSentStatus::BadRecipient { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"BadRecipient".into())?;
        }
        libparsec::InvitationEmailSentStatus::NotAvailable { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"NotAvailable".into())?;
        }
        libparsec::InvitationEmailSentStatus::Success { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Success".into())?;
        }
    }
    Ok(js_obj)
}

// NewUserInvitationError

#[allow(dead_code)]
fn variant_newuserinvitationerror_rs_to_js(
    rs_obj: libparsec::NewUserInvitationError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::NewUserInvitationError::AlreadyMember { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AlreadyMember".into())?;
        }
        libparsec::NewUserInvitationError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::NewUserInvitationError::NotAllowed { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"NotAllowed".into())?;
        }
        libparsec::NewUserInvitationError::Offline { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Offline".into())?;
        }
    }
    Ok(js_obj)
}

// NewDeviceInvitationError

#[allow(dead_code)]
fn variant_newdeviceinvitationerror_rs_to_js(
    rs_obj: libparsec::NewDeviceInvitationError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::NewDeviceInvitationError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::NewDeviceInvitationError::Offline { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Offline".into())?;
        }
        libparsec::NewDeviceInvitationError::SendEmailToUserWithoutEmail { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"SendEmailToUserWithoutEmail".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// DeleteInvitationError

#[allow(dead_code)]
fn variant_deleteinvitationerror_rs_to_js(
    rs_obj: libparsec::DeleteInvitationError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::DeleteInvitationError::AlreadyDeleted { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AlreadyDeleted".into())?;
        }
        libparsec::DeleteInvitationError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::DeleteInvitationError::NotFound { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"NotFound".into())?;
        }
        libparsec::DeleteInvitationError::Offline { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Offline".into())?;
        }
    }
    Ok(js_obj)
}

// InviteListItem

#[allow(dead_code)]
fn variant_invitelistitem_js_to_rs(obj: JsValue) -> Result<libparsec::InviteListItem, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("Device") => {
            let token = {
                let js_val = Reflect::get(&obj, &"token".into())?;
                js_val
                    .dyn_into::<Uint8Array>()
                    .map_err(|_| TypeError::new("Not a Uint8Array"))
                    .and_then(|x| {
                        let custom_from_rs_bytes = |x: &[u8]| -> Result<_, _> {
                            libparsec::InvitationToken::try_from(x).map_err(|e| e.to_string())
                        };
                        custom_from_rs_bytes(&x.to_vec()).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid InvitationToken"))?
            };
            let created_on = {
                let js_val = Reflect::get(&obj, &"createdOn".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::DateTime::from_rfc3339(&s).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid DateTime"))?
            };
            let status = {
                let js_val = Reflect::get(&obj, &"status".into())?;
                variant_invitationstatus_js_to_rs(js_val)?
            };
            Ok(libparsec::InviteListItem::Device {
                token,
                created_on,
                status,
            })
        }
        tag if tag == JsValue::from_str("User") => {
            let token = {
                let js_val = Reflect::get(&obj, &"token".into())?;
                js_val
                    .dyn_into::<Uint8Array>()
                    .map_err(|_| TypeError::new("Not a Uint8Array"))
                    .and_then(|x| {
                        let custom_from_rs_bytes = |x: &[u8]| -> Result<_, _> {
                            libparsec::InvitationToken::try_from(x).map_err(|e| e.to_string())
                        };
                        custom_from_rs_bytes(&x.to_vec()).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid InvitationToken"))?
            };
            let created_on = {
                let js_val = Reflect::get(&obj, &"createdOn".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::DateTime::from_rfc3339(&s).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid DateTime"))?
            };
            let claimer_email = {
                let js_val = Reflect::get(&obj, &"claimerEmail".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            let status = {
                let js_val = Reflect::get(&obj, &"status".into())?;
                variant_invitationstatus_js_to_rs(js_val)?
            };
            Ok(libparsec::InviteListItem::User {
                token,
                created_on,
                claimer_email,
                status,
            })
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a InviteListItem",
        ))),
    }
}

#[allow(dead_code)]
fn variant_invitelistitem_rs_to_js(rs_obj: libparsec::InviteListItem) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::InviteListItem::Device {
            token,
            created_on,
            status,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"Device".into())?;
            let js_token = JsValue::from(Uint8Array::from({
                let custom_to_rs_bytes =
                    |x: libparsec::InvitationToken| -> Result<_, &'static str> {
                        Ok(x.as_bytes().to_owned())
                    };
                match custom_to_rs_bytes(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            }));
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
            let js_created_on = JsValue::from_str({
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
            let js_status = variant_invitationstatus_rs_to_js(status)?;
            Reflect::set(&js_obj, &"status".into(), &js_status)?;
        }
        libparsec::InviteListItem::User {
            token,
            created_on,
            claimer_email,
            status,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"User".into())?;
            let js_token = JsValue::from(Uint8Array::from({
                let custom_to_rs_bytes =
                    |x: libparsec::InvitationToken| -> Result<_, &'static str> {
                        Ok(x.as_bytes().to_owned())
                    };
                match custom_to_rs_bytes(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            }));
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
            let js_created_on = JsValue::from_str({
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
            let js_claimer_email = claimer_email.into();
            Reflect::set(&js_obj, &"claimerEmail".into(), &js_claimer_email)?;
            let js_status = variant_invitationstatus_rs_to_js(status)?;
            Reflect::set(&js_obj, &"status".into(), &js_status)?;
        }
    }
    Ok(js_obj)
}

// ListInvitationsError

#[allow(dead_code)]
fn variant_listinvitationserror_rs_to_js(
    rs_obj: libparsec::ListInvitationsError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ListInvitationsError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::ListInvitationsError::Offline { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Offline".into())?;
        }
    }
    Ok(js_obj)
}

// ClientStartInvitationGreetError

#[allow(dead_code)]
fn variant_clientstartinvitationgreeterror_rs_to_js(
    rs_obj: libparsec::ClientStartInvitationGreetError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientStartInvitationGreetError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
    }
    Ok(js_obj)
}

// GreetInProgressError

#[allow(dead_code)]
fn variant_greetinprogresserror_rs_to_js(
    rs_obj: libparsec::GreetInProgressError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::GreetInProgressError::ActiveUsersLimitReached { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ActiveUsersLimitReached".into())?;
        }
        libparsec::GreetInProgressError::AlreadyUsed { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AlreadyUsed".into())?;
        }
        libparsec::GreetInProgressError::BadTimestamp {
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
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"serverTimestamp".into(), &js_server_timestamp)?;
            let js_client_timestamp = JsValue::from_str({
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"clientTimestamp".into(), &js_client_timestamp)?;
            let js_ballpark_client_early_offset = ballpark_client_early_offset.into();
            Reflect::set(
                &js_obj,
                &"ballparkClientEarlyOffset".into(),
                &js_ballpark_client_early_offset,
            )?;
            let js_ballpark_client_late_offset = ballpark_client_late_offset.into();
            Reflect::set(
                &js_obj,
                &"ballparkClientLateOffset".into(),
                &js_ballpark_client_late_offset,
            )?;
        }
        libparsec::GreetInProgressError::Cancelled { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Cancelled".into())?;
        }
        libparsec::GreetInProgressError::CorruptedInviteUserData { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"CorruptedInviteUserData".into())?;
        }
        libparsec::GreetInProgressError::DeviceAlreadyExists { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"DeviceAlreadyExists".into())?;
        }
        libparsec::GreetInProgressError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::GreetInProgressError::NonceMismatch { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"NonceMismatch".into())?;
        }
        libparsec::GreetInProgressError::NotFound { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"NotFound".into())?;
        }
        libparsec::GreetInProgressError::Offline { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Offline".into())?;
        }
        libparsec::GreetInProgressError::PeerReset { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"PeerReset".into())?;
        }
        libparsec::GreetInProgressError::UserAlreadyExists { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"UserAlreadyExists".into())?;
        }
        libparsec::GreetInProgressError::UserCreateNotAllowed { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"UserCreateNotAllowed".into())?;
        }
    }
    Ok(js_obj)
}

// cancel
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn cancel(canceller: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::cancel(canceller);
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let _ = value;
                    JsValue::null()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_cancelerror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// new_canceller
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn newCanceller() -> Promise {
    future_to_promise(async move {
        let ret = libparsec::new_canceller();
        Ok(JsValue::from(ret))
    })
}

// client_start
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientStart(config: Object, on_event_callback: Function, access: Object) -> Promise {
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

        let access = access.into();
        let access = variant_deviceaccessstrategy_js_to_rs(access)?;

        let ret = libparsec::client_start(config, on_event_callback, access).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from(value);
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_clientstarterror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_stop
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientStop(client: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::client_stop(client).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let _ = value;
                    JsValue::null()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_clientstoperror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_list_workspaces
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientListWorkspaces(client: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::client_list_workspaces(client).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(value.len() as u32);
                    for (i, elem) in value.into_iter().enumerate() {
                        let js_elem = {
                            let (x1, x2) = elem;
                            let js_array = Array::new_with_length(2);
                            let js_value = JsValue::from(Uint8Array::from({
                                let custom_to_rs_bytes =
                                    |x: libparsec::RealmID| -> Result<_, &'static str> {
                                        Ok(x.as_bytes().to_owned())
                                    };
                                match custom_to_rs_bytes(x1) {
                                    Ok(ok) => ok,
                                    Err(err) => {
                                        return Err(JsValue::from(TypeError::new(err.as_ref())))
                                    }
                                }
                                .as_ref()
                            }));
                            js_array.push(&js_value);
                            let js_value = JsValue::from_str(x2.as_ref());
                            js_array.push(&js_value);
                            js_array.into()
                        };
                        js_array.set(i as u32, js_elem);
                    }
                    js_array.into()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_clientlistworkspaceserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_workspace_create
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientWorkspaceCreate(client: u32, name: String) -> Promise {
    future_to_promise(async move {
        let name = {
            let custom_from_rs_string = |s: String| -> Result<_, _> {
                s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(name).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_workspace_create(client, name).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from(Uint8Array::from({
                    let custom_to_rs_bytes = |x: libparsec::RealmID| -> Result<_, &'static str> {
                        Ok(x.as_bytes().to_owned())
                    };
                    match custom_to_rs_bytes(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                    .as_ref()
                }));
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_clientworkspacecreateerror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_workspace_rename
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientWorkspaceRename(client: u32, realm_id: Uint8Array, new_name: String) -> Promise {
    future_to_promise(async move {
        let realm_id = {
            let custom_from_rs_bytes = |x: &[u8]| -> Result<_, _> {
                libparsec::RealmID::try_from(x).map_err(|e| e.to_string())
            };
            custom_from_rs_bytes(&realm_id.to_vec()).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let new_name = {
            let custom_from_rs_string = |s: String| -> Result<_, _> {
                s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(new_name).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_workspace_rename(client, realm_id, new_name).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let _ = value;
                    JsValue::null()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_clientworkspacerenameerror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_workspace_share
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientWorkspaceShare(
    client: u32,
    realm_id: Uint8Array,
    recipient: String,
    role: Option<Object>,
) -> Promise {
    future_to_promise(async move {
        let realm_id = {
            let custom_from_rs_bytes = |x: &[u8]| -> Result<_, _> {
                libparsec::RealmID::try_from(x).map_err(|e| e.to_string())
            };
            custom_from_rs_bytes(&realm_id.to_vec()).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let recipient = recipient
            .parse()
            .map_err(|_| JsValue::from(TypeError::new("Not a valid UserID")))?;

        let role = match role {
            Some(role) => {
                let role = role.into();
                let role = variant_realmrole_js_to_rs(role)?;

                Some(role)
            }
            None => None,
        };

        let ret = libparsec::client_workspace_share(client, realm_id, recipient, role).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let _ = value;
                    JsValue::null()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_clientworkspaceshareerror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// claimer_greeter_abort_operation
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerGreeterAbortOperation(handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::claimer_greeter_abort_operation(handle);
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let _ = value;
                    JsValue::null()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claimergreeterabortoperationerror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
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
                let js_value = variant_userordeviceclaiminitialinfo_rs_to_js(value)?;
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

// claimer_user_initial_do_wait_peer
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerUserInitialDoWaitPeer(canceller: u32, handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::claimer_user_initial_do_wait_peer(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_userclaiminprogress1info_rs_to_js(value)?;
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

// claimer_device_initial_do_wait_peer
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerDeviceInitialDoWaitPeer(canceller: u32, handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::claimer_device_initial_do_wait_peer(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_deviceclaiminprogress1info_rs_to_js(value)?;
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

// claimer_user_in_progress_1_do_signify_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerUserInProgress1DoSignifyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::claimer_user_in_progress_1_do_signify_trust(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_userclaiminprogress2info_rs_to_js(value)?;
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

// claimer_device_in_progress_1_do_signify_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerDeviceInProgress1DoSignifyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::claimer_device_in_progress_1_do_signify_trust(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_deviceclaiminprogress2info_rs_to_js(value)?;
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
pub fn claimerUserInProgress2DoWaitPeerTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::claimer_user_in_progress_2_do_wait_peer_trust(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_userclaiminprogress3info_rs_to_js(value)?;
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
pub fn claimerDeviceInProgress2DoWaitPeerTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(async move {
        let ret =
            libparsec::claimer_device_in_progress_2_do_wait_peer_trust(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_deviceclaiminprogress3info_rs_to_js(value)?;
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
    canceller: u32,
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
            canceller,
            handle,
            requested_device_label,
            requested_human_handle,
        )
        .await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_userclaimfinalizeinfo_rs_to_js(value)?;
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
    canceller: u32,
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

        let ret = libparsec::claimer_device_in_progress_3_do_claim(
            canceller,
            handle,
            requested_device_label,
        )
        .await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_deviceclaimfinalizeinfo_rs_to_js(value)?;
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

// client_new_user_invitation
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientNewUserInvitation(client: u32, claimer_email: String, send_email: bool) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::client_new_user_invitation(client, claimer_email, send_email).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let (x1, x2) = value;
                    let js_array = Array::new_with_length(2);
                    let js_value = JsValue::from(Uint8Array::from({
                        let custom_to_rs_bytes =
                            |x: libparsec::InvitationToken| -> Result<_, &'static str> {
                                Ok(x.as_bytes().to_owned())
                            };
                        match custom_to_rs_bytes(x1) {
                            Ok(ok) => ok,
                            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                        }
                        .as_ref()
                    }));
                    js_array.push(&js_value);
                    let js_value = variant_invitationemailsentstatus_rs_to_js(x2)?;
                    js_array.push(&js_value);
                    js_array.into()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_newuserinvitationerror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_new_device_invitation
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientNewDeviceInvitation(client: u32, send_email: bool) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::client_new_device_invitation(client, send_email).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let (x1, x2) = value;
                    let js_array = Array::new_with_length(2);
                    let js_value = JsValue::from(Uint8Array::from({
                        let custom_to_rs_bytes =
                            |x: libparsec::InvitationToken| -> Result<_, &'static str> {
                                Ok(x.as_bytes().to_owned())
                            };
                        match custom_to_rs_bytes(x1) {
                            Ok(ok) => ok,
                            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                        }
                        .as_ref()
                    }));
                    js_array.push(&js_value);
                    let js_value = variant_invitationemailsentstatus_rs_to_js(x2)?;
                    js_array.push(&js_value);
                    js_array.into()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_newdeviceinvitationerror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_delete_invitation
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientDeleteInvitation(client: u32, token: Uint8Array) -> Promise {
    future_to_promise(async move {
        let token = {
            let custom_from_rs_bytes = |x: &[u8]| -> Result<_, _> {
                libparsec::InvitationToken::try_from(x).map_err(|e| e.to_string())
            };
            custom_from_rs_bytes(&token.to_vec()).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_delete_invitation(client, token).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let _ = value;
                    JsValue::null()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_deleteinvitationerror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_list_invitations
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientListInvitations(client: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::client_list_invitations(client).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(value.len() as u32);
                    for (i, elem) in value.into_iter().enumerate() {
                        let js_elem = variant_invitelistitem_rs_to_js(elem)?;
                        js_array.set(i as u32, js_elem);
                    }
                    js_array.into()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_listinvitationserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_start_user_invitation_greet
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientStartUserInvitationGreet(client: u32, token: Uint8Array) -> Promise {
    future_to_promise(async move {
        let token = {
            let custom_from_rs_bytes = |x: &[u8]| -> Result<_, _> {
                libparsec::InvitationToken::try_from(x).map_err(|e| e.to_string())
            };
            custom_from_rs_bytes(&token.to_vec()).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_start_user_invitation_greet(client, token).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_usergreetinitialinfo_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_clientstartinvitationgreeterror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_start_device_invitation_greet
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientStartDeviceInvitationGreet(client: u32, token: Uint8Array) -> Promise {
    future_to_promise(async move {
        let token = {
            let custom_from_rs_bytes = |x: &[u8]| -> Result<_, _> {
                libparsec::InvitationToken::try_from(x).map_err(|e| e.to_string())
            };
            custom_from_rs_bytes(&token.to_vec()).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_start_device_invitation_greet(client, token).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_devicegreetinitialinfo_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_clientstartinvitationgreeterror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// greeter_user_initial_do_wait_peer
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterUserInitialDoWaitPeer(canceller: u32, handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::greeter_user_initial_do_wait_peer(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_usergreetinprogress1info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greetinprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// greeter_device_initial_do_wait_peer
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterDeviceInitialDoWaitPeer(canceller: u32, handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::greeter_device_initial_do_wait_peer(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_devicegreetinprogress1info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greetinprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// greeter_user_in_progress_1_do_wait_peer_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterUserInProgress1DoWaitPeerTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::greeter_user_in_progress_1_do_wait_peer_trust(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_usergreetinprogress2info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greetinprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// greeter_device_in_progress_1_do_wait_peer_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterDeviceInProgress1DoWaitPeerTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(async move {
        let ret =
            libparsec::greeter_device_in_progress_1_do_wait_peer_trust(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_devicegreetinprogress2info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greetinprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// greeter_user_in_progress_2_do_signify_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterUserInProgress2DoSignifyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::greeter_user_in_progress_2_do_signify_trust(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_usergreetinprogress3info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greetinprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// greeter_device_in_progress_2_do_signify_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterDeviceInProgress2DoSignifyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::greeter_device_in_progress_2_do_signify_trust(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_devicegreetinprogress3info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greetinprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// greeter_user_in_progress_3_do_get_claim_requests
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterUserInProgress3DoGetClaimRequests(canceller: u32, handle: u32) -> Promise {
    future_to_promise(async move {
        let ret =
            libparsec::greeter_user_in_progress_3_do_get_claim_requests(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_usergreetinprogress4info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greetinprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// greeter_device_in_progress_3_do_get_claim_requests
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterDeviceInProgress3DoGetClaimRequests(canceller: u32, handle: u32) -> Promise {
    future_to_promise(async move {
        let ret =
            libparsec::greeter_device_in_progress_3_do_get_claim_requests(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_devicegreetinprogress4info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greetinprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// greeter_user_in_progress_4_do_create
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterUserInProgress4DoCreate(
    canceller: u32,
    handle: u32,
    human_handle: Option<String>,
    device_label: Option<String>,
    profile: Object,
) -> Promise {
    future_to_promise(async move {
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

        let profile = profile.into();
        let profile = variant_userprofile_js_to_rs(profile)?;

        let ret = libparsec::greeter_user_in_progress_4_do_create(
            canceller,
            handle,
            human_handle,
            device_label,
            profile,
        )
        .await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let _ = value;
                    JsValue::null()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greetinprogresserror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// greeter_device_in_progress_4_do_create
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterDeviceInProgress4DoCreate(
    canceller: u32,
    handle: u32,
    device_label: Option<String>,
) -> Promise {
    future_to_promise(async move {
        let device_label = match device_label {
            Some(device_label) => {
                let device_label = device_label
                    .parse()
                    .map_err(|_| JsValue::from(TypeError::new("Not a valid DeviceLabel")))?;

                Some(device_label)
            }
            None => None,
        };

        let ret =
            libparsec::greeter_device_in_progress_4_do_create(canceller, handle, device_label)
                .await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let _ = value;
                    JsValue::null()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greetinprogresserror_rs_to_js(err)?;
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

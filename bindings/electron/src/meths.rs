// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

#[allow(unused_imports)]
use neon::{prelude::*, types::buffer::TypedArray};

// ClientConfig

#[allow(dead_code)]
fn struct_clientconfig_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ClientConfig> {
    let config_dir = {
        let js_val: Handle<JsString> = obj.get(cx, "configDir")?;
        {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let data_base_dir = {
        let js_val: Handle<JsString> = obj.get(cx, "dataBaseDir")?;
        {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let mountpoint_base_dir = {
        let js_val: Handle<JsString> = obj.get(cx, "mountpointBaseDir")?;
        {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let workspace_storage_cache_size = {
        let js_val: Handle<JsObject> = obj.get(cx, "workspaceStorageCacheSize")?;
        variant_workspacestoragecachesize_js_to_rs(cx, js_val)?
    };
    Ok(libparsec::ClientConfig {
        config_dir,
        data_base_dir,
        mountpoint_base_dir,
        workspace_storage_cache_size,
    })
}

#[allow(dead_code)]
fn struct_clientconfig_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientConfig,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_config_dir = JsString::try_new(cx, {
        let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        };
        match custom_to_rs_string(rs_obj.config_dir) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "configDir", js_config_dir)?;
    let js_data_base_dir = JsString::try_new(cx, {
        let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        };
        match custom_to_rs_string(rs_obj.data_base_dir) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "dataBaseDir", js_data_base_dir)?;
    let js_mountpoint_base_dir = JsString::try_new(cx, {
        let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        };
        match custom_to_rs_string(rs_obj.mountpoint_base_dir) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "mountpointBaseDir", js_mountpoint_base_dir)?;
    let js_workspace_storage_cache_size =
        variant_workspacestoragecachesize_rs_to_js(cx, rs_obj.workspace_storage_cache_size)?;
    js_obj.set(
        cx,
        "workspaceStorageCacheSize",
        js_workspace_storage_cache_size,
    )?;
    Ok(js_obj)
}

// HumanHandle

#[allow(dead_code)]
fn struct_humanhandle_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::HumanHandle> {
    let email = {
        let js_val: Handle<JsString> = obj.get(cx, "email")?;
        js_val.value(cx)
    };
    let label = {
        let js_val: Handle<JsString> = obj.get(cx, "label")?;
        js_val.value(cx)
    };

    |email: &str, label: &str| -> Result<_, String> {
        libparsec::HumanHandle::new(email, label).map_err(|e| e.to_string())
    }(&email, &label)
    .or_else(|e| cx.throw_error(e))
}

#[allow(dead_code)]
fn struct_humanhandle_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::HumanHandle,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_email = JsString::try_new(
        cx,
        (|obj| {
            fn access(obj: &libparsec::HumanHandle) -> &str {
                obj.email()
            }
            access(obj)
        })(&rs_obj),
    )
    .or_throw(cx)?;
    js_obj.set(cx, "email", js_email)?;
    let js_label = JsString::try_new(
        cx,
        (|obj| {
            fn access(obj: &libparsec::HumanHandle) -> &str {
                obj.label()
            }
            access(obj)
        })(&rs_obj),
    )
    .or_throw(cx)?;
    js_obj.set(cx, "label", js_label)?;
    Ok(js_obj)
}

// AvailableDevice

#[allow(dead_code)]
fn struct_availabledevice_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::AvailableDevice> {
    let key_file_path = {
        let js_val: Handle<JsString> = obj.get(cx, "keyFilePath")?;
        {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let organization_id = {
        let js_val: Handle<JsString> = obj.get(cx, "organizationId")?;
        {
            match js_val.value(cx).parse() {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let device_id = {
        let js_val: Handle<JsString> = obj.get(cx, "deviceId")?;
        {
            match js_val.value(cx).parse() {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let human_handle = {
        let js_val: Handle<JsValue> = obj.get(cx, "humanHandle")?;
        {
            if js_val.is_a::<JsNull, _>(cx) {
                None
            } else {
                let js_val = js_val.downcast_or_throw::<JsObject, _>(cx)?;
                Some(struct_humanhandle_js_to_rs(cx, js_val)?)
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
                Some({
                    match js_val.value(cx).parse() {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
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
    let js_key_file_path = JsString::try_new(cx, {
        let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        };
        match custom_to_rs_string(rs_obj.key_file_path) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "keyFilePath", js_key_file_path)?;
    let js_organization_id = JsString::try_new(cx, rs_obj.organization_id).or_throw(cx)?;
    js_obj.set(cx, "organizationId", js_organization_id)?;
    let js_device_id = JsString::try_new(cx, rs_obj.device_id).or_throw(cx)?;
    js_obj.set(cx, "deviceId", js_device_id)?;
    let js_human_handle = match rs_obj.human_handle {
        Some(elem) => struct_humanhandle_rs_to_js(cx, elem)?.as_value(cx),
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

// UserClaimInProgress1Info

#[allow(dead_code)]
fn struct_userclaiminprogress1info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserClaimInProgress1Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let greeter_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "greeterSas")?;
        {
            match js_val.value(cx).parse() {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let greeter_sas_choices = {
        let js_val: Handle<JsArray> = obj.get(cx, "greeterSasChoices")?;
        {
            let size = js_val.len(cx);
            let mut v = Vec::with_capacity(size as usize);
            for i in 0..size {
                let js_item: Handle<JsString> = js_val.get(cx, i)?;
                v.push({
                    match js_item.value(cx).parse() {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                });
            }
            v
        }
    };
    Ok(libparsec::UserClaimInProgress1Info {
        handle,
        greeter_sas,
        greeter_sas_choices,
    })
}

#[allow(dead_code)]
fn struct_userclaiminprogress1info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserClaimInProgress1Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_greeter_sas = JsString::try_new(cx, rs_obj.greeter_sas).or_throw(cx)?;
    js_obj.set(cx, "greeterSas", js_greeter_sas)?;
    let js_greeter_sas_choices = {
        // JsArray::new allocates with `undefined` value, that's why we `set` value
        let js_array = JsArray::new(cx, rs_obj.greeter_sas_choices.len() as u32);
        for (i, elem) in rs_obj.greeter_sas_choices.into_iter().enumerate() {
            let js_elem = JsString::try_new(cx, elem).or_throw(cx)?;
            js_array.set(cx, i as u32, js_elem)?;
        }
        js_array
    };
    js_obj.set(cx, "greeterSasChoices", js_greeter_sas_choices)?;
    Ok(js_obj)
}

// DeviceClaimInProgress1Info

#[allow(dead_code)]
fn struct_deviceclaiminprogress1info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceClaimInProgress1Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let greeter_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "greeterSas")?;
        {
            match js_val.value(cx).parse() {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let greeter_sas_choices = {
        let js_val: Handle<JsArray> = obj.get(cx, "greeterSasChoices")?;
        {
            let size = js_val.len(cx);
            let mut v = Vec::with_capacity(size as usize);
            for i in 0..size {
                let js_item: Handle<JsString> = js_val.get(cx, i)?;
                v.push({
                    match js_item.value(cx).parse() {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                });
            }
            v
        }
    };
    Ok(libparsec::DeviceClaimInProgress1Info {
        handle,
        greeter_sas,
        greeter_sas_choices,
    })
}

#[allow(dead_code)]
fn struct_deviceclaiminprogress1info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceClaimInProgress1Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_greeter_sas = JsString::try_new(cx, rs_obj.greeter_sas).or_throw(cx)?;
    js_obj.set(cx, "greeterSas", js_greeter_sas)?;
    let js_greeter_sas_choices = {
        // JsArray::new allocates with `undefined` value, that's why we `set` value
        let js_array = JsArray::new(cx, rs_obj.greeter_sas_choices.len() as u32);
        for (i, elem) in rs_obj.greeter_sas_choices.into_iter().enumerate() {
            let js_elem = JsString::try_new(cx, elem).or_throw(cx)?;
            js_array.set(cx, i as u32, js_elem)?;
        }
        js_array
    };
    js_obj.set(cx, "greeterSasChoices", js_greeter_sas_choices)?;
    Ok(js_obj)
}

// UserClaimInProgress2Info

#[allow(dead_code)]
fn struct_userclaiminprogress2info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserClaimInProgress2Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let claimer_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "claimerSas")?;
        {
            match js_val.value(cx).parse() {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    Ok(libparsec::UserClaimInProgress2Info {
        handle,
        claimer_sas,
    })
}

#[allow(dead_code)]
fn struct_userclaiminprogress2info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserClaimInProgress2Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_claimer_sas = JsString::try_new(cx, rs_obj.claimer_sas).or_throw(cx)?;
    js_obj.set(cx, "claimerSas", js_claimer_sas)?;
    Ok(js_obj)
}

// DeviceClaimInProgress2Info

#[allow(dead_code)]
fn struct_deviceclaiminprogress2info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceClaimInProgress2Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let claimer_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "claimerSas")?;
        {
            match js_val.value(cx).parse() {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    Ok(libparsec::DeviceClaimInProgress2Info {
        handle,
        claimer_sas,
    })
}

#[allow(dead_code)]
fn struct_deviceclaiminprogress2info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceClaimInProgress2Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_claimer_sas = JsString::try_new(cx, rs_obj.claimer_sas).or_throw(cx)?;
    js_obj.set(cx, "claimerSas", js_claimer_sas)?;
    Ok(js_obj)
}

// UserClaimInProgress3Info

#[allow(dead_code)]
fn struct_userclaiminprogress3info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserClaimInProgress3Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    Ok(libparsec::UserClaimInProgress3Info { handle })
}

#[allow(dead_code)]
fn struct_userclaiminprogress3info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserClaimInProgress3Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// DeviceClaimInProgress3Info

#[allow(dead_code)]
fn struct_deviceclaiminprogress3info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceClaimInProgress3Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    Ok(libparsec::DeviceClaimInProgress3Info { handle })
}

#[allow(dead_code)]
fn struct_deviceclaiminprogress3info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceClaimInProgress3Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// UserClaimFinalizeInfo

#[allow(dead_code)]
fn struct_userclaimfinalizeinfo_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserClaimFinalizeInfo> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    Ok(libparsec::UserClaimFinalizeInfo { handle })
}

#[allow(dead_code)]
fn struct_userclaimfinalizeinfo_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserClaimFinalizeInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// DeviceClaimFinalizeInfo

#[allow(dead_code)]
fn struct_deviceclaimfinalizeinfo_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceClaimFinalizeInfo> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    Ok(libparsec::DeviceClaimFinalizeInfo { handle })
}

#[allow(dead_code)]
fn struct_deviceclaimfinalizeinfo_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceClaimFinalizeInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// UserGreetInitialInfo

#[allow(dead_code)]
fn struct_usergreetinitialinfo_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserGreetInitialInfo> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    Ok(libparsec::UserGreetInitialInfo { handle })
}

#[allow(dead_code)]
fn struct_usergreetinitialinfo_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserGreetInitialInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// DeviceGreetInitialInfo

#[allow(dead_code)]
fn struct_devicegreetinitialinfo_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceGreetInitialInfo> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    Ok(libparsec::DeviceGreetInitialInfo { handle })
}

#[allow(dead_code)]
fn struct_devicegreetinitialinfo_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceGreetInitialInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// UserGreetInProgress1Info

#[allow(dead_code)]
fn struct_usergreetinprogress1info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserGreetInProgress1Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let greeter_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "greeterSas")?;
        {
            match js_val.value(cx).parse() {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    Ok(libparsec::UserGreetInProgress1Info {
        handle,
        greeter_sas,
    })
}

#[allow(dead_code)]
fn struct_usergreetinprogress1info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserGreetInProgress1Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_greeter_sas = JsString::try_new(cx, rs_obj.greeter_sas).or_throw(cx)?;
    js_obj.set(cx, "greeterSas", js_greeter_sas)?;
    Ok(js_obj)
}

// DeviceGreetInProgress1Info

#[allow(dead_code)]
fn struct_devicegreetinprogress1info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceGreetInProgress1Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let greeter_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "greeterSas")?;
        {
            match js_val.value(cx).parse() {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    Ok(libparsec::DeviceGreetInProgress1Info {
        handle,
        greeter_sas,
    })
}

#[allow(dead_code)]
fn struct_devicegreetinprogress1info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceGreetInProgress1Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_greeter_sas = JsString::try_new(cx, rs_obj.greeter_sas).or_throw(cx)?;
    js_obj.set(cx, "greeterSas", js_greeter_sas)?;
    Ok(js_obj)
}

// UserGreetInProgress2Info

#[allow(dead_code)]
fn struct_usergreetinprogress2info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserGreetInProgress2Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let claimer_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "claimerSas")?;
        {
            match js_val.value(cx).parse() {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let claimer_sas_choices = {
        let js_val: Handle<JsArray> = obj.get(cx, "claimerSasChoices")?;
        {
            let size = js_val.len(cx);
            let mut v = Vec::with_capacity(size as usize);
            for i in 0..size {
                let js_item: Handle<JsString> = js_val.get(cx, i)?;
                v.push({
                    match js_item.value(cx).parse() {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                });
            }
            v
        }
    };
    Ok(libparsec::UserGreetInProgress2Info {
        handle,
        claimer_sas,
        claimer_sas_choices,
    })
}

#[allow(dead_code)]
fn struct_usergreetinprogress2info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserGreetInProgress2Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_claimer_sas = JsString::try_new(cx, rs_obj.claimer_sas).or_throw(cx)?;
    js_obj.set(cx, "claimerSas", js_claimer_sas)?;
    let js_claimer_sas_choices = {
        // JsArray::new allocates with `undefined` value, that's why we `set` value
        let js_array = JsArray::new(cx, rs_obj.claimer_sas_choices.len() as u32);
        for (i, elem) in rs_obj.claimer_sas_choices.into_iter().enumerate() {
            let js_elem = JsString::try_new(cx, elem).or_throw(cx)?;
            js_array.set(cx, i as u32, js_elem)?;
        }
        js_array
    };
    js_obj.set(cx, "claimerSasChoices", js_claimer_sas_choices)?;
    Ok(js_obj)
}

// DeviceGreetInProgress2Info

#[allow(dead_code)]
fn struct_devicegreetinprogress2info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceGreetInProgress2Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let claimer_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "claimerSas")?;
        {
            match js_val.value(cx).parse() {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let claimer_sas_choices = {
        let js_val: Handle<JsArray> = obj.get(cx, "claimerSasChoices")?;
        {
            let size = js_val.len(cx);
            let mut v = Vec::with_capacity(size as usize);
            for i in 0..size {
                let js_item: Handle<JsString> = js_val.get(cx, i)?;
                v.push({
                    match js_item.value(cx).parse() {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                });
            }
            v
        }
    };
    Ok(libparsec::DeviceGreetInProgress2Info {
        handle,
        claimer_sas,
        claimer_sas_choices,
    })
}

#[allow(dead_code)]
fn struct_devicegreetinprogress2info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceGreetInProgress2Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_claimer_sas = JsString::try_new(cx, rs_obj.claimer_sas).or_throw(cx)?;
    js_obj.set(cx, "claimerSas", js_claimer_sas)?;
    let js_claimer_sas_choices = {
        // JsArray::new allocates with `undefined` value, that's why we `set` value
        let js_array = JsArray::new(cx, rs_obj.claimer_sas_choices.len() as u32);
        for (i, elem) in rs_obj.claimer_sas_choices.into_iter().enumerate() {
            let js_elem = JsString::try_new(cx, elem).or_throw(cx)?;
            js_array.set(cx, i as u32, js_elem)?;
        }
        js_array
    };
    js_obj.set(cx, "claimerSasChoices", js_claimer_sas_choices)?;
    Ok(js_obj)
}

// UserGreetInProgress3Info

#[allow(dead_code)]
fn struct_usergreetinprogress3info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserGreetInProgress3Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    Ok(libparsec::UserGreetInProgress3Info { handle })
}

#[allow(dead_code)]
fn struct_usergreetinprogress3info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserGreetInProgress3Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// DeviceGreetInProgress3Info

#[allow(dead_code)]
fn struct_devicegreetinprogress3info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceGreetInProgress3Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    Ok(libparsec::DeviceGreetInProgress3Info { handle })
}

#[allow(dead_code)]
fn struct_devicegreetinprogress3info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceGreetInProgress3Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// UserGreetInProgress4Info

#[allow(dead_code)]
fn struct_usergreetinprogress4info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserGreetInProgress4Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let requested_human_handle = {
        let js_val: Handle<JsValue> = obj.get(cx, "requestedHumanHandle")?;
        {
            if js_val.is_a::<JsNull, _>(cx) {
                None
            } else {
                let js_val = js_val.downcast_or_throw::<JsObject, _>(cx)?;
                Some(struct_humanhandle_js_to_rs(cx, js_val)?)
            }
        }
    };
    let requested_device_label = {
        let js_val: Handle<JsValue> = obj.get(cx, "requestedDeviceLabel")?;
        {
            if js_val.is_a::<JsNull, _>(cx) {
                None
            } else {
                let js_val = js_val.downcast_or_throw::<JsString, _>(cx)?;
                Some({
                    match js_val.value(cx).parse() {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                })
            }
        }
    };
    Ok(libparsec::UserGreetInProgress4Info {
        handle,
        requested_human_handle,
        requested_device_label,
    })
}

#[allow(dead_code)]
fn struct_usergreetinprogress4info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserGreetInProgress4Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_requested_human_handle = match rs_obj.requested_human_handle {
        Some(elem) => struct_humanhandle_rs_to_js(cx, elem)?.as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "requestedHumanHandle", js_requested_human_handle)?;
    let js_requested_device_label = match rs_obj.requested_device_label {
        Some(elem) => JsString::try_new(cx, elem).or_throw(cx)?.as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "requestedDeviceLabel", js_requested_device_label)?;
    Ok(js_obj)
}

// DeviceGreetInProgress4Info

#[allow(dead_code)]
fn struct_devicegreetinprogress4info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceGreetInProgress4Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let requested_device_label = {
        let js_val: Handle<JsValue> = obj.get(cx, "requestedDeviceLabel")?;
        {
            if js_val.is_a::<JsNull, _>(cx) {
                None
            } else {
                let js_val = js_val.downcast_or_throw::<JsString, _>(cx)?;
                Some({
                    match js_val.value(cx).parse() {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                })
            }
        }
    };
    Ok(libparsec::DeviceGreetInProgress4Info {
        handle,
        requested_device_label,
    })
}

#[allow(dead_code)]
fn struct_devicegreetinprogress4info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceGreetInProgress4Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_requested_device_label = match rs_obj.requested_device_label {
        Some(elem) => JsString::try_new(cx, elem).or_throw(cx)?.as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "requestedDeviceLabel", js_requested_device_label)?;
    Ok(js_obj)
}

// CancelError

#[allow(dead_code)]
fn variant_cancelerror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::CancelError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::CancelError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::CancelError::NotBinded { .. } => {
            let js_tag = JsString::try_new(cx, "NotBinded").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// RealmRole

#[allow(dead_code)]
fn variant_realmrole_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::RealmRole> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "Contributor" => Ok(libparsec::RealmRole::Contributor {}),
        "Manager" => Ok(libparsec::RealmRole::Manager {}),
        "Owner" => Ok(libparsec::RealmRole::Owner {}),
        "Reader" => Ok(libparsec::RealmRole::Reader {}),
        _ => cx.throw_type_error("Object is not a RealmRole"),
    }
}

#[allow(dead_code)]
fn variant_realmrole_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::RealmRole,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::RealmRole::Contributor { .. } => {
            let js_tag = JsString::try_new(cx, "Contributor").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::RealmRole::Manager { .. } => {
            let js_tag = JsString::try_new(cx, "Manager").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::RealmRole::Owner { .. } => {
            let js_tag = JsString::try_new(cx, "Owner").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::RealmRole::Reader { .. } => {
            let js_tag = JsString::try_new(cx, "Reader").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// DeviceAccessStrategy

#[allow(dead_code)]
fn variant_deviceaccessstrategy_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceAccessStrategy> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "Password" => {
            let password = {
                let js_val: Handle<JsString> = obj.get(cx, "password")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<_, String> { Ok(s.into()) };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let key_file = {
                let js_val: Handle<JsString> = obj.get(cx, "key_file")?;
                {
                    let custom_from_rs_string =
                        |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            Ok(libparsec::DeviceAccessStrategy::Password { password, key_file })
        }
        "Smartcard" => {
            let key_file = {
                let js_val: Handle<JsString> = obj.get(cx, "key_file")?;
                {
                    let custom_from_rs_string =
                        |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            Ok(libparsec::DeviceAccessStrategy::Smartcard { key_file })
        }
        _ => cx.throw_type_error("Object is not a DeviceAccessStrategy"),
    }
}

#[allow(dead_code)]
fn variant_deviceaccessstrategy_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceAccessStrategy,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::DeviceAccessStrategy::Password {
            password, key_file, ..
        } => {
            let js_tag = JsString::try_new(cx, "Password").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_password = JsString::try_new(cx, password).or_throw(cx)?;
            js_obj.set(cx, "password", js_password)?;
            let js_key_file = JsString::try_new(cx, {
                let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                };
                match custom_to_rs_string(key_file) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "key_file", js_key_file)?;
        }
        libparsec::DeviceAccessStrategy::Smartcard { key_file, .. } => {
            let js_tag = JsString::try_new(cx, "Smartcard").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_key_file = JsString::try_new(cx, {
                let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                };
                match custom_to_rs_string(key_file) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "key_file", js_key_file)?;
        }
    }
    Ok(js_obj)
}

// ClientStartError

#[allow(dead_code)]
fn variant_clientstarterror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientStartError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientStartError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartError::LoadDeviceDecryptionFailed { .. } => {
            let js_tag = JsString::try_new(cx, "LoadDeviceDecryptionFailed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartError::LoadDeviceInvalidData { .. } => {
            let js_tag = JsString::try_new(cx, "LoadDeviceInvalidData").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartError::LoadDeviceInvalidPath { .. } => {
            let js_tag = JsString::try_new(cx, "LoadDeviceInvalidPath").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientStopError

#[allow(dead_code)]
fn variant_clientstoperror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientStopError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientStopError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientListWorkspacesError

#[allow(dead_code)]
fn variant_clientlistworkspaceserror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientListWorkspacesError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientListWorkspacesError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientWorkspaceCreateError

#[allow(dead_code)]
fn variant_clientworkspacecreateerror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientWorkspaceCreateError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientWorkspaceCreateError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientWorkspaceRenameError

#[allow(dead_code)]
fn variant_clientworkspacerenameerror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientWorkspaceRenameError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientWorkspaceRenameError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientWorkspaceRenameError::UnknownWorkspace { .. } => {
            let js_tag = JsString::try_new(cx, "UnknownWorkspace").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientWorkspaceShareError

#[allow(dead_code)]
fn variant_clientworkspaceshareerror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientWorkspaceShareError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientWorkspaceShareError::BadTimestamp {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "BadTimestamp").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_server_timestamp = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339().into())
                    };
                match custom_to_rs_string(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "server_timestamp", js_server_timestamp)?;
            let js_client_timestamp = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339().into())
                    };
                match custom_to_rs_string(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "client_timestamp", js_client_timestamp)?;
            let js_ballpark_client_early_offset = JsNumber::new(cx, ballpark_client_early_offset);
            js_obj.set(
                cx,
                "ballpark_client_early_offset",
                js_ballpark_client_early_offset,
            )?;
            let js_ballpark_client_late_offset = JsNumber::new(cx, ballpark_client_late_offset);
            js_obj.set(
                cx,
                "ballpark_client_late_offset",
                js_ballpark_client_late_offset,
            )?;
        }
        libparsec::ClientWorkspaceShareError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientWorkspaceShareError::NotAllowed { .. } => {
            let js_tag = JsString::try_new(cx, "NotAllowed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientWorkspaceShareError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "Offline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientWorkspaceShareError::OutsiderCannotBeManagerOrOwner { .. } => {
            let js_tag = JsString::try_new(cx, "OutsiderCannotBeManagerOrOwner").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientWorkspaceShareError::RevokedRecipient { .. } => {
            let js_tag = JsString::try_new(cx, "RevokedRecipient").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientWorkspaceShareError::ShareToSelf { .. } => {
            let js_tag = JsString::try_new(cx, "ShareToSelf").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientWorkspaceShareError::UnknownRecipient { .. } => {
            let js_tag = JsString::try_new(cx, "UnknownRecipient").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientWorkspaceShareError::UnknownRecipientOrWorkspace { .. } => {
            let js_tag = JsString::try_new(cx, "UnknownRecipientOrWorkspace").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientWorkspaceShareError::UnknownWorkspace { .. } => {
            let js_tag = JsString::try_new(cx, "UnknownWorkspace").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientWorkspaceShareError::WorkspaceInMaintenance { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceInMaintenance").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// UserProfile

#[allow(dead_code)]
fn variant_userprofile_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserProfile> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "Admin" => Ok(libparsec::UserProfile::Admin {}),
        "Outsider" => Ok(libparsec::UserProfile::Outsider {}),
        "Standard" => Ok(libparsec::UserProfile::Standard {}),
        _ => cx.throw_type_error("Object is not a UserProfile"),
    }
}

#[allow(dead_code)]
fn variant_userprofile_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserProfile,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::UserProfile::Admin { .. } => {
            let js_tag = JsString::try_new(cx, "Admin").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::UserProfile::Outsider { .. } => {
            let js_tag = JsString::try_new(cx, "Outsider").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::UserProfile::Standard { .. } => {
            let js_tag = JsString::try_new(cx, "Standard").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceStorageCacheSize

#[allow(dead_code)]
fn variant_workspacestoragecachesize_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::WorkspaceStorageCacheSize> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "Custom" => {
            let size = {
                let js_val: Handle<JsNumber> = obj.get(cx, "size")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    v as u32
                }
            };
            Ok(libparsec::WorkspaceStorageCacheSize::Custom { size })
        }
        "Default" => Ok(libparsec::WorkspaceStorageCacheSize::Default {}),
        _ => cx.throw_type_error("Object is not a WorkspaceStorageCacheSize"),
    }
}

#[allow(dead_code)]
fn variant_workspacestoragecachesize_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceStorageCacheSize,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::WorkspaceStorageCacheSize::Custom { size, .. } => {
            let js_tag = JsString::try_new(cx, "Custom").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_size = JsNumber::new(cx, size as f64);
            js_obj.set(cx, "size", js_size)?;
        }
        libparsec::WorkspaceStorageCacheSize::Default { .. } => {
            let js_tag = JsString::try_new(cx, "Default").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientEvent

#[allow(dead_code)]
fn variant_clientevent_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ClientEvent> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "Ping" => {
            let ping = {
                let js_val: Handle<JsString> = obj.get(cx, "ping")?;
                js_val.value(cx)
            };
            Ok(libparsec::ClientEvent::Ping { ping })
        }
        _ => cx.throw_type_error("Object is not a ClientEvent"),
    }
}

#[allow(dead_code)]
fn variant_clientevent_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientEvent,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::ClientEvent::Ping { ping, .. } => {
            let js_tag = JsString::try_new(cx, "Ping").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_ping = JsString::try_new(cx, ping).or_throw(cx)?;
            js_obj.set(cx, "ping", js_ping)?;
        }
    }
    Ok(js_obj)
}

// ClaimerGreeterAbortOperationError

#[allow(dead_code)]
fn variant_claimergreeterabortoperationerror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClaimerGreeterAbortOperationError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClaimerGreeterAbortOperationError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
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
        libparsec::DeviceFileType::Password { .. } => {
            let js_tag = JsString::try_new(cx, "Password").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::DeviceFileType::Recovery { .. } => {
            let js_tag = JsString::try_new(cx, "Recovery").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::DeviceFileType::Smartcard { .. } => {
            let js_tag = JsString::try_new(cx, "Smartcard").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// DeviceSaveStrategy

#[allow(dead_code)]
fn variant_devicesavestrategy_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceSaveStrategy> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "Password" => {
            let password = {
                let js_val: Handle<JsString> = obj.get(cx, "password")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<_, String> { Ok(s.into()) };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            Ok(libparsec::DeviceSaveStrategy::Password { password })
        }
        "Smartcard" => Ok(libparsec::DeviceSaveStrategy::Smartcard {}),
        _ => cx.throw_type_error("Object is not a DeviceSaveStrategy"),
    }
}

#[allow(dead_code)]
fn variant_devicesavestrategy_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceSaveStrategy,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::DeviceSaveStrategy::Password { password, .. } => {
            let js_tag = JsString::try_new(cx, "Password").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_password = JsString::try_new(cx, password).or_throw(cx)?;
            js_obj.set(cx, "password", js_password)?;
        }
        libparsec::DeviceSaveStrategy::Smartcard { .. } => {
            let js_tag = JsString::try_new(cx, "Smartcard").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// BootstrapOrganizationError

#[allow(dead_code)]
fn variant_bootstraporganizationerror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::BootstrapOrganizationError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::BootstrapOrganizationError::AlreadyUsedToken { .. } => {
            let js_tag = JsString::try_new(cx, "AlreadyUsedToken").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::BootstrapOrganizationError::BadTimestamp {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "BadTimestamp").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_server_timestamp = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339().into())
                    };
                match custom_to_rs_string(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "server_timestamp", js_server_timestamp)?;
            let js_client_timestamp = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339().into())
                    };
                match custom_to_rs_string(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "client_timestamp", js_client_timestamp)?;
            let js_ballpark_client_early_offset = JsNumber::new(cx, ballpark_client_early_offset);
            js_obj.set(
                cx,
                "ballpark_client_early_offset",
                js_ballpark_client_early_offset,
            )?;
            let js_ballpark_client_late_offset = JsNumber::new(cx, ballpark_client_late_offset);
            js_obj.set(
                cx,
                "ballpark_client_late_offset",
                js_ballpark_client_late_offset,
            )?;
        }
        libparsec::BootstrapOrganizationError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::BootstrapOrganizationError::InvalidToken { .. } => {
            let js_tag = JsString::try_new(cx, "InvalidToken").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::BootstrapOrganizationError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "Offline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::BootstrapOrganizationError::SaveDeviceError { .. } => {
            let js_tag = JsString::try_new(cx, "SaveDeviceError").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClaimerRetrieveInfoError

#[allow(dead_code)]
fn variant_claimerretrieveinfoerror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClaimerRetrieveInfoError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClaimerRetrieveInfoError::AlreadyUsed { .. } => {
            let js_tag = JsString::try_new(cx, "AlreadyUsed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimerRetrieveInfoError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimerRetrieveInfoError::NotFound { .. } => {
            let js_tag = JsString::try_new(cx, "NotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimerRetrieveInfoError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "Offline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClaimInProgressError

#[allow(dead_code)]
fn variant_claiminprogresserror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClaimInProgressError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClaimInProgressError::ActiveUsersLimitReached { .. } => {
            let js_tag = JsString::try_new(cx, "ActiveUsersLimitReached").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::AlreadyUsed { .. } => {
            let js_tag = JsString::try_new(cx, "AlreadyUsed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::Cancelled { .. } => {
            let js_tag = JsString::try_new(cx, "Cancelled").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::CorruptedConfirmation { .. } => {
            let js_tag = JsString::try_new(cx, "CorruptedConfirmation").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::NotFound { .. } => {
            let js_tag = JsString::try_new(cx, "NotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "Offline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::PeerReset { .. } => {
            let js_tag = JsString::try_new(cx, "PeerReset").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// UserOrDeviceClaimInitialInfo

#[allow(dead_code)]
fn variant_userordeviceclaiminitialinfo_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserOrDeviceClaimInitialInfo> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "Device" => {
            let handle = {
                let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    v as u32
                }
            };
            let greeter_user_id = {
                let js_val: Handle<JsString> = obj.get(cx, "greeter_user_id")?;
                {
                    match js_val.value(cx).parse() {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let greeter_human_handle = {
                let js_val: Handle<JsValue> = obj.get(cx, "greeter_human_handle")?;
                {
                    if js_val.is_a::<JsNull, _>(cx) {
                        None
                    } else {
                        let js_val = js_val.downcast_or_throw::<JsObject, _>(cx)?;
                        Some(struct_humanhandle_js_to_rs(cx, js_val)?)
                    }
                }
            };
            Ok(libparsec::UserOrDeviceClaimInitialInfo::Device {
                handle,
                greeter_user_id,
                greeter_human_handle,
            })
        }
        "User" => {
            let handle = {
                let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    v as u32
                }
            };
            let claimer_email = {
                let js_val: Handle<JsString> = obj.get(cx, "claimer_email")?;
                js_val.value(cx)
            };
            let greeter_user_id = {
                let js_val: Handle<JsString> = obj.get(cx, "greeter_user_id")?;
                {
                    match js_val.value(cx).parse() {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let greeter_human_handle = {
                let js_val: Handle<JsValue> = obj.get(cx, "greeter_human_handle")?;
                {
                    if js_val.is_a::<JsNull, _>(cx) {
                        None
                    } else {
                        let js_val = js_val.downcast_or_throw::<JsObject, _>(cx)?;
                        Some(struct_humanhandle_js_to_rs(cx, js_val)?)
                    }
                }
            };
            Ok(libparsec::UserOrDeviceClaimInitialInfo::User {
                handle,
                claimer_email,
                greeter_user_id,
                greeter_human_handle,
            })
        }
        _ => cx.throw_type_error("Object is not a UserOrDeviceClaimInitialInfo"),
    }
}

#[allow(dead_code)]
fn variant_userordeviceclaiminitialinfo_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserOrDeviceClaimInitialInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::UserOrDeviceClaimInitialInfo::Device {
            handle,
            greeter_user_id,
            greeter_human_handle,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "Device").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_handle = JsNumber::new(cx, handle as f64);
            js_obj.set(cx, "handle", js_handle)?;
            let js_greeter_user_id = JsString::try_new(cx, greeter_user_id).or_throw(cx)?;
            js_obj.set(cx, "greeter_user_id", js_greeter_user_id)?;
            let js_greeter_human_handle = match greeter_human_handle {
                Some(elem) => struct_humanhandle_rs_to_js(cx, elem)?.as_value(cx),
                None => JsNull::new(cx).as_value(cx),
            };
            js_obj.set(cx, "greeter_human_handle", js_greeter_human_handle)?;
        }
        libparsec::UserOrDeviceClaimInitialInfo::User {
            handle,
            claimer_email,
            greeter_user_id,
            greeter_human_handle,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "User").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_handle = JsNumber::new(cx, handle as f64);
            js_obj.set(cx, "handle", js_handle)?;
            let js_claimer_email = JsString::try_new(cx, claimer_email).or_throw(cx)?;
            js_obj.set(cx, "claimer_email", js_claimer_email)?;
            let js_greeter_user_id = JsString::try_new(cx, greeter_user_id).or_throw(cx)?;
            js_obj.set(cx, "greeter_user_id", js_greeter_user_id)?;
            let js_greeter_human_handle = match greeter_human_handle {
                Some(elem) => struct_humanhandle_rs_to_js(cx, elem)?.as_value(cx),
                None => JsNull::new(cx).as_value(cx),
            };
            js_obj.set(cx, "greeter_human_handle", js_greeter_human_handle)?;
        }
    }
    Ok(js_obj)
}

// InvitationStatus

#[allow(dead_code)]
fn variant_invitationstatus_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::InvitationStatus> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "Deleted" => Ok(libparsec::InvitationStatus::Deleted {}),
        "Idle" => Ok(libparsec::InvitationStatus::Idle {}),
        "Ready" => Ok(libparsec::InvitationStatus::Ready {}),
        _ => cx.throw_type_error("Object is not a InvitationStatus"),
    }
}

#[allow(dead_code)]
fn variant_invitationstatus_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::InvitationStatus,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::InvitationStatus::Deleted { .. } => {
            let js_tag = JsString::try_new(cx, "Deleted").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::InvitationStatus::Idle { .. } => {
            let js_tag = JsString::try_new(cx, "Idle").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::InvitationStatus::Ready { .. } => {
            let js_tag = JsString::try_new(cx, "Ready").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// InvitationEmailSentStatus

#[allow(dead_code)]
fn variant_invitationemailsentstatus_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::InvitationEmailSentStatus> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "BadRecipient" => Ok(libparsec::InvitationEmailSentStatus::BadRecipient {}),
        "NotAvailable" => Ok(libparsec::InvitationEmailSentStatus::NotAvailable {}),
        "Success" => Ok(libparsec::InvitationEmailSentStatus::Success {}),
        _ => cx.throw_type_error("Object is not a InvitationEmailSentStatus"),
    }
}

#[allow(dead_code)]
fn variant_invitationemailsentstatus_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::InvitationEmailSentStatus,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::InvitationEmailSentStatus::BadRecipient { .. } => {
            let js_tag = JsString::try_new(cx, "BadRecipient").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::InvitationEmailSentStatus::NotAvailable { .. } => {
            let js_tag = JsString::try_new(cx, "NotAvailable").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::InvitationEmailSentStatus::Success { .. } => {
            let js_tag = JsString::try_new(cx, "Success").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// NewUserInvitationError

#[allow(dead_code)]
fn variant_newuserinvitationerror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::NewUserInvitationError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::NewUserInvitationError::AlreadyMember { .. } => {
            let js_tag = JsString::try_new(cx, "AlreadyMember").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::NewUserInvitationError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::NewUserInvitationError::NotAllowed { .. } => {
            let js_tag = JsString::try_new(cx, "NotAllowed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::NewUserInvitationError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "Offline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// NewDeviceInvitationError

#[allow(dead_code)]
fn variant_newdeviceinvitationerror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::NewDeviceInvitationError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::NewDeviceInvitationError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::NewDeviceInvitationError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "Offline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::NewDeviceInvitationError::SendEmailToUserWithoutEmail { .. } => {
            let js_tag = JsString::try_new(cx, "SendEmailToUserWithoutEmail").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// DeleteInvitationError

#[allow(dead_code)]
fn variant_deleteinvitationerror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeleteInvitationError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::DeleteInvitationError::AlreadyDeleted { .. } => {
            let js_tag = JsString::try_new(cx, "AlreadyDeleted").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::DeleteInvitationError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::DeleteInvitationError::NotFound { .. } => {
            let js_tag = JsString::try_new(cx, "NotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::DeleteInvitationError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "Offline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// InviteListItem

#[allow(dead_code)]
fn variant_invitelistitem_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::InviteListItem> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "Device" => {
            let token = {
                let js_val: Handle<JsTypedArray<u8>> = obj.get(cx, "token")?;
                {
                    let custom_from_rs_bytes = |x: &[u8]| -> Result<_, _> {
                        libparsec::InvitationToken::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_bytes(js_val.as_slice(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(format!("{}", err)),
                    }
                }
            };
            let created_on = {
                let js_val: Handle<JsString> = obj.get(cx, "created_on")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::DateTime::from_rfc3339(&s).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let status = {
                let js_val: Handle<JsObject> = obj.get(cx, "status")?;
                variant_invitationstatus_js_to_rs(cx, js_val)?
            };
            Ok(libparsec::InviteListItem::Device {
                token,
                created_on,
                status,
            })
        }
        "User" => {
            let token = {
                let js_val: Handle<JsTypedArray<u8>> = obj.get(cx, "token")?;
                {
                    let custom_from_rs_bytes = |x: &[u8]| -> Result<_, _> {
                        libparsec::InvitationToken::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_bytes(js_val.as_slice(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(format!("{}", err)),
                    }
                }
            };
            let created_on = {
                let js_val: Handle<JsString> = obj.get(cx, "created_on")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::DateTime::from_rfc3339(&s).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let claimer_email = {
                let js_val: Handle<JsString> = obj.get(cx, "claimer_email")?;
                js_val.value(cx)
            };
            let status = {
                let js_val: Handle<JsObject> = obj.get(cx, "status")?;
                variant_invitationstatus_js_to_rs(cx, js_val)?
            };
            Ok(libparsec::InviteListItem::User {
                token,
                created_on,
                claimer_email,
                status,
            })
        }
        _ => cx.throw_type_error("Object is not a InviteListItem"),
    }
}

#[allow(dead_code)]
fn variant_invitelistitem_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::InviteListItem,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::InviteListItem::Device {
            token,
            created_on,
            status,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "Device").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_token = {
                let rs_buff = {
                    let custom_to_rs_bytes =
                        |x: libparsec::InvitationToken| -> Result<_, &'static str> {
                            Ok(x.as_bytes().to_owned())
                        };
                    match custom_to_rs_bytes(token) {
                        Ok(ok) => ok,
                        Err(err) => return cx.throw_type_error(err),
                    }
                };
                let mut js_buff = JsArrayBuffer::new(cx, rs_buff.len())?;
                let js_buff_slice = js_buff.as_mut_slice(cx);
                for (i, c) in rs_buff.iter().enumerate() {
                    js_buff_slice[i] = *c;
                }
                js_buff
            };
            js_obj.set(cx, "token", js_token)?;
            let js_created_on = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339().into())
                    };
                match custom_to_rs_string(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "created_on", js_created_on)?;
            let js_status = variant_invitationstatus_rs_to_js(cx, status)?;
            js_obj.set(cx, "status", js_status)?;
        }
        libparsec::InviteListItem::User {
            token,
            created_on,
            claimer_email,
            status,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "User").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_token = {
                let rs_buff = {
                    let custom_to_rs_bytes =
                        |x: libparsec::InvitationToken| -> Result<_, &'static str> {
                            Ok(x.as_bytes().to_owned())
                        };
                    match custom_to_rs_bytes(token) {
                        Ok(ok) => ok,
                        Err(err) => return cx.throw_type_error(err),
                    }
                };
                let mut js_buff = JsArrayBuffer::new(cx, rs_buff.len())?;
                let js_buff_slice = js_buff.as_mut_slice(cx);
                for (i, c) in rs_buff.iter().enumerate() {
                    js_buff_slice[i] = *c;
                }
                js_buff
            };
            js_obj.set(cx, "token", js_token)?;
            let js_created_on = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339().into())
                    };
                match custom_to_rs_string(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "created_on", js_created_on)?;
            let js_claimer_email = JsString::try_new(cx, claimer_email).or_throw(cx)?;
            js_obj.set(cx, "claimer_email", js_claimer_email)?;
            let js_status = variant_invitationstatus_rs_to_js(cx, status)?;
            js_obj.set(cx, "status", js_status)?;
        }
    }
    Ok(js_obj)
}

// ListInvitationsError

#[allow(dead_code)]
fn variant_listinvitationserror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ListInvitationsError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ListInvitationsError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ListInvitationsError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "Offline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientStartInvitationGreetError

#[allow(dead_code)]
fn variant_clientstartinvitationgreeterror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientStartInvitationGreetError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientStartInvitationGreetError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// GreetInProgressError

#[allow(dead_code)]
fn variant_greetinprogresserror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::GreetInProgressError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::GreetInProgressError::ActiveUsersLimitReached { .. } => {
            let js_tag = JsString::try_new(cx, "ActiveUsersLimitReached").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::AlreadyUsed { .. } => {
            let js_tag = JsString::try_new(cx, "AlreadyUsed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::BadTimestamp {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "BadTimestamp").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_server_timestamp = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339().into())
                    };
                match custom_to_rs_string(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "server_timestamp", js_server_timestamp)?;
            let js_client_timestamp = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339().into())
                    };
                match custom_to_rs_string(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "client_timestamp", js_client_timestamp)?;
            let js_ballpark_client_early_offset = JsNumber::new(cx, ballpark_client_early_offset);
            js_obj.set(
                cx,
                "ballpark_client_early_offset",
                js_ballpark_client_early_offset,
            )?;
            let js_ballpark_client_late_offset = JsNumber::new(cx, ballpark_client_late_offset);
            js_obj.set(
                cx,
                "ballpark_client_late_offset",
                js_ballpark_client_late_offset,
            )?;
        }
        libparsec::GreetInProgressError::Cancelled { .. } => {
            let js_tag = JsString::try_new(cx, "Cancelled").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::CorruptedInviteUserData { .. } => {
            let js_tag = JsString::try_new(cx, "CorruptedInviteUserData").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::DeviceAlreadyExists { .. } => {
            let js_tag = JsString::try_new(cx, "DeviceAlreadyExists").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::NonceMismatch { .. } => {
            let js_tag = JsString::try_new(cx, "NonceMismatch").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::NotFound { .. } => {
            let js_tag = JsString::try_new(cx, "NotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "Offline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::PeerReset { .. } => {
            let js_tag = JsString::try_new(cx, "PeerReset").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::UserAlreadyExists { .. } => {
            let js_tag = JsString::try_new(cx, "UserAlreadyExists").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::UserCreateNotAllowed { .. } => {
            let js_tag = JsString::try_new(cx, "UserCreateNotAllowed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// cancel
fn cancel(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let ret = libparsec::cancel(canceller);
    let js_ret = match ret {
        Ok(ok) => {
            let js_obj = JsObject::new(&mut cx);
            let js_tag = JsBoolean::new(&mut cx, true);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_value = {
                let _ = ok;
                JsNull::new(&mut cx)
            };
            js_obj.set(&mut cx, "value", js_value)?;
            js_obj
        }
        Err(err) => {
            let js_obj = cx.empty_object();
            let js_tag = JsBoolean::new(&mut cx, false);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_err = variant_cancelerror_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// new_canceller
fn new_canceller(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let ret = libparsec::new_canceller();
    let js_ret = JsNumber::new(&mut cx, ret as f64);
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// client_start
fn client_start(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let config = {
        let js_val = cx.argument::<JsObject>(0)?;
        struct_clientconfig_js_to_rs(&mut cx, js_val)?
    };
    let on_event_callback = {
        let js_val = cx.argument::<JsFunction>(1)?;
        // The Javascript function object is going to be shared between the closure
        // called by rust (that can be called multiple times) and the single-use
        // closure sent to the js runtime.
        // So we must use an Arc to ensure the resource is shared correctly, but
        // that's not all of it !
        // When the resource is no longer use, we must consume the reference we
        // had on the javascript function in a neon context so that it can itself
        // notify the js runtime's garbage collector.
        struct Callback {
            js_fn: Option<neon::handle::Root<JsFunction>>,
            channel: neon::event::Channel,
        }
        impl Drop for Callback {
            fn drop(&mut self) {
                if let Some(js_fn) = self.js_fn.take() {
                    // Return the js object to the js runtime to avoid memory leak
                    self.channel.send(move |mut cx| {
                        js_fn.to_inner(&mut cx);
                        Ok(())
                    });
                }
            }
        }
        let callback = std::sync::Arc::new(Callback {
            js_fn: Some(js_val.root(&mut cx)),
            channel: cx.channel(),
        });
        std::sync::Arc::new(move |event: libparsec::ClientEvent| {
            let callback2 = callback.clone();
            callback.channel.send(move |mut cx| {
                // TODO: log an error instead of panic ? (it is a bit harsh to crash
                // the current task if an unrelated event handler has a bug...)
                let js_event = variant_clientevent_rs_to_js(&mut cx, event)?;
                if let Some(ref js_fn) = callback2.js_fn {
                    js_fn
                        .to_inner(&mut cx)
                        .call_with(&cx)
                        .arg(js_event)
                        .apply::<JsNull, _>(&mut cx)?;
                }
                Ok(())
            });
        }) as std::sync::Arc<dyn Fn(libparsec::ClientEvent) + Send + Sync>
    };
    let access = {
        let js_val = cx.argument::<JsObject>(2)?;
        variant_deviceaccessstrategy_js_to_rs(&mut cx, js_val)?
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_start(config, on_event_callback, access).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = JsNumber::new(&mut cx, ok as f64);
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_clientstarterror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_stop
fn client_stop(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_stop(client).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let _ = ok;
                            JsNull::new(&mut cx)
                        };
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_clientstoperror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_list_workspaces
fn client_list_workspaces(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_list_workspaces(client).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            // JsArray::new allocates with `undefined` value, that's why we `set` value
                            let js_array = JsArray::new(&mut cx, ok.len() as u32);
                            for (i, elem) in ok.into_iter().enumerate() {
                                let js_elem = {
                                    let (x1, x2) = elem;
                                    let js_array = JsArray::new(&mut cx, 2);
                                    let js_value = {
                                        let rs_buff = {
                                            let custom_to_rs_bytes =
                                                |x: libparsec::EntryID| -> Result<_, &'static str> {
                                                    Ok(x.as_bytes().to_owned())
                                                };
                                            match custom_to_rs_bytes(x1) {
                                                Ok(ok) => ok,
                                                Err(err) => return cx.throw_type_error(err),
                                            }
                                        };
                                        let mut js_buff =
                                            JsArrayBuffer::new(&mut cx, rs_buff.len())?;
                                        let js_buff_slice = js_buff.as_mut_slice(&mut cx);
                                        for (i, c) in rs_buff.iter().enumerate() {
                                            js_buff_slice[i] = *c;
                                        }
                                        js_buff
                                    };
                                    js_array.set(&mut cx, 1, js_value)?;
                                    let js_value =
                                        JsString::try_new(&mut cx, x2).or_throw(&mut cx)?;
                                    js_array.set(&mut cx, 2, js_value)?;
                                    js_array
                                };
                                js_array.set(&mut cx, i as u32, js_elem)?;
                            }
                            js_array
                        };
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_clientlistworkspaceserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_workspace_create
fn client_workspace_create(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let name = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, _> {
                s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_workspace_create(client, name).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let rs_buff = {
                                let custom_to_rs_bytes =
                                    |x: libparsec::EntryID| -> Result<_, &'static str> {
                                        Ok(x.as_bytes().to_owned())
                                    };
                                match custom_to_rs_bytes(ok) {
                                    Ok(ok) => ok,
                                    Err(err) => return cx.throw_type_error(err),
                                }
                            };
                            let mut js_buff = JsArrayBuffer::new(&mut cx, rs_buff.len())?;
                            let js_buff_slice = js_buff.as_mut_slice(&mut cx);
                            for (i, c) in rs_buff.iter().enumerate() {
                                js_buff_slice[i] = *c;
                            }
                            js_buff
                        };
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_clientworkspacecreateerror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_workspace_rename
fn client_workspace_rename(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let workspace_id = {
        let js_val = cx.argument::<JsTypedArray<u8>>(1)?;
        {
            let custom_from_rs_bytes = |x: &[u8]| -> Result<_, _> {
                libparsec::EntryID::try_from(x).map_err(|e| e.to_string())
            };
            match custom_from_rs_bytes(js_val.as_slice(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(format!("{}", err)),
            }
        }
    };
    let new_name = {
        let js_val = cx.argument::<JsString>(2)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, _> {
                s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_workspace_rename(client, workspace_id, new_name).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let _ = ok;
                            JsNull::new(&mut cx)
                        };
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_clientworkspacerenameerror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_workspace_share
fn client_workspace_share(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let workspace_id = {
        let js_val = cx.argument::<JsTypedArray<u8>>(1)?;
        {
            let custom_from_rs_bytes = |x: &[u8]| -> Result<_, _> {
                libparsec::EntryID::try_from(x).map_err(|e| e.to_string())
            };
            match custom_from_rs_bytes(js_val.as_slice(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(format!("{}", err)),
            }
        }
    };
    let recipient = {
        let js_val = cx.argument::<JsString>(2)?;
        {
            match js_val.value(&mut cx).parse() {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let role = match cx.argument_opt(3) {
        Some(v) => match v.downcast::<JsObject, _>(&mut cx) {
            Ok(js_val) => Some(variant_realmrole_js_to_rs(&mut cx, js_val)?),
            Err(_) => None,
        },
        None => None,
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::client_workspace_share(client, workspace_id, recipient, role).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let _ = ok;
                            JsNull::new(&mut cx)
                        };
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_clientworkspaceshareerror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// claimer_greeter_abort_operation
fn claimer_greeter_abort_operation(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let ret = libparsec::claimer_greeter_abort_operation(handle);
    let js_ret = match ret {
        Ok(ok) => {
            let js_obj = JsObject::new(&mut cx);
            let js_tag = JsBoolean::new(&mut cx, true);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_value = {
                let _ = ok;
                JsNull::new(&mut cx)
            };
            js_obj.set(&mut cx, "value", js_value)?;
            js_obj
        }
        Err(err) => {
            let js_obj = cx.empty_object();
            let js_tag = JsBoolean::new(&mut cx, false);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_err = variant_claimergreeterabortoperationerror_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// list_available_devices
fn list_available_devices(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let path = {
        let js_val = cx.argument::<JsString>(0)?;
        {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::list_available_devices(&path).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = {
                    // JsArray::new allocates with `undefined` value, that's why we `set` value
                    let js_array = JsArray::new(&mut cx, ret.len() as u32);
                    for (i, elem) in ret.into_iter().enumerate() {
                        let js_elem = struct_availabledevice_rs_to_js(&mut cx, elem)?;
                        js_array.set(&mut cx, i as u32, js_elem)?;
                    }
                    js_array
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// bootstrap_organization
fn bootstrap_organization(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let config = {
        let js_val = cx.argument::<JsObject>(0)?;
        struct_clientconfig_js_to_rs(&mut cx, js_val)?
    };
    let on_event_callback = {
        let js_val = cx.argument::<JsFunction>(1)?;
        // The Javascript function object is going to be shared between the closure
        // called by rust (that can be called multiple times) and the single-use
        // closure sent to the js runtime.
        // So we must use an Arc to ensure the resource is shared correctly, but
        // that's not all of it !
        // When the resource is no longer use, we must consume the reference we
        // had on the javascript function in a neon context so that it can itself
        // notify the js runtime's garbage collector.
        struct Callback {
            js_fn: Option<neon::handle::Root<JsFunction>>,
            channel: neon::event::Channel,
        }
        impl Drop for Callback {
            fn drop(&mut self) {
                if let Some(js_fn) = self.js_fn.take() {
                    // Return the js object to the js runtime to avoid memory leak
                    self.channel.send(move |mut cx| {
                        js_fn.to_inner(&mut cx);
                        Ok(())
                    });
                }
            }
        }
        let callback = std::sync::Arc::new(Callback {
            js_fn: Some(js_val.root(&mut cx)),
            channel: cx.channel(),
        });
        std::sync::Arc::new(move |event: libparsec::ClientEvent| {
            let callback2 = callback.clone();
            callback.channel.send(move |mut cx| {
                // TODO: log an error instead of panic ? (it is a bit harsh to crash
                // the current task if an unrelated event handler has a bug...)
                let js_event = variant_clientevent_rs_to_js(&mut cx, event)?;
                if let Some(ref js_fn) = callback2.js_fn {
                    js_fn
                        .to_inner(&mut cx)
                        .call_with(&cx)
                        .arg(js_event)
                        .apply::<JsNull, _>(&mut cx)?;
                }
                Ok(())
            });
        }) as std::sync::Arc<dyn Fn(libparsec::ClientEvent) + Send + Sync>
    };
    let bootstrap_organization_addr = {
        let js_val = cx.argument::<JsString>(2)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::BackendOrganizationBootstrapAddr::from_any(&s).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let save_strategy = {
        let js_val = cx.argument::<JsObject>(3)?;
        variant_devicesavestrategy_js_to_rs(&mut cx, js_val)?
    };
    let human_handle = match cx.argument_opt(4) {
        Some(v) => match v.downcast::<JsObject, _>(&mut cx) {
            Ok(js_val) => Some(struct_humanhandle_js_to_rs(&mut cx, js_val)?),
            Err(_) => None,
        },
        None => None,
    };
    let device_label = match cx.argument_opt(5) {
        Some(v) => match v.downcast::<JsString, _>(&mut cx) {
            Ok(js_val) => Some({
                match js_val.value(&mut cx).parse() {
                    Ok(val) => val,
                    Err(err) => return cx.throw_type_error(err),
                }
            }),
            Err(_) => None,
        },
        None => None,
    };
    let sequester_authority_verify_key = match cx.argument_opt(6) {
        Some(v) => match v.downcast::<JsTypedArray<u8>, _>(&mut cx) {
            Ok(js_val) => Some({
                match js_val.as_slice(&mut cx).try_into() {
                    Ok(val) => val,
                    Err(err) => return cx.throw_type_error(format!("{}", err)),
                }
            }),
            Err(_) => None,
        },
        None => None,
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
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

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_availabledevice_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_bootstraporganizationerror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// claimer_retrieve_info
fn claimer_retrieve_info(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let config = {
        let js_val = cx.argument::<JsObject>(0)?;
        struct_clientconfig_js_to_rs(&mut cx, js_val)?
    };
    let on_event_callback = {
        let js_val = cx.argument::<JsFunction>(1)?;
        // The Javascript function object is going to be shared between the closure
        // called by rust (that can be called multiple times) and the single-use
        // closure sent to the js runtime.
        // So we must use an Arc to ensure the resource is shared correctly, but
        // that's not all of it !
        // When the resource is no longer use, we must consume the reference we
        // had on the javascript function in a neon context so that it can itself
        // notify the js runtime's garbage collector.
        struct Callback {
            js_fn: Option<neon::handle::Root<JsFunction>>,
            channel: neon::event::Channel,
        }
        impl Drop for Callback {
            fn drop(&mut self) {
                if let Some(js_fn) = self.js_fn.take() {
                    // Return the js object to the js runtime to avoid memory leak
                    self.channel.send(move |mut cx| {
                        js_fn.to_inner(&mut cx);
                        Ok(())
                    });
                }
            }
        }
        let callback = std::sync::Arc::new(Callback {
            js_fn: Some(js_val.root(&mut cx)),
            channel: cx.channel(),
        });
        std::sync::Arc::new(move |event: libparsec::ClientEvent| {
            let callback2 = callback.clone();
            callback.channel.send(move |mut cx| {
                // TODO: log an error instead of panic ? (it is a bit harsh to crash
                // the current task if an unrelated event handler has a bug...)
                let js_event = variant_clientevent_rs_to_js(&mut cx, event)?;
                if let Some(ref js_fn) = callback2.js_fn {
                    js_fn
                        .to_inner(&mut cx)
                        .call_with(&cx)
                        .arg(js_event)
                        .apply::<JsNull, _>(&mut cx)?;
                }
                Ok(())
            });
        }) as std::sync::Arc<dyn Fn(libparsec::ClientEvent) + Send + Sync>
    };
    let addr = {
        let js_val = cx.argument::<JsString>(2)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::BackendInvitationAddr::from_any(&s).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_retrieve_info(config, on_event_callback, addr).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = variant_userordeviceclaiminitialinfo_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claimerretrieveinfoerror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// claimer_user_initial_do_wait_peer
fn claimer_user_initial_do_wait_peer(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_user_initial_do_wait_peer(canceller, handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_userclaiminprogress1info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claiminprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// claimer_device_initial_do_wait_peer
fn claimer_device_initial_do_wait_peer(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_device_initial_do_wait_peer(canceller, handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_deviceclaiminprogress1info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claiminprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// claimer_user_in_progress_1_do_signify_trust
fn claimer_user_in_progress_1_do_signify_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::claimer_user_in_progress_1_do_signify_trust(canceller, handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_userclaiminprogress2info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claiminprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// claimer_device_in_progress_1_do_signify_trust
fn claimer_device_in_progress_1_do_signify_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::claimer_device_in_progress_1_do_signify_trust(canceller, handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_deviceclaiminprogress2info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claiminprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// claimer_user_in_progress_2_do_wait_peer_trust
fn claimer_user_in_progress_2_do_wait_peer_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::claimer_user_in_progress_2_do_wait_peer_trust(canceller, handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_userclaiminprogress3info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claiminprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// claimer_device_in_progress_2_do_wait_peer_trust
fn claimer_device_in_progress_2_do_wait_peer_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::claimer_device_in_progress_2_do_wait_peer_trust(canceller, handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_deviceclaiminprogress3info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claiminprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// claimer_user_in_progress_3_do_claim
fn claimer_user_in_progress_3_do_claim(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let requested_device_label = match cx.argument_opt(2) {
        Some(v) => match v.downcast::<JsString, _>(&mut cx) {
            Ok(js_val) => Some({
                match js_val.value(&mut cx).parse() {
                    Ok(val) => val,
                    Err(err) => return cx.throw_type_error(err),
                }
            }),
            Err(_) => None,
        },
        None => None,
    };
    let requested_human_handle = match cx.argument_opt(3) {
        Some(v) => match v.downcast::<JsObject, _>(&mut cx) {
            Ok(js_val) => Some(struct_humanhandle_js_to_rs(&mut cx, js_val)?),
            Err(_) => None,
        },
        None => None,
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_user_in_progress_3_do_claim(
                canceller,
                handle,
                requested_device_label,
                requested_human_handle,
            )
            .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_userclaimfinalizeinfo_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claiminprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// claimer_device_in_progress_3_do_claim
fn claimer_device_in_progress_3_do_claim(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let requested_device_label = match cx.argument_opt(2) {
        Some(v) => match v.downcast::<JsString, _>(&mut cx) {
            Ok(js_val) => Some({
                match js_val.value(&mut cx).parse() {
                    Ok(val) => val,
                    Err(err) => return cx.throw_type_error(err),
                }
            }),
            Err(_) => None,
        },
        None => None,
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_device_in_progress_3_do_claim(
                canceller,
                handle,
                requested_device_label,
            )
            .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_deviceclaimfinalizeinfo_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claiminprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// claimer_user_finalize_save_local_device
fn claimer_user_finalize_save_local_device(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let save_strategy = {
        let js_val = cx.argument::<JsObject>(1)?;
        variant_devicesavestrategy_js_to_rs(&mut cx, js_val)?
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::claimer_user_finalize_save_local_device(handle, save_strategy).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_availabledevice_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claiminprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// claimer_device_finalize_save_local_device
fn claimer_device_finalize_save_local_device(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let save_strategy = {
        let js_val = cx.argument::<JsObject>(1)?;
        variant_devicesavestrategy_js_to_rs(&mut cx, js_val)?
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::claimer_device_finalize_save_local_device(handle, save_strategy).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_availabledevice_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claiminprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_new_user_invitation
fn client_new_user_invitation(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let claimer_email = {
        let js_val = cx.argument::<JsString>(1)?;
        js_val.value(&mut cx)
    };
    let send_email = {
        let js_val = cx.argument::<JsBoolean>(2)?;
        js_val.value(&mut cx)
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::client_new_user_invitation(client, claimer_email, send_email).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let (x1, x2) = ok;
                            let js_array = JsArray::new(&mut cx, 2);
                            let js_value = {
                                let rs_buff = {
                                    let custom_to_rs_bytes =
                                        |x: libparsec::InvitationToken| -> Result<_, &'static str> {
                                            Ok(x.as_bytes().to_owned())
                                        };
                                    match custom_to_rs_bytes(x1) {
                                        Ok(ok) => ok,
                                        Err(err) => return cx.throw_type_error(err),
                                    }
                                };
                                let mut js_buff = JsArrayBuffer::new(&mut cx, rs_buff.len())?;
                                let js_buff_slice = js_buff.as_mut_slice(&mut cx);
                                for (i, c) in rs_buff.iter().enumerate() {
                                    js_buff_slice[i] = *c;
                                }
                                js_buff
                            };
                            js_array.set(&mut cx, 1, js_value)?;
                            let js_value = variant_invitationemailsentstatus_rs_to_js(&mut cx, x2)?;
                            js_array.set(&mut cx, 2, js_value)?;
                            js_array
                        };
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_newuserinvitationerror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_new_device_invitation
fn client_new_device_invitation(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let send_email = {
        let js_val = cx.argument::<JsBoolean>(1)?;
        js_val.value(&mut cx)
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_new_device_invitation(client, send_email).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let (x1, x2) = ok;
                            let js_array = JsArray::new(&mut cx, 2);
                            let js_value = {
                                let rs_buff = {
                                    let custom_to_rs_bytes =
                                        |x: libparsec::InvitationToken| -> Result<_, &'static str> {
                                            Ok(x.as_bytes().to_owned())
                                        };
                                    match custom_to_rs_bytes(x1) {
                                        Ok(ok) => ok,
                                        Err(err) => return cx.throw_type_error(err),
                                    }
                                };
                                let mut js_buff = JsArrayBuffer::new(&mut cx, rs_buff.len())?;
                                let js_buff_slice = js_buff.as_mut_slice(&mut cx);
                                for (i, c) in rs_buff.iter().enumerate() {
                                    js_buff_slice[i] = *c;
                                }
                                js_buff
                            };
                            js_array.set(&mut cx, 1, js_value)?;
                            let js_value = variant_invitationemailsentstatus_rs_to_js(&mut cx, x2)?;
                            js_array.set(&mut cx, 2, js_value)?;
                            js_array
                        };
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_newdeviceinvitationerror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_delete_invitation
fn client_delete_invitation(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let token = {
        let js_val = cx.argument::<JsTypedArray<u8>>(1)?;
        {
            let custom_from_rs_bytes = |x: &[u8]| -> Result<_, _> {
                libparsec::InvitationToken::try_from(x).map_err(|e| e.to_string())
            };
            match custom_from_rs_bytes(js_val.as_slice(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(format!("{}", err)),
            }
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_delete_invitation(client, token).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let _ = ok;
                            JsNull::new(&mut cx)
                        };
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_deleteinvitationerror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_list_invitations
fn client_list_invitations(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_list_invitations(client).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            // JsArray::new allocates with `undefined` value, that's why we `set` value
                            let js_array = JsArray::new(&mut cx, ok.len() as u32);
                            for (i, elem) in ok.into_iter().enumerate() {
                                let js_elem = variant_invitelistitem_rs_to_js(&mut cx, elem)?;
                                js_array.set(&mut cx, i as u32, js_elem)?;
                            }
                            js_array
                        };
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_listinvitationserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_start_user_invitation_greet
fn client_start_user_invitation_greet(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let token = {
        let js_val = cx.argument::<JsTypedArray<u8>>(1)?;
        {
            let custom_from_rs_bytes = |x: &[u8]| -> Result<_, _> {
                libparsec::InvitationToken::try_from(x).map_err(|e| e.to_string())
            };
            match custom_from_rs_bytes(js_val.as_slice(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(format!("{}", err)),
            }
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_start_user_invitation_greet(client, token).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_usergreetinitialinfo_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_clientstartinvitationgreeterror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_start_device_invitation_greet
fn client_start_device_invitation_greet(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let token = {
        let js_val = cx.argument::<JsTypedArray<u8>>(1)?;
        {
            let custom_from_rs_bytes = |x: &[u8]| -> Result<_, _> {
                libparsec::InvitationToken::try_from(x).map_err(|e| e.to_string())
            };
            match custom_from_rs_bytes(js_val.as_slice(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(format!("{}", err)),
            }
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_start_device_invitation_greet(client, token).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_devicegreetinitialinfo_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_clientstartinvitationgreeterror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// greeter_user_initial_do_wait_peer
fn greeter_user_initial_do_wait_peer(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::greeter_user_initial_do_wait_peer(canceller, handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_usergreetinprogress1info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greetinprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// greeter_device_initial_do_wait_peer
fn greeter_device_initial_do_wait_peer(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::greeter_device_initial_do_wait_peer(canceller, handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_devicegreetinprogress1info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greetinprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// greeter_user_in_progress_1_do_wait_peer_trust
fn greeter_user_in_progress_1_do_wait_peer_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::greeter_user_in_progress_1_do_wait_peer_trust(canceller, handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_usergreetinprogress2info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greetinprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// greeter_device_in_progress_1_do_wait_peer_trust
fn greeter_device_in_progress_1_do_wait_peer_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::greeter_device_in_progress_1_do_wait_peer_trust(canceller, handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_devicegreetinprogress2info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greetinprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// greeter_user_in_progress_2_do_signify_trust
fn greeter_user_in_progress_2_do_signify_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::greeter_user_in_progress_2_do_signify_trust(canceller, handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_usergreetinprogress3info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greetinprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// greeter_device_in_progress_2_do_signify_trust
fn greeter_device_in_progress_2_do_signify_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::greeter_device_in_progress_2_do_signify_trust(canceller, handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_devicegreetinprogress3info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greetinprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// greeter_user_in_progress_3_do_get_claim_requests
fn greeter_user_in_progress_3_do_get_claim_requests(
    mut cx: FunctionContext,
) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::greeter_user_in_progress_3_do_get_claim_requests(canceller, handle)
                    .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_usergreetinprogress4info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greetinprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// greeter_device_in_progress_3_do_get_claim_requests
fn greeter_device_in_progress_3_do_get_claim_requests(
    mut cx: FunctionContext,
) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::greeter_device_in_progress_3_do_get_claim_requests(canceller, handle)
                    .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_devicegreetinprogress4info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greetinprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// greeter_user_in_progress_4_do_create
fn greeter_user_in_progress_4_do_create(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let human_handle = match cx.argument_opt(2) {
        Some(v) => match v.downcast::<JsObject, _>(&mut cx) {
            Ok(js_val) => Some(struct_humanhandle_js_to_rs(&mut cx, js_val)?),
            Err(_) => None,
        },
        None => None,
    };
    let device_label = match cx.argument_opt(3) {
        Some(v) => match v.downcast::<JsString, _>(&mut cx) {
            Ok(js_val) => Some({
                match js_val.value(&mut cx).parse() {
                    Ok(val) => val,
                    Err(err) => return cx.throw_type_error(err),
                }
            }),
            Err(_) => None,
        },
        None => None,
    };
    let profile = {
        let js_val = cx.argument::<JsObject>(4)?;
        variant_userprofile_js_to_rs(&mut cx, js_val)?
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::greeter_user_in_progress_4_do_create(
                canceller,
                handle,
                human_handle,
                device_label,
                profile,
            )
            .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let _ = ok;
                            JsNull::new(&mut cx)
                        };
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greetinprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// greeter_device_in_progress_4_do_create
fn greeter_device_in_progress_4_do_create(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            v as u32
        }
    };
    let device_label = match cx.argument_opt(2) {
        Some(v) => match v.downcast::<JsString, _>(&mut cx) {
            Ok(js_val) => Some({
                match js_val.value(&mut cx).parse() {
                    Ok(val) => val,
                    Err(err) => return cx.throw_type_error(err),
                }
            }),
            Err(_) => None,
        },
        None => None,
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::greeter_device_in_progress_4_do_create(canceller, handle, device_label)
                    .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let _ = ok;
                            JsNull::new(&mut cx)
                        };
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greetinprogresserror_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// test_new_testbed
fn test_new_testbed(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let template = {
        let js_val = cx.argument::<JsString>(0)?;
        js_val.value(&mut cx)
    };
    let test_server = match cx.argument_opt(1) {
        Some(v) => match v.downcast::<JsString, _>(&mut cx) {
            Ok(js_val) => Some({
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    libparsec::BackendAddr::from_any(&s).map_err(|e| e.to_string())
                };
                match custom_from_rs_string(js_val.value(&mut cx)) {
                    Ok(val) => val,
                    Err(err) => return cx.throw_type_error(err),
                }
            }),
            Err(_) => None,
        },
        None => None,
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::test_new_testbed(&template, test_server.as_ref()).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = JsString::try_new(&mut cx, {
                    let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                        path.into_os_string()
                            .into_string()
                            .map_err(|_| "Path contains non-utf8 characters")
                    };
                    match custom_to_rs_string(ret) {
                        Ok(ok) => ok,
                        Err(err) => return cx.throw_type_error(err),
                    }
                })
                .or_throw(&mut cx)?;
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// test_get_testbed_organization_id
fn test_get_testbed_organization_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let discriminant_dir = {
        let js_val = cx.argument::<JsString>(0)?;
        {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let ret = libparsec::test_get_testbed_organization_id(&discriminant_dir);
    let js_ret = match ret {
        Some(elem) => JsString::try_new(&mut cx, elem)
            .or_throw(&mut cx)?
            .as_value(&mut cx),
        None => JsNull::new(&mut cx).as_value(&mut cx),
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// test_get_testbed_bootstrap_organization_addr
fn test_get_testbed_bootstrap_organization_addr(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let discriminant_dir = {
        let js_val = cx.argument::<JsString>(0)?;
        {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let ret = libparsec::test_get_testbed_bootstrap_organization_addr(&discriminant_dir);
    let js_ret = match ret {
        Some(elem) => {
            JsString::try_new(&mut cx,{
                let custom_to_rs_string = |addr: libparsec::BackendOrganizationBootstrapAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) };
                match custom_to_rs_string(elem) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            }).or_throw(&mut cx)?.as_value(&mut cx)
        }
        None => JsNull::new(&mut cx).as_value(&mut cx),
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// test_drop_testbed
fn test_drop_testbed(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let path = {
        let js_val = cx.argument::<JsString>(0)?;
        {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            libparsec::test_drop_testbed(&path).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = cx.null();
                Ok(js_ret)
            });
        });

    Ok(promise)
}

pub fn register_meths(cx: &mut ModuleContext) -> NeonResult<()> {
    cx.export_function("cancel", cancel)?;
    cx.export_function("newCanceller", new_canceller)?;
    cx.export_function("clientStart", client_start)?;
    cx.export_function("clientStop", client_stop)?;
    cx.export_function("clientListWorkspaces", client_list_workspaces)?;
    cx.export_function("clientWorkspaceCreate", client_workspace_create)?;
    cx.export_function("clientWorkspaceRename", client_workspace_rename)?;
    cx.export_function("clientWorkspaceShare", client_workspace_share)?;
    cx.export_function(
        "claimerGreeterAbortOperation",
        claimer_greeter_abort_operation,
    )?;
    cx.export_function("listAvailableDevices", list_available_devices)?;
    cx.export_function("bootstrapOrganization", bootstrap_organization)?;
    cx.export_function("claimerRetrieveInfo", claimer_retrieve_info)?;
    cx.export_function(
        "claimerUserInitialDoWaitPeer",
        claimer_user_initial_do_wait_peer,
    )?;
    cx.export_function(
        "claimerDeviceInitialDoWaitPeer",
        claimer_device_initial_do_wait_peer,
    )?;
    cx.export_function(
        "claimerUserInProgress1DoSignifyTrust",
        claimer_user_in_progress_1_do_signify_trust,
    )?;
    cx.export_function(
        "claimerDeviceInProgress1DoSignifyTrust",
        claimer_device_in_progress_1_do_signify_trust,
    )?;
    cx.export_function(
        "claimerUserInProgress2DoWaitPeerTrust",
        claimer_user_in_progress_2_do_wait_peer_trust,
    )?;
    cx.export_function(
        "claimerDeviceInProgress2DoWaitPeerTrust",
        claimer_device_in_progress_2_do_wait_peer_trust,
    )?;
    cx.export_function(
        "claimerUserInProgress3DoClaim",
        claimer_user_in_progress_3_do_claim,
    )?;
    cx.export_function(
        "claimerDeviceInProgress3DoClaim",
        claimer_device_in_progress_3_do_claim,
    )?;
    cx.export_function(
        "claimerUserFinalizeSaveLocalDevice",
        claimer_user_finalize_save_local_device,
    )?;
    cx.export_function(
        "claimerDeviceFinalizeSaveLocalDevice",
        claimer_device_finalize_save_local_device,
    )?;
    cx.export_function("clientNewUserInvitation", client_new_user_invitation)?;
    cx.export_function("clientNewDeviceInvitation", client_new_device_invitation)?;
    cx.export_function("clientDeleteInvitation", client_delete_invitation)?;
    cx.export_function("clientListInvitations", client_list_invitations)?;
    cx.export_function(
        "clientStartUserInvitationGreet",
        client_start_user_invitation_greet,
    )?;
    cx.export_function(
        "clientStartDeviceInvitationGreet",
        client_start_device_invitation_greet,
    )?;
    cx.export_function(
        "greeterUserInitialDoWaitPeer",
        greeter_user_initial_do_wait_peer,
    )?;
    cx.export_function(
        "greeterDeviceInitialDoWaitPeer",
        greeter_device_initial_do_wait_peer,
    )?;
    cx.export_function(
        "greeterUserInProgress1DoWaitPeerTrust",
        greeter_user_in_progress_1_do_wait_peer_trust,
    )?;
    cx.export_function(
        "greeterDeviceInProgress1DoWaitPeerTrust",
        greeter_device_in_progress_1_do_wait_peer_trust,
    )?;
    cx.export_function(
        "greeterUserInProgress2DoSignifyTrust",
        greeter_user_in_progress_2_do_signify_trust,
    )?;
    cx.export_function(
        "greeterDeviceInProgress2DoSignifyTrust",
        greeter_device_in_progress_2_do_signify_trust,
    )?;
    cx.export_function(
        "greeterUserInProgress3DoGetClaimRequests",
        greeter_user_in_progress_3_do_get_claim_requests,
    )?;
    cx.export_function(
        "greeterDeviceInProgress3DoGetClaimRequests",
        greeter_device_in_progress_3_do_get_claim_requests,
    )?;
    cx.export_function(
        "greeterUserInProgress4DoCreate",
        greeter_user_in_progress_4_do_create,
    )?;
    cx.export_function(
        "greeterDeviceInProgress4DoCreate",
        greeter_device_in_progress_4_do_create,
    )?;
    cx.export_function("testNewTestbed", test_new_testbed)?;
    cx.export_function(
        "testGetTestbedOrganizationId",
        test_get_testbed_organization_id,
    )?;
    cx.export_function(
        "testGetTestbedBootstrapOrganizationAddr",
        test_get_testbed_bootstrap_organization_addr,
    )?;
    cx.export_function("testDropTestbed", test_drop_testbed)?;
    Ok(())
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

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

// UserClaimInProgress1CtxHandle

#[allow(dead_code)]
fn struct_userclaiminprogress1ctxhandle_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserClaimInProgress1CtxHandle> {
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
                let js_item = js_val.get(cx, i)?;
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
    Ok(libparsec::UserClaimInProgress1CtxHandle {
        handle,
        greeter_sas,
        greeter_sas_choices,
    })
}

#[allow(dead_code)]
fn struct_userclaiminprogress1ctxhandle_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserClaimInProgress1CtxHandle,
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

// DeviceClaimInProgress1CtxHandle

#[allow(dead_code)]
fn struct_deviceclaiminprogress1ctxhandle_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceClaimInProgress1CtxHandle> {
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
                let js_item = js_val.get(cx, i)?;
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
    Ok(libparsec::DeviceClaimInProgress1CtxHandle {
        handle,
        greeter_sas,
        greeter_sas_choices,
    })
}

#[allow(dead_code)]
fn struct_deviceclaiminprogress1ctxhandle_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceClaimInProgress1CtxHandle,
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

// UserClaimInProgress2CtxHandle

#[allow(dead_code)]
fn struct_userclaiminprogress2ctxhandle_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserClaimInProgress2CtxHandle> {
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
    Ok(libparsec::UserClaimInProgress2CtxHandle {
        handle,
        claimer_sas,
    })
}

#[allow(dead_code)]
fn struct_userclaiminprogress2ctxhandle_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserClaimInProgress2CtxHandle,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_claimer_sas = JsString::try_new(cx, rs_obj.claimer_sas).or_throw(cx)?;
    js_obj.set(cx, "claimerSas", js_claimer_sas)?;
    Ok(js_obj)
}

// DeviceClaimInProgress2CtxHandle

#[allow(dead_code)]
fn struct_deviceclaiminprogress2ctxhandle_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceClaimInProgress2CtxHandle> {
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
    Ok(libparsec::DeviceClaimInProgress2CtxHandle {
        handle,
        claimer_sas,
    })
}

#[allow(dead_code)]
fn struct_deviceclaiminprogress2ctxhandle_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceClaimInProgress2CtxHandle,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_claimer_sas = JsString::try_new(cx, rs_obj.claimer_sas).or_throw(cx)?;
    js_obj.set(cx, "claimerSas", js_claimer_sas)?;
    Ok(js_obj)
}

// UserClaimInProgress3CtxHandle

#[allow(dead_code)]
fn struct_userclaiminprogress3ctxhandle_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserClaimInProgress3CtxHandle> {
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
    Ok(libparsec::UserClaimInProgress3CtxHandle { handle })
}

#[allow(dead_code)]
fn struct_userclaiminprogress3ctxhandle_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserClaimInProgress3CtxHandle,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// DeviceClaimInProgress3CtxHandle

#[allow(dead_code)]
fn struct_deviceclaiminprogress3ctxhandle_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceClaimInProgress3CtxHandle> {
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
    Ok(libparsec::DeviceClaimInProgress3CtxHandle { handle })
}

#[allow(dead_code)]
fn struct_deviceclaiminprogress3ctxhandle_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceClaimInProgress3CtxHandle,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// UserClaimFinalizeCtxHandle

#[allow(dead_code)]
fn struct_userclaimfinalizectxhandle_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserClaimFinalizeCtxHandle> {
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
    Ok(libparsec::UserClaimFinalizeCtxHandle { handle })
}

#[allow(dead_code)]
fn struct_userclaimfinalizectxhandle_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserClaimFinalizeCtxHandle,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// DeviceClaimFinalizeCtxHandle

#[allow(dead_code)]
fn struct_deviceclaimfinalizectxhandle_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceClaimFinalizeCtxHandle> {
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
    Ok(libparsec::DeviceClaimFinalizeCtxHandle { handle })
}

#[allow(dead_code)]
fn struct_deviceclaimfinalizectxhandle_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceClaimFinalizeCtxHandle,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
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

// UserOrDeviceClaimInitialCtxHandle

#[allow(dead_code)]
fn variant_userordeviceclaiminitialctxhandle_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserOrDeviceClaimInitialCtxHandle> {
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
            Ok(libparsec::UserOrDeviceClaimInitialCtxHandle::Device {
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
            Ok(libparsec::UserOrDeviceClaimInitialCtxHandle::User {
                handle,
                claimer_email,
                greeter_user_id,
                greeter_human_handle,
            })
        }
        _ => cx.throw_type_error("Object is not a UserOrDeviceClaimInitialCtxHandle"),
    }
}

#[allow(dead_code)]
fn variant_userordeviceclaiminitialctxhandle_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserOrDeviceClaimInitialCtxHandle,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::UserOrDeviceClaimInitialCtxHandle::Device {
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
                Some(elem) => JsString::try_new(cx, elem).or_throw(cx)?.as_value(cx),
                None => JsNull::new(cx).as_value(cx),
            };
            js_obj.set(cx, "greeter_human_handle", js_greeter_human_handle)?;
        }
        libparsec::UserOrDeviceClaimInitialCtxHandle::User {
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
                Some(elem) => JsString::try_new(cx, elem).or_throw(cx)?.as_value(cx),
                None => JsNull::new(cx).as_value(cx),
            };
            js_obj.set(cx, "greeter_human_handle", js_greeter_human_handle)?;
        }
    }
    Ok(js_obj)
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
                        let js_value =
                            variant_userordeviceclaiminitialctxhandle_rs_to_js(&mut cx, ok)?;
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

// claimer_user_initial_ctx_do_wait_peer
fn claimer_user_initial_ctx_do_wait_peer(mut cx: FunctionContext) -> JsResult<JsPromise> {
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_user_initial_ctx_do_wait_peer(handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_userclaiminprogress1ctxhandle_rs_to_js(&mut cx, ok)?;
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

// claimer_device_initial_ctx_do_wait_peer
fn claimer_device_initial_ctx_do_wait_peer(mut cx: FunctionContext) -> JsResult<JsPromise> {
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_device_initial_ctx_do_wait_peer(handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value =
                            struct_deviceclaiminprogress1ctxhandle_rs_to_js(&mut cx, ok)?;
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

// claimer_user_in_progress_2_do_signify_trust
fn claimer_user_in_progress_2_do_signify_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_user_in_progress_2_do_signify_trust(handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_userclaiminprogress2ctxhandle_rs_to_js(&mut cx, ok)?;
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

// claimer_device_in_progress_2_do_signify_trust
fn claimer_device_in_progress_2_do_signify_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_device_in_progress_2_do_signify_trust(handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value =
                            struct_deviceclaiminprogress2ctxhandle_rs_to_js(&mut cx, ok)?;
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_user_in_progress_2_do_wait_peer_trust(handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_userclaiminprogress3ctxhandle_rs_to_js(&mut cx, ok)?;
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_device_in_progress_2_do_wait_peer_trust(handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value =
                            struct_deviceclaiminprogress3ctxhandle_rs_to_js(&mut cx, ok)?;
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
    let requested_device_label = match cx.argument_opt(1) {
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
    let requested_human_handle = match cx.argument_opt(2) {
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
            let ret = libparsec::claimer_user_in_progress_3_do_claim(
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
                        let js_value = struct_userclaimfinalizectxhandle_rs_to_js(&mut cx, ok)?;
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
    let requested_device_label = match cx.argument_opt(1) {
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
                libparsec::claimer_device_in_progress_3_do_claim(handle, requested_device_label)
                    .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_deviceclaimfinalizectxhandle_rs_to_js(&mut cx, ok)?;
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
    cx.export_function("listAvailableDevices", list_available_devices)?;
    cx.export_function("bootstrapOrganization", bootstrap_organization)?;
    cx.export_function("claimerRetrieveInfo", claimer_retrieve_info)?;
    cx.export_function(
        "claimerUserInitialCtxDoWaitPeer",
        claimer_user_initial_ctx_do_wait_peer,
    )?;
    cx.export_function(
        "claimerDeviceInitialCtxDoWaitPeer",
        claimer_device_initial_ctx_do_wait_peer,
    )?;
    cx.export_function(
        "claimerUserInProgress2DoSignifyTrust",
        claimer_user_in_progress_2_do_signify_trust,
    )?;
    cx.export_function(
        "claimerDeviceInProgress2DoSignifyTrust",
        claimer_device_in_progress_2_do_signify_trust,
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

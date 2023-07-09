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
    let preferred_org_creation_backend_addr = {
        let js_val: Handle<JsString> = obj.get(cx, "preferredOrgCreationBackendAddr")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::BackendAddr::from_any(&s).map_err(|e| e.to_string())
            };
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
        preferred_org_creation_backend_addr,
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
    let js_preferred_org_creation_backend_addr = JsString::try_new(cx, {
        let custom_to_rs_string = |addr: libparsec::BackendAddr| -> Result<String, &'static str> {
            Ok(addr.to_url().into())
        };
        match custom_to_rs_string(rs_obj.preferred_org_creation_backend_addr) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(
        cx,
        "preferredOrgCreationBackendAddr",
        js_preferred_org_creation_backend_addr,
    )?;
    let js_workspace_storage_cache_size =
        variant_workspacestoragecachesize_rs_to_js(cx, rs_obj.workspace_storage_cache_size)?;
    js_obj.set(
        cx,
        "workspaceStorageCacheSize",
        js_workspace_storage_cache_size,
    )?;
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
    cx.export_function("testNewTestbed", test_new_testbed)?;
    cx.export_function("testDropTestbed", test_drop_testbed)?;
    Ok(())
}

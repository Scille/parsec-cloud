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
        match (|s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) })(
            js_val.value(cx),
        ) {
            Ok(val) => val,
            Err(err) => return cx.throw_type_error(err),
        }
    };
    let data_base_dir = {
        let js_val: Handle<JsString> = obj.get(cx, "dataBaseDir")?;
        match (|s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) })(
            js_val.value(cx),
        ) {
            Ok(val) => val,
            Err(err) => return cx.throw_type_error(err),
        }
    };
    let mountpoint_base_dir = {
        let js_val: Handle<JsString> = obj.get(cx, "mountpointBaseDir")?;
        match (|s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) })(
            js_val.value(cx),
        ) {
            Ok(val) => val,
            Err(err) => return cx.throw_type_error(err),
        }
    };
    let preferred_org_creation_backend_addr = {
        let js_val: Handle<JsString> = obj.get(cx, "preferredOrgCreationBackendAddr")?;
        match (|s: String| -> Result<_, _> { libparsec::BackendAddr::from_any(&s) })(
            js_val.value(cx),
        ) {
            Ok(val) => val,
            Err(err) => return cx.throw_type_error(err),
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
    let js_config_dir = JsString::try_new(
        cx,
        match (|path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        })(rs_obj.config_dir)
        {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        },
    )
    .or_throw(cx)?;
    js_obj.set(cx, "configDir", js_config_dir)?;
    let js_data_base_dir = JsString::try_new(
        cx,
        match (|path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        })(rs_obj.data_base_dir)
        {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        },
    )
    .or_throw(cx)?;
    js_obj.set(cx, "dataBaseDir", js_data_base_dir)?;
    let js_mountpoint_base_dir = JsString::try_new(
        cx,
        match (|path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        })(rs_obj.mountpoint_base_dir)
        {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        },
    )
    .or_throw(cx)?;
    js_obj.set(cx, "mountpointBaseDir", js_mountpoint_base_dir)?;
    let js_preferred_org_creation_backend_addr = JsString::try_new(
        cx,
        match (|addr: libparsec::BackendAddr| -> Result<String, &'static str> {
            Ok(addr.to_url().into())
        })(rs_obj.preferred_org_creation_backend_addr)
        {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        },
    )
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

// ClientEvent

#[allow(dead_code)]
fn variant_clientevent_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ClientEvent> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "ClientConnectionChanged" => {
            let client = {
                let js_val: Handle<JsNumber> = obj.get(cx, "client")?;
                (js_val.value(cx) as i32).into()
            };
            Ok(libparsec::ClientEvent::ClientConnectionChanged { client })
        }
        "WorkspaceReencryptionEnded" => Ok(libparsec::ClientEvent::WorkspaceReencryptionEnded {}),
        "WorkspaceReencryptionNeeded" => Ok(libparsec::ClientEvent::WorkspaceReencryptionNeeded {}),
        "WorkspaceReencryptionStarted" => {
            Ok(libparsec::ClientEvent::WorkspaceReencryptionStarted {})
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
        libparsec::ClientEvent::ClientConnectionChanged { client } => {
            let js_tag = JsString::try_new(cx, "ClientConnectionChanged").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_client = JsNumber::new(cx, i32::from(client) as f64);
            js_obj.set(cx, "client", js_client)?;
        }
        libparsec::ClientEvent::WorkspaceReencryptionEnded {} => {
            let js_tag = JsString::try_new(cx, "WorkspaceReencryptionEnded").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientEvent::WorkspaceReencryptionNeeded {} => {
            let js_tag = JsString::try_new(cx, "WorkspaceReencryptionNeeded").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientEvent::WorkspaceReencryptionStarted {} => {
            let js_tag = JsString::try_new(cx, "WorkspaceReencryptionStarted").or_throw(cx)?;
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
                js_val.value(cx) as i32
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
        libparsec::WorkspaceStorageCacheSize::Custom { size } => {
            let js_tag = JsString::try_new(cx, "Custom").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_size = JsNumber::new(cx, size as f64);
            js_obj.set(cx, "size", js_size)?;
        }
        libparsec::WorkspaceStorageCacheSize::Default {} => {
            let js_tag = JsString::try_new(cx, "Default").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// DeviceAccessParams

#[allow(dead_code)]
fn variant_deviceaccessparams_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceAccessParams> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "Password" => {
            let path = {
                let js_val: Handle<JsString> = obj.get(cx, "path")?;
                match (|s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) })(
                    js_val.value(cx),
                ) {
                    Ok(val) => val,
                    Err(err) => return cx.throw_type_error(err),
                }
            };
            let password = {
                let js_val: Handle<JsString> = obj.get(cx, "password")?;
                js_val.value(cx)
            };
            Ok(libparsec::DeviceAccessParams::Password { path, password })
        }
        "Smartcard" => {
            let path = {
                let js_val: Handle<JsString> = obj.get(cx, "path")?;
                match (|s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) })(
                    js_val.value(cx),
                ) {
                    Ok(val) => val,
                    Err(err) => return cx.throw_type_error(err),
                }
            };
            Ok(libparsec::DeviceAccessParams::Smartcard { path })
        }
        _ => cx.throw_type_error("Object is not a DeviceAccessParams"),
    }
}

#[allow(dead_code)]
fn variant_deviceaccessparams_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceAccessParams,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::DeviceAccessParams::Password { path, password } => {
            let js_tag = JsString::try_new(cx, "Password").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_path = JsString::try_new(
                cx,
                match (|path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                })(path)
                {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                },
            )
            .or_throw(cx)?;
            js_obj.set(cx, "path", js_path)?;
            let js_password = JsString::try_new(cx, password).or_throw(cx)?;
            js_obj.set(cx, "password", js_password)?;
        }
        libparsec::DeviceAccessParams::Smartcard { path } => {
            let js_tag = JsString::try_new(cx, "Smartcard").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_path = JsString::try_new(
                cx,
                match (|path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                })(path)
                {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                },
            )
            .or_throw(cx)?;
            js_obj.set(cx, "path", js_path)?;
        }
    }
    Ok(js_obj)
}

// ClientLoginError

#[allow(dead_code)]
fn variant_clientloginerror_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ClientLoginError> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "AccessMethodNotAvailable" => Ok(libparsec::ClientLoginError::AccessMethodNotAvailable {}),
        "DecryptionFailed" => Ok(libparsec::ClientLoginError::DecryptionFailed {}),
        "DeviceAlreadyLoggedIn" => Ok(libparsec::ClientLoginError::DeviceAlreadyLoggedIn {}),
        "DeviceInvalidFormat" => Ok(libparsec::ClientLoginError::DeviceInvalidFormat {}),
        _ => cx.throw_type_error("Object is not a ClientLoginError"),
    }
}

#[allow(dead_code)]
fn variant_clientloginerror_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientLoginError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::ClientLoginError::AccessMethodNotAvailable {} => {
            let js_tag = JsString::try_new(cx, "AccessMethodNotAvailable").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientLoginError::DecryptionFailed {} => {
            let js_tag = JsString::try_new(cx, "DecryptionFailed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientLoginError::DeviceAlreadyLoggedIn {} => {
            let js_tag = JsString::try_new(cx, "DeviceAlreadyLoggedIn").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientLoginError::DeviceInvalidFormat {} => {
            let js_tag = JsString::try_new(cx, "DeviceInvalidFormat").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// client_login
fn client_login(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let load_device_params = {
        let js_val = cx.argument::<JsObject>(0)?;
        variant_deviceaccessparams_js_to_rs(&mut cx, js_val)?
    };
    let config = {
        let js_val = cx.argument::<JsObject>(1)?;
        struct_clientconfig_js_to_rs(&mut cx, js_val)?
    };
    let on_event_callback = {
        let js_val = cx.argument::<JsFunction>(2)?;
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
        Box::new(move |event: libparsec::ClientEvent| {
            let callback2 = callback.clone();
            callback.channel.send(move |mut cx| {
                // TODO: log an error instead of panic ? (it is a bit harsh to crash
                // the current task if an unrelated event handler has a bug...)
                let js_event = variant_clientevent_rs_to_js(&mut cx, event)?;
                if let Some(ref js_fn) = callback2.js_fn {
                    js_fn
                        .to_inner(&mut cx)
                        .call_with(&mut cx)
                        .arg(js_event)
                        .apply::<JsNull, _>(&mut cx)?;
                }
                Ok(())
            });
        }) as Box<dyn FnMut(libparsec::ClientEvent) + Send>
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_login(load_device_params, config, on_event_callback).await;

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
                        let js_err = variant_clientloginerror_rs_to_js(&mut cx, err)?;
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
    cx.export_function("clientLogin", client_login)?;
    Ok(())
}

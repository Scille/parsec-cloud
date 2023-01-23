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
                (|s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) })(x)
                    .map_err(|e| TypeError::new(e))
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
                (|s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) })(x)
                    .map_err(|e| TypeError::new(e))
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
                (|s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) })(x)
                    .map_err(|e| TypeError::new(e))
            })
            .map_err(|_| TypeError::new("Not a valid Path"))?
    };
    let preferred_org_creation_backend_addr = {
        let js_val = Reflect::get(&obj, &"preferredOrgCreationBackendAddr".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                (|s: String| -> Result<_, _> { libparsec::BackendAddr::from_any(&s) })(x)
                    .map_err(|e| TypeError::new(e))
            })
            .map_err(|_| TypeError::new("Not a valid BackendAddr"))?
    };
    let workspace_storage_cache_size = {
        let js_val = Reflect::get(&obj, &"workspaceStorageCacheSize".into())?;
        variant_workspacestoragecachesize_js_to_rs(js_val)?
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
fn struct_clientconfig_rs_to_js(rs_obj: libparsec::ClientConfig) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_config_dir = JsValue::from_str(
        match (|path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        })(rs_obj.config_dir)
        {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err))),
        }
        .as_ref(),
    );
    Reflect::set(&js_obj, &"configDir".into(), &js_config_dir)?;
    let js_data_base_dir = JsValue::from_str(
        match (|path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        })(rs_obj.data_base_dir)
        {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err))),
        }
        .as_ref(),
    );
    Reflect::set(&js_obj, &"dataBaseDir".into(), &js_data_base_dir)?;
    let js_mountpoint_base_dir = JsValue::from_str(
        match (|path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        })(rs_obj.mountpoint_base_dir)
        {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err))),
        }
        .as_ref(),
    );
    Reflect::set(
        &js_obj,
        &"mountpointBaseDir".into(),
        &js_mountpoint_base_dir,
    )?;
    let js_preferred_org_creation_backend_addr = JsValue::from_str(
        match (|addr: libparsec::BackendAddr| -> Result<String, &'static str> {
            Ok(addr.to_url().into())
        })(rs_obj.preferred_org_creation_backend_addr)
        {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err))),
        }
        .as_ref(),
    );
    Reflect::set(
        &js_obj,
        &"preferredOrgCreationBackendAddr".into(),
        &js_preferred_org_creation_backend_addr,
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

// ClientEvent

#[allow(dead_code)]
fn variant_clientevent_js_to_rs(obj: JsValue) -> Result<libparsec::ClientEvent, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("ClientConnectionChanged") => {
            let client = {
                let js_val = Reflect::get(&obj, &"client".into())?;
                (js_val
                    .dyn_into::<Number>()
                    .map_err(|_| TypeError::new("Not a number"))?
                    .value_of() as i32)
                    .into()
            };
            Ok(libparsec::ClientEvent::ClientConnectionChanged { client })
        }
        tag if tag == JsValue::from_str("WorkspaceReencryptionEnded") => {
            Ok(libparsec::ClientEvent::WorkspaceReencryptionEnded {})
        }
        tag if tag == JsValue::from_str("WorkspaceReencryptionNeeded") => {
            Ok(libparsec::ClientEvent::WorkspaceReencryptionNeeded {})
        }
        tag if tag == JsValue::from_str("WorkspaceReencryptionStarted") => {
            Ok(libparsec::ClientEvent::WorkspaceReencryptionStarted {})
        }
        _ => Err(JsValue::from(TypeError::new("Object is not a ClientEvent"))),
    }
}

#[allow(dead_code)]
fn variant_clientevent_rs_to_js(rs_obj: libparsec::ClientEvent) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::ClientEvent::ClientConnectionChanged { client } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientConnectionChanged".into())?;
            let js_client = JsValue::from(i32::from(client));
            Reflect::set(&js_obj, &"client".into(), &js_client)?;
        }
        libparsec::ClientEvent::WorkspaceReencryptionEnded {} => {
            Reflect::set(&js_obj, &"tag".into(), &"WorkspaceReencryptionEnded".into())?;
        }
        libparsec::ClientEvent::WorkspaceReencryptionNeeded {} => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceReencryptionNeeded".into(),
            )?;
        }
        libparsec::ClientEvent::WorkspaceReencryptionStarted {} => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceReencryptionStarted".into(),
            )?;
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
                js_val
                    .dyn_into::<Number>()
                    .map_err(|_| TypeError::new("Not a number"))?
                    .value_of() as i32
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
        libparsec::WorkspaceStorageCacheSize::Custom { size } => {
            Reflect::set(&js_obj, &"tag".into(), &"Custom".into())?;
            let js_size = size.into();
            Reflect::set(&js_obj, &"size".into(), &js_size)?;
        }
        libparsec::WorkspaceStorageCacheSize::Default {} => {
            Reflect::set(&js_obj, &"tag".into(), &"Default".into())?;
        }
    }
    Ok(js_obj)
}

// DeviceAccessParams

#[allow(dead_code)]
fn variant_deviceaccessparams_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::DeviceAccessParams, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("Password") => {
            let path = {
                let js_val = Reflect::get(&obj, &"path".into())?;
                js_val.dyn_into::<JsString>().ok().and_then(|s| s.as_string()).ok_or_else(|| TypeError::new("Not a string")).and_then(|x| { (|s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) })(x).map_err(|e| TypeError::new(e)) }).map_err(|_| TypeError::new("Not a valid Path"))?
            };
            let password = {
                let js_val = Reflect::get(&obj, &"password".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            Ok(libparsec::DeviceAccessParams::Password { path, password })
        }
        tag if tag == JsValue::from_str("Smartcard") => {
            let path = {
                let js_val = Reflect::get(&obj, &"path".into())?;
                js_val.dyn_into::<JsString>().ok().and_then(|s| s.as_string()).ok_or_else(|| TypeError::new("Not a string")).and_then(|x| { (|s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) })(x).map_err(|e| TypeError::new(e)) }).map_err(|_| TypeError::new("Not a valid Path"))?
            };
            Ok(libparsec::DeviceAccessParams::Smartcard { path })
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a DeviceAccessParams",
        ))),
    }
}

#[allow(dead_code)]
fn variant_deviceaccessparams_rs_to_js(
    rs_obj: libparsec::DeviceAccessParams,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::DeviceAccessParams::Password { path, password } => {
            Reflect::set(&js_obj, &"tag".into(), &"Password".into())?;
            let js_path = JsValue::from_str(
                match (|path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                })(path)
                {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err))),
                }
                .as_ref(),
            );
            Reflect::set(&js_obj, &"path".into(), &js_path)?;
            let js_password = password.into();
            Reflect::set(&js_obj, &"password".into(), &js_password)?;
        }
        libparsec::DeviceAccessParams::Smartcard { path } => {
            Reflect::set(&js_obj, &"tag".into(), &"Smartcard".into())?;
            let js_path = JsValue::from_str(
                match (|path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                })(path)
                {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err))),
                }
                .as_ref(),
            );
            Reflect::set(&js_obj, &"path".into(), &js_path)?;
        }
    }
    Ok(js_obj)
}

// ClientLoginError

#[allow(dead_code)]
fn variant_clientloginerror_js_to_rs(obj: JsValue) -> Result<libparsec::ClientLoginError, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("AccessMethodNotAvailable") => {
            Ok(libparsec::ClientLoginError::AccessMethodNotAvailable {})
        }
        tag if tag == JsValue::from_str("DecryptionFailed") => {
            Ok(libparsec::ClientLoginError::DecryptionFailed {})
        }
        tag if tag == JsValue::from_str("DeviceAlreadyLoggedIn") => {
            Ok(libparsec::ClientLoginError::DeviceAlreadyLoggedIn {})
        }
        tag if tag == JsValue::from_str("DeviceInvalidFormat") => {
            Ok(libparsec::ClientLoginError::DeviceInvalidFormat {})
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a ClientLoginError",
        ))),
    }
}

#[allow(dead_code)]
fn variant_clientloginerror_rs_to_js(
    rs_obj: libparsec::ClientLoginError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::ClientLoginError::AccessMethodNotAvailable {} => {
            Reflect::set(&js_obj, &"tag".into(), &"AccessMethodNotAvailable".into())?;
        }
        libparsec::ClientLoginError::DecryptionFailed {} => {
            Reflect::set(&js_obj, &"tag".into(), &"DecryptionFailed".into())?;
        }
        libparsec::ClientLoginError::DeviceAlreadyLoggedIn {} => {
            Reflect::set(&js_obj, &"tag".into(), &"DeviceAlreadyLoggedIn".into())?;
        }
        libparsec::ClientLoginError::DeviceInvalidFormat {} => {
            Reflect::set(&js_obj, &"tag".into(), &"DeviceInvalidFormat".into())?;
        }
    }
    Ok(js_obj)
}

// client_login
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientLogin(
    load_device_params: Object,
    config: Object,
    on_event_callback: Function,
) -> Promise {
    future_to_promise(async move {
        let load_device_params = load_device_params.into();
        let load_device_params = variant_deviceaccessparams_js_to_rs(load_device_params)?;

        let config = config.into();
        let config = struct_clientconfig_js_to_rs(config)?;

        let on_event_callback = Box::new(move |event: libparsec::ClientEvent| {
            // TODO: Better error handling here (log error ?)
            let js_event =
                variant_clientevent_rs_to_js(event).expect("event type conversion error");
            on_event_callback
                .call1(&JsValue::NULL, &js_event)
                .expect("error in event callback");
        }) as Box<dyn FnMut(libparsec::ClientEvent)>;

        let ret = libparsec::client_login(load_device_params, config, on_event_callback).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from(i32::from(value));
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_clientloginerror_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

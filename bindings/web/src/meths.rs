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

// DeviceFileType

#[allow(dead_code)]
fn enum_device_file_type_js_to_rs(raw_value: &str) -> Result<libparsec::DeviceFileType, JsValue> {
    match raw_value {
        "DeviceFileTypePassword" => Ok(libparsec::DeviceFileType::Password),
        "DeviceFileTypeRecovery" => Ok(libparsec::DeviceFileType::Recovery),
        "DeviceFileTypeSmartcard" => Ok(libparsec::DeviceFileType::Smartcard),
        _ => {
            let range_error = RangeError::new("Invalid value for enum DeviceFileType");
            range_error.set_cause(&JsValue::from(raw_value));
            Err(JsValue::from(range_error))
        }
    }
}

#[allow(dead_code)]
fn enum_device_file_type_rs_to_js(value: libparsec::DeviceFileType) -> &'static str {
    match value {
        libparsec::DeviceFileType::Password => "DeviceFileTypePassword",
        libparsec::DeviceFileType::Recovery => "DeviceFileTypeRecovery",
        libparsec::DeviceFileType::Smartcard => "DeviceFileTypeSmartcard",
    }
}

// InvitationEmailSentStatus

#[allow(dead_code)]
fn enum_invitation_email_sent_status_js_to_rs(
    raw_value: &str,
) -> Result<libparsec::InvitationEmailSentStatus, JsValue> {
    match raw_value {
        "InvitationEmailSentStatusBadRecipient" => {
            Ok(libparsec::InvitationEmailSentStatus::BadRecipient)
        }
        "InvitationEmailSentStatusNotAvailable" => {
            Ok(libparsec::InvitationEmailSentStatus::NotAvailable)
        }
        "InvitationEmailSentStatusSuccess" => Ok(libparsec::InvitationEmailSentStatus::Success),
        _ => {
            let range_error = RangeError::new("Invalid value for enum InvitationEmailSentStatus");
            range_error.set_cause(&JsValue::from(raw_value));
            Err(JsValue::from(range_error))
        }
    }
}

#[allow(dead_code)]
fn enum_invitation_email_sent_status_rs_to_js(
    value: libparsec::InvitationEmailSentStatus,
) -> &'static str {
    match value {
        libparsec::InvitationEmailSentStatus::BadRecipient => {
            "InvitationEmailSentStatusBadRecipient"
        }
        libparsec::InvitationEmailSentStatus::NotAvailable => {
            "InvitationEmailSentStatusNotAvailable"
        }
        libparsec::InvitationEmailSentStatus::Success => "InvitationEmailSentStatusSuccess",
    }
}

// InvitationStatus

#[allow(dead_code)]
fn enum_invitation_status_js_to_rs(
    raw_value: &str,
) -> Result<libparsec::InvitationStatus, JsValue> {
    match raw_value {
        "InvitationStatusDeleted" => Ok(libparsec::InvitationStatus::Deleted),
        "InvitationStatusIdle" => Ok(libparsec::InvitationStatus::Idle),
        "InvitationStatusReady" => Ok(libparsec::InvitationStatus::Ready),
        _ => {
            let range_error = RangeError::new("Invalid value for enum InvitationStatus");
            range_error.set_cause(&JsValue::from(raw_value));
            Err(JsValue::from(range_error))
        }
    }
}

#[allow(dead_code)]
fn enum_invitation_status_rs_to_js(value: libparsec::InvitationStatus) -> &'static str {
    match value {
        libparsec::InvitationStatus::Deleted => "InvitationStatusDeleted",
        libparsec::InvitationStatus::Idle => "InvitationStatusIdle",
        libparsec::InvitationStatus::Ready => "InvitationStatusReady",
    }
}

// Platform

#[allow(dead_code)]
fn enum_platform_js_to_rs(raw_value: &str) -> Result<libparsec::Platform, JsValue> {
    match raw_value {
        "PlatformAndroid" => Ok(libparsec::Platform::Android),
        "PlatformLinux" => Ok(libparsec::Platform::Linux),
        "PlatformMacOS" => Ok(libparsec::Platform::MacOS),
        "PlatformWeb" => Ok(libparsec::Platform::Web),
        "PlatformWindows" => Ok(libparsec::Platform::Windows),
        _ => {
            let range_error = RangeError::new("Invalid value for enum Platform");
            range_error.set_cause(&JsValue::from(raw_value));
            Err(JsValue::from(range_error))
        }
    }
}

#[allow(dead_code)]
fn enum_platform_rs_to_js(value: libparsec::Platform) -> &'static str {
    match value {
        libparsec::Platform::Android => "PlatformAndroid",
        libparsec::Platform::Linux => "PlatformLinux",
        libparsec::Platform::MacOS => "PlatformMacOS",
        libparsec::Platform::Web => "PlatformWeb",
        libparsec::Platform::Windows => "PlatformWindows",
    }
}

// RealmRole

#[allow(dead_code)]
fn enum_realm_role_js_to_rs(raw_value: &str) -> Result<libparsec::RealmRole, JsValue> {
    match raw_value {
        "RealmRoleContributor" => Ok(libparsec::RealmRole::Contributor),
        "RealmRoleManager" => Ok(libparsec::RealmRole::Manager),
        "RealmRoleOwner" => Ok(libparsec::RealmRole::Owner),
        "RealmRoleReader" => Ok(libparsec::RealmRole::Reader),
        _ => {
            let range_error = RangeError::new("Invalid value for enum RealmRole");
            range_error.set_cause(&JsValue::from(raw_value));
            Err(JsValue::from(range_error))
        }
    }
}

#[allow(dead_code)]
fn enum_realm_role_rs_to_js(value: libparsec::RealmRole) -> &'static str {
    match value {
        libparsec::RealmRole::Contributor => "RealmRoleContributor",
        libparsec::RealmRole::Manager => "RealmRoleManager",
        libparsec::RealmRole::Owner => "RealmRoleOwner",
        libparsec::RealmRole::Reader => "RealmRoleReader",
    }
}

// UserProfile

#[allow(dead_code)]
fn enum_user_profile_js_to_rs(raw_value: &str) -> Result<libparsec::UserProfile, JsValue> {
    match raw_value {
        "UserProfileAdmin" => Ok(libparsec::UserProfile::Admin),
        "UserProfileOutsider" => Ok(libparsec::UserProfile::Outsider),
        "UserProfileStandard" => Ok(libparsec::UserProfile::Standard),
        _ => {
            let range_error = RangeError::new("Invalid value for enum UserProfile");
            range_error.set_cause(&JsValue::from(raw_value));
            Err(JsValue::from(range_error))
        }
    }
}

#[allow(dead_code)]
fn enum_user_profile_rs_to_js(value: libparsec::UserProfile) -> &'static str {
    match value {
        libparsec::UserProfile::Admin => "UserProfileAdmin",
        libparsec::UserProfile::Outsider => "UserProfileOutsider",
        libparsec::UserProfile::Standard => "UserProfileStandard",
    }
}

// AvailableDevice

#[allow(dead_code)]
fn struct_available_device_js_to_rs(obj: JsValue) -> Result<libparsec::AvailableDevice, JsValue> {
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
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    s.parse::<libparsec::DeviceID>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })
            .map_err(|_| TypeError::new("Not a valid DeviceID"))?
    };
    let human_handle = {
        let js_val = Reflect::get(&obj, &"humanHandle".into())?;
        if js_val.is_null() {
            None
        } else {
            Some(struct_human_handle_js_to_rs(js_val)?)
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
        {
            let raw_string = js_val.as_string().ok_or_else(|| {
                let type_error = TypeError::new("value is not a string");
                type_error.set_cause(&js_val);
                JsValue::from(type_error)
            })?;
            enum_device_file_type_js_to_rs(raw_string.as_str())
        }?
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
fn struct_available_device_rs_to_js(
    rs_obj: libparsec::AvailableDevice,
) -> Result<JsValue, JsValue> {
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
    let js_device_id = JsValue::from_str({
        let custom_to_rs_string = |device_id: libparsec::DeviceID| -> Result<_, &'static str> {
            Ok(device_id.to_string())
        };
        match custom_to_rs_string(rs_obj.device_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"deviceId".into(), &js_device_id)?;
    let js_human_handle = match rs_obj.human_handle {
        Some(val) => struct_human_handle_rs_to_js(val)?,
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
    let js_ty = JsValue::from_str(enum_device_file_type_rs_to_js(rs_obj.ty));
    Reflect::set(&js_obj, &"ty".into(), &js_ty)?;
    Ok(js_obj)
}

// ClientConfig

#[allow(dead_code)]
fn struct_client_config_js_to_rs(obj: JsValue) -> Result<libparsec::ClientConfig, JsValue> {
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
        variant_workspace_storage_cache_size_js_to_rs(js_val)?
    };
    Ok(libparsec::ClientConfig {
        config_dir,
        data_base_dir,
        mountpoint_base_dir,
        workspace_storage_cache_size,
    })
}

#[allow(dead_code)]
fn struct_client_config_rs_to_js(rs_obj: libparsec::ClientConfig) -> Result<JsValue, JsValue> {
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
        variant_workspace_storage_cache_size_rs_to_js(rs_obj.workspace_storage_cache_size)?;
    Reflect::set(
        &js_obj,
        &"workspaceStorageCacheSize".into(),
        &js_workspace_storage_cache_size,
    )?;
    Ok(js_obj)
}

// ClientInfo

#[allow(dead_code)]
fn struct_client_info_js_to_rs(obj: JsValue) -> Result<libparsec::ClientInfo, JsValue> {
    let organization_addr = {
        let js_val = Reflect::get(&obj, &"organizationAddr".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    libparsec::BackendOrganizationAddr::from_any(&s).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })
            .map_err(|_| TypeError::new("Not a valid BackendOrganizationAddr"))?
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
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    s.parse::<libparsec::DeviceID>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })
            .map_err(|_| TypeError::new("Not a valid DeviceID"))?
    };
    let user_id = {
        let js_val = Reflect::get(&obj, &"userId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
            .parse()
            .map_err(|_| TypeError::new("Not a valid UserID"))?
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
    let human_handle = {
        let js_val = Reflect::get(&obj, &"humanHandle".into())?;
        if js_val.is_null() {
            None
        } else {
            Some(struct_human_handle_js_to_rs(js_val)?)
        }
    };
    let current_profile = {
        let js_val = Reflect::get(&obj, &"currentProfile".into())?;
        {
            let raw_string = js_val.as_string().ok_or_else(|| {
                let type_error = TypeError::new("value is not a string");
                type_error.set_cause(&js_val);
                JsValue::from(type_error)
            })?;
            enum_user_profile_js_to_rs(raw_string.as_str())
        }?
    };
    Ok(libparsec::ClientInfo {
        organization_addr,
        organization_id,
        device_id,
        user_id,
        device_label,
        human_handle,
        current_profile,
    })
}

#[allow(dead_code)]
fn struct_client_info_rs_to_js(rs_obj: libparsec::ClientInfo) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_organization_addr = JsValue::from_str({
        let custom_to_rs_string =
            |addr: libparsec::BackendOrganizationAddr| -> Result<String, &'static str> {
                Ok(addr.to_url().into())
            };
        match custom_to_rs_string(rs_obj.organization_addr) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"organizationAddr".into(), &js_organization_addr)?;
    let js_organization_id = JsValue::from_str(rs_obj.organization_id.as_ref());
    Reflect::set(&js_obj, &"organizationId".into(), &js_organization_id)?;
    let js_device_id = JsValue::from_str({
        let custom_to_rs_string = |device_id: libparsec::DeviceID| -> Result<_, &'static str> {
            Ok(device_id.to_string())
        };
        match custom_to_rs_string(rs_obj.device_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"deviceId".into(), &js_device_id)?;
    let js_user_id = JsValue::from_str(rs_obj.user_id.as_ref());
    Reflect::set(&js_obj, &"userId".into(), &js_user_id)?;
    let js_device_label = match rs_obj.device_label {
        Some(val) => JsValue::from_str(val.as_ref()),
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"deviceLabel".into(), &js_device_label)?;
    let js_human_handle = match rs_obj.human_handle {
        Some(val) => struct_human_handle_rs_to_js(val)?,
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"humanHandle".into(), &js_human_handle)?;
    let js_current_profile = JsValue::from_str(enum_user_profile_rs_to_js(rs_obj.current_profile));
    Reflect::set(&js_obj, &"currentProfile".into(), &js_current_profile)?;
    Ok(js_obj)
}

// DeviceClaimFinalizeInfo

#[allow(dead_code)]
fn struct_device_claim_finalize_info_js_to_rs(
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
fn struct_device_claim_finalize_info_rs_to_js(
    rs_obj: libparsec::DeviceClaimFinalizeInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// DeviceClaimInProgress1Info

#[allow(dead_code)]
fn struct_device_claim_in_progress1_info_js_to_rs(
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
fn struct_device_claim_in_progress1_info_rs_to_js(
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

// DeviceClaimInProgress2Info

#[allow(dead_code)]
fn struct_device_claim_in_progress2_info_js_to_rs(
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
fn struct_device_claim_in_progress2_info_rs_to_js(
    rs_obj: libparsec::DeviceClaimInProgress2Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_claimer_sas = JsValue::from_str(rs_obj.claimer_sas.as_ref());
    Reflect::set(&js_obj, &"claimerSas".into(), &js_claimer_sas)?;
    Ok(js_obj)
}

// DeviceClaimInProgress3Info

#[allow(dead_code)]
fn struct_device_claim_in_progress3_info_js_to_rs(
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
fn struct_device_claim_in_progress3_info_rs_to_js(
    rs_obj: libparsec::DeviceClaimInProgress3Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// DeviceGreetInProgress1Info

#[allow(dead_code)]
fn struct_device_greet_in_progress1_info_js_to_rs(
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
fn struct_device_greet_in_progress1_info_rs_to_js(
    rs_obj: libparsec::DeviceGreetInProgress1Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_greeter_sas = JsValue::from_str(rs_obj.greeter_sas.as_ref());
    Reflect::set(&js_obj, &"greeterSas".into(), &js_greeter_sas)?;
    Ok(js_obj)
}

// DeviceGreetInProgress2Info

#[allow(dead_code)]
fn struct_device_greet_in_progress2_info_js_to_rs(
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
fn struct_device_greet_in_progress2_info_rs_to_js(
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

// DeviceGreetInProgress3Info

#[allow(dead_code)]
fn struct_device_greet_in_progress3_info_js_to_rs(
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
fn struct_device_greet_in_progress3_info_rs_to_js(
    rs_obj: libparsec::DeviceGreetInProgress3Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// DeviceGreetInProgress4Info

#[allow(dead_code)]
fn struct_device_greet_in_progress4_info_js_to_rs(
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
fn struct_device_greet_in_progress4_info_rs_to_js(
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

// DeviceGreetInitialInfo

#[allow(dead_code)]
fn struct_device_greet_initial_info_js_to_rs(
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
fn struct_device_greet_initial_info_rs_to_js(
    rs_obj: libparsec::DeviceGreetInitialInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// DeviceInfo

#[allow(dead_code)]
fn struct_device_info_js_to_rs(obj: JsValue) -> Result<libparsec::DeviceInfo, JsValue> {
    let id = {
        let js_val = Reflect::get(&obj, &"id".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    s.parse::<libparsec::DeviceID>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })
            .map_err(|_| TypeError::new("Not a valid DeviceID"))?
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
    let created_on = {
        let js_val = Reflect::get(&obj, &"createdOn".into())?;
        {
            let v = js_val.dyn_into::<Number>()?.value_of();
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                Ok(libparsec::DateTime::from_f64_with_us_precision(n))
            };
            let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
            v
        }
    };
    let created_by = {
        let js_val = Reflect::get(&obj, &"createdBy".into())?;
        if js_val.is_null() {
            None
        } else {
            Some(
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            s.parse::<libparsec::DeviceID>().map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid DeviceID"))?,
            )
        }
    };
    Ok(libparsec::DeviceInfo {
        id,
        device_label,
        created_on,
        created_by,
    })
}

#[allow(dead_code)]
fn struct_device_info_rs_to_js(rs_obj: libparsec::DeviceInfo) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_id = JsValue::from_str({
        let custom_to_rs_string = |device_id: libparsec::DeviceID| -> Result<_, &'static str> {
            Ok(device_id.to_string())
        };
        match custom_to_rs_string(rs_obj.id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"id".into(), &js_id)?;
    let js_device_label = match rs_obj.device_label {
        Some(val) => JsValue::from_str(val.as_ref()),
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"deviceLabel".into(), &js_device_label)?;
    let js_created_on = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok(dt.get_f64_with_us_precision())
        };
        let v = match custom_to_rs_f64(rs_obj.created_on) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        };
        JsValue::from(v)
    };
    Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
    let js_created_by = match rs_obj.created_by {
        Some(val) => JsValue::from_str({
            let custom_to_rs_string = |device_id: libparsec::DeviceID| -> Result<_, &'static str> {
                Ok(device_id.to_string())
            };
            match custom_to_rs_string(val) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
            }
            .as_ref()
        }),
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
    Ok(js_obj)
}

// HumanHandle

#[allow(dead_code)]
fn struct_human_handle_js_to_rs(obj: JsValue) -> Result<libparsec::HumanHandle, JsValue> {
    let email = {
        let js_val = Reflect::get(&obj, &"email".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
    };
    let label = {
        let js_val = Reflect::get(&obj, &"label".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
    };

    |email: String, label: String| -> Result<_, String> {
        libparsec::HumanHandle::new(&email, &label).map_err(|e| e.to_string())
    }(email, label)
    .map_err(|e| e.into())
}

#[allow(dead_code)]
fn struct_human_handle_rs_to_js(rs_obj: libparsec::HumanHandle) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_email = {
        let custom_getter = |obj| {
            fn access(obj: &libparsec::HumanHandle) -> &str {
                obj.email()
            }
            access(obj)
        };
        custom_getter(&rs_obj).into()
    };
    Reflect::set(&js_obj, &"email".into(), &js_email)?;
    let js_label = {
        let custom_getter = |obj| {
            fn access(obj: &libparsec::HumanHandle) -> &str {
                obj.label()
            }
            access(obj)
        };
        custom_getter(&rs_obj).into()
    };
    Reflect::set(&js_obj, &"label".into(), &js_label)?;
    Ok(js_obj)
}

// NewInvitationInfo

#[allow(dead_code)]
fn struct_new_invitation_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::NewInvitationInfo, JsValue> {
    let addr = {
        let js_val = Reflect::get(&obj, &"addr".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    libparsec::BackendInvitationAddr::from_any(&s).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })
            .map_err(|_| TypeError::new("Not a valid BackendInvitationAddr"))?
    };
    let token = {
        let js_val = Reflect::get(&obj, &"token".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::InvitationToken, _> {
                    libparsec::InvitationToken::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })
            .map_err(|_| TypeError::new("Not a valid InvitationToken"))?
    };
    let email_sent_status = {
        let js_val = Reflect::get(&obj, &"emailSentStatus".into())?;
        {
            let raw_string = js_val.as_string().ok_or_else(|| {
                let type_error = TypeError::new("value is not a string");
                type_error.set_cause(&js_val);
                JsValue::from(type_error)
            })?;
            enum_invitation_email_sent_status_js_to_rs(raw_string.as_str())
        }?
    };
    Ok(libparsec::NewInvitationInfo {
        addr,
        token,
        email_sent_status,
    })
}

#[allow(dead_code)]
fn struct_new_invitation_info_rs_to_js(
    rs_obj: libparsec::NewInvitationInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_addr = JsValue::from_str({
        let custom_to_rs_string =
            |addr: libparsec::BackendInvitationAddr| -> Result<String, &'static str> {
                Ok(addr.to_url().into())
            };
        match custom_to_rs_string(rs_obj.addr) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"addr".into(), &js_addr)?;
    let js_token = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.token) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"token".into(), &js_token)?;
    let js_email_sent_status = JsValue::from_str(enum_invitation_email_sent_status_rs_to_js(
        rs_obj.email_sent_status,
    ));
    Reflect::set(&js_obj, &"emailSentStatus".into(), &js_email_sent_status)?;
    Ok(js_obj)
}

// UserClaimFinalizeInfo

#[allow(dead_code)]
fn struct_user_claim_finalize_info_js_to_rs(
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
fn struct_user_claim_finalize_info_rs_to_js(
    rs_obj: libparsec::UserClaimFinalizeInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// UserClaimInProgress1Info

#[allow(dead_code)]
fn struct_user_claim_in_progress1_info_js_to_rs(
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
fn struct_user_claim_in_progress1_info_rs_to_js(
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

// UserClaimInProgress2Info

#[allow(dead_code)]
fn struct_user_claim_in_progress2_info_js_to_rs(
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
fn struct_user_claim_in_progress2_info_rs_to_js(
    rs_obj: libparsec::UserClaimInProgress2Info,
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
fn struct_user_claim_in_progress3_info_js_to_rs(
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
fn struct_user_claim_in_progress3_info_rs_to_js(
    rs_obj: libparsec::UserClaimInProgress3Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// UserGreetInProgress1Info

#[allow(dead_code)]
fn struct_user_greet_in_progress1_info_js_to_rs(
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
fn struct_user_greet_in_progress1_info_rs_to_js(
    rs_obj: libparsec::UserGreetInProgress1Info,
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
fn struct_user_greet_in_progress2_info_js_to_rs(
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
fn struct_user_greet_in_progress2_info_rs_to_js(
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

// UserGreetInProgress3Info

#[allow(dead_code)]
fn struct_user_greet_in_progress3_info_js_to_rs(
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
fn struct_user_greet_in_progress3_info_rs_to_js(
    rs_obj: libparsec::UserGreetInProgress3Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// UserGreetInProgress4Info

#[allow(dead_code)]
fn struct_user_greet_in_progress4_info_js_to_rs(
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
            Some(struct_human_handle_js_to_rs(js_val)?)
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
fn struct_user_greet_in_progress4_info_rs_to_js(
    rs_obj: libparsec::UserGreetInProgress4Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_requested_human_handle = match rs_obj.requested_human_handle {
        Some(val) => struct_human_handle_rs_to_js(val)?,
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

// UserGreetInitialInfo

#[allow(dead_code)]
fn struct_user_greet_initial_info_js_to_rs(
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
fn struct_user_greet_initial_info_rs_to_js(
    rs_obj: libparsec::UserGreetInitialInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// UserInfo

#[allow(dead_code)]
fn struct_user_info_js_to_rs(obj: JsValue) -> Result<libparsec::UserInfo, JsValue> {
    let id = {
        let js_val = Reflect::get(&obj, &"id".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
            .parse()
            .map_err(|_| TypeError::new("Not a valid UserID"))?
    };
    let human_handle = {
        let js_val = Reflect::get(&obj, &"humanHandle".into())?;
        if js_val.is_null() {
            None
        } else {
            Some(struct_human_handle_js_to_rs(js_val)?)
        }
    };
    let current_profile = {
        let js_val = Reflect::get(&obj, &"currentProfile".into())?;
        {
            let raw_string = js_val.as_string().ok_or_else(|| {
                let type_error = TypeError::new("value is not a string");
                type_error.set_cause(&js_val);
                JsValue::from(type_error)
            })?;
            enum_user_profile_js_to_rs(raw_string.as_str())
        }?
    };
    let created_on = {
        let js_val = Reflect::get(&obj, &"createdOn".into())?;
        {
            let v = js_val.dyn_into::<Number>()?.value_of();
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                Ok(libparsec::DateTime::from_f64_with_us_precision(n))
            };
            let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
            v
        }
    };
    let created_by = {
        let js_val = Reflect::get(&obj, &"createdBy".into())?;
        if js_val.is_null() {
            None
        } else {
            Some(
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            s.parse::<libparsec::DeviceID>().map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid DeviceID"))?,
            )
        }
    };
    let revoked_on = {
        let js_val = Reflect::get(&obj, &"revokedOn".into())?;
        if js_val.is_null() {
            None
        } else {
            Some({
                let v = js_val.dyn_into::<Number>()?.value_of();
                let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                    Ok(libparsec::DateTime::from_f64_with_us_precision(n))
                };
                let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                v
            })
        }
    };
    let revoked_by = {
        let js_val = Reflect::get(&obj, &"revokedBy".into())?;
        if js_val.is_null() {
            None
        } else {
            Some(
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            s.parse::<libparsec::DeviceID>().map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid DeviceID"))?,
            )
        }
    };
    Ok(libparsec::UserInfo {
        id,
        human_handle,
        current_profile,
        created_on,
        created_by,
        revoked_on,
        revoked_by,
    })
}

#[allow(dead_code)]
fn struct_user_info_rs_to_js(rs_obj: libparsec::UserInfo) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_id = JsValue::from_str(rs_obj.id.as_ref());
    Reflect::set(&js_obj, &"id".into(), &js_id)?;
    let js_human_handle = match rs_obj.human_handle {
        Some(val) => struct_human_handle_rs_to_js(val)?,
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"humanHandle".into(), &js_human_handle)?;
    let js_current_profile = JsValue::from_str(enum_user_profile_rs_to_js(rs_obj.current_profile));
    Reflect::set(&js_obj, &"currentProfile".into(), &js_current_profile)?;
    let js_created_on = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok(dt.get_f64_with_us_precision())
        };
        let v = match custom_to_rs_f64(rs_obj.created_on) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        };
        JsValue::from(v)
    };
    Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
    let js_created_by = match rs_obj.created_by {
        Some(val) => JsValue::from_str({
            let custom_to_rs_string = |device_id: libparsec::DeviceID| -> Result<_, &'static str> {
                Ok(device_id.to_string())
            };
            match custom_to_rs_string(val) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
            }
            .as_ref()
        }),
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
    let js_revoked_on = match rs_obj.revoked_on {
        Some(val) => {
            let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                Ok(dt.get_f64_with_us_precision())
            };
            let v = match custom_to_rs_f64(val) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
            };
            JsValue::from(v)
        }
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"revokedOn".into(), &js_revoked_on)?;
    let js_revoked_by = match rs_obj.revoked_by {
        Some(val) => JsValue::from_str({
            let custom_to_rs_string = |device_id: libparsec::DeviceID| -> Result<_, &'static str> {
                Ok(device_id.to_string())
            };
            match custom_to_rs_string(val) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
            }
            .as_ref()
        }),
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"revokedBy".into(), &js_revoked_by)?;
    Ok(js_obj)
}

// WorkspaceInfo

#[allow(dead_code)]
fn struct_workspace_info_js_to_rs(obj: JsValue) -> Result<libparsec::WorkspaceInfo, JsValue> {
    let id = {
        let js_val = Reflect::get(&obj, &"id".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                    libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })
            .map_err(|_| TypeError::new("Not a valid VlobID"))?
    };
    let name = {
        let js_val = Reflect::get(&obj, &"name".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, _> {
                    s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })
            .map_err(|_| TypeError::new("Not a valid EntryName"))?
    };
    let self_role = {
        let js_val = Reflect::get(&obj, &"selfRole".into())?;
        {
            let raw_string = js_val.as_string().ok_or_else(|| {
                let type_error = TypeError::new("value is not a string");
                type_error.set_cause(&js_val);
                JsValue::from(type_error)
            })?;
            enum_realm_role_js_to_rs(raw_string.as_str())
        }?
    };
    Ok(libparsec::WorkspaceInfo {
        id,
        name,
        self_role,
    })
}

#[allow(dead_code)]
fn struct_workspace_info_rs_to_js(rs_obj: libparsec::WorkspaceInfo) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"id".into(), &js_id)?;
    let js_name = JsValue::from_str(rs_obj.name.as_ref());
    Reflect::set(&js_obj, &"name".into(), &js_name)?;
    let js_self_role = JsValue::from_str(enum_realm_role_rs_to_js(rs_obj.self_role));
    Reflect::set(&js_obj, &"selfRole".into(), &js_self_role)?;
    Ok(js_obj)
}

// WorkspaceUserAccessInfo

#[allow(dead_code)]
fn struct_workspace_user_access_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::WorkspaceUserAccessInfo, JsValue> {
    let user_id = {
        let js_val = Reflect::get(&obj, &"userId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
            .parse()
            .map_err(|_| TypeError::new("Not a valid UserID"))?
    };
    let human_handle = {
        let js_val = Reflect::get(&obj, &"humanHandle".into())?;
        if js_val.is_null() {
            None
        } else {
            Some(struct_human_handle_js_to_rs(js_val)?)
        }
    };
    let role = {
        let js_val = Reflect::get(&obj, &"role".into())?;
        {
            let raw_string = js_val.as_string().ok_or_else(|| {
                let type_error = TypeError::new("value is not a string");
                type_error.set_cause(&js_val);
                JsValue::from(type_error)
            })?;
            enum_realm_role_js_to_rs(raw_string.as_str())
        }?
    };
    Ok(libparsec::WorkspaceUserAccessInfo {
        user_id,
        human_handle,
        role,
    })
}

#[allow(dead_code)]
fn struct_workspace_user_access_info_rs_to_js(
    rs_obj: libparsec::WorkspaceUserAccessInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_user_id = JsValue::from_str(rs_obj.user_id.as_ref());
    Reflect::set(&js_obj, &"userId".into(), &js_user_id)?;
    let js_human_handle = match rs_obj.human_handle {
        Some(val) => struct_human_handle_rs_to_js(val)?,
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"humanHandle".into(), &js_human_handle)?;
    let js_role = JsValue::from_str(enum_realm_role_rs_to_js(rs_obj.role));
    Reflect::set(&js_obj, &"role".into(), &js_role)?;
    Ok(js_obj)
}

// BootstrapOrganizationError

#[allow(dead_code)]
fn variant_bootstrap_organization_error_rs_to_js(
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
            let js_server_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok(dt.get_f64_with_us_precision())
                };
                let v = match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"serverTimestamp".into(), &js_server_timestamp)?;
            let js_client_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok(dt.get_f64_with_us_precision())
                };
                let v = match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
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

// CancelError

#[allow(dead_code)]
fn variant_cancel_error_rs_to_js(rs_obj: libparsec::CancelError) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::CancelError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::CancelError::NotBound { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"NotBound".into())?;
        }
    }
    Ok(js_obj)
}

// ClaimInProgressError

#[allow(dead_code)]
fn variant_claim_in_progress_error_rs_to_js(
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

// ClaimerGreeterAbortOperationError

#[allow(dead_code)]
fn variant_claimer_greeter_abort_operation_error_rs_to_js(
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

// ClaimerRetrieveInfoError

#[allow(dead_code)]
fn variant_claimer_retrieve_info_error_rs_to_js(
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

// ClientCreateWorkspaceError

#[allow(dead_code)]
fn variant_client_create_workspace_error_rs_to_js(
    rs_obj: libparsec::ClientCreateWorkspaceError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientCreateWorkspaceError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
    }
    Ok(js_obj)
}

// ClientEvent

#[allow(dead_code)]
fn variant_client_event_js_to_rs(obj: JsValue) -> Result<libparsec::ClientEvent, JsValue> {
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
fn variant_client_event_rs_to_js(rs_obj: libparsec::ClientEvent) -> Result<JsValue, JsValue> {
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

// ClientGetUserDeviceError

#[allow(dead_code)]
fn variant_client_get_user_device_error_rs_to_js(
    rs_obj: libparsec::ClientGetUserDeviceError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientGetUserDeviceError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::ClientGetUserDeviceError::NonExisting { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"NonExisting".into())?;
        }
    }
    Ok(js_obj)
}

// ClientInfoError

#[allow(dead_code)]
fn variant_client_info_error_rs_to_js(
    rs_obj: libparsec::ClientInfoError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientInfoError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
    }
    Ok(js_obj)
}

// ClientListUserDevicesError

#[allow(dead_code)]
fn variant_client_list_user_devices_error_rs_to_js(
    rs_obj: libparsec::ClientListUserDevicesError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientListUserDevicesError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
    }
    Ok(js_obj)
}

// ClientListUsersError

#[allow(dead_code)]
fn variant_client_list_users_error_rs_to_js(
    rs_obj: libparsec::ClientListUsersError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientListUsersError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
    }
    Ok(js_obj)
}

// ClientListWorkspaceUsersError

#[allow(dead_code)]
fn variant_client_list_workspace_users_error_rs_to_js(
    rs_obj: libparsec::ClientListWorkspaceUsersError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientListWorkspaceUsersError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
    }
    Ok(js_obj)
}

// ClientListWorkspacesError

#[allow(dead_code)]
fn variant_client_list_workspaces_error_rs_to_js(
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

// ClientRenameWorkspaceError

#[allow(dead_code)]
fn variant_client_rename_workspace_error_rs_to_js(
    rs_obj: libparsec::ClientRenameWorkspaceError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientRenameWorkspaceError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::ClientRenameWorkspaceError::UnknownWorkspace { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"UnknownWorkspace".into())?;
        }
    }
    Ok(js_obj)
}

// ClientShareWorkspaceError

#[allow(dead_code)]
fn variant_client_share_workspace_error_rs_to_js(
    rs_obj: libparsec::ClientShareWorkspaceError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientShareWorkspaceError::BadTimestamp {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"BadTimestamp".into())?;
            let js_server_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok(dt.get_f64_with_us_precision())
                };
                let v = match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"serverTimestamp".into(), &js_server_timestamp)?;
            let js_client_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok(dt.get_f64_with_us_precision())
                };
                let v = match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
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
        libparsec::ClientShareWorkspaceError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::ClientShareWorkspaceError::NotAllowed { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"NotAllowed".into())?;
        }
        libparsec::ClientShareWorkspaceError::Offline { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Offline".into())?;
        }
        libparsec::ClientShareWorkspaceError::OutsiderCannotBeManagerOrOwner { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"OutsiderCannotBeManagerOrOwner".into(),
            )?;
        }
        libparsec::ClientShareWorkspaceError::RevokedRecipient { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"RevokedRecipient".into())?;
        }
        libparsec::ClientShareWorkspaceError::ShareToSelf { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ShareToSelf".into())?;
        }
        libparsec::ClientShareWorkspaceError::UnknownRecipient { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"UnknownRecipient".into())?;
        }
        libparsec::ClientShareWorkspaceError::UnknownRecipientOrWorkspace { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"UnknownRecipientOrWorkspace".into(),
            )?;
        }
        libparsec::ClientShareWorkspaceError::UnknownWorkspace { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"UnknownWorkspace".into())?;
        }
        libparsec::ClientShareWorkspaceError::WorkspaceInMaintenance { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"WorkspaceInMaintenance".into())?;
        }
    }
    Ok(js_obj)
}

// ClientStartError

#[allow(dead_code)]
fn variant_client_start_error_rs_to_js(
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

// ClientStartInvitationGreetError

#[allow(dead_code)]
fn variant_client_start_invitation_greet_error_rs_to_js(
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

// ClientStartWorkspaceError

#[allow(dead_code)]
fn variant_client_start_workspace_error_rs_to_js(
    rs_obj: libparsec::ClientStartWorkspaceError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientStartWorkspaceError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::ClientStartWorkspaceError::NoAccess { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"NoAccess".into())?;
        }
    }
    Ok(js_obj)
}

// ClientStopError

#[allow(dead_code)]
fn variant_client_stop_error_rs_to_js(
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

// DeleteInvitationError

#[allow(dead_code)]
fn variant_delete_invitation_error_rs_to_js(
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

// DeviceAccessStrategy

#[allow(dead_code)]
fn variant_device_access_strategy_js_to_rs(
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
fn variant_device_access_strategy_rs_to_js(
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

// DeviceSaveStrategy

#[allow(dead_code)]
fn variant_device_save_strategy_js_to_rs(
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
fn variant_device_save_strategy_rs_to_js(
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

// EntryStat

#[allow(dead_code)]
fn variant_entry_stat_js_to_rs(obj: JsValue) -> Result<libparsec::EntryStat, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("File") => {
            let confinement_point = {
                let js_val = Reflect::get(&obj, &"confinementPoint".into())?;
                if js_val.is_null() {
                    None
                } else {
                    Some(
                        js_val
                            .dyn_into::<JsString>()
                            .ok()
                            .and_then(|s| s.as_string())
                            .ok_or_else(|| TypeError::new("Not a string"))
                            .and_then(|x| {
                                let custom_from_rs_string =
                                    |s: String| -> Result<libparsec::VlobID, _> {
                                        libparsec::VlobID::from_hex(s.as_str())
                                            .map_err(|e| e.to_string())
                                    };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })
                            .map_err(|_| TypeError::new("Not a valid VlobID"))?,
                    )
                }
            };
            let id = {
                let js_val = Reflect::get(&obj, &"id".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                            libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid VlobID"))?
            };
            let created = {
                let js_val = Reflect::get(&obj, &"created".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        Ok(libparsec::DateTime::from_f64_with_us_precision(n))
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let updated = {
                let js_val = Reflect::get(&obj, &"updated".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        Ok(libparsec::DateTime::from_f64_with_us_precision(n))
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let base_version = {
                let js_val = Reflect::get(&obj, &"baseVersion".into())?;
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
            let is_placeholder = {
                let js_val = Reflect::get(&obj, &"isPlaceholder".into())?;
                js_val
                    .dyn_into::<Boolean>()
                    .map_err(|_| TypeError::new("Not a boolean"))?
                    .value_of()
            };
            let need_sync = {
                let js_val = Reflect::get(&obj, &"needSync".into())?;
                js_val
                    .dyn_into::<Boolean>()
                    .map_err(|_| TypeError::new("Not a boolean"))?
                    .value_of()
            };
            let size = {
                let js_val = Reflect::get(&obj, &"size".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u64 number")));
                    }
                    v as u64
                }
            };
            Ok(libparsec::EntryStat::File {
                confinement_point,
                id,
                created,
                updated,
                base_version,
                is_placeholder,
                need_sync,
                size,
            })
        }
        tag if tag == JsValue::from_str("Folder") => {
            let confinement_point = {
                let js_val = Reflect::get(&obj, &"confinementPoint".into())?;
                if js_val.is_null() {
                    None
                } else {
                    Some(
                        js_val
                            .dyn_into::<JsString>()
                            .ok()
                            .and_then(|s| s.as_string())
                            .ok_or_else(|| TypeError::new("Not a string"))
                            .and_then(|x| {
                                let custom_from_rs_string =
                                    |s: String| -> Result<libparsec::VlobID, _> {
                                        libparsec::VlobID::from_hex(s.as_str())
                                            .map_err(|e| e.to_string())
                                    };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })
                            .map_err(|_| TypeError::new("Not a valid VlobID"))?,
                    )
                }
            };
            let id = {
                let js_val = Reflect::get(&obj, &"id".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                            libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid VlobID"))?
            };
            let created = {
                let js_val = Reflect::get(&obj, &"created".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        Ok(libparsec::DateTime::from_f64_with_us_precision(n))
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let updated = {
                let js_val = Reflect::get(&obj, &"updated".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        Ok(libparsec::DateTime::from_f64_with_us_precision(n))
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let base_version = {
                let js_val = Reflect::get(&obj, &"baseVersion".into())?;
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
            let is_placeholder = {
                let js_val = Reflect::get(&obj, &"isPlaceholder".into())?;
                js_val
                    .dyn_into::<Boolean>()
                    .map_err(|_| TypeError::new("Not a boolean"))?
                    .value_of()
            };
            let need_sync = {
                let js_val = Reflect::get(&obj, &"needSync".into())?;
                js_val
                    .dyn_into::<Boolean>()
                    .map_err(|_| TypeError::new("Not a boolean"))?
                    .value_of()
            };
            let children = {
                let js_val = Reflect::get(&obj, &"children".into())?;
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
                            .ok_or_else(|| TypeError::new("Not a string"))
                            .and_then(|x| {
                                let custom_from_rs_string = |s: String| -> Result<_, _> {
                                    s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
                                };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })
                            .map_err(|_| TypeError::new("Not a valid EntryName"))?;
                        converted.push(x_converted);
                    }
                    converted
                }
            };
            Ok(libparsec::EntryStat::Folder {
                confinement_point,
                id,
                created,
                updated,
                base_version,
                is_placeholder,
                need_sync,
                children,
            })
        }
        _ => Err(JsValue::from(TypeError::new("Object is not a EntryStat"))),
    }
}

#[allow(dead_code)]
fn variant_entry_stat_rs_to_js(rs_obj: libparsec::EntryStat) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::EntryStat::File {
            confinement_point,
            id,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            size,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"File".into())?;
            let js_confinement_point = match confinement_point {
                Some(val) => JsValue::from_str({
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(val) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                    .as_ref()
                }),
                None => JsValue::NULL,
            };
            Reflect::set(&js_obj, &"confinementPoint".into(), &js_confinement_point)?;
            let js_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"id".into(), &js_id)?;
            let js_created = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok(dt.get_f64_with_us_precision())
                };
                let v = match custom_to_rs_f64(created) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"created".into(), &js_created)?;
            let js_updated = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok(dt.get_f64_with_us_precision())
                };
                let v = match custom_to_rs_f64(updated) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"updated".into(), &js_updated)?;
            let js_base_version = JsValue::from(base_version);
            Reflect::set(&js_obj, &"baseVersion".into(), &js_base_version)?;
            let js_is_placeholder = is_placeholder.into();
            Reflect::set(&js_obj, &"isPlaceholder".into(), &js_is_placeholder)?;
            let js_need_sync = need_sync.into();
            Reflect::set(&js_obj, &"needSync".into(), &js_need_sync)?;
            let js_size = JsValue::from(size);
            Reflect::set(&js_obj, &"size".into(), &js_size)?;
        }
        libparsec::EntryStat::Folder {
            confinement_point,
            id,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            children,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"Folder".into())?;
            let js_confinement_point = match confinement_point {
                Some(val) => JsValue::from_str({
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(val) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                    .as_ref()
                }),
                None => JsValue::NULL,
            };
            Reflect::set(&js_obj, &"confinementPoint".into(), &js_confinement_point)?;
            let js_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"id".into(), &js_id)?;
            let js_created = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok(dt.get_f64_with_us_precision())
                };
                let v = match custom_to_rs_f64(created) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"created".into(), &js_created)?;
            let js_updated = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok(dt.get_f64_with_us_precision())
                };
                let v = match custom_to_rs_f64(updated) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"updated".into(), &js_updated)?;
            let js_base_version = JsValue::from(base_version);
            Reflect::set(&js_obj, &"baseVersion".into(), &js_base_version)?;
            let js_is_placeholder = is_placeholder.into();
            Reflect::set(&js_obj, &"isPlaceholder".into(), &js_is_placeholder)?;
            let js_need_sync = need_sync.into();
            Reflect::set(&js_obj, &"needSync".into(), &js_need_sync)?;
            let js_children = {
                // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                let js_array = Array::new_with_length(children.len() as u32);
                for (i, elem) in children.into_iter().enumerate() {
                    let js_elem = JsValue::from_str(elem.as_ref());
                    js_array.set(i as u32, js_elem);
                }
                js_array.into()
            };
            Reflect::set(&js_obj, &"children".into(), &js_children)?;
        }
    }
    Ok(js_obj)
}

// GreetInProgressError

#[allow(dead_code)]
fn variant_greet_in_progress_error_rs_to_js(
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
            let js_server_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok(dt.get_f64_with_us_precision())
                };
                let v = match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"serverTimestamp".into(), &js_server_timestamp)?;
            let js_client_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok(dt.get_f64_with_us_precision())
                };
                let v = match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
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

// InviteListItem

#[allow(dead_code)]
fn variant_invite_list_item_js_to_rs(obj: JsValue) -> Result<libparsec::InviteListItem, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("Device") => {
            let addr = {
                let js_val = Reflect::get(&obj, &"addr".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::BackendInvitationAddr::from_any(&s)
                                .map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid BackendInvitationAddr"))?
            };
            let token = {
                let js_val = Reflect::get(&obj, &"token".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string =
                            |s: String| -> Result<libparsec::InvitationToken, _> {
                                libparsec::InvitationToken::from_hex(s.as_str())
                                    .map_err(|e| e.to_string())
                            };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid InvitationToken"))?
            };
            let created_on = {
                let js_val = Reflect::get(&obj, &"createdOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        Ok(libparsec::DateTime::from_f64_with_us_precision(n))
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let status = {
                let js_val = Reflect::get(&obj, &"status".into())?;
                {
                    let raw_string = js_val.as_string().ok_or_else(|| {
                        let type_error = TypeError::new("value is not a string");
                        type_error.set_cause(&js_val);
                        JsValue::from(type_error)
                    })?;
                    enum_invitation_status_js_to_rs(raw_string.as_str())
                }?
            };
            Ok(libparsec::InviteListItem::Device {
                addr,
                token,
                created_on,
                status,
            })
        }
        tag if tag == JsValue::from_str("User") => {
            let addr = {
                let js_val = Reflect::get(&obj, &"addr".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::BackendInvitationAddr::from_any(&s)
                                .map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid BackendInvitationAddr"))?
            };
            let token = {
                let js_val = Reflect::get(&obj, &"token".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string =
                            |s: String| -> Result<libparsec::InvitationToken, _> {
                                libparsec::InvitationToken::from_hex(s.as_str())
                                    .map_err(|e| e.to_string())
                            };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid InvitationToken"))?
            };
            let created_on = {
                let js_val = Reflect::get(&obj, &"createdOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        Ok(libparsec::DateTime::from_f64_with_us_precision(n))
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
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
            let status = {
                let js_val = Reflect::get(&obj, &"status".into())?;
                {
                    let raw_string = js_val.as_string().ok_or_else(|| {
                        let type_error = TypeError::new("value is not a string");
                        type_error.set_cause(&js_val);
                        JsValue::from(type_error)
                    })?;
                    enum_invitation_status_js_to_rs(raw_string.as_str())
                }?
            };
            Ok(libparsec::InviteListItem::User {
                addr,
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
fn variant_invite_list_item_rs_to_js(
    rs_obj: libparsec::InviteListItem,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::InviteListItem::Device {
            addr,
            token,
            created_on,
            status,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"Device".into())?;
            let js_addr = JsValue::from_str({
                let custom_to_rs_string =
                    |addr: libparsec::BackendInvitationAddr| -> Result<String, &'static str> {
                        Ok(addr.to_url().into())
                    };
                match custom_to_rs_string(addr) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"addr".into(), &js_addr)?;
            let js_token = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
            let js_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok(dt.get_f64_with_us_precision())
                };
                let v = match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
            let js_status = JsValue::from_str(enum_invitation_status_rs_to_js(status));
            Reflect::set(&js_obj, &"status".into(), &js_status)?;
        }
        libparsec::InviteListItem::User {
            addr,
            token,
            created_on,
            claimer_email,
            status,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"User".into())?;
            let js_addr = JsValue::from_str({
                let custom_to_rs_string =
                    |addr: libparsec::BackendInvitationAddr| -> Result<String, &'static str> {
                        Ok(addr.to_url().into())
                    };
                match custom_to_rs_string(addr) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"addr".into(), &js_addr)?;
            let js_token = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
            let js_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok(dt.get_f64_with_us_precision())
                };
                let v = match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
            let js_claimer_email = claimer_email.into();
            Reflect::set(&js_obj, &"claimerEmail".into(), &js_claimer_email)?;
            let js_status = JsValue::from_str(enum_invitation_status_rs_to_js(status));
            Reflect::set(&js_obj, &"status".into(), &js_status)?;
        }
    }
    Ok(js_obj)
}

// ListInvitationsError

#[allow(dead_code)]
fn variant_list_invitations_error_rs_to_js(
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

// NewDeviceInvitationError

#[allow(dead_code)]
fn variant_new_device_invitation_error_rs_to_js(
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

// NewUserInvitationError

#[allow(dead_code)]
fn variant_new_user_invitation_error_rs_to_js(
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

// ParseBackendAddrError

#[allow(dead_code)]
fn variant_parse_backend_addr_error_rs_to_js(
    rs_obj: libparsec::ParseBackendAddrError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ParseBackendAddrError::InvalidUrl { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"InvalidUrl".into())?;
        }
    }
    Ok(js_obj)
}

// ParsedBackendAddr

#[allow(dead_code)]
fn variant_parsed_backend_addr_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::ParsedBackendAddr, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    match tag {
        tag if tag == JsValue::from_str("InvitationDevice") => {
            let hostname = {
                let js_val = Reflect::get(&obj, &"hostname".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            let port = {
                let js_val = Reflect::get(&obj, &"port".into())?;
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
            let use_ssl = {
                let js_val = Reflect::get(&obj, &"useSsl".into())?;
                js_val
                    .dyn_into::<Boolean>()
                    .map_err(|_| TypeError::new("Not a boolean"))?
                    .value_of()
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
            let token = {
                let js_val = Reflect::get(&obj, &"token".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string =
                            |s: String| -> Result<libparsec::InvitationToken, _> {
                                libparsec::InvitationToken::from_hex(s.as_str())
                                    .map_err(|e| e.to_string())
                            };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid InvitationToken"))?
            };
            Ok(libparsec::ParsedBackendAddr::InvitationDevice {
                hostname,
                port,
                use_ssl,
                organization_id,
                token,
            })
        }
        tag if tag == JsValue::from_str("InvitationUser") => {
            let hostname = {
                let js_val = Reflect::get(&obj, &"hostname".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            let port = {
                let js_val = Reflect::get(&obj, &"port".into())?;
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
            let use_ssl = {
                let js_val = Reflect::get(&obj, &"useSsl".into())?;
                js_val
                    .dyn_into::<Boolean>()
                    .map_err(|_| TypeError::new("Not a boolean"))?
                    .value_of()
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
            let token = {
                let js_val = Reflect::get(&obj, &"token".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string =
                            |s: String| -> Result<libparsec::InvitationToken, _> {
                                libparsec::InvitationToken::from_hex(s.as_str())
                                    .map_err(|e| e.to_string())
                            };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid InvitationToken"))?
            };
            Ok(libparsec::ParsedBackendAddr::InvitationUser {
                hostname,
                port,
                use_ssl,
                organization_id,
                token,
            })
        }
        tag if tag == JsValue::from_str("Organization") => {
            let hostname = {
                let js_val = Reflect::get(&obj, &"hostname".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            let port = {
                let js_val = Reflect::get(&obj, &"port".into())?;
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
            let use_ssl = {
                let js_val = Reflect::get(&obj, &"useSsl".into())?;
                js_val
                    .dyn_into::<Boolean>()
                    .map_err(|_| TypeError::new("Not a boolean"))?
                    .value_of()
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
            Ok(libparsec::ParsedBackendAddr::Organization {
                hostname,
                port,
                use_ssl,
                organization_id,
            })
        }
        tag if tag == JsValue::from_str("OrganizationBootstrap") => {
            let hostname = {
                let js_val = Reflect::get(&obj, &"hostname".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            let port = {
                let js_val = Reflect::get(&obj, &"port".into())?;
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
            let use_ssl = {
                let js_val = Reflect::get(&obj, &"useSsl".into())?;
                js_val
                    .dyn_into::<Boolean>()
                    .map_err(|_| TypeError::new("Not a boolean"))?
                    .value_of()
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
            let token = {
                let js_val = Reflect::get(&obj, &"token".into())?;
                if js_val.is_null() {
                    None
                } else {
                    Some(
                        js_val
                            .dyn_into::<JsString>()
                            .ok()
                            .and_then(|s| s.as_string())
                            .ok_or_else(|| TypeError::new("Not a string"))?,
                    )
                }
            };
            Ok(libparsec::ParsedBackendAddr::OrganizationBootstrap {
                hostname,
                port,
                use_ssl,
                organization_id,
                token,
            })
        }
        tag if tag == JsValue::from_str("OrganizationFileLink") => {
            let hostname = {
                let js_val = Reflect::get(&obj, &"hostname".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            let port = {
                let js_val = Reflect::get(&obj, &"port".into())?;
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
            let use_ssl = {
                let js_val = Reflect::get(&obj, &"useSsl".into())?;
                js_val
                    .dyn_into::<Boolean>()
                    .map_err(|_| TypeError::new("Not a boolean"))?
                    .value_of()
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
            let workspace_id = {
                let js_val = Reflect::get(&obj, &"workspaceId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                            libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })
                    .map_err(|_| TypeError::new("Not a valid VlobID"))?
            };
            let encrypted_path = {
                let js_val = Reflect::get(&obj, &"encryptedPath".into())?;
                js_val
                    .dyn_into::<Uint8Array>()
                    .map_err(|_| TypeError::new("Not a Uint8Array"))?
                    .to_vec()
            };
            let encrypted_timestamp = {
                let js_val = Reflect::get(&obj, &"encryptedTimestamp".into())?;
                if js_val.is_null() {
                    None
                } else {
                    Some(
                        js_val
                            .dyn_into::<Uint8Array>()
                            .map_err(|_| TypeError::new("Not a Uint8Array"))?
                            .to_vec(),
                    )
                }
            };
            Ok(libparsec::ParsedBackendAddr::OrganizationFileLink {
                hostname,
                port,
                use_ssl,
                organization_id,
                workspace_id,
                encrypted_path,
                encrypted_timestamp,
            })
        }
        tag if tag == JsValue::from_str("PkiEnrollment") => {
            let hostname = {
                let js_val = Reflect::get(&obj, &"hostname".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            let port = {
                let js_val = Reflect::get(&obj, &"port".into())?;
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
            let use_ssl = {
                let js_val = Reflect::get(&obj, &"useSsl".into())?;
                js_val
                    .dyn_into::<Boolean>()
                    .map_err(|_| TypeError::new("Not a boolean"))?
                    .value_of()
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
            Ok(libparsec::ParsedBackendAddr::PkiEnrollment {
                hostname,
                port,
                use_ssl,
                organization_id,
            })
        }
        tag if tag == JsValue::from_str("Server") => {
            let hostname = {
                let js_val = Reflect::get(&obj, &"hostname".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            let port = {
                let js_val = Reflect::get(&obj, &"port".into())?;
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
            let use_ssl = {
                let js_val = Reflect::get(&obj, &"useSsl".into())?;
                js_val
                    .dyn_into::<Boolean>()
                    .map_err(|_| TypeError::new("Not a boolean"))?
                    .value_of()
            };
            Ok(libparsec::ParsedBackendAddr::Server {
                hostname,
                port,
                use_ssl,
            })
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a ParsedBackendAddr",
        ))),
    }
}

#[allow(dead_code)]
fn variant_parsed_backend_addr_rs_to_js(
    rs_obj: libparsec::ParsedBackendAddr,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::ParsedBackendAddr::InvitationDevice {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"InvitationDevice".into())?;
            let js_hostname = hostname.into();
            Reflect::set(&js_obj, &"hostname".into(), &js_hostname)?;
            let js_port = JsValue::from(port);
            Reflect::set(&js_obj, &"port".into(), &js_port)?;
            let js_use_ssl = use_ssl.into();
            Reflect::set(&js_obj, &"useSsl".into(), &js_use_ssl)?;
            let js_organization_id = JsValue::from_str(organization_id.as_ref());
            Reflect::set(&js_obj, &"organizationId".into(), &js_organization_id)?;
            let js_token = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
        }
        libparsec::ParsedBackendAddr::InvitationUser {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"InvitationUser".into())?;
            let js_hostname = hostname.into();
            Reflect::set(&js_obj, &"hostname".into(), &js_hostname)?;
            let js_port = JsValue::from(port);
            Reflect::set(&js_obj, &"port".into(), &js_port)?;
            let js_use_ssl = use_ssl.into();
            Reflect::set(&js_obj, &"useSsl".into(), &js_use_ssl)?;
            let js_organization_id = JsValue::from_str(organization_id.as_ref());
            Reflect::set(&js_obj, &"organizationId".into(), &js_organization_id)?;
            let js_token = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
        }
        libparsec::ParsedBackendAddr::Organization {
            hostname,
            port,
            use_ssl,
            organization_id,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"Organization".into())?;
            let js_hostname = hostname.into();
            Reflect::set(&js_obj, &"hostname".into(), &js_hostname)?;
            let js_port = JsValue::from(port);
            Reflect::set(&js_obj, &"port".into(), &js_port)?;
            let js_use_ssl = use_ssl.into();
            Reflect::set(&js_obj, &"useSsl".into(), &js_use_ssl)?;
            let js_organization_id = JsValue::from_str(organization_id.as_ref());
            Reflect::set(&js_obj, &"organizationId".into(), &js_organization_id)?;
        }
        libparsec::ParsedBackendAddr::OrganizationBootstrap {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"OrganizationBootstrap".into())?;
            let js_hostname = hostname.into();
            Reflect::set(&js_obj, &"hostname".into(), &js_hostname)?;
            let js_port = JsValue::from(port);
            Reflect::set(&js_obj, &"port".into(), &js_port)?;
            let js_use_ssl = use_ssl.into();
            Reflect::set(&js_obj, &"useSsl".into(), &js_use_ssl)?;
            let js_organization_id = JsValue::from_str(organization_id.as_ref());
            Reflect::set(&js_obj, &"organizationId".into(), &js_organization_id)?;
            let js_token = match token {
                Some(val) => val.into(),
                None => JsValue::NULL,
            };
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
        }
        libparsec::ParsedBackendAddr::OrganizationFileLink {
            hostname,
            port,
            use_ssl,
            organization_id,
            workspace_id,
            encrypted_path,
            encrypted_timestamp,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"OrganizationFileLink".into())?;
            let js_hostname = hostname.into();
            Reflect::set(&js_obj, &"hostname".into(), &js_hostname)?;
            let js_port = JsValue::from(port);
            Reflect::set(&js_obj, &"port".into(), &js_port)?;
            let js_use_ssl = use_ssl.into();
            Reflect::set(&js_obj, &"useSsl".into(), &js_use_ssl)?;
            let js_organization_id = JsValue::from_str(organization_id.as_ref());
            Reflect::set(&js_obj, &"organizationId".into(), &js_organization_id)?;
            let js_workspace_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(workspace_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"workspaceId".into(), &js_workspace_id)?;
            let js_encrypted_path = JsValue::from(Uint8Array::from(encrypted_path.as_ref()));
            Reflect::set(&js_obj, &"encryptedPath".into(), &js_encrypted_path)?;
            let js_encrypted_timestamp = match encrypted_timestamp {
                Some(val) => JsValue::from(Uint8Array::from(val.as_ref())),
                None => JsValue::NULL,
            };
            Reflect::set(
                &js_obj,
                &"encryptedTimestamp".into(),
                &js_encrypted_timestamp,
            )?;
        }
        libparsec::ParsedBackendAddr::PkiEnrollment {
            hostname,
            port,
            use_ssl,
            organization_id,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"PkiEnrollment".into())?;
            let js_hostname = hostname.into();
            Reflect::set(&js_obj, &"hostname".into(), &js_hostname)?;
            let js_port = JsValue::from(port);
            Reflect::set(&js_obj, &"port".into(), &js_port)?;
            let js_use_ssl = use_ssl.into();
            Reflect::set(&js_obj, &"useSsl".into(), &js_use_ssl)?;
            let js_organization_id = JsValue::from_str(organization_id.as_ref());
            Reflect::set(&js_obj, &"organizationId".into(), &js_organization_id)?;
        }
        libparsec::ParsedBackendAddr::Server {
            hostname,
            port,
            use_ssl,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"Server".into())?;
            let js_hostname = hostname.into();
            Reflect::set(&js_obj, &"hostname".into(), &js_hostname)?;
            let js_port = JsValue::from(port);
            Reflect::set(&js_obj, &"port".into(), &js_port)?;
            let js_use_ssl = use_ssl.into();
            Reflect::set(&js_obj, &"useSsl".into(), &js_use_ssl)?;
        }
    }
    Ok(js_obj)
}

// UserOrDeviceClaimInitialInfo

#[allow(dead_code)]
fn variant_user_or_device_claim_initial_info_js_to_rs(
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
                    Some(struct_human_handle_js_to_rs(js_val)?)
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
                    Some(struct_human_handle_js_to_rs(js_val)?)
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
fn variant_user_or_device_claim_initial_info_rs_to_js(
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
                Some(val) => struct_human_handle_rs_to_js(val)?,
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
                Some(val) => struct_human_handle_rs_to_js(val)?,
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

// WorkspaceFsOperationError

#[allow(dead_code)]
fn variant_workspace_fs_operation_error_rs_to_js(
    rs_obj: libparsec::WorkspaceFsOperationError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceFsOperationError::BadTimestamp {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"BadTimestamp".into())?;
            let js_server_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok(dt.get_f64_with_us_precision())
                };
                let v = match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"serverTimestamp".into(), &js_server_timestamp)?;
            let js_client_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok(dt.get_f64_with_us_precision())
                };
                let v = match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
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
        libparsec::WorkspaceFsOperationError::CannotRenameRoot { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"CannotRenameRoot".into())?;
        }
        libparsec::WorkspaceFsOperationError::EntryExists { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"EntryExists".into())?;
        }
        libparsec::WorkspaceFsOperationError::EntryNotFound { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"EntryNotFound".into())?;
        }
        libparsec::WorkspaceFsOperationError::FolderNotEmpty { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"FolderNotEmpty".into())?;
        }
        libparsec::WorkspaceFsOperationError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
        libparsec::WorkspaceFsOperationError::InvalidCertificate { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"InvalidCertificate".into())?;
        }
        libparsec::WorkspaceFsOperationError::InvalidManifest { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"InvalidManifest".into())?;
        }
        libparsec::WorkspaceFsOperationError::IsAFolder { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"IsAFolder".into())?;
        }
        libparsec::WorkspaceFsOperationError::NoRealmAccess { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"NoRealmAccess".into())?;
        }
        libparsec::WorkspaceFsOperationError::NotAFolder { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"NotAFolder".into())?;
        }
        libparsec::WorkspaceFsOperationError::Offline { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Offline".into())?;
        }
        libparsec::WorkspaceFsOperationError::ReadOnlyRealm { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ReadOnlyRealm".into())?;
        }
    }
    Ok(js_obj)
}

// WorkspaceStopError

#[allow(dead_code)]
fn variant_workspace_stop_error_rs_to_js(
    rs_obj: libparsec::WorkspaceStopError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceStopError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"Internal".into())?;
        }
    }
    Ok(js_obj)
}

// WorkspaceStorageCacheSize

#[allow(dead_code)]
fn variant_workspace_storage_cache_size_js_to_rs(
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
fn variant_workspace_storage_cache_size_rs_to_js(
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

// bootstrap_organization
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn bootstrapOrganization(
    config: Object,
    on_event_callback: Function,
    bootstrap_organization_addr: String,
    save_strategy: Object,
    human_handle: Option<Object>,
    device_label: Option<String>,
    sequester_authority_verify_key: Option<Uint8Array>,
) -> Promise {
    future_to_promise(async move {
        let config = config.into();
        let config = struct_client_config_js_to_rs(config)?;

        let on_event_callback = std::sync::Arc::new(move |event: libparsec::ClientEvent| {
            // TODO: Better error handling here (log error ?)
            let js_event =
                variant_client_event_rs_to_js(event).expect("event type conversion error");
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
        let save_strategy = variant_device_save_strategy_js_to_rs(save_strategy)?;

        let human_handle = match human_handle {
            Some(human_handle) => {
                let human_handle = human_handle.into();
                let human_handle = struct_human_handle_js_to_rs(human_handle)?;

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
                let js_value = struct_available_device_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_bootstrap_organization_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// build_backend_organization_bootstrap_addr
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn buildBackendOrganizationBootstrapAddr(addr: String, organization_id: String) -> Promise {
    future_to_promise(async move {
        let addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::BackendAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(addr).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let organization_id = organization_id
            .parse()
            .map_err(|_| JsValue::from(TypeError::new("Not a valid OrganizationID")))?;
        let ret = libparsec::build_backend_organization_bootstrap_addr(addr, organization_id);
        Ok(JsValue::from_str({
            let custom_to_rs_string = |addr: libparsec::BackendOrganizationBootstrapAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) };
            match custom_to_rs_string(ret) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
            }
            .as_ref()
        }))
    })
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
                let js_err = variant_cancel_error_rs_to_js(err)?;
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
        let save_strategy = variant_device_save_strategy_js_to_rs(save_strategy)?;

        let ret = libparsec::claimer_device_finalize_save_local_device(handle, save_strategy).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_available_device_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claim_in_progress_error_rs_to_js(err)?;
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
                let js_value = struct_device_claim_in_progress2_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claim_in_progress_error_rs_to_js(err)?;
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
                let js_value = struct_device_claim_in_progress3_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claim_in_progress_error_rs_to_js(err)?;
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
                let js_value = struct_device_claim_finalize_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claim_in_progress_error_rs_to_js(err)?;
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
                let js_value = struct_device_claim_in_progress1_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claim_in_progress_error_rs_to_js(err)?;
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
                let js_err = variant_claimer_greeter_abort_operation_error_rs_to_js(err)?;
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
        let config = struct_client_config_js_to_rs(config)?;

        let on_event_callback = std::sync::Arc::new(move |event: libparsec::ClientEvent| {
            // TODO: Better error handling here (log error ?)
            let js_event =
                variant_client_event_rs_to_js(event).expect("event type conversion error");
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
                let js_value = variant_user_or_device_claim_initial_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claimer_retrieve_info_error_rs_to_js(err)?;
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
        let save_strategy = variant_device_save_strategy_js_to_rs(save_strategy)?;

        let ret = libparsec::claimer_user_finalize_save_local_device(handle, save_strategy).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_available_device_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claim_in_progress_error_rs_to_js(err)?;
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
                let js_value = struct_user_claim_in_progress2_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claim_in_progress_error_rs_to_js(err)?;
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
                let js_value = struct_user_claim_in_progress3_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claim_in_progress_error_rs_to_js(err)?;
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
    requested_human_handle: Option<Object>,
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
                let requested_human_handle = requested_human_handle.into();
                let requested_human_handle = struct_human_handle_js_to_rs(requested_human_handle)?;

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
                let js_value = struct_user_claim_finalize_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claim_in_progress_error_rs_to_js(err)?;
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
                let js_value = struct_user_claim_in_progress1_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_claim_in_progress_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_create_workspace
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientCreateWorkspace(client: u32, name: String) -> Promise {
    future_to_promise(async move {
        let name = {
            let custom_from_rs_string = |s: String| -> Result<_, _> {
                s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(name).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_create_workspace(client, name).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from_str({
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                    .as_ref()
                });
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_client_create_workspace_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_delete_invitation
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientDeleteInvitation(client: u32, token: String) -> Promise {
    future_to_promise(async move {
        let token = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::InvitationToken, _> {
                libparsec::InvitationToken::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(token).map_err(|e| TypeError::new(e.as_ref()))
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
                let js_err = variant_delete_invitation_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_get_user_device
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientGetUserDevice(client: u32, device: String) -> Promise {
    future_to_promise(async move {
        let device = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::DeviceID>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(device).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_get_user_device(client, device).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let (x1, x2) = value;
                    let js_array = Array::new_with_length(2);
                    let js_value = struct_user_info_rs_to_js(x1)?;
                    js_array.push(&js_value);
                    let js_value = struct_device_info_rs_to_js(x2)?;
                    js_array.push(&js_value);
                    js_array.into()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_client_get_user_device_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_info
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientInfo(client: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::client_info(client).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_client_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_client_info_error_rs_to_js(err)?;
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
                        let js_elem = variant_invite_list_item_rs_to_js(elem)?;
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
                let js_err = variant_list_invitations_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_list_user_devices
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientListUserDevices(client: u32, user: String) -> Promise {
    future_to_promise(async move {
        let user = user
            .parse()
            .map_err(|_| JsValue::from(TypeError::new("Not a valid UserID")))?;

        let ret = libparsec::client_list_user_devices(client, user).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(value.len() as u32);
                    for (i, elem) in value.into_iter().enumerate() {
                        let js_elem = struct_device_info_rs_to_js(elem)?;
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
                let js_err = variant_client_list_user_devices_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_list_users
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientListUsers(client: u32, skip_revoked: bool) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::client_list_users(client, skip_revoked).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(value.len() as u32);
                    for (i, elem) in value.into_iter().enumerate() {
                        let js_elem = struct_user_info_rs_to_js(elem)?;
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
                let js_err = variant_client_list_users_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_list_workspace_users
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientListWorkspaceUsers(client: u32, realm_id: String) -> Promise {
    future_to_promise(async move {
        let realm_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(realm_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_list_workspace_users(client, realm_id).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(value.len() as u32);
                    for (i, elem) in value.into_iter().enumerate() {
                        let js_elem = struct_workspace_user_access_info_rs_to_js(elem)?;
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
                let js_err = variant_client_list_workspace_users_error_rs_to_js(err)?;
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
                        let js_elem = struct_workspace_info_rs_to_js(elem)?;
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
                let js_err = variant_client_list_workspaces_error_rs_to_js(err)?;
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
                let js_value = struct_new_invitation_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_new_device_invitation_error_rs_to_js(err)?;
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
                let js_value = struct_new_invitation_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_new_user_invitation_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_rename_workspace
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientRenameWorkspace(client: u32, realm_id: String, new_name: String) -> Promise {
    future_to_promise(async move {
        let realm_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(realm_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let new_name = {
            let custom_from_rs_string = |s: String| -> Result<_, _> {
                s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(new_name).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_rename_workspace(client, realm_id, new_name).await;
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
                let js_err = variant_client_rename_workspace_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_share_workspace
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientShareWorkspace(
    client: u32,
    realm_id: String,
    recipient: String,
    role: Option<String>,
) -> Promise {
    future_to_promise(async move {
        let realm_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(realm_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let recipient = recipient
            .parse()
            .map_err(|_| JsValue::from(TypeError::new("Not a valid UserID")))?;

        let role = match role {
            Some(role) => {
                let role = enum_realm_role_js_to_rs(&role)?;

                Some(role)
            }
            None => None,
        };

        let ret = libparsec::client_share_workspace(client, realm_id, recipient, role).await;
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
                let js_err = variant_client_share_workspace_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_start
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientStart(config: Object, on_event_callback: Function, access: Object) -> Promise {
    future_to_promise(async move {
        let config = config.into();
        let config = struct_client_config_js_to_rs(config)?;

        let on_event_callback = std::sync::Arc::new(move |event: libparsec::ClientEvent| {
            // TODO: Better error handling here (log error ?)
            let js_event =
                variant_client_event_rs_to_js(event).expect("event type conversion error");
            on_event_callback
                .call1(&JsValue::NULL, &js_event)
                .expect("error in event callback");
        }) as std::sync::Arc<dyn Fn(libparsec::ClientEvent)>;

        let access = access.into();
        let access = variant_device_access_strategy_js_to_rs(access)?;

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
                let js_err = variant_client_start_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_start_device_invitation_greet
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientStartDeviceInvitationGreet(client: u32, token: String) -> Promise {
    future_to_promise(async move {
        let token = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::InvitationToken, _> {
                libparsec::InvitationToken::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(token).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_start_device_invitation_greet(client, token).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_device_greet_initial_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_client_start_invitation_greet_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_start_user_invitation_greet
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientStartUserInvitationGreet(client: u32, token: String) -> Promise {
    future_to_promise(async move {
        let token = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::InvitationToken, _> {
                libparsec::InvitationToken::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(token).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_start_user_invitation_greet(client, token).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_user_greet_initial_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_client_start_invitation_greet_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// client_start_workspace
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientStartWorkspace(client: u32, realm_id: String) -> Promise {
    future_to_promise(async move {
        let realm_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(realm_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_start_workspace(client, realm_id).await;
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
                let js_err = variant_client_start_workspace_error_rs_to_js(err)?;
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
                let js_err = variant_client_stop_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// get_default_config_dir
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn getDefaultConfigDir() -> Promise {
    future_to_promise(async move {
        let ret = libparsec::get_default_config_dir();
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

// get_default_data_base_dir
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn getDefaultDataBaseDir() -> Promise {
    future_to_promise(async move {
        let ret = libparsec::get_default_data_base_dir();
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

// get_default_mountpoint_base_dir
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn getDefaultMountpointBaseDir() -> Promise {
    future_to_promise(async move {
        let ret = libparsec::get_default_mountpoint_base_dir();
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

// get_platform
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn getPlatform() -> Promise {
    future_to_promise(async move {
        let ret = libparsec::get_platform();
        Ok(JsValue::from_str(enum_platform_rs_to_js(ret)))
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
                let js_value = struct_device_greet_in_progress2_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greet_in_progress_error_rs_to_js(err)?;
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
                let js_value = struct_device_greet_in_progress3_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greet_in_progress_error_rs_to_js(err)?;
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
                let js_value = struct_device_greet_in_progress4_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greet_in_progress_error_rs_to_js(err)?;
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
                let js_err = variant_greet_in_progress_error_rs_to_js(err)?;
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
                let js_value = struct_device_greet_in_progress1_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greet_in_progress_error_rs_to_js(err)?;
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
                let js_value = struct_user_greet_in_progress2_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greet_in_progress_error_rs_to_js(err)?;
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
                let js_value = struct_user_greet_in_progress3_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greet_in_progress_error_rs_to_js(err)?;
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
                let js_value = struct_user_greet_in_progress4_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greet_in_progress_error_rs_to_js(err)?;
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
    human_handle: Option<Object>,
    device_label: Option<String>,
    profile: String,
) -> Promise {
    future_to_promise(async move {
        let human_handle = match human_handle {
            Some(human_handle) => {
                let human_handle = human_handle.into();
                let human_handle = struct_human_handle_js_to_rs(human_handle)?;

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

        let profile = enum_user_profile_js_to_rs(&profile)?;

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
                let js_err = variant_greet_in_progress_error_rs_to_js(err)?;
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
                let js_value = struct_user_greet_in_progress1_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_greet_in_progress_error_rs_to_js(err)?;
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
                let js_elem = struct_available_device_rs_to_js(elem)?;
                js_array.set(i as u32, js_elem);
            }
            js_array.into()
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

// parse_backend_addr
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn parseBackendAddr(url: String) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::parse_backend_addr(&url);
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = variant_parsed_backend_addr_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_parse_backend_addr_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
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

// validate_device_label
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn validateDeviceLabel(raw: String) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::validate_device_label(&raw);
        Ok(ret.into())
    })
}

// validate_email
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn validateEmail(raw: String) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::validate_email(&raw);
        Ok(ret.into())
    })
}

// validate_entry_name
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn validateEntryName(raw: String) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::validate_entry_name(&raw);
        Ok(ret.into())
    })
}

// validate_human_handle_label
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn validateHumanHandleLabel(raw: String) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::validate_human_handle_label(&raw);
        Ok(ret.into())
    })
}

// validate_invitation_token
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn validateInvitationToken(raw: String) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::validate_invitation_token(&raw);
        Ok(ret.into())
    })
}

// validate_path
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn validatePath(raw: String) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::validate_path(&raw);
        Ok(ret.into())
    })
}

// workspace_create_file
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceCreateFile(workspace: u32, path: String) -> Promise {
    future_to_promise(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_create_file(workspace, &path).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from_str({
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                    .as_ref()
                });
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_fs_operation_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// workspace_create_folder
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceCreateFolder(workspace: u32, path: String) -> Promise {
    future_to_promise(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_create_folder(workspace, &path).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from_str({
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                    .as_ref()
                });
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_fs_operation_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// workspace_create_folder_all
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceCreateFolderAll(workspace: u32, path: String) -> Promise {
    future_to_promise(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_create_folder_all(workspace, &path).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from_str({
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                    .as_ref()
                });
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_fs_operation_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// workspace_remove_entry
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceRemoveEntry(workspace: u32, path: String) -> Promise {
    future_to_promise(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_remove_entry(workspace, &path).await;
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
                let js_err = variant_workspace_fs_operation_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// workspace_remove_file
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceRemoveFile(workspace: u32, path: String) -> Promise {
    future_to_promise(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_remove_file(workspace, &path).await;
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
                let js_err = variant_workspace_fs_operation_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// workspace_remove_folder
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceRemoveFolder(workspace: u32, path: String) -> Promise {
    future_to_promise(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_remove_folder(workspace, &path).await;
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
                let js_err = variant_workspace_fs_operation_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// workspace_remove_folder_all
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceRemoveFolderAll(workspace: u32, path: String) -> Promise {
    future_to_promise(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_remove_folder_all(workspace, &path).await;
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
                let js_err = variant_workspace_fs_operation_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// workspace_rename_entry
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceRenameEntry(
    workspace: u32,
    path: String,
    new_name: String,
    overwrite: bool,
) -> Promise {
    future_to_promise(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let new_name = {
            let custom_from_rs_string = |s: String| -> Result<_, _> {
                s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(new_name).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_rename_entry(workspace, &path, new_name, overwrite).await;
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
                let js_err = variant_workspace_fs_operation_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// workspace_stat_entry
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceStatEntry(workspace: u32, path: String) -> Promise {
    future_to_promise(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_stat_entry(workspace, &path).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = variant_entry_stat_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_fs_operation_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

// workspace_stop
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceStop(workspace: u32) -> Promise {
    future_to_promise(async move {
        let ret = libparsec::workspace_stop(workspace).await;
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
                let js_err = variant_workspace_stop_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    })
}

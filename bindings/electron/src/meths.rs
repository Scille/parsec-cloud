// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

#[allow(unused_imports)]
use neon::{prelude::*, types::buffer::TypedArray};

// DeviceFileType

#[allow(dead_code)]
fn enum_device_file_type_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    raw_value: &str,
) -> NeonResult<libparsec::DeviceFileType> {
    match raw_value {
        "DeviceFileTypePassword" => Ok(libparsec::DeviceFileType::Password),
        "DeviceFileTypeRecovery" => Ok(libparsec::DeviceFileType::Recovery),
        "DeviceFileTypeSmartcard" => Ok(libparsec::DeviceFileType::Smartcard),
        _ => cx.throw_range_error(format!(
            "Invalid value `{raw_value}` for enum DeviceFileType"
        )),
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
fn enum_invitation_email_sent_status_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    raw_value: &str,
) -> NeonResult<libparsec::InvitationEmailSentStatus> {
    match raw_value {
        "InvitationEmailSentStatusBadRecipient" => {
            Ok(libparsec::InvitationEmailSentStatus::BadRecipient)
        }
        "InvitationEmailSentStatusNotAvailable" => {
            Ok(libparsec::InvitationEmailSentStatus::NotAvailable)
        }
        "InvitationEmailSentStatusSuccess" => Ok(libparsec::InvitationEmailSentStatus::Success),
        _ => cx.throw_range_error(format!(
            "Invalid value `{raw_value}` for enum InvitationEmailSentStatus"
        )),
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
fn enum_invitation_status_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    raw_value: &str,
) -> NeonResult<libparsec::InvitationStatus> {
    match raw_value {
        "InvitationStatusDeleted" => Ok(libparsec::InvitationStatus::Deleted),
        "InvitationStatusIdle" => Ok(libparsec::InvitationStatus::Idle),
        "InvitationStatusReady" => Ok(libparsec::InvitationStatus::Ready),
        _ => cx.throw_range_error(format!(
            "Invalid value `{raw_value}` for enum InvitationStatus"
        )),
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
fn enum_platform_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    raw_value: &str,
) -> NeonResult<libparsec::Platform> {
    match raw_value {
        "PlatformAndroid" => Ok(libparsec::Platform::Android),
        "PlatformLinux" => Ok(libparsec::Platform::Linux),
        "PlatformMacOS" => Ok(libparsec::Platform::MacOS),
        "PlatformWeb" => Ok(libparsec::Platform::Web),
        "PlatformWindows" => Ok(libparsec::Platform::Windows),
        _ => cx.throw_range_error(format!("Invalid value `{raw_value}` for enum Platform")),
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
fn enum_realm_role_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    raw_value: &str,
) -> NeonResult<libparsec::RealmRole> {
    match raw_value {
        "RealmRoleContributor" => Ok(libparsec::RealmRole::Contributor),
        "RealmRoleManager" => Ok(libparsec::RealmRole::Manager),
        "RealmRoleOwner" => Ok(libparsec::RealmRole::Owner),
        "RealmRoleReader" => Ok(libparsec::RealmRole::Reader),
        _ => cx.throw_range_error(format!("Invalid value `{raw_value}` for enum RealmRole")),
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
fn enum_user_profile_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    raw_value: &str,
) -> NeonResult<libparsec::UserProfile> {
    match raw_value {
        "UserProfileAdmin" => Ok(libparsec::UserProfile::Admin),
        "UserProfileOutsider" => Ok(libparsec::UserProfile::Outsider),
        "UserProfileStandard" => Ok(libparsec::UserProfile::Standard),
        _ => cx.throw_range_error(format!("Invalid value `{raw_value}` for enum UserProfile")),
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
fn struct_available_device_js_to_rs<'a>(
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
                Some(struct_human_handle_js_to_rs(cx, js_val)?)
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
        let js_val: Handle<JsString> = obj.get(cx, "ty")?;
        {
            let js_string = js_val.value(cx);
            enum_device_file_type_js_to_rs(cx, js_string.as_str())?
        }
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
fn struct_available_device_rs_to_js<'a>(
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
        Some(elem) => struct_human_handle_rs_to_js(cx, elem)?.as_value(cx),
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
    let js_ty = JsString::try_new(cx, enum_device_file_type_rs_to_js(rs_obj.ty)).or_throw(cx)?;
    js_obj.set(cx, "ty", js_ty)?;
    Ok(js_obj)
}

// ClientConfig

#[allow(dead_code)]
fn struct_client_config_js_to_rs<'a>(
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
        variant_workspace_storage_cache_size_js_to_rs(cx, js_val)?
    };
    Ok(libparsec::ClientConfig {
        config_dir,
        data_base_dir,
        mountpoint_base_dir,
        workspace_storage_cache_size,
    })
}

#[allow(dead_code)]
fn struct_client_config_rs_to_js<'a>(
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
        variant_workspace_storage_cache_size_rs_to_js(cx, rs_obj.workspace_storage_cache_size)?;
    js_obj.set(
        cx,
        "workspaceStorageCacheSize",
        js_workspace_storage_cache_size,
    )?;
    Ok(js_obj)
}

// ClientInfo

#[allow(dead_code)]
fn struct_client_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ClientInfo> {
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
    let user_id = {
        let js_val: Handle<JsString> = obj.get(cx, "userId")?;
        {
            match js_val.value(cx).parse() {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
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
    let human_handle = {
        let js_val: Handle<JsValue> = obj.get(cx, "humanHandle")?;
        {
            if js_val.is_a::<JsNull, _>(cx) {
                None
            } else {
                let js_val = js_val.downcast_or_throw::<JsObject, _>(cx)?;
                Some(struct_human_handle_js_to_rs(cx, js_val)?)
            }
        }
    };
    let current_profile = {
        let js_val: Handle<JsString> = obj.get(cx, "currentProfile")?;
        {
            let js_string = js_val.value(cx);
            enum_user_profile_js_to_rs(cx, js_string.as_str())?
        }
    };
    Ok(libparsec::ClientInfo {
        organization_id,
        device_id,
        user_id,
        device_label,
        human_handle,
        current_profile,
    })
}

#[allow(dead_code)]
fn struct_client_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_organization_id = JsString::try_new(cx, rs_obj.organization_id).or_throw(cx)?;
    js_obj.set(cx, "organizationId", js_organization_id)?;
    let js_device_id = JsString::try_new(cx, rs_obj.device_id).or_throw(cx)?;
    js_obj.set(cx, "deviceId", js_device_id)?;
    let js_user_id = JsString::try_new(cx, rs_obj.user_id).or_throw(cx)?;
    js_obj.set(cx, "userId", js_user_id)?;
    let js_device_label = match rs_obj.device_label {
        Some(elem) => JsString::try_new(cx, elem).or_throw(cx)?.as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "deviceLabel", js_device_label)?;
    let js_human_handle = match rs_obj.human_handle {
        Some(elem) => struct_human_handle_rs_to_js(cx, elem)?.as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "humanHandle", js_human_handle)?;
    let js_current_profile =
        JsString::try_new(cx, enum_user_profile_rs_to_js(rs_obj.current_profile)).or_throw(cx)?;
    js_obj.set(cx, "currentProfile", js_current_profile)?;
    Ok(js_obj)
}

// DeviceClaimFinalizeInfo

#[allow(dead_code)]
fn struct_device_claim_finalize_info_js_to_rs<'a>(
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
fn struct_device_claim_finalize_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceClaimFinalizeInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// DeviceClaimInProgress1Info

#[allow(dead_code)]
fn struct_device_claim_in_progress1_info_js_to_rs<'a>(
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
fn struct_device_claim_in_progress1_info_rs_to_js<'a>(
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

// DeviceClaimInProgress2Info

#[allow(dead_code)]
fn struct_device_claim_in_progress2_info_js_to_rs<'a>(
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
fn struct_device_claim_in_progress2_info_rs_to_js<'a>(
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

// DeviceClaimInProgress3Info

#[allow(dead_code)]
fn struct_device_claim_in_progress3_info_js_to_rs<'a>(
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
fn struct_device_claim_in_progress3_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceClaimInProgress3Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// DeviceGreetInProgress1Info

#[allow(dead_code)]
fn struct_device_greet_in_progress1_info_js_to_rs<'a>(
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
fn struct_device_greet_in_progress1_info_rs_to_js<'a>(
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

// DeviceGreetInProgress2Info

#[allow(dead_code)]
fn struct_device_greet_in_progress2_info_js_to_rs<'a>(
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
fn struct_device_greet_in_progress2_info_rs_to_js<'a>(
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

// DeviceGreetInProgress3Info

#[allow(dead_code)]
fn struct_device_greet_in_progress3_info_js_to_rs<'a>(
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
fn struct_device_greet_in_progress3_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceGreetInProgress3Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// DeviceGreetInProgress4Info

#[allow(dead_code)]
fn struct_device_greet_in_progress4_info_js_to_rs<'a>(
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
fn struct_device_greet_in_progress4_info_rs_to_js<'a>(
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

// DeviceGreetInitialInfo

#[allow(dead_code)]
fn struct_device_greet_initial_info_js_to_rs<'a>(
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
fn struct_device_greet_initial_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceGreetInitialInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// DeviceInfo

#[allow(dead_code)]
fn struct_device_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::DeviceInfo> {
    let id = {
        let js_val: Handle<JsString> = obj.get(cx, "id")?;
        {
            match js_val.value(cx).parse() {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
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
    let created_on = {
        let js_val: Handle<JsString> = obj.get(cx, "createdOn")?;
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
    let created_by = {
        let js_val: Handle<JsValue> = obj.get(cx, "createdBy")?;
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
    Ok(libparsec::DeviceInfo {
        id,
        device_label,
        created_on,
        created_by,
    })
}

#[allow(dead_code)]
fn struct_device_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::DeviceInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_id = JsString::try_new(cx, rs_obj.id).or_throw(cx)?;
    js_obj.set(cx, "id", js_id)?;
    let js_device_label = match rs_obj.device_label {
        Some(elem) => JsString::try_new(cx, elem).or_throw(cx)?.as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "deviceLabel", js_device_label)?;
    let js_created_on = JsString::try_new(cx, {
        let custom_to_rs_string =
            |dt: libparsec::DateTime| -> Result<String, &'static str> { Ok(dt.to_rfc3339()) };
        match custom_to_rs_string(rs_obj.created_on) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "createdOn", js_created_on)?;
    let js_created_by = match rs_obj.created_by {
        Some(elem) => JsString::try_new(cx, elem).or_throw(cx)?.as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "createdBy", js_created_by)?;
    Ok(js_obj)
}

// HumanHandle

#[allow(dead_code)]
fn struct_human_handle_js_to_rs<'a>(
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

    |email: String, label: String| -> Result<_, String> {
        libparsec::HumanHandle::new(&email, &label).map_err(|e| e.to_string())
    }(email, label)
    .or_else(|e| cx.throw_error(e))
}

#[allow(dead_code)]
fn struct_human_handle_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::HumanHandle,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_email = {
        let custom_getter = |obj| {
            fn access(obj: &libparsec::HumanHandle) -> &str {
                obj.email()
            }
            access(obj)
        };
        JsString::try_new(cx, custom_getter(&rs_obj)).or_throw(cx)?
    };
    js_obj.set(cx, "email", js_email)?;
    let js_label = {
        let custom_getter = |obj| {
            fn access(obj: &libparsec::HumanHandle) -> &str {
                obj.label()
            }
            access(obj)
        };
        JsString::try_new(cx, custom_getter(&rs_obj)).or_throw(cx)?
    };
    js_obj.set(cx, "label", js_label)?;
    Ok(js_obj)
}

// UserClaimFinalizeInfo

#[allow(dead_code)]
fn struct_user_claim_finalize_info_js_to_rs<'a>(
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
fn struct_user_claim_finalize_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserClaimFinalizeInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// UserClaimInProgress1Info

#[allow(dead_code)]
fn struct_user_claim_in_progress1_info_js_to_rs<'a>(
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
fn struct_user_claim_in_progress1_info_rs_to_js<'a>(
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

// UserClaimInProgress2Info

#[allow(dead_code)]
fn struct_user_claim_in_progress2_info_js_to_rs<'a>(
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
fn struct_user_claim_in_progress2_info_rs_to_js<'a>(
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

// UserClaimInProgress3Info

#[allow(dead_code)]
fn struct_user_claim_in_progress3_info_js_to_rs<'a>(
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
fn struct_user_claim_in_progress3_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserClaimInProgress3Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// UserGreetInProgress1Info

#[allow(dead_code)]
fn struct_user_greet_in_progress1_info_js_to_rs<'a>(
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
fn struct_user_greet_in_progress1_info_rs_to_js<'a>(
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

// UserGreetInProgress2Info

#[allow(dead_code)]
fn struct_user_greet_in_progress2_info_js_to_rs<'a>(
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
fn struct_user_greet_in_progress2_info_rs_to_js<'a>(
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

// UserGreetInProgress3Info

#[allow(dead_code)]
fn struct_user_greet_in_progress3_info_js_to_rs<'a>(
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
fn struct_user_greet_in_progress3_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserGreetInProgress3Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// UserGreetInProgress4Info

#[allow(dead_code)]
fn struct_user_greet_in_progress4_info_js_to_rs<'a>(
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
                Some(struct_human_handle_js_to_rs(cx, js_val)?)
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
fn struct_user_greet_in_progress4_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserGreetInProgress4Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_requested_human_handle = match rs_obj.requested_human_handle {
        Some(elem) => struct_human_handle_rs_to_js(cx, elem)?.as_value(cx),
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

// UserGreetInitialInfo

#[allow(dead_code)]
fn struct_user_greet_initial_info_js_to_rs<'a>(
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
fn struct_user_greet_initial_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserGreetInitialInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// UserInfo

#[allow(dead_code)]
fn struct_user_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserInfo> {
    let id = {
        let js_val: Handle<JsString> = obj.get(cx, "id")?;
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
                Some(struct_human_handle_js_to_rs(cx, js_val)?)
            }
        }
    };
    let current_profile = {
        let js_val: Handle<JsString> = obj.get(cx, "currentProfile")?;
        {
            let js_string = js_val.value(cx);
            enum_user_profile_js_to_rs(cx, js_string.as_str())?
        }
    };
    let created_on = {
        let js_val: Handle<JsString> = obj.get(cx, "createdOn")?;
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
    let created_by = {
        let js_val: Handle<JsValue> = obj.get(cx, "createdBy")?;
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
    let revoked_on = {
        let js_val: Handle<JsValue> = obj.get(cx, "revokedOn")?;
        {
            if js_val.is_a::<JsNull, _>(cx) {
                None
            } else {
                let js_val = js_val.downcast_or_throw::<JsString, _>(cx)?;
                Some({
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::DateTime::from_rfc3339(&s).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                })
            }
        }
    };
    let revoked_by = {
        let js_val: Handle<JsValue> = obj.get(cx, "revokedBy")?;
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
fn struct_user_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_id = JsString::try_new(cx, rs_obj.id).or_throw(cx)?;
    js_obj.set(cx, "id", js_id)?;
    let js_human_handle = match rs_obj.human_handle {
        Some(elem) => struct_human_handle_rs_to_js(cx, elem)?.as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "humanHandle", js_human_handle)?;
    let js_current_profile =
        JsString::try_new(cx, enum_user_profile_rs_to_js(rs_obj.current_profile)).or_throw(cx)?;
    js_obj.set(cx, "currentProfile", js_current_profile)?;
    let js_created_on = JsString::try_new(cx, {
        let custom_to_rs_string =
            |dt: libparsec::DateTime| -> Result<String, &'static str> { Ok(dt.to_rfc3339()) };
        match custom_to_rs_string(rs_obj.created_on) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "createdOn", js_created_on)?;
    let js_created_by = match rs_obj.created_by {
        Some(elem) => JsString::try_new(cx, elem).or_throw(cx)?.as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "createdBy", js_created_by)?;
    let js_revoked_on = match rs_obj.revoked_on {
        Some(elem) => JsString::try_new(cx, {
            let custom_to_rs_string =
                |dt: libparsec::DateTime| -> Result<String, &'static str> { Ok(dt.to_rfc3339()) };
            match custom_to_rs_string(elem) {
                Ok(ok) => ok,
                Err(err) => return cx.throw_type_error(err),
            }
        })
        .or_throw(cx)?
        .as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "revokedOn", js_revoked_on)?;
    let js_revoked_by = match rs_obj.revoked_by {
        Some(elem) => JsString::try_new(cx, elem).or_throw(cx)?.as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "revokedBy", js_revoked_by)?;
    Ok(js_obj)
}

// WorkspaceInfo

#[allow(dead_code)]
fn struct_workspace_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::WorkspaceInfo> {
    let id = {
        let js_val: Handle<JsString> = obj.get(cx, "id")?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let name = {
        let js_val: Handle<JsString> = obj.get(cx, "name")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, _> {
                s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let self_role = {
        let js_val: Handle<JsString> = obj.get(cx, "selfRole")?;
        {
            let js_string = js_val.value(cx);
            enum_realm_role_js_to_rs(cx, js_string.as_str())?
        }
    };
    Ok(libparsec::WorkspaceInfo {
        id,
        name,
        self_role,
    })
}

#[allow(dead_code)]
fn struct_workspace_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_id = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.id) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "id", js_id)?;
    let js_name = JsString::try_new(cx, rs_obj.name).or_throw(cx)?;
    js_obj.set(cx, "name", js_name)?;
    let js_self_role =
        JsString::try_new(cx, enum_realm_role_rs_to_js(rs_obj.self_role)).or_throw(cx)?;
    js_obj.set(cx, "selfRole", js_self_role)?;
    Ok(js_obj)
}

// BootstrapOrganizationError

#[allow(dead_code)]
fn variant_bootstrap_organization_error_rs_to_js<'a>(
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
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "serverTimestamp", js_server_timestamp)?;
            let js_client_timestamp = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "clientTimestamp", js_client_timestamp)?;
            let js_ballpark_client_early_offset = JsNumber::new(cx, ballpark_client_early_offset);
            js_obj.set(
                cx,
                "ballparkClientEarlyOffset",
                js_ballpark_client_early_offset,
            )?;
            let js_ballpark_client_late_offset = JsNumber::new(cx, ballpark_client_late_offset);
            js_obj.set(
                cx,
                "ballparkClientLateOffset",
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

// CancelError

#[allow(dead_code)]
fn variant_cancel_error_rs_to_js<'a>(
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
        libparsec::CancelError::NotBound { .. } => {
            let js_tag = JsString::try_new(cx, "NotBound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClaimInProgressError

#[allow(dead_code)]
fn variant_claim_in_progress_error_rs_to_js<'a>(
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

// ClaimerGreeterAbortOperationError

#[allow(dead_code)]
fn variant_claimer_greeter_abort_operation_error_rs_to_js<'a>(
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

// ClaimerRetrieveInfoError

#[allow(dead_code)]
fn variant_claimer_retrieve_info_error_rs_to_js<'a>(
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

// ClientCreateWorkspaceError

#[allow(dead_code)]
fn variant_client_create_workspace_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientCreateWorkspaceError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientCreateWorkspaceError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientEvent

#[allow(dead_code)]
fn variant_client_event_js_to_rs<'a>(
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
fn variant_client_event_rs_to_js<'a>(
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

// ClientGetUserDeviceError

#[allow(dead_code)]
fn variant_client_get_user_device_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientGetUserDeviceError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientGetUserDeviceError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientGetUserDeviceError::NonExisting { .. } => {
            let js_tag = JsString::try_new(cx, "NonExisting").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientInfoError

#[allow(dead_code)]
fn variant_client_info_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientInfoError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientInfoError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientListUserDevicesError

#[allow(dead_code)]
fn variant_client_list_user_devices_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientListUserDevicesError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientListUserDevicesError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientListUsersError

#[allow(dead_code)]
fn variant_client_list_users_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientListUsersError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientListUsersError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientListWorkspacesError

#[allow(dead_code)]
fn variant_client_list_workspaces_error_rs_to_js<'a>(
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

// ClientRenameWorkspaceError

#[allow(dead_code)]
fn variant_client_rename_workspace_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientRenameWorkspaceError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientRenameWorkspaceError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::UnknownWorkspace { .. } => {
            let js_tag = JsString::try_new(cx, "UnknownWorkspace").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientShareWorkspaceError

#[allow(dead_code)]
fn variant_client_share_workspace_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientShareWorkspaceError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientShareWorkspaceError::BadTimestamp {
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
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "serverTimestamp", js_server_timestamp)?;
            let js_client_timestamp = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "clientTimestamp", js_client_timestamp)?;
            let js_ballpark_client_early_offset = JsNumber::new(cx, ballpark_client_early_offset);
            js_obj.set(
                cx,
                "ballparkClientEarlyOffset",
                js_ballpark_client_early_offset,
            )?;
            let js_ballpark_client_late_offset = JsNumber::new(cx, ballpark_client_late_offset);
            js_obj.set(
                cx,
                "ballparkClientLateOffset",
                js_ballpark_client_late_offset,
            )?;
        }
        libparsec::ClientShareWorkspaceError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "Internal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::NotAllowed { .. } => {
            let js_tag = JsString::try_new(cx, "NotAllowed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "Offline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::OutsiderCannotBeManagerOrOwner { .. } => {
            let js_tag = JsString::try_new(cx, "OutsiderCannotBeManagerOrOwner").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::RevokedRecipient { .. } => {
            let js_tag = JsString::try_new(cx, "RevokedRecipient").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::ShareToSelf { .. } => {
            let js_tag = JsString::try_new(cx, "ShareToSelf").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::UnknownRecipient { .. } => {
            let js_tag = JsString::try_new(cx, "UnknownRecipient").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::UnknownRecipientOrWorkspace { .. } => {
            let js_tag = JsString::try_new(cx, "UnknownRecipientOrWorkspace").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::UnknownWorkspace { .. } => {
            let js_tag = JsString::try_new(cx, "UnknownWorkspace").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::WorkspaceInMaintenance { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceInMaintenance").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientStartError

#[allow(dead_code)]
fn variant_client_start_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientStartError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientStartError::DeviceAlreadyRunning { .. } => {
            let js_tag = JsString::try_new(cx, "DeviceAlreadyRunning").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
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

// ClientStartInvitationGreetError

#[allow(dead_code)]
fn variant_client_start_invitation_greet_error_rs_to_js<'a>(
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

// ClientStopError

#[allow(dead_code)]
fn variant_client_stop_error_rs_to_js<'a>(
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

// DeleteInvitationError

#[allow(dead_code)]
fn variant_delete_invitation_error_rs_to_js<'a>(
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

// DeviceAccessStrategy

#[allow(dead_code)]
fn variant_device_access_strategy_js_to_rs<'a>(
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
                let js_val: Handle<JsString> = obj.get(cx, "keyFile")?;
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
                let js_val: Handle<JsString> = obj.get(cx, "keyFile")?;
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
fn variant_device_access_strategy_rs_to_js<'a>(
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
            js_obj.set(cx, "keyFile", js_key_file)?;
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
            js_obj.set(cx, "keyFile", js_key_file)?;
        }
    }
    Ok(js_obj)
}

// DeviceSaveStrategy

#[allow(dead_code)]
fn variant_device_save_strategy_js_to_rs<'a>(
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
fn variant_device_save_strategy_rs_to_js<'a>(
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

// GreetInProgressError

#[allow(dead_code)]
fn variant_greet_in_progress_error_rs_to_js<'a>(
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
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "serverTimestamp", js_server_timestamp)?;
            let js_client_timestamp = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "clientTimestamp", js_client_timestamp)?;
            let js_ballpark_client_early_offset = JsNumber::new(cx, ballpark_client_early_offset);
            js_obj.set(
                cx,
                "ballparkClientEarlyOffset",
                js_ballpark_client_early_offset,
            )?;
            let js_ballpark_client_late_offset = JsNumber::new(cx, ballpark_client_late_offset);
            js_obj.set(
                cx,
                "ballparkClientLateOffset",
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

// InviteListItem

#[allow(dead_code)]
fn variant_invite_list_item_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::InviteListItem> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "Device" => {
            let token = {
                let js_val: Handle<JsString> = obj.get(cx, "token")?;
                {
                    let custom_from_rs_string =
                        |s: String| -> Result<libparsec::InvitationToken, _> {
                            libparsec::InvitationToken::from_hex(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_on = {
                let js_val: Handle<JsString> = obj.get(cx, "createdOn")?;
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
                let js_val: Handle<JsString> = obj.get(cx, "status")?;
                {
                    let js_string = js_val.value(cx);
                    enum_invitation_status_js_to_rs(cx, js_string.as_str())?
                }
            };
            Ok(libparsec::InviteListItem::Device {
                token,
                created_on,
                status,
            })
        }
        "User" => {
            let token = {
                let js_val: Handle<JsString> = obj.get(cx, "token")?;
                {
                    let custom_from_rs_string =
                        |s: String| -> Result<libparsec::InvitationToken, _> {
                            libparsec::InvitationToken::from_hex(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_on = {
                let js_val: Handle<JsString> = obj.get(cx, "createdOn")?;
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
                let js_val: Handle<JsString> = obj.get(cx, "claimerEmail")?;
                js_val.value(cx)
            };
            let status = {
                let js_val: Handle<JsString> = obj.get(cx, "status")?;
                {
                    let js_string = js_val.value(cx);
                    enum_invitation_status_js_to_rs(cx, js_string.as_str())?
                }
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
fn variant_invite_list_item_rs_to_js<'a>(
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
            let js_token = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "token", js_token)?;
            let js_created_on = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "createdOn", js_created_on)?;
            let js_status =
                JsString::try_new(cx, enum_invitation_status_rs_to_js(status)).or_throw(cx)?;
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
            let js_token = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "token", js_token)?;
            let js_created_on = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |dt: libparsec::DateTime| -> Result<String, &'static str> {
                        Ok(dt.to_rfc3339())
                    };
                match custom_to_rs_string(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "createdOn", js_created_on)?;
            let js_claimer_email = JsString::try_new(cx, claimer_email).or_throw(cx)?;
            js_obj.set(cx, "claimerEmail", js_claimer_email)?;
            let js_status =
                JsString::try_new(cx, enum_invitation_status_rs_to_js(status)).or_throw(cx)?;
            js_obj.set(cx, "status", js_status)?;
        }
    }
    Ok(js_obj)
}

// ListInvitationsError

#[allow(dead_code)]
fn variant_list_invitations_error_rs_to_js<'a>(
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

// NewDeviceInvitationError

#[allow(dead_code)]
fn variant_new_device_invitation_error_rs_to_js<'a>(
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

// NewUserInvitationError

#[allow(dead_code)]
fn variant_new_user_invitation_error_rs_to_js<'a>(
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

// ParseBackendAddrError

#[allow(dead_code)]
fn variant_parse_backend_addr_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ParseBackendAddrError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ParseBackendAddrError::InvalidUrl { .. } => {
            let js_tag = JsString::try_new(cx, "InvalidUrl").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ParsedBackendAddr

#[allow(dead_code)]
fn variant_parsed_backend_addr_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ParsedBackendAddr> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "Base" => {
            let hostname = {
                let js_val: Handle<JsString> = obj.get(cx, "hostname")?;
                js_val.value(cx)
            };
            let port = {
                let js_val: Handle<JsNumber> = obj.get(cx, "port")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    v as u32
                }
            };
            let use_ssl = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "useSsl")?;
                js_val.value(cx)
            };
            Ok(libparsec::ParsedBackendAddr::Base {
                hostname,
                port,
                use_ssl,
            })
        }
        "InvitationDevice" => {
            let hostname = {
                let js_val: Handle<JsString> = obj.get(cx, "hostname")?;
                js_val.value(cx)
            };
            let port = {
                let js_val: Handle<JsNumber> = obj.get(cx, "port")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    v as u32
                }
            };
            let use_ssl = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "useSsl")?;
                js_val.value(cx)
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
            let token = {
                let js_val: Handle<JsString> = obj.get(cx, "token")?;
                {
                    let custom_from_rs_string =
                        |s: String| -> Result<libparsec::InvitationToken, _> {
                            libparsec::InvitationToken::from_hex(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            Ok(libparsec::ParsedBackendAddr::InvitationDevice {
                hostname,
                port,
                use_ssl,
                organization_id,
                token,
            })
        }
        "InvitationUser" => {
            let hostname = {
                let js_val: Handle<JsString> = obj.get(cx, "hostname")?;
                js_val.value(cx)
            };
            let port = {
                let js_val: Handle<JsNumber> = obj.get(cx, "port")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    v as u32
                }
            };
            let use_ssl = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "useSsl")?;
                js_val.value(cx)
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
            let token = {
                let js_val: Handle<JsString> = obj.get(cx, "token")?;
                {
                    let custom_from_rs_string =
                        |s: String| -> Result<libparsec::InvitationToken, _> {
                            libparsec::InvitationToken::from_hex(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            Ok(libparsec::ParsedBackendAddr::InvitationUser {
                hostname,
                port,
                use_ssl,
                organization_id,
                token,
            })
        }
        "OrganizationBootstrap" => {
            let hostname = {
                let js_val: Handle<JsString> = obj.get(cx, "hostname")?;
                js_val.value(cx)
            };
            let port = {
                let js_val: Handle<JsNumber> = obj.get(cx, "port")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    v as u32
                }
            };
            let use_ssl = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "useSsl")?;
                js_val.value(cx)
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
            let token = {
                let js_val: Handle<JsValue> = obj.get(cx, "token")?;
                {
                    if js_val.is_a::<JsNull, _>(cx) {
                        None
                    } else {
                        let js_val = js_val.downcast_or_throw::<JsString, _>(cx)?;
                        Some(js_val.value(cx))
                    }
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
        "OrganizationFileLink" => {
            let hostname = {
                let js_val: Handle<JsString> = obj.get(cx, "hostname")?;
                js_val.value(cx)
            };
            let port = {
                let js_val: Handle<JsNumber> = obj.get(cx, "port")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    v as u32
                }
            };
            let use_ssl = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "useSsl")?;
                js_val.value(cx)
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
            let workspace_id = {
                let js_val: Handle<JsString> = obj.get(cx, "workspaceId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                        libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let encrypted_path = {
                let js_val: Handle<JsTypedArray<u8>> = obj.get(cx, "encryptedPath")?;
                js_val.as_slice(cx).to_vec()
            };
            let encrypted_timestamp = {
                let js_val: Handle<JsValue> = obj.get(cx, "encryptedTimestamp")?;
                {
                    if js_val.is_a::<JsNull, _>(cx) {
                        None
                    } else {
                        let js_val = js_val.downcast_or_throw::<JsTypedArray<u8>, _>(cx)?;
                        Some(js_val.as_slice(cx).to_vec())
                    }
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
        "PkiEnrollment" => {
            let hostname = {
                let js_val: Handle<JsString> = obj.get(cx, "hostname")?;
                js_val.value(cx)
            };
            let port = {
                let js_val: Handle<JsNumber> = obj.get(cx, "port")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    v as u32
                }
            };
            let use_ssl = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "useSsl")?;
                js_val.value(cx)
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
            Ok(libparsec::ParsedBackendAddr::PkiEnrollment {
                hostname,
                port,
                use_ssl,
                organization_id,
            })
        }
        _ => cx.throw_type_error("Object is not a ParsedBackendAddr"),
    }
}

#[allow(dead_code)]
fn variant_parsed_backend_addr_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ParsedBackendAddr,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::ParsedBackendAddr::Base {
            hostname,
            port,
            use_ssl,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "Base").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_hostname = JsString::try_new(cx, hostname).or_throw(cx)?;
            js_obj.set(cx, "hostname", js_hostname)?;
            let js_port = JsNumber::new(cx, port as f64);
            js_obj.set(cx, "port", js_port)?;
            let js_use_ssl = JsBoolean::new(cx, use_ssl);
            js_obj.set(cx, "useSsl", js_use_ssl)?;
        }
        libparsec::ParsedBackendAddr::InvitationDevice {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "InvitationDevice").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_hostname = JsString::try_new(cx, hostname).or_throw(cx)?;
            js_obj.set(cx, "hostname", js_hostname)?;
            let js_port = JsNumber::new(cx, port as f64);
            js_obj.set(cx, "port", js_port)?;
            let js_use_ssl = JsBoolean::new(cx, use_ssl);
            js_obj.set(cx, "useSsl", js_use_ssl)?;
            let js_organization_id = JsString::try_new(cx, organization_id).or_throw(cx)?;
            js_obj.set(cx, "organizationId", js_organization_id)?;
            let js_token = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "token", js_token)?;
        }
        libparsec::ParsedBackendAddr::InvitationUser {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "InvitationUser").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_hostname = JsString::try_new(cx, hostname).or_throw(cx)?;
            js_obj.set(cx, "hostname", js_hostname)?;
            let js_port = JsNumber::new(cx, port as f64);
            js_obj.set(cx, "port", js_port)?;
            let js_use_ssl = JsBoolean::new(cx, use_ssl);
            js_obj.set(cx, "useSsl", js_use_ssl)?;
            let js_organization_id = JsString::try_new(cx, organization_id).or_throw(cx)?;
            js_obj.set(cx, "organizationId", js_organization_id)?;
            let js_token = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "token", js_token)?;
        }
        libparsec::ParsedBackendAddr::OrganizationBootstrap {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "OrganizationBootstrap").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_hostname = JsString::try_new(cx, hostname).or_throw(cx)?;
            js_obj.set(cx, "hostname", js_hostname)?;
            let js_port = JsNumber::new(cx, port as f64);
            js_obj.set(cx, "port", js_port)?;
            let js_use_ssl = JsBoolean::new(cx, use_ssl);
            js_obj.set(cx, "useSsl", js_use_ssl)?;
            let js_organization_id = JsString::try_new(cx, organization_id).or_throw(cx)?;
            js_obj.set(cx, "organizationId", js_organization_id)?;
            let js_token = match token {
                Some(elem) => JsString::try_new(cx, elem).or_throw(cx)?.as_value(cx),
                None => JsNull::new(cx).as_value(cx),
            };
            js_obj.set(cx, "token", js_token)?;
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
            let js_tag = JsString::try_new(cx, "OrganizationFileLink").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_hostname = JsString::try_new(cx, hostname).or_throw(cx)?;
            js_obj.set(cx, "hostname", js_hostname)?;
            let js_port = JsNumber::new(cx, port as f64);
            js_obj.set(cx, "port", js_port)?;
            let js_use_ssl = JsBoolean::new(cx, use_ssl);
            js_obj.set(cx, "useSsl", js_use_ssl)?;
            let js_organization_id = JsString::try_new(cx, organization_id).or_throw(cx)?;
            js_obj.set(cx, "organizationId", js_organization_id)?;
            let js_workspace_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(workspace_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "workspaceId", js_workspace_id)?;
            let js_encrypted_path = {
                let mut js_buff = JsArrayBuffer::new(cx, encrypted_path.len())?;
                let js_buff_slice = js_buff.as_mut_slice(cx);
                for (i, c) in encrypted_path.iter().enumerate() {
                    js_buff_slice[i] = *c;
                }
                js_buff
            };
            js_obj.set(cx, "encryptedPath", js_encrypted_path)?;
            let js_encrypted_timestamp = match encrypted_timestamp {
                Some(elem) => {
                    let mut js_buff = JsArrayBuffer::new(cx, elem.len())?;
                    let js_buff_slice = js_buff.as_mut_slice(cx);
                    for (i, c) in elem.iter().enumerate() {
                        js_buff_slice[i] = *c;
                    }
                    js_buff
                }
                .as_value(cx),
                None => JsNull::new(cx).as_value(cx),
            };
            js_obj.set(cx, "encryptedTimestamp", js_encrypted_timestamp)?;
        }
        libparsec::ParsedBackendAddr::PkiEnrollment {
            hostname,
            port,
            use_ssl,
            organization_id,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "PkiEnrollment").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_hostname = JsString::try_new(cx, hostname).or_throw(cx)?;
            js_obj.set(cx, "hostname", js_hostname)?;
            let js_port = JsNumber::new(cx, port as f64);
            js_obj.set(cx, "port", js_port)?;
            let js_use_ssl = JsBoolean::new(cx, use_ssl);
            js_obj.set(cx, "useSsl", js_use_ssl)?;
            let js_organization_id = JsString::try_new(cx, organization_id).or_throw(cx)?;
            js_obj.set(cx, "organizationId", js_organization_id)?;
        }
    }
    Ok(js_obj)
}

// UserOrDeviceClaimInitialInfo

#[allow(dead_code)]
fn variant_user_or_device_claim_initial_info_js_to_rs<'a>(
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
                let js_val: Handle<JsString> = obj.get(cx, "greeterUserId")?;
                {
                    match js_val.value(cx).parse() {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let greeter_human_handle = {
                let js_val: Handle<JsValue> = obj.get(cx, "greeterHumanHandle")?;
                {
                    if js_val.is_a::<JsNull, _>(cx) {
                        None
                    } else {
                        let js_val = js_val.downcast_or_throw::<JsObject, _>(cx)?;
                        Some(struct_human_handle_js_to_rs(cx, js_val)?)
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
                let js_val: Handle<JsString> = obj.get(cx, "claimerEmail")?;
                js_val.value(cx)
            };
            let greeter_user_id = {
                let js_val: Handle<JsString> = obj.get(cx, "greeterUserId")?;
                {
                    match js_val.value(cx).parse() {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let greeter_human_handle = {
                let js_val: Handle<JsValue> = obj.get(cx, "greeterHumanHandle")?;
                {
                    if js_val.is_a::<JsNull, _>(cx) {
                        None
                    } else {
                        let js_val = js_val.downcast_or_throw::<JsObject, _>(cx)?;
                        Some(struct_human_handle_js_to_rs(cx, js_val)?)
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
fn variant_user_or_device_claim_initial_info_rs_to_js<'a>(
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
            js_obj.set(cx, "greeterUserId", js_greeter_user_id)?;
            let js_greeter_human_handle = match greeter_human_handle {
                Some(elem) => struct_human_handle_rs_to_js(cx, elem)?.as_value(cx),
                None => JsNull::new(cx).as_value(cx),
            };
            js_obj.set(cx, "greeterHumanHandle", js_greeter_human_handle)?;
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
            js_obj.set(cx, "claimerEmail", js_claimer_email)?;
            let js_greeter_user_id = JsString::try_new(cx, greeter_user_id).or_throw(cx)?;
            js_obj.set(cx, "greeterUserId", js_greeter_user_id)?;
            let js_greeter_human_handle = match greeter_human_handle {
                Some(elem) => struct_human_handle_rs_to_js(cx, elem)?.as_value(cx),
                None => JsNull::new(cx).as_value(cx),
            };
            js_obj.set(cx, "greeterHumanHandle", js_greeter_human_handle)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceStorageCacheSize

#[allow(dead_code)]
fn variant_workspace_storage_cache_size_js_to_rs<'a>(
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
fn variant_workspace_storage_cache_size_rs_to_js<'a>(
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

// bootstrap_organization
fn bootstrap_organization(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let config = {
        let js_val = cx.argument::<JsObject>(0)?;
        struct_client_config_js_to_rs(&mut cx, js_val)?
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
                let js_event = variant_client_event_rs_to_js(&mut cx, event)?;
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
        variant_device_save_strategy_js_to_rs(&mut cx, js_val)?
    };
    let human_handle = match cx.argument_opt(4) {
        Some(v) => match v.downcast::<JsObject, _>(&mut cx) {
            Ok(js_val) => Some(struct_human_handle_js_to_rs(&mut cx, js_val)?),
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
        Some(v) => {
            match v.downcast::<JsTypedArray<u8>, _>(&mut cx) {
                Ok(js_val) => {
                    Some({
                        #[allow(clippy::unnecessary_mut_passed)]
                        match js_val.as_slice(&mut cx).try_into() {
                            Ok(val) => val,
                            // err can't infer type in some case, because of the previous `try_into`
                            #[allow(clippy::useless_format)]
                            Err(err) => return cx.throw_type_error(format!("{}", err)),
                        }
                    })
                }
                Err(_) => None,
            }
        }
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
                        let js_value = struct_available_device_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_bootstrap_organization_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// build_backend_organization_bootstrap_addr
fn build_backend_organization_bootstrap_addr(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let addr = {
        let js_val = cx.argument::<JsString>(0)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::BackendAddr::from_any(&s).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let organization_id = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            match js_val.value(&mut cx).parse() {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let ret = libparsec::build_backend_organization_bootstrap_addr(addr, organization_id);
    let js_ret = JsString::try_new(&mut cx, {
        let custom_to_rs_string =
            |addr: libparsec::BackendOrganizationBootstrapAddr| -> Result<String, &'static str> {
                Ok(addr.to_url().into())
            };
        match custom_to_rs_string(ret) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(&mut cx)?;
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
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
                #[allow(clippy::let_unit_value)]
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
            let js_err = variant_cancel_error_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
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
        variant_device_save_strategy_js_to_rs(&mut cx, js_val)?
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
                        let js_value = struct_available_device_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claim_in_progress_error_rs_to_js(&mut cx, err)?;
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
                        let js_value = struct_device_claim_in_progress2_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claim_in_progress_error_rs_to_js(&mut cx, err)?;
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
                        let js_value = struct_device_claim_in_progress3_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claim_in_progress_error_rs_to_js(&mut cx, err)?;
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
                        let js_value = struct_device_claim_finalize_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claim_in_progress_error_rs_to_js(&mut cx, err)?;
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
                        let js_value = struct_device_claim_in_progress1_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claim_in_progress_error_rs_to_js(&mut cx, err)?;
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
                #[allow(clippy::let_unit_value)]
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
            let js_err = variant_claimer_greeter_abort_operation_error_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// claimer_retrieve_info
fn claimer_retrieve_info(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let config = {
        let js_val = cx.argument::<JsObject>(0)?;
        struct_client_config_js_to_rs(&mut cx, js_val)?
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
                let js_event = variant_client_event_rs_to_js(&mut cx, event)?;
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
                            variant_user_or_device_claim_initial_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claimer_retrieve_info_error_rs_to_js(&mut cx, err)?;
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
        variant_device_save_strategy_js_to_rs(&mut cx, js_val)?
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
                        let js_value = struct_available_device_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claim_in_progress_error_rs_to_js(&mut cx, err)?;
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
                        let js_value = struct_user_claim_in_progress2_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claim_in_progress_error_rs_to_js(&mut cx, err)?;
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
                        let js_value = struct_user_claim_in_progress3_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claim_in_progress_error_rs_to_js(&mut cx, err)?;
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
            Ok(js_val) => Some(struct_human_handle_js_to_rs(&mut cx, js_val)?),
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
                        let js_value = struct_user_claim_finalize_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claim_in_progress_error_rs_to_js(&mut cx, err)?;
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
                        let js_value = struct_user_claim_in_progress1_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_claim_in_progress_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_create_workspace
fn client_create_workspace(mut cx: FunctionContext) -> JsResult<JsPromise> {
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
            let ret = libparsec::client_create_workspace(client, name).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = JsString::try_new(&mut cx, {
                            let custom_to_rs_string =
                                |x: libparsec::VlobID| -> Result<String, &'static str> {
                                    Ok(x.hex())
                                };
                            match custom_to_rs_string(ok) {
                                Ok(ok) => ok,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        })
                        .or_throw(&mut cx)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_client_create_workspace_error_rs_to_js(&mut cx, err)?;
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
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::InvitationToken, _> {
                libparsec::InvitationToken::from_hex(s.as_str()).map_err(|e| e.to_string())
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
            let ret = libparsec::client_delete_invitation(client, token).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            #[allow(clippy::let_unit_value)]
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
                        let js_err = variant_delete_invitation_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_get_user_device
fn client_get_user_device(mut cx: FunctionContext) -> JsResult<JsPromise> {
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
    let device = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            match js_val.value(&mut cx).parse() {
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
            let ret = libparsec::client_get_user_device(client, device).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let (x1, x2) = ok;
                            let js_array = JsArray::new(&mut cx, 2);
                            let js_value = struct_user_info_rs_to_js(&mut cx, x1)?;
                            js_array.set(&mut cx, 1, js_value)?;
                            let js_value = struct_device_info_rs_to_js(&mut cx, x2)?;
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
                        let js_err = variant_client_get_user_device_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_info
fn client_info(mut cx: FunctionContext) -> JsResult<JsPromise> {
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
            let ret = libparsec::client_info(client).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_client_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_client_info_error_rs_to_js(&mut cx, err)?;
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
                                let js_elem = variant_invite_list_item_rs_to_js(&mut cx, elem)?;
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
                        let js_err = variant_list_invitations_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_list_user_devices
fn client_list_user_devices(mut cx: FunctionContext) -> JsResult<JsPromise> {
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
    let user = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            match js_val.value(&mut cx).parse() {
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
            let ret = libparsec::client_list_user_devices(client, user).await;

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
                                let js_elem = struct_device_info_rs_to_js(&mut cx, elem)?;
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
                        let js_err = variant_client_list_user_devices_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_list_users
fn client_list_users(mut cx: FunctionContext) -> JsResult<JsPromise> {
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
    let skip_revoked = {
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
            let ret = libparsec::client_list_users(client, skip_revoked).await;

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
                                let js_elem = struct_user_info_rs_to_js(&mut cx, elem)?;
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
                        let js_err = variant_client_list_users_error_rs_to_js(&mut cx, err)?;
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
                                let js_elem = struct_workspace_info_rs_to_js(&mut cx, elem)?;
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
                        let js_err = variant_client_list_workspaces_error_rs_to_js(&mut cx, err)?;
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
    let _handle = crate::TOKIO_RUNTIME.lock().expect("Mutex is poisoned").spawn(async move {

        let ret = libparsec::client_new_device_invitation(
            client,
            send_email,
        ).await;

        deferred.settle_with(&channel, move |mut cx| {
            let js_ret = match ret {
            Ok(ok) => {
                let js_obj = JsObject::new(&mut cx);
                let js_tag = JsBoolean::new(&mut cx, true);
                js_obj.set(&mut cx, "ok", js_tag)?;
                let js_value = {
                    let (x1, x2) = ok;
                    let js_array = JsArray::new(&mut cx, 2);
                    let js_value = JsString::try_new(&mut cx,{
                        let custom_to_rs_string = |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(x1) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    }).or_throw(&mut cx)?;
                    js_array.set(&mut cx, 1, js_value)?;
                    let js_value = JsString::try_new(&mut cx, enum_invitation_email_sent_status_rs_to_js(x2)).or_throw(&mut cx)?;
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
                let js_err = variant_new_device_invitation_error_rs_to_js(&mut cx, err)?;
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
    let _handle = crate::TOKIO_RUNTIME.lock().expect("Mutex is poisoned").spawn(async move {

        let ret = libparsec::client_new_user_invitation(
            client,
            claimer_email,
            send_email,
        ).await;

        deferred.settle_with(&channel, move |mut cx| {
            let js_ret = match ret {
            Ok(ok) => {
                let js_obj = JsObject::new(&mut cx);
                let js_tag = JsBoolean::new(&mut cx, true);
                js_obj.set(&mut cx, "ok", js_tag)?;
                let js_value = {
                    let (x1, x2) = ok;
                    let js_array = JsArray::new(&mut cx, 2);
                    let js_value = JsString::try_new(&mut cx,{
                        let custom_to_rs_string = |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(x1) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    }).or_throw(&mut cx)?;
                    js_array.set(&mut cx, 1, js_value)?;
                    let js_value = JsString::try_new(&mut cx, enum_invitation_email_sent_status_rs_to_js(x2)).or_throw(&mut cx)?;
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
                let js_err = variant_new_user_invitation_error_rs_to_js(&mut cx, err)?;
                js_obj.set(&mut cx, "error", js_err)?;
                js_obj
            }
        };
            Ok(js_ret)
        });
    });

    Ok(promise)
}

// client_rename_workspace
fn client_rename_workspace(mut cx: FunctionContext) -> JsResult<JsPromise> {
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
    let realm_id = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
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
            let ret = libparsec::client_rename_workspace(client, realm_id, new_name).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            #[allow(clippy::let_unit_value)]
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
                        let js_err = variant_client_rename_workspace_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_share_workspace
fn client_share_workspace(mut cx: FunctionContext) -> JsResult<JsPromise> {
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
    let realm_id = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
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
        Some(v) => match v.downcast::<JsString, _>(&mut cx) {
            Ok(js_val) => Some({
                let js_string = js_val.value(&mut cx);
                enum_realm_role_js_to_rs(&mut cx, js_string.as_str())?
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
            let ret = libparsec::client_share_workspace(client, realm_id, recipient, role).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            #[allow(clippy::let_unit_value)]
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
                        let js_err = variant_client_share_workspace_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_start
fn client_start(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let config = {
        let js_val = cx.argument::<JsObject>(0)?;
        struct_client_config_js_to_rs(&mut cx, js_val)?
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
                let js_event = variant_client_event_rs_to_js(&mut cx, event)?;
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
        variant_device_access_strategy_js_to_rs(&mut cx, js_val)?
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
                        let js_err = variant_client_start_error_rs_to_js(&mut cx, err)?;
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
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::InvitationToken, _> {
                libparsec::InvitationToken::from_hex(s.as_str()).map_err(|e| e.to_string())
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
            let ret = libparsec::client_start_device_invitation_greet(client, token).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_device_greet_initial_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_client_start_invitation_greet_error_rs_to_js(&mut cx, err)?;
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
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::InvitationToken, _> {
                libparsec::InvitationToken::from_hex(s.as_str()).map_err(|e| e.to_string())
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
            let ret = libparsec::client_start_user_invitation_greet(client, token).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_user_greet_initial_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_client_start_invitation_greet_error_rs_to_js(&mut cx, err)?;
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
                            #[allow(clippy::let_unit_value)]
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
                        let js_err = variant_client_stop_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// get_platform
fn get_platform(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let ret = libparsec::get_platform();
    let js_ret = JsString::try_new(&mut cx, enum_platform_rs_to_js(ret)).or_throw(&mut cx)?;
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
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
                        let js_value = struct_device_greet_in_progress2_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greet_in_progress_error_rs_to_js(&mut cx, err)?;
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
                        let js_value = struct_device_greet_in_progress3_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greet_in_progress_error_rs_to_js(&mut cx, err)?;
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
                        let js_value = struct_device_greet_in_progress4_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greet_in_progress_error_rs_to_js(&mut cx, err)?;
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
                            #[allow(clippy::let_unit_value)]
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
                        let js_err = variant_greet_in_progress_error_rs_to_js(&mut cx, err)?;
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
                        let js_value = struct_device_greet_in_progress1_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greet_in_progress_error_rs_to_js(&mut cx, err)?;
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
                        let js_value = struct_user_greet_in_progress2_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greet_in_progress_error_rs_to_js(&mut cx, err)?;
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
                        let js_value = struct_user_greet_in_progress3_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greet_in_progress_error_rs_to_js(&mut cx, err)?;
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
                        let js_value = struct_user_greet_in_progress4_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greet_in_progress_error_rs_to_js(&mut cx, err)?;
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
            Ok(js_val) => Some(struct_human_handle_js_to_rs(&mut cx, js_val)?),
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
        let js_val = cx.argument::<JsString>(4)?;
        {
            let js_string = js_val.value(&mut cx);
            enum_user_profile_js_to_rs(&mut cx, js_string.as_str())?
        }
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
                            #[allow(clippy::let_unit_value)]
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
                        let js_err = variant_greet_in_progress_error_rs_to_js(&mut cx, err)?;
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
                        let js_value = struct_user_greet_in_progress1_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_greet_in_progress_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

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
                        let js_elem = struct_available_device_rs_to_js(&mut cx, elem)?;
                        js_array.set(&mut cx, i as u32, js_elem)?;
                    }
                    js_array
                };
                Ok(js_ret)
            });
        });

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

// parse_backend_addr
fn parse_backend_addr(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let url = {
        let js_val = cx.argument::<JsString>(0)?;
        js_val.value(&mut cx)
    };
    let ret = libparsec::parse_backend_addr(&url);
    let js_ret = match ret {
        Ok(ok) => {
            let js_obj = JsObject::new(&mut cx);
            let js_tag = JsBoolean::new(&mut cx, true);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_value = variant_parsed_backend_addr_rs_to_js(&mut cx, ok)?;
            js_obj.set(&mut cx, "value", js_value)?;
            js_obj
        }
        Err(err) => {
            let js_obj = cx.empty_object();
            let js_tag = JsBoolean::new(&mut cx, false);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_err = variant_parse_backend_addr_error_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
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

// validate_device_label
fn validate_device_label(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let raw = {
        let js_val = cx.argument::<JsString>(0)?;
        js_val.value(&mut cx)
    };
    let ret = libparsec::validate_device_label(&raw);
    let js_ret = JsBoolean::new(&mut cx, ret);
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// validate_email
fn validate_email(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let raw = {
        let js_val = cx.argument::<JsString>(0)?;
        js_val.value(&mut cx)
    };
    let ret = libparsec::validate_email(&raw);
    let js_ret = JsBoolean::new(&mut cx, ret);
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// validate_entry_name
fn validate_entry_name(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let raw = {
        let js_val = cx.argument::<JsString>(0)?;
        js_val.value(&mut cx)
    };
    let ret = libparsec::validate_entry_name(&raw);
    let js_ret = JsBoolean::new(&mut cx, ret);
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// validate_human_handle_label
fn validate_human_handle_label(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let raw = {
        let js_val = cx.argument::<JsString>(0)?;
        js_val.value(&mut cx)
    };
    let ret = libparsec::validate_human_handle_label(&raw);
    let js_ret = JsBoolean::new(&mut cx, ret);
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// validate_invitation_token
fn validate_invitation_token(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let raw = {
        let js_val = cx.argument::<JsString>(0)?;
        js_val.value(&mut cx)
    };
    let ret = libparsec::validate_invitation_token(&raw);
    let js_ret = JsBoolean::new(&mut cx, ret);
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// validate_path
fn validate_path(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let raw = {
        let js_val = cx.argument::<JsString>(0)?;
        js_val.value(&mut cx)
    };
    let ret = libparsec::validate_path(&raw);
    let js_ret = JsBoolean::new(&mut cx, ret);
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

pub fn register_meths(cx: &mut ModuleContext) -> NeonResult<()> {
    cx.export_function("bootstrapOrganization", bootstrap_organization)?;
    cx.export_function(
        "buildBackendOrganizationBootstrapAddr",
        build_backend_organization_bootstrap_addr,
    )?;
    cx.export_function("cancel", cancel)?;
    cx.export_function(
        "claimerDeviceFinalizeSaveLocalDevice",
        claimer_device_finalize_save_local_device,
    )?;
    cx.export_function(
        "claimerDeviceInProgress1DoSignifyTrust",
        claimer_device_in_progress_1_do_signify_trust,
    )?;
    cx.export_function(
        "claimerDeviceInProgress2DoWaitPeerTrust",
        claimer_device_in_progress_2_do_wait_peer_trust,
    )?;
    cx.export_function(
        "claimerDeviceInProgress3DoClaim",
        claimer_device_in_progress_3_do_claim,
    )?;
    cx.export_function(
        "claimerDeviceInitialDoWaitPeer",
        claimer_device_initial_do_wait_peer,
    )?;
    cx.export_function(
        "claimerGreeterAbortOperation",
        claimer_greeter_abort_operation,
    )?;
    cx.export_function("claimerRetrieveInfo", claimer_retrieve_info)?;
    cx.export_function(
        "claimerUserFinalizeSaveLocalDevice",
        claimer_user_finalize_save_local_device,
    )?;
    cx.export_function(
        "claimerUserInProgress1DoSignifyTrust",
        claimer_user_in_progress_1_do_signify_trust,
    )?;
    cx.export_function(
        "claimerUserInProgress2DoWaitPeerTrust",
        claimer_user_in_progress_2_do_wait_peer_trust,
    )?;
    cx.export_function(
        "claimerUserInProgress3DoClaim",
        claimer_user_in_progress_3_do_claim,
    )?;
    cx.export_function(
        "claimerUserInitialDoWaitPeer",
        claimer_user_initial_do_wait_peer,
    )?;
    cx.export_function("clientCreateWorkspace", client_create_workspace)?;
    cx.export_function("clientDeleteInvitation", client_delete_invitation)?;
    cx.export_function("clientGetUserDevice", client_get_user_device)?;
    cx.export_function("clientInfo", client_info)?;
    cx.export_function("clientListInvitations", client_list_invitations)?;
    cx.export_function("clientListUserDevices", client_list_user_devices)?;
    cx.export_function("clientListUsers", client_list_users)?;
    cx.export_function("clientListWorkspaces", client_list_workspaces)?;
    cx.export_function("clientNewDeviceInvitation", client_new_device_invitation)?;
    cx.export_function("clientNewUserInvitation", client_new_user_invitation)?;
    cx.export_function("clientRenameWorkspace", client_rename_workspace)?;
    cx.export_function("clientShareWorkspace", client_share_workspace)?;
    cx.export_function("clientStart", client_start)?;
    cx.export_function(
        "clientStartDeviceInvitationGreet",
        client_start_device_invitation_greet,
    )?;
    cx.export_function(
        "clientStartUserInvitationGreet",
        client_start_user_invitation_greet,
    )?;
    cx.export_function("clientStop", client_stop)?;
    cx.export_function("getPlatform", get_platform)?;
    cx.export_function(
        "greeterDeviceInProgress1DoWaitPeerTrust",
        greeter_device_in_progress_1_do_wait_peer_trust,
    )?;
    cx.export_function(
        "greeterDeviceInProgress2DoSignifyTrust",
        greeter_device_in_progress_2_do_signify_trust,
    )?;
    cx.export_function(
        "greeterDeviceInProgress3DoGetClaimRequests",
        greeter_device_in_progress_3_do_get_claim_requests,
    )?;
    cx.export_function(
        "greeterDeviceInProgress4DoCreate",
        greeter_device_in_progress_4_do_create,
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
        "greeterUserInProgress2DoSignifyTrust",
        greeter_user_in_progress_2_do_signify_trust,
    )?;
    cx.export_function(
        "greeterUserInProgress3DoGetClaimRequests",
        greeter_user_in_progress_3_do_get_claim_requests,
    )?;
    cx.export_function(
        "greeterUserInProgress4DoCreate",
        greeter_user_in_progress_4_do_create,
    )?;
    cx.export_function(
        "greeterUserInitialDoWaitPeer",
        greeter_user_initial_do_wait_peer,
    )?;
    cx.export_function("listAvailableDevices", list_available_devices)?;
    cx.export_function("newCanceller", new_canceller)?;
    cx.export_function("parseBackendAddr", parse_backend_addr)?;
    cx.export_function("testDropTestbed", test_drop_testbed)?;
    cx.export_function(
        "testGetTestbedBootstrapOrganizationAddr",
        test_get_testbed_bootstrap_organization_addr,
    )?;
    cx.export_function(
        "testGetTestbedOrganizationId",
        test_get_testbed_organization_id,
    )?;
    cx.export_function("testNewTestbed", test_new_testbed)?;
    cx.export_function("validateDeviceLabel", validate_device_label)?;
    cx.export_function("validateEmail", validate_email)?;
    cx.export_function("validateEntryName", validate_entry_name)?;
    cx.export_function("validateHumanHandleLabel", validate_human_handle_label)?;
    cx.export_function("validateInvitationToken", validate_invitation_token)?;
    cx.export_function("validatePath", validate_path)?;
    Ok(())
}

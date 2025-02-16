// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

#[allow(unused_imports)]
use neon::{prelude::*, types::buffer::TypedArray};

// CancelledGreetingAttemptReason

#[allow(dead_code)]
fn enum_cancelled_greeting_attempt_reason_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    raw_value: &str,
) -> NeonResult<libparsec::CancelledGreetingAttemptReason> {
    match raw_value {
        "CancelledGreetingAttemptReasonAutomaticallyCancelled" => {
            Ok(libparsec::CancelledGreetingAttemptReason::AutomaticallyCancelled)
        }
        "CancelledGreetingAttemptReasonInconsistentPayload" => {
            Ok(libparsec::CancelledGreetingAttemptReason::InconsistentPayload)
        }
        "CancelledGreetingAttemptReasonInvalidNonceHash" => {
            Ok(libparsec::CancelledGreetingAttemptReason::InvalidNonceHash)
        }
        "CancelledGreetingAttemptReasonInvalidSasCode" => {
            Ok(libparsec::CancelledGreetingAttemptReason::InvalidSasCode)
        }
        "CancelledGreetingAttemptReasonManuallyCancelled" => {
            Ok(libparsec::CancelledGreetingAttemptReason::ManuallyCancelled)
        }
        "CancelledGreetingAttemptReasonUndecipherablePayload" => {
            Ok(libparsec::CancelledGreetingAttemptReason::UndecipherablePayload)
        }
        "CancelledGreetingAttemptReasonUndeserializablePayload" => {
            Ok(libparsec::CancelledGreetingAttemptReason::UndeserializablePayload)
        }
        _ => cx.throw_range_error(format!(
            "Invalid value `{raw_value}` for enum CancelledGreetingAttemptReason"
        )),
    }
}

#[allow(dead_code)]
fn enum_cancelled_greeting_attempt_reason_rs_to_js(
    value: libparsec::CancelledGreetingAttemptReason,
) -> &'static str {
    match value {
        libparsec::CancelledGreetingAttemptReason::AutomaticallyCancelled => {
            "CancelledGreetingAttemptReasonAutomaticallyCancelled"
        }
        libparsec::CancelledGreetingAttemptReason::InconsistentPayload => {
            "CancelledGreetingAttemptReasonInconsistentPayload"
        }
        libparsec::CancelledGreetingAttemptReason::InvalidNonceHash => {
            "CancelledGreetingAttemptReasonInvalidNonceHash"
        }
        libparsec::CancelledGreetingAttemptReason::InvalidSasCode => {
            "CancelledGreetingAttemptReasonInvalidSasCode"
        }
        libparsec::CancelledGreetingAttemptReason::ManuallyCancelled => {
            "CancelledGreetingAttemptReasonManuallyCancelled"
        }
        libparsec::CancelledGreetingAttemptReason::UndecipherablePayload => {
            "CancelledGreetingAttemptReasonUndecipherablePayload"
        }
        libparsec::CancelledGreetingAttemptReason::UndeserializablePayload => {
            "CancelledGreetingAttemptReasonUndeserializablePayload"
        }
    }
}

// DeviceFileType

#[allow(dead_code)]
fn enum_device_file_type_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    raw_value: &str,
) -> NeonResult<libparsec::DeviceFileType> {
    match raw_value {
        "DeviceFileTypeKeyring" => Ok(libparsec::DeviceFileType::Keyring),
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
        libparsec::DeviceFileType::Keyring => "DeviceFileTypeKeyring",
        libparsec::DeviceFileType::Password => "DeviceFileTypePassword",
        libparsec::DeviceFileType::Recovery => "DeviceFileTypeRecovery",
        libparsec::DeviceFileType::Smartcard => "DeviceFileTypeSmartcard",
    }
}

// DevicePurpose

#[allow(dead_code)]
fn enum_device_purpose_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    raw_value: &str,
) -> NeonResult<libparsec::DevicePurpose> {
    match raw_value {
        "DevicePurposePassphraseRecovery" => Ok(libparsec::DevicePurpose::PassphraseRecovery),
        "DevicePurposeShamirRecovery" => Ok(libparsec::DevicePurpose::ShamirRecovery),
        "DevicePurposeStandard" => Ok(libparsec::DevicePurpose::Standard),
        "DevicePurposeWebAuth" => Ok(libparsec::DevicePurpose::WebAuth),
        _ => cx.throw_range_error(format!(
            "Invalid value `{raw_value}` for enum DevicePurpose"
        )),
    }
}

#[allow(dead_code)]
fn enum_device_purpose_rs_to_js(value: libparsec::DevicePurpose) -> &'static str {
    match value {
        libparsec::DevicePurpose::PassphraseRecovery => "DevicePurposePassphraseRecovery",
        libparsec::DevicePurpose::ShamirRecovery => "DevicePurposeShamirRecovery",
        libparsec::DevicePurpose::Standard => "DevicePurposeStandard",
        libparsec::DevicePurpose::WebAuth => "DevicePurposeWebAuth",
    }
}

// GreeterOrClaimer

#[allow(dead_code)]
fn enum_greeter_or_claimer_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    raw_value: &str,
) -> NeonResult<libparsec::GreeterOrClaimer> {
    match raw_value {
        "GreeterOrClaimerClaimer" => Ok(libparsec::GreeterOrClaimer::Claimer),
        "GreeterOrClaimerGreeter" => Ok(libparsec::GreeterOrClaimer::Greeter),
        _ => cx.throw_range_error(format!(
            "Invalid value `{raw_value}` for enum GreeterOrClaimer"
        )),
    }
}

#[allow(dead_code)]
fn enum_greeter_or_claimer_rs_to_js(value: libparsec::GreeterOrClaimer) -> &'static str {
    match value {
        libparsec::GreeterOrClaimer::Claimer => "GreeterOrClaimerClaimer",
        libparsec::GreeterOrClaimer::Greeter => "GreeterOrClaimerGreeter",
    }
}

// InvitationEmailSentStatus

#[allow(dead_code)]
fn enum_invitation_email_sent_status_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    raw_value: &str,
) -> NeonResult<libparsec::InvitationEmailSentStatus> {
    match raw_value {
        "InvitationEmailSentStatusRecipientRefused" => {
            Ok(libparsec::InvitationEmailSentStatus::RecipientRefused)
        }
        "InvitationEmailSentStatusServerUnavailable" => {
            Ok(libparsec::InvitationEmailSentStatus::ServerUnavailable)
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
        libparsec::InvitationEmailSentStatus::RecipientRefused => {
            "InvitationEmailSentStatusRecipientRefused"
        }
        libparsec::InvitationEmailSentStatus::ServerUnavailable => {
            "InvitationEmailSentStatusServerUnavailable"
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
        "InvitationStatusCancelled" => Ok(libparsec::InvitationStatus::Cancelled),
        "InvitationStatusFinished" => Ok(libparsec::InvitationStatus::Finished),
        "InvitationStatusPending" => Ok(libparsec::InvitationStatus::Pending),
        _ => cx.throw_range_error(format!(
            "Invalid value `{raw_value}` for enum InvitationStatus"
        )),
    }
}

#[allow(dead_code)]
fn enum_invitation_status_rs_to_js(value: libparsec::InvitationStatus) -> &'static str {
    match value {
        libparsec::InvitationStatus::Cancelled => "InvitationStatusCancelled",
        libparsec::InvitationStatus::Finished => "InvitationStatusFinished",
        libparsec::InvitationStatus::Pending => "InvitationStatusPending",
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

// UserOnlineStatus

#[allow(dead_code)]
fn enum_user_online_status_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    raw_value: &str,
) -> NeonResult<libparsec::UserOnlineStatus> {
    match raw_value {
        "UserOnlineStatusOffline" => Ok(libparsec::UserOnlineStatus::Offline),
        "UserOnlineStatusOnline" => Ok(libparsec::UserOnlineStatus::Online),
        "UserOnlineStatusUnknown" => Ok(libparsec::UserOnlineStatus::Unknown),
        _ => cx.throw_range_error(format!(
            "Invalid value `{raw_value}` for enum UserOnlineStatus"
        )),
    }
}

#[allow(dead_code)]
fn enum_user_online_status_rs_to_js(value: libparsec::UserOnlineStatus) -> &'static str {
    match value {
        libparsec::UserOnlineStatus::Offline => "UserOnlineStatusOffline",
        libparsec::UserOnlineStatus::Online => "UserOnlineStatusOnline",
        libparsec::UserOnlineStatus::Unknown => "UserOnlineStatusUnknown",
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
    let created_on = {
        let js_val: Handle<JsNumber> = obj.get(cx, "createdOn")?;
        {
            let v = js_val.value(cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let protected_on = {
        let js_val: Handle<JsNumber> = obj.get(cx, "protectedOn")?;
        {
            let v = js_val.value(cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let server_url = {
        let js_val: Handle<JsString> = obj.get(cx, "serverUrl")?;
        js_val.value(cx)
    };
    let organization_id = {
        let js_val: Handle<JsString> = obj.get(cx, "organizationId")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let user_id = {
        let js_val: Handle<JsString> = obj.get(cx, "userId")?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let device_id = {
        let js_val: Handle<JsString> = obj.get(cx, "deviceId")?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let human_handle = {
        let js_val: Handle<JsObject> = obj.get(cx, "humanHandle")?;
        struct_human_handle_js_to_rs(cx, js_val)?
    };
    let device_label = {
        let js_val: Handle<JsString> = obj.get(cx, "deviceLabel")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
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
        created_on,
        protected_on,
        server_url,
        organization_id,
        user_id,
        device_id,
        human_handle,
        device_label,
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
    let js_created_on = JsNumber::new(cx, {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        match custom_to_rs_f64(rs_obj.created_on) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    });
    js_obj.set(cx, "createdOn", js_created_on)?;
    let js_protected_on = JsNumber::new(cx, {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        match custom_to_rs_f64(rs_obj.protected_on) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    });
    js_obj.set(cx, "protectedOn", js_protected_on)?;
    let js_server_url = JsString::try_new(cx, rs_obj.server_url).or_throw(cx)?;
    js_obj.set(cx, "serverUrl", js_server_url)?;
    let js_organization_id = JsString::try_new(cx, rs_obj.organization_id).or_throw(cx)?;
    js_obj.set(cx, "organizationId", js_organization_id)?;
    let js_user_id = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.user_id) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "userId", js_user_id)?;
    let js_device_id = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.device_id) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "deviceId", js_device_id)?;
    let js_human_handle = struct_human_handle_rs_to_js(cx, rs_obj.human_handle)?;
    js_obj.set(cx, "humanHandle", js_human_handle)?;
    let js_device_label = JsString::try_new(cx, rs_obj.device_label).or_throw(cx)?;
    js_obj.set(cx, "deviceLabel", js_device_label)?;
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
    let mountpoint_mount_strategy = {
        let js_val: Handle<JsObject> = obj.get(cx, "mountpointMountStrategy")?;
        variant_mountpoint_mount_strategy_js_to_rs(cx, js_val)?
    };
    let workspace_storage_cache_size = {
        let js_val: Handle<JsObject> = obj.get(cx, "workspaceStorageCacheSize")?;
        variant_workspace_storage_cache_size_js_to_rs(cx, js_val)?
    };
    let with_monitors = {
        let js_val: Handle<JsBoolean> = obj.get(cx, "withMonitors")?;
        js_val.value(cx)
    };
    let prevent_sync_pattern = {
        let js_val: Handle<JsValue> = obj.get(cx, "preventSyncPattern")?;
        {
            if js_val.is_a::<JsNull, _>(cx) {
                None
            } else {
                let js_val = js_val.downcast_or_throw::<JsString, _>(cx)?;
                Some(js_val.value(cx))
            }
        }
    };
    Ok(libparsec::ClientConfig {
        config_dir,
        data_base_dir,
        mountpoint_mount_strategy,
        workspace_storage_cache_size,
        with_monitors,
        prevent_sync_pattern,
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
    let js_mountpoint_mount_strategy =
        variant_mountpoint_mount_strategy_rs_to_js(cx, rs_obj.mountpoint_mount_strategy)?;
    js_obj.set(cx, "mountpointMountStrategy", js_mountpoint_mount_strategy)?;
    let js_workspace_storage_cache_size =
        variant_workspace_storage_cache_size_rs_to_js(cx, rs_obj.workspace_storage_cache_size)?;
    js_obj.set(
        cx,
        "workspaceStorageCacheSize",
        js_workspace_storage_cache_size,
    )?;
    let js_with_monitors = JsBoolean::new(cx, rs_obj.with_monitors);
    js_obj.set(cx, "withMonitors", js_with_monitors)?;
    let js_prevent_sync_pattern = match rs_obj.prevent_sync_pattern {
        Some(elem) => JsString::try_new(cx, elem).or_throw(cx)?.as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "preventSyncPattern", js_prevent_sync_pattern)?;
    Ok(js_obj)
}

// ClientInfo

#[allow(dead_code)]
fn struct_client_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ClientInfo> {
    let organization_addr = {
        let js_val: Handle<JsString> = obj.get(cx, "organizationAddr")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecOrganizationAddr::from_any(&s).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let organization_id = {
        let js_val: Handle<JsString> = obj.get(cx, "organizationId")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let device_id = {
        let js_val: Handle<JsString> = obj.get(cx, "deviceId")?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let user_id = {
        let js_val: Handle<JsString> = obj.get(cx, "userId")?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let device_label = {
        let js_val: Handle<JsString> = obj.get(cx, "deviceLabel")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let human_handle = {
        let js_val: Handle<JsObject> = obj.get(cx, "humanHandle")?;
        struct_human_handle_js_to_rs(cx, js_val)?
    };
    let current_profile = {
        let js_val: Handle<JsString> = obj.get(cx, "currentProfile")?;
        {
            let js_string = js_val.value(cx);
            enum_user_profile_js_to_rs(cx, js_string.as_str())?
        }
    };
    let server_config = {
        let js_val: Handle<JsObject> = obj.get(cx, "serverConfig")?;
        struct_server_config_js_to_rs(cx, js_val)?
    };
    Ok(libparsec::ClientInfo {
        organization_addr,
        organization_id,
        device_id,
        user_id,
        device_label,
        human_handle,
        current_profile,
        server_config,
    })
}

#[allow(dead_code)]
fn struct_client_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_organization_addr = JsString::try_new(cx, {
        let custom_to_rs_string =
            |addr: libparsec::ParsecOrganizationAddr| -> Result<String, &'static str> {
                Ok(addr.to_url().into())
            };
        match custom_to_rs_string(rs_obj.organization_addr) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "organizationAddr", js_organization_addr)?;
    let js_organization_id = JsString::try_new(cx, rs_obj.organization_id).or_throw(cx)?;
    js_obj.set(cx, "organizationId", js_organization_id)?;
    let js_device_id = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.device_id) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "deviceId", js_device_id)?;
    let js_user_id = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.user_id) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "userId", js_user_id)?;
    let js_device_label = JsString::try_new(cx, rs_obj.device_label).or_throw(cx)?;
    js_obj.set(cx, "deviceLabel", js_device_label)?;
    let js_human_handle = struct_human_handle_rs_to_js(cx, rs_obj.human_handle)?;
    js_obj.set(cx, "humanHandle", js_human_handle)?;
    let js_current_profile =
        JsString::try_new(cx, enum_user_profile_rs_to_js(rs_obj.current_profile)).or_throw(cx)?;
    js_obj.set(cx, "currentProfile", js_current_profile)?;
    let js_server_config = struct_server_config_rs_to_js(cx, rs_obj.server_config)?;
    js_obj.set(cx, "serverConfig", js_server_config)?;
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
            let v = v as u32;
            v
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
            let v = v as u32;
            v
        }
    };
    let greeter_user_id = {
        let js_val: Handle<JsString> = obj.get(cx, "greeterUserId")?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let greeter_human_handle = {
        let js_val: Handle<JsObject> = obj.get(cx, "greeterHumanHandle")?;
        struct_human_handle_js_to_rs(cx, js_val)?
    };
    let greeter_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "greeterSas")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
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
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_item.value(cx)) {
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
        greeter_user_id,
        greeter_human_handle,
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
    let js_greeter_user_id = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.greeter_user_id) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "greeterUserId", js_greeter_user_id)?;
    let js_greeter_human_handle = struct_human_handle_rs_to_js(cx, rs_obj.greeter_human_handle)?;
    js_obj.set(cx, "greeterHumanHandle", js_greeter_human_handle)?;
    let js_greeter_sas = JsString::try_new(cx, rs_obj.greeter_sas).or_throw(cx)?;
    js_obj.set(cx, "greeterSas", js_greeter_sas)?;
    let js_greeter_sas_choices = {
        // JsArray::new allocates with `undefined` value, that's why we `set` value
        let js_array = JsArray::new(cx, rs_obj.greeter_sas_choices.len());
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
            let v = v as u32;
            v
        }
    };
    let claimer_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "claimerSas")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
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
            let v = v as u32;
            v
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
            let v = v as u32;
            v
        }
    };
    let greeter_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "greeterSas")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
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
            let v = v as u32;
            v
        }
    };
    let claimer_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "claimerSas")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
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
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_item.value(cx)) {
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
        let js_array = JsArray::new(cx, rs_obj.claimer_sas_choices.len());
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
            let v = v as u32;
            v
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
            let v = v as u32;
            v
        }
    };
    let requested_device_label = {
        let js_val: Handle<JsString> = obj.get(cx, "requestedDeviceLabel")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
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
    let js_requested_device_label =
        JsString::try_new(cx, rs_obj.requested_device_label).or_throw(cx)?;
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
            let v = v as u32;
            v
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
            let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let purpose = {
        let js_val: Handle<JsString> = obj.get(cx, "purpose")?;
        {
            let js_string = js_val.value(cx);
            enum_device_purpose_js_to_rs(cx, js_string.as_str())?
        }
    };
    let device_label = {
        let js_val: Handle<JsString> = obj.get(cx, "deviceLabel")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let created_on = {
        let js_val: Handle<JsNumber> = obj.get(cx, "createdOn")?;
        {
            let v = js_val.value(cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
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
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                })
            }
        }
    };
    Ok(libparsec::DeviceInfo {
        id,
        purpose,
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
    let js_id = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.id) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "id", js_id)?;
    let js_purpose =
        JsString::try_new(cx, enum_device_purpose_rs_to_js(rs_obj.purpose)).or_throw(cx)?;
    js_obj.set(cx, "purpose", js_purpose)?;
    let js_device_label = JsString::try_new(cx, rs_obj.device_label).or_throw(cx)?;
    js_obj.set(cx, "deviceLabel", js_device_label)?;
    let js_created_on = JsNumber::new(cx, {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        match custom_to_rs_f64(rs_obj.created_on) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    });
    js_obj.set(cx, "createdOn", js_created_on)?;
    let js_created_by = match rs_obj.created_by {
        Some(elem) => JsString::try_new(cx, {
            let custom_to_rs_string =
                |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
            match custom_to_rs_string(elem) {
                Ok(ok) => ok,
                Err(err) => return cx.throw_type_error(err),
            }
        })
        .or_throw(cx)?
        .as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "createdBy", js_created_by)?;
    Ok(js_obj)
}

// FileStat

#[allow(dead_code)]
fn struct_file_stat_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::FileStat> {
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
    let created = {
        let js_val: Handle<JsNumber> = obj.get(cx, "created")?;
        {
            let v = js_val.value(cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let updated = {
        let js_val: Handle<JsNumber> = obj.get(cx, "updated")?;
        {
            let v = js_val.value(cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let base_version = {
        let js_val: Handle<JsNumber> = obj.get(cx, "baseVersion")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let is_placeholder = {
        let js_val: Handle<JsBoolean> = obj.get(cx, "isPlaceholder")?;
        js_val.value(cx)
    };
    let need_sync = {
        let js_val: Handle<JsBoolean> = obj.get(cx, "needSync")?;
        js_val.value(cx)
    };
    let size = {
        let js_val: Handle<JsNumber> = obj.get(cx, "size")?;
        {
            let v = js_val.value(cx);
            if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                cx.throw_type_error("Not an u64 number")?
            }
            let v = v as u64;
            v
        }
    };
    Ok(libparsec::FileStat {
        id,
        created,
        updated,
        base_version,
        is_placeholder,
        need_sync,
        size,
    })
}

#[allow(dead_code)]
fn struct_file_stat_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::FileStat,
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
    let js_created = JsNumber::new(cx, {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        match custom_to_rs_f64(rs_obj.created) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    });
    js_obj.set(cx, "created", js_created)?;
    let js_updated = JsNumber::new(cx, {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        match custom_to_rs_f64(rs_obj.updated) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    });
    js_obj.set(cx, "updated", js_updated)?;
    let js_base_version = JsNumber::new(cx, rs_obj.base_version as f64);
    js_obj.set(cx, "baseVersion", js_base_version)?;
    let js_is_placeholder = JsBoolean::new(cx, rs_obj.is_placeholder);
    js_obj.set(cx, "isPlaceholder", js_is_placeholder)?;
    let js_need_sync = JsBoolean::new(cx, rs_obj.need_sync);
    js_obj.set(cx, "needSync", js_need_sync)?;
    let js_size = JsNumber::new(cx, rs_obj.size as f64);
    js_obj.set(cx, "size", js_size)?;
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
    {
        let custom_init = |email: String, label: String| -> Result<_, String> {
            libparsec::HumanHandle::new(&email, &label).map_err(|e| e.to_string())
        };
        custom_init(email, label).or_else(|e| cx.throw_error(e))
    }
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

// NewInvitationInfo

#[allow(dead_code)]
fn struct_new_invitation_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::NewInvitationInfo> {
    let addr = {
        let js_val: Handle<JsString> = obj.get(cx, "addr")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecInvitationAddr::from_any(&s).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let token = {
        let js_val: Handle<JsString> = obj.get(cx, "token")?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::InvitationToken, _> {
                libparsec::InvitationToken::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let email_sent_status = {
        let js_val: Handle<JsString> = obj.get(cx, "emailSentStatus")?;
        {
            let js_string = js_val.value(cx);
            enum_invitation_email_sent_status_js_to_rs(cx, js_string.as_str())?
        }
    };
    Ok(libparsec::NewInvitationInfo {
        addr,
        token,
        email_sent_status,
    })
}

#[allow(dead_code)]
fn struct_new_invitation_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::NewInvitationInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_addr = JsString::try_new(cx, {
        let custom_to_rs_string =
            |addr: libparsec::ParsecInvitationAddr| -> Result<String, &'static str> {
                Ok(addr.to_url().into())
            };
        match custom_to_rs_string(rs_obj.addr) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "addr", js_addr)?;
    let js_token = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.token) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "token", js_token)?;
    let js_email_sent_status = JsString::try_new(
        cx,
        enum_invitation_email_sent_status_rs_to_js(rs_obj.email_sent_status),
    )
    .or_throw(cx)?;
    js_obj.set(cx, "emailSentStatus", js_email_sent_status)?;
    Ok(js_obj)
}

// OpenOptions

#[allow(dead_code)]
fn struct_open_options_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::OpenOptions> {
    let read = {
        let js_val: Handle<JsBoolean> = obj.get(cx, "read")?;
        js_val.value(cx)
    };
    let write = {
        let js_val: Handle<JsBoolean> = obj.get(cx, "write")?;
        js_val.value(cx)
    };
    let truncate = {
        let js_val: Handle<JsBoolean> = obj.get(cx, "truncate")?;
        js_val.value(cx)
    };
    let create = {
        let js_val: Handle<JsBoolean> = obj.get(cx, "create")?;
        js_val.value(cx)
    };
    let create_new = {
        let js_val: Handle<JsBoolean> = obj.get(cx, "createNew")?;
        js_val.value(cx)
    };
    Ok(libparsec::OpenOptions {
        read,
        write,
        truncate,
        create,
        create_new,
    })
}

#[allow(dead_code)]
fn struct_open_options_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::OpenOptions,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_read = JsBoolean::new(cx, rs_obj.read);
    js_obj.set(cx, "read", js_read)?;
    let js_write = JsBoolean::new(cx, rs_obj.write);
    js_obj.set(cx, "write", js_write)?;
    let js_truncate = JsBoolean::new(cx, rs_obj.truncate);
    js_obj.set(cx, "truncate", js_truncate)?;
    let js_create = JsBoolean::new(cx, rs_obj.create);
    js_obj.set(cx, "create", js_create)?;
    let js_create_new = JsBoolean::new(cx, rs_obj.create_new);
    js_obj.set(cx, "createNew", js_create_new)?;
    Ok(js_obj)
}

// ServerConfig

#[allow(dead_code)]
fn struct_server_config_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ServerConfig> {
    let user_profile_outsider_allowed = {
        let js_val: Handle<JsBoolean> = obj.get(cx, "userProfileOutsiderAllowed")?;
        js_val.value(cx)
    };
    let active_users_limit = {
        let js_val: Handle<JsObject> = obj.get(cx, "activeUsersLimit")?;
        variant_active_users_limit_js_to_rs(cx, js_val)?
    };
    Ok(libparsec::ServerConfig {
        user_profile_outsider_allowed,
        active_users_limit,
    })
}

#[allow(dead_code)]
fn struct_server_config_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ServerConfig,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_user_profile_outsider_allowed = JsBoolean::new(cx, rs_obj.user_profile_outsider_allowed);
    js_obj.set(
        cx,
        "userProfileOutsiderAllowed",
        js_user_profile_outsider_allowed,
    )?;
    let js_active_users_limit = variant_active_users_limit_rs_to_js(cx, rs_obj.active_users_limit)?;
    js_obj.set(cx, "activeUsersLimit", js_active_users_limit)?;
    Ok(js_obj)
}

// ShamirRecoveryClaimInProgress1Info

#[allow(dead_code)]
fn struct_shamir_recovery_claim_in_progress1_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ShamirRecoveryClaimInProgress1Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let greeter_user_id = {
        let js_val: Handle<JsString> = obj.get(cx, "greeterUserId")?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let greeter_human_handle = {
        let js_val: Handle<JsObject> = obj.get(cx, "greeterHumanHandle")?;
        struct_human_handle_js_to_rs(cx, js_val)?
    };
    let greeter_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "greeterSas")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
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
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_item.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                });
            }
            v
        }
    };
    Ok(libparsec::ShamirRecoveryClaimInProgress1Info {
        handle,
        greeter_user_id,
        greeter_human_handle,
        greeter_sas,
        greeter_sas_choices,
    })
}

#[allow(dead_code)]
fn struct_shamir_recovery_claim_in_progress1_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ShamirRecoveryClaimInProgress1Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_greeter_user_id = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.greeter_user_id) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "greeterUserId", js_greeter_user_id)?;
    let js_greeter_human_handle = struct_human_handle_rs_to_js(cx, rs_obj.greeter_human_handle)?;
    js_obj.set(cx, "greeterHumanHandle", js_greeter_human_handle)?;
    let js_greeter_sas = JsString::try_new(cx, rs_obj.greeter_sas).or_throw(cx)?;
    js_obj.set(cx, "greeterSas", js_greeter_sas)?;
    let js_greeter_sas_choices = {
        // JsArray::new allocates with `undefined` value, that's why we `set` value
        let js_array = JsArray::new(cx, rs_obj.greeter_sas_choices.len());
        for (i, elem) in rs_obj.greeter_sas_choices.into_iter().enumerate() {
            let js_elem = JsString::try_new(cx, elem).or_throw(cx)?;
            js_array.set(cx, i as u32, js_elem)?;
        }
        js_array
    };
    js_obj.set(cx, "greeterSasChoices", js_greeter_sas_choices)?;
    Ok(js_obj)
}

// ShamirRecoveryClaimInProgress2Info

#[allow(dead_code)]
fn struct_shamir_recovery_claim_in_progress2_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ShamirRecoveryClaimInProgress2Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let claimer_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "claimerSas")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    Ok(libparsec::ShamirRecoveryClaimInProgress2Info {
        handle,
        claimer_sas,
    })
}

#[allow(dead_code)]
fn struct_shamir_recovery_claim_in_progress2_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ShamirRecoveryClaimInProgress2Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_claimer_sas = JsString::try_new(cx, rs_obj.claimer_sas).or_throw(cx)?;
    js_obj.set(cx, "claimerSas", js_claimer_sas)?;
    Ok(js_obj)
}

// ShamirRecoveryClaimInProgress3Info

#[allow(dead_code)]
fn struct_shamir_recovery_claim_in_progress3_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ShamirRecoveryClaimInProgress3Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    Ok(libparsec::ShamirRecoveryClaimInProgress3Info { handle })
}

#[allow(dead_code)]
fn struct_shamir_recovery_claim_in_progress3_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ShamirRecoveryClaimInProgress3Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// ShamirRecoveryClaimInitialInfo

#[allow(dead_code)]
fn struct_shamir_recovery_claim_initial_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ShamirRecoveryClaimInitialInfo> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let greeter_user_id = {
        let js_val: Handle<JsString> = obj.get(cx, "greeterUserId")?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let greeter_human_handle = {
        let js_val: Handle<JsObject> = obj.get(cx, "greeterHumanHandle")?;
        struct_human_handle_js_to_rs(cx, js_val)?
    };
    Ok(libparsec::ShamirRecoveryClaimInitialInfo {
        handle,
        greeter_user_id,
        greeter_human_handle,
    })
}

#[allow(dead_code)]
fn struct_shamir_recovery_claim_initial_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ShamirRecoveryClaimInitialInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_greeter_user_id = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.greeter_user_id) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "greeterUserId", js_greeter_user_id)?;
    let js_greeter_human_handle = struct_human_handle_rs_to_js(cx, rs_obj.greeter_human_handle)?;
    js_obj.set(cx, "greeterHumanHandle", js_greeter_human_handle)?;
    Ok(js_obj)
}

// ShamirRecoveryClaimShareInfo

#[allow(dead_code)]
fn struct_shamir_recovery_claim_share_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ShamirRecoveryClaimShareInfo> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    Ok(libparsec::ShamirRecoveryClaimShareInfo { handle })
}

#[allow(dead_code)]
fn struct_shamir_recovery_claim_share_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ShamirRecoveryClaimShareInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// ShamirRecoveryGreetInProgress1Info

#[allow(dead_code)]
fn struct_shamir_recovery_greet_in_progress1_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ShamirRecoveryGreetInProgress1Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let greeter_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "greeterSas")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    Ok(libparsec::ShamirRecoveryGreetInProgress1Info {
        handle,
        greeter_sas,
    })
}

#[allow(dead_code)]
fn struct_shamir_recovery_greet_in_progress1_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ShamirRecoveryGreetInProgress1Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_greeter_sas = JsString::try_new(cx, rs_obj.greeter_sas).or_throw(cx)?;
    js_obj.set(cx, "greeterSas", js_greeter_sas)?;
    Ok(js_obj)
}

// ShamirRecoveryGreetInProgress2Info

#[allow(dead_code)]
fn struct_shamir_recovery_greet_in_progress2_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ShamirRecoveryGreetInProgress2Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let claimer_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "claimerSas")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
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
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_item.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                });
            }
            v
        }
    };
    Ok(libparsec::ShamirRecoveryGreetInProgress2Info {
        handle,
        claimer_sas,
        claimer_sas_choices,
    })
}

#[allow(dead_code)]
fn struct_shamir_recovery_greet_in_progress2_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ShamirRecoveryGreetInProgress2Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_claimer_sas = JsString::try_new(cx, rs_obj.claimer_sas).or_throw(cx)?;
    js_obj.set(cx, "claimerSas", js_claimer_sas)?;
    let js_claimer_sas_choices = {
        // JsArray::new allocates with `undefined` value, that's why we `set` value
        let js_array = JsArray::new(cx, rs_obj.claimer_sas_choices.len());
        for (i, elem) in rs_obj.claimer_sas_choices.into_iter().enumerate() {
            let js_elem = JsString::try_new(cx, elem).or_throw(cx)?;
            js_array.set(cx, i as u32, js_elem)?;
        }
        js_array
    };
    js_obj.set(cx, "claimerSasChoices", js_claimer_sas_choices)?;
    Ok(js_obj)
}

// ShamirRecoveryGreetInProgress3Info

#[allow(dead_code)]
fn struct_shamir_recovery_greet_in_progress3_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ShamirRecoveryGreetInProgress3Info> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    Ok(libparsec::ShamirRecoveryGreetInProgress3Info { handle })
}

#[allow(dead_code)]
fn struct_shamir_recovery_greet_in_progress3_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ShamirRecoveryGreetInProgress3Info,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// ShamirRecoveryGreetInitialInfo

#[allow(dead_code)]
fn struct_shamir_recovery_greet_initial_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ShamirRecoveryGreetInitialInfo> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    Ok(libparsec::ShamirRecoveryGreetInitialInfo { handle })
}

#[allow(dead_code)]
fn struct_shamir_recovery_greet_initial_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ShamirRecoveryGreetInitialInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    Ok(js_obj)
}

// ShamirRecoveryRecipient

#[allow(dead_code)]
fn struct_shamir_recovery_recipient_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ShamirRecoveryRecipient> {
    let user_id = {
        let js_val: Handle<JsString> = obj.get(cx, "userId")?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let human_handle = {
        let js_val: Handle<JsObject> = obj.get(cx, "humanHandle")?;
        struct_human_handle_js_to_rs(cx, js_val)?
    };
    let revoked_on = {
        let js_val: Handle<JsValue> = obj.get(cx, "revokedOn")?;
        {
            if js_val.is_a::<JsNull, _>(cx) {
                None
            } else {
                let js_val = js_val.downcast_or_throw::<JsNumber, _>(cx)?;
                Some({
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                })
            }
        }
    };
    let shares = {
        let js_val: Handle<JsNumber> = obj.get(cx, "shares")?;
        {
            let v = js_val.value(cx);
            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                cx.throw_type_error("Not an u8 number")?
            }
            let v = v as u8;
            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
            };
            match custom_from_rs_u8(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let online_status = {
        let js_val: Handle<JsString> = obj.get(cx, "onlineStatus")?;
        {
            let js_string = js_val.value(cx);
            enum_user_online_status_js_to_rs(cx, js_string.as_str())?
        }
    };
    Ok(libparsec::ShamirRecoveryRecipient {
        user_id,
        human_handle,
        revoked_on,
        shares,
        online_status,
    })
}

#[allow(dead_code)]
fn struct_shamir_recovery_recipient_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ShamirRecoveryRecipient,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_user_id = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.user_id) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "userId", js_user_id)?;
    let js_human_handle = struct_human_handle_rs_to_js(cx, rs_obj.human_handle)?;
    js_obj.set(cx, "humanHandle", js_human_handle)?;
    let js_revoked_on = match rs_obj.revoked_on {
        Some(elem) => JsNumber::new(cx, {
            let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
            };
            match custom_to_rs_f64(elem) {
                Ok(ok) => ok,
                Err(err) => return cx.throw_type_error(err),
            }
        })
        .as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "revokedOn", js_revoked_on)?;
    let js_shares = JsNumber::new(cx, {
        let custom_to_rs_u8 = |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
        match custom_to_rs_u8(rs_obj.shares) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    } as f64);
    js_obj.set(cx, "shares", js_shares)?;
    let js_online_status =
        JsString::try_new(cx, enum_user_online_status_rs_to_js(rs_obj.online_status))
            .or_throw(cx)?;
    js_obj.set(cx, "onlineStatus", js_online_status)?;
    Ok(js_obj)
}

// StartedWorkspaceInfo

#[allow(dead_code)]
fn struct_started_workspace_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::StartedWorkspaceInfo> {
    let client = {
        let js_val: Handle<JsNumber> = obj.get(cx, "client")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
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
    let current_name = {
        let js_val: Handle<JsString> = obj.get(cx, "currentName")?;
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
    let current_self_role = {
        let js_val: Handle<JsString> = obj.get(cx, "currentSelfRole")?;
        {
            let js_string = js_val.value(cx);
            enum_realm_role_js_to_rs(cx, js_string.as_str())?
        }
    };
    let mountpoints = {
        let js_val: Handle<JsArray> = obj.get(cx, "mountpoints")?;
        {
            let size = js_val.len(cx);
            let mut v = Vec::with_capacity(size as usize);
            for i in 0..size {
                let js_item: Handle<JsArray> = js_val.get(cx, i)?;
                v.push((
                    {
                        let js_item: Handle<JsNumber> = js_item.get(cx, 0)?;
                        {
                            let v = js_item.value(cx);
                            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                                cx.throw_type_error("Not an u32 number")?
                            }
                            let v = v as u32;
                            v
                        }
                    },
                    {
                        let js_item: Handle<JsString> = js_item.get(cx, 1)?;
                        {
                            let custom_from_rs_string = |s: String| -> Result<_, &'static str> {
                                Ok(std::path::PathBuf::from(s))
                            };
                            match custom_from_rs_string(js_item.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        }
                    },
                ));
            }
            v
        }
    };
    Ok(libparsec::StartedWorkspaceInfo {
        client,
        id,
        current_name,
        current_self_role,
        mountpoints,
    })
}

#[allow(dead_code)]
fn struct_started_workspace_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::StartedWorkspaceInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_client = JsNumber::new(cx, rs_obj.client as f64);
    js_obj.set(cx, "client", js_client)?;
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
    let js_current_name = JsString::try_new(cx, rs_obj.current_name).or_throw(cx)?;
    js_obj.set(cx, "currentName", js_current_name)?;
    let js_current_self_role =
        JsString::try_new(cx, enum_realm_role_rs_to_js(rs_obj.current_self_role)).or_throw(cx)?;
    js_obj.set(cx, "currentSelfRole", js_current_self_role)?;
    let js_mountpoints = {
        // JsArray::new allocates with `undefined` value, that's why we `set` value
        let js_array = JsArray::new(cx, rs_obj.mountpoints.len());
        for (i, elem) in rs_obj.mountpoints.into_iter().enumerate() {
            let js_elem = {
                let (x0, x1) = elem;
                let js_array = JsArray::new(cx, 2);
                let js_value = JsNumber::new(cx, x0 as f64);
                js_array.set(cx, 0, js_value)?;
                let js_value = JsString::try_new(cx, {
                    let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                        path.into_os_string()
                            .into_string()
                            .map_err(|_| "Path contains non-utf8 characters")
                    };
                    match custom_to_rs_string(x1) {
                        Ok(ok) => ok,
                        Err(err) => return cx.throw_type_error(err),
                    }
                })
                .or_throw(cx)?;
                js_array.set(cx, 1, js_value)?;
                js_array
            };
            js_array.set(cx, i as u32, js_elem)?;
        }
        js_array
    };
    js_obj.set(cx, "mountpoints", js_mountpoints)?;
    Ok(js_obj)
}

// Tos

#[allow(dead_code)]
fn struct_tos_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::Tos> {
    let per_locale_urls = {
        let js_val: Handle<JsObject> = obj.get(cx, "perLocaleUrls")?;
        {
            let mut d = std::collections::HashMap::with_capacity(
                js_val.get::<JsNumber, _, _>(cx, "size")?.value(cx) as usize,
            );

            let js_keys = js_val
                .call_method_with(cx, "keys")?
                .apply::<JsObject, _>(cx)?;
            let js_values = js_val
                .call_method_with(cx, "values")?
                .apply::<JsObject, _>(cx)?;
            let js_keys_next_cb = js_keys.call_method_with(cx, "next")?;
            let js_values_next_cb = js_values.call_method_with(cx, "next")?;

            loop {
                let next_js_key = js_keys_next_cb.apply::<JsObject, _>(cx)?;
                let next_js_value = js_values_next_cb.apply::<JsObject, _>(cx)?;

                let keys_done = next_js_key.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                let values_done = next_js_value.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                match (keys_done, values_done) {
                    (true, true) => break,
                    (false, false) => (),
                    _ => unreachable!(),
                }

                let js_key = next_js_key.get::<JsString, _, _>(cx, "value")?;
                let js_value = next_js_value.get::<JsString, _, _>(cx, "value")?;

                let key = js_key.value(cx);
                let value = js_value.value(cx);
                d.insert(key, value);
            }
            d
        }
    };
    let updated_on = {
        let js_val: Handle<JsNumber> = obj.get(cx, "updatedOn")?;
        {
            let v = js_val.value(cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    Ok(libparsec::Tos {
        per_locale_urls,
        updated_on,
    })
}

#[allow(dead_code)]
fn struct_tos_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::Tos,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_per_locale_urls = {
        let new_map_code = (cx).string("new Map()");
        let js_map = neon::reflect::eval(cx, new_map_code)?.downcast_or_throw::<JsObject, _>(cx)?;
        for (key, value) in rs_obj.per_locale_urls.into_iter() {
            let js_key = JsString::try_new(cx, key).or_throw(cx)?;
            let js_value = JsString::try_new(cx, value).or_throw(cx)?;
            js_map
                .call_method_with(cx, "set")?
                .arg(js_key)
                .arg(js_value)
                .exec(cx)?;
        }
        js_map
    };
    js_obj.set(cx, "perLocaleUrls", js_per_locale_urls)?;
    let js_updated_on = JsNumber::new(cx, {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        match custom_to_rs_f64(rs_obj.updated_on) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    });
    js_obj.set(cx, "updatedOn", js_updated_on)?;
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
            let v = v as u32;
            v
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
            let v = v as u32;
            v
        }
    };
    let greeter_user_id = {
        let js_val: Handle<JsString> = obj.get(cx, "greeterUserId")?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let greeter_human_handle = {
        let js_val: Handle<JsObject> = obj.get(cx, "greeterHumanHandle")?;
        struct_human_handle_js_to_rs(cx, js_val)?
    };
    let greeter_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "greeterSas")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
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
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_item.value(cx)) {
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
        greeter_user_id,
        greeter_human_handle,
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
    let js_greeter_user_id = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.greeter_user_id) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "greeterUserId", js_greeter_user_id)?;
    let js_greeter_human_handle = struct_human_handle_rs_to_js(cx, rs_obj.greeter_human_handle)?;
    js_obj.set(cx, "greeterHumanHandle", js_greeter_human_handle)?;
    let js_greeter_sas = JsString::try_new(cx, rs_obj.greeter_sas).or_throw(cx)?;
    js_obj.set(cx, "greeterSas", js_greeter_sas)?;
    let js_greeter_sas_choices = {
        // JsArray::new allocates with `undefined` value, that's why we `set` value
        let js_array = JsArray::new(cx, rs_obj.greeter_sas_choices.len());
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
            let v = v as u32;
            v
        }
    };
    let claimer_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "claimerSas")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
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
            let v = v as u32;
            v
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

// UserClaimInitialInfo

#[allow(dead_code)]
fn struct_user_claim_initial_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserClaimInitialInfo> {
    let handle = {
        let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let greeter_user_id = {
        let js_val: Handle<JsString> = obj.get(cx, "greeterUserId")?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let greeter_human_handle = {
        let js_val: Handle<JsObject> = obj.get(cx, "greeterHumanHandle")?;
        struct_human_handle_js_to_rs(cx, js_val)?
    };
    let online_status = {
        let js_val: Handle<JsString> = obj.get(cx, "onlineStatus")?;
        {
            let js_string = js_val.value(cx);
            enum_user_online_status_js_to_rs(cx, js_string.as_str())?
        }
    };
    let last_greeting_attempt_joined_on = {
        let js_val: Handle<JsValue> = obj.get(cx, "lastGreetingAttemptJoinedOn")?;
        {
            if js_val.is_a::<JsNull, _>(cx) {
                None
            } else {
                let js_val = js_val.downcast_or_throw::<JsNumber, _>(cx)?;
                Some({
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                })
            }
        }
    };
    Ok(libparsec::UserClaimInitialInfo {
        handle,
        greeter_user_id,
        greeter_human_handle,
        online_status,
        last_greeting_attempt_joined_on,
    })
}

#[allow(dead_code)]
fn struct_user_claim_initial_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserClaimInitialInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_handle = JsNumber::new(cx, rs_obj.handle as f64);
    js_obj.set(cx, "handle", js_handle)?;
    let js_greeter_user_id = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.greeter_user_id) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "greeterUserId", js_greeter_user_id)?;
    let js_greeter_human_handle = struct_human_handle_rs_to_js(cx, rs_obj.greeter_human_handle)?;
    js_obj.set(cx, "greeterHumanHandle", js_greeter_human_handle)?;
    let js_online_status =
        JsString::try_new(cx, enum_user_online_status_rs_to_js(rs_obj.online_status))
            .or_throw(cx)?;
    js_obj.set(cx, "onlineStatus", js_online_status)?;
    let js_last_greeting_attempt_joined_on = match rs_obj.last_greeting_attempt_joined_on {
        Some(elem) => JsNumber::new(cx, {
            let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
            };
            match custom_to_rs_f64(elem) {
                Ok(ok) => ok,
                Err(err) => return cx.throw_type_error(err),
            }
        })
        .as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(
        cx,
        "lastGreetingAttemptJoinedOn",
        js_last_greeting_attempt_joined_on,
    )?;
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
            let v = v as u32;
            v
        }
    };
    let greeter_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "greeterSas")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
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
            let v = v as u32;
            v
        }
    };
    let claimer_sas = {
        let js_val: Handle<JsString> = obj.get(cx, "claimerSas")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
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
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_item.value(cx)) {
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
        let js_array = JsArray::new(cx, rs_obj.claimer_sas_choices.len());
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
            let v = v as u32;
            v
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
            let v = v as u32;
            v
        }
    };
    let requested_human_handle = {
        let js_val: Handle<JsObject> = obj.get(cx, "requestedHumanHandle")?;
        struct_human_handle_js_to_rs(cx, js_val)?
    };
    let requested_device_label = {
        let js_val: Handle<JsString> = obj.get(cx, "requestedDeviceLabel")?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
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
    let js_requested_human_handle =
        struct_human_handle_rs_to_js(cx, rs_obj.requested_human_handle)?;
    js_obj.set(cx, "requestedHumanHandle", js_requested_human_handle)?;
    let js_requested_device_label =
        JsString::try_new(cx, rs_obj.requested_device_label).or_throw(cx)?;
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
            let v = v as u32;
            v
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

// UserGreetingAdministrator

#[allow(dead_code)]
fn struct_user_greeting_administrator_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::UserGreetingAdministrator> {
    let user_id = {
        let js_val: Handle<JsString> = obj.get(cx, "userId")?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let human_handle = {
        let js_val: Handle<JsObject> = obj.get(cx, "humanHandle")?;
        struct_human_handle_js_to_rs(cx, js_val)?
    };
    let online_status = {
        let js_val: Handle<JsString> = obj.get(cx, "onlineStatus")?;
        {
            let js_string = js_val.value(cx);
            enum_user_online_status_js_to_rs(cx, js_string.as_str())?
        }
    };
    let last_greeting_attempt_joined_on = {
        let js_val: Handle<JsValue> = obj.get(cx, "lastGreetingAttemptJoinedOn")?;
        {
            if js_val.is_a::<JsNull, _>(cx) {
                None
            } else {
                let js_val = js_val.downcast_or_throw::<JsNumber, _>(cx)?;
                Some({
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                })
            }
        }
    };
    Ok(libparsec::UserGreetingAdministrator {
        user_id,
        human_handle,
        online_status,
        last_greeting_attempt_joined_on,
    })
}

#[allow(dead_code)]
fn struct_user_greeting_administrator_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserGreetingAdministrator,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_user_id = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.user_id) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "userId", js_user_id)?;
    let js_human_handle = struct_human_handle_rs_to_js(cx, rs_obj.human_handle)?;
    js_obj.set(cx, "humanHandle", js_human_handle)?;
    let js_online_status =
        JsString::try_new(cx, enum_user_online_status_rs_to_js(rs_obj.online_status))
            .or_throw(cx)?;
    js_obj.set(cx, "onlineStatus", js_online_status)?;
    let js_last_greeting_attempt_joined_on = match rs_obj.last_greeting_attempt_joined_on {
        Some(elem) => JsNumber::new(cx, {
            let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
            };
            match custom_to_rs_f64(elem) {
                Ok(ok) => ok,
                Err(err) => return cx.throw_type_error(err),
            }
        })
        .as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(
        cx,
        "lastGreetingAttemptJoinedOn",
        js_last_greeting_attempt_joined_on,
    )?;
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
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let human_handle = {
        let js_val: Handle<JsObject> = obj.get(cx, "humanHandle")?;
        struct_human_handle_js_to_rs(cx, js_val)?
    };
    let current_profile = {
        let js_val: Handle<JsString> = obj.get(cx, "currentProfile")?;
        {
            let js_string = js_val.value(cx);
            enum_user_profile_js_to_rs(cx, js_string.as_str())?
        }
    };
    let created_on = {
        let js_val: Handle<JsNumber> = obj.get(cx, "createdOn")?;
        {
            let v = js_val.value(cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
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
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
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
                let js_val = js_val.downcast_or_throw::<JsNumber, _>(cx)?;
                Some({
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
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
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
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
    let js_id = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.id) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "id", js_id)?;
    let js_human_handle = struct_human_handle_rs_to_js(cx, rs_obj.human_handle)?;
    js_obj.set(cx, "humanHandle", js_human_handle)?;
    let js_current_profile =
        JsString::try_new(cx, enum_user_profile_rs_to_js(rs_obj.current_profile)).or_throw(cx)?;
    js_obj.set(cx, "currentProfile", js_current_profile)?;
    let js_created_on = JsNumber::new(cx, {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        match custom_to_rs_f64(rs_obj.created_on) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    });
    js_obj.set(cx, "createdOn", js_created_on)?;
    let js_created_by = match rs_obj.created_by {
        Some(elem) => JsString::try_new(cx, {
            let custom_to_rs_string =
                |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
            match custom_to_rs_string(elem) {
                Ok(ok) => ok,
                Err(err) => return cx.throw_type_error(err),
            }
        })
        .or_throw(cx)?
        .as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "createdBy", js_created_by)?;
    let js_revoked_on = match rs_obj.revoked_on {
        Some(elem) => JsNumber::new(cx, {
            let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
            };
            match custom_to_rs_f64(elem) {
                Ok(ok) => ok,
                Err(err) => return cx.throw_type_error(err),
            }
        })
        .as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "revokedOn", js_revoked_on)?;
    let js_revoked_by = match rs_obj.revoked_by {
        Some(elem) => JsString::try_new(cx, {
            let custom_to_rs_string =
                |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
            match custom_to_rs_string(elem) {
                Ok(ok) => ok,
                Err(err) => return cx.throw_type_error(err),
            }
        })
        .or_throw(cx)?
        .as_value(cx),
        None => JsNull::new(cx).as_value(cx),
    };
    js_obj.set(cx, "revokedBy", js_revoked_by)?;
    Ok(js_obj)
}

// WorkspaceHistory2FileStat

#[allow(dead_code)]
fn struct_workspace_history2_file_stat_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::WorkspaceHistory2FileStat> {
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
    let created = {
        let js_val: Handle<JsNumber> = obj.get(cx, "created")?;
        {
            let v = js_val.value(cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let updated = {
        let js_val: Handle<JsNumber> = obj.get(cx, "updated")?;
        {
            let v = js_val.value(cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let version = {
        let js_val: Handle<JsNumber> = obj.get(cx, "version")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let size = {
        let js_val: Handle<JsNumber> = obj.get(cx, "size")?;
        {
            let v = js_val.value(cx);
            if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                cx.throw_type_error("Not an u64 number")?
            }
            let v = v as u64;
            v
        }
    };
    Ok(libparsec::WorkspaceHistory2FileStat {
        id,
        created,
        updated,
        version,
        size,
    })
}

#[allow(dead_code)]
fn struct_workspace_history2_file_stat_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistory2FileStat,
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
    let js_created = JsNumber::new(cx, {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        match custom_to_rs_f64(rs_obj.created) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    });
    js_obj.set(cx, "created", js_created)?;
    let js_updated = JsNumber::new(cx, {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        match custom_to_rs_f64(rs_obj.updated) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    });
    js_obj.set(cx, "updated", js_updated)?;
    let js_version = JsNumber::new(cx, rs_obj.version as f64);
    js_obj.set(cx, "version", js_version)?;
    let js_size = JsNumber::new(cx, rs_obj.size as f64);
    js_obj.set(cx, "size", js_size)?;
    Ok(js_obj)
}

// WorkspaceHistoryFileStat

#[allow(dead_code)]
fn struct_workspace_history_file_stat_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::WorkspaceHistoryFileStat> {
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
    let created = {
        let js_val: Handle<JsNumber> = obj.get(cx, "created")?;
        {
            let v = js_val.value(cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let updated = {
        let js_val: Handle<JsNumber> = obj.get(cx, "updated")?;
        {
            let v = js_val.value(cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let version = {
        let js_val: Handle<JsNumber> = obj.get(cx, "version")?;
        {
            let v = js_val.value(cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let size = {
        let js_val: Handle<JsNumber> = obj.get(cx, "size")?;
        {
            let v = js_val.value(cx);
            if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                cx.throw_type_error("Not an u64 number")?
            }
            let v = v as u64;
            v
        }
    };
    Ok(libparsec::WorkspaceHistoryFileStat {
        id,
        created,
        updated,
        version,
        size,
    })
}

#[allow(dead_code)]
fn struct_workspace_history_file_stat_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistoryFileStat,
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
    let js_created = JsNumber::new(cx, {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        match custom_to_rs_f64(rs_obj.created) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    });
    js_obj.set(cx, "created", js_created)?;
    let js_updated = JsNumber::new(cx, {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        match custom_to_rs_f64(rs_obj.updated) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    });
    js_obj.set(cx, "updated", js_updated)?;
    let js_version = JsNumber::new(cx, rs_obj.version as f64);
    js_obj.set(cx, "version", js_version)?;
    let js_size = JsNumber::new(cx, rs_obj.size as f64);
    js_obj.set(cx, "size", js_size)?;
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
    let current_name = {
        let js_val: Handle<JsString> = obj.get(cx, "currentName")?;
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
    let current_self_role = {
        let js_val: Handle<JsString> = obj.get(cx, "currentSelfRole")?;
        {
            let js_string = js_val.value(cx);
            enum_realm_role_js_to_rs(cx, js_string.as_str())?
        }
    };
    let is_started = {
        let js_val: Handle<JsBoolean> = obj.get(cx, "isStarted")?;
        js_val.value(cx)
    };
    let is_bootstrapped = {
        let js_val: Handle<JsBoolean> = obj.get(cx, "isBootstrapped")?;
        js_val.value(cx)
    };
    Ok(libparsec::WorkspaceInfo {
        id,
        current_name,
        current_self_role,
        is_started,
        is_bootstrapped,
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
    let js_current_name = JsString::try_new(cx, rs_obj.current_name).or_throw(cx)?;
    js_obj.set(cx, "currentName", js_current_name)?;
    let js_current_self_role =
        JsString::try_new(cx, enum_realm_role_rs_to_js(rs_obj.current_self_role)).or_throw(cx)?;
    js_obj.set(cx, "currentSelfRole", js_current_self_role)?;
    let js_is_started = JsBoolean::new(cx, rs_obj.is_started);
    js_obj.set(cx, "isStarted", js_is_started)?;
    let js_is_bootstrapped = JsBoolean::new(cx, rs_obj.is_bootstrapped);
    js_obj.set(cx, "isBootstrapped", js_is_bootstrapped)?;
    Ok(js_obj)
}

// WorkspaceUserAccessInfo

#[allow(dead_code)]
fn struct_workspace_user_access_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::WorkspaceUserAccessInfo> {
    let user_id = {
        let js_val: Handle<JsString> = obj.get(cx, "userId")?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let human_handle = {
        let js_val: Handle<JsObject> = obj.get(cx, "humanHandle")?;
        struct_human_handle_js_to_rs(cx, js_val)?
    };
    let current_profile = {
        let js_val: Handle<JsString> = obj.get(cx, "currentProfile")?;
        {
            let js_string = js_val.value(cx);
            enum_user_profile_js_to_rs(cx, js_string.as_str())?
        }
    };
    let current_role = {
        let js_val: Handle<JsString> = obj.get(cx, "currentRole")?;
        {
            let js_string = js_val.value(cx);
            enum_realm_role_js_to_rs(cx, js_string.as_str())?
        }
    };
    Ok(libparsec::WorkspaceUserAccessInfo {
        user_id,
        human_handle,
        current_profile,
        current_role,
    })
}

#[allow(dead_code)]
fn struct_workspace_user_access_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceUserAccessInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_user_id = JsString::try_new(cx, {
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.user_id) {
            Ok(ok) => ok,
            Err(err) => return cx.throw_type_error(err),
        }
    })
    .or_throw(cx)?;
    js_obj.set(cx, "userId", js_user_id)?;
    let js_human_handle = struct_human_handle_rs_to_js(cx, rs_obj.human_handle)?;
    js_obj.set(cx, "humanHandle", js_human_handle)?;
    let js_current_profile =
        JsString::try_new(cx, enum_user_profile_rs_to_js(rs_obj.current_profile)).or_throw(cx)?;
    js_obj.set(cx, "currentProfile", js_current_profile)?;
    let js_current_role =
        JsString::try_new(cx, enum_realm_role_rs_to_js(rs_obj.current_role)).or_throw(cx)?;
    js_obj.set(cx, "currentRole", js_current_role)?;
    Ok(js_obj)
}

// ActiveUsersLimit

#[allow(dead_code)]
fn variant_active_users_limit_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ActiveUsersLimit> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "ActiveUsersLimitLimitedTo" => {
            let x0 = {
                let js_val: Handle<JsNumber> = obj.get(cx, "x0")?;
                {
                    let v = js_val.value(cx);
                    if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                        cx.throw_type_error("Not an u64 number")?
                    }
                    let v = v as u64;
                    v
                }
            };
            Ok(libparsec::ActiveUsersLimit::LimitedTo(x0))
        }
        "ActiveUsersLimitNoLimit" => Ok(libparsec::ActiveUsersLimit::NoLimit {}),
        _ => cx.throw_type_error("Object is not a ActiveUsersLimit"),
    }
}

#[allow(dead_code)]
fn variant_active_users_limit_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ActiveUsersLimit,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::ActiveUsersLimit::LimitedTo(x0, ..) => {
            let js_tag = JsString::try_new(cx, "LimitedTo").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_x0 = JsNumber::new(cx, x0 as f64);
            js_obj.set(cx, "x0", js_x0)?;
        }
        libparsec::ActiveUsersLimit::NoLimit { .. } => {
            let js_tag = JsString::try_new(cx, "ActiveUsersLimitNoLimit").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// AnyClaimRetrievedInfo

#[allow(dead_code)]
fn variant_any_claim_retrieved_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::AnyClaimRetrievedInfo> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "AnyClaimRetrievedInfoDevice" => {
            let handle = {
                let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    let v = v as u32;
                    v
                }
            };
            let greeter_user_id = {
                let js_val: Handle<JsString> = obj.get(cx, "greeterUserId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                        libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let greeter_human_handle = {
                let js_val: Handle<JsObject> = obj.get(cx, "greeterHumanHandle")?;
                struct_human_handle_js_to_rs(cx, js_val)?
            };
            Ok(libparsec::AnyClaimRetrievedInfo::Device {
                handle,
                greeter_user_id,
                greeter_human_handle,
            })
        }
        "AnyClaimRetrievedInfoShamirRecovery" => {
            let handle = {
                let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    let v = v as u32;
                    v
                }
            };
            let claimer_user_id = {
                let js_val: Handle<JsString> = obj.get(cx, "claimerUserId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                        libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let claimer_human_handle = {
                let js_val: Handle<JsObject> = obj.get(cx, "claimerHumanHandle")?;
                struct_human_handle_js_to_rs(cx, js_val)?
            };
            let invitation_created_by = {
                let js_val: Handle<JsObject> = obj.get(cx, "invitationCreatedBy")?;
                variant_invite_info_invitation_created_by_js_to_rs(cx, js_val)?
            };
            let shamir_recovery_created_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "shamirRecoveryCreatedOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let recipients = {
                let js_val: Handle<JsArray> = obj.get(cx, "recipients")?;
                {
                    let size = js_val.len(cx);
                    let mut v = Vec::with_capacity(size as usize);
                    for i in 0..size {
                        let js_item: Handle<JsObject> = js_val.get(cx, i)?;
                        v.push(struct_shamir_recovery_recipient_js_to_rs(cx, js_item)?);
                    }
                    v
                }
            };
            let threshold = {
                let js_val: Handle<JsNumber> = obj.get(cx, "threshold")?;
                {
                    let v = js_val.value(cx);
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        cx.throw_type_error("Not an u8 number")?
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let is_recoverable = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "isRecoverable")?;
                js_val.value(cx)
            };
            Ok(libparsec::AnyClaimRetrievedInfo::ShamirRecovery {
                handle,
                claimer_user_id,
                claimer_human_handle,
                invitation_created_by,
                shamir_recovery_created_on,
                recipients,
                threshold,
                is_recoverable,
            })
        }
        "AnyClaimRetrievedInfoUser" => {
            let handle = {
                let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    let v = v as u32;
                    v
                }
            };
            let claimer_email = {
                let js_val: Handle<JsString> = obj.get(cx, "claimerEmail")?;
                js_val.value(cx)
            };
            let created_by = {
                let js_val: Handle<JsObject> = obj.get(cx, "createdBy")?;
                variant_invite_info_invitation_created_by_js_to_rs(cx, js_val)?
            };
            let administrators = {
                let js_val: Handle<JsArray> = obj.get(cx, "administrators")?;
                {
                    let size = js_val.len(cx);
                    let mut v = Vec::with_capacity(size as usize);
                    for i in 0..size {
                        let js_item: Handle<JsObject> = js_val.get(cx, i)?;
                        v.push(struct_user_greeting_administrator_js_to_rs(cx, js_item)?);
                    }
                    v
                }
            };
            let preferred_greeter = {
                let js_val: Handle<JsValue> = obj.get(cx, "preferredGreeter")?;
                {
                    if js_val.is_a::<JsNull, _>(cx) {
                        None
                    } else {
                        let js_val = js_val.downcast_or_throw::<JsObject, _>(cx)?;
                        Some(struct_user_greeting_administrator_js_to_rs(cx, js_val)?)
                    }
                }
            };
            Ok(libparsec::AnyClaimRetrievedInfo::User {
                handle,
                claimer_email,
                created_by,
                administrators,
                preferred_greeter,
            })
        }
        _ => cx.throw_type_error("Object is not a AnyClaimRetrievedInfo"),
    }
}

#[allow(dead_code)]
fn variant_any_claim_retrieved_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::AnyClaimRetrievedInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::AnyClaimRetrievedInfo::Device {
            handle,
            greeter_user_id,
            greeter_human_handle,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "AnyClaimRetrievedInfoDevice").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_handle = JsNumber::new(cx, handle as f64);
            js_obj.set(cx, "handle", js_handle)?;
            let js_greeter_user_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(greeter_user_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "greeterUserId", js_greeter_user_id)?;
            let js_greeter_human_handle = struct_human_handle_rs_to_js(cx, greeter_human_handle)?;
            js_obj.set(cx, "greeterHumanHandle", js_greeter_human_handle)?;
        }
        libparsec::AnyClaimRetrievedInfo::ShamirRecovery {
            handle,
            claimer_user_id,
            claimer_human_handle,
            invitation_created_by,
            shamir_recovery_created_on,
            recipients,
            threshold,
            is_recoverable,
            ..
        } => {
            let js_tag =
                JsString::try_new(cx, "AnyClaimRetrievedInfoShamirRecovery").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_handle = JsNumber::new(cx, handle as f64);
            js_obj.set(cx, "handle", js_handle)?;
            let js_claimer_user_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(claimer_user_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "claimerUserId", js_claimer_user_id)?;
            let js_claimer_human_handle = struct_human_handle_rs_to_js(cx, claimer_human_handle)?;
            js_obj.set(cx, "claimerHumanHandle", js_claimer_human_handle)?;
            let js_invitation_created_by =
                variant_invite_info_invitation_created_by_rs_to_js(cx, invitation_created_by)?;
            js_obj.set(cx, "invitationCreatedBy", js_invitation_created_by)?;
            let js_shamir_recovery_created_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(shamir_recovery_created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "shamirRecoveryCreatedOn", js_shamir_recovery_created_on)?;
            let js_recipients = {
                // JsArray::new allocates with `undefined` value, that's why we `set` value
                let js_array = JsArray::new(cx, recipients.len());
                for (i, elem) in recipients.into_iter().enumerate() {
                    let js_elem = struct_shamir_recovery_recipient_rs_to_js(cx, elem)?;
                    js_array.set(cx, i as u32, js_elem)?;
                }
                js_array
            };
            js_obj.set(cx, "recipients", js_recipients)?;
            let js_threshold = JsNumber::new(cx, {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            } as f64);
            js_obj.set(cx, "threshold", js_threshold)?;
            let js_is_recoverable = JsBoolean::new(cx, is_recoverable);
            js_obj.set(cx, "isRecoverable", js_is_recoverable)?;
        }
        libparsec::AnyClaimRetrievedInfo::User {
            handle,
            claimer_email,
            created_by,
            administrators,
            preferred_greeter,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "AnyClaimRetrievedInfoUser").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_handle = JsNumber::new(cx, handle as f64);
            js_obj.set(cx, "handle", js_handle)?;
            let js_claimer_email = JsString::try_new(cx, claimer_email).or_throw(cx)?;
            js_obj.set(cx, "claimerEmail", js_claimer_email)?;
            let js_created_by = variant_invite_info_invitation_created_by_rs_to_js(cx, created_by)?;
            js_obj.set(cx, "createdBy", js_created_by)?;
            let js_administrators = {
                // JsArray::new allocates with `undefined` value, that's why we `set` value
                let js_array = JsArray::new(cx, administrators.len());
                for (i, elem) in administrators.into_iter().enumerate() {
                    let js_elem = struct_user_greeting_administrator_rs_to_js(cx, elem)?;
                    js_array.set(cx, i as u32, js_elem)?;
                }
                js_array
            };
            js_obj.set(cx, "administrators", js_administrators)?;
            let js_preferred_greeter = match preferred_greeter {
                Some(elem) => struct_user_greeting_administrator_rs_to_js(cx, elem)?.as_value(cx),
                None => JsNull::new(cx).as_value(cx),
            };
            js_obj.set(cx, "preferredGreeter", js_preferred_greeter)?;
        }
    }
    Ok(js_obj)
}

// ArchiveDeviceError

#[allow(dead_code)]
fn variant_archive_device_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ArchiveDeviceError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ArchiveDeviceError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "ArchiveDeviceErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
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
            let js_tag =
                JsString::try_new(cx, "BootstrapOrganizationErrorAlreadyUsedToken").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::BootstrapOrganizationError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "BootstrapOrganizationErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::BootstrapOrganizationError::InvalidToken { .. } => {
            let js_tag =
                JsString::try_new(cx, "BootstrapOrganizationErrorInvalidToken").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::BootstrapOrganizationError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "BootstrapOrganizationErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::BootstrapOrganizationError::OrganizationExpired { .. } => {
            let js_tag = JsString::try_new(cx, "BootstrapOrganizationErrorOrganizationExpired")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::BootstrapOrganizationError::SaveDeviceError { .. } => {
            let js_tag =
                JsString::try_new(cx, "BootstrapOrganizationErrorSaveDeviceError").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::BootstrapOrganizationError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "BootstrapOrganizationErrorTimestampOutOfBallpark")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_server_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "serverTimestamp", js_server_timestamp)?;
            let js_client_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
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
            let js_tag = JsString::try_new(cx, "CancelErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::CancelError::NotBound { .. } => {
            let js_tag = JsString::try_new(cx, "CancelErrorNotBound").or_throw(cx)?;
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
            let js_tag = JsString::try_new(cx, "ClaimInProgressErrorActiveUsersLimitReached")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::AlreadyUsedOrDeleted { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClaimInProgressErrorAlreadyUsedOrDeleted").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::Cancelled { .. } => {
            let js_tag = JsString::try_new(cx, "ClaimInProgressErrorCancelled").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::CorruptedConfirmation { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClaimInProgressErrorCorruptedConfirmation").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::GreeterNotAllowed { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClaimInProgressErrorGreeterNotAllowed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::GreetingAttemptCancelled {
            origin,
            reason,
            timestamp,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "ClaimInProgressErrorGreetingAttemptCancelled")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_origin =
                JsString::try_new(cx, enum_greeter_or_claimer_rs_to_js(origin)).or_throw(cx)?;
            js_obj.set(cx, "origin", js_origin)?;
            let js_reason =
                JsString::try_new(cx, enum_cancelled_greeting_attempt_reason_rs_to_js(reason))
                    .or_throw(cx)?;
            js_obj.set(cx, "reason", js_reason)?;
            let js_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "timestamp", js_timestamp)?;
        }
        libparsec::ClaimInProgressError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "ClaimInProgressErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::NotFound { .. } => {
            let js_tag = JsString::try_new(cx, "ClaimInProgressErrorNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "ClaimInProgressErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::OrganizationExpired { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClaimInProgressErrorOrganizationExpired").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimInProgressError::PeerReset { .. } => {
            let js_tag = JsString::try_new(cx, "ClaimInProgressErrorPeerReset").or_throw(cx)?;
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
            let js_tag =
                JsString::try_new(cx, "ClaimerGreeterAbortOperationErrorInternal").or_throw(cx)?;
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
        libparsec::ClaimerRetrieveInfoError::AlreadyUsedOrDeleted { .. } => {
            let js_tag = JsString::try_new(cx, "ClaimerRetrieveInfoErrorAlreadyUsedOrDeleted")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimerRetrieveInfoError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "ClaimerRetrieveInfoErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimerRetrieveInfoError::NotFound { .. } => {
            let js_tag = JsString::try_new(cx, "ClaimerRetrieveInfoErrorNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimerRetrieveInfoError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "ClaimerRetrieveInfoErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClaimerRetrieveInfoError::OrganizationExpired { .. } => {
            let js_tag = JsString::try_new(cx, "ClaimerRetrieveInfoErrorOrganizationExpired")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientAcceptTosError

#[allow(dead_code)]
fn variant_client_accept_tos_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientAcceptTosError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientAcceptTosError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "ClientAcceptTosErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientAcceptTosError::NoTos { .. } => {
            let js_tag = JsString::try_new(cx, "ClientAcceptTosErrorNoTos").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientAcceptTosError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "ClientAcceptTosErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientAcceptTosError::TosMismatch { .. } => {
            let js_tag = JsString::try_new(cx, "ClientAcceptTosErrorTosMismatch").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientCancelInvitationError

#[allow(dead_code)]
fn variant_client_cancel_invitation_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientCancelInvitationError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientCancelInvitationError::AlreadyCancelled { .. } => {
            let js_tag = JsString::try_new(cx, "ClientCancelInvitationErrorAlreadyCancelled")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientCancelInvitationError::Completed { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientCancelInvitationErrorCompleted").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientCancelInvitationError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientCancelInvitationErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientCancelInvitationError::NotAllowed { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientCancelInvitationErrorNotAllowed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientCancelInvitationError::NotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientCancelInvitationErrorNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientCancelInvitationError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientCancelInvitationErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientChangeAuthenticationError

#[allow(dead_code)]
fn variant_client_change_authentication_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientChangeAuthenticationError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientChangeAuthenticationError::DecryptionFailed { .. } => {
            let js_tag = JsString::try_new(cx, "ClientChangeAuthenticationErrorDecryptionFailed")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientChangeAuthenticationError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientChangeAuthenticationErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientChangeAuthenticationError::InvalidData { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientChangeAuthenticationErrorInvalidData").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientChangeAuthenticationError::InvalidPath { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientChangeAuthenticationErrorInvalidPath").or_throw(cx)?;
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
            let js_tag =
                JsString::try_new(cx, "ClientCreateWorkspaceErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientCreateWorkspaceError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "ClientCreateWorkspaceErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientDeleteShamirRecoveryError

#[allow(dead_code)]
fn variant_client_delete_shamir_recovery_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientDeleteShamirRecoveryError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientDeleteShamirRecoveryError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientDeleteShamirRecoveryErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientDeleteShamirRecoveryError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "ClientDeleteShamirRecoveryErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientDeleteShamirRecoveryError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientDeleteShamirRecoveryErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientDeleteShamirRecoveryError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientDeleteShamirRecoveryErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientDeleteShamirRecoveryError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let js_tag =
                JsString::try_new(cx, "ClientDeleteShamirRecoveryErrorTimestampOutOfBallpark")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_server_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "serverTimestamp", js_server_timestamp)?;
            let js_client_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
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
        "ClientEventExpiredOrganization" => Ok(libparsec::ClientEvent::ExpiredOrganization {}),
        "ClientEventGreetingAttemptCancelled" => {
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
            let greeting_attempt = {
                let js_val: Handle<JsString> = obj.get(cx, "greetingAttempt")?;
                {
                    let custom_from_rs_string =
                        |s: String| -> Result<libparsec::GreetingAttemptID, _> {
                            libparsec::GreetingAttemptID::from_hex(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            Ok(libparsec::ClientEvent::GreetingAttemptCancelled {
                token,
                greeting_attempt,
            })
        }
        "ClientEventGreetingAttemptJoined" => {
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
            let greeting_attempt = {
                let js_val: Handle<JsString> = obj.get(cx, "greetingAttempt")?;
                {
                    let custom_from_rs_string =
                        |s: String| -> Result<libparsec::GreetingAttemptID, _> {
                            libparsec::GreetingAttemptID::from_hex(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            Ok(libparsec::ClientEvent::GreetingAttemptJoined {
                token,
                greeting_attempt,
            })
        }
        "ClientEventGreetingAttemptReady" => {
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
            let greeting_attempt = {
                let js_val: Handle<JsString> = obj.get(cx, "greetingAttempt")?;
                {
                    let custom_from_rs_string =
                        |s: String| -> Result<libparsec::GreetingAttemptID, _> {
                            libparsec::GreetingAttemptID::from_hex(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            Ok(libparsec::ClientEvent::GreetingAttemptReady {
                token,
                greeting_attempt,
            })
        }
        "ClientEventIncompatibleServer" => {
            let detail = {
                let js_val: Handle<JsString> = obj.get(cx, "detail")?;
                js_val.value(cx)
            };
            Ok(libparsec::ClientEvent::IncompatibleServer { detail })
        }
        "ClientEventInvitationChanged" => {
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
            let status = {
                let js_val: Handle<JsString> = obj.get(cx, "status")?;
                {
                    let js_string = js_val.value(cx);
                    enum_invitation_status_js_to_rs(cx, js_string.as_str())?
                }
            };
            Ok(libparsec::ClientEvent::InvitationChanged { token, status })
        }
        "ClientEventMustAcceptTos" => Ok(libparsec::ClientEvent::MustAcceptTos {}),
        "ClientEventOffline" => Ok(libparsec::ClientEvent::Offline {}),
        "ClientEventOnline" => Ok(libparsec::ClientEvent::Online {}),
        "ClientEventPing" => {
            let ping = {
                let js_val: Handle<JsString> = obj.get(cx, "ping")?;
                js_val.value(cx)
            };
            Ok(libparsec::ClientEvent::Ping { ping })
        }
        "ClientEventRevokedSelfUser" => Ok(libparsec::ClientEvent::RevokedSelfUser {}),
        "ClientEventServerConfigChanged" => Ok(libparsec::ClientEvent::ServerConfigChanged {}),
        "ClientEventTooMuchDriftWithServerClock" => {
            let server_timestamp = {
                let js_val: Handle<JsNumber> = obj.get(cx, "serverTimestamp")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let client_timestamp = {
                let js_val: Handle<JsNumber> = obj.get(cx, "clientTimestamp")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let ballpark_client_early_offset = {
                let js_val: Handle<JsNumber> = obj.get(cx, "ballparkClientEarlyOffset")?;
                js_val.value(cx)
            };
            let ballpark_client_late_offset = {
                let js_val: Handle<JsNumber> = obj.get(cx, "ballparkClientLateOffset")?;
                js_val.value(cx)
            };
            Ok(libparsec::ClientEvent::TooMuchDriftWithServerClock {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            })
        }
        "ClientEventWorkspaceLocallyCreated" => {
            Ok(libparsec::ClientEvent::WorkspaceLocallyCreated {})
        }
        "ClientEventWorkspaceOpsInboundSyncDone" => {
            let realm_id = {
                let js_val: Handle<JsString> = obj.get(cx, "realmId")?;
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
            let entry_id = {
                let js_val: Handle<JsString> = obj.get(cx, "entryId")?;
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
            Ok(libparsec::ClientEvent::WorkspaceOpsInboundSyncDone { realm_id, entry_id })
        }
        "ClientEventWorkspaceOpsOutboundSyncAborted" => {
            let realm_id = {
                let js_val: Handle<JsString> = obj.get(cx, "realmId")?;
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
            let entry_id = {
                let js_val: Handle<JsString> = obj.get(cx, "entryId")?;
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
            Ok(libparsec::ClientEvent::WorkspaceOpsOutboundSyncAborted { realm_id, entry_id })
        }
        "ClientEventWorkspaceOpsOutboundSyncDone" => {
            let realm_id = {
                let js_val: Handle<JsString> = obj.get(cx, "realmId")?;
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
            let entry_id = {
                let js_val: Handle<JsString> = obj.get(cx, "entryId")?;
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
            Ok(libparsec::ClientEvent::WorkspaceOpsOutboundSyncDone { realm_id, entry_id })
        }
        "ClientEventWorkspaceOpsOutboundSyncProgress" => {
            let realm_id = {
                let js_val: Handle<JsString> = obj.get(cx, "realmId")?;
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
            let entry_id = {
                let js_val: Handle<JsString> = obj.get(cx, "entryId")?;
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
            let blocks = {
                let js_val: Handle<JsNumber> = obj.get(cx, "blocks")?;
                {
                    let v = js_val.value(cx);
                    if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                        cx.throw_type_error("Not an u64 number")?
                    }
                    let v = v as u64;
                    v
                }
            };
            let block_index = {
                let js_val: Handle<JsNumber> = obj.get(cx, "blockIndex")?;
                {
                    let v = js_val.value(cx);
                    if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                        cx.throw_type_error("Not an u64 number")?
                    }
                    let v = v as u64;
                    v
                }
            };
            let blocksize = {
                let js_val: Handle<JsNumber> = obj.get(cx, "blocksize")?;
                {
                    let v = js_val.value(cx);
                    if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                        cx.throw_type_error("Not an u64 number")?
                    }
                    let v = v as u64;
                    v
                }
            };
            Ok(libparsec::ClientEvent::WorkspaceOpsOutboundSyncProgress {
                realm_id,
                entry_id,
                blocks,
                block_index,
                blocksize,
            })
        }
        "ClientEventWorkspaceOpsOutboundSyncStarted" => {
            let realm_id = {
                let js_val: Handle<JsString> = obj.get(cx, "realmId")?;
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
            let entry_id = {
                let js_val: Handle<JsString> = obj.get(cx, "entryId")?;
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
            Ok(libparsec::ClientEvent::WorkspaceOpsOutboundSyncStarted { realm_id, entry_id })
        }
        "ClientEventWorkspaceWatchedEntryChanged" => {
            let realm_id = {
                let js_val: Handle<JsString> = obj.get(cx, "realmId")?;
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
            let entry_id = {
                let js_val: Handle<JsString> = obj.get(cx, "entryId")?;
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
            Ok(libparsec::ClientEvent::WorkspaceWatchedEntryChanged { realm_id, entry_id })
        }
        "ClientEventWorkspacesSelfListChanged" => {
            Ok(libparsec::ClientEvent::WorkspacesSelfListChanged {})
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
        libparsec::ClientEvent::ExpiredOrganization { .. } => {
            let js_tag = JsString::try_new(cx, "ClientEventExpiredOrganization").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientEvent::GreetingAttemptCancelled {
            token,
            greeting_attempt,
            ..
        } => {
            let js_tag =
                JsString::try_new(cx, "ClientEventGreetingAttemptCancelled").or_throw(cx)?;
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
            let js_greeting_attempt = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::GreetingAttemptID| -> Result<String, &'static str> {
                        Ok(x.hex())
                    };
                match custom_to_rs_string(greeting_attempt) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "greetingAttempt", js_greeting_attempt)?;
        }
        libparsec::ClientEvent::GreetingAttemptJoined {
            token,
            greeting_attempt,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "ClientEventGreetingAttemptJoined").or_throw(cx)?;
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
            let js_greeting_attempt = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::GreetingAttemptID| -> Result<String, &'static str> {
                        Ok(x.hex())
                    };
                match custom_to_rs_string(greeting_attempt) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "greetingAttempt", js_greeting_attempt)?;
        }
        libparsec::ClientEvent::GreetingAttemptReady {
            token,
            greeting_attempt,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "ClientEventGreetingAttemptReady").or_throw(cx)?;
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
            let js_greeting_attempt = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::GreetingAttemptID| -> Result<String, &'static str> {
                        Ok(x.hex())
                    };
                match custom_to_rs_string(greeting_attempt) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "greetingAttempt", js_greeting_attempt)?;
        }
        libparsec::ClientEvent::IncompatibleServer { detail, .. } => {
            let js_tag = JsString::try_new(cx, "ClientEventIncompatibleServer").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_detail = JsString::try_new(cx, detail).or_throw(cx)?;
            js_obj.set(cx, "detail", js_detail)?;
        }
        libparsec::ClientEvent::InvitationChanged { token, status, .. } => {
            let js_tag = JsString::try_new(cx, "ClientEventInvitationChanged").or_throw(cx)?;
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
            let js_status =
                JsString::try_new(cx, enum_invitation_status_rs_to_js(status)).or_throw(cx)?;
            js_obj.set(cx, "status", js_status)?;
        }
        libparsec::ClientEvent::MustAcceptTos { .. } => {
            let js_tag = JsString::try_new(cx, "ClientEventMustAcceptTos").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientEvent::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "ClientEventOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientEvent::Online { .. } => {
            let js_tag = JsString::try_new(cx, "ClientEventOnline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientEvent::Ping { ping, .. } => {
            let js_tag = JsString::try_new(cx, "ClientEventPing").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_ping = JsString::try_new(cx, ping).or_throw(cx)?;
            js_obj.set(cx, "ping", js_ping)?;
        }
        libparsec::ClientEvent::RevokedSelfUser { .. } => {
            let js_tag = JsString::try_new(cx, "ClientEventRevokedSelfUser").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientEvent::ServerConfigChanged { .. } => {
            let js_tag = JsString::try_new(cx, "ClientEventServerConfigChanged").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientEvent::TooMuchDriftWithServerClock {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let js_tag =
                JsString::try_new(cx, "ClientEventTooMuchDriftWithServerClock").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_server_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "serverTimestamp", js_server_timestamp)?;
            let js_client_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
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
        libparsec::ClientEvent::WorkspaceLocallyCreated { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientEventWorkspaceLocallyCreated").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientEvent::WorkspaceOpsInboundSyncDone {
            realm_id, entry_id, ..
        } => {
            let js_tag =
                JsString::try_new(cx, "ClientEventWorkspaceOpsInboundSyncDone").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_realm_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(realm_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "realmId", js_realm_id)?;
            let js_entry_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(entry_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "entryId", js_entry_id)?;
        }
        libparsec::ClientEvent::WorkspaceOpsOutboundSyncAborted {
            realm_id, entry_id, ..
        } => {
            let js_tag =
                JsString::try_new(cx, "ClientEventWorkspaceOpsOutboundSyncAborted").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_realm_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(realm_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "realmId", js_realm_id)?;
            let js_entry_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(entry_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "entryId", js_entry_id)?;
        }
        libparsec::ClientEvent::WorkspaceOpsOutboundSyncDone {
            realm_id, entry_id, ..
        } => {
            let js_tag =
                JsString::try_new(cx, "ClientEventWorkspaceOpsOutboundSyncDone").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_realm_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(realm_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "realmId", js_realm_id)?;
            let js_entry_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(entry_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "entryId", js_entry_id)?;
        }
        libparsec::ClientEvent::WorkspaceOpsOutboundSyncProgress {
            realm_id,
            entry_id,
            blocks,
            block_index,
            blocksize,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "ClientEventWorkspaceOpsOutboundSyncProgress")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_realm_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(realm_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "realmId", js_realm_id)?;
            let js_entry_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(entry_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "entryId", js_entry_id)?;
            let js_blocks = JsNumber::new(cx, blocks as f64);
            js_obj.set(cx, "blocks", js_blocks)?;
            let js_block_index = JsNumber::new(cx, block_index as f64);
            js_obj.set(cx, "blockIndex", js_block_index)?;
            let js_blocksize = JsNumber::new(cx, blocksize as f64);
            js_obj.set(cx, "blocksize", js_blocksize)?;
        }
        libparsec::ClientEvent::WorkspaceOpsOutboundSyncStarted {
            realm_id, entry_id, ..
        } => {
            let js_tag =
                JsString::try_new(cx, "ClientEventWorkspaceOpsOutboundSyncStarted").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_realm_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(realm_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "realmId", js_realm_id)?;
            let js_entry_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(entry_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "entryId", js_entry_id)?;
        }
        libparsec::ClientEvent::WorkspaceWatchedEntryChanged {
            realm_id, entry_id, ..
        } => {
            let js_tag =
                JsString::try_new(cx, "ClientEventWorkspaceWatchedEntryChanged").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_realm_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(realm_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "realmId", js_realm_id)?;
            let js_entry_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(entry_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "entryId", js_entry_id)?;
        }
        libparsec::ClientEvent::WorkspacesSelfListChanged { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientEventWorkspacesSelfListChanged").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientExportRecoveryDeviceError

#[allow(dead_code)]
fn variant_client_export_recovery_device_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientExportRecoveryDeviceError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientExportRecoveryDeviceError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientExportRecoveryDeviceErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientExportRecoveryDeviceError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "ClientExportRecoveryDeviceErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientExportRecoveryDeviceError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientExportRecoveryDeviceErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientExportRecoveryDeviceError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientExportRecoveryDeviceErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientExportRecoveryDeviceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let js_tag =
                JsString::try_new(cx, "ClientExportRecoveryDeviceErrorTimestampOutOfBallpark")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_server_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "serverTimestamp", js_server_timestamp)?;
            let js_client_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
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
    }
    Ok(js_obj)
}

// ClientGetSelfShamirRecoveryError

#[allow(dead_code)]
fn variant_client_get_self_shamir_recovery_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientGetSelfShamirRecoveryError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientGetSelfShamirRecoveryError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientGetSelfShamirRecoveryErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientGetSelfShamirRecoveryError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientGetSelfShamirRecoveryErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientGetTosError

#[allow(dead_code)]
fn variant_client_get_tos_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientGetTosError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientGetTosError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "ClientGetTosErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientGetTosError::NoTos { .. } => {
            let js_tag = JsString::try_new(cx, "ClientGetTosErrorNoTos").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientGetTosError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "ClientGetTosErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
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
            let js_tag = JsString::try_new(cx, "ClientGetUserDeviceErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientGetUserDeviceError::NonExisting { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientGetUserDeviceErrorNonExisting").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientGetUserDeviceError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "ClientGetUserDeviceErrorStopped").or_throw(cx)?;
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
            let js_tag = JsString::try_new(cx, "ClientInfoErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientInfoError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "ClientInfoErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientListFrozenUsersError

#[allow(dead_code)]
fn variant_client_list_frozen_users_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientListFrozenUsersError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientListFrozenUsersError::AuthorNotAllowed { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientListFrozenUsersErrorAuthorNotAllowed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientListFrozenUsersError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientListFrozenUsersErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientListFrozenUsersError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "ClientListFrozenUsersErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientListShamirRecoveriesForOthersError

#[allow(dead_code)]
fn variant_client_list_shamir_recoveries_for_others_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientListShamirRecoveriesForOthersError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientListShamirRecoveriesForOthersError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "ClientListShamirRecoveriesForOthersErrorInternal")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientListShamirRecoveriesForOthersError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "ClientListShamirRecoveriesForOthersErrorStopped")
                .or_throw(cx)?;
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
            let js_tag =
                JsString::try_new(cx, "ClientListUserDevicesErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientListUserDevicesError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "ClientListUserDevicesErrorStopped").or_throw(cx)?;
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
            let js_tag = JsString::try_new(cx, "ClientListUsersErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientListUsersError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "ClientListUsersErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientListWorkspaceUsersError

#[allow(dead_code)]
fn variant_client_list_workspace_users_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientListWorkspaceUsersError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientListWorkspaceUsersError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientListWorkspaceUsersErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientListWorkspaceUsersError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientListWorkspaceUsersErrorStopped").or_throw(cx)?;
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
            let js_tag = JsString::try_new(cx, "ClientListWorkspacesErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientNewDeviceInvitationError

#[allow(dead_code)]
fn variant_client_new_device_invitation_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientNewDeviceInvitationError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientNewDeviceInvitationError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientNewDeviceInvitationErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientNewDeviceInvitationError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientNewDeviceInvitationErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientNewShamirRecoveryInvitationError

#[allow(dead_code)]
fn variant_client_new_shamir_recovery_invitation_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientNewShamirRecoveryInvitationError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientNewShamirRecoveryInvitationError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "ClientNewShamirRecoveryInvitationErrorInternal")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientNewShamirRecoveryInvitationError::NotAllowed { .. } => {
            let js_tag = JsString::try_new(cx, "ClientNewShamirRecoveryInvitationErrorNotAllowed")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientNewShamirRecoveryInvitationError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "ClientNewShamirRecoveryInvitationErrorOffline")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientNewShamirRecoveryInvitationError::UserNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientNewShamirRecoveryInvitationErrorUserNotFound")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientNewUserInvitationError

#[allow(dead_code)]
fn variant_client_new_user_invitation_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientNewUserInvitationError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientNewUserInvitationError::AlreadyMember { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientNewUserInvitationErrorAlreadyMember").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientNewUserInvitationError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientNewUserInvitationErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientNewUserInvitationError::NotAllowed { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientNewUserInvitationErrorNotAllowed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientNewUserInvitationError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientNewUserInvitationErrorOffline").or_throw(cx)?;
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
        libparsec::ClientRenameWorkspaceError::AuthorNotAllowed { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientRenameWorkspaceErrorAuthorNotAllowed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientRenameWorkspaceErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "ClientRenameWorkspaceErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::InvalidEncryptedRealmName { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientRenameWorkspaceErrorInvalidEncryptedRealmName")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::InvalidKeysBundle { .. } => {
            let js_tag = JsString::try_new(cx, "ClientRenameWorkspaceErrorInvalidKeysBundle")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::NoKey { .. } => {
            let js_tag = JsString::try_new(cx, "ClientRenameWorkspaceErrorNoKey").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "ClientRenameWorkspaceErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "ClientRenameWorkspaceErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "ClientRenameWorkspaceErrorTimestampOutOfBallpark")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_server_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "serverTimestamp", js_server_timestamp)?;
            let js_client_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
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
        libparsec::ClientRenameWorkspaceError::WorkspaceNotFound { .. } => {
            let js_tag = JsString::try_new(cx, "ClientRenameWorkspaceErrorWorkspaceNotFound")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientRevokeUserError

#[allow(dead_code)]
fn variant_client_revoke_user_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientRevokeUserError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientRevokeUserError::AuthorNotAllowed { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientRevokeUserErrorAuthorNotAllowed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRevokeUserError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "ClientRevokeUserErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRevokeUserError::InvalidCertificate { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientRevokeUserErrorInvalidCertificate").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRevokeUserError::InvalidKeysBundle { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientRevokeUserErrorInvalidKeysBundle").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRevokeUserError::NoKey { .. } => {
            let js_tag = JsString::try_new(cx, "ClientRevokeUserErrorNoKey").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRevokeUserError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "ClientRevokeUserErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRevokeUserError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "ClientRevokeUserErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRevokeUserError::TimestampOutOfBallpark { .. } => {
            let js_tag = JsString::try_new(cx, "ClientRevokeUserErrorTimestampOutOfBallpark")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRevokeUserError::UserIsSelf { .. } => {
            let js_tag = JsString::try_new(cx, "ClientRevokeUserErrorUserIsSelf").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientRevokeUserError::UserNotFound { .. } => {
            let js_tag = JsString::try_new(cx, "ClientRevokeUserErrorUserNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientSetupShamirRecoveryError

#[allow(dead_code)]
fn variant_client_setup_shamir_recovery_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientSetupShamirRecoveryError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientSetupShamirRecoveryError::AuthorAmongRecipients { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientSetupShamirRecoveryErrorAuthorAmongRecipients")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientSetupShamirRecoveryError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientSetupShamirRecoveryErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientSetupShamirRecoveryError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "ClientSetupShamirRecoveryErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientSetupShamirRecoveryError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientSetupShamirRecoveryErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientSetupShamirRecoveryError::RecipientNotFound { .. } => {
            let js_tag = JsString::try_new(cx, "ClientSetupShamirRecoveryErrorRecipientNotFound")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientSetupShamirRecoveryError::RecipientRevoked { .. } => {
            let js_tag = JsString::try_new(cx, "ClientSetupShamirRecoveryErrorRecipientRevoked")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientSetupShamirRecoveryError::ShamirRecoveryAlreadyExists { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "ClientSetupShamirRecoveryErrorShamirRecoveryAlreadyExists",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientSetupShamirRecoveryError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientSetupShamirRecoveryErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientSetupShamirRecoveryError::ThresholdBiggerThanSumOfShares { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "ClientSetupShamirRecoveryErrorThresholdBiggerThanSumOfShares",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientSetupShamirRecoveryError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let js_tag =
                JsString::try_new(cx, "ClientSetupShamirRecoveryErrorTimestampOutOfBallpark")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_server_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "serverTimestamp", js_server_timestamp)?;
            let js_client_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
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
        libparsec::ClientSetupShamirRecoveryError::TooManyShares { .. } => {
            let js_tag = JsString::try_new(cx, "ClientSetupShamirRecoveryErrorTooManyShares")
                .or_throw(cx)?;
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
        libparsec::ClientShareWorkspaceError::AuthorNotAllowed { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientShareWorkspaceErrorAuthorNotAllowed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "ClientShareWorkspaceErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "ClientShareWorkspaceErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::InvalidKeysBundle { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientShareWorkspaceErrorInvalidKeysBundle").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "ClientShareWorkspaceErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::RecipientIsSelf { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientShareWorkspaceErrorRecipientIsSelf").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::RecipientNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientShareWorkspaceErrorRecipientNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::RecipientRevoked { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientShareWorkspaceErrorRecipientRevoked").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::RoleIncompatibleWithOutsider { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientShareWorkspaceErrorRoleIncompatibleWithOutsider")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "ClientShareWorkspaceErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientShareWorkspaceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "ClientShareWorkspaceErrorTimestampOutOfBallpark")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_server_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "serverTimestamp", js_server_timestamp)?;
            let js_client_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
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
        libparsec::ClientShareWorkspaceError::WorkspaceNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientShareWorkspaceErrorWorkspaceNotFound").or_throw(cx)?;
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
        libparsec::ClientStartError::DeviceUsedByAnotherProcess { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientStartErrorDeviceUsedByAnotherProcess").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "ClientStartErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartError::LoadDeviceDecryptionFailed { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientStartErrorLoadDeviceDecryptionFailed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartError::LoadDeviceInvalidData { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientStartErrorLoadDeviceInvalidData").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartError::LoadDeviceInvalidPath { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientStartErrorLoadDeviceInvalidPath").or_throw(cx)?;
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
            let js_tag =
                JsString::try_new(cx, "ClientStartInvitationGreetErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientStartShamirRecoveryInvitationGreetError

#[allow(dead_code)]
fn variant_client_start_shamir_recovery_invitation_greet_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientStartShamirRecoveryInvitationGreetError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientStartShamirRecoveryInvitationGreetError::CorruptedShareData { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "ClientStartShamirRecoveryInvitationGreetErrorCorruptedShareData",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientStartShamirRecoveryInvitationGreetErrorInternal")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "ClientStartShamirRecoveryInvitationGreetErrorInvalidCertificate",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::InvitationNotFound { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "ClientStartShamirRecoveryInvitationGreetErrorInvitationNotFound",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientStartShamirRecoveryInvitationGreetErrorOffline")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::ShamirRecoveryDeleted {
            ..
        } => {
            let js_tag = JsString::try_new(
                cx,
                "ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryDeleted",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::ShamirRecoveryNotFound {
            ..
        } => {
            let js_tag = JsString::try_new(
                cx,
                "ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryNotFound",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::ShamirRecoveryUnusable {
            ..
        } => {
            let js_tag = JsString::try_new(
                cx,
                "ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryUnusable",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientStartShamirRecoveryInvitationGreetErrorStopped")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientStartWorkspaceError

#[allow(dead_code)]
fn variant_client_start_workspace_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientStartWorkspaceError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientStartWorkspaceError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "ClientStartWorkspaceErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartWorkspaceError::WorkspaceNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientStartWorkspaceErrorWorkspaceNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientStartWorkspaceHistory2Error

#[allow(dead_code)]
fn variant_client_start_workspace_history2_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientStartWorkspaceHistory2Error,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientStartWorkspaceHistory2Error::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientStartWorkspaceHistory2ErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartWorkspaceHistory2Error::InvalidCertificate { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientStartWorkspaceHistory2ErrorInvalidCertificate")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartWorkspaceHistory2Error::InvalidKeysBundle { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientStartWorkspaceHistory2ErrorInvalidKeysBundle")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartWorkspaceHistory2Error::InvalidManifest { .. } => {
            let js_tag = JsString::try_new(cx, "ClientStartWorkspaceHistory2ErrorInvalidManifest")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartWorkspaceHistory2Error::NoHistory { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientStartWorkspaceHistory2ErrorNoHistory").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartWorkspaceHistory2Error::NoRealmAccess { .. } => {
            let js_tag = JsString::try_new(cx, "ClientStartWorkspaceHistory2ErrorNoRealmAccess")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartWorkspaceHistory2Error::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientStartWorkspaceHistory2ErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientStartWorkspaceHistory2Error::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientStartWorkspaceHistory2ErrorStopped").or_throw(cx)?;
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
            let js_tag = JsString::try_new(cx, "ClientStopErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ClientUserUpdateProfileError

#[allow(dead_code)]
fn variant_client_user_update_profile_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ClientUserUpdateProfileError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ClientUserUpdateProfileError::AuthorNotAllowed { .. } => {
            let js_tag = JsString::try_new(cx, "ClientUserUpdateProfileErrorAuthorNotAllowed")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientUserUpdateProfileError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientUserUpdateProfileErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientUserUpdateProfileError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "ClientUserUpdateProfileErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientUserUpdateProfileError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientUserUpdateProfileErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientUserUpdateProfileError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientUserUpdateProfileErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientUserUpdateProfileError::TimestampOutOfBallpark { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientUserUpdateProfileErrorTimestampOutOfBallpark")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientUserUpdateProfileError::UserIsSelf { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientUserUpdateProfileErrorUserIsSelf").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientUserUpdateProfileError::UserNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientUserUpdateProfileErrorUserNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ClientUserUpdateProfileError::UserRevoked { .. } => {
            let js_tag =
                JsString::try_new(cx, "ClientUserUpdateProfileErrorUserRevoked").or_throw(cx)?;
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
        "DeviceAccessStrategyKeyring" => {
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
            Ok(libparsec::DeviceAccessStrategy::Keyring { key_file })
        }
        "DeviceAccessStrategyPassword" => {
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
        "DeviceAccessStrategySmartcard" => {
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
        libparsec::DeviceAccessStrategy::Keyring { key_file, .. } => {
            let js_tag = JsString::try_new(cx, "DeviceAccessStrategyKeyring").or_throw(cx)?;
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
        libparsec::DeviceAccessStrategy::Password {
            password, key_file, ..
        } => {
            let js_tag = JsString::try_new(cx, "DeviceAccessStrategyPassword").or_throw(cx)?;
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
            let js_tag = JsString::try_new(cx, "DeviceAccessStrategySmartcard").or_throw(cx)?;
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
        "DeviceSaveStrategyKeyring" => Ok(libparsec::DeviceSaveStrategy::Keyring {}),
        "DeviceSaveStrategyPassword" => {
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
        "DeviceSaveStrategySmartcard" => Ok(libparsec::DeviceSaveStrategy::Smartcard {}),
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
        libparsec::DeviceSaveStrategy::Keyring { .. } => {
            let js_tag = JsString::try_new(cx, "DeviceSaveStrategyKeyring").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::DeviceSaveStrategy::Password { password, .. } => {
            let js_tag = JsString::try_new(cx, "DeviceSaveStrategyPassword").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_password = JsString::try_new(cx, password).or_throw(cx)?;
            js_obj.set(cx, "password", js_password)?;
        }
        libparsec::DeviceSaveStrategy::Smartcard { .. } => {
            let js_tag = JsString::try_new(cx, "DeviceSaveStrategySmartcard").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// EntryStat

#[allow(dead_code)]
fn variant_entry_stat_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::EntryStat> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "EntryStatFile" => {
            let confinement_point = {
                let js_val: Handle<JsValue> = obj.get(cx, "confinementPoint")?;
                {
                    if js_val.is_a::<JsNull, _>(cx) {
                        None
                    } else {
                        let js_val = js_val.downcast_or_throw::<JsString, _>(cx)?;
                        Some({
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::VlobID, _> {
                                    libparsec::VlobID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(js_val.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        })
                    }
                }
            };
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
            let parent = {
                let js_val: Handle<JsString> = obj.get(cx, "parent")?;
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
            let created = {
                let js_val: Handle<JsNumber> = obj.get(cx, "created")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let updated = {
                let js_val: Handle<JsNumber> = obj.get(cx, "updated")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let base_version = {
                let js_val: Handle<JsNumber> = obj.get(cx, "baseVersion")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    let v = v as u32;
                    v
                }
            };
            let is_placeholder = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "isPlaceholder")?;
                js_val.value(cx)
            };
            let need_sync = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "needSync")?;
                js_val.value(cx)
            };
            let size = {
                let js_val: Handle<JsNumber> = obj.get(cx, "size")?;
                {
                    let v = js_val.value(cx);
                    if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                        cx.throw_type_error("Not an u64 number")?
                    }
                    let v = v as u64;
                    v
                }
            };
            Ok(libparsec::EntryStat::File {
                confinement_point,
                id,
                parent,
                created,
                updated,
                base_version,
                is_placeholder,
                need_sync,
                size,
            })
        }
        "EntryStatFolder" => {
            let confinement_point = {
                let js_val: Handle<JsValue> = obj.get(cx, "confinementPoint")?;
                {
                    if js_val.is_a::<JsNull, _>(cx) {
                        None
                    } else {
                        let js_val = js_val.downcast_or_throw::<JsString, _>(cx)?;
                        Some({
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::VlobID, _> {
                                    libparsec::VlobID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(js_val.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        })
                    }
                }
            };
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
            let parent = {
                let js_val: Handle<JsString> = obj.get(cx, "parent")?;
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
            let created = {
                let js_val: Handle<JsNumber> = obj.get(cx, "created")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let updated = {
                let js_val: Handle<JsNumber> = obj.get(cx, "updated")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let base_version = {
                let js_val: Handle<JsNumber> = obj.get(cx, "baseVersion")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    let v = v as u32;
                    v
                }
            };
            let is_placeholder = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "isPlaceholder")?;
                js_val.value(cx)
            };
            let need_sync = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "needSync")?;
                js_val.value(cx)
            };
            Ok(libparsec::EntryStat::Folder {
                confinement_point,
                id,
                parent,
                created,
                updated,
                base_version,
                is_placeholder,
                need_sync,
            })
        }
        _ => cx.throw_type_error("Object is not a EntryStat"),
    }
}

#[allow(dead_code)]
fn variant_entry_stat_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::EntryStat,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::EntryStat::File {
            confinement_point,
            id,
            parent,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            size,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "EntryStatFile").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_confinement_point = match confinement_point {
                Some(elem) => JsString::try_new(cx, {
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(elem) {
                        Ok(ok) => ok,
                        Err(err) => return cx.throw_type_error(err),
                    }
                })
                .or_throw(cx)?
                .as_value(cx),
                None => JsNull::new(cx).as_value(cx),
            };
            js_obj.set(cx, "confinementPoint", js_confinement_point)?;
            let js_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "id", js_id)?;
            let js_parent = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(parent) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "parent", js_parent)?;
            let js_created = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "created", js_created)?;
            let js_updated = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(updated) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "updated", js_updated)?;
            let js_base_version = JsNumber::new(cx, base_version as f64);
            js_obj.set(cx, "baseVersion", js_base_version)?;
            let js_is_placeholder = JsBoolean::new(cx, is_placeholder);
            js_obj.set(cx, "isPlaceholder", js_is_placeholder)?;
            let js_need_sync = JsBoolean::new(cx, need_sync);
            js_obj.set(cx, "needSync", js_need_sync)?;
            let js_size = JsNumber::new(cx, size as f64);
            js_obj.set(cx, "size", js_size)?;
        }
        libparsec::EntryStat::Folder {
            confinement_point,
            id,
            parent,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "EntryStatFolder").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_confinement_point = match confinement_point {
                Some(elem) => JsString::try_new(cx, {
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(elem) {
                        Ok(ok) => ok,
                        Err(err) => return cx.throw_type_error(err),
                    }
                })
                .or_throw(cx)?
                .as_value(cx),
                None => JsNull::new(cx).as_value(cx),
            };
            js_obj.set(cx, "confinementPoint", js_confinement_point)?;
            let js_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "id", js_id)?;
            let js_parent = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(parent) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "parent", js_parent)?;
            let js_created = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "created", js_created)?;
            let js_updated = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(updated) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "updated", js_updated)?;
            let js_base_version = JsNumber::new(cx, base_version as f64);
            js_obj.set(cx, "baseVersion", js_base_version)?;
            let js_is_placeholder = JsBoolean::new(cx, is_placeholder);
            js_obj.set(cx, "isPlaceholder", js_is_placeholder)?;
            let js_need_sync = JsBoolean::new(cx, need_sync);
            js_obj.set(cx, "needSync", js_need_sync)?;
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
            let js_tag = JsString::try_new(cx, "GreetInProgressErrorActiveUsersLimitReached")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::AlreadyDeleted { .. } => {
            let js_tag =
                JsString::try_new(cx, "GreetInProgressErrorAlreadyDeleted").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::Cancelled { .. } => {
            let js_tag = JsString::try_new(cx, "GreetInProgressErrorCancelled").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::CorruptedInviteUserData { .. } => {
            let js_tag = JsString::try_new(cx, "GreetInProgressErrorCorruptedInviteUserData")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::DeviceAlreadyExists { .. } => {
            let js_tag =
                JsString::try_new(cx, "GreetInProgressErrorDeviceAlreadyExists").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::GreeterNotAllowed { .. } => {
            let js_tag =
                JsString::try_new(cx, "GreetInProgressErrorGreeterNotAllowed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::GreetingAttemptCancelled {
            origin,
            reason,
            timestamp,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "GreetInProgressErrorGreetingAttemptCancelled")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_origin =
                JsString::try_new(cx, enum_greeter_or_claimer_rs_to_js(origin)).or_throw(cx)?;
            js_obj.set(cx, "origin", js_origin)?;
            let js_reason =
                JsString::try_new(cx, enum_cancelled_greeting_attempt_reason_rs_to_js(reason))
                    .or_throw(cx)?;
            js_obj.set(cx, "reason", js_reason)?;
            let js_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "timestamp", js_timestamp)?;
        }
        libparsec::GreetInProgressError::HumanHandleAlreadyTaken { .. } => {
            let js_tag = JsString::try_new(cx, "GreetInProgressErrorHumanHandleAlreadyTaken")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "GreetInProgressErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::NonceMismatch { .. } => {
            let js_tag = JsString::try_new(cx, "GreetInProgressErrorNonceMismatch").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::NotFound { .. } => {
            let js_tag = JsString::try_new(cx, "GreetInProgressErrorNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "GreetInProgressErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::PeerReset { .. } => {
            let js_tag = JsString::try_new(cx, "GreetInProgressErrorPeerReset").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let js_tag =
                JsString::try_new(cx, "GreetInProgressErrorTimestampOutOfBallpark").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_server_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "serverTimestamp", js_server_timestamp)?;
            let js_client_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
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
        libparsec::GreetInProgressError::UserAlreadyExists { .. } => {
            let js_tag =
                JsString::try_new(cx, "GreetInProgressErrorUserAlreadyExists").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::GreetInProgressError::UserCreateNotAllowed { .. } => {
            let js_tag =
                JsString::try_new(cx, "GreetInProgressErrorUserCreateNotAllowed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ImportRecoveryDeviceError

#[allow(dead_code)]
fn variant_import_recovery_device_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ImportRecoveryDeviceError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ImportRecoveryDeviceError::DecryptionFailed { .. } => {
            let js_tag =
                JsString::try_new(cx, "ImportRecoveryDeviceErrorDecryptionFailed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ImportRecoveryDeviceError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "ImportRecoveryDeviceErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ImportRecoveryDeviceError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "ImportRecoveryDeviceErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ImportRecoveryDeviceError::InvalidData { .. } => {
            let js_tag =
                JsString::try_new(cx, "ImportRecoveryDeviceErrorInvalidData").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ImportRecoveryDeviceError::InvalidPassphrase { .. } => {
            let js_tag =
                JsString::try_new(cx, "ImportRecoveryDeviceErrorInvalidPassphrase").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ImportRecoveryDeviceError::InvalidPath { .. } => {
            let js_tag =
                JsString::try_new(cx, "ImportRecoveryDeviceErrorInvalidPath").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ImportRecoveryDeviceError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "ImportRecoveryDeviceErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ImportRecoveryDeviceError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "ImportRecoveryDeviceErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ImportRecoveryDeviceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "ImportRecoveryDeviceErrorTimestampOutOfBallpark")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_server_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "serverTimestamp", js_server_timestamp)?;
            let js_client_timestamp = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
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
    }
    Ok(js_obj)
}

// InviteInfoInvitationCreatedBy

#[allow(dead_code)]
fn variant_invite_info_invitation_created_by_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::InviteInfoInvitationCreatedBy> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "InviteInfoInvitationCreatedByExternalService" => {
            let service_label = {
                let js_val: Handle<JsString> = obj.get(cx, "serviceLabel")?;
                js_val.value(cx)
            };
            Ok(libparsec::InviteInfoInvitationCreatedBy::ExternalService { service_label })
        }
        "InviteInfoInvitationCreatedByUser" => {
            let user_id = {
                let js_val: Handle<JsString> = obj.get(cx, "userId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                        libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let human_handle = {
                let js_val: Handle<JsObject> = obj.get(cx, "humanHandle")?;
                struct_human_handle_js_to_rs(cx, js_val)?
            };
            Ok(libparsec::InviteInfoInvitationCreatedBy::User {
                user_id,
                human_handle,
            })
        }
        _ => cx.throw_type_error("Object is not a InviteInfoInvitationCreatedBy"),
    }
}

#[allow(dead_code)]
fn variant_invite_info_invitation_created_by_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::InviteInfoInvitationCreatedBy,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::InviteInfoInvitationCreatedBy::ExternalService { service_label, .. } => {
            let js_tag = JsString::try_new(cx, "InviteInfoInvitationCreatedByExternalService")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_service_label = JsString::try_new(cx, service_label).or_throw(cx)?;
            js_obj.set(cx, "serviceLabel", js_service_label)?;
        }
        libparsec::InviteInfoInvitationCreatedBy::User {
            user_id,
            human_handle,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "InviteInfoInvitationCreatedByUser").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_user_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(user_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "userId", js_user_id)?;
            let js_human_handle = struct_human_handle_rs_to_js(cx, human_handle)?;
            js_obj.set(cx, "humanHandle", js_human_handle)?;
        }
    }
    Ok(js_obj)
}

// InviteListInvitationCreatedBy

#[allow(dead_code)]
fn variant_invite_list_invitation_created_by_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::InviteListInvitationCreatedBy> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "InviteListInvitationCreatedByExternalService" => {
            let service_label = {
                let js_val: Handle<JsString> = obj.get(cx, "serviceLabel")?;
                js_val.value(cx)
            };
            Ok(libparsec::InviteListInvitationCreatedBy::ExternalService { service_label })
        }
        "InviteListInvitationCreatedByUser" => {
            let user_id = {
                let js_val: Handle<JsString> = obj.get(cx, "userId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                        libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let human_handle = {
                let js_val: Handle<JsObject> = obj.get(cx, "humanHandle")?;
                struct_human_handle_js_to_rs(cx, js_val)?
            };
            Ok(libparsec::InviteListInvitationCreatedBy::User {
                user_id,
                human_handle,
            })
        }
        _ => cx.throw_type_error("Object is not a InviteListInvitationCreatedBy"),
    }
}

#[allow(dead_code)]
fn variant_invite_list_invitation_created_by_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::InviteListInvitationCreatedBy,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::InviteListInvitationCreatedBy::ExternalService { service_label, .. } => {
            let js_tag = JsString::try_new(cx, "InviteListInvitationCreatedByExternalService")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_service_label = JsString::try_new(cx, service_label).or_throw(cx)?;
            js_obj.set(cx, "serviceLabel", js_service_label)?;
        }
        libparsec::InviteListInvitationCreatedBy::User {
            user_id,
            human_handle,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "InviteListInvitationCreatedByUser").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_user_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(user_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "userId", js_user_id)?;
            let js_human_handle = struct_human_handle_rs_to_js(cx, human_handle)?;
            js_obj.set(cx, "humanHandle", js_human_handle)?;
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
        "InviteListItemDevice" => {
            let addr = {
                let js_val: Handle<JsString> = obj.get(cx, "addr")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::ParsecInvitationAddr::from_any(&s).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
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
            let created_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "createdOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_by = {
                let js_val: Handle<JsObject> = obj.get(cx, "createdBy")?;
                variant_invite_list_invitation_created_by_js_to_rs(cx, js_val)?
            };
            let status = {
                let js_val: Handle<JsString> = obj.get(cx, "status")?;
                {
                    let js_string = js_val.value(cx);
                    enum_invitation_status_js_to_rs(cx, js_string.as_str())?
                }
            };
            Ok(libparsec::InviteListItem::Device {
                addr,
                token,
                created_on,
                created_by,
                status,
            })
        }
        "InviteListItemShamirRecovery" => {
            let addr = {
                let js_val: Handle<JsString> = obj.get(cx, "addr")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::ParsecInvitationAddr::from_any(&s).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
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
            let created_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "createdOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_by = {
                let js_val: Handle<JsObject> = obj.get(cx, "createdBy")?;
                variant_invite_list_invitation_created_by_js_to_rs(cx, js_val)?
            };
            let claimer_user_id = {
                let js_val: Handle<JsString> = obj.get(cx, "claimerUserId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                        libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let shamir_recovery_created_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "shamirRecoveryCreatedOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
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
            Ok(libparsec::InviteListItem::ShamirRecovery {
                addr,
                token,
                created_on,
                created_by,
                claimer_user_id,
                shamir_recovery_created_on,
                status,
            })
        }
        "InviteListItemUser" => {
            let addr = {
                let js_val: Handle<JsString> = obj.get(cx, "addr")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::ParsecInvitationAddr::from_any(&s).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
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
            let created_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "createdOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_by = {
                let js_val: Handle<JsObject> = obj.get(cx, "createdBy")?;
                variant_invite_list_invitation_created_by_js_to_rs(cx, js_val)?
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
                addr,
                token,
                created_on,
                created_by,
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
            addr,
            token,
            created_on,
            created_by,
            status,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "InviteListItemDevice").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_addr = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |addr: libparsec::ParsecInvitationAddr| -> Result<String, &'static str> {
                        Ok(addr.to_url().into())
                    };
                match custom_to_rs_string(addr) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "addr", js_addr)?;
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
            let js_created_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "createdOn", js_created_on)?;
            let js_created_by = variant_invite_list_invitation_created_by_rs_to_js(cx, created_by)?;
            js_obj.set(cx, "createdBy", js_created_by)?;
            let js_status =
                JsString::try_new(cx, enum_invitation_status_rs_to_js(status)).or_throw(cx)?;
            js_obj.set(cx, "status", js_status)?;
        }
        libparsec::InviteListItem::ShamirRecovery {
            addr,
            token,
            created_on,
            created_by,
            claimer_user_id,
            shamir_recovery_created_on,
            status,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "InviteListItemShamirRecovery").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_addr = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |addr: libparsec::ParsecInvitationAddr| -> Result<String, &'static str> {
                        Ok(addr.to_url().into())
                    };
                match custom_to_rs_string(addr) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "addr", js_addr)?;
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
            let js_created_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "createdOn", js_created_on)?;
            let js_created_by = variant_invite_list_invitation_created_by_rs_to_js(cx, created_by)?;
            js_obj.set(cx, "createdBy", js_created_by)?;
            let js_claimer_user_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(claimer_user_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "claimerUserId", js_claimer_user_id)?;
            let js_shamir_recovery_created_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(shamir_recovery_created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "shamirRecoveryCreatedOn", js_shamir_recovery_created_on)?;
            let js_status =
                JsString::try_new(cx, enum_invitation_status_rs_to_js(status)).or_throw(cx)?;
            js_obj.set(cx, "status", js_status)?;
        }
        libparsec::InviteListItem::User {
            addr,
            token,
            created_on,
            created_by,
            claimer_email,
            status,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "InviteListItemUser").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_addr = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |addr: libparsec::ParsecInvitationAddr| -> Result<String, &'static str> {
                        Ok(addr.to_url().into())
                    };
                match custom_to_rs_string(addr) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "addr", js_addr)?;
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
            let js_created_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "createdOn", js_created_on)?;
            let js_created_by = variant_invite_list_invitation_created_by_rs_to_js(cx, created_by)?;
            js_obj.set(cx, "createdBy", js_created_by)?;
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
            let js_tag = JsString::try_new(cx, "ListInvitationsErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ListInvitationsError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "ListInvitationsErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// MountpointMountStrategy

#[allow(dead_code)]
fn variant_mountpoint_mount_strategy_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::MountpointMountStrategy> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "MountpointMountStrategyDirectory" => {
            let base_dir = {
                let js_val: Handle<JsString> = obj.get(cx, "baseDir")?;
                {
                    let custom_from_rs_string =
                        |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            Ok(libparsec::MountpointMountStrategy::Directory { base_dir })
        }
        "MountpointMountStrategyDisabled" => Ok(libparsec::MountpointMountStrategy::Disabled),
        "MountpointMountStrategyDriveLetter" => Ok(libparsec::MountpointMountStrategy::DriveLetter),
        _ => cx.throw_type_error("Object is not a MountpointMountStrategy"),
    }
}

#[allow(dead_code)]
fn variant_mountpoint_mount_strategy_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::MountpointMountStrategy,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::MountpointMountStrategy::Directory { base_dir, .. } => {
            let js_tag = JsString::try_new(cx, "MountpointMountStrategyDirectory").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_base_dir = JsString::try_new(cx, {
                let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                };
                match custom_to_rs_string(base_dir) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "baseDir", js_base_dir)?;
        }
        libparsec::MountpointMountStrategy::Disabled => {
            let js_tag = JsString::try_new(cx, "Disabled").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::MountpointMountStrategy::DriveLetter => {
            let js_tag = JsString::try_new(cx, "DriveLetter").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// MountpointToOsPathError

#[allow(dead_code)]
fn variant_mountpoint_to_os_path_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::MountpointToOsPathError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::MountpointToOsPathError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "MountpointToOsPathErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// MountpointUnmountError

#[allow(dead_code)]
fn variant_mountpoint_unmount_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::MountpointUnmountError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::MountpointUnmountError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "MountpointUnmountErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// MoveEntryMode

#[allow(dead_code)]
fn variant_move_entry_mode_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::MoveEntryMode> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "MoveEntryModeCanReplace" => Ok(libparsec::MoveEntryMode::CanReplace),
        "MoveEntryModeCanReplaceFileOnly" => Ok(libparsec::MoveEntryMode::CanReplaceFileOnly),
        "MoveEntryModeExchange" => Ok(libparsec::MoveEntryMode::Exchange),
        "MoveEntryModeNoReplace" => Ok(libparsec::MoveEntryMode::NoReplace),
        _ => cx.throw_type_error("Object is not a MoveEntryMode"),
    }
}

#[allow(dead_code)]
fn variant_move_entry_mode_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::MoveEntryMode,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::MoveEntryMode::CanReplace => {
            let js_tag = JsString::try_new(cx, "CanReplace").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::MoveEntryMode::CanReplaceFileOnly => {
            let js_tag = JsString::try_new(cx, "CanReplaceFileOnly").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::MoveEntryMode::Exchange => {
            let js_tag = JsString::try_new(cx, "Exchange").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::MoveEntryMode::NoReplace => {
            let js_tag = JsString::try_new(cx, "NoReplace").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// OtherShamirRecoveryInfo

#[allow(dead_code)]
fn variant_other_shamir_recovery_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::OtherShamirRecoveryInfo> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "OtherShamirRecoveryInfoDeleted" => {
            let user_id = {
                let js_val: Handle<JsString> = obj.get(cx, "userId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                        libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "createdOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_by = {
                let js_val: Handle<JsString> = obj.get(cx, "createdBy")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let threshold = {
                let js_val: Handle<JsNumber> = obj.get(cx, "threshold")?;
                {
                    let v = js_val.value(cx);
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        cx.throw_type_error("Not an u8 number")?
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val: Handle<JsObject> = obj.get(cx, "perRecipientShares")?;
                {
                    let mut d = std::collections::HashMap::with_capacity(
                        js_val.get::<JsNumber, _, _>(cx, "size")?.value(cx) as usize,
                    );

                    let js_keys = js_val
                        .call_method_with(cx, "keys")?
                        .apply::<JsObject, _>(cx)?;
                    let js_values = js_val
                        .call_method_with(cx, "values")?
                        .apply::<JsObject, _>(cx)?;
                    let js_keys_next_cb = js_keys.call_method_with(cx, "next")?;
                    let js_values_next_cb = js_values.call_method_with(cx, "next")?;

                    loop {
                        let next_js_key = js_keys_next_cb.apply::<JsObject, _>(cx)?;
                        let next_js_value = js_values_next_cb.apply::<JsObject, _>(cx)?;

                        let keys_done = next_js_key.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        let values_done =
                            next_js_value.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        match (keys_done, values_done) {
                            (true, true) => break,
                            (false, false) => (),
                            _ => unreachable!(),
                        }

                        let js_key = next_js_key.get::<JsString, _, _>(cx, "value")?;
                        let js_value = next_js_value.get::<JsNumber, _, _>(cx, "value")?;

                        let key = {
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::UserID, _> {
                                    libparsec::UserID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(js_key.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        let value = {
                            let v = js_value.value(cx);
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                cx.throw_type_error("Not an u8 number")?
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        d.insert(key, value);
                    }
                    d
                }
            };
            let deleted_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "deletedOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let deleted_by = {
                let js_val: Handle<JsString> = obj.get(cx, "deletedBy")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            Ok(libparsec::OtherShamirRecoveryInfo::Deleted {
                user_id,
                created_on,
                created_by,
                threshold,
                per_recipient_shares,
                deleted_on,
                deleted_by,
            })
        }
        "OtherShamirRecoveryInfoSetupAllValid" => {
            let user_id = {
                let js_val: Handle<JsString> = obj.get(cx, "userId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                        libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "createdOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_by = {
                let js_val: Handle<JsString> = obj.get(cx, "createdBy")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let threshold = {
                let js_val: Handle<JsNumber> = obj.get(cx, "threshold")?;
                {
                    let v = js_val.value(cx);
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        cx.throw_type_error("Not an u8 number")?
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val: Handle<JsObject> = obj.get(cx, "perRecipientShares")?;
                {
                    let mut d = std::collections::HashMap::with_capacity(
                        js_val.get::<JsNumber, _, _>(cx, "size")?.value(cx) as usize,
                    );

                    let js_keys = js_val
                        .call_method_with(cx, "keys")?
                        .apply::<JsObject, _>(cx)?;
                    let js_values = js_val
                        .call_method_with(cx, "values")?
                        .apply::<JsObject, _>(cx)?;
                    let js_keys_next_cb = js_keys.call_method_with(cx, "next")?;
                    let js_values_next_cb = js_values.call_method_with(cx, "next")?;

                    loop {
                        let next_js_key = js_keys_next_cb.apply::<JsObject, _>(cx)?;
                        let next_js_value = js_values_next_cb.apply::<JsObject, _>(cx)?;

                        let keys_done = next_js_key.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        let values_done =
                            next_js_value.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        match (keys_done, values_done) {
                            (true, true) => break,
                            (false, false) => (),
                            _ => unreachable!(),
                        }

                        let js_key = next_js_key.get::<JsString, _, _>(cx, "value")?;
                        let js_value = next_js_value.get::<JsNumber, _, _>(cx, "value")?;

                        let key = {
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::UserID, _> {
                                    libparsec::UserID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(js_key.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        let value = {
                            let v = js_value.value(cx);
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                cx.throw_type_error("Not an u8 number")?
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        d.insert(key, value);
                    }
                    d
                }
            };
            Ok(libparsec::OtherShamirRecoveryInfo::SetupAllValid {
                user_id,
                created_on,
                created_by,
                threshold,
                per_recipient_shares,
            })
        }
        "OtherShamirRecoveryInfoSetupButUnusable" => {
            let user_id = {
                let js_val: Handle<JsString> = obj.get(cx, "userId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                        libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "createdOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_by = {
                let js_val: Handle<JsString> = obj.get(cx, "createdBy")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let threshold = {
                let js_val: Handle<JsNumber> = obj.get(cx, "threshold")?;
                {
                    let v = js_val.value(cx);
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        cx.throw_type_error("Not an u8 number")?
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val: Handle<JsObject> = obj.get(cx, "perRecipientShares")?;
                {
                    let mut d = std::collections::HashMap::with_capacity(
                        js_val.get::<JsNumber, _, _>(cx, "size")?.value(cx) as usize,
                    );

                    let js_keys = js_val
                        .call_method_with(cx, "keys")?
                        .apply::<JsObject, _>(cx)?;
                    let js_values = js_val
                        .call_method_with(cx, "values")?
                        .apply::<JsObject, _>(cx)?;
                    let js_keys_next_cb = js_keys.call_method_with(cx, "next")?;
                    let js_values_next_cb = js_values.call_method_with(cx, "next")?;

                    loop {
                        let next_js_key = js_keys_next_cb.apply::<JsObject, _>(cx)?;
                        let next_js_value = js_values_next_cb.apply::<JsObject, _>(cx)?;

                        let keys_done = next_js_key.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        let values_done =
                            next_js_value.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        match (keys_done, values_done) {
                            (true, true) => break,
                            (false, false) => (),
                            _ => unreachable!(),
                        }

                        let js_key = next_js_key.get::<JsString, _, _>(cx, "value")?;
                        let js_value = next_js_value.get::<JsNumber, _, _>(cx, "value")?;

                        let key = {
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::UserID, _> {
                                    libparsec::UserID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(js_key.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        let value = {
                            let v = js_value.value(cx);
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                cx.throw_type_error("Not an u8 number")?
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        d.insert(key, value);
                    }
                    d
                }
            };
            let revoked_recipients = {
                let js_val: Handle<JsArray> = obj.get(cx, "revokedRecipients")?;
                {
                    let size = js_val.len(cx);
                    let mut v = Vec::with_capacity(size as usize);
                    for i in 0..size {
                        let js_item: Handle<JsString> = js_val.get(cx, i)?;
                        v.push({
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::UserID, _> {
                                    libparsec::UserID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(js_item.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        });
                    }
                    v.into_iter().collect()
                }
            };
            Ok(libparsec::OtherShamirRecoveryInfo::SetupButUnusable {
                user_id,
                created_on,
                created_by,
                threshold,
                per_recipient_shares,
                revoked_recipients,
            })
        }
        "OtherShamirRecoveryInfoSetupWithRevokedRecipients" => {
            let user_id = {
                let js_val: Handle<JsString> = obj.get(cx, "userId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                        libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "createdOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_by = {
                let js_val: Handle<JsString> = obj.get(cx, "createdBy")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let threshold = {
                let js_val: Handle<JsNumber> = obj.get(cx, "threshold")?;
                {
                    let v = js_val.value(cx);
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        cx.throw_type_error("Not an u8 number")?
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val: Handle<JsObject> = obj.get(cx, "perRecipientShares")?;
                {
                    let mut d = std::collections::HashMap::with_capacity(
                        js_val.get::<JsNumber, _, _>(cx, "size")?.value(cx) as usize,
                    );

                    let js_keys = js_val
                        .call_method_with(cx, "keys")?
                        .apply::<JsObject, _>(cx)?;
                    let js_values = js_val
                        .call_method_with(cx, "values")?
                        .apply::<JsObject, _>(cx)?;
                    let js_keys_next_cb = js_keys.call_method_with(cx, "next")?;
                    let js_values_next_cb = js_values.call_method_with(cx, "next")?;

                    loop {
                        let next_js_key = js_keys_next_cb.apply::<JsObject, _>(cx)?;
                        let next_js_value = js_values_next_cb.apply::<JsObject, _>(cx)?;

                        let keys_done = next_js_key.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        let values_done =
                            next_js_value.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        match (keys_done, values_done) {
                            (true, true) => break,
                            (false, false) => (),
                            _ => unreachable!(),
                        }

                        let js_key = next_js_key.get::<JsString, _, _>(cx, "value")?;
                        let js_value = next_js_value.get::<JsNumber, _, _>(cx, "value")?;

                        let key = {
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::UserID, _> {
                                    libparsec::UserID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(js_key.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        let value = {
                            let v = js_value.value(cx);
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                cx.throw_type_error("Not an u8 number")?
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        d.insert(key, value);
                    }
                    d
                }
            };
            let revoked_recipients = {
                let js_val: Handle<JsArray> = obj.get(cx, "revokedRecipients")?;
                {
                    let size = js_val.len(cx);
                    let mut v = Vec::with_capacity(size as usize);
                    for i in 0..size {
                        let js_item: Handle<JsString> = js_val.get(cx, i)?;
                        v.push({
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::UserID, _> {
                                    libparsec::UserID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(js_item.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        });
                    }
                    v.into_iter().collect()
                }
            };
            Ok(
                libparsec::OtherShamirRecoveryInfo::SetupWithRevokedRecipients {
                    user_id,
                    created_on,
                    created_by,
                    threshold,
                    per_recipient_shares,
                    revoked_recipients,
                },
            )
        }
        _ => cx.throw_type_error("Object is not a OtherShamirRecoveryInfo"),
    }
}

#[allow(dead_code)]
fn variant_other_shamir_recovery_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::OtherShamirRecoveryInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::OtherShamirRecoveryInfo::Deleted {
            user_id,
            created_on,
            created_by,
            threshold,
            per_recipient_shares,
            deleted_on,
            deleted_by,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "OtherShamirRecoveryInfoDeleted").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_user_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(user_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "userId", js_user_id)?;
            let js_created_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "createdOn", js_created_on)?;
            let js_created_by = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "createdBy", js_created_by)?;
            let js_threshold = JsNumber::new(cx, {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            } as f64);
            js_obj.set(cx, "threshold", js_threshold)?;
            let js_per_recipient_shares = {
                let new_map_code = (cx).string("new Map()");
                let js_map =
                    neon::reflect::eval(cx, new_map_code)?.downcast_or_throw::<JsObject, _>(cx)?;
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsString::try_new(cx, {
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    })
                    .or_throw(cx)?;
                    let js_value = JsNumber::new(cx, {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    } as f64);
                    js_map
                        .call_method_with(cx, "set")?
                        .arg(js_key)
                        .arg(js_value)
                        .exec(cx)?;
                }
                js_map
            };
            js_obj.set(cx, "perRecipientShares", js_per_recipient_shares)?;
            let js_deleted_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(deleted_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "deletedOn", js_deleted_on)?;
            let js_deleted_by = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(deleted_by) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "deletedBy", js_deleted_by)?;
        }
        libparsec::OtherShamirRecoveryInfo::SetupAllValid {
            user_id,
            created_on,
            created_by,
            threshold,
            per_recipient_shares,
            ..
        } => {
            let js_tag =
                JsString::try_new(cx, "OtherShamirRecoveryInfoSetupAllValid").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_user_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(user_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "userId", js_user_id)?;
            let js_created_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "createdOn", js_created_on)?;
            let js_created_by = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "createdBy", js_created_by)?;
            let js_threshold = JsNumber::new(cx, {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            } as f64);
            js_obj.set(cx, "threshold", js_threshold)?;
            let js_per_recipient_shares = {
                let new_map_code = (cx).string("new Map()");
                let js_map =
                    neon::reflect::eval(cx, new_map_code)?.downcast_or_throw::<JsObject, _>(cx)?;
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsString::try_new(cx, {
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    })
                    .or_throw(cx)?;
                    let js_value = JsNumber::new(cx, {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    } as f64);
                    js_map
                        .call_method_with(cx, "set")?
                        .arg(js_key)
                        .arg(js_value)
                        .exec(cx)?;
                }
                js_map
            };
            js_obj.set(cx, "perRecipientShares", js_per_recipient_shares)?;
        }
        libparsec::OtherShamirRecoveryInfo::SetupButUnusable {
            user_id,
            created_on,
            created_by,
            threshold,
            per_recipient_shares,
            revoked_recipients,
            ..
        } => {
            let js_tag =
                JsString::try_new(cx, "OtherShamirRecoveryInfoSetupButUnusable").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_user_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(user_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "userId", js_user_id)?;
            let js_created_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "createdOn", js_created_on)?;
            let js_created_by = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "createdBy", js_created_by)?;
            let js_threshold = JsNumber::new(cx, {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            } as f64);
            js_obj.set(cx, "threshold", js_threshold)?;
            let js_per_recipient_shares = {
                let new_map_code = (cx).string("new Map()");
                let js_map =
                    neon::reflect::eval(cx, new_map_code)?.downcast_or_throw::<JsObject, _>(cx)?;
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsString::try_new(cx, {
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    })
                    .or_throw(cx)?;
                    let js_value = JsNumber::new(cx, {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    } as f64);
                    js_map
                        .call_method_with(cx, "set")?
                        .arg(js_key)
                        .arg(js_value)
                        .exec(cx)?;
                }
                js_map
            };
            js_obj.set(cx, "perRecipientShares", js_per_recipient_shares)?;
            let js_revoked_recipients = {
                // JsArray::new allocates with `undefined` value, that's why we `set` value
                let js_array = JsArray::new(cx, revoked_recipients.len());
                for (i, elem) in revoked_recipients.into_iter().enumerate() {
                    let js_elem = JsString::try_new(cx, {
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(elem) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    })
                    .or_throw(cx)?;
                    js_array.set(cx, i as u32, js_elem)?;
                }
                js_array
            };
            js_obj.set(cx, "revokedRecipients", js_revoked_recipients)?;
        }
        libparsec::OtherShamirRecoveryInfo::SetupWithRevokedRecipients {
            user_id,
            created_on,
            created_by,
            threshold,
            per_recipient_shares,
            revoked_recipients,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "OtherShamirRecoveryInfoSetupWithRevokedRecipients")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_user_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(user_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "userId", js_user_id)?;
            let js_created_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "createdOn", js_created_on)?;
            let js_created_by = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "createdBy", js_created_by)?;
            let js_threshold = JsNumber::new(cx, {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            } as f64);
            js_obj.set(cx, "threshold", js_threshold)?;
            let js_per_recipient_shares = {
                let new_map_code = (cx).string("new Map()");
                let js_map =
                    neon::reflect::eval(cx, new_map_code)?.downcast_or_throw::<JsObject, _>(cx)?;
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsString::try_new(cx, {
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    })
                    .or_throw(cx)?;
                    let js_value = JsNumber::new(cx, {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    } as f64);
                    js_map
                        .call_method_with(cx, "set")?
                        .arg(js_key)
                        .arg(js_value)
                        .exec(cx)?;
                }
                js_map
            };
            js_obj.set(cx, "perRecipientShares", js_per_recipient_shares)?;
            let js_revoked_recipients = {
                // JsArray::new allocates with `undefined` value, that's why we `set` value
                let js_array = JsArray::new(cx, revoked_recipients.len());
                for (i, elem) in revoked_recipients.into_iter().enumerate() {
                    let js_elem = JsString::try_new(cx, {
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(elem) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    })
                    .or_throw(cx)?;
                    js_array.set(cx, i as u32, js_elem)?;
                }
                js_array
            };
            js_obj.set(cx, "revokedRecipients", js_revoked_recipients)?;
        }
    }
    Ok(js_obj)
}

// ParseParsecAddrError

#[allow(dead_code)]
fn variant_parse_parsec_addr_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ParseParsecAddrError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ParseParsecAddrError::InvalidUrl { .. } => {
            let js_tag = JsString::try_new(cx, "ParseParsecAddrErrorInvalidUrl").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ParsedParsecAddr

#[allow(dead_code)]
fn variant_parsed_parsec_addr_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ParsedParsecAddr> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "ParsedParsecAddrInvitationDevice" => {
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
                    let v = v as u32;
                    v
                }
            };
            let use_ssl = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "useSsl")?;
                js_val.value(cx)
            };
            let organization_id = {
                let js_val: Handle<JsString> = obj.get(cx, "organizationId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
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
            Ok(libparsec::ParsedParsecAddr::InvitationDevice {
                hostname,
                port,
                use_ssl,
                organization_id,
                token,
            })
        }
        "ParsedParsecAddrInvitationShamirRecovery" => {
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
                    let v = v as u32;
                    v
                }
            };
            let use_ssl = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "useSsl")?;
                js_val.value(cx)
            };
            let organization_id = {
                let js_val: Handle<JsString> = obj.get(cx, "organizationId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
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
            Ok(libparsec::ParsedParsecAddr::InvitationShamirRecovery {
                hostname,
                port,
                use_ssl,
                organization_id,
                token,
            })
        }
        "ParsedParsecAddrInvitationUser" => {
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
                    let v = v as u32;
                    v
                }
            };
            let use_ssl = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "useSsl")?;
                js_val.value(cx)
            };
            let organization_id = {
                let js_val: Handle<JsString> = obj.get(cx, "organizationId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
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
            Ok(libparsec::ParsedParsecAddr::InvitationUser {
                hostname,
                port,
                use_ssl,
                organization_id,
                token,
            })
        }
        "ParsedParsecAddrOrganization" => {
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
                    let v = v as u32;
                    v
                }
            };
            let use_ssl = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "useSsl")?;
                js_val.value(cx)
            };
            let organization_id = {
                let js_val: Handle<JsString> = obj.get(cx, "organizationId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            Ok(libparsec::ParsedParsecAddr::Organization {
                hostname,
                port,
                use_ssl,
                organization_id,
            })
        }
        "ParsedParsecAddrOrganizationBootstrap" => {
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
                    let v = v as u32;
                    v
                }
            };
            let use_ssl = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "useSsl")?;
                js_val.value(cx)
            };
            let organization_id = {
                let js_val: Handle<JsString> = obj.get(cx, "organizationId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
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
            Ok(libparsec::ParsedParsecAddr::OrganizationBootstrap {
                hostname,
                port,
                use_ssl,
                organization_id,
                token,
            })
        }
        "ParsedParsecAddrPkiEnrollment" => {
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
                    let v = v as u32;
                    v
                }
            };
            let use_ssl = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "useSsl")?;
                js_val.value(cx)
            };
            let organization_id = {
                let js_val: Handle<JsString> = obj.get(cx, "organizationId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            Ok(libparsec::ParsedParsecAddr::PkiEnrollment {
                hostname,
                port,
                use_ssl,
                organization_id,
            })
        }
        "ParsedParsecAddrServer" => {
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
                    let v = v as u32;
                    v
                }
            };
            let use_ssl = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "useSsl")?;
                js_val.value(cx)
            };
            Ok(libparsec::ParsedParsecAddr::Server {
                hostname,
                port,
                use_ssl,
            })
        }
        "ParsedParsecAddrWorkspacePath" => {
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
                    let v = v as u32;
                    v
                }
            };
            let use_ssl = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "useSsl")?;
                js_val.value(cx)
            };
            let organization_id = {
                let js_val: Handle<JsString> = obj.get(cx, "organizationId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
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
            let key_index = {
                let js_val: Handle<JsNumber> = obj.get(cx, "keyIndex")?;
                {
                    let v = js_val.value(cx);
                    if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                        cx.throw_type_error("Not an u64 number")?
                    }
                    let v = v as u64;
                    v
                }
            };
            let encrypted_path = {
                let js_val: Handle<JsTypedArray<u8>> = obj.get(cx, "encryptedPath")?;
                js_val.as_slice(cx).to_vec()
            };
            Ok(libparsec::ParsedParsecAddr::WorkspacePath {
                hostname,
                port,
                use_ssl,
                organization_id,
                workspace_id,
                key_index,
                encrypted_path,
            })
        }
        _ => cx.throw_type_error("Object is not a ParsedParsecAddr"),
    }
}

#[allow(dead_code)]
fn variant_parsed_parsec_addr_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ParsedParsecAddr,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::ParsedParsecAddr::InvitationDevice {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "ParsedParsecAddrInvitationDevice").or_throw(cx)?;
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
        libparsec::ParsedParsecAddr::InvitationShamirRecovery {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            let js_tag =
                JsString::try_new(cx, "ParsedParsecAddrInvitationShamirRecovery").or_throw(cx)?;
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
        libparsec::ParsedParsecAddr::InvitationUser {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "ParsedParsecAddrInvitationUser").or_throw(cx)?;
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
        libparsec::ParsedParsecAddr::Organization {
            hostname,
            port,
            use_ssl,
            organization_id,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "ParsedParsecAddrOrganization").or_throw(cx)?;
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
        libparsec::ParsedParsecAddr::OrganizationBootstrap {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            let js_tag =
                JsString::try_new(cx, "ParsedParsecAddrOrganizationBootstrap").or_throw(cx)?;
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
        libparsec::ParsedParsecAddr::PkiEnrollment {
            hostname,
            port,
            use_ssl,
            organization_id,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "ParsedParsecAddrPkiEnrollment").or_throw(cx)?;
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
        libparsec::ParsedParsecAddr::Server {
            hostname,
            port,
            use_ssl,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "ParsedParsecAddrServer").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_hostname = JsString::try_new(cx, hostname).or_throw(cx)?;
            js_obj.set(cx, "hostname", js_hostname)?;
            let js_port = JsNumber::new(cx, port as f64);
            js_obj.set(cx, "port", js_port)?;
            let js_use_ssl = JsBoolean::new(cx, use_ssl);
            js_obj.set(cx, "useSsl", js_use_ssl)?;
        }
        libparsec::ParsedParsecAddr::WorkspacePath {
            hostname,
            port,
            use_ssl,
            organization_id,
            workspace_id,
            key_index,
            encrypted_path,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "ParsedParsecAddrWorkspacePath").or_throw(cx)?;
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
            let js_key_index = JsNumber::new(cx, key_index as f64);
            js_obj.set(cx, "keyIndex", js_key_index)?;
            let js_encrypted_path = {
                let mut js_buff = JsArrayBuffer::new(cx, encrypted_path.len())?;
                let js_buff_slice = js_buff.as_mut_slice(cx);
                for (i, c) in encrypted_path.iter().enumerate() {
                    js_buff_slice[i] = *c;
                }
                js_buff
            };
            js_obj.set(cx, "encryptedPath", js_encrypted_path)?;
        }
    }
    Ok(js_obj)
}

// SelfShamirRecoveryInfo

#[allow(dead_code)]
fn variant_self_shamir_recovery_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::SelfShamirRecoveryInfo> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "SelfShamirRecoveryInfoDeleted" => {
            let created_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "createdOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_by = {
                let js_val: Handle<JsString> = obj.get(cx, "createdBy")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let threshold = {
                let js_val: Handle<JsNumber> = obj.get(cx, "threshold")?;
                {
                    let v = js_val.value(cx);
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        cx.throw_type_error("Not an u8 number")?
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val: Handle<JsObject> = obj.get(cx, "perRecipientShares")?;
                {
                    let mut d = std::collections::HashMap::with_capacity(
                        js_val.get::<JsNumber, _, _>(cx, "size")?.value(cx) as usize,
                    );

                    let js_keys = js_val
                        .call_method_with(cx, "keys")?
                        .apply::<JsObject, _>(cx)?;
                    let js_values = js_val
                        .call_method_with(cx, "values")?
                        .apply::<JsObject, _>(cx)?;
                    let js_keys_next_cb = js_keys.call_method_with(cx, "next")?;
                    let js_values_next_cb = js_values.call_method_with(cx, "next")?;

                    loop {
                        let next_js_key = js_keys_next_cb.apply::<JsObject, _>(cx)?;
                        let next_js_value = js_values_next_cb.apply::<JsObject, _>(cx)?;

                        let keys_done = next_js_key.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        let values_done =
                            next_js_value.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        match (keys_done, values_done) {
                            (true, true) => break,
                            (false, false) => (),
                            _ => unreachable!(),
                        }

                        let js_key = next_js_key.get::<JsString, _, _>(cx, "value")?;
                        let js_value = next_js_value.get::<JsNumber, _, _>(cx, "value")?;

                        let key = {
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::UserID, _> {
                                    libparsec::UserID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(js_key.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        let value = {
                            let v = js_value.value(cx);
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                cx.throw_type_error("Not an u8 number")?
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        d.insert(key, value);
                    }
                    d
                }
            };
            let deleted_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "deletedOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let deleted_by = {
                let js_val: Handle<JsString> = obj.get(cx, "deletedBy")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            Ok(libparsec::SelfShamirRecoveryInfo::Deleted {
                created_on,
                created_by,
                threshold,
                per_recipient_shares,
                deleted_on,
                deleted_by,
            })
        }
        "SelfShamirRecoveryInfoNeverSetup" => Ok(libparsec::SelfShamirRecoveryInfo::NeverSetup {}),
        "SelfShamirRecoveryInfoSetupAllValid" => {
            let created_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "createdOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_by = {
                let js_val: Handle<JsString> = obj.get(cx, "createdBy")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let threshold = {
                let js_val: Handle<JsNumber> = obj.get(cx, "threshold")?;
                {
                    let v = js_val.value(cx);
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        cx.throw_type_error("Not an u8 number")?
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val: Handle<JsObject> = obj.get(cx, "perRecipientShares")?;
                {
                    let mut d = std::collections::HashMap::with_capacity(
                        js_val.get::<JsNumber, _, _>(cx, "size")?.value(cx) as usize,
                    );

                    let js_keys = js_val
                        .call_method_with(cx, "keys")?
                        .apply::<JsObject, _>(cx)?;
                    let js_values = js_val
                        .call_method_with(cx, "values")?
                        .apply::<JsObject, _>(cx)?;
                    let js_keys_next_cb = js_keys.call_method_with(cx, "next")?;
                    let js_values_next_cb = js_values.call_method_with(cx, "next")?;

                    loop {
                        let next_js_key = js_keys_next_cb.apply::<JsObject, _>(cx)?;
                        let next_js_value = js_values_next_cb.apply::<JsObject, _>(cx)?;

                        let keys_done = next_js_key.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        let values_done =
                            next_js_value.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        match (keys_done, values_done) {
                            (true, true) => break,
                            (false, false) => (),
                            _ => unreachable!(),
                        }

                        let js_key = next_js_key.get::<JsString, _, _>(cx, "value")?;
                        let js_value = next_js_value.get::<JsNumber, _, _>(cx, "value")?;

                        let key = {
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::UserID, _> {
                                    libparsec::UserID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(js_key.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        let value = {
                            let v = js_value.value(cx);
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                cx.throw_type_error("Not an u8 number")?
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        d.insert(key, value);
                    }
                    d
                }
            };
            Ok(libparsec::SelfShamirRecoveryInfo::SetupAllValid {
                created_on,
                created_by,
                threshold,
                per_recipient_shares,
            })
        }
        "SelfShamirRecoveryInfoSetupButUnusable" => {
            let created_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "createdOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_by = {
                let js_val: Handle<JsString> = obj.get(cx, "createdBy")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let threshold = {
                let js_val: Handle<JsNumber> = obj.get(cx, "threshold")?;
                {
                    let v = js_val.value(cx);
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        cx.throw_type_error("Not an u8 number")?
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val: Handle<JsObject> = obj.get(cx, "perRecipientShares")?;
                {
                    let mut d = std::collections::HashMap::with_capacity(
                        js_val.get::<JsNumber, _, _>(cx, "size")?.value(cx) as usize,
                    );

                    let js_keys = js_val
                        .call_method_with(cx, "keys")?
                        .apply::<JsObject, _>(cx)?;
                    let js_values = js_val
                        .call_method_with(cx, "values")?
                        .apply::<JsObject, _>(cx)?;
                    let js_keys_next_cb = js_keys.call_method_with(cx, "next")?;
                    let js_values_next_cb = js_values.call_method_with(cx, "next")?;

                    loop {
                        let next_js_key = js_keys_next_cb.apply::<JsObject, _>(cx)?;
                        let next_js_value = js_values_next_cb.apply::<JsObject, _>(cx)?;

                        let keys_done = next_js_key.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        let values_done =
                            next_js_value.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        match (keys_done, values_done) {
                            (true, true) => break,
                            (false, false) => (),
                            _ => unreachable!(),
                        }

                        let js_key = next_js_key.get::<JsString, _, _>(cx, "value")?;
                        let js_value = next_js_value.get::<JsNumber, _, _>(cx, "value")?;

                        let key = {
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::UserID, _> {
                                    libparsec::UserID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(js_key.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        let value = {
                            let v = js_value.value(cx);
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                cx.throw_type_error("Not an u8 number")?
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        d.insert(key, value);
                    }
                    d
                }
            };
            let revoked_recipients = {
                let js_val: Handle<JsArray> = obj.get(cx, "revokedRecipients")?;
                {
                    let size = js_val.len(cx);
                    let mut v = Vec::with_capacity(size as usize);
                    for i in 0..size {
                        let js_item: Handle<JsString> = js_val.get(cx, i)?;
                        v.push({
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::UserID, _> {
                                    libparsec::UserID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(js_item.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        });
                    }
                    v.into_iter().collect()
                }
            };
            Ok(libparsec::SelfShamirRecoveryInfo::SetupButUnusable {
                created_on,
                created_by,
                threshold,
                per_recipient_shares,
                revoked_recipients,
            })
        }
        "SelfShamirRecoveryInfoSetupWithRevokedRecipients" => {
            let created_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "createdOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let created_by = {
                let js_val: Handle<JsString> = obj.get(cx, "createdBy")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let threshold = {
                let js_val: Handle<JsNumber> = obj.get(cx, "threshold")?;
                {
                    let v = js_val.value(cx);
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        cx.throw_type_error("Not an u8 number")?
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val: Handle<JsObject> = obj.get(cx, "perRecipientShares")?;
                {
                    let mut d = std::collections::HashMap::with_capacity(
                        js_val.get::<JsNumber, _, _>(cx, "size")?.value(cx) as usize,
                    );

                    let js_keys = js_val
                        .call_method_with(cx, "keys")?
                        .apply::<JsObject, _>(cx)?;
                    let js_values = js_val
                        .call_method_with(cx, "values")?
                        .apply::<JsObject, _>(cx)?;
                    let js_keys_next_cb = js_keys.call_method_with(cx, "next")?;
                    let js_values_next_cb = js_values.call_method_with(cx, "next")?;

                    loop {
                        let next_js_key = js_keys_next_cb.apply::<JsObject, _>(cx)?;
                        let next_js_value = js_values_next_cb.apply::<JsObject, _>(cx)?;

                        let keys_done = next_js_key.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        let values_done =
                            next_js_value.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        match (keys_done, values_done) {
                            (true, true) => break,
                            (false, false) => (),
                            _ => unreachable!(),
                        }

                        let js_key = next_js_key.get::<JsString, _, _>(cx, "value")?;
                        let js_value = next_js_value.get::<JsNumber, _, _>(cx, "value")?;

                        let key = {
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::UserID, _> {
                                    libparsec::UserID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(js_key.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        let value = {
                            let v = js_value.value(cx);
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                cx.throw_type_error("Not an u8 number")?
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        d.insert(key, value);
                    }
                    d
                }
            };
            let revoked_recipients = {
                let js_val: Handle<JsArray> = obj.get(cx, "revokedRecipients")?;
                {
                    let size = js_val.len(cx);
                    let mut v = Vec::with_capacity(size as usize);
                    for i in 0..size {
                        let js_item: Handle<JsString> = js_val.get(cx, i)?;
                        v.push({
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::UserID, _> {
                                    libparsec::UserID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(js_item.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        });
                    }
                    v.into_iter().collect()
                }
            };
            Ok(
                libparsec::SelfShamirRecoveryInfo::SetupWithRevokedRecipients {
                    created_on,
                    created_by,
                    threshold,
                    per_recipient_shares,
                    revoked_recipients,
                },
            )
        }
        _ => cx.throw_type_error("Object is not a SelfShamirRecoveryInfo"),
    }
}

#[allow(dead_code)]
fn variant_self_shamir_recovery_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::SelfShamirRecoveryInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::SelfShamirRecoveryInfo::Deleted {
            created_on,
            created_by,
            threshold,
            per_recipient_shares,
            deleted_on,
            deleted_by,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "SelfShamirRecoveryInfoDeleted").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_created_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "createdOn", js_created_on)?;
            let js_created_by = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "createdBy", js_created_by)?;
            let js_threshold = JsNumber::new(cx, {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            } as f64);
            js_obj.set(cx, "threshold", js_threshold)?;
            let js_per_recipient_shares = {
                let new_map_code = (cx).string("new Map()");
                let js_map =
                    neon::reflect::eval(cx, new_map_code)?.downcast_or_throw::<JsObject, _>(cx)?;
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsString::try_new(cx, {
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    })
                    .or_throw(cx)?;
                    let js_value = JsNumber::new(cx, {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    } as f64);
                    js_map
                        .call_method_with(cx, "set")?
                        .arg(js_key)
                        .arg(js_value)
                        .exec(cx)?;
                }
                js_map
            };
            js_obj.set(cx, "perRecipientShares", js_per_recipient_shares)?;
            let js_deleted_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(deleted_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "deletedOn", js_deleted_on)?;
            let js_deleted_by = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(deleted_by) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "deletedBy", js_deleted_by)?;
        }
        libparsec::SelfShamirRecoveryInfo::NeverSetup { .. } => {
            let js_tag = JsString::try_new(cx, "SelfShamirRecoveryInfoNeverSetup").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::SelfShamirRecoveryInfo::SetupAllValid {
            created_on,
            created_by,
            threshold,
            per_recipient_shares,
            ..
        } => {
            let js_tag =
                JsString::try_new(cx, "SelfShamirRecoveryInfoSetupAllValid").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_created_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "createdOn", js_created_on)?;
            let js_created_by = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "createdBy", js_created_by)?;
            let js_threshold = JsNumber::new(cx, {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            } as f64);
            js_obj.set(cx, "threshold", js_threshold)?;
            let js_per_recipient_shares = {
                let new_map_code = (cx).string("new Map()");
                let js_map =
                    neon::reflect::eval(cx, new_map_code)?.downcast_or_throw::<JsObject, _>(cx)?;
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsString::try_new(cx, {
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    })
                    .or_throw(cx)?;
                    let js_value = JsNumber::new(cx, {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    } as f64);
                    js_map
                        .call_method_with(cx, "set")?
                        .arg(js_key)
                        .arg(js_value)
                        .exec(cx)?;
                }
                js_map
            };
            js_obj.set(cx, "perRecipientShares", js_per_recipient_shares)?;
        }
        libparsec::SelfShamirRecoveryInfo::SetupButUnusable {
            created_on,
            created_by,
            threshold,
            per_recipient_shares,
            revoked_recipients,
            ..
        } => {
            let js_tag =
                JsString::try_new(cx, "SelfShamirRecoveryInfoSetupButUnusable").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_created_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "createdOn", js_created_on)?;
            let js_created_by = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "createdBy", js_created_by)?;
            let js_threshold = JsNumber::new(cx, {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            } as f64);
            js_obj.set(cx, "threshold", js_threshold)?;
            let js_per_recipient_shares = {
                let new_map_code = (cx).string("new Map()");
                let js_map =
                    neon::reflect::eval(cx, new_map_code)?.downcast_or_throw::<JsObject, _>(cx)?;
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsString::try_new(cx, {
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    })
                    .or_throw(cx)?;
                    let js_value = JsNumber::new(cx, {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    } as f64);
                    js_map
                        .call_method_with(cx, "set")?
                        .arg(js_key)
                        .arg(js_value)
                        .exec(cx)?;
                }
                js_map
            };
            js_obj.set(cx, "perRecipientShares", js_per_recipient_shares)?;
            let js_revoked_recipients = {
                // JsArray::new allocates with `undefined` value, that's why we `set` value
                let js_array = JsArray::new(cx, revoked_recipients.len());
                for (i, elem) in revoked_recipients.into_iter().enumerate() {
                    let js_elem = JsString::try_new(cx, {
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(elem) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    })
                    .or_throw(cx)?;
                    js_array.set(cx, i as u32, js_elem)?;
                }
                js_array
            };
            js_obj.set(cx, "revokedRecipients", js_revoked_recipients)?;
        }
        libparsec::SelfShamirRecoveryInfo::SetupWithRevokedRecipients {
            created_on,
            created_by,
            threshold,
            per_recipient_shares,
            revoked_recipients,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "SelfShamirRecoveryInfoSetupWithRevokedRecipients")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_created_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "createdOn", js_created_on)?;
            let js_created_by = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "createdBy", js_created_by)?;
            let js_threshold = JsNumber::new(cx, {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            } as f64);
            js_obj.set(cx, "threshold", js_threshold)?;
            let js_per_recipient_shares = {
                let new_map_code = (cx).string("new Map()");
                let js_map =
                    neon::reflect::eval(cx, new_map_code)?.downcast_or_throw::<JsObject, _>(cx)?;
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsString::try_new(cx, {
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    })
                    .or_throw(cx)?;
                    let js_value = JsNumber::new(cx, {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    } as f64);
                    js_map
                        .call_method_with(cx, "set")?
                        .arg(js_key)
                        .arg(js_value)
                        .exec(cx)?;
                }
                js_map
            };
            js_obj.set(cx, "perRecipientShares", js_per_recipient_shares)?;
            let js_revoked_recipients = {
                // JsArray::new allocates with `undefined` value, that's why we `set` value
                let js_array = JsArray::new(cx, revoked_recipients.len());
                for (i, elem) in revoked_recipients.into_iter().enumerate() {
                    let js_elem = JsString::try_new(cx, {
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(elem) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    })
                    .or_throw(cx)?;
                    js_array.set(cx, i as u32, js_elem)?;
                }
                js_array
            };
            js_obj.set(cx, "revokedRecipients", js_revoked_recipients)?;
        }
    }
    Ok(js_obj)
}

// ShamirRecoveryClaimAddShareError

#[allow(dead_code)]
fn variant_shamir_recovery_claim_add_share_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ShamirRecoveryClaimAddShareError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ShamirRecoveryClaimAddShareError::CorruptedSecret { .. } => {
            let js_tag = JsString::try_new(cx, "ShamirRecoveryClaimAddShareErrorCorruptedSecret")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ShamirRecoveryClaimAddShareError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "ShamirRecoveryClaimAddShareErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ShamirRecoveryClaimAddShareError::RecipientNotFound { .. } => {
            let js_tag = JsString::try_new(cx, "ShamirRecoveryClaimAddShareErrorRecipientNotFound")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ShamirRecoveryClaimMaybeFinalizeInfo

#[allow(dead_code)]
fn variant_shamir_recovery_claim_maybe_finalize_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ShamirRecoveryClaimMaybeFinalizeInfo> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "ShamirRecoveryClaimMaybeFinalizeInfoFinalize" => {
            let handle = {
                let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    let v = v as u32;
                    v
                }
            };
            Ok(libparsec::ShamirRecoveryClaimMaybeFinalizeInfo::Finalize { handle })
        }
        "ShamirRecoveryClaimMaybeFinalizeInfoOffline" => {
            let handle = {
                let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    let v = v as u32;
                    v
                }
            };
            Ok(libparsec::ShamirRecoveryClaimMaybeFinalizeInfo::Offline { handle })
        }
        _ => cx.throw_type_error("Object is not a ShamirRecoveryClaimMaybeFinalizeInfo"),
    }
}

#[allow(dead_code)]
fn variant_shamir_recovery_claim_maybe_finalize_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ShamirRecoveryClaimMaybeFinalizeInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::ShamirRecoveryClaimMaybeFinalizeInfo::Finalize { handle, .. } => {
            let js_tag = JsString::try_new(cx, "ShamirRecoveryClaimMaybeFinalizeInfoFinalize")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_handle = JsNumber::new(cx, handle as f64);
            js_obj.set(cx, "handle", js_handle)?;
        }
        libparsec::ShamirRecoveryClaimMaybeFinalizeInfo::Offline { handle, .. } => {
            let js_tag = JsString::try_new(cx, "ShamirRecoveryClaimMaybeFinalizeInfoOffline")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_handle = JsNumber::new(cx, handle as f64);
            js_obj.set(cx, "handle", js_handle)?;
        }
    }
    Ok(js_obj)
}

// ShamirRecoveryClaimMaybeRecoverDeviceInfo

#[allow(dead_code)]
fn variant_shamir_recovery_claim_maybe_recover_device_info_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::ShamirRecoveryClaimMaybeRecoverDeviceInfo> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient" => {
            let handle = {
                let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    let v = v as u32;
                    v
                }
            };
            let claimer_user_id = {
                let js_val: Handle<JsString> = obj.get(cx, "claimerUserId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                        libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let claimer_human_handle = {
                let js_val: Handle<JsObject> = obj.get(cx, "claimerHumanHandle")?;
                struct_human_handle_js_to_rs(cx, js_val)?
            };
            let shamir_recovery_created_on = {
                let js_val: Handle<JsNumber> = obj.get(cx, "shamirRecoveryCreatedOn")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let recipients = {
                let js_val: Handle<JsArray> = obj.get(cx, "recipients")?;
                {
                    let size = js_val.len(cx);
                    let mut v = Vec::with_capacity(size as usize);
                    for i in 0..size {
                        let js_item: Handle<JsObject> = js_val.get(cx, i)?;
                        v.push(struct_shamir_recovery_recipient_js_to_rs(cx, js_item)?);
                    }
                    v
                }
            };
            let threshold = {
                let js_val: Handle<JsNumber> = obj.get(cx, "threshold")?;
                {
                    let v = js_val.value(cx);
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        cx.throw_type_error("Not an u8 number")?
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let recovered_shares = {
                let js_val: Handle<JsObject> = obj.get(cx, "recoveredShares")?;
                {
                    let mut d = std::collections::HashMap::with_capacity(
                        js_val.get::<JsNumber, _, _>(cx, "size")?.value(cx) as usize,
                    );

                    let js_keys = js_val
                        .call_method_with(cx, "keys")?
                        .apply::<JsObject, _>(cx)?;
                    let js_values = js_val
                        .call_method_with(cx, "values")?
                        .apply::<JsObject, _>(cx)?;
                    let js_keys_next_cb = js_keys.call_method_with(cx, "next")?;
                    let js_values_next_cb = js_values.call_method_with(cx, "next")?;

                    loop {
                        let next_js_key = js_keys_next_cb.apply::<JsObject, _>(cx)?;
                        let next_js_value = js_values_next_cb.apply::<JsObject, _>(cx)?;

                        let keys_done = next_js_key.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        let values_done =
                            next_js_value.get::<JsBoolean, _, _>(cx, "done")?.value(cx);
                        match (keys_done, values_done) {
                            (true, true) => break,
                            (false, false) => (),
                            _ => unreachable!(),
                        }

                        let js_key = next_js_key.get::<JsString, _, _>(cx, "value")?;
                        let js_value = next_js_value.get::<JsNumber, _, _>(cx, "value")?;

                        let key = {
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::UserID, _> {
                                    libparsec::UserID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(js_key.value(cx)) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        let value = {
                            let v = js_value.value(cx);
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                cx.throw_type_error("Not an u8 number")?
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        };
                        d.insert(key, value);
                    }
                    d
                }
            };
            let is_recoverable = {
                let js_val: Handle<JsBoolean> = obj.get(cx, "isRecoverable")?;
                js_val.value(cx)
            };
            Ok(
                libparsec::ShamirRecoveryClaimMaybeRecoverDeviceInfo::PickRecipient {
                    handle,
                    claimer_user_id,
                    claimer_human_handle,
                    shamir_recovery_created_on,
                    recipients,
                    threshold,
                    recovered_shares,
                    is_recoverable,
                },
            )
        }
        "ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice" => {
            let handle = {
                let js_val: Handle<JsNumber> = obj.get(cx, "handle")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    let v = v as u32;
                    v
                }
            };
            let claimer_user_id = {
                let js_val: Handle<JsString> = obj.get(cx, "claimerUserId")?;
                {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                        libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_val.value(cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let claimer_human_handle = {
                let js_val: Handle<JsObject> = obj.get(cx, "claimerHumanHandle")?;
                struct_human_handle_js_to_rs(cx, js_val)?
            };
            Ok(
                libparsec::ShamirRecoveryClaimMaybeRecoverDeviceInfo::RecoverDevice {
                    handle,
                    claimer_user_id,
                    claimer_human_handle,
                },
            )
        }
        _ => cx.throw_type_error("Object is not a ShamirRecoveryClaimMaybeRecoverDeviceInfo"),
    }
}

#[allow(dead_code)]
fn variant_shamir_recovery_claim_maybe_recover_device_info_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ShamirRecoveryClaimMaybeRecoverDeviceInfo,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::ShamirRecoveryClaimMaybeRecoverDeviceInfo::PickRecipient {
            handle,
            claimer_user_id,
            claimer_human_handle,
            shamir_recovery_created_on,
            recipients,
            threshold,
            recovered_shares,
            is_recoverable,
            ..
        } => {
            let js_tag =
                JsString::try_new(cx, "ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_handle = JsNumber::new(cx, handle as f64);
            js_obj.set(cx, "handle", js_handle)?;
            let js_claimer_user_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(claimer_user_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "claimerUserId", js_claimer_user_id)?;
            let js_claimer_human_handle = struct_human_handle_rs_to_js(cx, claimer_human_handle)?;
            js_obj.set(cx, "claimerHumanHandle", js_claimer_human_handle)?;
            let js_shamir_recovery_created_on = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(shamir_recovery_created_on) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "shamirRecoveryCreatedOn", js_shamir_recovery_created_on)?;
            let js_recipients = {
                // JsArray::new allocates with `undefined` value, that's why we `set` value
                let js_array = JsArray::new(cx, recipients.len());
                for (i, elem) in recipients.into_iter().enumerate() {
                    let js_elem = struct_shamir_recovery_recipient_rs_to_js(cx, elem)?;
                    js_array.set(cx, i as u32, js_elem)?;
                }
                js_array
            };
            js_obj.set(cx, "recipients", js_recipients)?;
            let js_threshold = JsNumber::new(cx, {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            } as f64);
            js_obj.set(cx, "threshold", js_threshold)?;
            let js_recovered_shares = {
                let new_map_code = (cx).string("new Map()");
                let js_map =
                    neon::reflect::eval(cx, new_map_code)?.downcast_or_throw::<JsObject, _>(cx)?;
                for (key, value) in recovered_shares.into_iter() {
                    let js_key = JsString::try_new(cx, {
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    })
                    .or_throw(cx)?;
                    let js_value = JsNumber::new(cx, {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return cx.throw_type_error(err),
                        }
                    } as f64);
                    js_map
                        .call_method_with(cx, "set")?
                        .arg(js_key)
                        .arg(js_value)
                        .exec(cx)?;
                }
                js_map
            };
            js_obj.set(cx, "recoveredShares", js_recovered_shares)?;
            let js_is_recoverable = JsBoolean::new(cx, is_recoverable);
            js_obj.set(cx, "isRecoverable", js_is_recoverable)?;
        }
        libparsec::ShamirRecoveryClaimMaybeRecoverDeviceInfo::RecoverDevice {
            handle,
            claimer_user_id,
            claimer_human_handle,
            ..
        } => {
            let js_tag =
                JsString::try_new(cx, "ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_handle = JsNumber::new(cx, handle as f64);
            js_obj.set(cx, "handle", js_handle)?;
            let js_claimer_user_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(claimer_user_id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "claimerUserId", js_claimer_user_id)?;
            let js_claimer_human_handle = struct_human_handle_rs_to_js(cx, claimer_human_handle)?;
            js_obj.set(cx, "claimerHumanHandle", js_claimer_human_handle)?;
        }
    }
    Ok(js_obj)
}

// ShamirRecoveryClaimPickRecipientError

#[allow(dead_code)]
fn variant_shamir_recovery_claim_pick_recipient_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ShamirRecoveryClaimPickRecipientError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ShamirRecoveryClaimPickRecipientError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "ShamirRecoveryClaimPickRecipientErrorInternal")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ShamirRecoveryClaimPickRecipientError::RecipientAlreadyPicked { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "ShamirRecoveryClaimPickRecipientErrorRecipientAlreadyPicked",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ShamirRecoveryClaimPickRecipientError::RecipientNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "ShamirRecoveryClaimPickRecipientErrorRecipientNotFound")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ShamirRecoveryClaimPickRecipientError::RecipientRevoked { .. } => {
            let js_tag =
                JsString::try_new(cx, "ShamirRecoveryClaimPickRecipientErrorRecipientRevoked")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// ShamirRecoveryClaimRecoverDeviceError

#[allow(dead_code)]
fn variant_shamir_recovery_claim_recover_device_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::ShamirRecoveryClaimRecoverDeviceError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::ShamirRecoveryClaimRecoverDeviceError::AlreadyUsed { .. } => {
            let js_tag = JsString::try_new(cx, "ShamirRecoveryClaimRecoverDeviceErrorAlreadyUsed")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ShamirRecoveryClaimRecoverDeviceError::CipheredDataNotFound { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "ShamirRecoveryClaimRecoverDeviceErrorCipheredDataNotFound",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ShamirRecoveryClaimRecoverDeviceError::CorruptedCipheredData { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "ShamirRecoveryClaimRecoverDeviceErrorCorruptedCipheredData",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ShamirRecoveryClaimRecoverDeviceError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "ShamirRecoveryClaimRecoverDeviceErrorInternal")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ShamirRecoveryClaimRecoverDeviceError::NotFound { .. } => {
            let js_tag = JsString::try_new(cx, "ShamirRecoveryClaimRecoverDeviceErrorNotFound")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ShamirRecoveryClaimRecoverDeviceError::OrganizationExpired { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "ShamirRecoveryClaimRecoverDeviceErrorOrganizationExpired",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::ShamirRecoveryClaimRecoverDeviceError::RegisterNewDeviceError { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "ShamirRecoveryClaimRecoverDeviceErrorRegisterNewDeviceError",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// TestbedError

#[allow(dead_code)]
fn variant_testbed_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::TestbedError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::TestbedError::Disabled { .. } => {
            let js_tag = JsString::try_new(cx, "TestbedErrorDisabled").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::TestbedError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "TestbedErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// UserClaimListInitialInfosError

#[allow(dead_code)]
fn variant_user_claim_list_initial_infos_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::UserClaimListInitialInfosError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::UserClaimListInitialInfosError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "UserClaimListInitialInfosErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WaitForDeviceAvailableError

#[allow(dead_code)]
fn variant_wait_for_device_available_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WaitForDeviceAvailableError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WaitForDeviceAvailableError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WaitForDeviceAvailableErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceCreateFileError

#[allow(dead_code)]
fn variant_workspace_create_file_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceCreateFileError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceCreateFileError::EntryExists { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceCreateFileErrorEntryExists").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFileError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceCreateFileErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFileError::InvalidCertificate { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceCreateFileErrorInvalidCertificate").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFileError::InvalidKeysBundle { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceCreateFileErrorInvalidKeysBundle").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFileError::InvalidManifest { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceCreateFileErrorInvalidManifest").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFileError::NoRealmAccess { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceCreateFileErrorNoRealmAccess").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFileError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceCreateFileErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFileError::ParentNotAFolder { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceCreateFileErrorParentNotAFolder").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFileError::ParentNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceCreateFileErrorParentNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFileError::ReadOnlyRealm { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceCreateFileErrorReadOnlyRealm").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFileError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceCreateFileErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceCreateFolderError

#[allow(dead_code)]
fn variant_workspace_create_folder_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceCreateFolderError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceCreateFolderError::EntryExists { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceCreateFolderErrorEntryExists").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceCreateFolderErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceCreateFolderErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::InvalidKeysBundle { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceCreateFolderErrorInvalidKeysBundle")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::InvalidManifest { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceCreateFolderErrorInvalidManifest").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::NoRealmAccess { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceCreateFolderErrorNoRealmAccess").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceCreateFolderErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::ParentNotAFolder { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceCreateFolderErrorParentNotAFolder").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::ParentNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceCreateFolderErrorParentNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::ReadOnlyRealm { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceCreateFolderErrorReadOnlyRealm").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceCreateFolderErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceDecryptPathAddrError

#[allow(dead_code)]
fn variant_workspace_decrypt_path_addr_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceDecryptPathAddrError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceDecryptPathAddrError::CorruptedData { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceDecryptPathAddrErrorCorruptedData").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::CorruptedKey { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceDecryptPathAddrErrorCorruptedKey").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceDecryptPathAddrErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceDecryptPathAddrErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::InvalidKeysBundle { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceDecryptPathAddrErrorInvalidKeysBundle")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::KeyNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceDecryptPathAddrErrorKeyNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::NotAllowed { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceDecryptPathAddrErrorNotAllowed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceDecryptPathAddrErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceDecryptPathAddrErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceFdCloseError

#[allow(dead_code)]
fn variant_workspace_fd_close_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceFdCloseError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceFdCloseError::BadFileDescriptor { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceFdCloseErrorBadFileDescriptor").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdCloseError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceFdCloseErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdCloseError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceFdCloseErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceFdFlushError

#[allow(dead_code)]
fn variant_workspace_fd_flush_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceFdFlushError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceFdFlushError::BadFileDescriptor { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceFdFlushErrorBadFileDescriptor").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdFlushError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceFdFlushErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdFlushError::NotInWriteMode { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceFdFlushErrorNotInWriteMode").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdFlushError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceFdFlushErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceFdReadError

#[allow(dead_code)]
fn variant_workspace_fd_read_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceFdReadError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceFdReadError::BadFileDescriptor { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceFdReadErrorBadFileDescriptor").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdReadError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceFdReadErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdReadError::InvalidBlockAccess { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceFdReadErrorInvalidBlockAccess").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdReadError::InvalidCertificate { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceFdReadErrorInvalidCertificate").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdReadError::InvalidKeysBundle { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceFdReadErrorInvalidKeysBundle").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdReadError::NoRealmAccess { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceFdReadErrorNoRealmAccess").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdReadError::NotInReadMode { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceFdReadErrorNotInReadMode").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdReadError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceFdReadErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdReadError::ServerBlockstoreUnavailable { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceFdReadErrorServerBlockstoreUnavailable")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdReadError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceFdReadErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceFdResizeError

#[allow(dead_code)]
fn variant_workspace_fd_resize_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceFdResizeError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceFdResizeError::BadFileDescriptor { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceFdResizeErrorBadFileDescriptor").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdResizeError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceFdResizeErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdResizeError::NotInWriteMode { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceFdResizeErrorNotInWriteMode").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceFdStatError

#[allow(dead_code)]
fn variant_workspace_fd_stat_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceFdStatError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceFdStatError::BadFileDescriptor { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceFdStatErrorBadFileDescriptor").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdStatError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceFdStatErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceFdWriteError

#[allow(dead_code)]
fn variant_workspace_fd_write_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceFdWriteError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceFdWriteError::BadFileDescriptor { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceFdWriteErrorBadFileDescriptor").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdWriteError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceFdWriteErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceFdWriteError::NotInWriteMode { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceFdWriteErrorNotInWriteMode").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceGeneratePathAddrError

#[allow(dead_code)]
fn variant_workspace_generate_path_addr_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceGeneratePathAddrError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceGeneratePathAddrError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceGeneratePathAddrErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceGeneratePathAddrError::InvalidKeysBundle { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceGeneratePathAddrErrorInvalidKeysBundle")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceGeneratePathAddrError::NoKey { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceGeneratePathAddrErrorNoKey").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceGeneratePathAddrError::NotAllowed { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceGeneratePathAddrErrorNotAllowed").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceGeneratePathAddrError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceGeneratePathAddrErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceGeneratePathAddrError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceGeneratePathAddrErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistory2EntryStat

#[allow(dead_code)]
fn variant_workspace_history2_entry_stat_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::WorkspaceHistory2EntryStat> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "WorkspaceHistory2EntryStatFile" => {
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
            let parent = {
                let js_val: Handle<JsString> = obj.get(cx, "parent")?;
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
            let created = {
                let js_val: Handle<JsNumber> = obj.get(cx, "created")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let updated = {
                let js_val: Handle<JsNumber> = obj.get(cx, "updated")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let version = {
                let js_val: Handle<JsNumber> = obj.get(cx, "version")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    let v = v as u32;
                    v
                }
            };
            let size = {
                let js_val: Handle<JsNumber> = obj.get(cx, "size")?;
                {
                    let v = js_val.value(cx);
                    if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                        cx.throw_type_error("Not an u64 number")?
                    }
                    let v = v as u64;
                    v
                }
            };
            Ok(libparsec::WorkspaceHistory2EntryStat::File {
                id,
                parent,
                created,
                updated,
                version,
                size,
            })
        }
        "WorkspaceHistory2EntryStatFolder" => {
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
            let parent = {
                let js_val: Handle<JsString> = obj.get(cx, "parent")?;
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
            let created = {
                let js_val: Handle<JsNumber> = obj.get(cx, "created")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let updated = {
                let js_val: Handle<JsNumber> = obj.get(cx, "updated")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let version = {
                let js_val: Handle<JsNumber> = obj.get(cx, "version")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    let v = v as u32;
                    v
                }
            };
            Ok(libparsec::WorkspaceHistory2EntryStat::Folder {
                id,
                parent,
                created,
                updated,
                version,
            })
        }
        _ => cx.throw_type_error("Object is not a WorkspaceHistory2EntryStat"),
    }
}

#[allow(dead_code)]
fn variant_workspace_history2_entry_stat_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistory2EntryStat,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::WorkspaceHistory2EntryStat::File {
            id,
            parent,
            created,
            updated,
            version,
            size,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2EntryStatFile").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "id", js_id)?;
            let js_parent = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(parent) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "parent", js_parent)?;
            let js_created = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "created", js_created)?;
            let js_updated = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(updated) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "updated", js_updated)?;
            let js_version = JsNumber::new(cx, version as f64);
            js_obj.set(cx, "version", js_version)?;
            let js_size = JsNumber::new(cx, size as f64);
            js_obj.set(cx, "size", js_size)?;
        }
        libparsec::WorkspaceHistory2EntryStat::Folder {
            id,
            parent,
            created,
            updated,
            version,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2EntryStatFolder").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "id", js_id)?;
            let js_parent = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(parent) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "parent", js_parent)?;
            let js_created = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "created", js_created)?;
            let js_updated = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(updated) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "updated", js_updated)?;
            let js_version = JsNumber::new(cx, version as f64);
            js_obj.set(cx, "version", js_version)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistory2FdCloseError

#[allow(dead_code)]
fn variant_workspace_history2_fd_close_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistory2FdCloseError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceHistory2FdCloseError::BadFileDescriptor { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2FdCloseErrorBadFileDescriptor")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2FdCloseError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2FdCloseErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistory2FdReadError

#[allow(dead_code)]
fn variant_workspace_history2_fd_read_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistory2FdReadError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceHistory2FdReadError::BadFileDescriptor { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2FdReadErrorBadFileDescriptor")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2FdReadError::BlockNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2FdReadErrorBlockNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2FdReadError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2FdReadErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2FdReadError::InvalidBlockAccess { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2FdReadErrorInvalidBlockAccess")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2FdReadError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2FdReadErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2FdReadError::InvalidKeysBundle { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2FdReadErrorInvalidKeysBundle")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2FdReadError::NoRealmAccess { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2FdReadErrorNoRealmAccess").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2FdReadError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2FdReadErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2FdReadError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2FdReadErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistory2FdStatError

#[allow(dead_code)]
fn variant_workspace_history2_fd_stat_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistory2FdStatError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceHistory2FdStatError::BadFileDescriptor { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2FdStatErrorBadFileDescriptor")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2FdStatError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2FdStatErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistory2InternalOnlyError

#[allow(dead_code)]
fn variant_workspace_history2_internal_only_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistory2InternalOnlyError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceHistory2InternalOnlyError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2InternalOnlyErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistory2OpenFileError

#[allow(dead_code)]
fn variant_workspace_history2_open_file_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistory2OpenFileError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceHistory2OpenFileError::EntryNotAFile { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2OpenFileErrorEntryNotAFile")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2OpenFileError::EntryNotFound { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2OpenFileErrorEntryNotFound")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2OpenFileError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2OpenFileErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2OpenFileError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2OpenFileErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2OpenFileError::InvalidHistory { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2OpenFileErrorInvalidHistory")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2OpenFileError::InvalidKeysBundle { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2OpenFileErrorInvalidKeysBundle")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2OpenFileError::InvalidManifest { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2OpenFileErrorInvalidManifest")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2OpenFileError::NoRealmAccess { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2OpenFileErrorNoRealmAccess")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2OpenFileError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2OpenFileErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2OpenFileError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2OpenFileErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistory2SetTimestampOfInterestError

#[allow(dead_code)]
fn variant_workspace_history2_set_timestamp_of_interest_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistory2SetTimestampOfInterestError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceHistory2SetTimestampOfInterestError::EntryNotFound { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistory2SetTimestampOfInterestErrorEntryNotFound",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2SetTimestampOfInterestError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2SetTimestampOfInterestErrorInternal")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2SetTimestampOfInterestError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistory2SetTimestampOfInterestErrorInvalidCertificate",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2SetTimestampOfInterestError::InvalidHistory { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistory2SetTimestampOfInterestErrorInvalidHistory",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2SetTimestampOfInterestError::InvalidKeysBundle { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistory2SetTimestampOfInterestErrorInvalidKeysBundle",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2SetTimestampOfInterestError::InvalidManifest { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistory2SetTimestampOfInterestErrorInvalidManifest",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2SetTimestampOfInterestError::NewerThanHigherBound {
            ..
        } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistory2SetTimestampOfInterestErrorNewerThanHigherBound",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2SetTimestampOfInterestError::NoRealmAccess { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistory2SetTimestampOfInterestErrorNoRealmAccess",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2SetTimestampOfInterestError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2SetTimestampOfInterestErrorOffline")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2SetTimestampOfInterestError::OlderThanLowerBound { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistory2SetTimestampOfInterestErrorOlderThanLowerBound",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2SetTimestampOfInterestError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2SetTimestampOfInterestErrorStopped")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistory2StatEntryError

#[allow(dead_code)]
fn variant_workspace_history2_stat_entry_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistory2StatEntryError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceHistory2StatEntryError::EntryNotFound { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2StatEntryErrorEntryNotFound")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatEntryError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2StatEntryErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatEntryError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2StatEntryErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatEntryError::InvalidHistory { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2StatEntryErrorInvalidHistory")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatEntryError::InvalidKeysBundle { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2StatEntryErrorInvalidKeysBundle")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatEntryError::InvalidManifest { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2StatEntryErrorInvalidManifest")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatEntryError::NoRealmAccess { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2StatEntryErrorNoRealmAccess")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatEntryError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2StatEntryErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatEntryError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2StatEntryErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistory2StatFolderChildrenError

#[allow(dead_code)]
fn variant_workspace_history2_stat_folder_children_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistory2StatFolderChildrenError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceHistory2StatFolderChildrenError::EntryIsFile { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2StatFolderChildrenErrorEntryIsFile")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatFolderChildrenError::EntryNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2StatFolderChildrenErrorEntryNotFound")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatFolderChildrenError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2StatFolderChildrenErrorInternal")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatFolderChildrenError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistory2StatFolderChildrenErrorInvalidCertificate",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatFolderChildrenError::InvalidHistory { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2StatFolderChildrenErrorInvalidHistory")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatFolderChildrenError::InvalidKeysBundle { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistory2StatFolderChildrenErrorInvalidKeysBundle",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatFolderChildrenError::InvalidManifest { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistory2StatFolderChildrenErrorInvalidManifest",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatFolderChildrenError::NoRealmAccess { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistory2StatFolderChildrenErrorNoRealmAccess")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatFolderChildrenError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2StatFolderChildrenErrorOffline")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistory2StatFolderChildrenError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistory2StatFolderChildrenErrorStopped")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryEntryStat

#[allow(dead_code)]
fn variant_workspace_history_entry_stat_js_to_rs<'a>(
    cx: &mut impl Context<'a>,
    obj: Handle<'a, JsObject>,
) -> NeonResult<libparsec::WorkspaceHistoryEntryStat> {
    let tag = obj.get::<JsString, _, _>(cx, "tag")?.value(cx);
    match tag.as_str() {
        "WorkspaceHistoryEntryStatFile" => {
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
            let parent = {
                let js_val: Handle<JsString> = obj.get(cx, "parent")?;
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
            let created = {
                let js_val: Handle<JsNumber> = obj.get(cx, "created")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let updated = {
                let js_val: Handle<JsNumber> = obj.get(cx, "updated")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let version = {
                let js_val: Handle<JsNumber> = obj.get(cx, "version")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    let v = v as u32;
                    v
                }
            };
            let size = {
                let js_val: Handle<JsNumber> = obj.get(cx, "size")?;
                {
                    let v = js_val.value(cx);
                    if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                        cx.throw_type_error("Not an u64 number")?
                    }
                    let v = v as u64;
                    v
                }
            };
            Ok(libparsec::WorkspaceHistoryEntryStat::File {
                id,
                parent,
                created,
                updated,
                version,
                size,
            })
        }
        "WorkspaceHistoryEntryStatFolder" => {
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
            let parent = {
                let js_val: Handle<JsString> = obj.get(cx, "parent")?;
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
            let created = {
                let js_val: Handle<JsNumber> = obj.get(cx, "created")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let updated = {
                let js_val: Handle<JsNumber> = obj.get(cx, "updated")?;
                {
                    let v = js_val.value(cx);
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                }
            };
            let version = {
                let js_val: Handle<JsNumber> = obj.get(cx, "version")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    let v = v as u32;
                    v
                }
            };
            Ok(libparsec::WorkspaceHistoryEntryStat::Folder {
                id,
                parent,
                created,
                updated,
                version,
            })
        }
        _ => cx.throw_type_error("Object is not a WorkspaceHistoryEntryStat"),
    }
}

#[allow(dead_code)]
fn variant_workspace_history_entry_stat_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistoryEntryStat,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    match rs_obj {
        libparsec::WorkspaceHistoryEntryStat::File {
            id,
            parent,
            created,
            updated,
            version,
            size,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryEntryStatFile").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "id", js_id)?;
            let js_parent = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(parent) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "parent", js_parent)?;
            let js_created = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "created", js_created)?;
            let js_updated = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(updated) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "updated", js_updated)?;
            let js_version = JsNumber::new(cx, version as f64);
            js_obj.set(cx, "version", js_version)?;
            let js_size = JsNumber::new(cx, size as f64);
            js_obj.set(cx, "size", js_size)?;
        }
        libparsec::WorkspaceHistoryEntryStat::Folder {
            id,
            parent,
            created,
            updated,
            version,
            ..
        } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryEntryStatFolder").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_id = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(id) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "id", js_id)?;
            let js_parent = JsString::try_new(cx, {
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(parent) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            })
            .or_throw(cx)?;
            js_obj.set(cx, "parent", js_parent)?;
            let js_created = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "created", js_created)?;
            let js_updated = JsNumber::new(cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(updated) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(cx, "updated", js_updated)?;
            let js_version = JsNumber::new(cx, version as f64);
            js_obj.set(cx, "version", js_version)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryFdCloseError

#[allow(dead_code)]
fn variant_workspace_history_fd_close_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistoryFdCloseError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceHistoryFdCloseError::BadFileDescriptor { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryFdCloseErrorBadFileDescriptor")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryFdCloseError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryFdCloseErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryFdReadError

#[allow(dead_code)]
fn variant_workspace_history_fd_read_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistoryFdReadError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceHistoryFdReadError::BadFileDescriptor { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryFdReadErrorBadFileDescriptor")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryFdReadError::BlockNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryFdReadErrorBlockNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryFdReadError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryFdReadErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryFdReadError::InvalidBlockAccess { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryFdReadErrorInvalidBlockAccess")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryFdReadError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryFdReadErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryFdReadError::InvalidKeysBundle { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryFdReadErrorInvalidKeysBundle")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryFdReadError::NoRealmAccess { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryFdReadErrorNoRealmAccess").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryFdReadError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryFdReadErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryFdReadError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryFdReadErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryFdStatError

#[allow(dead_code)]
fn variant_workspace_history_fd_stat_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistoryFdStatError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceHistoryFdStatError::BadFileDescriptor { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryFdStatErrorBadFileDescriptor")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryFdStatError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryFdStatErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryGetWorkspaceManifestV1TimestampError

#[allow(dead_code)]
fn variant_workspace_history_get_workspace_manifest_v1_timestamp_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistoryGetWorkspaceManifestV1TimestampError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceHistoryGetWorkspaceManifestV1TimestampError::Internal { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInternal",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryGetWorkspaceManifestV1TimestampError::InvalidCertificate {
            ..
        } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInvalidCertificate",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryGetWorkspaceManifestV1TimestampError::InvalidKeysBundle {
            ..
        } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInvalidKeysBundle",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryGetWorkspaceManifestV1TimestampError::InvalidManifest {
            ..
        } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorInvalidManifest",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryGetWorkspaceManifestV1TimestampError::NoRealmAccess {
            ..
        } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorNoRealmAccess",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryGetWorkspaceManifestV1TimestampError::Offline { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorOffline",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryGetWorkspaceManifestV1TimestampError::Stopped { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistoryGetWorkspaceManifestV1TimestampErrorStopped",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryOpenFileError

#[allow(dead_code)]
fn variant_workspace_history_open_file_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistoryOpenFileError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceHistoryOpenFileError::EntryNotAFile { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryOpenFileErrorEntryNotAFile").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryOpenFileError::EntryNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryOpenFileErrorEntryNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryOpenFileError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryOpenFileErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryOpenFileError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryOpenFileErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryOpenFileError::InvalidKeysBundle { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryOpenFileErrorInvalidKeysBundle")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryOpenFileError::InvalidManifest { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryOpenFileErrorInvalidManifest")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryOpenFileError::NoRealmAccess { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryOpenFileErrorNoRealmAccess").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryOpenFileError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryOpenFileErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryOpenFileError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryOpenFileErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryStatEntryError

#[allow(dead_code)]
fn variant_workspace_history_stat_entry_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistoryStatEntryError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceHistoryStatEntryError::EntryNotFound { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryStatEntryErrorEntryNotFound")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryStatEntryError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryStatEntryErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryStatEntryError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryStatEntryErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryStatEntryError::InvalidKeysBundle { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryStatEntryErrorInvalidKeysBundle")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryStatEntryError::InvalidManifest { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryStatEntryErrorInvalidManifest")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryStatEntryError::NoRealmAccess { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryStatEntryErrorNoRealmAccess")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryStatEntryError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryStatEntryErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryStatEntryError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryStatEntryErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryStatFolderChildrenError

#[allow(dead_code)]
fn variant_workspace_history_stat_folder_children_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceHistoryStatFolderChildrenError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceHistoryStatFolderChildrenError::EntryIsFile { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryStatFolderChildrenErrorEntryIsFile")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::EntryNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryStatFolderChildrenErrorEntryNotFound")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryStatFolderChildrenErrorInternal")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistoryStatFolderChildrenErrorInvalidCertificate",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::InvalidKeysBundle { .. } => {
            let js_tag = JsString::try_new(
                cx,
                "WorkspaceHistoryStatFolderChildrenErrorInvalidKeysBundle",
            )
            .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::InvalidManifest { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryStatFolderChildrenErrorInvalidManifest")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::NoRealmAccess { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceHistoryStatFolderChildrenErrorNoRealmAccess")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryStatFolderChildrenErrorOffline")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceHistoryStatFolderChildrenErrorStopped")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceInfoError

#[allow(dead_code)]
fn variant_workspace_info_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceInfoError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceInfoError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceInfoErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceMountError

#[allow(dead_code)]
fn variant_workspace_mount_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceMountError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceMountError::Disabled { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceMountErrorDisabled").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceMountError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceMountErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceMoveEntryError

#[allow(dead_code)]
fn variant_workspace_move_entry_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceMoveEntryError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceMoveEntryError::CannotMoveRoot { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceMoveEntryErrorCannotMoveRoot").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::DestinationExists { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceMoveEntryErrorDestinationExists").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::DestinationNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceMoveEntryErrorDestinationNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceMoveEntryErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::InvalidCertificate { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceMoveEntryErrorInvalidCertificate").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::InvalidKeysBundle { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceMoveEntryErrorInvalidKeysBundle").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::InvalidManifest { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceMoveEntryErrorInvalidManifest").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::NoRealmAccess { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceMoveEntryErrorNoRealmAccess").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceMoveEntryErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::ReadOnlyRealm { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceMoveEntryErrorReadOnlyRealm").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::SourceNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceMoveEntryErrorSourceNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceMoveEntryErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceOpenFileError

#[allow(dead_code)]
fn variant_workspace_open_file_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceOpenFileError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceOpenFileError::EntryExistsInCreateNewMode { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceOpenFileErrorEntryExistsInCreateNewMode")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceOpenFileError::EntryNotAFile { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceOpenFileErrorEntryNotAFile").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceOpenFileError::EntryNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceOpenFileErrorEntryNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceOpenFileError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceOpenFileErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceOpenFileError::InvalidCertificate { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceOpenFileErrorInvalidCertificate").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceOpenFileError::InvalidKeysBundle { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceOpenFileErrorInvalidKeysBundle").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceOpenFileError::InvalidManifest { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceOpenFileErrorInvalidManifest").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceOpenFileError::NoRealmAccess { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceOpenFileErrorNoRealmAccess").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceOpenFileError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceOpenFileErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceOpenFileError::ReadOnlyRealm { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceOpenFileErrorReadOnlyRealm").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceOpenFileError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceOpenFileErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceRemoveEntryError

#[allow(dead_code)]
fn variant_workspace_remove_entry_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceRemoveEntryError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceRemoveEntryError::CannotRemoveRoot { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceRemoveEntryErrorCannotRemoveRoot").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::EntryIsFile { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceRemoveEntryErrorEntryIsFile").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::EntryIsFolder { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceRemoveEntryErrorEntryIsFolder").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::EntryIsNonEmptyFolder { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceRemoveEntryErrorEntryIsNonEmptyFolder")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::EntryNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceRemoveEntryErrorEntryNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceRemoveEntryErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceRemoveEntryErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::InvalidKeysBundle { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceRemoveEntryErrorInvalidKeysBundle").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::InvalidManifest { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceRemoveEntryErrorInvalidManifest").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::NoRealmAccess { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceRemoveEntryErrorNoRealmAccess").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceRemoveEntryErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::ReadOnlyRealm { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceRemoveEntryErrorReadOnlyRealm").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceRemoveEntryErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceStatEntryError

#[allow(dead_code)]
fn variant_workspace_stat_entry_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceStatEntryError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceStatEntryError::EntryNotFound { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceStatEntryErrorEntryNotFound").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceStatEntryError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceStatEntryErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceStatEntryError::InvalidCertificate { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceStatEntryErrorInvalidCertificate").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceStatEntryError::InvalidKeysBundle { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceStatEntryErrorInvalidKeysBundle").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceStatEntryError::InvalidManifest { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceStatEntryErrorInvalidManifest").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceStatEntryError::NoRealmAccess { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceStatEntryErrorNoRealmAccess").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceStatEntryError::Offline { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceStatEntryErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceStatEntryError::Stopped { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceStatEntryErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceStatFolderChildrenError

#[allow(dead_code)]
fn variant_workspace_stat_folder_children_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceStatFolderChildrenError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceStatFolderChildrenError::EntryIsFile { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceStatFolderChildrenErrorEntryIsFile")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::EntryNotFound { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceStatFolderChildrenErrorEntryNotFound")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceStatFolderChildrenErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::InvalidCertificate { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceStatFolderChildrenErrorInvalidCertificate")
                    .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::InvalidKeysBundle { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceStatFolderChildrenErrorInvalidKeysBundle")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::InvalidManifest { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceStatFolderChildrenErrorInvalidManifest")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::NoRealmAccess { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceStatFolderChildrenErrorNoRealmAccess")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceStatFolderChildrenErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceStatFolderChildrenErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceStopError

#[allow(dead_code)]
fn variant_workspace_stop_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceStopError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceStopError::Internal { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceStopErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
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
        "WorkspaceStorageCacheSizeCustom" => {
            let size = {
                let js_val: Handle<JsNumber> = obj.get(cx, "size")?;
                {
                    let v = js_val.value(cx);
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        cx.throw_type_error("Not an u32 number")?
                    }
                    let v = v as u32;
                    v
                }
            };
            Ok(libparsec::WorkspaceStorageCacheSize::Custom { size })
        }
        "WorkspaceStorageCacheSizeDefault" => Ok(libparsec::WorkspaceStorageCacheSize::Default {}),
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
            let js_tag = JsString::try_new(cx, "WorkspaceStorageCacheSizeCustom").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
            let js_size = JsNumber::new(cx, size as f64);
            js_obj.set(cx, "size", js_size)?;
        }
        libparsec::WorkspaceStorageCacheSize::Default { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceStorageCacheSizeDefault").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceWatchEntryOneShotError

#[allow(dead_code)]
fn variant_workspace_watch_entry_one_shot_error_rs_to_js<'a>(
    cx: &mut impl Context<'a>,
    rs_obj: libparsec::WorkspaceWatchEntryOneShotError,
) -> NeonResult<Handle<'a, JsObject>> {
    let js_obj = cx.empty_object();
    let js_display = JsString::try_new(cx, &rs_obj.to_string()).or_throw(cx)?;
    js_obj.set(cx, "error", js_display)?;
    match rs_obj {
        libparsec::WorkspaceWatchEntryOneShotError::EntryNotFound { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceWatchEntryOneShotErrorEntryNotFound")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceWatchEntryOneShotError::Internal { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceWatchEntryOneShotErrorInternal").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceWatchEntryOneShotError::InvalidCertificate { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceWatchEntryOneShotErrorInvalidCertificate")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceWatchEntryOneShotError::InvalidKeysBundle { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceWatchEntryOneShotErrorInvalidKeysBundle")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceWatchEntryOneShotError::InvalidManifest { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceWatchEntryOneShotErrorInvalidManifest")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceWatchEntryOneShotError::NoRealmAccess { .. } => {
            let js_tag = JsString::try_new(cx, "WorkspaceWatchEntryOneShotErrorNoRealmAccess")
                .or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceWatchEntryOneShotError::Offline { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceWatchEntryOneShotErrorOffline").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
        libparsec::WorkspaceWatchEntryOneShotError::Stopped { .. } => {
            let js_tag =
                JsString::try_new(cx, "WorkspaceWatchEntryOneShotErrorStopped").or_throw(cx)?;
            js_obj.set(cx, "tag", js_tag)?;
        }
    }
    Ok(js_obj)
}

// archive_device
fn archive_device(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let device_path = {
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
            let ret = libparsec::archive_device(&device_path).await;

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
                        let js_err = variant_archive_device_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// bootstrap_organization
fn bootstrap_organization(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
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
        std::sync::Arc::new(
            move |handle: libparsec::Handle, event: libparsec::ClientEvent| {
                let callback2 = callback.clone();
                callback.channel.send(move |mut cx| {
                    // TODO: log an error instead of panic ? (it is a bit harsh to crash
                    // the current task if an unrelated event handler has a bug...)
                    let js_event = variant_client_event_rs_to_js(&mut cx, event)?;
                    let js_handle = JsNumber::new(&mut cx, handle);
                    if let Some(ref js_fn) = callback2.js_fn {
                        js_fn
                            .to_inner(&mut cx)
                            .call_with(&cx)
                            .args((js_handle, js_event))
                            .apply::<JsValue, _>(&mut cx)?;
                    }
                    Ok(())
                });
            },
        ) as std::sync::Arc<dyn Fn(libparsec::Handle, libparsec::ClientEvent) + Send + Sync>
    };
    let bootstrap_organization_addr = {
        let js_val = cx.argument::<JsString>(2)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecOrganizationBootstrapAddr::from_any(&s).map_err(|e| e.to_string())
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
    let human_handle = {
        let js_val = cx.argument::<JsObject>(4)?;
        struct_human_handle_js_to_rs(&mut cx, js_val)?
    };
    let device_label = {
        let js_val = cx.argument::<JsString>(5)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
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

// build_parsec_organization_bootstrap_addr
fn build_parsec_organization_bootstrap_addr(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let addr = {
        let js_val = cx.argument::<JsString>(0)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
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
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let ret = libparsec::build_parsec_organization_bootstrap_addr(addr, organization_id);
    let js_ret = JsString::try_new(&mut cx, {
        let custom_to_rs_string =
            |addr: libparsec::ParsecOrganizationBootstrapAddr| -> Result<String, &'static str> {
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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
    crate::init_sentry();
    let handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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

// claimer_device_in_progress_1_do_deny_trust
fn claimer_device_in_progress_1_do_deny_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
                libparsec::claimer_device_in_progress_1_do_deny_trust(canceller, handle).await;

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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let requested_device_label = {
        let js_val = cx.argument::<JsString>(2)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
    crate::init_sentry();
    let handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_greeter_abort_operation(handle).await;

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
                        let js_err =
                            variant_claimer_greeter_abort_operation_error_rs_to_js(&mut cx, err)?;
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
    crate::init_sentry();
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
        std::sync::Arc::new(
            move |handle: libparsec::Handle, event: libparsec::ClientEvent| {
                let callback2 = callback.clone();
                callback.channel.send(move |mut cx| {
                    // TODO: log an error instead of panic ? (it is a bit harsh to crash
                    // the current task if an unrelated event handler has a bug...)
                    let js_event = variant_client_event_rs_to_js(&mut cx, event)?;
                    let js_handle = JsNumber::new(&mut cx, handle);
                    if let Some(ref js_fn) = callback2.js_fn {
                        js_fn
                            .to_inner(&mut cx)
                            .call_with(&cx)
                            .args((js_handle, js_event))
                            .apply::<JsValue, _>(&mut cx)?;
                    }
                    Ok(())
                });
            },
        ) as std::sync::Arc<dyn Fn(libparsec::Handle, libparsec::ClientEvent) + Send + Sync>
    };
    let addr = {
        let js_val = cx.argument::<JsString>(2)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecInvitationAddr::from_any(&s).map_err(|e| e.to_string())
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
                        let js_value = variant_any_claim_retrieved_info_rs_to_js(&mut cx, ok)?;
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

// claimer_shamir_recovery_add_share
fn claimer_shamir_recovery_add_share(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let recipient_pick_handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let share_handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let ret = libparsec::claimer_shamir_recovery_add_share(recipient_pick_handle, share_handle);
    let js_ret = match ret {
        Ok(ok) => {
            let js_obj = JsObject::new(&mut cx);
            let js_tag = JsBoolean::new(&mut cx, true);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_value =
                variant_shamir_recovery_claim_maybe_recover_device_info_rs_to_js(&mut cx, ok)?;
            js_obj.set(&mut cx, "value", js_value)?;
            js_obj
        }
        Err(err) => {
            let js_obj = cx.empty_object();
            let js_tag = JsBoolean::new(&mut cx, false);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_err = variant_shamir_recovery_claim_add_share_error_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// claimer_shamir_recovery_finalize_save_local_device
fn claimer_shamir_recovery_finalize_save_local_device(
    mut cx: FunctionContext,
) -> JsResult<JsPromise> {
    crate::init_sentry();
    let handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
            let ret = libparsec::claimer_shamir_recovery_finalize_save_local_device(
                handle,
                save_strategy,
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

// claimer_shamir_recovery_in_progress_1_do_deny_trust
fn claimer_shamir_recovery_in_progress_1_do_deny_trust(
    mut cx: FunctionContext,
) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
                libparsec::claimer_shamir_recovery_in_progress_1_do_deny_trust(canceller, handle)
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

// claimer_shamir_recovery_in_progress_1_do_signify_trust
fn claimer_shamir_recovery_in_progress_1_do_signify_trust(
    mut cx: FunctionContext,
) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_shamir_recovery_in_progress_1_do_signify_trust(
                canceller, handle,
            )
            .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value =
                            struct_shamir_recovery_claim_in_progress2_info_rs_to_js(&mut cx, ok)?;
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

// claimer_shamir_recovery_in_progress_2_do_wait_peer_trust
fn claimer_shamir_recovery_in_progress_2_do_wait_peer_trust(
    mut cx: FunctionContext,
) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_shamir_recovery_in_progress_2_do_wait_peer_trust(
                canceller, handle,
            )
            .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value =
                            struct_shamir_recovery_claim_in_progress3_info_rs_to_js(&mut cx, ok)?;
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

// claimer_shamir_recovery_in_progress_3_do_claim
fn claimer_shamir_recovery_in_progress_3_do_claim(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
                libparsec::claimer_shamir_recovery_in_progress_3_do_claim(canceller, handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value =
                            struct_shamir_recovery_claim_share_info_rs_to_js(&mut cx, ok)?;
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

// claimer_shamir_recovery_initial_do_wait_peer
fn claimer_shamir_recovery_initial_do_wait_peer(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
                libparsec::claimer_shamir_recovery_initial_do_wait_peer(canceller, handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value =
                            struct_shamir_recovery_claim_in_progress1_info_rs_to_js(&mut cx, ok)?;
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

// claimer_shamir_recovery_pick_recipient
fn claimer_shamir_recovery_pick_recipient(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let recipient_user_id = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let ret = libparsec::claimer_shamir_recovery_pick_recipient(handle, recipient_user_id);
    let js_ret = match ret {
        Ok(ok) => {
            let js_obj = JsObject::new(&mut cx);
            let js_tag = JsBoolean::new(&mut cx, true);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_value = struct_shamir_recovery_claim_initial_info_rs_to_js(&mut cx, ok)?;
            js_obj.set(&mut cx, "value", js_value)?;
            js_obj
        }
        Err(err) => {
            let js_obj = cx.empty_object();
            let js_tag = JsBoolean::new(&mut cx, false);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_err = variant_shamir_recovery_claim_pick_recipient_error_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// claimer_shamir_recovery_recover_device
fn claimer_shamir_recovery_recover_device(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let requested_device_label = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
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
            let ret =
                libparsec::claimer_shamir_recovery_recover_device(handle, requested_device_label)
                    .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = variant_shamir_recovery_claim_maybe_finalize_info_rs_to_js(
                            &mut cx, ok,
                        )?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_shamir_recovery_claim_recover_device_error_rs_to_js(
                            &mut cx, err,
                        )?;
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
    crate::init_sentry();
    let handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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

// claimer_user_in_progress_1_do_deny_trust
fn claimer_user_in_progress_1_do_deny_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_user_in_progress_1_do_deny_trust(canceller, handle).await;

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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let requested_device_label = {
        let js_val = cx.argument::<JsString>(2)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let requested_human_handle = {
        let js_val = cx.argument::<JsObject>(3)?;
        struct_human_handle_js_to_rs(&mut cx, js_val)?
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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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

// claimer_user_list_initial_info
fn claimer_user_list_initial_info(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let ret = libparsec::claimer_user_list_initial_info(handle);
    let js_ret = match ret {
        Ok(ok) => {
            let js_obj = JsObject::new(&mut cx);
            let js_tag = JsBoolean::new(&mut cx, true);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_value = {
                // JsArray::new allocates with `undefined` value, that's why we `set` value
                let js_array = JsArray::new(&mut cx, ok.len());
                for (i, elem) in ok.into_iter().enumerate() {
                    let js_elem = struct_user_claim_initial_info_rs_to_js(&mut cx, elem)?;
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
            let js_err = variant_user_claim_list_initial_infos_error_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// claimer_user_wait_all_peers
fn claimer_user_wait_all_peers(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::claimer_user_wait_all_peers(canceller, handle).await;

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

// client_accept_tos
fn client_accept_tos(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let tos_updated_on = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
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
            let ret = libparsec::client_accept_tos(client, tos_updated_on).await;

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
                        let js_err = variant_client_accept_tos_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_cancel_invitation
fn client_cancel_invitation(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
            let ret = libparsec::client_cancel_invitation(client, token).await;

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
                        let js_err = variant_client_cancel_invitation_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_change_authentication
fn client_change_authentication(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client_config = {
        let js_val = cx.argument::<JsObject>(0)?;
        struct_client_config_js_to_rs(&mut cx, js_val)?
    };
    let current_auth = {
        let js_val = cx.argument::<JsObject>(1)?;
        variant_device_access_strategy_js_to_rs(&mut cx, js_val)?
    };
    let new_auth = {
        let js_val = cx.argument::<JsObject>(2)?;
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
                libparsec::client_change_authentication(client_config, current_auth, new_auth)
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
                        let js_err =
                            variant_client_change_authentication_error_rs_to_js(&mut cx, err)?;
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
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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

// client_delete_shamir_recovery
fn client_delete_shamir_recovery(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client_handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_delete_shamir_recovery(client_handle).await;

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
                        let js_err =
                            variant_client_delete_shamir_recovery_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_export_recovery_device
fn client_export_recovery_device(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client_handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let device_label = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
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
            let ret = libparsec::client_export_recovery_device(client_handle, device_label).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let (x0, x1) = ok;
                            let js_array = JsArray::new(&mut cx, 2);
                            let js_value = JsString::try_new(&mut cx, x0).or_throw(&mut cx)?;
                            js_array.set(&mut cx, 0, js_value)?;
                            let js_value = {
                                let mut js_buff = JsArrayBuffer::new(&mut cx, x1.len())?;
                                let js_buff_slice = js_buff.as_mut_slice(&mut cx);
                                for (i, c) in x1.iter().enumerate() {
                                    js_buff_slice[i] = *c;
                                }
                                js_buff
                            };
                            js_array.set(&mut cx, 1, js_value)?;
                            js_array
                        };
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_client_export_recovery_device_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_get_self_shamir_recovery
fn client_get_self_shamir_recovery(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client_handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_get_self_shamir_recovery(client_handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = variant_self_shamir_recovery_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_client_get_self_shamir_recovery_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_get_tos
fn client_get_tos(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_get_tos(client).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_tos_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_client_get_tos_error_rs_to_js(&mut cx, err)?;
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
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let device = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
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
            let ret = libparsec::client_get_user_device(client, device).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let (x0, x1) = ok;
                            let js_array = JsArray::new(&mut cx, 2);
                            let js_value = struct_user_info_rs_to_js(&mut cx, x0)?;
                            js_array.set(&mut cx, 0, js_value)?;
                            let js_value = struct_device_info_rs_to_js(&mut cx, x1)?;
                            js_array.set(&mut cx, 1, js_value)?;
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
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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

// client_list_frozen_users
fn client_list_frozen_users(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client_handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_list_frozen_users(client_handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            // JsArray::new allocates with `undefined` value, that's why we `set` value
                            let js_array = JsArray::new(&mut cx, ok.len());
                            for (i, elem) in ok.into_iter().enumerate() {
                                let js_elem = JsString::try_new(&mut cx, {
                                    let custom_to_rs_string =
                                        |x: libparsec::UserID| -> Result<String, &'static str> {
                                            Ok(x.hex())
                                        };
                                    match custom_to_rs_string(elem) {
                                        Ok(ok) => ok,
                                        Err(err) => return cx.throw_type_error(err),
                                    }
                                })
                                .or_throw(&mut cx)?;
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
                        let js_err = variant_client_list_frozen_users_error_rs_to_js(&mut cx, err)?;
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
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
                            let js_array = JsArray::new(&mut cx, ok.len());
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

// client_list_shamir_recoveries_for_others
fn client_list_shamir_recoveries_for_others(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client_handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_list_shamir_recoveries_for_others(client_handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            // JsArray::new allocates with `undefined` value, that's why we `set` value
                            let js_array = JsArray::new(&mut cx, ok.len());
                            for (i, elem) in ok.into_iter().enumerate() {
                                let js_elem =
                                    variant_other_shamir_recovery_info_rs_to_js(&mut cx, elem)?;
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
                        let js_err =
                            variant_client_list_shamir_recoveries_for_others_error_rs_to_js(
                                &mut cx, err,
                            )?;
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
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let user = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
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
            let ret = libparsec::client_list_user_devices(client, user).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            // JsArray::new allocates with `undefined` value, that's why we `set` value
                            let js_array = JsArray::new(&mut cx, ok.len());
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
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
                            let js_array = JsArray::new(&mut cx, ok.len());
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

// client_list_workspace_users
fn client_list_workspace_users(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_list_workspace_users(client, realm_id).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            // JsArray::new allocates with `undefined` value, that's why we `set` value
                            let js_array = JsArray::new(&mut cx, ok.len());
                            for (i, elem) in ok.into_iter().enumerate() {
                                let js_elem =
                                    struct_workspace_user_access_info_rs_to_js(&mut cx, elem)?;
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
                        let js_err =
                            variant_client_list_workspace_users_error_rs_to_js(&mut cx, err)?;
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
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
                            let js_array = JsArray::new(&mut cx, ok.len());
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
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
                        let js_value = struct_new_invitation_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_client_new_device_invitation_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_new_shamir_recovery_invitation
fn client_new_shamir_recovery_invitation(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let claimer_user_id = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
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
            let ret = libparsec::client_new_shamir_recovery_invitation(
                client,
                claimer_user_id,
                send_email,
            )
            .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_new_invitation_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_client_new_shamir_recovery_invitation_error_rs_to_js(
                            &mut cx, err,
                        )?;
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
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
                        let js_value = struct_new_invitation_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_client_new_user_invitation_error_rs_to_js(&mut cx, err)?;
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
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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

// client_revoke_user
fn client_revoke_user(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let user = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
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
            let ret = libparsec::client_revoke_user(client, user).await;

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
                        let js_err = variant_client_revoke_user_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_setup_shamir_recovery
fn client_setup_shamir_recovery(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client_handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let per_recipient_shares = {
        let js_val = cx.argument::<JsObject>(1)?;
        {
            let mut d = std::collections::HashMap::with_capacity(
                js_val
                    .get::<JsNumber, _, _>(&mut cx, "size")?
                    .value(&mut cx) as usize,
            );

            let js_keys = js_val
                .call_method_with(&mut cx, "keys")?
                .apply::<JsObject, _>(&mut cx)?;
            let js_values = js_val
                .call_method_with(&mut cx, "values")?
                .apply::<JsObject, _>(&mut cx)?;
            let js_keys_next_cb = js_keys.call_method_with(&mut cx, "next")?;
            let js_values_next_cb = js_values.call_method_with(&mut cx, "next")?;

            loop {
                let next_js_key = js_keys_next_cb.apply::<JsObject, _>(&mut cx)?;
                let next_js_value = js_values_next_cb.apply::<JsObject, _>(&mut cx)?;

                let keys_done = next_js_key
                    .get::<JsBoolean, _, _>(&mut cx, "done")?
                    .value(&mut cx);
                let values_done = next_js_value
                    .get::<JsBoolean, _, _>(&mut cx, "done")?
                    .value(&mut cx);
                match (keys_done, values_done) {
                    (true, true) => break,
                    (false, false) => (),
                    _ => unreachable!(),
                }

                let js_key = next_js_key.get::<JsString, _, _>(&mut cx, "value")?;
                let js_value = next_js_value.get::<JsNumber, _, _>(&mut cx, "value")?;

                let key = {
                    let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                        libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(js_key.value(&mut cx)) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                };
                let value = {
                    let v = js_value.value(&mut cx);
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        cx.throw_type_error("Not an u8 number")?
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return cx.throw_type_error(err),
                    }
                };
                d.insert(key, value);
            }
            d
        }
    };
    let threshold = {
        let js_val = cx.argument::<JsNumber>(2)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                cx.throw_type_error("Not an u8 number")?
            }
            let v = v as u8;
            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
            };
            match custom_from_rs_u8(v) {
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
            let ret = libparsec::client_setup_shamir_recovery(
                client_handle,
                per_recipient_shares,
                threshold,
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
                        let js_err =
                            variant_client_setup_shamir_recovery_error_rs_to_js(&mut cx, err)?;
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
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
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
    crate::init_sentry();
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
        std::sync::Arc::new(
            move |handle: libparsec::Handle, event: libparsec::ClientEvent| {
                let callback2 = callback.clone();
                callback.channel.send(move |mut cx| {
                    // TODO: log an error instead of panic ? (it is a bit harsh to crash
                    // the current task if an unrelated event handler has a bug...)
                    let js_event = variant_client_event_rs_to_js(&mut cx, event)?;
                    let js_handle = JsNumber::new(&mut cx, handle);
                    if let Some(ref js_fn) = callback2.js_fn {
                        js_fn
                            .to_inner(&mut cx)
                            .call_with(&cx)
                            .args((js_handle, js_event))
                            .apply::<JsValue, _>(&mut cx)?;
                    }
                    Ok(())
                });
            },
        ) as std::sync::Arc<dyn Fn(libparsec::Handle, libparsec::ClientEvent) + Send + Sync>
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
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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

// client_start_shamir_recovery_invitation_greet
fn client_start_shamir_recovery_invitation_greet(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
            let ret = libparsec::client_start_shamir_recovery_invitation_greet(client, token).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value =
                            struct_shamir_recovery_greet_initial_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_client_start_shamir_recovery_invitation_greet_error_rs_to_js(
                                &mut cx, err,
                            )?;
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
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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

// client_start_workspace
fn client_start_workspace(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_start_workspace(client, realm_id).await;

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
                        let js_err = variant_client_start_workspace_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// client_start_workspace_history2
fn client_start_workspace_history2(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::client_start_workspace_history2(client, realm_id).await;

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
                        let js_err =
                            variant_client_start_workspace_history2_error_rs_to_js(&mut cx, err)?;
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
    crate::init_sentry();
    let client = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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

// client_update_user_profile
fn client_update_user_profile(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let client_handle = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let user = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let new_profile = {
        let js_val = cx.argument::<JsString>(2)?;
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
            let ret = libparsec::client_update_user_profile(client_handle, user, new_profile).await;

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
                        let js_err =
                            variant_client_user_update_profile_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// get_default_config_dir
fn get_default_config_dir(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let ret = libparsec::get_default_config_dir();
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
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// get_default_data_base_dir
fn get_default_data_base_dir(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let ret = libparsec::get_default_data_base_dir();
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
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// get_default_mountpoint_base_dir
fn get_default_mountpoint_base_dir(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let ret = libparsec::get_default_mountpoint_base_dir();
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
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// get_platform
fn get_platform(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let ret = libparsec::get_platform();
    let js_ret = JsString::try_new(&mut cx, enum_platform_rs_to_js(ret)).or_throw(&mut cx)?;
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// greeter_device_in_progress_1_do_wait_peer_trust
fn greeter_device_in_progress_1_do_wait_peer_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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

// greeter_device_in_progress_2_do_deny_trust
fn greeter_device_in_progress_2_do_deny_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
                libparsec::greeter_device_in_progress_2_do_deny_trust(canceller, handle).await;

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

// greeter_device_in_progress_2_do_signify_trust
fn greeter_device_in_progress_2_do_signify_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let device_label = {
        let js_val = cx.argument::<JsString>(2)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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

// greeter_shamir_recovery_in_progress_1_do_wait_peer_trust
fn greeter_shamir_recovery_in_progress_1_do_wait_peer_trust(
    mut cx: FunctionContext,
) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::greeter_shamir_recovery_in_progress_1_do_wait_peer_trust(
                canceller, handle,
            )
            .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value =
                            struct_shamir_recovery_greet_in_progress2_info_rs_to_js(&mut cx, ok)?;
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

// greeter_shamir_recovery_in_progress_2_do_deny_trust
fn greeter_shamir_recovery_in_progress_2_do_deny_trust(
    mut cx: FunctionContext,
) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
                libparsec::greeter_shamir_recovery_in_progress_2_do_deny_trust(canceller, handle)
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

// greeter_shamir_recovery_in_progress_2_do_signify_trust
fn greeter_shamir_recovery_in_progress_2_do_signify_trust(
    mut cx: FunctionContext,
) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::greeter_shamir_recovery_in_progress_2_do_signify_trust(
                canceller, handle,
            )
            .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value =
                            struct_shamir_recovery_greet_in_progress3_info_rs_to_js(&mut cx, ok)?;
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

// greeter_shamir_recovery_in_progress_3_do_get_claim_requests
fn greeter_shamir_recovery_in_progress_3_do_get_claim_requests(
    mut cx: FunctionContext,
) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::greeter_shamir_recovery_in_progress_3_do_get_claim_requests(
                canceller, handle,
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

// greeter_shamir_recovery_initial_do_wait_peer
fn greeter_shamir_recovery_initial_do_wait_peer(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
                libparsec::greeter_shamir_recovery_initial_do_wait_peer(canceller, handle).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value =
                            struct_shamir_recovery_greet_in_progress1_info_rs_to_js(&mut cx, ok)?;
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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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

// greeter_user_in_progress_2_do_deny_trust
fn greeter_user_in_progress_2_do_deny_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::greeter_user_in_progress_2_do_deny_trust(canceller, handle).await;

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

// greeter_user_in_progress_2_do_signify_trust
fn greeter_user_in_progress_2_do_signify_trust(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let human_handle = {
        let js_val = cx.argument::<JsObject>(2)?;
        struct_human_handle_js_to_rs(&mut cx, js_val)?
    };
    let device_label = {
        let js_val = cx.argument::<JsString>(3)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
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
    crate::init_sentry();
    let canceller = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let handle = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
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

// import_recovery_device
fn import_recovery_device(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let config = {
        let js_val = cx.argument::<JsObject>(0)?;
        struct_client_config_js_to_rs(&mut cx, js_val)?
    };
    let recovery_device = {
        let js_val = cx.argument::<JsTypedArray<u8>>(1)?;
        js_val.as_slice(&mut cx).to_vec()
    };
    let passphrase = {
        let js_val = cx.argument::<JsString>(2)?;
        js_val.value(&mut cx)
    };
    let device_label = {
        let js_val = cx.argument::<JsString>(3)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let save_strategy = {
        let js_val = cx.argument::<JsObject>(4)?;
        variant_device_save_strategy_js_to_rs(&mut cx, js_val)?
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::import_recovery_device(
                config,
                &recovery_device,
                passphrase,
                device_label,
                save_strategy,
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
                        let js_err = variant_import_recovery_device_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// init_libparsec
fn init_libparsec(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let config = {
        let js_val = cx.argument::<JsObject>(0)?;
        struct_client_config_js_to_rs(&mut cx, js_val)?
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            libparsec::init_libparsec(config).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = cx.null();
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// is_keyring_available
fn is_keyring_available(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let ret = libparsec::is_keyring_available();
    let js_ret = JsBoolean::new(&mut cx, ret);
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// list_available_devices
fn list_available_devices(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
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
                    let js_array = JsArray::new(&mut cx, ret.len());
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

// mountpoint_to_os_path
fn mountpoint_to_os_path(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let mountpoint = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let parsec_path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::mountpoint_to_os_path(mountpoint, parsec_path).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = JsString::try_new(&mut cx, {
                            let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                                path.into_os_string()
                                    .into_string()
                                    .map_err(|_| "Path contains non-utf8 characters")
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
                        let js_err = variant_mountpoint_to_os_path_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// mountpoint_unmount
fn mountpoint_unmount(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let mountpoint = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::mountpoint_unmount(mountpoint).await;

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
                        let js_err = variant_mountpoint_unmount_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// new_canceller
fn new_canceller(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let ret = libparsec::new_canceller();
    let js_ret = JsNumber::new(&mut cx, ret as f64);
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// parse_parsec_addr
fn parse_parsec_addr(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let url = {
        let js_val = cx.argument::<JsString>(0)?;
        js_val.value(&mut cx)
    };
    let ret = libparsec::parse_parsec_addr(&url);
    let js_ret = match ret {
        Ok(ok) => {
            let js_obj = JsObject::new(&mut cx);
            let js_tag = JsBoolean::new(&mut cx, true);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_value = variant_parsed_parsec_addr_rs_to_js(&mut cx, ok)?;
            js_obj.set(&mut cx, "value", js_value)?;
            js_obj
        }
        Err(err) => {
            let js_obj = cx.empty_object();
            let js_tag = JsBoolean::new(&mut cx, false);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_err = variant_parse_parsec_addr_error_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// path_filename
fn path_filename(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let path = {
        let js_val = cx.argument::<JsString>(0)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let ret = libparsec::path_filename(&path);
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

// path_join
fn path_join(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let parent = {
        let js_val = cx.argument::<JsString>(0)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let child = {
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
    let ret = libparsec::path_join(&parent, &child);
    let js_ret = JsString::try_new(&mut cx, {
        let custom_to_rs_string =
            |path: libparsec::FsPath| -> Result<_, &'static str> { Ok(path.to_string()) };
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

// path_normalize
fn path_normalize(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let path = {
        let js_val = cx.argument::<JsString>(0)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let ret = libparsec::path_normalize(path);
    let js_ret = JsString::try_new(&mut cx, {
        let custom_to_rs_string =
            |path: libparsec::FsPath| -> Result<_, &'static str> { Ok(path.to_string()) };
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

// path_parent
fn path_parent(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let path = {
        let js_val = cx.argument::<JsString>(0)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let ret = libparsec::path_parent(&path);
    let js_ret = JsString::try_new(&mut cx, {
        let custom_to_rs_string =
            |path: libparsec::FsPath| -> Result<_, &'static str> { Ok(path.to_string()) };
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

// path_split
fn path_split(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let path = {
        let js_val = cx.argument::<JsString>(0)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let ret = libparsec::path_split(&path);
    let js_ret = {
        // JsArray::new allocates with `undefined` value, that's why we `set` value
        let js_array = JsArray::new(&mut cx, ret.len());
        for (i, elem) in ret.into_iter().enumerate() {
            let js_elem = JsString::try_new(&mut cx, elem).or_throw(&mut cx)?;
            js_array.set(&mut cx, i as u32, js_elem)?;
        }
        js_array
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// test_drop_testbed
fn test_drop_testbed(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
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
            let ret = libparsec::test_drop_testbed(&path).await;

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
                        let js_err = variant_testbed_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// test_get_testbed_bootstrap_organization_addr
fn test_get_testbed_bootstrap_organization_addr(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
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
        Ok(ok) => {
            let js_obj = JsObject::new(&mut cx);
            let js_tag = JsBoolean::new(&mut cx, true);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_value = match ok {
    Some(elem) => {
        JsString::try_new(&mut cx,{
    let custom_to_rs_string = |addr: libparsec::ParsecOrganizationBootstrapAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) };
    match custom_to_rs_string(elem) {
        Ok(ok) => ok,
        Err(err) => return cx.throw_type_error(err),
    }
}).or_throw(&mut cx)?.as_value(&mut cx)
    }
    None => JsNull::new(&mut cx).as_value(&mut cx),
};
            js_obj.set(&mut cx, "value", js_value)?;
            js_obj
        }
        Err(err) => {
            let js_obj = cx.empty_object();
            let js_tag = JsBoolean::new(&mut cx, false);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_err = variant_testbed_error_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// test_get_testbed_organization_id
fn test_get_testbed_organization_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
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
        Ok(ok) => {
            let js_obj = JsObject::new(&mut cx);
            let js_tag = JsBoolean::new(&mut cx, true);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_value = match ok {
                Some(elem) => JsString::try_new(&mut cx, elem)
                    .or_throw(&mut cx)?
                    .as_value(&mut cx),
                None => JsNull::new(&mut cx).as_value(&mut cx),
            };
            js_obj.set(&mut cx, "value", js_value)?;
            js_obj
        }
        Err(err) => {
            let js_obj = cx.empty_object();
            let js_tag = JsBoolean::new(&mut cx, false);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_err = variant_testbed_error_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// test_new_testbed
fn test_new_testbed(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let template = {
        let js_val = cx.argument::<JsString>(0)?;
        js_val.value(&mut cx)
    };
    let test_server = match cx.argument_opt(1) {
        Some(v) => match v.downcast::<JsString, _>(&mut cx) {
            Ok(js_val) => Some({
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
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
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = JsString::try_new(&mut cx, {
                            let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                                path.into_os_string()
                                    .into_string()
                                    .map_err(|_| "Path contains non-utf8 characters")
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
                        let js_err = variant_testbed_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// validate_device_label
fn validate_device_label(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
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
    crate::init_sentry();
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
    crate::init_sentry();
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
    crate::init_sentry();
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
    crate::init_sentry();
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

// validate_organization_id
fn validate_organization_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let raw = {
        let js_val = cx.argument::<JsString>(0)?;
        js_val.value(&mut cx)
    };
    let ret = libparsec::validate_organization_id(&raw);
    let js_ret = JsBoolean::new(&mut cx, ret);
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// validate_path
fn validate_path(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
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

// wait_for_device_available
fn wait_for_device_available(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let config_dir = {
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
    let device_id = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
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
            let ret = libparsec::wait_for_device_available(&config_dir, device_id).await;

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
                        let js_err =
                            variant_wait_for_device_available_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_create_file
fn workspace_create_file(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_create_file(workspace, path).await;

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
                        let js_err = variant_workspace_create_file_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_create_folder
fn workspace_create_folder(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_create_folder(workspace, path).await;

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
                        let js_err = variant_workspace_create_folder_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_create_folder_all
fn workspace_create_folder_all(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_create_folder_all(workspace, path).await;

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
                        let js_err = variant_workspace_create_folder_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_decrypt_path_addr
fn workspace_decrypt_path_addr(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let link = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecWorkspacePathAddr::from_any(&s).map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_decrypt_path_addr(workspace, &link).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = JsString::try_new(&mut cx, {
                            let custom_to_rs_string =
                                |path: libparsec::FsPath| -> Result<_, &'static str> {
                                    Ok(path.to_string())
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
                        let js_err =
                            variant_workspace_decrypt_path_addr_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_fd_close
fn workspace_fd_close(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let fd = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            match custom_from_rs_u32(v) {
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
            let ret = libparsec::workspace_fd_close(workspace, fd).await;

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
                        let js_err = variant_workspace_fd_close_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_fd_flush
fn workspace_fd_flush(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let fd = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            match custom_from_rs_u32(v) {
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
            let ret = libparsec::workspace_fd_flush(workspace, fd).await;

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
                        let js_err = variant_workspace_fd_flush_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_fd_read
fn workspace_fd_read(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let fd = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            match custom_from_rs_u32(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let offset = {
        let js_val = cx.argument::<JsNumber>(2)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                cx.throw_type_error("Not an u64 number")?
            }
            let v = v as u64;
            v
        }
    };
    let size = {
        let js_val = cx.argument::<JsNumber>(3)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                cx.throw_type_error("Not an u64 number")?
            }
            let v = v as u64;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_fd_read(workspace, fd, offset, size).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let mut js_buff = JsArrayBuffer::new(&mut cx, ok.len())?;
                            let js_buff_slice = js_buff.as_mut_slice(&mut cx);
                            for (i, c) in ok.iter().enumerate() {
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
                        let js_err = variant_workspace_fd_read_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_fd_resize
fn workspace_fd_resize(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let fd = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            match custom_from_rs_u32(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let length = {
        let js_val = cx.argument::<JsNumber>(2)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                cx.throw_type_error("Not an u64 number")?
            }
            let v = v as u64;
            v
        }
    };
    let truncate_only = {
        let js_val = cx.argument::<JsBoolean>(3)?;
        js_val.value(&mut cx)
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_fd_resize(workspace, fd, length, truncate_only).await;

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
                        let js_err = variant_workspace_fd_resize_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_fd_stat
fn workspace_fd_stat(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let fd = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            match custom_from_rs_u32(v) {
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
            let ret = libparsec::workspace_fd_stat(workspace, fd).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_file_stat_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_workspace_fd_stat_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_fd_write
fn workspace_fd_write(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let fd = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            match custom_from_rs_u32(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let offset = {
        let js_val = cx.argument::<JsNumber>(2)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                cx.throw_type_error("Not an u64 number")?
            }
            let v = v as u64;
            v
        }
    };
    let data = {
        let js_val = cx.argument::<JsTypedArray<u8>>(3)?;
        js_val.as_slice(&mut cx).to_vec()
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_fd_write(workspace, fd, offset, &data).await;

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
                        let js_err = variant_workspace_fd_write_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_fd_write_constrained_io
fn workspace_fd_write_constrained_io(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let fd = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            match custom_from_rs_u32(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let offset = {
        let js_val = cx.argument::<JsNumber>(2)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                cx.throw_type_error("Not an u64 number")?
            }
            let v = v as u64;
            v
        }
    };
    let data = {
        let js_val = cx.argument::<JsTypedArray<u8>>(3)?;
        js_val.as_slice(&mut cx).to_vec()
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::workspace_fd_write_constrained_io(workspace, fd, offset, &data).await;

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
                        let js_err = variant_workspace_fd_write_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_fd_write_start_eof
fn workspace_fd_write_start_eof(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let fd = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            match custom_from_rs_u32(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let data = {
        let js_val = cx.argument::<JsTypedArray<u8>>(2)?;
        js_val.as_slice(&mut cx).to_vec()
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_fd_write_start_eof(workspace, fd, &data).await;

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
                        let js_err = variant_workspace_fd_write_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_generate_path_addr
fn workspace_generate_path_addr(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
    let _handle = crate::TOKIO_RUNTIME.lock().expect("Mutex is poisoned").spawn(async move {

        let ret = libparsec::workspace_generate_path_addr(
            workspace,
            &path,
        ).await;

        deferred.settle_with(&channel, move |mut cx| {
            let js_ret = match ret {
    Ok(ok) => {
        let js_obj = JsObject::new(&mut cx);
        let js_tag = JsBoolean::new(&mut cx, true);
        js_obj.set(&mut cx, "ok", js_tag)?;
        let js_value = JsString::try_new(&mut cx,{
    let custom_to_rs_string = |addr: libparsec::ParsecWorkspacePathAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) };
    match custom_to_rs_string(ok) {
        Ok(ok) => ok,
        Err(err) => return cx.throw_type_error(err),
    }
}).or_throw(&mut cx)?;
        js_obj.set(&mut cx, "value", js_value)?;
        js_obj
    }
    Err(err) => {
        let js_obj = cx.empty_object();
        let js_tag = JsBoolean::new(&mut cx, false);
        js_obj.set(&mut cx, "ok", js_tag)?;
        let js_err = variant_workspace_generate_path_addr_error_rs_to_js(&mut cx, err)?;
        js_obj.set(&mut cx, "error", js_err)?;
        js_obj
    }
};
            Ok(js_ret)
        });
    });

    Ok(promise)
}

// workspace_history2_fd_close
fn workspace_history2_fd_close(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace_history = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let fd = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            match custom_from_rs_u32(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let ret = libparsec::workspace_history2_fd_close(workspace_history, fd);
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
            let js_err = variant_workspace_history2_fd_close_error_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// workspace_history2_fd_read
fn workspace_history2_fd_read(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace_history = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let fd = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            match custom_from_rs_u32(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let offset = {
        let js_val = cx.argument::<JsNumber>(2)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                cx.throw_type_error("Not an u64 number")?
            }
            let v = v as u64;
            v
        }
    };
    let size = {
        let js_val = cx.argument::<JsNumber>(3)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                cx.throw_type_error("Not an u64 number")?
            }
            let v = v as u64;
            v
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
                libparsec::workspace_history2_fd_read(workspace_history, fd, offset, size).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let mut js_buff = JsArrayBuffer::new(&mut cx, ok.len())?;
                            let js_buff_slice = js_buff.as_mut_slice(&mut cx);
                            for (i, c) in ok.iter().enumerate() {
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
                        let js_err =
                            variant_workspace_history2_fd_read_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history2_fd_stat
fn workspace_history2_fd_stat(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace_history = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let fd = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            match custom_from_rs_u32(v) {
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
            let ret = libparsec::workspace_history2_fd_stat(workspace_history, fd).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_workspace_history2_file_stat_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_workspace_history2_fd_stat_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history2_get_timestamp_higher_bound
fn workspace_history2_get_timestamp_higher_bound(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace_history = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let ret = libparsec::workspace_history2_get_timestamp_higher_bound(workspace_history);
    let js_ret = match ret {
        Ok(ok) => {
            let js_obj = JsObject::new(&mut cx);
            let js_tag = JsBoolean::new(&mut cx, true);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_value = JsNumber::new(&mut cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(ok) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(&mut cx, "value", js_value)?;
            js_obj
        }
        Err(err) => {
            let js_obj = cx.empty_object();
            let js_tag = JsBoolean::new(&mut cx, false);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_err = variant_workspace_history2_internal_only_error_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// workspace_history2_get_timestamp_lower_bound
fn workspace_history2_get_timestamp_lower_bound(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace_history = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let ret = libparsec::workspace_history2_get_timestamp_lower_bound(workspace_history);
    let js_ret = match ret {
        Ok(ok) => {
            let js_obj = JsObject::new(&mut cx);
            let js_tag = JsBoolean::new(&mut cx, true);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_value = JsNumber::new(&mut cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(ok) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(&mut cx, "value", js_value)?;
            js_obj
        }
        Err(err) => {
            let js_obj = cx.empty_object();
            let js_tag = JsBoolean::new(&mut cx, false);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_err = variant_workspace_history2_internal_only_error_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// workspace_history2_get_timestamp_of_interest
fn workspace_history2_get_timestamp_of_interest(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace_history = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let ret = libparsec::workspace_history2_get_timestamp_of_interest(workspace_history);
    let js_ret = match ret {
        Ok(ok) => {
            let js_obj = JsObject::new(&mut cx);
            let js_tag = JsBoolean::new(&mut cx, true);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_value = JsNumber::new(&mut cx, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(ok) {
                    Ok(ok) => ok,
                    Err(err) => return cx.throw_type_error(err),
                }
            });
            js_obj.set(&mut cx, "value", js_value)?;
            js_obj
        }
        Err(err) => {
            let js_obj = cx.empty_object();
            let js_tag = JsBoolean::new(&mut cx, false);
            js_obj.set(&mut cx, "ok", js_tag)?;
            let js_err = variant_workspace_history2_internal_only_error_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// workspace_history2_open_file
fn workspace_history2_open_file(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace_history = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_history2_open_file(workspace_history, path).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = JsNumber::new(&mut cx, {
                            let custom_to_rs_u32 =
                                |fd: libparsec::FileDescriptor| -> Result<_, &'static str> {
                                    Ok(fd.0)
                                };
                            match custom_to_rs_u32(ok) {
                                Ok(ok) => ok,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        } as f64);
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_workspace_history2_open_file_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history2_open_file_and_get_id
fn workspace_history2_open_file_and_get_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace_history = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret =
                libparsec::workspace_history2_open_file_and_get_id(workspace_history, path).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let (x0, x1) = ok;
                            let js_array = JsArray::new(&mut cx, 2);
                            let js_value = JsNumber::new(&mut cx, {
                                let custom_to_rs_u32 =
                                    |fd: libparsec::FileDescriptor| -> Result<_, &'static str> {
                                        Ok(fd.0)
                                    };
                                match custom_to_rs_u32(x0) {
                                    Ok(ok) => ok,
                                    Err(err) => return cx.throw_type_error(err),
                                }
                            }
                                as f64);
                            js_array.set(&mut cx, 0, js_value)?;
                            let js_value = JsString::try_new(&mut cx, {
                                let custom_to_rs_string =
                                    |x: libparsec::VlobID| -> Result<String, &'static str> {
                                        Ok(x.hex())
                                    };
                                match custom_to_rs_string(x1) {
                                    Ok(ok) => ok,
                                    Err(err) => return cx.throw_type_error(err),
                                }
                            })
                            .or_throw(&mut cx)?;
                            js_array.set(&mut cx, 1, js_value)?;
                            js_array
                        };
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_workspace_history2_open_file_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history2_open_file_by_id
fn workspace_history2_open_file_by_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace_history = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let entry_id = {
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::workspace_history2_open_file_by_id(workspace_history, entry_id).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = JsNumber::new(&mut cx, {
                            let custom_to_rs_u32 =
                                |fd: libparsec::FileDescriptor| -> Result<_, &'static str> {
                                    Ok(fd.0)
                                };
                            match custom_to_rs_u32(ok) {
                                Ok(ok) => ok,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        } as f64);
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_workspace_history2_open_file_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history2_set_timestamp_of_interest
fn workspace_history2_set_timestamp_of_interest(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace_history = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let toi = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
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
            let ret =
                libparsec::workspace_history2_set_timestamp_of_interest(workspace_history, toi)
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
                        let js_err =
                            variant_workspace_history2_set_timestamp_of_interest_error_rs_to_js(
                                &mut cx, err,
                            )?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history2_stat_entry
fn workspace_history2_stat_entry(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace_history = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_history2_stat_entry(workspace_history, &path).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = variant_workspace_history2_entry_stat_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_workspace_history2_stat_entry_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history2_stat_entry_by_id
fn workspace_history2_stat_entry_by_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace_history = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let entry_id = {
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::workspace_history2_stat_entry_by_id(workspace_history, entry_id).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = variant_workspace_history2_entry_stat_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_workspace_history2_stat_entry_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history2_stat_folder_children
fn workspace_history2_stat_folder_children(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace_history = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret =
                libparsec::workspace_history2_stat_folder_children(workspace_history, &path).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            // JsArray::new allocates with `undefined` value, that's why we `set` value
                            let js_array = JsArray::new(&mut cx, ok.len());
                            for (i, elem) in ok.into_iter().enumerate() {
                                let js_elem = {
                                    let (x0, x1) = elem;
                                    let js_array = JsArray::new(&mut cx, 2);
                                    let js_value =
                                        JsString::try_new(&mut cx, x0).or_throw(&mut cx)?;
                                    js_array.set(&mut cx, 0, js_value)?;
                                    let js_value = variant_workspace_history2_entry_stat_rs_to_js(
                                        &mut cx, x1,
                                    )?;
                                    js_array.set(&mut cx, 1, js_value)?;
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
                        let js_err =
                            variant_workspace_history2_stat_folder_children_error_rs_to_js(
                                &mut cx, err,
                            )?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history2_stat_folder_children_by_id
fn workspace_history2_stat_folder_children_by_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace_history = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let entry_id = {
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_history2_stat_folder_children_by_id(
                workspace_history,
                entry_id,
            )
            .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            // JsArray::new allocates with `undefined` value, that's why we `set` value
                            let js_array = JsArray::new(&mut cx, ok.len());
                            for (i, elem) in ok.into_iter().enumerate() {
                                let js_elem = {
                                    let (x0, x1) = elem;
                                    let js_array = JsArray::new(&mut cx, 2);
                                    let js_value =
                                        JsString::try_new(&mut cx, x0).or_throw(&mut cx)?;
                                    js_array.set(&mut cx, 0, js_value)?;
                                    let js_value = variant_workspace_history2_entry_stat_rs_to_js(
                                        &mut cx, x1,
                                    )?;
                                    js_array.set(&mut cx, 1, js_value)?;
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
                        let js_err =
                            variant_workspace_history2_stat_folder_children_error_rs_to_js(
                                &mut cx, err,
                            )?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history2_stop
fn workspace_history2_stop(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace_history = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let ret = libparsec::workspace_history2_stop(workspace_history);
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
            let js_err = variant_workspace_history2_internal_only_error_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// workspace_history_fd_close
fn workspace_history_fd_close(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let fd = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            match custom_from_rs_u32(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let ret = libparsec::workspace_history_fd_close(workspace, fd);
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
            let js_err = variant_workspace_history_fd_close_error_rs_to_js(&mut cx, err)?;
            js_obj.set(&mut cx, "error", js_err)?;
            js_obj
        }
    };
    let (deferred, promise) = cx.promise();
    deferred.resolve(&mut cx, js_ret);
    Ok(promise)
}

// workspace_history_fd_read
fn workspace_history_fd_read(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let fd = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            match custom_from_rs_u32(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let offset = {
        let js_val = cx.argument::<JsNumber>(2)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                cx.throw_type_error("Not an u64 number")?
            }
            let v = v as u64;
            v
        }
    };
    let size = {
        let js_val = cx.argument::<JsNumber>(3)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u64::MIN as f64) || (u64::MAX as f64) < v {
                cx.throw_type_error("Not an u64 number")?
            }
            let v = v as u64;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_history_fd_read(workspace, fd, offset, size).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let mut js_buff = JsArrayBuffer::new(&mut cx, ok.len())?;
                            let js_buff_slice = js_buff.as_mut_slice(&mut cx);
                            for (i, c) in ok.iter().enumerate() {
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
                        let js_err =
                            variant_workspace_history_fd_read_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history_fd_stat
fn workspace_history_fd_stat(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let fd = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            match custom_from_rs_u32(v) {
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
            let ret = libparsec::workspace_history_fd_stat(workspace, fd).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_workspace_history_file_stat_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_workspace_history_fd_stat_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history_get_workspace_manifest_v1_timestamp
fn workspace_history_get_workspace_manifest_v1_timestamp(
    mut cx: FunctionContext,
) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME.lock().expect("Mutex is poisoned").spawn(async move {

        let ret = libparsec::workspace_history_get_workspace_manifest_v1_timestamp(
            workspace,
        ).await;

        deferred.settle_with(&channel, move |mut cx| {
            let js_ret = match ret {
    Ok(ok) => {
        let js_obj = JsObject::new(&mut cx);
        let js_tag = JsBoolean::new(&mut cx, true);
        js_obj.set(&mut cx, "ok", js_tag)?;
        let js_value = match ok {
    Some(elem) => {
        JsNumber::new(&mut cx,{
    let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> { Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64) };
    match custom_to_rs_f64(elem) {
        Ok(ok) => ok,
        Err(err) => return cx.throw_type_error(err),
    }
}).as_value(&mut cx)
    }
    None => JsNull::new(&mut cx).as_value(&mut cx),
};
        js_obj.set(&mut cx, "value", js_value)?;
        js_obj
    }
    Err(err) => {
        let js_obj = cx.empty_object();
        let js_tag = JsBoolean::new(&mut cx, false);
        js_obj.set(&mut cx, "ok", js_tag)?;
        let js_err = variant_workspace_history_get_workspace_manifest_v1_timestamp_error_rs_to_js(&mut cx, err)?;
        js_obj.set(&mut cx, "error", js_err)?;
        js_obj
    }
};
            Ok(js_ret)
        });
    });

    Ok(promise)
}

// workspace_history_open_file
fn workspace_history_open_file(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let at = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(2)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_history_open_file(workspace, at, path).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = JsNumber::new(&mut cx, {
                            let custom_to_rs_u32 =
                                |fd: libparsec::FileDescriptor| -> Result<_, &'static str> {
                                    Ok(fd.0)
                                };
                            match custom_to_rs_u32(ok) {
                                Ok(ok) => ok,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        } as f64);
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_workspace_history_open_file_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history_open_file_by_id
fn workspace_history_open_file_by_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let at = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let entry_id = {
        let js_val = cx.argument::<JsString>(2)?;
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_history_open_file_by_id(workspace, at, entry_id).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = JsNumber::new(&mut cx, {
                            let custom_to_rs_u32 =
                                |fd: libparsec::FileDescriptor| -> Result<_, &'static str> {
                                    Ok(fd.0)
                                };
                            match custom_to_rs_u32(ok) {
                                Ok(ok) => ok,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        } as f64);
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_workspace_history_open_file_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history_stat_entry
fn workspace_history_stat_entry(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let at = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(2)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_history_stat_entry(workspace, at, &path).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = variant_workspace_history_entry_stat_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_workspace_history_stat_entry_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history_stat_entry_by_id
fn workspace_history_stat_entry_by_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let at = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let entry_id = {
        let js_val = cx.argument::<JsString>(2)?;
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_history_stat_entry_by_id(workspace, at, entry_id).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = variant_workspace_history_entry_stat_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err =
                            variant_workspace_history_stat_entry_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history_stat_folder_children
fn workspace_history_stat_folder_children(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let at = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(2)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_history_stat_folder_children(workspace, at, &path).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            // JsArray::new allocates with `undefined` value, that's why we `set` value
                            let js_array = JsArray::new(&mut cx, ok.len());
                            for (i, elem) in ok.into_iter().enumerate() {
                                let js_elem = {
                                    let (x0, x1) = elem;
                                    let js_array = JsArray::new(&mut cx, 2);
                                    let js_value =
                                        JsString::try_new(&mut cx, x0).or_throw(&mut cx)?;
                                    js_array.set(&mut cx, 0, js_value)?;
                                    let js_value =
                                        variant_workspace_history_entry_stat_rs_to_js(&mut cx, x1)?;
                                    js_array.set(&mut cx, 1, js_value)?;
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
                        let js_err = variant_workspace_history_stat_folder_children_error_rs_to_js(
                            &mut cx, err,
                        )?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_history_stat_folder_children_by_id
fn workspace_history_stat_folder_children_by_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let at = {
        let js_val = cx.argument::<JsNumber>(1)?;
        {
            let v = js_val.value(&mut cx);
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let entry_id = {
        let js_val = cx.argument::<JsString>(2)?;
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::workspace_history_stat_folder_children_by_id(workspace, at, entry_id)
                    .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            // JsArray::new allocates with `undefined` value, that's why we `set` value
                            let js_array = JsArray::new(&mut cx, ok.len());
                            for (i, elem) in ok.into_iter().enumerate() {
                                let js_elem = {
                                    let (x0, x1) = elem;
                                    let js_array = JsArray::new(&mut cx, 2);
                                    let js_value =
                                        JsString::try_new(&mut cx, x0).or_throw(&mut cx)?;
                                    js_array.set(&mut cx, 0, js_value)?;
                                    let js_value =
                                        variant_workspace_history_entry_stat_rs_to_js(&mut cx, x1)?;
                                    js_array.set(&mut cx, 1, js_value)?;
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
                        let js_err = variant_workspace_history_stat_folder_children_error_rs_to_js(
                            &mut cx, err,
                        )?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_info
fn workspace_info(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_info(workspace).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = struct_started_workspace_info_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_workspace_info_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_mount
fn workspace_mount(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_mount(workspace).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let (x0, x1) = ok;
                            let js_array = JsArray::new(&mut cx, 2);
                            let js_value = JsNumber::new(&mut cx, x0 as f64);
                            js_array.set(&mut cx, 0, js_value)?;
                            let js_value = JsString::try_new(&mut cx, {
                                let custom_to_rs_string =
                                    |path: std::path::PathBuf| -> Result<_, _> {
                                        path.into_os_string()
                                            .into_string()
                                            .map_err(|_| "Path contains non-utf8 characters")
                                    };
                                match custom_to_rs_string(x1) {
                                    Ok(ok) => ok,
                                    Err(err) => return cx.throw_type_error(err),
                                }
                            })
                            .or_throw(&mut cx)?;
                            js_array.set(&mut cx, 1, js_value)?;
                            js_array
                        };
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_workspace_mount_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_move_entry
fn workspace_move_entry(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let src = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let dst = {
        let js_val = cx.argument::<JsString>(2)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let mode = {
        let js_val = cx.argument::<JsObject>(3)?;
        variant_move_entry_mode_js_to_rs(&mut cx, js_val)?
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_move_entry(workspace, src, dst, mode).await;

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
                        let js_err = variant_workspace_move_entry_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_open_file
fn workspace_open_file(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let mode = {
        let js_val = cx.argument::<JsObject>(2)?;
        struct_open_options_js_to_rs(&mut cx, js_val)?
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_open_file(workspace, path, mode).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = JsNumber::new(&mut cx, {
                            let custom_to_rs_u32 =
                                |fd: libparsec::FileDescriptor| -> Result<_, &'static str> {
                                    Ok(fd.0)
                                };
                            match custom_to_rs_u32(ok) {
                                Ok(ok) => ok,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        } as f64);
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_workspace_open_file_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_open_file_and_get_id
fn workspace_open_file_and_get_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(js_val.value(&mut cx)) {
                Ok(val) => val,
                Err(err) => return cx.throw_type_error(err),
            }
        }
    };
    let mode = {
        let js_val = cx.argument::<JsObject>(2)?;
        struct_open_options_js_to_rs(&mut cx, js_val)?
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_open_file_and_get_id(workspace, path, mode).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            let (x0, x1) = ok;
                            let js_array = JsArray::new(&mut cx, 2);
                            let js_value = JsNumber::new(&mut cx, {
                                let custom_to_rs_u32 =
                                    |fd: libparsec::FileDescriptor| -> Result<_, &'static str> {
                                        Ok(fd.0)
                                    };
                                match custom_to_rs_u32(x0) {
                                    Ok(ok) => ok,
                                    Err(err) => return cx.throw_type_error(err),
                                }
                            }
                                as f64);
                            js_array.set(&mut cx, 0, js_value)?;
                            let js_value = JsString::try_new(&mut cx, {
                                let custom_to_rs_string =
                                    |x: libparsec::VlobID| -> Result<String, &'static str> {
                                        Ok(x.hex())
                                    };
                                match custom_to_rs_string(x1) {
                                    Ok(ok) => ok,
                                    Err(err) => return cx.throw_type_error(err),
                                }
                            })
                            .or_throw(&mut cx)?;
                            js_array.set(&mut cx, 1, js_value)?;
                            js_array
                        };
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_workspace_open_file_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_open_file_by_id
fn workspace_open_file_by_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let entry_id = {
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
    let mode = {
        let js_val = cx.argument::<JsObject>(2)?;
        struct_open_options_js_to_rs(&mut cx, js_val)?
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_open_file_by_id(workspace, entry_id, mode).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = JsNumber::new(&mut cx, {
                            let custom_to_rs_u32 =
                                |fd: libparsec::FileDescriptor| -> Result<_, &'static str> {
                                    Ok(fd.0)
                                };
                            match custom_to_rs_u32(ok) {
                                Ok(ok) => ok,
                                Err(err) => return cx.throw_type_error(err),
                            }
                        } as f64);
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_workspace_open_file_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_remove_entry
fn workspace_remove_entry(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_remove_entry(workspace, path).await;

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
                        let js_err = variant_workspace_remove_entry_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_remove_file
fn workspace_remove_file(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_remove_file(workspace, path).await;

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
                        let js_err = variant_workspace_remove_entry_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_remove_folder
fn workspace_remove_folder(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_remove_folder(workspace, path).await;

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
                        let js_err = variant_workspace_remove_entry_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_remove_folder_all
fn workspace_remove_folder_all(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_remove_folder_all(workspace, path).await;

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
                        let js_err = variant_workspace_remove_entry_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_rename_entry_by_id
fn workspace_rename_entry_by_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let src_parent_id = {
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
    let src_name = {
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
    let dst_name = {
        let js_val = cx.argument::<JsString>(3)?;
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
    let mode = {
        let js_val = cx.argument::<JsObject>(4)?;
        variant_move_entry_mode_js_to_rs(&mut cx, js_val)?
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_rename_entry_by_id(
                workspace,
                src_parent_id,
                src_name,
                dst_name,
                mode,
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
                        let js_err = variant_workspace_move_entry_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_stat_entry
fn workspace_stat_entry(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_stat_entry(workspace, &path).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = variant_entry_stat_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_workspace_stat_entry_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_stat_entry_by_id
fn workspace_stat_entry_by_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let entry_id = {
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_stat_entry_by_id(workspace, entry_id).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = variant_entry_stat_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_workspace_stat_entry_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_stat_entry_by_id_ignore_confinement_point
fn workspace_stat_entry_by_id_ignore_confinement_point(
    mut cx: FunctionContext,
) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let entry_id = {
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret =
                libparsec::workspace_stat_entry_by_id_ignore_confinement_point(workspace, entry_id)
                    .await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = variant_entry_stat_rs_to_js(&mut cx, ok)?;
                        js_obj.set(&mut cx, "value", js_value)?;
                        js_obj
                    }
                    Err(err) => {
                        let js_obj = cx.empty_object();
                        let js_tag = JsBoolean::new(&mut cx, false);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_err = variant_workspace_stat_entry_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_stat_folder_children
fn workspace_stat_folder_children(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_stat_folder_children(workspace, &path).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            // JsArray::new allocates with `undefined` value, that's why we `set` value
                            let js_array = JsArray::new(&mut cx, ok.len());
                            for (i, elem) in ok.into_iter().enumerate() {
                                let js_elem = {
                                    let (x0, x1) = elem;
                                    let js_array = JsArray::new(&mut cx, 2);
                                    let js_value =
                                        JsString::try_new(&mut cx, x0).or_throw(&mut cx)?;
                                    js_array.set(&mut cx, 0, js_value)?;
                                    let js_value = variant_entry_stat_rs_to_js(&mut cx, x1)?;
                                    js_array.set(&mut cx, 1, js_value)?;
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
                        let js_err =
                            variant_workspace_stat_folder_children_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_stat_folder_children_by_id
fn workspace_stat_folder_children_by_id(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let entry_id = {
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
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_stat_folder_children_by_id(workspace, entry_id).await;

            deferred.settle_with(&channel, move |mut cx| {
                let js_ret = match ret {
                    Ok(ok) => {
                        let js_obj = JsObject::new(&mut cx);
                        let js_tag = JsBoolean::new(&mut cx, true);
                        js_obj.set(&mut cx, "ok", js_tag)?;
                        let js_value = {
                            // JsArray::new allocates with `undefined` value, that's why we `set` value
                            let js_array = JsArray::new(&mut cx, ok.len());
                            for (i, elem) in ok.into_iter().enumerate() {
                                let js_elem = {
                                    let (x0, x1) = elem;
                                    let js_array = JsArray::new(&mut cx, 2);
                                    let js_value =
                                        JsString::try_new(&mut cx, x0).or_throw(&mut cx)?;
                                    js_array.set(&mut cx, 0, js_value)?;
                                    let js_value = variant_entry_stat_rs_to_js(&mut cx, x1)?;
                                    js_array.set(&mut cx, 1, js_value)?;
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
                        let js_err =
                            variant_workspace_stat_folder_children_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_stop
fn workspace_stop(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    // TODO: Promises are not cancellable in Javascript by default, should we add a custom cancel method ?
    let _handle = crate::TOKIO_RUNTIME
        .lock()
        .expect("Mutex is poisoned")
        .spawn(async move {
            let ret = libparsec::workspace_stop(workspace).await;

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
                        let js_err = variant_workspace_stop_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

// workspace_watch_entry_oneshot
fn workspace_watch_entry_oneshot(mut cx: FunctionContext) -> JsResult<JsPromise> {
    crate::init_sentry();
    let workspace = {
        let js_val = cx.argument::<JsNumber>(0)?;
        {
            let v = js_val.value(&mut cx);
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                cx.throw_type_error("Not an u32 number")?
            }
            let v = v as u32;
            v
        }
    };
    let path = {
        let js_val = cx.argument::<JsString>(1)?;
        {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
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
            let ret = libparsec::workspace_watch_entry_oneshot(workspace, path).await;

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
                        let js_err =
                            variant_workspace_watch_entry_one_shot_error_rs_to_js(&mut cx, err)?;
                        js_obj.set(&mut cx, "error", js_err)?;
                        js_obj
                    }
                };
                Ok(js_ret)
            });
        });

    Ok(promise)
}

pub fn register_meths(cx: &mut ModuleContext) -> NeonResult<()> {
    cx.export_function("archiveDevice", archive_device)?;
    cx.export_function("bootstrapOrganization", bootstrap_organization)?;
    cx.export_function(
        "buildParsecOrganizationBootstrapAddr",
        build_parsec_organization_bootstrap_addr,
    )?;
    cx.export_function("cancel", cancel)?;
    cx.export_function(
        "claimerDeviceFinalizeSaveLocalDevice",
        claimer_device_finalize_save_local_device,
    )?;
    cx.export_function(
        "claimerDeviceInProgress1DoDenyTrust",
        claimer_device_in_progress_1_do_deny_trust,
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
        "claimerShamirRecoveryAddShare",
        claimer_shamir_recovery_add_share,
    )?;
    cx.export_function(
        "claimerShamirRecoveryFinalizeSaveLocalDevice",
        claimer_shamir_recovery_finalize_save_local_device,
    )?;
    cx.export_function(
        "claimerShamirRecoveryInProgress1DoDenyTrust",
        claimer_shamir_recovery_in_progress_1_do_deny_trust,
    )?;
    cx.export_function(
        "claimerShamirRecoveryInProgress1DoSignifyTrust",
        claimer_shamir_recovery_in_progress_1_do_signify_trust,
    )?;
    cx.export_function(
        "claimerShamirRecoveryInProgress2DoWaitPeerTrust",
        claimer_shamir_recovery_in_progress_2_do_wait_peer_trust,
    )?;
    cx.export_function(
        "claimerShamirRecoveryInProgress3DoClaim",
        claimer_shamir_recovery_in_progress_3_do_claim,
    )?;
    cx.export_function(
        "claimerShamirRecoveryInitialDoWaitPeer",
        claimer_shamir_recovery_initial_do_wait_peer,
    )?;
    cx.export_function(
        "claimerShamirRecoveryPickRecipient",
        claimer_shamir_recovery_pick_recipient,
    )?;
    cx.export_function(
        "claimerShamirRecoveryRecoverDevice",
        claimer_shamir_recovery_recover_device,
    )?;
    cx.export_function(
        "claimerUserFinalizeSaveLocalDevice",
        claimer_user_finalize_save_local_device,
    )?;
    cx.export_function(
        "claimerUserInProgress1DoDenyTrust",
        claimer_user_in_progress_1_do_deny_trust,
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
    cx.export_function("claimerUserListInitialInfo", claimer_user_list_initial_info)?;
    cx.export_function("claimerUserWaitAllPeers", claimer_user_wait_all_peers)?;
    cx.export_function("clientAcceptTos", client_accept_tos)?;
    cx.export_function("clientCancelInvitation", client_cancel_invitation)?;
    cx.export_function("clientChangeAuthentication", client_change_authentication)?;
    cx.export_function("clientCreateWorkspace", client_create_workspace)?;
    cx.export_function("clientDeleteShamirRecovery", client_delete_shamir_recovery)?;
    cx.export_function("clientExportRecoveryDevice", client_export_recovery_device)?;
    cx.export_function(
        "clientGetSelfShamirRecovery",
        client_get_self_shamir_recovery,
    )?;
    cx.export_function("clientGetTos", client_get_tos)?;
    cx.export_function("clientGetUserDevice", client_get_user_device)?;
    cx.export_function("clientInfo", client_info)?;
    cx.export_function("clientListFrozenUsers", client_list_frozen_users)?;
    cx.export_function("clientListInvitations", client_list_invitations)?;
    cx.export_function(
        "clientListShamirRecoveriesForOthers",
        client_list_shamir_recoveries_for_others,
    )?;
    cx.export_function("clientListUserDevices", client_list_user_devices)?;
    cx.export_function("clientListUsers", client_list_users)?;
    cx.export_function("clientListWorkspaceUsers", client_list_workspace_users)?;
    cx.export_function("clientListWorkspaces", client_list_workspaces)?;
    cx.export_function("clientNewDeviceInvitation", client_new_device_invitation)?;
    cx.export_function(
        "clientNewShamirRecoveryInvitation",
        client_new_shamir_recovery_invitation,
    )?;
    cx.export_function("clientNewUserInvitation", client_new_user_invitation)?;
    cx.export_function("clientRenameWorkspace", client_rename_workspace)?;
    cx.export_function("clientRevokeUser", client_revoke_user)?;
    cx.export_function("clientSetupShamirRecovery", client_setup_shamir_recovery)?;
    cx.export_function("clientShareWorkspace", client_share_workspace)?;
    cx.export_function("clientStart", client_start)?;
    cx.export_function(
        "clientStartDeviceInvitationGreet",
        client_start_device_invitation_greet,
    )?;
    cx.export_function(
        "clientStartShamirRecoveryInvitationGreet",
        client_start_shamir_recovery_invitation_greet,
    )?;
    cx.export_function(
        "clientStartUserInvitationGreet",
        client_start_user_invitation_greet,
    )?;
    cx.export_function("clientStartWorkspace", client_start_workspace)?;
    cx.export_function(
        "clientStartWorkspaceHistory2",
        client_start_workspace_history2,
    )?;
    cx.export_function("clientStop", client_stop)?;
    cx.export_function("clientUpdateUserProfile", client_update_user_profile)?;
    cx.export_function("getDefaultConfigDir", get_default_config_dir)?;
    cx.export_function("getDefaultDataBaseDir", get_default_data_base_dir)?;
    cx.export_function(
        "getDefaultMountpointBaseDir",
        get_default_mountpoint_base_dir,
    )?;
    cx.export_function("getPlatform", get_platform)?;
    cx.export_function(
        "greeterDeviceInProgress1DoWaitPeerTrust",
        greeter_device_in_progress_1_do_wait_peer_trust,
    )?;
    cx.export_function(
        "greeterDeviceInProgress2DoDenyTrust",
        greeter_device_in_progress_2_do_deny_trust,
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
        "greeterShamirRecoveryInProgress1DoWaitPeerTrust",
        greeter_shamir_recovery_in_progress_1_do_wait_peer_trust,
    )?;
    cx.export_function(
        "greeterShamirRecoveryInProgress2DoDenyTrust",
        greeter_shamir_recovery_in_progress_2_do_deny_trust,
    )?;
    cx.export_function(
        "greeterShamirRecoveryInProgress2DoSignifyTrust",
        greeter_shamir_recovery_in_progress_2_do_signify_trust,
    )?;
    cx.export_function(
        "greeterShamirRecoveryInProgress3DoGetClaimRequests",
        greeter_shamir_recovery_in_progress_3_do_get_claim_requests,
    )?;
    cx.export_function(
        "greeterShamirRecoveryInitialDoWaitPeer",
        greeter_shamir_recovery_initial_do_wait_peer,
    )?;
    cx.export_function(
        "greeterUserInProgress1DoWaitPeerTrust",
        greeter_user_in_progress_1_do_wait_peer_trust,
    )?;
    cx.export_function(
        "greeterUserInProgress2DoDenyTrust",
        greeter_user_in_progress_2_do_deny_trust,
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
    cx.export_function("importRecoveryDevice", import_recovery_device)?;
    cx.export_function("initLibparsec", init_libparsec)?;
    cx.export_function("isKeyringAvailable", is_keyring_available)?;
    cx.export_function("listAvailableDevices", list_available_devices)?;
    cx.export_function("mountpointToOsPath", mountpoint_to_os_path)?;
    cx.export_function("mountpointUnmount", mountpoint_unmount)?;
    cx.export_function("newCanceller", new_canceller)?;
    cx.export_function("parseParsecAddr", parse_parsec_addr)?;
    cx.export_function("pathFilename", path_filename)?;
    cx.export_function("pathJoin", path_join)?;
    cx.export_function("pathNormalize", path_normalize)?;
    cx.export_function("pathParent", path_parent)?;
    cx.export_function("pathSplit", path_split)?;
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
    cx.export_function("validateOrganizationId", validate_organization_id)?;
    cx.export_function("validatePath", validate_path)?;
    cx.export_function("waitForDeviceAvailable", wait_for_device_available)?;
    cx.export_function("workspaceCreateFile", workspace_create_file)?;
    cx.export_function("workspaceCreateFolder", workspace_create_folder)?;
    cx.export_function("workspaceCreateFolderAll", workspace_create_folder_all)?;
    cx.export_function("workspaceDecryptPathAddr", workspace_decrypt_path_addr)?;
    cx.export_function("workspaceFdClose", workspace_fd_close)?;
    cx.export_function("workspaceFdFlush", workspace_fd_flush)?;
    cx.export_function("workspaceFdRead", workspace_fd_read)?;
    cx.export_function("workspaceFdResize", workspace_fd_resize)?;
    cx.export_function("workspaceFdStat", workspace_fd_stat)?;
    cx.export_function("workspaceFdWrite", workspace_fd_write)?;
    cx.export_function(
        "workspaceFdWriteConstrainedIo",
        workspace_fd_write_constrained_io,
    )?;
    cx.export_function("workspaceFdWriteStartEof", workspace_fd_write_start_eof)?;
    cx.export_function("workspaceGeneratePathAddr", workspace_generate_path_addr)?;
    cx.export_function("workspaceHistory2FdClose", workspace_history2_fd_close)?;
    cx.export_function("workspaceHistory2FdRead", workspace_history2_fd_read)?;
    cx.export_function("workspaceHistory2FdStat", workspace_history2_fd_stat)?;
    cx.export_function(
        "workspaceHistory2GetTimestampHigherBound",
        workspace_history2_get_timestamp_higher_bound,
    )?;
    cx.export_function(
        "workspaceHistory2GetTimestampLowerBound",
        workspace_history2_get_timestamp_lower_bound,
    )?;
    cx.export_function(
        "workspaceHistory2GetTimestampOfInterest",
        workspace_history2_get_timestamp_of_interest,
    )?;
    cx.export_function("workspaceHistory2OpenFile", workspace_history2_open_file)?;
    cx.export_function(
        "workspaceHistory2OpenFileAndGetId",
        workspace_history2_open_file_and_get_id,
    )?;
    cx.export_function(
        "workspaceHistory2OpenFileById",
        workspace_history2_open_file_by_id,
    )?;
    cx.export_function(
        "workspaceHistory2SetTimestampOfInterest",
        workspace_history2_set_timestamp_of_interest,
    )?;
    cx.export_function("workspaceHistory2StatEntry", workspace_history2_stat_entry)?;
    cx.export_function(
        "workspaceHistory2StatEntryById",
        workspace_history2_stat_entry_by_id,
    )?;
    cx.export_function(
        "workspaceHistory2StatFolderChildren",
        workspace_history2_stat_folder_children,
    )?;
    cx.export_function(
        "workspaceHistory2StatFolderChildrenById",
        workspace_history2_stat_folder_children_by_id,
    )?;
    cx.export_function("workspaceHistory2Stop", workspace_history2_stop)?;
    cx.export_function("workspaceHistoryFdClose", workspace_history_fd_close)?;
    cx.export_function("workspaceHistoryFdRead", workspace_history_fd_read)?;
    cx.export_function("workspaceHistoryFdStat", workspace_history_fd_stat)?;
    cx.export_function(
        "workspaceHistoryGetWorkspaceManifestV1Timestamp",
        workspace_history_get_workspace_manifest_v1_timestamp,
    )?;
    cx.export_function("workspaceHistoryOpenFile", workspace_history_open_file)?;
    cx.export_function(
        "workspaceHistoryOpenFileById",
        workspace_history_open_file_by_id,
    )?;
    cx.export_function("workspaceHistoryStatEntry", workspace_history_stat_entry)?;
    cx.export_function(
        "workspaceHistoryStatEntryById",
        workspace_history_stat_entry_by_id,
    )?;
    cx.export_function(
        "workspaceHistoryStatFolderChildren",
        workspace_history_stat_folder_children,
    )?;
    cx.export_function(
        "workspaceHistoryStatFolderChildrenById",
        workspace_history_stat_folder_children_by_id,
    )?;
    cx.export_function("workspaceInfo", workspace_info)?;
    cx.export_function("workspaceMount", workspace_mount)?;
    cx.export_function("workspaceMoveEntry", workspace_move_entry)?;
    cx.export_function("workspaceOpenFile", workspace_open_file)?;
    cx.export_function("workspaceOpenFileAndGetId", workspace_open_file_and_get_id)?;
    cx.export_function("workspaceOpenFileById", workspace_open_file_by_id)?;
    cx.export_function("workspaceRemoveEntry", workspace_remove_entry)?;
    cx.export_function("workspaceRemoveFile", workspace_remove_file)?;
    cx.export_function("workspaceRemoveFolder", workspace_remove_folder)?;
    cx.export_function("workspaceRemoveFolderAll", workspace_remove_folder_all)?;
    cx.export_function("workspaceRenameEntryById", workspace_rename_entry_by_id)?;
    cx.export_function("workspaceStatEntry", workspace_stat_entry)?;
    cx.export_function("workspaceStatEntryById", workspace_stat_entry_by_id)?;
    cx.export_function(
        "workspaceStatEntryByIdIgnoreConfinementPoint",
        workspace_stat_entry_by_id_ignore_confinement_point,
    )?;
    cx.export_function(
        "workspaceStatFolderChildren",
        workspace_stat_folder_children,
    )?;
    cx.export_function(
        "workspaceStatFolderChildrenById",
        workspace_stat_folder_children_by_id,
    )?;
    cx.export_function("workspaceStop", workspace_stop)?;
    cx.export_function("workspaceWatchEntryOneshot", workspace_watch_entry_oneshot)?;
    Ok(())
}

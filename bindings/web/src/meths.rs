// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

#[allow(unused_imports)]
use js_sys::*;
use std::str::FromStr;
#[allow(unused_imports)]
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
#[allow(unused_imports)]
use wasm_bindgen_futures::*;

// AccountOrganizationsAccountVaultStrategy

#[allow(dead_code)]
fn enum_account_organizations_account_vault_strategy_js_to_rs(
    raw_value: &str,
) -> Result<libparsec::AccountOrganizationsAccountVaultStrategy, JsValue> {
    match raw_value {
        "AccountOrganizationsAccountVaultStrategyAllowed" => {
            Ok(libparsec::AccountOrganizationsAccountVaultStrategy::Allowed)
        }
        "AccountOrganizationsAccountVaultStrategyForbidden" => {
            Ok(libparsec::AccountOrganizationsAccountVaultStrategy::Forbidden)
        }
        _ => {
            let range_error =
                RangeError::new("Invalid value for enum AccountOrganizationsAccountVaultStrategy");
            range_error.set_cause(&JsValue::from(raw_value));
            Err(JsValue::from(range_error))
        }
    }
}

#[allow(dead_code)]
fn enum_account_organizations_account_vault_strategy_rs_to_js(
    value: libparsec::AccountOrganizationsAccountVaultStrategy,
) -> &'static str {
    match value {
        libparsec::AccountOrganizationsAccountVaultStrategy::Allowed => {
            "AccountOrganizationsAccountVaultStrategyAllowed"
        }
        libparsec::AccountOrganizationsAccountVaultStrategy::Forbidden => {
            "AccountOrganizationsAccountVaultStrategyForbidden"
        }
    }
}

// AccountOrganizationsAllowedClientAgent

#[allow(dead_code)]
fn enum_account_organizations_allowed_client_agent_js_to_rs(
    raw_value: &str,
) -> Result<libparsec::AccountOrganizationsAllowedClientAgent, JsValue> {
    match raw_value {
        "AccountOrganizationsAllowedClientAgentNativeOnly" => {
            Ok(libparsec::AccountOrganizationsAllowedClientAgent::NativeOnly)
        }
        "AccountOrganizationsAllowedClientAgentNativeOrWeb" => {
            Ok(libparsec::AccountOrganizationsAllowedClientAgent::NativeOrWeb)
        }
        _ => {
            let range_error =
                RangeError::new("Invalid value for enum AccountOrganizationsAllowedClientAgent");
            range_error.set_cause(&JsValue::from(raw_value));
            Err(JsValue::from(range_error))
        }
    }
}

#[allow(dead_code)]
fn enum_account_organizations_allowed_client_agent_rs_to_js(
    value: libparsec::AccountOrganizationsAllowedClientAgent,
) -> &'static str {
    match value {
        libparsec::AccountOrganizationsAllowedClientAgent::NativeOnly => {
            "AccountOrganizationsAllowedClientAgentNativeOnly"
        }
        libparsec::AccountOrganizationsAllowedClientAgent::NativeOrWeb => {
            "AccountOrganizationsAllowedClientAgentNativeOrWeb"
        }
    }
}

// CancelledGreetingAttemptReason

#[allow(dead_code)]
fn enum_cancelled_greeting_attempt_reason_js_to_rs(
    raw_value: &str,
) -> Result<libparsec::CancelledGreetingAttemptReason, JsValue> {
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
        _ => {
            let range_error =
                RangeError::new("Invalid value for enum CancelledGreetingAttemptReason");
            range_error.set_cause(&JsValue::from(raw_value));
            Err(JsValue::from(range_error))
        }
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

// DevicePurpose

#[allow(dead_code)]
fn enum_device_purpose_js_to_rs(raw_value: &str) -> Result<libparsec::DevicePurpose, JsValue> {
    match raw_value {
        "DevicePurposePassphraseRecovery" => Ok(libparsec::DevicePurpose::PassphraseRecovery),
        "DevicePurposeRegistration" => Ok(libparsec::DevicePurpose::Registration),
        "DevicePurposeShamirRecovery" => Ok(libparsec::DevicePurpose::ShamirRecovery),
        "DevicePurposeStandard" => Ok(libparsec::DevicePurpose::Standard),
        _ => {
            let range_error = RangeError::new("Invalid value for enum DevicePurpose");
            range_error.set_cause(&JsValue::from(raw_value));
            Err(JsValue::from(range_error))
        }
    }
}

#[allow(dead_code)]
fn enum_device_purpose_rs_to_js(value: libparsec::DevicePurpose) -> &'static str {
    match value {
        libparsec::DevicePurpose::PassphraseRecovery => "DevicePurposePassphraseRecovery",
        libparsec::DevicePurpose::Registration => "DevicePurposeRegistration",
        libparsec::DevicePurpose::ShamirRecovery => "DevicePurposeShamirRecovery",
        libparsec::DevicePurpose::Standard => "DevicePurposeStandard",
    }
}

// GreeterOrClaimer

#[allow(dead_code)]
fn enum_greeter_or_claimer_js_to_rs(
    raw_value: &str,
) -> Result<libparsec::GreeterOrClaimer, JsValue> {
    match raw_value {
        "GreeterOrClaimerClaimer" => Ok(libparsec::GreeterOrClaimer::Claimer),
        "GreeterOrClaimerGreeter" => Ok(libparsec::GreeterOrClaimer::Greeter),
        _ => {
            let range_error = RangeError::new("Invalid value for enum GreeterOrClaimer");
            range_error.set_cause(&JsValue::from(raw_value));
            Err(JsValue::from(range_error))
        }
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
fn enum_invitation_email_sent_status_js_to_rs(
    raw_value: &str,
) -> Result<libparsec::InvitationEmailSentStatus, JsValue> {
    match raw_value {
        "InvitationEmailSentStatusRecipientRefused" => {
            Ok(libparsec::InvitationEmailSentStatus::RecipientRefused)
        }
        "InvitationEmailSentStatusServerUnavailable" => {
            Ok(libparsec::InvitationEmailSentStatus::ServerUnavailable)
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
fn enum_invitation_status_js_to_rs(
    raw_value: &str,
) -> Result<libparsec::InvitationStatus, JsValue> {
    match raw_value {
        "InvitationStatusCancelled" => Ok(libparsec::InvitationStatus::Cancelled),
        "InvitationStatusFinished" => Ok(libparsec::InvitationStatus::Finished),
        "InvitationStatusPending" => Ok(libparsec::InvitationStatus::Pending),
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
        libparsec::InvitationStatus::Cancelled => "InvitationStatusCancelled",
        libparsec::InvitationStatus::Finished => "InvitationStatusFinished",
        libparsec::InvitationStatus::Pending => "InvitationStatusPending",
    }
}

// InvitationType

#[allow(dead_code)]
fn enum_invitation_type_js_to_rs(raw_value: &str) -> Result<libparsec::InvitationType, JsValue> {
    match raw_value {
        "InvitationTypeDevice" => Ok(libparsec::InvitationType::Device),
        "InvitationTypeShamirRecovery" => Ok(libparsec::InvitationType::ShamirRecovery),
        "InvitationTypeUser" => Ok(libparsec::InvitationType::User),
        _ => {
            let range_error = RangeError::new("Invalid value for enum InvitationType");
            range_error.set_cause(&JsValue::from(raw_value));
            Err(JsValue::from(range_error))
        }
    }
}

#[allow(dead_code)]
fn enum_invitation_type_rs_to_js(value: libparsec::InvitationType) -> &'static str {
    match value {
        libparsec::InvitationType::Device => "InvitationTypeDevice",
        libparsec::InvitationType::ShamirRecovery => "InvitationTypeShamirRecovery",
        libparsec::InvitationType::User => "InvitationTypeUser",
    }
}

// LogLevel

#[allow(dead_code)]
fn enum_log_level_js_to_rs(raw_value: &str) -> Result<libparsec::LogLevel, JsValue> {
    match raw_value {
        "LogLevelDebug" => Ok(libparsec::LogLevel::Debug),
        "LogLevelError" => Ok(libparsec::LogLevel::Error),
        "LogLevelInfo" => Ok(libparsec::LogLevel::Info),
        "LogLevelTrace" => Ok(libparsec::LogLevel::Trace),
        "LogLevelWarn" => Ok(libparsec::LogLevel::Warn),
        _ => {
            let range_error = RangeError::new("Invalid value for enum LogLevel");
            range_error.set_cause(&JsValue::from(raw_value));
            Err(JsValue::from(range_error))
        }
    }
}

#[allow(dead_code)]
fn enum_log_level_rs_to_js(value: libparsec::LogLevel) -> &'static str {
    match value {
        libparsec::LogLevel::Debug => "LogLevelDebug",
        libparsec::LogLevel::Error => "LogLevelError",
        libparsec::LogLevel::Info => "LogLevelInfo",
        libparsec::LogLevel::Trace => "LogLevelTrace",
        libparsec::LogLevel::Warn => "LogLevelWarn",
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

// UserOnlineStatus

#[allow(dead_code)]
fn enum_user_online_status_js_to_rs(
    raw_value: &str,
) -> Result<libparsec::UserOnlineStatus, JsValue> {
    match raw_value {
        "UserOnlineStatusOffline" => Ok(libparsec::UserOnlineStatus::Offline),
        "UserOnlineStatusOnline" => Ok(libparsec::UserOnlineStatus::Online),
        "UserOnlineStatusUnknown" => Ok(libparsec::UserOnlineStatus::Unknown),
        _ => {
            let range_error = RangeError::new("Invalid value for enum UserOnlineStatus");
            range_error.set_cause(&JsValue::from(raw_value));
            Err(JsValue::from(range_error))
        }
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

// AccountInfo

#[allow(dead_code)]
fn struct_account_info_js_to_rs(obj: JsValue) -> Result<libparsec::AccountInfo, JsValue> {
    let server_addr = {
        let js_val = Reflect::get(&obj, &"serverAddr".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let in_use_auth_method = {
        let js_val = Reflect::get(&obj, &"inUseAuthMethod".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string =
                    |s: String| -> Result<libparsec::AccountAuthMethodID, _> {
                        libparsec::AccountAuthMethodID::from_hex(s.as_str())
                            .map_err(|e| e.to_string())
                    };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let human_handle = {
        let js_val = Reflect::get(&obj, &"humanHandle".into())?;
        struct_human_handle_js_to_rs(js_val)?
    };
    Ok(libparsec::AccountInfo {
        server_addr,
        in_use_auth_method,
        human_handle,
    })
}

#[allow(dead_code)]
fn struct_account_info_rs_to_js(rs_obj: libparsec::AccountInfo) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_server_addr = JsValue::from_str({
        let custom_to_rs_string = |addr: libparsec::ParsecAddr| -> Result<String, &'static str> {
            Ok(addr.to_url().into())
        };
        match custom_to_rs_string(rs_obj.server_addr) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"serverAddr".into(), &js_server_addr)?;
    let js_in_use_auth_method = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::AccountAuthMethodID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.in_use_auth_method) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"inUseAuthMethod".into(), &js_in_use_auth_method)?;
    let js_human_handle = struct_human_handle_rs_to_js(rs_obj.human_handle)?;
    Reflect::set(&js_obj, &"humanHandle".into(), &js_human_handle)?;
    Ok(js_obj)
}

// AccountOrganizations

#[allow(dead_code)]
fn struct_account_organizations_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::AccountOrganizations, JsValue> {
    let active = {
        let js_val = Reflect::get(&obj, &"active".into())?;
        {
            let js_val = js_val
                .dyn_into::<Array>()
                .map_err(|_| TypeError::new("Not an array"))?;
            let mut converted = Vec::with_capacity(js_val.length() as usize);
            for x in js_val.iter() {
                let x_converted = struct_account_organizations_active_user_js_to_rs(x)?;
                converted.push(x_converted);
            }
            converted
        }
    };
    let revoked = {
        let js_val = Reflect::get(&obj, &"revoked".into())?;
        {
            let js_val = js_val
                .dyn_into::<Array>()
                .map_err(|_| TypeError::new("Not an array"))?;
            let mut converted = Vec::with_capacity(js_val.length() as usize);
            for x in js_val.iter() {
                let x_converted = struct_account_organizations_revoked_user_js_to_rs(x)?;
                converted.push(x_converted);
            }
            converted
        }
    };
    Ok(libparsec::AccountOrganizations { active, revoked })
}

#[allow(dead_code)]
fn struct_account_organizations_rs_to_js(
    rs_obj: libparsec::AccountOrganizations,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_active = {
        // Array::new_with_length allocates with `undefined` value, that's why we `set` value
        let js_array = Array::new_with_length(rs_obj.active.len() as u32);
        for (i, elem) in rs_obj.active.into_iter().enumerate() {
            let js_elem = struct_account_organizations_active_user_rs_to_js(elem)?;
            js_array.set(i as u32, js_elem);
        }
        js_array.into()
    };
    Reflect::set(&js_obj, &"active".into(), &js_active)?;
    let js_revoked = {
        // Array::new_with_length allocates with `undefined` value, that's why we `set` value
        let js_array = Array::new_with_length(rs_obj.revoked.len() as u32);
        for (i, elem) in rs_obj.revoked.into_iter().enumerate() {
            let js_elem = struct_account_organizations_revoked_user_rs_to_js(elem)?;
            js_array.set(i as u32, js_elem);
        }
        js_array.into()
    };
    Reflect::set(&js_obj, &"revoked".into(), &js_revoked)?;
    Ok(js_obj)
}

// AccountOrganizationsActiveUser

#[allow(dead_code)]
fn struct_account_organizations_active_user_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::AccountOrganizationsActiveUser, JsValue> {
    let organization_id = {
        let js_val = Reflect::get(&obj, &"organizationId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let user_id = {
        let js_val = Reflect::get(&obj, &"userId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                    libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let created_on = {
        let js_val = Reflect::get(&obj, &"createdOn".into())?;
        {
            let v = js_val.dyn_into::<Number>()?.value_of();
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
            v
        }
    };
    let is_frozen = {
        let js_val = Reflect::get(&obj, &"isFrozen".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
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
    let organization_config = {
        let js_val = Reflect::get(&obj, &"organizationConfig".into())?;
        struct_account_organizations_organization_config_js_to_rs(js_val)?
    };
    Ok(libparsec::AccountOrganizationsActiveUser {
        organization_id,
        user_id,
        created_on,
        is_frozen,
        current_profile,
        organization_config,
    })
}

#[allow(dead_code)]
fn struct_account_organizations_active_user_rs_to_js(
    rs_obj: libparsec::AccountOrganizationsActiveUser,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_organization_id = JsValue::from_str(rs_obj.organization_id.as_ref());
    Reflect::set(&js_obj, &"organizationId".into(), &js_organization_id)?;
    let js_user_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.user_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"userId".into(), &js_user_id)?;
    let js_created_on = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        let v = match custom_to_rs_f64(rs_obj.created_on) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        };
        JsValue::from(v)
    };
    Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
    let js_is_frozen = rs_obj.is_frozen.into();
    Reflect::set(&js_obj, &"isFrozen".into(), &js_is_frozen)?;
    let js_current_profile = JsValue::from_str(enum_user_profile_rs_to_js(rs_obj.current_profile));
    Reflect::set(&js_obj, &"currentProfile".into(), &js_current_profile)?;
    let js_organization_config =
        struct_account_organizations_organization_config_rs_to_js(rs_obj.organization_config)?;
    Reflect::set(
        &js_obj,
        &"organizationConfig".into(),
        &js_organization_config,
    )?;
    Ok(js_obj)
}

// AccountOrganizationsOrganizationConfig

#[allow(dead_code)]
fn struct_account_organizations_organization_config_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::AccountOrganizationsOrganizationConfig, JsValue> {
    let is_expired = {
        let js_val = Reflect::get(&obj, &"isExpired".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
    };
    let user_profile_outsider_allowed = {
        let js_val = Reflect::get(&obj, &"userProfileOutsiderAllowed".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
    };
    let active_users_limit = {
        let js_val = Reflect::get(&obj, &"activeUsersLimit".into())?;
        variant_active_users_limit_js_to_rs(js_val)?
    };
    let allowed_client_agent = {
        let js_val = Reflect::get(&obj, &"allowedClientAgent".into())?;
        {
            let raw_string = js_val.as_string().ok_or_else(|| {
                let type_error = TypeError::new("value is not a string");
                type_error.set_cause(&js_val);
                JsValue::from(type_error)
            })?;
            enum_account_organizations_allowed_client_agent_js_to_rs(raw_string.as_str())
        }?
    };
    let account_vault_strategy = {
        let js_val = Reflect::get(&obj, &"accountVaultStrategy".into())?;
        {
            let raw_string = js_val.as_string().ok_or_else(|| {
                let type_error = TypeError::new("value is not a string");
                type_error.set_cause(&js_val);
                JsValue::from(type_error)
            })?;
            enum_account_organizations_account_vault_strategy_js_to_rs(raw_string.as_str())
        }?
    };
    Ok(libparsec::AccountOrganizationsOrganizationConfig {
        is_expired,
        user_profile_outsider_allowed,
        active_users_limit,
        allowed_client_agent,
        account_vault_strategy,
    })
}

#[allow(dead_code)]
fn struct_account_organizations_organization_config_rs_to_js(
    rs_obj: libparsec::AccountOrganizationsOrganizationConfig,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_is_expired = rs_obj.is_expired.into();
    Reflect::set(&js_obj, &"isExpired".into(), &js_is_expired)?;
    let js_user_profile_outsider_allowed = rs_obj.user_profile_outsider_allowed.into();
    Reflect::set(
        &js_obj,
        &"userProfileOutsiderAllowed".into(),
        &js_user_profile_outsider_allowed,
    )?;
    let js_active_users_limit = variant_active_users_limit_rs_to_js(rs_obj.active_users_limit)?;
    Reflect::set(&js_obj, &"activeUsersLimit".into(), &js_active_users_limit)?;
    let js_allowed_client_agent = JsValue::from_str(
        enum_account_organizations_allowed_client_agent_rs_to_js(rs_obj.allowed_client_agent),
    );
    Reflect::set(
        &js_obj,
        &"allowedClientAgent".into(),
        &js_allowed_client_agent,
    )?;
    let js_account_vault_strategy = JsValue::from_str(
        enum_account_organizations_account_vault_strategy_rs_to_js(rs_obj.account_vault_strategy),
    );
    Reflect::set(
        &js_obj,
        &"accountVaultStrategy".into(),
        &js_account_vault_strategy,
    )?;
    Ok(js_obj)
}

// AccountOrganizationsRevokedUser

#[allow(dead_code)]
fn struct_account_organizations_revoked_user_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::AccountOrganizationsRevokedUser, JsValue> {
    let organization_id = {
        let js_val = Reflect::get(&obj, &"organizationId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let user_id = {
        let js_val = Reflect::get(&obj, &"userId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                    libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let created_on = {
        let js_val = Reflect::get(&obj, &"createdOn".into())?;
        {
            let v = js_val.dyn_into::<Number>()?.value_of();
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
            v
        }
    };
    let revoked_on = {
        let js_val = Reflect::get(&obj, &"revokedOn".into())?;
        {
            let v = js_val.dyn_into::<Number>()?.value_of();
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
            v
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
    Ok(libparsec::AccountOrganizationsRevokedUser {
        organization_id,
        user_id,
        created_on,
        revoked_on,
        current_profile,
    })
}

#[allow(dead_code)]
fn struct_account_organizations_revoked_user_rs_to_js(
    rs_obj: libparsec::AccountOrganizationsRevokedUser,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_organization_id = JsValue::from_str(rs_obj.organization_id.as_ref());
    Reflect::set(&js_obj, &"organizationId".into(), &js_organization_id)?;
    let js_user_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.user_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"userId".into(), &js_user_id)?;
    let js_created_on = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        let v = match custom_to_rs_f64(rs_obj.created_on) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        };
        JsValue::from(v)
    };
    Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
    let js_revoked_on = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        let v = match custom_to_rs_f64(rs_obj.revoked_on) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        };
        JsValue::from(v)
    };
    Reflect::set(&js_obj, &"revokedOn".into(), &js_revoked_on)?;
    let js_current_profile = JsValue::from_str(enum_user_profile_rs_to_js(rs_obj.current_profile));
    Reflect::set(&js_obj, &"currentProfile".into(), &js_current_profile)?;
    Ok(js_obj)
}

// AuthMethodInfo

#[allow(dead_code)]
fn struct_auth_method_info_js_to_rs(obj: JsValue) -> Result<libparsec::AuthMethodInfo, JsValue> {
    let auth_method_id = {
        let js_val = Reflect::get(&obj, &"authMethodId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string =
                    |s: String| -> Result<libparsec::AccountAuthMethodID, _> {
                        libparsec::AccountAuthMethodID::from_hex(s.as_str())
                            .map_err(|e| e.to_string())
                    };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let created_on = {
        let js_val = Reflect::get(&obj, &"createdOn".into())?;
        {
            let v = js_val.dyn_into::<Number>()?.value_of();
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
            v
        }
    };
    let created_by_ip = {
        let js_val = Reflect::get(&obj, &"createdByIp".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
    };
    let created_by_user_agent = {
        let js_val = Reflect::get(&obj, &"createdByUserAgent".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
    };
    let use_password = {
        let js_val = Reflect::get(&obj, &"usePassword".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
    };
    Ok(libparsec::AuthMethodInfo {
        auth_method_id,
        created_on,
        created_by_ip,
        created_by_user_agent,
        use_password,
    })
}

#[allow(dead_code)]
fn struct_auth_method_info_rs_to_js(rs_obj: libparsec::AuthMethodInfo) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_auth_method_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::AccountAuthMethodID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.auth_method_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"authMethodId".into(), &js_auth_method_id)?;
    let js_created_on = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        let v = match custom_to_rs_f64(rs_obj.created_on) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        };
        JsValue::from(v)
    };
    Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
    let js_created_by_ip = rs_obj.created_by_ip.into();
    Reflect::set(&js_obj, &"createdByIp".into(), &js_created_by_ip)?;
    let js_created_by_user_agent = rs_obj.created_by_user_agent.into();
    Reflect::set(
        &js_obj,
        &"createdByUserAgent".into(),
        &js_created_by_user_agent,
    )?;
    let js_use_password = rs_obj.use_password.into();
    Reflect::set(&js_obj, &"usePassword".into(), &js_use_password)?;
    Ok(js_obj)
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
            })?
    };
    let created_on = {
        let js_val = Reflect::get(&obj, &"createdOn".into())?;
        {
            let v = js_val.dyn_into::<Number>()?.value_of();
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
            v
        }
    };
    let protected_on = {
        let js_val = Reflect::get(&obj, &"protectedOn".into())?;
        {
            let v = js_val.dyn_into::<Number>()?.value_of();
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
            v
        }
    };
    let server_url = {
        let js_val = Reflect::get(&obj, &"serverUrl".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
    };
    let organization_id = {
        let js_val = Reflect::get(&obj, &"organizationId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let user_id = {
        let js_val = Reflect::get(&obj, &"userId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                    libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let device_id = {
        let js_val = Reflect::get(&obj, &"deviceId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                    libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let human_handle = {
        let js_val = Reflect::get(&obj, &"humanHandle".into())?;
        struct_human_handle_js_to_rs(js_val)?
    };
    let device_label = {
        let js_val = Reflect::get(&obj, &"deviceLabel".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let ty = {
        let js_val = Reflect::get(&obj, &"ty".into())?;
        variant_available_device_type_js_to_rs(js_val)?
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
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"keyFilePath".into(), &js_key_file_path)?;
    let js_created_on = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        let v = match custom_to_rs_f64(rs_obj.created_on) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        };
        JsValue::from(v)
    };
    Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
    let js_protected_on = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        let v = match custom_to_rs_f64(rs_obj.protected_on) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        };
        JsValue::from(v)
    };
    Reflect::set(&js_obj, &"protectedOn".into(), &js_protected_on)?;
    let js_server_url = rs_obj.server_url.into();
    Reflect::set(&js_obj, &"serverUrl".into(), &js_server_url)?;
    let js_organization_id = JsValue::from_str(rs_obj.organization_id.as_ref());
    Reflect::set(&js_obj, &"organizationId".into(), &js_organization_id)?;
    let js_user_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.user_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"userId".into(), &js_user_id)?;
    let js_device_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.device_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"deviceId".into(), &js_device_id)?;
    let js_human_handle = struct_human_handle_rs_to_js(rs_obj.human_handle)?;
    Reflect::set(&js_obj, &"humanHandle".into(), &js_human_handle)?;
    let js_device_label = JsValue::from_str(rs_obj.device_label.as_ref());
    Reflect::set(&js_obj, &"deviceLabel".into(), &js_device_label)?;
    let js_ty = variant_available_device_type_rs_to_js(rs_obj.ty)?;
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
            })?
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
            })?
    };
    let mountpoint_mount_strategy = {
        let js_val = Reflect::get(&obj, &"mountpointMountStrategy".into())?;
        variant_mountpoint_mount_strategy_js_to_rs(js_val)?
    };
    let workspace_storage_cache_size = {
        let js_val = Reflect::get(&obj, &"workspaceStorageCacheSize".into())?;
        variant_workspace_storage_cache_size_js_to_rs(js_val)?
    };
    let with_monitors = {
        let js_val = Reflect::get(&obj, &"withMonitors".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
    };
    let prevent_sync_pattern = {
        let js_val = Reflect::get(&obj, &"preventSyncPattern".into())?;
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
    let log_level = {
        let js_val = Reflect::get(&obj, &"logLevel".into())?;
        if js_val.is_null() {
            None
        } else {
            Some({
                let raw_string = js_val.as_string().ok_or_else(|| {
                    let type_error = TypeError::new("value is not a string");
                    type_error.set_cause(&js_val);
                    JsValue::from(type_error)
                })?;
                enum_log_level_js_to_rs(raw_string.as_str())
            }?)
        }
    };
    Ok(libparsec::ClientConfig {
        config_dir,
        data_base_dir,
        mountpoint_mount_strategy,
        workspace_storage_cache_size,
        with_monitors,
        prevent_sync_pattern,
        log_level,
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
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
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
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"dataBaseDir".into(), &js_data_base_dir)?;
    let js_mountpoint_mount_strategy =
        variant_mountpoint_mount_strategy_rs_to_js(rs_obj.mountpoint_mount_strategy)?;
    Reflect::set(
        &js_obj,
        &"mountpointMountStrategy".into(),
        &js_mountpoint_mount_strategy,
    )?;
    let js_workspace_storage_cache_size =
        variant_workspace_storage_cache_size_rs_to_js(rs_obj.workspace_storage_cache_size)?;
    Reflect::set(
        &js_obj,
        &"workspaceStorageCacheSize".into(),
        &js_workspace_storage_cache_size,
    )?;
    let js_with_monitors = rs_obj.with_monitors.into();
    Reflect::set(&js_obj, &"withMonitors".into(), &js_with_monitors)?;
    let js_prevent_sync_pattern = match rs_obj.prevent_sync_pattern {
        Some(val) => val.into(),
        None => JsValue::NULL,
    };
    Reflect::set(
        &js_obj,
        &"preventSyncPattern".into(),
        &js_prevent_sync_pattern,
    )?;
    let js_log_level = match rs_obj.log_level {
        Some(val) => JsValue::from_str(enum_log_level_rs_to_js(val)),
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"logLevel".into(), &js_log_level)?;
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
                    libparsec::ParsecOrganizationAddr::from_any(&s).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let organization_id = {
        let js_val = Reflect::get(&obj, &"organizationId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let device_id = {
        let js_val = Reflect::get(&obj, &"deviceId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                    libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let user_id = {
        let js_val = Reflect::get(&obj, &"userId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                    libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let device_label = {
        let js_val = Reflect::get(&obj, &"deviceLabel".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let human_handle = {
        let js_val = Reflect::get(&obj, &"humanHandle".into())?;
        struct_human_handle_js_to_rs(js_val)?
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
    let server_config = {
        let js_val = Reflect::get(&obj, &"serverConfig".into())?;
        struct_server_config_js_to_rs(js_val)?
    };
    let is_server_online = {
        let js_val = Reflect::get(&obj, &"isServerOnline".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
    };
    let is_organization_expired = {
        let js_val = Reflect::get(&obj, &"isOrganizationExpired".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
    };
    let must_accept_tos = {
        let js_val = Reflect::get(&obj, &"mustAcceptTos".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
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
        is_server_online,
        is_organization_expired,
        must_accept_tos,
    })
}

#[allow(dead_code)]
fn struct_client_info_rs_to_js(rs_obj: libparsec::ClientInfo) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_organization_addr = JsValue::from_str({
        let custom_to_rs_string =
            |addr: libparsec::ParsecOrganizationAddr| -> Result<String, &'static str> {
                Ok(addr.to_url().into())
            };
        match custom_to_rs_string(rs_obj.organization_addr) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"organizationAddr".into(), &js_organization_addr)?;
    let js_organization_id = JsValue::from_str(rs_obj.organization_id.as_ref());
    Reflect::set(&js_obj, &"organizationId".into(), &js_organization_id)?;
    let js_device_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.device_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"deviceId".into(), &js_device_id)?;
    let js_user_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.user_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"userId".into(), &js_user_id)?;
    let js_device_label = JsValue::from_str(rs_obj.device_label.as_ref());
    Reflect::set(&js_obj, &"deviceLabel".into(), &js_device_label)?;
    let js_human_handle = struct_human_handle_rs_to_js(rs_obj.human_handle)?;
    Reflect::set(&js_obj, &"humanHandle".into(), &js_human_handle)?;
    let js_current_profile = JsValue::from_str(enum_user_profile_rs_to_js(rs_obj.current_profile));
    Reflect::set(&js_obj, &"currentProfile".into(), &js_current_profile)?;
    let js_server_config = struct_server_config_rs_to_js(rs_obj.server_config)?;
    Reflect::set(&js_obj, &"serverConfig".into(), &js_server_config)?;
    let js_is_server_online = rs_obj.is_server_online.into();
    Reflect::set(&js_obj, &"isServerOnline".into(), &js_is_server_online)?;
    let js_is_organization_expired = rs_obj.is_organization_expired.into();
    Reflect::set(
        &js_obj,
        &"isOrganizationExpired".into(),
        &js_is_organization_expired,
    )?;
    let js_must_accept_tos = rs_obj.must_accept_tos.into();
    Reflect::set(&js_obj, &"mustAcceptTos".into(), &js_must_accept_tos)?;
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
            let v = v as u32;
            v
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
            let v = v as u32;
            v
        }
    };
    let greeter_user_id = {
        let js_val = Reflect::get(&obj, &"greeterUserId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                    libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let greeter_human_handle = {
        let js_val = Reflect::get(&obj, &"greeterHumanHandle".into())?;
        struct_human_handle_js_to_rs(js_val)?
    };
    let greeter_sas = {
        let js_val = Reflect::get(&obj, &"greeterSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
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
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?;
                converted.push(x_converted);
            }
            converted
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
fn struct_device_claim_in_progress1_info_rs_to_js(
    rs_obj: libparsec::DeviceClaimInProgress1Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_greeter_user_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.greeter_user_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"greeterUserId".into(), &js_greeter_user_id)?;
    let js_greeter_human_handle = struct_human_handle_rs_to_js(rs_obj.greeter_human_handle)?;
    Reflect::set(
        &js_obj,
        &"greeterHumanHandle".into(),
        &js_greeter_human_handle,
    )?;
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
            let v = v as u32;
            v
        }
    };
    let claimer_sas = {
        let js_val = Reflect::get(&obj, &"claimerSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
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
            let v = v as u32;
            v
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
            let v = v as u32;
            v
        }
    };
    let greeter_sas = {
        let js_val = Reflect::get(&obj, &"greeterSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
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
            let v = v as u32;
            v
        }
    };
    let claimer_sas = {
        let js_val = Reflect::get(&obj, &"claimerSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
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
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?;
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
            let v = v as u32;
            v
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
            let v = v as u32;
            v
        }
    };
    let requested_device_label = {
        let js_val = Reflect::get(&obj, &"requestedDeviceLabel".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
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
    let js_requested_device_label = JsValue::from_str(rs_obj.requested_device_label.as_ref());
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
            let v = v as u32;
            v
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
                let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                    libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let purpose = {
        let js_val = Reflect::get(&obj, &"purpose".into())?;
        {
            let raw_string = js_val.as_string().ok_or_else(|| {
                let type_error = TypeError::new("value is not a string");
                type_error.set_cause(&js_val);
                JsValue::from(type_error)
            })?;
            enum_device_purpose_js_to_rs(raw_string.as_str())
        }?
    };
    let device_label = {
        let js_val = Reflect::get(&obj, &"deviceLabel".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let created_on = {
        let js_val = Reflect::get(&obj, &"createdOn".into())?;
        {
            let v = js_val.dyn_into::<Number>()?.value_of();
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
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
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?,
            )
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
fn struct_device_info_rs_to_js(rs_obj: libparsec::DeviceInfo) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"id".into(), &js_id)?;
    let js_purpose = JsValue::from_str(enum_device_purpose_rs_to_js(rs_obj.purpose));
    Reflect::set(&js_obj, &"purpose".into(), &js_purpose)?;
    let js_device_label = JsValue::from_str(rs_obj.device_label.as_ref());
    Reflect::set(&js_obj, &"deviceLabel".into(), &js_device_label)?;
    let js_created_on = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
            let custom_to_rs_string =
                |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
            match custom_to_rs_string(val) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
            }
            .as_ref()
        }),
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
    Ok(js_obj)
}

// FileStat

#[allow(dead_code)]
fn struct_file_stat_js_to_rs(obj: JsValue) -> Result<libparsec::FileStat, JsValue> {
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
            })?
    };
    let created = {
        let js_val = Reflect::get(&obj, &"created".into())?;
        {
            let v = js_val.dyn_into::<Number>()?.value_of();
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
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
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
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
            let v = v as u32;
            v
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
            let v = u64::try_from(js_val)
                .map_err(|_| TypeError::new("Not a BigInt representing an u64 number"))?;
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
fn struct_file_stat_rs_to_js(rs_obj: libparsec::FileStat) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"id".into(), &js_id)?;
    let js_created = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        let v = match custom_to_rs_f64(rs_obj.created) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        };
        JsValue::from(v)
    };
    Reflect::set(&js_obj, &"created".into(), &js_created)?;
    let js_updated = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        let v = match custom_to_rs_f64(rs_obj.updated) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        };
        JsValue::from(v)
    };
    Reflect::set(&js_obj, &"updated".into(), &js_updated)?;
    let js_base_version = JsValue::from(rs_obj.base_version);
    Reflect::set(&js_obj, &"baseVersion".into(), &js_base_version)?;
    let js_is_placeholder = rs_obj.is_placeholder.into();
    Reflect::set(&js_obj, &"isPlaceholder".into(), &js_is_placeholder)?;
    let js_need_sync = rs_obj.need_sync.into();
    Reflect::set(&js_obj, &"needSync".into(), &js_need_sync)?;
    let js_size = JsValue::from(rs_obj.size);
    Reflect::set(&js_obj, &"size".into(), &js_size)?;
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
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    libparsec::EmailAddress::from_str(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let label = {
        let js_val = Reflect::get(&obj, &"label".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
    };
    {
        let custom_init = |email: libparsec::EmailAddress, label: String| -> Result<_, String> {
            libparsec::HumanHandle::new(email, &label).map_err(|e| e.to_string())
        };
        custom_init(email, label).map_err(|e| e.into())
    }
}

#[allow(dead_code)]
fn struct_human_handle_rs_to_js(rs_obj: libparsec::HumanHandle) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_email = {
        let custom_getter =
            |obj: &libparsec::HumanHandle| -> libparsec::EmailAddress { obj.email().clone() };
        JsValue::from_str({
            let custom_to_rs_string =
                |x: libparsec::EmailAddress| -> Result<_, &'static str> { Ok(x.to_string()) };
            match custom_to_rs_string(custom_getter(&rs_obj)) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
            }
            .as_ref()
        })
    };
    Reflect::set(&js_obj, &"email".into(), &js_email)?;
    let js_label = {
        let custom_getter = |obj| -> &str {
            fn a(o: &libparsec::HumanHandle) -> &str {
                o.label()
            }
            a(obj)
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
                    libparsec::ParsecInvitationAddr::from_any(&s).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
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
            })?
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
            |addr: libparsec::ParsecInvitationAddr| -> Result<String, &'static str> {
                Ok(addr.to_url().into())
            };
        match custom_to_rs_string(rs_obj.addr) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"addr".into(), &js_addr)?;
    let js_token = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.token) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
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

// OpenOptions

#[allow(dead_code)]
fn struct_open_options_js_to_rs(obj: JsValue) -> Result<libparsec::OpenOptions, JsValue> {
    let read = {
        let js_val = Reflect::get(&obj, &"read".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
    };
    let write = {
        let js_val = Reflect::get(&obj, &"write".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
    };
    let truncate = {
        let js_val = Reflect::get(&obj, &"truncate".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
    };
    let create = {
        let js_val = Reflect::get(&obj, &"create".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
    };
    let create_new = {
        let js_val = Reflect::get(&obj, &"createNew".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
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
fn struct_open_options_rs_to_js(rs_obj: libparsec::OpenOptions) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_read = rs_obj.read.into();
    Reflect::set(&js_obj, &"read".into(), &js_read)?;
    let js_write = rs_obj.write.into();
    Reflect::set(&js_obj, &"write".into(), &js_write)?;
    let js_truncate = rs_obj.truncate.into();
    Reflect::set(&js_obj, &"truncate".into(), &js_truncate)?;
    let js_create = rs_obj.create.into();
    Reflect::set(&js_obj, &"create".into(), &js_create)?;
    let js_create_new = rs_obj.create_new.into();
    Reflect::set(&js_obj, &"createNew".into(), &js_create_new)?;
    Ok(js_obj)
}

// OrganizationInfo

#[allow(dead_code)]
fn struct_organization_info_js_to_rs(obj: JsValue) -> Result<libparsec::OrganizationInfo, JsValue> {
    let total_block_bytes = {
        let js_val = Reflect::get(&obj, &"totalBlockBytes".into())?;
        {
            let v = u64::try_from(js_val)
                .map_err(|_| TypeError::new("Not a BigInt representing an u64 number"))?;
            v
        }
    };
    let total_metadata_bytes = {
        let js_val = Reflect::get(&obj, &"totalMetadataBytes".into())?;
        {
            let v = u64::try_from(js_val)
                .map_err(|_| TypeError::new("Not a BigInt representing an u64 number"))?;
            v
        }
    };
    Ok(libparsec::OrganizationInfo {
        total_block_bytes,
        total_metadata_bytes,
    })
}

#[allow(dead_code)]
fn struct_organization_info_rs_to_js(
    rs_obj: libparsec::OrganizationInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_total_block_bytes = JsValue::from(rs_obj.total_block_bytes);
    Reflect::set(&js_obj, &"totalBlockBytes".into(), &js_total_block_bytes)?;
    let js_total_metadata_bytes = JsValue::from(rs_obj.total_metadata_bytes);
    Reflect::set(
        &js_obj,
        &"totalMetadataBytes".into(),
        &js_total_metadata_bytes,
    )?;
    Ok(js_obj)
}

// PkiEnrollmentListItem

#[allow(dead_code)]
fn struct_pki_enrollment_list_item_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::PkiEnrollmentListItem, JsValue> {
    let enrollment_id = {
        let js_val = Reflect::get(&obj, &"enrollmentId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::EnrollmentID, _> {
                    libparsec::EnrollmentID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let submitted_on = {
        let js_val = Reflect::get(&obj, &"submittedOn".into())?;
        {
            let v = js_val.dyn_into::<Number>()?.value_of();
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
            v
        }
    };
    let der_x509_certificate = {
        let js_val = Reflect::get(&obj, &"derX509Certificate".into())?;
        js_val
            .dyn_into::<Uint8Array>()
            .map(|x| x.to_vec())
            .map_err(|_| TypeError::new("Not a Uint8Array"))
            .and_then(|x| {
                let custom_from_rs_bytes =
                    |v: &[u8]| -> Result<libparsec::Bytes, String> { Ok(v.to_vec().into()) };
                custom_from_rs_bytes(&x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let payload_signature = {
        let js_val = Reflect::get(&obj, &"payloadSignature".into())?;
        js_val
            .dyn_into::<Uint8Array>()
            .map(|x| x.to_vec())
            .map_err(|_| TypeError::new("Not a Uint8Array"))
            .and_then(|x| {
                let custom_from_rs_bytes =
                    |v: &[u8]| -> Result<libparsec::Bytes, String> { Ok(v.to_vec().into()) };
                custom_from_rs_bytes(&x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let payload_signature_algorithm = {
        let js_val = Reflect::get(&obj, &"payloadSignatureAlgorithm".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))?
            .parse()
            .map_err(|_| TypeError::new("Not a valid PkiSignatureAlgorithm"))?
    };
    let payload = {
        let js_val = Reflect::get(&obj, &"payload".into())?;
        js_val
            .dyn_into::<Uint8Array>()
            .map(|x| x.to_vec())
            .map_err(|_| TypeError::new("Not a Uint8Array"))
            .and_then(|x| {
                let custom_from_rs_bytes =
                    |v: &[u8]| -> Result<libparsec::Bytes, String> { Ok(v.to_vec().into()) };
                custom_from_rs_bytes(&x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    Ok(libparsec::PkiEnrollmentListItem {
        enrollment_id,
        submitted_on,
        der_x509_certificate,
        payload_signature,
        payload_signature_algorithm,
        payload,
    })
}

#[allow(dead_code)]
fn struct_pki_enrollment_list_item_rs_to_js(
    rs_obj: libparsec::PkiEnrollmentListItem,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_enrollment_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::EnrollmentID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.enrollment_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"enrollmentId".into(), &js_enrollment_id)?;
    let js_submitted_on = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        let v = match custom_to_rs_f64(rs_obj.submitted_on) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        };
        JsValue::from(v)
    };
    Reflect::set(&js_obj, &"submittedOn".into(), &js_submitted_on)?;
    let js_der_x509_certificate =
        JsValue::from(Uint8Array::from(rs_obj.der_x509_certificate.as_ref()));
    Reflect::set(
        &js_obj,
        &"derX509Certificate".into(),
        &js_der_x509_certificate,
    )?;
    let js_payload_signature = JsValue::from(Uint8Array::from(rs_obj.payload_signature.as_ref()));
    Reflect::set(&js_obj, &"payloadSignature".into(), &js_payload_signature)?;
    let js_payload_signature_algorithm = JsValue::from_str({
        let custom_to_rs_string =
            |v| -> Result<_, std::convert::Infallible> { Ok(std::string::ToString::to_string(&v)) };
        match custom_to_rs_string(rs_obj.payload_signature_algorithm) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(
        &js_obj,
        &"payloadSignatureAlgorithm".into(),
        &js_payload_signature_algorithm,
    )?;
    let js_payload = JsValue::from(Uint8Array::from(rs_obj.payload.as_ref()));
    Reflect::set(&js_obj, &"payload".into(), &js_payload)?;
    Ok(js_obj)
}

// ServerConfig

#[allow(dead_code)]
fn struct_server_config_js_to_rs(obj: JsValue) -> Result<libparsec::ServerConfig, JsValue> {
    let user_profile_outsider_allowed = {
        let js_val = Reflect::get(&obj, &"userProfileOutsiderAllowed".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
    };
    let active_users_limit = {
        let js_val = Reflect::get(&obj, &"activeUsersLimit".into())?;
        variant_active_users_limit_js_to_rs(js_val)?
    };
    Ok(libparsec::ServerConfig {
        user_profile_outsider_allowed,
        active_users_limit,
    })
}

#[allow(dead_code)]
fn struct_server_config_rs_to_js(rs_obj: libparsec::ServerConfig) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_user_profile_outsider_allowed = rs_obj.user_profile_outsider_allowed.into();
    Reflect::set(
        &js_obj,
        &"userProfileOutsiderAllowed".into(),
        &js_user_profile_outsider_allowed,
    )?;
    let js_active_users_limit = variant_active_users_limit_rs_to_js(rs_obj.active_users_limit)?;
    Reflect::set(&js_obj, &"activeUsersLimit".into(), &js_active_users_limit)?;
    Ok(js_obj)
}

// ShamirRecoveryClaimInProgress1Info

#[allow(dead_code)]
fn struct_shamir_recovery_claim_in_progress1_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::ShamirRecoveryClaimInProgress1Info, JsValue> {
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
            let v = v as u32;
            v
        }
    };
    let greeter_user_id = {
        let js_val = Reflect::get(&obj, &"greeterUserId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                    libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let greeter_human_handle = {
        let js_val = Reflect::get(&obj, &"greeterHumanHandle".into())?;
        struct_human_handle_js_to_rs(js_val)?
    };
    let greeter_sas = {
        let js_val = Reflect::get(&obj, &"greeterSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
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
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?;
                converted.push(x_converted);
            }
            converted
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
fn struct_shamir_recovery_claim_in_progress1_info_rs_to_js(
    rs_obj: libparsec::ShamirRecoveryClaimInProgress1Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_greeter_user_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.greeter_user_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"greeterUserId".into(), &js_greeter_user_id)?;
    let js_greeter_human_handle = struct_human_handle_rs_to_js(rs_obj.greeter_human_handle)?;
    Reflect::set(
        &js_obj,
        &"greeterHumanHandle".into(),
        &js_greeter_human_handle,
    )?;
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

// ShamirRecoveryClaimInProgress2Info

#[allow(dead_code)]
fn struct_shamir_recovery_claim_in_progress2_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::ShamirRecoveryClaimInProgress2Info, JsValue> {
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
            let v = v as u32;
            v
        }
    };
    let claimer_sas = {
        let js_val = Reflect::get(&obj, &"claimerSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    Ok(libparsec::ShamirRecoveryClaimInProgress2Info {
        handle,
        claimer_sas,
    })
}

#[allow(dead_code)]
fn struct_shamir_recovery_claim_in_progress2_info_rs_to_js(
    rs_obj: libparsec::ShamirRecoveryClaimInProgress2Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_claimer_sas = JsValue::from_str(rs_obj.claimer_sas.as_ref());
    Reflect::set(&js_obj, &"claimerSas".into(), &js_claimer_sas)?;
    Ok(js_obj)
}

// ShamirRecoveryClaimInProgress3Info

#[allow(dead_code)]
fn struct_shamir_recovery_claim_in_progress3_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::ShamirRecoveryClaimInProgress3Info, JsValue> {
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
            let v = v as u32;
            v
        }
    };
    Ok(libparsec::ShamirRecoveryClaimInProgress3Info { handle })
}

#[allow(dead_code)]
fn struct_shamir_recovery_claim_in_progress3_info_rs_to_js(
    rs_obj: libparsec::ShamirRecoveryClaimInProgress3Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// ShamirRecoveryClaimInitialInfo

#[allow(dead_code)]
fn struct_shamir_recovery_claim_initial_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::ShamirRecoveryClaimInitialInfo, JsValue> {
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
            let v = v as u32;
            v
        }
    };
    let greeter_user_id = {
        let js_val = Reflect::get(&obj, &"greeterUserId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                    libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let greeter_human_handle = {
        let js_val = Reflect::get(&obj, &"greeterHumanHandle".into())?;
        struct_human_handle_js_to_rs(js_val)?
    };
    Ok(libparsec::ShamirRecoveryClaimInitialInfo {
        handle,
        greeter_user_id,
        greeter_human_handle,
    })
}

#[allow(dead_code)]
fn struct_shamir_recovery_claim_initial_info_rs_to_js(
    rs_obj: libparsec::ShamirRecoveryClaimInitialInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_greeter_user_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.greeter_user_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"greeterUserId".into(), &js_greeter_user_id)?;
    let js_greeter_human_handle = struct_human_handle_rs_to_js(rs_obj.greeter_human_handle)?;
    Reflect::set(
        &js_obj,
        &"greeterHumanHandle".into(),
        &js_greeter_human_handle,
    )?;
    Ok(js_obj)
}

// ShamirRecoveryClaimShareInfo

#[allow(dead_code)]
fn struct_shamir_recovery_claim_share_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::ShamirRecoveryClaimShareInfo, JsValue> {
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
            let v = v as u32;
            v
        }
    };
    Ok(libparsec::ShamirRecoveryClaimShareInfo { handle })
}

#[allow(dead_code)]
fn struct_shamir_recovery_claim_share_info_rs_to_js(
    rs_obj: libparsec::ShamirRecoveryClaimShareInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// ShamirRecoveryGreetInProgress1Info

#[allow(dead_code)]
fn struct_shamir_recovery_greet_in_progress1_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::ShamirRecoveryGreetInProgress1Info, JsValue> {
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
            let v = v as u32;
            v
        }
    };
    let greeter_sas = {
        let js_val = Reflect::get(&obj, &"greeterSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    Ok(libparsec::ShamirRecoveryGreetInProgress1Info {
        handle,
        greeter_sas,
    })
}

#[allow(dead_code)]
fn struct_shamir_recovery_greet_in_progress1_info_rs_to_js(
    rs_obj: libparsec::ShamirRecoveryGreetInProgress1Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_greeter_sas = JsValue::from_str(rs_obj.greeter_sas.as_ref());
    Reflect::set(&js_obj, &"greeterSas".into(), &js_greeter_sas)?;
    Ok(js_obj)
}

// ShamirRecoveryGreetInProgress2Info

#[allow(dead_code)]
fn struct_shamir_recovery_greet_in_progress2_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::ShamirRecoveryGreetInProgress2Info, JsValue> {
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
            let v = v as u32;
            v
        }
    };
    let claimer_sas = {
        let js_val = Reflect::get(&obj, &"claimerSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
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
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?;
                converted.push(x_converted);
            }
            converted
        }
    };
    Ok(libparsec::ShamirRecoveryGreetInProgress2Info {
        handle,
        claimer_sas,
        claimer_sas_choices,
    })
}

#[allow(dead_code)]
fn struct_shamir_recovery_greet_in_progress2_info_rs_to_js(
    rs_obj: libparsec::ShamirRecoveryGreetInProgress2Info,
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

// ShamirRecoveryGreetInProgress3Info

#[allow(dead_code)]
fn struct_shamir_recovery_greet_in_progress3_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::ShamirRecoveryGreetInProgress3Info, JsValue> {
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
            let v = v as u32;
            v
        }
    };
    Ok(libparsec::ShamirRecoveryGreetInProgress3Info { handle })
}

#[allow(dead_code)]
fn struct_shamir_recovery_greet_in_progress3_info_rs_to_js(
    rs_obj: libparsec::ShamirRecoveryGreetInProgress3Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// ShamirRecoveryGreetInitialInfo

#[allow(dead_code)]
fn struct_shamir_recovery_greet_initial_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::ShamirRecoveryGreetInitialInfo, JsValue> {
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
            let v = v as u32;
            v
        }
    };
    Ok(libparsec::ShamirRecoveryGreetInitialInfo { handle })
}

#[allow(dead_code)]
fn struct_shamir_recovery_greet_initial_info_rs_to_js(
    rs_obj: libparsec::ShamirRecoveryGreetInitialInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    Ok(js_obj)
}

// ShamirRecoveryRecipient

#[allow(dead_code)]
fn struct_shamir_recovery_recipient_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::ShamirRecoveryRecipient, JsValue> {
    let user_id = {
        let js_val = Reflect::get(&obj, &"userId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                    libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let human_handle = {
        let js_val = Reflect::get(&obj, &"humanHandle".into())?;
        struct_human_handle_js_to_rs(js_val)?
    };
    let revoked_on = {
        let js_val = Reflect::get(&obj, &"revokedOn".into())?;
        if js_val.is_null() {
            None
        } else {
            Some({
                let v = js_val.dyn_into::<Number>()?.value_of();
                let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                    libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                        .map_err(|_| "Out-of-bound datetime")
                };
                let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                v
            })
        }
    };
    let shares = {
        let js_val = Reflect::get(&obj, &"shares".into())?;
        {
            let v = js_val
                .dyn_into::<Number>()
                .map_err(|_| TypeError::new("Not a number"))?
                .value_of();
            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                return Err(JsValue::from(TypeError::new("Not an u8 number")));
            }
            let v = v as u8;
            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
            };
            match custom_from_rs_u8(v) {
                Ok(val) => val,
                Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
            }
        }
    };
    let online_status = {
        let js_val = Reflect::get(&obj, &"onlineStatus".into())?;
        {
            let raw_string = js_val.as_string().ok_or_else(|| {
                let type_error = TypeError::new("value is not a string");
                type_error.set_cause(&js_val);
                JsValue::from(type_error)
            })?;
            enum_user_online_status_js_to_rs(raw_string.as_str())
        }?
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
fn struct_shamir_recovery_recipient_rs_to_js(
    rs_obj: libparsec::ShamirRecoveryRecipient,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_user_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.user_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"userId".into(), &js_user_id)?;
    let js_human_handle = struct_human_handle_rs_to_js(rs_obj.human_handle)?;
    Reflect::set(&js_obj, &"humanHandle".into(), &js_human_handle)?;
    let js_revoked_on = match rs_obj.revoked_on {
        Some(val) => {
            let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
    let js_shares = {
        let custom_to_rs_u8 = |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
        let v = match custom_to_rs_u8(rs_obj.shares) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        };
        JsValue::from(v)
    };
    Reflect::set(&js_obj, &"shares".into(), &js_shares)?;
    let js_online_status =
        JsValue::from_str(enum_user_online_status_rs_to_js(rs_obj.online_status));
    Reflect::set(&js_obj, &"onlineStatus".into(), &js_online_status)?;
    Ok(js_obj)
}

// StartedWorkspaceInfo

#[allow(dead_code)]
fn struct_started_workspace_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::StartedWorkspaceInfo, JsValue> {
    let client = {
        let js_val = Reflect::get(&obj, &"client".into())?;
        {
            let v = js_val
                .dyn_into::<Number>()
                .map_err(|_| TypeError::new("Not a number"))?
                .value_of();
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                return Err(JsValue::from(TypeError::new("Not an u32 number")));
            }
            let v = v as u32;
            v
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
            })?
    };
    let current_name = {
        let js_val = Reflect::get(&obj, &"currentName".into())?;
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
            })?
    };
    let current_self_role = {
        let js_val = Reflect::get(&obj, &"currentSelfRole".into())?;
        {
            let raw_string = js_val.as_string().ok_or_else(|| {
                let type_error = TypeError::new("value is not a string");
                type_error.set_cause(&js_val);
                JsValue::from(type_error)
            })?;
            enum_realm_role_js_to_rs(raw_string.as_str())
        }?
    };
    let mountpoints = {
        let js_val = Reflect::get(&obj, &"mountpoints".into())?;
        {
            let js_val = js_val
                .dyn_into::<Array>()
                .map_err(|_| TypeError::new("Not an array"))?;
            let mut converted = Vec::with_capacity(js_val.length() as usize);
            for x in js_val.iter() {
                let x_converted = (
                    {
                        let js_x1 = Reflect::get_u32(&x, 1)?;
                        {
                            let v = js_x1
                                .dyn_into::<Number>()
                                .map_err(|_| TypeError::new("Not a number"))?
                                .value_of();
                            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                                return Err(JsValue::from(TypeError::new("Not an u32 number")));
                            }
                            let v = v as u32;
                            v
                        }
                    },
                    {
                        let js_x2 = Reflect::get_u32(&x, 2)?;
                        js_x2
                            .dyn_into::<JsString>()
                            .ok()
                            .and_then(|s| s.as_string())
                            .ok_or_else(|| TypeError::new("Not a string"))
                            .and_then(|x| {
                                let custom_from_rs_string = |s: String| -> Result<_, &'static str> {
                                    Ok(std::path::PathBuf::from(s))
                                };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })?
                    },
                );
                converted.push(x_converted);
            }
            converted
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
fn struct_started_workspace_info_rs_to_js(
    rs_obj: libparsec::StartedWorkspaceInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_client = JsValue::from(rs_obj.client);
    Reflect::set(&js_obj, &"client".into(), &js_client)?;
    let js_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"id".into(), &js_id)?;
    let js_current_name = JsValue::from_str(rs_obj.current_name.as_ref());
    Reflect::set(&js_obj, &"currentName".into(), &js_current_name)?;
    let js_current_self_role =
        JsValue::from_str(enum_realm_role_rs_to_js(rs_obj.current_self_role));
    Reflect::set(&js_obj, &"currentSelfRole".into(), &js_current_self_role)?;
    let js_mountpoints = {
        // Array::new_with_length allocates with `undefined` value, that's why we `set` value
        let js_array = Array::new_with_length(rs_obj.mountpoints.len() as u32);
        for (i, elem) in rs_obj.mountpoints.into_iter().enumerate() {
            let js_elem = {
                let (x1, x2) = elem;
                // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                let js_array = Array::new_with_length(2);
                let js_value = JsValue::from(x1);
                js_array.set(0, js_value);
                let js_value = JsValue::from_str({
                    let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                        path.into_os_string()
                            .into_string()
                            .map_err(|_| "Path contains non-utf8 characters")
                    };
                    match custom_to_rs_string(x2) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                    }
                    .as_ref()
                });
                js_array.set(1, js_value);
                js_array.into()
            };
            js_array.set(i as u32, js_elem);
        }
        js_array.into()
    };
    Reflect::set(&js_obj, &"mountpoints".into(), &js_mountpoints)?;
    Ok(js_obj)
}

// Tos

#[allow(dead_code)]
fn struct_tos_js_to_rs(obj: JsValue) -> Result<libparsec::Tos, JsValue> {
    let per_locale_urls = {
        let js_val = Reflect::get(&obj, &"perLocaleUrls".into())?;
        {
            let js_val = js_val
                .dyn_into::<Map>()
                .map_err(|_| TypeError::new("Not a Map"))?;
            let mut converted = std::collections::HashMap::with_capacity(js_val.size() as usize);
            let js_keys = js_val.keys();
            let js_values = js_val.values();
            loop {
                let next_js_key = js_keys.next()?;
                let next_js_value = js_values.next()?;
                if next_js_key.done() {
                    assert!(next_js_value.done());
                    break;
                }
                assert!(!next_js_value.done());

                let js_key = next_js_key.value();
                let js_value = next_js_value.value();

                let key = js_key
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?;
                let value = js_value
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?;
                converted.insert(key, value);
            }
            converted
        }
    };
    let updated_on = {
        let js_val = Reflect::get(&obj, &"updatedOn".into())?;
        {
            let v = js_val.dyn_into::<Number>()?.value_of();
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
            v
        }
    };
    Ok(libparsec::Tos {
        per_locale_urls,
        updated_on,
    })
}

#[allow(dead_code)]
fn struct_tos_rs_to_js(rs_obj: libparsec::Tos) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_per_locale_urls = {
        let js_map = Map::new();
        for (key, value) in rs_obj.per_locale_urls.into_iter() {
            let js_key = key.into();
            let js_value = value.into();
            js_map.set(&js_key, &js_value);
        }
        js_map.into()
    };
    Reflect::set(&js_obj, &"perLocaleUrls".into(), &js_per_locale_urls)?;
    let js_updated_on = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        let v = match custom_to_rs_f64(rs_obj.updated_on) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        };
        JsValue::from(v)
    };
    Reflect::set(&js_obj, &"updatedOn".into(), &js_updated_on)?;
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
            let v = v as u32;
            v
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
            let v = v as u32;
            v
        }
    };
    let greeter_user_id = {
        let js_val = Reflect::get(&obj, &"greeterUserId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                    libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let greeter_human_handle = {
        let js_val = Reflect::get(&obj, &"greeterHumanHandle".into())?;
        struct_human_handle_js_to_rs(js_val)?
    };
    let greeter_sas = {
        let js_val = Reflect::get(&obj, &"greeterSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
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
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?;
                converted.push(x_converted);
            }
            converted
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
fn struct_user_claim_in_progress1_info_rs_to_js(
    rs_obj: libparsec::UserClaimInProgress1Info,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_greeter_user_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.greeter_user_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"greeterUserId".into(), &js_greeter_user_id)?;
    let js_greeter_human_handle = struct_human_handle_rs_to_js(rs_obj.greeter_human_handle)?;
    Reflect::set(
        &js_obj,
        &"greeterHumanHandle".into(),
        &js_greeter_human_handle,
    )?;
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
            let v = v as u32;
            v
        }
    };
    let claimer_sas = {
        let js_val = Reflect::get(&obj, &"claimerSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
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
            let v = v as u32;
            v
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

// UserClaimInitialInfo

#[allow(dead_code)]
fn struct_user_claim_initial_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserClaimInitialInfo, JsValue> {
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
            let v = v as u32;
            v
        }
    };
    let greeter_user_id = {
        let js_val = Reflect::get(&obj, &"greeterUserId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                    libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let greeter_human_handle = {
        let js_val = Reflect::get(&obj, &"greeterHumanHandle".into())?;
        struct_human_handle_js_to_rs(js_val)?
    };
    let online_status = {
        let js_val = Reflect::get(&obj, &"onlineStatus".into())?;
        {
            let raw_string = js_val.as_string().ok_or_else(|| {
                let type_error = TypeError::new("value is not a string");
                type_error.set_cause(&js_val);
                JsValue::from(type_error)
            })?;
            enum_user_online_status_js_to_rs(raw_string.as_str())
        }?
    };
    let last_greeting_attempt_joined_on = {
        let js_val = Reflect::get(&obj, &"lastGreetingAttemptJoinedOn".into())?;
        if js_val.is_null() {
            None
        } else {
            Some({
                let v = js_val.dyn_into::<Number>()?.value_of();
                let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                    libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                        .map_err(|_| "Out-of-bound datetime")
                };
                let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                v
            })
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
fn struct_user_claim_initial_info_rs_to_js(
    rs_obj: libparsec::UserClaimInitialInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_handle = JsValue::from(rs_obj.handle);
    Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
    let js_greeter_user_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.greeter_user_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"greeterUserId".into(), &js_greeter_user_id)?;
    let js_greeter_human_handle = struct_human_handle_rs_to_js(rs_obj.greeter_human_handle)?;
    Reflect::set(
        &js_obj,
        &"greeterHumanHandle".into(),
        &js_greeter_human_handle,
    )?;
    let js_online_status =
        JsValue::from_str(enum_user_online_status_rs_to_js(rs_obj.online_status));
    Reflect::set(&js_obj, &"onlineStatus".into(), &js_online_status)?;
    let js_last_greeting_attempt_joined_on = match rs_obj.last_greeting_attempt_joined_on {
        Some(val) => {
            let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
            };
            let v = match custom_to_rs_f64(val) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
            };
            JsValue::from(v)
        }
        None => JsValue::NULL,
    };
    Reflect::set(
        &js_obj,
        &"lastGreetingAttemptJoinedOn".into(),
        &js_last_greeting_attempt_joined_on,
    )?;
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
            let v = v as u32;
            v
        }
    };
    let greeter_sas = {
        let js_val = Reflect::get(&obj, &"greeterSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
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
            let v = v as u32;
            v
        }
    };
    let claimer_sas = {
        let js_val = Reflect::get(&obj, &"claimerSas".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
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
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            s.parse::<libparsec::SASCode>().map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?;
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
            let v = v as u32;
            v
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
            let v = v as u32;
            v
        }
    };
    let requested_human_handle = {
        let js_val = Reflect::get(&obj, &"requestedHumanHandle".into())?;
        struct_human_handle_js_to_rs(js_val)?
    };
    let requested_device_label = {
        let js_val = Reflect::get(&obj, &"requestedDeviceLabel".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
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
    let js_requested_human_handle = struct_human_handle_rs_to_js(rs_obj.requested_human_handle)?;
    Reflect::set(
        &js_obj,
        &"requestedHumanHandle".into(),
        &js_requested_human_handle,
    )?;
    let js_requested_device_label = JsValue::from_str(rs_obj.requested_device_label.as_ref());
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
            let v = v as u32;
            v
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

// UserGreetingAdministrator

#[allow(dead_code)]
fn struct_user_greeting_administrator_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::UserGreetingAdministrator, JsValue> {
    let user_id = {
        let js_val = Reflect::get(&obj, &"userId".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                    libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let human_handle = {
        let js_val = Reflect::get(&obj, &"humanHandle".into())?;
        struct_human_handle_js_to_rs(js_val)?
    };
    let online_status = {
        let js_val = Reflect::get(&obj, &"onlineStatus".into())?;
        {
            let raw_string = js_val.as_string().ok_or_else(|| {
                let type_error = TypeError::new("value is not a string");
                type_error.set_cause(&js_val);
                JsValue::from(type_error)
            })?;
            enum_user_online_status_js_to_rs(raw_string.as_str())
        }?
    };
    let last_greeting_attempt_joined_on = {
        let js_val = Reflect::get(&obj, &"lastGreetingAttemptJoinedOn".into())?;
        if js_val.is_null() {
            None
        } else {
            Some({
                let v = js_val.dyn_into::<Number>()?.value_of();
                let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                    libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                        .map_err(|_| "Out-of-bound datetime")
                };
                let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                v
            })
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
fn struct_user_greeting_administrator_rs_to_js(
    rs_obj: libparsec::UserGreetingAdministrator,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_user_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.user_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"userId".into(), &js_user_id)?;
    let js_human_handle = struct_human_handle_rs_to_js(rs_obj.human_handle)?;
    Reflect::set(&js_obj, &"humanHandle".into(), &js_human_handle)?;
    let js_online_status =
        JsValue::from_str(enum_user_online_status_rs_to_js(rs_obj.online_status));
    Reflect::set(&js_obj, &"onlineStatus".into(), &js_online_status)?;
    let js_last_greeting_attempt_joined_on = match rs_obj.last_greeting_attempt_joined_on {
        Some(val) => {
            let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
            };
            let v = match custom_to_rs_f64(val) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
            };
            JsValue::from(v)
        }
        None => JsValue::NULL,
    };
    Reflect::set(
        &js_obj,
        &"lastGreetingAttemptJoinedOn".into(),
        &js_last_greeting_attempt_joined_on,
    )?;
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
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                    libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let human_handle = {
        let js_val = Reflect::get(&obj, &"humanHandle".into())?;
        struct_human_handle_js_to_rs(js_val)?
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
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
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
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?,
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
                    libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                        .map_err(|_| "Out-of-bound datetime")
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
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?,
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
    let js_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"id".into(), &js_id)?;
    let js_human_handle = struct_human_handle_rs_to_js(rs_obj.human_handle)?;
    Reflect::set(&js_obj, &"humanHandle".into(), &js_human_handle)?;
    let js_current_profile = JsValue::from_str(enum_user_profile_rs_to_js(rs_obj.current_profile));
    Reflect::set(&js_obj, &"currentProfile".into(), &js_current_profile)?;
    let js_created_on = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
            let custom_to_rs_string =
                |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
            match custom_to_rs_string(val) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
            }
            .as_ref()
        }),
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
    let js_revoked_on = match rs_obj.revoked_on {
        Some(val) => {
            let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
            let custom_to_rs_string =
                |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
            match custom_to_rs_string(val) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
            }
            .as_ref()
        }),
        None => JsValue::NULL,
    };
    Reflect::set(&js_obj, &"revokedBy".into(), &js_revoked_by)?;
    Ok(js_obj)
}

// WorkspaceHistoryFileStat

#[allow(dead_code)]
fn struct_workspace_history_file_stat_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::WorkspaceHistoryFileStat, JsValue> {
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
            })?
    };
    let created = {
        let js_val = Reflect::get(&obj, &"created".into())?;
        {
            let v = js_val.dyn_into::<Number>()?.value_of();
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
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
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
            v
        }
    };
    let version = {
        let js_val = Reflect::get(&obj, &"version".into())?;
        {
            let v = js_val
                .dyn_into::<Number>()
                .map_err(|_| TypeError::new("Not a number"))?
                .value_of();
            if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                return Err(JsValue::from(TypeError::new("Not an u32 number")));
            }
            let v = v as u32;
            v
        }
    };
    let size = {
        let js_val = Reflect::get(&obj, &"size".into())?;
        {
            let v = u64::try_from(js_val)
                .map_err(|_| TypeError::new("Not a BigInt representing an u64 number"))?;
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
fn struct_workspace_history_file_stat_rs_to_js(
    rs_obj: libparsec::WorkspaceHistoryFileStat,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"id".into(), &js_id)?;
    let js_created = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        let v = match custom_to_rs_f64(rs_obj.created) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        };
        JsValue::from(v)
    };
    Reflect::set(&js_obj, &"created".into(), &js_created)?;
    let js_updated = {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        let v = match custom_to_rs_f64(rs_obj.updated) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
        };
        JsValue::from(v)
    };
    Reflect::set(&js_obj, &"updated".into(), &js_updated)?;
    let js_version = JsValue::from(rs_obj.version);
    Reflect::set(&js_obj, &"version".into(), &js_version)?;
    let js_size = JsValue::from(rs_obj.size);
    Reflect::set(&js_obj, &"size".into(), &js_size)?;
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
            })?
    };
    let current_name = {
        let js_val = Reflect::get(&obj, &"currentName".into())?;
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
            })?
    };
    let current_self_role = {
        let js_val = Reflect::get(&obj, &"currentSelfRole".into())?;
        {
            let raw_string = js_val.as_string().ok_or_else(|| {
                let type_error = TypeError::new("value is not a string");
                type_error.set_cause(&js_val);
                JsValue::from(type_error)
            })?;
            enum_realm_role_js_to_rs(raw_string.as_str())
        }?
    };
    let is_started = {
        let js_val = Reflect::get(&obj, &"isStarted".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
    };
    let is_bootstrapped = {
        let js_val = Reflect::get(&obj, &"isBootstrapped".into())?;
        js_val
            .dyn_into::<Boolean>()
            .map_err(|_| TypeError::new("Not a boolean"))?
            .value_of()
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
fn struct_workspace_info_rs_to_js(rs_obj: libparsec::WorkspaceInfo) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"id".into(), &js_id)?;
    let js_current_name = JsValue::from_str(rs_obj.current_name.as_ref());
    Reflect::set(&js_obj, &"currentName".into(), &js_current_name)?;
    let js_current_self_role =
        JsValue::from_str(enum_realm_role_rs_to_js(rs_obj.current_self_role));
    Reflect::set(&js_obj, &"currentSelfRole".into(), &js_current_self_role)?;
    let js_is_started = rs_obj.is_started.into();
    Reflect::set(&js_obj, &"isStarted".into(), &js_is_started)?;
    let js_is_bootstrapped = rs_obj.is_bootstrapped.into();
    Reflect::set(&js_obj, &"isBootstrapped".into(), &js_is_bootstrapped)?;
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
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                    libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    let human_handle = {
        let js_val = Reflect::get(&obj, &"humanHandle".into())?;
        struct_human_handle_js_to_rs(js_val)?
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
    let current_role = {
        let js_val = Reflect::get(&obj, &"currentRole".into())?;
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
        current_profile,
        current_role,
    })
}

#[allow(dead_code)]
fn struct_workspace_user_access_info_rs_to_js(
    rs_obj: libparsec::WorkspaceUserAccessInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_user_id = JsValue::from_str({
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.user_id) {
            Ok(ok) => ok,
            Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
        }
        .as_ref()
    });
    Reflect::set(&js_obj, &"userId".into(), &js_user_id)?;
    let js_human_handle = struct_human_handle_rs_to_js(rs_obj.human_handle)?;
    Reflect::set(&js_obj, &"humanHandle".into(), &js_human_handle)?;
    let js_current_profile = JsValue::from_str(enum_user_profile_rs_to_js(rs_obj.current_profile));
    Reflect::set(&js_obj, &"currentProfile".into(), &js_current_profile)?;
    let js_current_role = JsValue::from_str(enum_realm_role_rs_to_js(rs_obj.current_role));
    Reflect::set(&js_obj, &"currentRole".into(), &js_current_role)?;
    Ok(js_obj)
}

// X509CertificateReference

#[allow(dead_code)]
fn struct_x509_certificate_reference_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::X509CertificateReference, JsValue> {
    let uris = {
        let js_val = Reflect::get(&obj, &"uris".into())?;
        {
            let js_val = js_val
                .dyn_into::<Array>()
                .map_err(|_| TypeError::new("Not an array"))?;
            let mut converted = Vec::with_capacity(js_val.length() as usize);
            for x in js_val.iter() {
                let x_converted = variant_x509_uri_flavor_value_js_to_rs(x)?;
                converted.push(x_converted);
            }
            converted
        }
    };
    let hash = {
        let js_val = Reflect::get(&obj, &"hash".into())?;
        js_val
            .dyn_into::<JsString>()
            .ok()
            .and_then(|s| s.as_string())
            .ok_or_else(|| TypeError::new("Not a string"))
            .and_then(|x| {
                let custom_from_rs_string = |s: String| -> Result<_, String> {
                    <libparsec::X509CertificateHash as std::str::FromStr>::from_str(s.as_str())
                        .map_err(|e| e.to_string())
                };
                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
            })?
    };
    {
        let custom_init = |uris: Vec<libparsec::X509URIFlavorValue>,
                           hash: libparsec::X509CertificateHash|
         -> Result<_, String> {
            let mut cert_ref = libparsec::X509CertificateReference::from(hash);
            for uri in uris {
                cert_ref = cert_ref.add_or_replace_uri_wrapped(uri);
            }
            Ok(cert_ref)
        };
        custom_init(uris, hash).map_err(|e| e.into())
    }
}

#[allow(dead_code)]
fn struct_x509_certificate_reference_rs_to_js(
    rs_obj: libparsec::X509CertificateReference,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_uris = {
        let custom_getter =
            |o: &libparsec::X509CertificateReference| -> Vec<libparsec::X509URIFlavorValue> {
                o.uris().cloned().collect()
            };
        {
            // Array::new_with_length allocates with `undefined` value, that's why we `set` value
            let js_array = Array::new_with_length(custom_getter(&rs_obj).len() as u32);
            for (i, elem) in custom_getter(&rs_obj).into_iter().enumerate() {
                let js_elem = variant_x509_uri_flavor_value_rs_to_js(elem)?;
                js_array.set(i as u32, js_elem);
            }
            js_array.into()
        }
    };
    Reflect::set(&js_obj, &"uris".into(), &js_uris)?;
    let js_hash = {
        let custom_getter =
            |o: &libparsec::X509CertificateReference| -> libparsec::X509CertificateHash {
                o.hash.clone()
            };
        JsValue::from_str({
            let custom_to_rs_string =
                |x: libparsec::X509CertificateHash| -> Result<_, &'static str> {
                    Ok(x.to_string())
                };
            match custom_to_rs_string(custom_getter(&rs_obj)) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
            }
            .as_ref()
        })
    };
    Reflect::set(&js_obj, &"hash".into(), &js_hash)?;
    Ok(js_obj)
}

// AccountAuthMethodStrategy

#[allow(dead_code)]
fn variant_account_auth_method_strategy_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::AccountAuthMethodStrategy, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "AccountAuthMethodStrategyMasterSecret" => {
            let master_secret = {
                let js_val = Reflect::get(&obj, &"masterSecret".into())?;
                js_val
                    .dyn_into::<Uint8Array>()
                    .map(|x| x.to_vec())
                    .map_err(|_| TypeError::new("Not a Uint8Array"))
                    .and_then(|x| {
                        let xx: &[u8] = &x;
                        xx.try_into()
                            .map_err(|_| TypeError::new("Not a valid KeyDerivation"))
                    })?
            };
            Ok(libparsec::AccountAuthMethodStrategy::MasterSecret { master_secret })
        }
        "AccountAuthMethodStrategyPassword" => {
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
                    })?
            };
            Ok(libparsec::AccountAuthMethodStrategy::Password { password })
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a AccountAuthMethodStrategy",
        ))),
    }
}

#[allow(dead_code)]
fn variant_account_auth_method_strategy_rs_to_js(
    rs_obj: libparsec::AccountAuthMethodStrategy,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::AccountAuthMethodStrategy::MasterSecret { master_secret, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountAuthMethodStrategyMasterSecret".into(),
            )?;
            let js_master_secret = JsValue::from(Uint8Array::from(master_secret.as_ref()));
            Reflect::set(&js_obj, &"masterSecret".into(), &js_master_secret)?;
        }
        libparsec::AccountAuthMethodStrategy::Password { password, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountAuthMethodStrategyPassword".into(),
            )?;
            let js_password = JsValue::from_str(password.as_ref());
            Reflect::set(&js_obj, &"password".into(), &js_password)?;
        }
    }
    Ok(js_obj)
}

// AccountCreateAuthMethodError

#[allow(dead_code)]
fn variant_account_create_auth_method_error_rs_to_js(
    rs_obj: libparsec::AccountCreateAuthMethodError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountCreateAuthMethodError::BadVaultKeyAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountCreateAuthMethodErrorBadVaultKeyAccess".into(),
            )?;
        }
        libparsec::AccountCreateAuthMethodError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountCreateAuthMethodErrorInternal".into(),
            )?;
        }
        libparsec::AccountCreateAuthMethodError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountCreateAuthMethodErrorOffline".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// AccountCreateError

#[allow(dead_code)]
fn variant_account_create_error_rs_to_js(
    rs_obj: libparsec::AccountCreateError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountCreateError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountCreateErrorInternal".into())?;
        }
        libparsec::AccountCreateError::InvalidValidationCode { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountCreateErrorInvalidValidationCode".into(),
            )?;
        }
        libparsec::AccountCreateError::Offline { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountCreateErrorOffline".into())?;
        }
        libparsec::AccountCreateError::SendValidationEmailRequired { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountCreateErrorSendValidationEmailRequired".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// AccountCreateRegistrationDeviceError

#[allow(dead_code)]
fn variant_account_create_registration_device_error_rs_to_js(
    rs_obj: libparsec::AccountCreateRegistrationDeviceError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountCreateRegistrationDeviceError::BadVaultKeyAccess{   .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountCreateRegistrationDeviceErrorBadVaultKeyAccess".into())?;
        }
        libparsec::AccountCreateRegistrationDeviceError::CannotObtainOrganizationVaultStrategy{   .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountCreateRegistrationDeviceErrorCannotObtainOrganizationVaultStrategy".into())?;
        }
        libparsec::AccountCreateRegistrationDeviceError::Internal{   .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountCreateRegistrationDeviceErrorInternal".into())?;
        }
        libparsec::AccountCreateRegistrationDeviceError::LoadDeviceDecryptionFailed{   .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountCreateRegistrationDeviceErrorLoadDeviceDecryptionFailed".into())?;
        }
        libparsec::AccountCreateRegistrationDeviceError::LoadDeviceInvalidData{   .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountCreateRegistrationDeviceErrorLoadDeviceInvalidData".into())?;
        }
        libparsec::AccountCreateRegistrationDeviceError::LoadDeviceInvalidPath{   .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountCreateRegistrationDeviceErrorLoadDeviceInvalidPath".into())?;
        }
        libparsec::AccountCreateRegistrationDeviceError::NotAllowedByOrganizationVaultStrategy{   .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountCreateRegistrationDeviceErrorNotAllowedByOrganizationVaultStrategy".into())?;
        }
        libparsec::AccountCreateRegistrationDeviceError::Offline{   .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountCreateRegistrationDeviceErrorOffline".into())?;
        }
        libparsec::AccountCreateRegistrationDeviceError::RemoteOpaqueKeyFetchFailed{   .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountCreateRegistrationDeviceErrorRemoteOpaqueKeyFetchFailed".into())?;
        }
        libparsec::AccountCreateRegistrationDeviceError::TimestampOutOfBallpark{   .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountCreateRegistrationDeviceErrorTimestampOutOfBallpark".into())?;
        }
    }
    Ok(js_obj)
}

// AccountCreateSendValidationEmailError

#[allow(dead_code)]
fn variant_account_create_send_validation_email_error_rs_to_js(
    rs_obj: libparsec::AccountCreateSendValidationEmailError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountCreateSendValidationEmailError::EmailRecipientRefused { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountCreateSendValidationEmailErrorEmailRecipientRefused".into(),
            )?;
        }
        libparsec::AccountCreateSendValidationEmailError::EmailSendingRateLimited {
            wait_until,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountCreateSendValidationEmailErrorEmailSendingRateLimited".into(),
            )?;
            let js_wait_until = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(wait_until) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"waitUntil".into(), &js_wait_until)?;
        }
        libparsec::AccountCreateSendValidationEmailError::EmailServerUnavailable { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountCreateSendValidationEmailErrorEmailServerUnavailable".into(),
            )?;
        }
        libparsec::AccountCreateSendValidationEmailError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountCreateSendValidationEmailErrorInternal".into(),
            )?;
        }
        libparsec::AccountCreateSendValidationEmailError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountCreateSendValidationEmailErrorOffline".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// AccountDeleteProceedError

#[allow(dead_code)]
fn variant_account_delete_proceed_error_rs_to_js(
    rs_obj: libparsec::AccountDeleteProceedError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountDeleteProceedError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountDeleteProceedErrorInternal".into(),
            )?;
        }
        libparsec::AccountDeleteProceedError::InvalidValidationCode { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountDeleteProceedErrorInvalidValidationCode".into(),
            )?;
        }
        libparsec::AccountDeleteProceedError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountDeleteProceedErrorOffline".into(),
            )?;
        }
        libparsec::AccountDeleteProceedError::SendValidationEmailRequired { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountDeleteProceedErrorSendValidationEmailRequired".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// AccountDeleteSendValidationEmailError

#[allow(dead_code)]
fn variant_account_delete_send_validation_email_error_rs_to_js(
    rs_obj: libparsec::AccountDeleteSendValidationEmailError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountDeleteSendValidationEmailError::EmailRecipientRefused { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountDeleteSendValidationEmailErrorEmailRecipientRefused".into(),
            )?;
        }
        libparsec::AccountDeleteSendValidationEmailError::EmailSendingRateLimited {
            wait_until,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountDeleteSendValidationEmailErrorEmailSendingRateLimited".into(),
            )?;
            let js_wait_until = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(wait_until) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"waitUntil".into(), &js_wait_until)?;
        }
        libparsec::AccountDeleteSendValidationEmailError::EmailServerUnavailable { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountDeleteSendValidationEmailErrorEmailServerUnavailable".into(),
            )?;
        }
        libparsec::AccountDeleteSendValidationEmailError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountDeleteSendValidationEmailErrorInternal".into(),
            )?;
        }
        libparsec::AccountDeleteSendValidationEmailError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountDeleteSendValidationEmailErrorOffline".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// AccountDisableAuthMethodError

#[allow(dead_code)]
fn variant_account_disable_auth_method_error_rs_to_js(
    rs_obj: libparsec::AccountDisableAuthMethodError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountDisableAuthMethodError::AuthMethodAlreadyDisabled { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountDisableAuthMethodErrorAuthMethodAlreadyDisabled".into(),
            )?;
        }
        libparsec::AccountDisableAuthMethodError::AuthMethodNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountDisableAuthMethodErrorAuthMethodNotFound".into(),
            )?;
        }
        libparsec::AccountDisableAuthMethodError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountDisableAuthMethodErrorInternal".into(),
            )?;
        }
        libparsec::AccountDisableAuthMethodError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountDisableAuthMethodErrorOffline".into(),
            )?;
        }
        libparsec::AccountDisableAuthMethodError::SelfDisableNotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountDisableAuthMethodErrorSelfDisableNotAllowed".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// AccountInfoError

#[allow(dead_code)]
fn variant_account_info_error_rs_to_js(
    rs_obj: libparsec::AccountInfoError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountInfoError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountInfoErrorInternal".into())?;
        }
    }
    Ok(js_obj)
}

// AccountListAuthMethodsError

#[allow(dead_code)]
fn variant_account_list_auth_methods_error_rs_to_js(
    rs_obj: libparsec::AccountListAuthMethodsError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountListAuthMethodsError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountListAuthMethodsErrorInternal".into(),
            )?;
        }
        libparsec::AccountListAuthMethodsError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountListAuthMethodsErrorOffline".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// AccountListInvitationsError

#[allow(dead_code)]
fn variant_account_list_invitations_error_rs_to_js(
    rs_obj: libparsec::AccountListInvitationsError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountListInvitationsError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountListInvitationsErrorInternal".into(),
            )?;
        }
        libparsec::AccountListInvitationsError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountListInvitationsErrorOffline".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// AccountListOrganizationsError

#[allow(dead_code)]
fn variant_account_list_organizations_error_rs_to_js(
    rs_obj: libparsec::AccountListOrganizationsError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountListOrganizationsError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountListOrganizationsErrorInternal".into(),
            )?;
        }
        libparsec::AccountListOrganizationsError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountListOrganizationsErrorOffline".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// AccountListRegistrationDevicesError

#[allow(dead_code)]
fn variant_account_list_registration_devices_error_rs_to_js(
    rs_obj: libparsec::AccountListRegistrationDevicesError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountListRegistrationDevicesError::BadVaultKeyAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountListRegistrationDevicesErrorBadVaultKeyAccess".into(),
            )?;
        }
        libparsec::AccountListRegistrationDevicesError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountListRegistrationDevicesErrorInternal".into(),
            )?;
        }
        libparsec::AccountListRegistrationDevicesError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountListRegistrationDevicesErrorOffline".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// AccountLoginError

#[allow(dead_code)]
fn variant_account_login_error_rs_to_js(
    rs_obj: libparsec::AccountLoginError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountLoginError::BadPasswordAlgorithm { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountLoginErrorBadPasswordAlgorithm".into(),
            )?;
        }
        libparsec::AccountLoginError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountLoginErrorInternal".into())?;
        }
        libparsec::AccountLoginError::Offline { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountLoginErrorOffline".into())?;
        }
    }
    Ok(js_obj)
}

// AccountLoginStrategy

#[allow(dead_code)]
fn variant_account_login_strategy_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::AccountLoginStrategy, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "AccountLoginStrategyMasterSecret" => {
            let master_secret = {
                let js_val = Reflect::get(&obj, &"masterSecret".into())?;
                js_val
                    .dyn_into::<Uint8Array>()
                    .map(|x| x.to_vec())
                    .map_err(|_| TypeError::new("Not a Uint8Array"))
                    .and_then(|x| {
                        let xx: &[u8] = &x;
                        xx.try_into()
                            .map_err(|_| TypeError::new("Not a valid KeyDerivation"))
                    })?
            };
            Ok(libparsec::AccountLoginStrategy::MasterSecret { master_secret })
        }
        "AccountLoginStrategyPassword" => {
            let email = {
                let js_val = Reflect::get(&obj, &"email".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::EmailAddress::from_str(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
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
                    })?
            };
            Ok(libparsec::AccountLoginStrategy::Password { email, password })
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a AccountLoginStrategy",
        ))),
    }
}

#[allow(dead_code)]
fn variant_account_login_strategy_rs_to_js(
    rs_obj: libparsec::AccountLoginStrategy,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::AccountLoginStrategy::MasterSecret { master_secret, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountLoginStrategyMasterSecret".into(),
            )?;
            let js_master_secret = JsValue::from(Uint8Array::from(master_secret.as_ref()));
            Reflect::set(&js_obj, &"masterSecret".into(), &js_master_secret)?;
        }
        libparsec::AccountLoginStrategy::Password {
            email, password, ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountLoginStrategyPassword".into(),
            )?;
            let js_email = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::EmailAddress| -> Result<_, &'static str> { Ok(x.to_string()) };
                match custom_to_rs_string(email) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"email".into(), &js_email)?;
            let js_password = JsValue::from_str(password.as_ref());
            Reflect::set(&js_obj, &"password".into(), &js_password)?;
        }
    }
    Ok(js_obj)
}

// AccountLogoutError

#[allow(dead_code)]
fn variant_account_logout_error_rs_to_js(
    rs_obj: libparsec::AccountLogoutError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountLogoutError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AccountLogoutErrorInternal".into())?;
        }
    }
    Ok(js_obj)
}

// AccountRecoverProceedError

#[allow(dead_code)]
fn variant_account_recover_proceed_error_rs_to_js(
    rs_obj: libparsec::AccountRecoverProceedError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountRecoverProceedError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRecoverProceedErrorInternal".into(),
            )?;
        }
        libparsec::AccountRecoverProceedError::InvalidValidationCode { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRecoverProceedErrorInvalidValidationCode".into(),
            )?;
        }
        libparsec::AccountRecoverProceedError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRecoverProceedErrorOffline".into(),
            )?;
        }
        libparsec::AccountRecoverProceedError::SendValidationEmailRequired { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRecoverProceedErrorSendValidationEmailRequired".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// AccountRecoverSendValidationEmailError

#[allow(dead_code)]
fn variant_account_recover_send_validation_email_error_rs_to_js(
    rs_obj: libparsec::AccountRecoverSendValidationEmailError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountRecoverSendValidationEmailError::EmailRecipientRefused { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRecoverSendValidationEmailErrorEmailRecipientRefused".into(),
            )?;
        }
        libparsec::AccountRecoverSendValidationEmailError::EmailSendingRateLimited {
            wait_until,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRecoverSendValidationEmailErrorEmailSendingRateLimited".into(),
            )?;
            let js_wait_until = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(wait_until) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"waitUntil".into(), &js_wait_until)?;
        }
        libparsec::AccountRecoverSendValidationEmailError::EmailServerUnavailable { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRecoverSendValidationEmailErrorEmailServerUnavailable".into(),
            )?;
        }
        libparsec::AccountRecoverSendValidationEmailError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRecoverSendValidationEmailErrorInternal".into(),
            )?;
        }
        libparsec::AccountRecoverSendValidationEmailError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRecoverSendValidationEmailErrorOffline".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// AccountRegisterNewDeviceError

#[allow(dead_code)]
fn variant_account_register_new_device_error_rs_to_js(
    rs_obj: libparsec::AccountRegisterNewDeviceError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::AccountRegisterNewDeviceError::BadVaultKeyAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRegisterNewDeviceErrorBadVaultKeyAccess".into(),
            )?;
        }
        libparsec::AccountRegisterNewDeviceError::CorruptedRegistrationDevice { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRegisterNewDeviceErrorCorruptedRegistrationDevice".into(),
            )?;
        }
        libparsec::AccountRegisterNewDeviceError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRegisterNewDeviceErrorInternal".into(),
            )?;
        }
        libparsec::AccountRegisterNewDeviceError::InvalidPath { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRegisterNewDeviceErrorInvalidPath".into(),
            )?;
        }
        libparsec::AccountRegisterNewDeviceError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRegisterNewDeviceErrorOffline".into(),
            )?;
        }
        libparsec::AccountRegisterNewDeviceError::RemoteOpaqueKeyUploadFailed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRegisterNewDeviceErrorRemoteOpaqueKeyUploadFailed".into(),
            )?;
        }
        libparsec::AccountRegisterNewDeviceError::StorageNotAvailable { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRegisterNewDeviceErrorStorageNotAvailable".into(),
            )?;
        }
        libparsec::AccountRegisterNewDeviceError::TimestampOutOfBallpark { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRegisterNewDeviceErrorTimestampOutOfBallpark".into(),
            )?;
        }
        libparsec::AccountRegisterNewDeviceError::UnknownRegistrationDevice { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AccountRegisterNewDeviceErrorUnknownRegistrationDevice".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ActiveUsersLimit

#[allow(dead_code)]
fn variant_active_users_limit_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::ActiveUsersLimit, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "ActiveUsersLimitLimitedTo" => {
            let x1 = {
                let js_val = Reflect::get(&obj, &"x1".into())?;
                {
                    let v = u64::try_from(js_val)
                        .map_err(|_| TypeError::new("Not a BigInt representing an u64 number"))?;
                    v
                }
            };
            Ok(libparsec::ActiveUsersLimit::LimitedTo(x1))
        }
        "ActiveUsersLimitNoLimit" => Ok(libparsec::ActiveUsersLimit::NoLimit {}),
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a ActiveUsersLimit",
        ))),
    }
}

#[allow(dead_code)]
fn variant_active_users_limit_rs_to_js(
    rs_obj: libparsec::ActiveUsersLimit,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::ActiveUsersLimit::LimitedTo(x1, ..) => {
            Reflect::set(&js_obj, &"tag".into(), &"ActiveUsersLimitLimitedTo".into())?;
            let js_x1 = JsValue::from(x1);
            Reflect::set(&js_obj, &"x1".into(), &js_x1.into())?;
        }
        libparsec::ActiveUsersLimit::NoLimit { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ActiveUsersLimitNoLimit".into())?;
        }
    }
    Ok(js_obj)
}

// AnyClaimRetrievedInfo

#[allow(dead_code)]
fn variant_any_claim_retrieved_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::AnyClaimRetrievedInfo, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "AnyClaimRetrievedInfoDevice" => {
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
                    let v = v as u32;
                    v
                }
            };
            let greeter_user_id = {
                let js_val = Reflect::get(&obj, &"greeterUserId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                            libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let greeter_human_handle = {
                let js_val = Reflect::get(&obj, &"greeterHumanHandle".into())?;
                struct_human_handle_js_to_rs(js_val)?
            };
            Ok(libparsec::AnyClaimRetrievedInfo::Device {
                handle,
                greeter_user_id,
                greeter_human_handle,
            })
        }
        "AnyClaimRetrievedInfoShamirRecovery" => {
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
                    let v = v as u32;
                    v
                }
            };
            let claimer_user_id = {
                let js_val = Reflect::get(&obj, &"claimerUserId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                            libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let claimer_human_handle = {
                let js_val = Reflect::get(&obj, &"claimerHumanHandle".into())?;
                struct_human_handle_js_to_rs(js_val)?
            };
            let invitation_created_by = {
                let js_val = Reflect::get(&obj, &"invitationCreatedBy".into())?;
                variant_invite_info_invitation_created_by_js_to_rs(js_val)?
            };
            let shamir_recovery_created_on = {
                let js_val = Reflect::get(&obj, &"shamirRecoveryCreatedOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let recipients = {
                let js_val = Reflect::get(&obj, &"recipients".into())?;
                {
                    let js_val = js_val
                        .dyn_into::<Array>()
                        .map_err(|_| TypeError::new("Not an array"))?;
                    let mut converted = Vec::with_capacity(js_val.length() as usize);
                    for x in js_val.iter() {
                        let x_converted = struct_shamir_recovery_recipient_js_to_rs(x)?;
                        converted.push(x_converted);
                    }
                    converted
                }
            };
            let threshold = {
                let js_val = Reflect::get(&obj, &"threshold".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u8 number")));
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                }
            };
            let is_recoverable = {
                let js_val = Reflect::get(&obj, &"isRecoverable".into())?;
                js_val
                    .dyn_into::<Boolean>()
                    .map_err(|_| TypeError::new("Not a boolean"))?
                    .value_of()
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
                let js_val = Reflect::get(&obj, &"handle".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u32 number")));
                    }
                    let v = v as u32;
                    v
                }
            };
            let claimer_email = {
                let js_val = Reflect::get(&obj, &"claimerEmail".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::EmailAddress::from_str(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let created_by = {
                let js_val = Reflect::get(&obj, &"createdBy".into())?;
                variant_invite_info_invitation_created_by_js_to_rs(js_val)?
            };
            let administrators = {
                let js_val = Reflect::get(&obj, &"administrators".into())?;
                {
                    let js_val = js_val
                        .dyn_into::<Array>()
                        .map_err(|_| TypeError::new("Not an array"))?;
                    let mut converted = Vec::with_capacity(js_val.length() as usize);
                    for x in js_val.iter() {
                        let x_converted = struct_user_greeting_administrator_js_to_rs(x)?;
                        converted.push(x_converted);
                    }
                    converted
                }
            };
            let preferred_greeter = {
                let js_val = Reflect::get(&obj, &"preferredGreeter".into())?;
                if js_val.is_null() {
                    None
                } else {
                    Some(struct_user_greeting_administrator_js_to_rs(js_val)?)
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
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a AnyClaimRetrievedInfo",
        ))),
    }
}

#[allow(dead_code)]
fn variant_any_claim_retrieved_info_rs_to_js(
    rs_obj: libparsec::AnyClaimRetrievedInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::AnyClaimRetrievedInfo::Device {
            handle,
            greeter_user_id,
            greeter_human_handle,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AnyClaimRetrievedInfoDevice".into(),
            )?;
            let js_handle = JsValue::from(handle);
            Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
            let js_greeter_user_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(greeter_user_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"greeterUserId".into(), &js_greeter_user_id)?;
            let js_greeter_human_handle = struct_human_handle_rs_to_js(greeter_human_handle)?;
            Reflect::set(
                &js_obj,
                &"greeterHumanHandle".into(),
                &js_greeter_human_handle,
            )?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AnyClaimRetrievedInfoShamirRecovery".into(),
            )?;
            let js_handle = JsValue::from(handle);
            Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
            let js_claimer_user_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(claimer_user_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"claimerUserId".into(), &js_claimer_user_id)?;
            let js_claimer_human_handle = struct_human_handle_rs_to_js(claimer_human_handle)?;
            Reflect::set(
                &js_obj,
                &"claimerHumanHandle".into(),
                &js_claimer_human_handle,
            )?;
            let js_invitation_created_by =
                variant_invite_info_invitation_created_by_rs_to_js(invitation_created_by)?;
            Reflect::set(
                &js_obj,
                &"invitationCreatedBy".into(),
                &js_invitation_created_by,
            )?;
            let js_shamir_recovery_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(shamir_recovery_created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(
                &js_obj,
                &"shamirRecoveryCreatedOn".into(),
                &js_shamir_recovery_created_on,
            )?;
            let js_recipients = {
                // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                let js_array = Array::new_with_length(recipients.len() as u32);
                for (i, elem) in recipients.into_iter().enumerate() {
                    let js_elem = struct_shamir_recovery_recipient_rs_to_js(elem)?;
                    js_array.set(i as u32, js_elem);
                }
                js_array.into()
            };
            Reflect::set(&js_obj, &"recipients".into(), &js_recipients)?;
            let js_threshold = {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                let v = match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"threshold".into(), &js_threshold)?;
            let js_is_recoverable = is_recoverable.into();
            Reflect::set(&js_obj, &"isRecoverable".into(), &js_is_recoverable)?;
        }
        libparsec::AnyClaimRetrievedInfo::User {
            handle,
            claimer_email,
            created_by,
            administrators,
            preferred_greeter,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"AnyClaimRetrievedInfoUser".into())?;
            let js_handle = JsValue::from(handle);
            Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
            let js_claimer_email = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::EmailAddress| -> Result<_, &'static str> { Ok(x.to_string()) };
                match custom_to_rs_string(claimer_email) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"claimerEmail".into(), &js_claimer_email)?;
            let js_created_by = variant_invite_info_invitation_created_by_rs_to_js(created_by)?;
            Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
            let js_administrators = {
                // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                let js_array = Array::new_with_length(administrators.len() as u32);
                for (i, elem) in administrators.into_iter().enumerate() {
                    let js_elem = struct_user_greeting_administrator_rs_to_js(elem)?;
                    js_array.set(i as u32, js_elem);
                }
                js_array.into()
            };
            Reflect::set(&js_obj, &"administrators".into(), &js_administrators)?;
            let js_preferred_greeter = match preferred_greeter {
                Some(val) => struct_user_greeting_administrator_rs_to_js(val)?,
                None => JsValue::NULL,
            };
            Reflect::set(&js_obj, &"preferredGreeter".into(), &js_preferred_greeter)?;
        }
    }
    Ok(js_obj)
}

// ArchiveDeviceError

#[allow(dead_code)]
fn variant_archive_device_error_rs_to_js(
    rs_obj: libparsec::ArchiveDeviceError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ArchiveDeviceError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ArchiveDeviceErrorInternal".into())?;
        }
        libparsec::ArchiveDeviceError::StorageNotAvailable { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ArchiveDeviceErrorStorageNotAvailable".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// AvailableDeviceType

#[allow(dead_code)]
fn variant_available_device_type_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::AvailableDeviceType, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "AvailableDeviceTypeAccountVault" => Ok(libparsec::AvailableDeviceType::AccountVault {}),
        "AvailableDeviceTypeKeyring" => Ok(libparsec::AvailableDeviceType::Keyring {}),
        "AvailableDeviceTypePassword" => Ok(libparsec::AvailableDeviceType::Password {}),
        "AvailableDeviceTypeRecovery" => Ok(libparsec::AvailableDeviceType::Recovery {}),
        "AvailableDeviceTypeSmartcard" => Ok(libparsec::AvailableDeviceType::Smartcard {}),
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a AvailableDeviceType",
        ))),
    }
}

#[allow(dead_code)]
fn variant_available_device_type_rs_to_js(
    rs_obj: libparsec::AvailableDeviceType,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::AvailableDeviceType::AccountVault { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AvailableDeviceTypeAccountVault".into(),
            )?;
        }
        libparsec::AvailableDeviceType::Keyring { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"AvailableDeviceTypeKeyring".into())?;
        }
        libparsec::AvailableDeviceType::Password { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AvailableDeviceTypePassword".into(),
            )?;
        }
        libparsec::AvailableDeviceType::Recovery { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AvailableDeviceTypeRecovery".into(),
            )?;
        }
        libparsec::AvailableDeviceType::Smartcard { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"AvailableDeviceTypeSmartcard".into(),
            )?;
        }
    }
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"BootstrapOrganizationErrorAlreadyUsedToken".into(),
            )?;
        }
        libparsec::BootstrapOrganizationError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"BootstrapOrganizationErrorInternal".into(),
            )?;
        }
        libparsec::BootstrapOrganizationError::InvalidSequesterAuthorityVerifyKey { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"BootstrapOrganizationErrorInvalidSequesterAuthorityVerifyKey".into(),
            )?;
        }
        libparsec::BootstrapOrganizationError::InvalidToken { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"BootstrapOrganizationErrorInvalidToken".into(),
            )?;
        }
        libparsec::BootstrapOrganizationError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"BootstrapOrganizationErrorOffline".into(),
            )?;
        }
        libparsec::BootstrapOrganizationError::OrganizationExpired { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"BootstrapOrganizationErrorOrganizationExpired".into(),
            )?;
        }
        libparsec::BootstrapOrganizationError::SaveDeviceInvalidPath { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"BootstrapOrganizationErrorSaveDeviceInvalidPath".into(),
            )?;
        }
        libparsec::BootstrapOrganizationError::SaveDeviceRemoteOpaqueKeyUploadFailed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"BootstrapOrganizationErrorSaveDeviceRemoteOpaqueKeyUploadFailed".into(),
            )?;
        }
        libparsec::BootstrapOrganizationError::SaveDeviceStorageNotAvailable { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"BootstrapOrganizationErrorSaveDeviceStorageNotAvailable".into(),
            )?;
        }
        libparsec::BootstrapOrganizationError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"BootstrapOrganizationErrorTimestampOutOfBallpark".into(),
            )?;
            let js_server_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
            Reflect::set(&js_obj, &"tag".into(), &"CancelErrorInternal".into())?;
        }
        libparsec::CancelError::NotBound { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"CancelErrorNotBound".into())?;
        }
    }
    Ok(js_obj)
}

// ClaimFinalizeError

#[allow(dead_code)]
fn variant_claim_finalize_error_rs_to_js(
    rs_obj: libparsec::ClaimFinalizeError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClaimFinalizeError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClaimFinalizeErrorInternal".into())?;
        }
        libparsec::ClaimFinalizeError::InvalidPath { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimFinalizeErrorInvalidPath".into(),
            )?;
        }
        libparsec::ClaimFinalizeError::RemoteOpaqueKeyUploadFailed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimFinalizeErrorRemoteOpaqueKeyUploadFailed".into(),
            )?;
        }
        libparsec::ClaimFinalizeError::RemoteOpaqueKeyUploadOffline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimFinalizeErrorRemoteOpaqueKeyUploadOffline".into(),
            )?;
        }
        libparsec::ClaimFinalizeError::StorageNotAvailable { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimFinalizeErrorStorageNotAvailable".into(),
            )?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimInProgressErrorActiveUsersLimitReached".into(),
            )?;
        }
        libparsec::ClaimInProgressError::AlreadyUsedOrDeleted { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimInProgressErrorAlreadyUsedOrDeleted".into(),
            )?;
        }
        libparsec::ClaimInProgressError::Cancelled { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimInProgressErrorCancelled".into(),
            )?;
        }
        libparsec::ClaimInProgressError::CorruptedConfirmation { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimInProgressErrorCorruptedConfirmation".into(),
            )?;
        }
        libparsec::ClaimInProgressError::CorruptedSharedSecretKey { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimInProgressErrorCorruptedSharedSecretKey".into(),
            )?;
        }
        libparsec::ClaimInProgressError::GreeterNotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimInProgressErrorGreeterNotAllowed".into(),
            )?;
        }
        libparsec::ClaimInProgressError::GreetingAttemptCancelled {
            origin,
            reason,
            timestamp,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimInProgressErrorGreetingAttemptCancelled".into(),
            )?;
            let js_origin = JsValue::from_str(enum_greeter_or_claimer_rs_to_js(origin));
            Reflect::set(&js_obj, &"origin".into(), &js_origin)?;
            let js_reason =
                JsValue::from_str(enum_cancelled_greeting_attempt_reason_rs_to_js(reason));
            Reflect::set(&js_obj, &"reason".into(), &js_reason)?;
            let js_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"timestamp".into(), &js_timestamp)?;
        }
        libparsec::ClaimInProgressError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimInProgressErrorInternal".into(),
            )?;
        }
        libparsec::ClaimInProgressError::NotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimInProgressErrorNotFound".into(),
            )?;
        }
        libparsec::ClaimInProgressError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimInProgressErrorOffline".into(),
            )?;
        }
        libparsec::ClaimInProgressError::OrganizationExpired { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimInProgressErrorOrganizationExpired".into(),
            )?;
        }
        libparsec::ClaimInProgressError::PeerReset { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimInProgressErrorPeerReset".into(),
            )?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimerGreeterAbortOperationErrorInternal".into(),
            )?;
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
        libparsec::ClaimerRetrieveInfoError::AlreadyUsedOrDeleted { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimerRetrieveInfoErrorAlreadyUsedOrDeleted".into(),
            )?;
        }
        libparsec::ClaimerRetrieveInfoError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimerRetrieveInfoErrorInternal".into(),
            )?;
        }
        libparsec::ClaimerRetrieveInfoError::NotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimerRetrieveInfoErrorNotFound".into(),
            )?;
        }
        libparsec::ClaimerRetrieveInfoError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimerRetrieveInfoErrorOffline".into(),
            )?;
        }
        libparsec::ClaimerRetrieveInfoError::OrganizationExpired { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClaimerRetrieveInfoErrorOrganizationExpired".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientAcceptTosError

#[allow(dead_code)]
fn variant_client_accept_tos_error_rs_to_js(
    rs_obj: libparsec::ClientAcceptTosError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientAcceptTosError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientAcceptTosErrorInternal".into(),
            )?;
        }
        libparsec::ClientAcceptTosError::NoTos { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientAcceptTosErrorNoTos".into())?;
        }
        libparsec::ClientAcceptTosError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientAcceptTosErrorOffline".into(),
            )?;
        }
        libparsec::ClientAcceptTosError::TosMismatch { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientAcceptTosErrorTosMismatch".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientCancelInvitationError

#[allow(dead_code)]
fn variant_client_cancel_invitation_error_rs_to_js(
    rs_obj: libparsec::ClientCancelInvitationError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientCancelInvitationError::AlreadyCancelled { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientCancelInvitationErrorAlreadyCancelled".into(),
            )?;
        }
        libparsec::ClientCancelInvitationError::Completed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientCancelInvitationErrorCompleted".into(),
            )?;
        }
        libparsec::ClientCancelInvitationError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientCancelInvitationErrorInternal".into(),
            )?;
        }
        libparsec::ClientCancelInvitationError::NotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientCancelInvitationErrorNotAllowed".into(),
            )?;
        }
        libparsec::ClientCancelInvitationError::NotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientCancelInvitationErrorNotFound".into(),
            )?;
        }
        libparsec::ClientCancelInvitationError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientCancelInvitationErrorOffline".into(),
            )?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientCreateWorkspaceErrorInternal".into(),
            )?;
        }
        libparsec::ClientCreateWorkspaceError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientCreateWorkspaceErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientDeleteShamirRecoveryError

#[allow(dead_code)]
fn variant_client_delete_shamir_recovery_error_rs_to_js(
    rs_obj: libparsec::ClientDeleteShamirRecoveryError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientDeleteShamirRecoveryError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientDeleteShamirRecoveryErrorInternal".into(),
            )?;
        }
        libparsec::ClientDeleteShamirRecoveryError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientDeleteShamirRecoveryErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::ClientDeleteShamirRecoveryError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientDeleteShamirRecoveryErrorOffline".into(),
            )?;
        }
        libparsec::ClientDeleteShamirRecoveryError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientDeleteShamirRecoveryErrorStopped".into(),
            )?;
        }
        libparsec::ClientDeleteShamirRecoveryError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientDeleteShamirRecoveryErrorTimestampOutOfBallpark".into(),
            )?;
            let js_server_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
    }
    Ok(js_obj)
}

// ClientEvent

#[allow(dead_code)]
fn variant_client_event_js_to_rs(obj: JsValue) -> Result<libparsec::ClientEvent, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "ClientEventClientErrorResponse" => {
            let error_type = {
                let js_val = Reflect::get(&obj, &"errorType".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            Ok(libparsec::ClientEvent::ClientErrorResponse { error_type })
        }
        "ClientEventClientStarted" => {
            let device_id = {
                let js_val = Reflect::get(&obj, &"deviceId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            Ok(libparsec::ClientEvent::ClientStarted { device_id })
        }
        "ClientEventClientStopped" => {
            let device_id = {
                let js_val = Reflect::get(&obj, &"deviceId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            Ok(libparsec::ClientEvent::ClientStopped { device_id })
        }
        "ClientEventExpiredOrganization" => Ok(libparsec::ClientEvent::ExpiredOrganization {}),
        "ClientEventFrozenSelfUser" => Ok(libparsec::ClientEvent::FrozenSelfUser {}),
        "ClientEventGreetingAttemptCancelled" => {
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
                    })?
            };
            let greeting_attempt = {
                let js_val = Reflect::get(&obj, &"greetingAttempt".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string =
                            |s: String| -> Result<libparsec::GreetingAttemptID, _> {
                                libparsec::GreetingAttemptID::from_hex(s.as_str())
                                    .map_err(|e| e.to_string())
                            };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            Ok(libparsec::ClientEvent::GreetingAttemptCancelled {
                token,
                greeting_attempt,
            })
        }
        "ClientEventGreetingAttemptJoined" => {
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
                    })?
            };
            let greeting_attempt = {
                let js_val = Reflect::get(&obj, &"greetingAttempt".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string =
                            |s: String| -> Result<libparsec::GreetingAttemptID, _> {
                                libparsec::GreetingAttemptID::from_hex(s.as_str())
                                    .map_err(|e| e.to_string())
                            };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            Ok(libparsec::ClientEvent::GreetingAttemptJoined {
                token,
                greeting_attempt,
            })
        }
        "ClientEventGreetingAttemptReady" => {
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
                    })?
            };
            let greeting_attempt = {
                let js_val = Reflect::get(&obj, &"greetingAttempt".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string =
                            |s: String| -> Result<libparsec::GreetingAttemptID, _> {
                                libparsec::GreetingAttemptID::from_hex(s.as_str())
                                    .map_err(|e| e.to_string())
                            };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            Ok(libparsec::ClientEvent::GreetingAttemptReady {
                token,
                greeting_attempt,
            })
        }
        "ClientEventIncompatibleServer" => {
            let api_version = {
                let js_val = Reflect::get(&obj, &"apiVersion".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::ApiVersion::try_from(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let supported_api_version = {
                let js_val = Reflect::get(&obj, &"supportedApiVersion".into())?;
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
                                let custom_from_rs_string = |s: String| -> Result<_, String> {
                                    libparsec::ApiVersion::try_from(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })?;
                        converted.push(x_converted);
                    }
                    converted
                }
            };
            Ok(libparsec::ClientEvent::IncompatibleServer {
                api_version,
                supported_api_version,
            })
        }
        "ClientEventInvalidCertificate" => {
            let detail = {
                let js_val = Reflect::get(&obj, &"detail".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            Ok(libparsec::ClientEvent::InvalidCertificate { detail })
        }
        "ClientEventInvitationAlreadyUsedOrDeleted" => {
            Ok(libparsec::ClientEvent::InvitationAlreadyUsedOrDeleted {})
        }
        "ClientEventInvitationChanged" => {
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
                    })?
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
            Ok(libparsec::ClientEvent::InvitationChanged { token, status })
        }
        "ClientEventMustAcceptTos" => Ok(libparsec::ClientEvent::MustAcceptTos {}),
        "ClientEventOffline" => Ok(libparsec::ClientEvent::Offline {}),
        "ClientEventOnline" => Ok(libparsec::ClientEvent::Online {}),
        "ClientEventOrganizationNotFound" => Ok(libparsec::ClientEvent::OrganizationNotFound {}),
        "ClientEventPing" => {
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
        "ClientEventRevokedSelfUser" => Ok(libparsec::ClientEvent::RevokedSelfUser {}),
        "ClientEventServerConfigChanged" => Ok(libparsec::ClientEvent::ServerConfigChanged {}),
        "ClientEventServerInvalidResponseContent" => {
            let protocol_decode_error = {
                let js_val = Reflect::get(&obj, &"protocolDecodeError".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            Ok(libparsec::ClientEvent::ServerInvalidResponseContent {
                protocol_decode_error,
            })
        }
        "ClientEventServerInvalidResponseStatus" => {
            let status_code = {
                let js_val = Reflect::get(&obj, &"statusCode".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            Ok(libparsec::ClientEvent::ServerInvalidResponseStatus { status_code })
        }
        "ClientEventTooMuchDriftWithServerClock" => {
            let server_timestamp = {
                let js_val = Reflect::get(&obj, &"serverTimestamp".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let client_timestamp = {
                let js_val = Reflect::get(&obj, &"clientTimestamp".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let ballpark_client_early_offset = {
                let js_val = Reflect::get(&obj, &"ballparkClientEarlyOffset".into())?;
                js_val.dyn_into::<Number>()?.value_of()
            };
            let ballpark_client_late_offset = {
                let js_val = Reflect::get(&obj, &"ballparkClientLateOffset".into())?;
                js_val.dyn_into::<Number>()?.value_of()
            };
            Ok(libparsec::ClientEvent::TooMuchDriftWithServerClock {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            })
        }
        "ClientEventWebClientNotAllowedByOrganization" => {
            Ok(libparsec::ClientEvent::WebClientNotAllowedByOrganization {})
        }
        "ClientEventWorkspaceLocallyCreated" => {
            Ok(libparsec::ClientEvent::WorkspaceLocallyCreated {})
        }
        "ClientEventWorkspaceOpsInboundSyncDone" => {
            let realm_id = {
                let js_val = Reflect::get(&obj, &"realmId".into())?;
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
                    })?
            };
            let entry_id = {
                let js_val = Reflect::get(&obj, &"entryId".into())?;
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
                    })?
            };
            Ok(libparsec::ClientEvent::WorkspaceOpsInboundSyncDone { realm_id, entry_id })
        }
        "ClientEventWorkspaceOpsOutboundSyncAborted" => {
            let realm_id = {
                let js_val = Reflect::get(&obj, &"realmId".into())?;
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
                    })?
            };
            let entry_id = {
                let js_val = Reflect::get(&obj, &"entryId".into())?;
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
                    })?
            };
            Ok(libparsec::ClientEvent::WorkspaceOpsOutboundSyncAborted { realm_id, entry_id })
        }
        "ClientEventWorkspaceOpsOutboundSyncDone" => {
            let realm_id = {
                let js_val = Reflect::get(&obj, &"realmId".into())?;
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
                    })?
            };
            let entry_id = {
                let js_val = Reflect::get(&obj, &"entryId".into())?;
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
                    })?
            };
            Ok(libparsec::ClientEvent::WorkspaceOpsOutboundSyncDone { realm_id, entry_id })
        }
        "ClientEventWorkspaceOpsOutboundSyncProgress" => {
            let realm_id = {
                let js_val = Reflect::get(&obj, &"realmId".into())?;
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
                    })?
            };
            let entry_id = {
                let js_val = Reflect::get(&obj, &"entryId".into())?;
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
                    })?
            };
            let blocks = {
                let js_val = Reflect::get(&obj, &"blocks".into())?;
                {
                    let v = u64::try_from(js_val)
                        .map_err(|_| TypeError::new("Not a BigInt representing an u64 number"))?;
                    v
                }
            };
            let block_index = {
                let js_val = Reflect::get(&obj, &"blockIndex".into())?;
                {
                    let v = u64::try_from(js_val)
                        .map_err(|_| TypeError::new("Not a BigInt representing an u64 number"))?;
                    v
                }
            };
            let blocksize = {
                let js_val = Reflect::get(&obj, &"blocksize".into())?;
                {
                    let v = u64::try_from(js_val)
                        .map_err(|_| TypeError::new("Not a BigInt representing an u64 number"))?;
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
                let js_val = Reflect::get(&obj, &"realmId".into())?;
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
                    })?
            };
            let entry_id = {
                let js_val = Reflect::get(&obj, &"entryId".into())?;
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
                    })?
            };
            Ok(libparsec::ClientEvent::WorkspaceOpsOutboundSyncStarted { realm_id, entry_id })
        }
        "ClientEventWorkspaceWatchedEntryChanged" => {
            let realm_id = {
                let js_val = Reflect::get(&obj, &"realmId".into())?;
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
                    })?
            };
            let entry_id = {
                let js_val = Reflect::get(&obj, &"entryId".into())?;
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
                    })?
            };
            Ok(libparsec::ClientEvent::WorkspaceWatchedEntryChanged { realm_id, entry_id })
        }
        "ClientEventWorkspacesSelfListChanged" => {
            Ok(libparsec::ClientEvent::WorkspacesSelfListChanged {})
        }
        _ => Err(JsValue::from(TypeError::new("Object is not a ClientEvent"))),
    }
}

#[allow(dead_code)]
fn variant_client_event_rs_to_js(rs_obj: libparsec::ClientEvent) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::ClientEvent::ClientErrorResponse { error_type, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventClientErrorResponse".into(),
            )?;
            let js_error_type = error_type.into();
            Reflect::set(&js_obj, &"errorType".into(), &js_error_type)?;
        }
        libparsec::ClientEvent::ClientStarted { device_id, .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientEventClientStarted".into())?;
            let js_device_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(device_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"deviceId".into(), &js_device_id)?;
        }
        libparsec::ClientEvent::ClientStopped { device_id, .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientEventClientStopped".into())?;
            let js_device_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(device_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"deviceId".into(), &js_device_id)?;
        }
        libparsec::ClientEvent::ExpiredOrganization { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventExpiredOrganization".into(),
            )?;
        }
        libparsec::ClientEvent::FrozenSelfUser { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientEventFrozenSelfUser".into())?;
        }
        libparsec::ClientEvent::GreetingAttemptCancelled {
            token,
            greeting_attempt,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventGreetingAttemptCancelled".into(),
            )?;
            let js_token = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
            let js_greeting_attempt = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::GreetingAttemptID| -> Result<String, &'static str> {
                        Ok(x.hex())
                    };
                match custom_to_rs_string(greeting_attempt) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"greetingAttempt".into(), &js_greeting_attempt)?;
        }
        libparsec::ClientEvent::GreetingAttemptJoined {
            token,
            greeting_attempt,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventGreetingAttemptJoined".into(),
            )?;
            let js_token = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
            let js_greeting_attempt = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::GreetingAttemptID| -> Result<String, &'static str> {
                        Ok(x.hex())
                    };
                match custom_to_rs_string(greeting_attempt) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"greetingAttempt".into(), &js_greeting_attempt)?;
        }
        libparsec::ClientEvent::GreetingAttemptReady {
            token,
            greeting_attempt,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventGreetingAttemptReady".into(),
            )?;
            let js_token = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
            let js_greeting_attempt = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::GreetingAttemptID| -> Result<String, &'static str> {
                        Ok(x.hex())
                    };
                match custom_to_rs_string(greeting_attempt) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"greetingAttempt".into(), &js_greeting_attempt)?;
        }
        libparsec::ClientEvent::IncompatibleServer {
            api_version,
            supported_api_version,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventIncompatibleServer".into(),
            )?;
            let js_api_version = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::ApiVersion| -> Result<String, &'static str> {
                        Ok(x.to_string())
                    };
                match custom_to_rs_string(api_version) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"apiVersion".into(), &js_api_version)?;
            let js_supported_api_version = {
                // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                let js_array = Array::new_with_length(supported_api_version.len() as u32);
                for (i, elem) in supported_api_version.into_iter().enumerate() {
                    let js_elem = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::ApiVersion| -> Result<String, &'static str> {
                                Ok(x.to_string())
                            };
                        match custom_to_rs_string(elem) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    js_array.set(i as u32, js_elem);
                }
                js_array.into()
            };
            Reflect::set(
                &js_obj,
                &"supportedApiVersion".into(),
                &js_supported_api_version,
            )?;
        }
        libparsec::ClientEvent::InvalidCertificate { detail, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventInvalidCertificate".into(),
            )?;
            let js_detail = detail.into();
            Reflect::set(&js_obj, &"detail".into(), &js_detail)?;
        }
        libparsec::ClientEvent::InvitationAlreadyUsedOrDeleted { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventInvitationAlreadyUsedOrDeleted".into(),
            )?;
        }
        libparsec::ClientEvent::InvitationChanged { token, status, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventInvitationChanged".into(),
            )?;
            let js_token = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
            let js_status = JsValue::from_str(enum_invitation_status_rs_to_js(status));
            Reflect::set(&js_obj, &"status".into(), &js_status)?;
        }
        libparsec::ClientEvent::MustAcceptTos { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientEventMustAcceptTos".into())?;
        }
        libparsec::ClientEvent::Offline { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientEventOffline".into())?;
        }
        libparsec::ClientEvent::Online { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientEventOnline".into())?;
        }
        libparsec::ClientEvent::OrganizationNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventOrganizationNotFound".into(),
            )?;
        }
        libparsec::ClientEvent::Ping { ping, .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientEventPing".into())?;
            let js_ping = ping.into();
            Reflect::set(&js_obj, &"ping".into(), &js_ping)?;
        }
        libparsec::ClientEvent::RevokedSelfUser { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientEventRevokedSelfUser".into())?;
        }
        libparsec::ClientEvent::ServerConfigChanged { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventServerConfigChanged".into(),
            )?;
        }
        libparsec::ClientEvent::ServerInvalidResponseContent {
            protocol_decode_error,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventServerInvalidResponseContent".into(),
            )?;
            let js_protocol_decode_error = protocol_decode_error.into();
            Reflect::set(
                &js_obj,
                &"protocolDecodeError".into(),
                &js_protocol_decode_error,
            )?;
        }
        libparsec::ClientEvent::ServerInvalidResponseStatus { status_code, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventServerInvalidResponseStatus".into(),
            )?;
            let js_status_code = status_code.into();
            Reflect::set(&js_obj, &"statusCode".into(), &js_status_code)?;
        }
        libparsec::ClientEvent::TooMuchDriftWithServerClock {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventTooMuchDriftWithServerClock".into(),
            )?;
            let js_server_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
        libparsec::ClientEvent::WebClientNotAllowedByOrganization { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventWebClientNotAllowedByOrganization".into(),
            )?;
        }
        libparsec::ClientEvent::WorkspaceLocallyCreated { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventWorkspaceLocallyCreated".into(),
            )?;
        }
        libparsec::ClientEvent::WorkspaceOpsInboundSyncDone {
            realm_id, entry_id, ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventWorkspaceOpsInboundSyncDone".into(),
            )?;
            let js_realm_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(realm_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"realmId".into(), &js_realm_id)?;
            let js_entry_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(entry_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"entryId".into(), &js_entry_id)?;
        }
        libparsec::ClientEvent::WorkspaceOpsOutboundSyncAborted {
            realm_id, entry_id, ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventWorkspaceOpsOutboundSyncAborted".into(),
            )?;
            let js_realm_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(realm_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"realmId".into(), &js_realm_id)?;
            let js_entry_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(entry_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"entryId".into(), &js_entry_id)?;
        }
        libparsec::ClientEvent::WorkspaceOpsOutboundSyncDone {
            realm_id, entry_id, ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventWorkspaceOpsOutboundSyncDone".into(),
            )?;
            let js_realm_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(realm_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"realmId".into(), &js_realm_id)?;
            let js_entry_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(entry_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"entryId".into(), &js_entry_id)?;
        }
        libparsec::ClientEvent::WorkspaceOpsOutboundSyncProgress {
            realm_id,
            entry_id,
            blocks,
            block_index,
            blocksize,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventWorkspaceOpsOutboundSyncProgress".into(),
            )?;
            let js_realm_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(realm_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"realmId".into(), &js_realm_id)?;
            let js_entry_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(entry_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"entryId".into(), &js_entry_id)?;
            let js_blocks = JsValue::from(blocks);
            Reflect::set(&js_obj, &"blocks".into(), &js_blocks)?;
            let js_block_index = JsValue::from(block_index);
            Reflect::set(&js_obj, &"blockIndex".into(), &js_block_index)?;
            let js_blocksize = JsValue::from(blocksize);
            Reflect::set(&js_obj, &"blocksize".into(), &js_blocksize)?;
        }
        libparsec::ClientEvent::WorkspaceOpsOutboundSyncStarted {
            realm_id, entry_id, ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventWorkspaceOpsOutboundSyncStarted".into(),
            )?;
            let js_realm_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(realm_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"realmId".into(), &js_realm_id)?;
            let js_entry_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(entry_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"entryId".into(), &js_entry_id)?;
        }
        libparsec::ClientEvent::WorkspaceWatchedEntryChanged {
            realm_id, entry_id, ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventWorkspaceWatchedEntryChanged".into(),
            )?;
            let js_realm_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(realm_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"realmId".into(), &js_realm_id)?;
            let js_entry_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(entry_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"entryId".into(), &js_entry_id)?;
        }
        libparsec::ClientEvent::WorkspacesSelfListChanged { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientEventWorkspacesSelfListChanged".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientExportRecoveryDeviceError

#[allow(dead_code)]
fn variant_client_export_recovery_device_error_rs_to_js(
    rs_obj: libparsec::ClientExportRecoveryDeviceError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientExportRecoveryDeviceError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientExportRecoveryDeviceErrorInternal".into(),
            )?;
        }
        libparsec::ClientExportRecoveryDeviceError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientExportRecoveryDeviceErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::ClientExportRecoveryDeviceError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientExportRecoveryDeviceErrorOffline".into(),
            )?;
        }
        libparsec::ClientExportRecoveryDeviceError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientExportRecoveryDeviceErrorStopped".into(),
            )?;
        }
        libparsec::ClientExportRecoveryDeviceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientExportRecoveryDeviceErrorTimestampOutOfBallpark".into(),
            )?;
            let js_server_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
    }
    Ok(js_obj)
}

// ClientForgetAllCertificatesError

#[allow(dead_code)]
fn variant_client_forget_all_certificates_error_rs_to_js(
    rs_obj: libparsec::ClientForgetAllCertificatesError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientForgetAllCertificatesError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientForgetAllCertificatesErrorInternal".into(),
            )?;
        }
        libparsec::ClientForgetAllCertificatesError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientForgetAllCertificatesErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientGetOrganizationBootstrapDateError

#[allow(dead_code)]
fn variant_client_get_organization_bootstrap_date_error_rs_to_js(
    rs_obj: libparsec::ClientGetOrganizationBootstrapDateError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientGetOrganizationBootstrapDateError::BootstrapDateNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientGetOrganizationBootstrapDateErrorBootstrapDateNotFound".into(),
            )?;
        }
        libparsec::ClientGetOrganizationBootstrapDateError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientGetOrganizationBootstrapDateErrorInternal".into(),
            )?;
        }
        libparsec::ClientGetOrganizationBootstrapDateError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientGetOrganizationBootstrapDateErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::ClientGetOrganizationBootstrapDateError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientGetOrganizationBootstrapDateErrorOffline".into(),
            )?;
        }
        libparsec::ClientGetOrganizationBootstrapDateError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientGetOrganizationBootstrapDateErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientGetSelfShamirRecoveryError

#[allow(dead_code)]
fn variant_client_get_self_shamir_recovery_error_rs_to_js(
    rs_obj: libparsec::ClientGetSelfShamirRecoveryError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientGetSelfShamirRecoveryError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientGetSelfShamirRecoveryErrorInternal".into(),
            )?;
        }
        libparsec::ClientGetSelfShamirRecoveryError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientGetSelfShamirRecoveryErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientGetTosError

#[allow(dead_code)]
fn variant_client_get_tos_error_rs_to_js(
    rs_obj: libparsec::ClientGetTosError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientGetTosError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientGetTosErrorInternal".into())?;
        }
        libparsec::ClientGetTosError::NoTos { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientGetTosErrorNoTos".into())?;
        }
        libparsec::ClientGetTosError::Offline { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientGetTosErrorOffline".into())?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientGetUserDeviceErrorInternal".into(),
            )?;
        }
        libparsec::ClientGetUserDeviceError::NonExisting { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientGetUserDeviceErrorNonExisting".into(),
            )?;
        }
        libparsec::ClientGetUserDeviceError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientGetUserDeviceErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientGetUserInfoError

#[allow(dead_code)]
fn variant_client_get_user_info_error_rs_to_js(
    rs_obj: libparsec::ClientGetUserInfoError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientGetUserInfoError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientGetUserInfoErrorInternal".into(),
            )?;
        }
        libparsec::ClientGetUserInfoError::NonExisting { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientGetUserInfoErrorNonExisting".into(),
            )?;
        }
        libparsec::ClientGetUserInfoError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientGetUserInfoErrorStopped".into(),
            )?;
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
            Reflect::set(&js_obj, &"tag".into(), &"ClientInfoErrorInternal".into())?;
        }
        libparsec::ClientInfoError::Stopped { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientInfoErrorStopped".into())?;
        }
    }
    Ok(js_obj)
}

// ClientListFrozenUsersError

#[allow(dead_code)]
fn variant_client_list_frozen_users_error_rs_to_js(
    rs_obj: libparsec::ClientListFrozenUsersError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientListFrozenUsersError::AuthorNotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientListFrozenUsersErrorAuthorNotAllowed".into(),
            )?;
        }
        libparsec::ClientListFrozenUsersError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientListFrozenUsersErrorInternal".into(),
            )?;
        }
        libparsec::ClientListFrozenUsersError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientListFrozenUsersErrorOffline".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientListShamirRecoveriesForOthersError

#[allow(dead_code)]
fn variant_client_list_shamir_recoveries_for_others_error_rs_to_js(
    rs_obj: libparsec::ClientListShamirRecoveriesForOthersError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientListShamirRecoveriesForOthersError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientListShamirRecoveriesForOthersErrorInternal".into(),
            )?;
        }
        libparsec::ClientListShamirRecoveriesForOthersError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientListShamirRecoveriesForOthersErrorStopped".into(),
            )?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientListUserDevicesErrorInternal".into(),
            )?;
        }
        libparsec::ClientListUserDevicesError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientListUserDevicesErrorStopped".into(),
            )?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientListUsersErrorInternal".into(),
            )?;
        }
        libparsec::ClientListUsersError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientListUsersErrorStopped".into(),
            )?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientListWorkspaceUsersErrorInternal".into(),
            )?;
        }
        libparsec::ClientListWorkspaceUsersError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientListWorkspaceUsersErrorStopped".into(),
            )?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientListWorkspacesErrorInternal".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientNewDeviceInvitationError

#[allow(dead_code)]
fn variant_client_new_device_invitation_error_rs_to_js(
    rs_obj: libparsec::ClientNewDeviceInvitationError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientNewDeviceInvitationError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientNewDeviceInvitationErrorInternal".into(),
            )?;
        }
        libparsec::ClientNewDeviceInvitationError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientNewDeviceInvitationErrorOffline".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientNewShamirRecoveryInvitationError

#[allow(dead_code)]
fn variant_client_new_shamir_recovery_invitation_error_rs_to_js(
    rs_obj: libparsec::ClientNewShamirRecoveryInvitationError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientNewShamirRecoveryInvitationError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientNewShamirRecoveryInvitationErrorInternal".into(),
            )?;
        }
        libparsec::ClientNewShamirRecoveryInvitationError::NotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientNewShamirRecoveryInvitationErrorNotAllowed".into(),
            )?;
        }
        libparsec::ClientNewShamirRecoveryInvitationError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientNewShamirRecoveryInvitationErrorOffline".into(),
            )?;
        }
        libparsec::ClientNewShamirRecoveryInvitationError::UserNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientNewShamirRecoveryInvitationErrorUserNotFound".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientNewUserInvitationError

#[allow(dead_code)]
fn variant_client_new_user_invitation_error_rs_to_js(
    rs_obj: libparsec::ClientNewUserInvitationError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientNewUserInvitationError::AlreadyMember { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientNewUserInvitationErrorAlreadyMember".into(),
            )?;
        }
        libparsec::ClientNewUserInvitationError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientNewUserInvitationErrorInternal".into(),
            )?;
        }
        libparsec::ClientNewUserInvitationError::NotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientNewUserInvitationErrorNotAllowed".into(),
            )?;
        }
        libparsec::ClientNewUserInvitationError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientNewUserInvitationErrorOffline".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientOrganizationInfoError

#[allow(dead_code)]
fn variant_client_organization_info_error_rs_to_js(
    rs_obj: libparsec::ClientOrganizationInfoError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientOrganizationInfoError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientOrganizationInfoErrorInternal".into(),
            )?;
        }
        libparsec::ClientOrganizationInfoError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientOrganizationInfoErrorOffline".into(),
            )?;
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
        libparsec::ClientRenameWorkspaceError::AuthorNotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRenameWorkspaceErrorAuthorNotAllowed".into(),
            )?;
        }
        libparsec::ClientRenameWorkspaceError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRenameWorkspaceErrorInternal".into(),
            )?;
        }
        libparsec::ClientRenameWorkspaceError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRenameWorkspaceErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::ClientRenameWorkspaceError::InvalidEncryptedRealmName { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRenameWorkspaceErrorInvalidEncryptedRealmName".into(),
            )?;
        }
        libparsec::ClientRenameWorkspaceError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRenameWorkspaceErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::ClientRenameWorkspaceError::NoKey { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRenameWorkspaceErrorNoKey".into(),
            )?;
        }
        libparsec::ClientRenameWorkspaceError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRenameWorkspaceErrorOffline".into(),
            )?;
        }
        libparsec::ClientRenameWorkspaceError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRenameWorkspaceErrorStopped".into(),
            )?;
        }
        libparsec::ClientRenameWorkspaceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRenameWorkspaceErrorTimestampOutOfBallpark".into(),
            )?;
            let js_server_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
        libparsec::ClientRenameWorkspaceError::WorkspaceNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRenameWorkspaceErrorWorkspaceNotFound".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientRevokeUserError

#[allow(dead_code)]
fn variant_client_revoke_user_error_rs_to_js(
    rs_obj: libparsec::ClientRevokeUserError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientRevokeUserError::AuthorNotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRevokeUserErrorAuthorNotAllowed".into(),
            )?;
        }
        libparsec::ClientRevokeUserError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRevokeUserErrorInternal".into(),
            )?;
        }
        libparsec::ClientRevokeUserError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRevokeUserErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::ClientRevokeUserError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRevokeUserErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::ClientRevokeUserError::NoKey { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientRevokeUserErrorNoKey".into())?;
        }
        libparsec::ClientRevokeUserError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRevokeUserErrorOffline".into(),
            )?;
        }
        libparsec::ClientRevokeUserError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRevokeUserErrorStopped".into(),
            )?;
        }
        libparsec::ClientRevokeUserError::TimestampOutOfBallpark { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRevokeUserErrorTimestampOutOfBallpark".into(),
            )?;
        }
        libparsec::ClientRevokeUserError::UserIsSelf { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRevokeUserErrorUserIsSelf".into(),
            )?;
        }
        libparsec::ClientRevokeUserError::UserNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientRevokeUserErrorUserNotFound".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientSetupShamirRecoveryError

#[allow(dead_code)]
fn variant_client_setup_shamir_recovery_error_rs_to_js(
    rs_obj: libparsec::ClientSetupShamirRecoveryError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientSetupShamirRecoveryError::AuthorAmongRecipients { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientSetupShamirRecoveryErrorAuthorAmongRecipients".into(),
            )?;
        }
        libparsec::ClientSetupShamirRecoveryError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientSetupShamirRecoveryErrorInternal".into(),
            )?;
        }
        libparsec::ClientSetupShamirRecoveryError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientSetupShamirRecoveryErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::ClientSetupShamirRecoveryError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientSetupShamirRecoveryErrorOffline".into(),
            )?;
        }
        libparsec::ClientSetupShamirRecoveryError::RecipientNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientSetupShamirRecoveryErrorRecipientNotFound".into(),
            )?;
        }
        libparsec::ClientSetupShamirRecoveryError::RecipientRevoked { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientSetupShamirRecoveryErrorRecipientRevoked".into(),
            )?;
        }
        libparsec::ClientSetupShamirRecoveryError::ShamirRecoveryAlreadyExists { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientSetupShamirRecoveryErrorShamirRecoveryAlreadyExists".into(),
            )?;
        }
        libparsec::ClientSetupShamirRecoveryError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientSetupShamirRecoveryErrorStopped".into(),
            )?;
        }
        libparsec::ClientSetupShamirRecoveryError::ThresholdBiggerThanSumOfShares { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientSetupShamirRecoveryErrorThresholdBiggerThanSumOfShares".into(),
            )?;
        }
        libparsec::ClientSetupShamirRecoveryError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientSetupShamirRecoveryErrorTimestampOutOfBallpark".into(),
            )?;
            let js_server_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
        libparsec::ClientSetupShamirRecoveryError::TooManyShares { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientSetupShamirRecoveryErrorTooManyShares".into(),
            )?;
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
        libparsec::ClientShareWorkspaceError::AuthorNotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientShareWorkspaceErrorAuthorNotAllowed".into(),
            )?;
        }
        libparsec::ClientShareWorkspaceError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientShareWorkspaceErrorInternal".into(),
            )?;
        }
        libparsec::ClientShareWorkspaceError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientShareWorkspaceErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::ClientShareWorkspaceError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientShareWorkspaceErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::ClientShareWorkspaceError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientShareWorkspaceErrorOffline".into(),
            )?;
        }
        libparsec::ClientShareWorkspaceError::RecipientIsSelf { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientShareWorkspaceErrorRecipientIsSelf".into(),
            )?;
        }
        libparsec::ClientShareWorkspaceError::RecipientNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientShareWorkspaceErrorRecipientNotFound".into(),
            )?;
        }
        libparsec::ClientShareWorkspaceError::RecipientRevoked { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientShareWorkspaceErrorRecipientRevoked".into(),
            )?;
        }
        libparsec::ClientShareWorkspaceError::RoleIncompatibleWithOutsider { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientShareWorkspaceErrorRoleIncompatibleWithOutsider".into(),
            )?;
        }
        libparsec::ClientShareWorkspaceError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientShareWorkspaceErrorStopped".into(),
            )?;
        }
        libparsec::ClientShareWorkspaceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientShareWorkspaceErrorTimestampOutOfBallpark".into(),
            )?;
            let js_server_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
        libparsec::ClientShareWorkspaceError::WorkspaceNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientShareWorkspaceErrorWorkspaceNotFound".into(),
            )?;
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
        libparsec::ClientStartError::DeviceUsedByAnotherProcess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartErrorDeviceUsedByAnotherProcess".into(),
            )?;
        }
        libparsec::ClientStartError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"ClientStartErrorInternal".into())?;
        }
        libparsec::ClientStartError::LoadDeviceDecryptionFailed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartErrorLoadDeviceDecryptionFailed".into(),
            )?;
        }
        libparsec::ClientStartError::LoadDeviceInvalidData { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartErrorLoadDeviceInvalidData".into(),
            )?;
        }
        libparsec::ClientStartError::LoadDeviceInvalidPath { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartErrorLoadDeviceInvalidPath".into(),
            )?;
        }
        libparsec::ClientStartError::LoadDeviceRemoteOpaqueKeyFetchFailed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartErrorLoadDeviceRemoteOpaqueKeyFetchFailed".into(),
            )?;
        }
        libparsec::ClientStartError::LoadDeviceRemoteOpaqueKeyFetchOffline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartErrorLoadDeviceRemoteOpaqueKeyFetchOffline".into(),
            )?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartInvitationGreetErrorInternal".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ClientStartShamirRecoveryInvitationGreetError

#[allow(dead_code)]
fn variant_client_start_shamir_recovery_invitation_greet_error_rs_to_js(
    rs_obj: libparsec::ClientStartShamirRecoveryInvitationGreetError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientStartShamirRecoveryInvitationGreetError::CorruptedShareData { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartShamirRecoveryInvitationGreetErrorCorruptedShareData".into(),
            )?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartShamirRecoveryInvitationGreetErrorInternal".into(),
            )?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartShamirRecoveryInvitationGreetErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::InvitationNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartShamirRecoveryInvitationGreetErrorInvitationNotFound".into(),
            )?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartShamirRecoveryInvitationGreetErrorOffline".into(),
            )?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::ShamirRecoveryDeleted {
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryDeleted".into(),
            )?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::ShamirRecoveryNotFound {
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryNotFound".into(),
            )?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::ShamirRecoveryUnusable {
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryUnusable".into(),
            )?;
        }
        libparsec::ClientStartShamirRecoveryInvitationGreetError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartShamirRecoveryInvitationGreetErrorStopped".into(),
            )?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartWorkspaceErrorInternal".into(),
            )?;
        }
        libparsec::ClientStartWorkspaceError::WorkspaceNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientStartWorkspaceErrorWorkspaceNotFound".into(),
            )?;
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
            Reflect::set(&js_obj, &"tag".into(), &"ClientStopErrorInternal".into())?;
        }
    }
    Ok(js_obj)
}

// ClientUserUpdateProfileError

#[allow(dead_code)]
fn variant_client_user_update_profile_error_rs_to_js(
    rs_obj: libparsec::ClientUserUpdateProfileError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ClientUserUpdateProfileError::AuthorNotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientUserUpdateProfileErrorAuthorNotAllowed".into(),
            )?;
        }
        libparsec::ClientUserUpdateProfileError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientUserUpdateProfileErrorInternal".into(),
            )?;
        }
        libparsec::ClientUserUpdateProfileError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientUserUpdateProfileErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::ClientUserUpdateProfileError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientUserUpdateProfileErrorOffline".into(),
            )?;
        }
        libparsec::ClientUserUpdateProfileError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientUserUpdateProfileErrorStopped".into(),
            )?;
        }
        libparsec::ClientUserUpdateProfileError::TimestampOutOfBallpark { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientUserUpdateProfileErrorTimestampOutOfBallpark".into(),
            )?;
        }
        libparsec::ClientUserUpdateProfileError::UserIsSelf { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientUserUpdateProfileErrorUserIsSelf".into(),
            )?;
        }
        libparsec::ClientUserUpdateProfileError::UserNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientUserUpdateProfileErrorUserNotFound".into(),
            )?;
        }
        libparsec::ClientUserUpdateProfileError::UserRevoked { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ClientUserUpdateProfileErrorUserRevoked".into(),
            )?;
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
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "DeviceAccessStrategyAccountVault" => {
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
                    })?
            };
            let account_handle = {
                let js_val = Reflect::get(&obj, &"accountHandle".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u32 number")));
                    }
                    let v = v as u32;
                    v
                }
            };
            Ok(libparsec::DeviceAccessStrategy::AccountVault {
                key_file,
                account_handle,
            })
        }
        "DeviceAccessStrategyKeyring" => {
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
                    })?
            };
            Ok(libparsec::DeviceAccessStrategy::Keyring { key_file })
        }
        "DeviceAccessStrategyPassword" => {
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
                    })?
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
                    })?
            };
            Ok(libparsec::DeviceAccessStrategy::Password { password, key_file })
        }
        "DeviceAccessStrategySmartcard" => {
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
                    })?
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
        libparsec::DeviceAccessStrategy::AccountVault {
            key_file,
            account_handle,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"DeviceAccessStrategyAccountVault".into(),
            )?;
            let js_key_file = JsValue::from_str({
                let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                };
                match custom_to_rs_string(key_file) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"keyFile".into(), &js_key_file)?;
            let js_account_handle = JsValue::from(account_handle);
            Reflect::set(&js_obj, &"accountHandle".into(), &js_account_handle)?;
        }
        libparsec::DeviceAccessStrategy::Keyring { key_file, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"DeviceAccessStrategyKeyring".into(),
            )?;
            let js_key_file = JsValue::from_str({
                let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                };
                match custom_to_rs_string(key_file) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"keyFile".into(), &js_key_file)?;
        }
        libparsec::DeviceAccessStrategy::Password {
            password, key_file, ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"DeviceAccessStrategyPassword".into(),
            )?;
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
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"keyFile".into(), &js_key_file)?;
        }
        libparsec::DeviceAccessStrategy::Smartcard { key_file, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"DeviceAccessStrategySmartcard".into(),
            )?;
            let js_key_file = JsValue::from_str({
                let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                };
                match custom_to_rs_string(key_file) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
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
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "DeviceSaveStrategyAccountVault" => {
            let account_handle = {
                let js_val = Reflect::get(&obj, &"accountHandle".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u32 number")));
                    }
                    let v = v as u32;
                    v
                }
            };
            Ok(libparsec::DeviceSaveStrategy::AccountVault { account_handle })
        }
        "DeviceSaveStrategyKeyring" => Ok(libparsec::DeviceSaveStrategy::Keyring {}),
        "DeviceSaveStrategyPassword" => {
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
                    })?
            };
            Ok(libparsec::DeviceSaveStrategy::Password { password })
        }
        "DeviceSaveStrategySmartcard" => {
            let certificate_reference = {
                let js_val = Reflect::get(&obj, &"certificateReference".into())?;
                struct_x509_certificate_reference_js_to_rs(js_val)?
            };
            Ok(libparsec::DeviceSaveStrategy::Smartcard {
                certificate_reference,
            })
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
        libparsec::DeviceSaveStrategy::AccountVault { account_handle, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"DeviceSaveStrategyAccountVault".into(),
            )?;
            let js_account_handle = JsValue::from(account_handle);
            Reflect::set(&js_obj, &"accountHandle".into(), &js_account_handle)?;
        }
        libparsec::DeviceSaveStrategy::Keyring { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"DeviceSaveStrategyKeyring".into())?;
        }
        libparsec::DeviceSaveStrategy::Password { password, .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"DeviceSaveStrategyPassword".into())?;
            let js_password = JsValue::from_str(password.as_ref());
            Reflect::set(&js_obj, &"password".into(), &js_password)?;
        }
        libparsec::DeviceSaveStrategy::Smartcard {
            certificate_reference,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"DeviceSaveStrategySmartcard".into(),
            )?;
            let js_certificate_reference =
                struct_x509_certificate_reference_rs_to_js(certificate_reference)?;
            Reflect::set(
                &js_obj,
                &"certificateReference".into(),
                &js_certificate_reference,
            )?;
        }
    }
    Ok(js_obj)
}

// EntryStat

#[allow(dead_code)]
fn variant_entry_stat_js_to_rs(obj: JsValue) -> Result<libparsec::EntryStat, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "EntryStatFile" => {
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
                            })?,
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
                    })?
            };
            let parent = {
                let js_val = Reflect::get(&obj, &"parent".into())?;
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
                    })?
            };
            let created = {
                let js_val = Reflect::get(&obj, &"created".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
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
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
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
                    let v = v as u32;
                    v
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
                    let v = u64::try_from(js_val)
                        .map_err(|_| TypeError::new("Not a BigInt representing an u64 number"))?;
                    v
                }
            };
            let last_updater = {
                let js_val = Reflect::get(&obj, &"lastUpdater".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
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
                last_updater,
            })
        }
        "EntryStatFolder" => {
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
                            })?,
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
                    })?
            };
            let parent = {
                let js_val = Reflect::get(&obj, &"parent".into())?;
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
                    })?
            };
            let created = {
                let js_val = Reflect::get(&obj, &"created".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
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
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
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
                    let v = v as u32;
                    v
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
            let last_updater = {
                let js_val = Reflect::get(&obj, &"lastUpdater".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
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
                last_updater,
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
            parent,
            created,
            updated,
            base_version,
            is_placeholder,
            need_sync,
            size,
            last_updater,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"EntryStatFile".into())?;
            let js_confinement_point = match confinement_point {
                Some(val) => JsValue::from_str({
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(val) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
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
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"id".into(), &js_id)?;
            let js_parent = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(parent) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"parent".into(), &js_parent)?;
            let js_created = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
            let js_last_updater = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(last_updater) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"lastUpdater".into(), &js_last_updater)?;
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
            last_updater,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"EntryStatFolder".into())?;
            let js_confinement_point = match confinement_point {
                Some(val) => JsValue::from_str({
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(val) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
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
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"id".into(), &js_id)?;
            let js_parent = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(parent) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"parent".into(), &js_parent)?;
            let js_created = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
            let js_last_updater = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(last_updater) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"lastUpdater".into(), &js_last_updater)?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorActiveUsersLimitReached".into(),
            )?;
        }
        libparsec::GreetInProgressError::AlreadyDeleted { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorAlreadyDeleted".into(),
            )?;
        }
        libparsec::GreetInProgressError::Cancelled { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorCancelled".into(),
            )?;
        }
        libparsec::GreetInProgressError::CorruptedInviteUserData { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorCorruptedInviteUserData".into(),
            )?;
        }
        libparsec::GreetInProgressError::CorruptedSharedSecretKey { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorCorruptedSharedSecretKey".into(),
            )?;
        }
        libparsec::GreetInProgressError::DeviceAlreadyExists { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorDeviceAlreadyExists".into(),
            )?;
        }
        libparsec::GreetInProgressError::GreeterNotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorGreeterNotAllowed".into(),
            )?;
        }
        libparsec::GreetInProgressError::GreetingAttemptCancelled {
            origin,
            reason,
            timestamp,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorGreetingAttemptCancelled".into(),
            )?;
            let js_origin = JsValue::from_str(enum_greeter_or_claimer_rs_to_js(origin));
            Reflect::set(&js_obj, &"origin".into(), &js_origin)?;
            let js_reason =
                JsValue::from_str(enum_cancelled_greeting_attempt_reason_rs_to_js(reason));
            Reflect::set(&js_obj, &"reason".into(), &js_reason)?;
            let js_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"timestamp".into(), &js_timestamp)?;
        }
        libparsec::GreetInProgressError::HumanHandleAlreadyTaken { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorHumanHandleAlreadyTaken".into(),
            )?;
        }
        libparsec::GreetInProgressError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorInternal".into(),
            )?;
        }
        libparsec::GreetInProgressError::NonceMismatch { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorNonceMismatch".into(),
            )?;
        }
        libparsec::GreetInProgressError::NotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorNotFound".into(),
            )?;
        }
        libparsec::GreetInProgressError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorOffline".into(),
            )?;
        }
        libparsec::GreetInProgressError::PeerReset { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorPeerReset".into(),
            )?;
        }
        libparsec::GreetInProgressError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorTimestampOutOfBallpark".into(),
            )?;
            let js_server_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
        libparsec::GreetInProgressError::UserAlreadyExists { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorUserAlreadyExists".into(),
            )?;
        }
        libparsec::GreetInProgressError::UserCreateNotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"GreetInProgressErrorUserCreateNotAllowed".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ImportRecoveryDeviceError

#[allow(dead_code)]
fn variant_import_recovery_device_error_rs_to_js(
    rs_obj: libparsec::ImportRecoveryDeviceError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ImportRecoveryDeviceError::DecryptionFailed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ImportRecoveryDeviceErrorDecryptionFailed".into(),
            )?;
        }
        libparsec::ImportRecoveryDeviceError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ImportRecoveryDeviceErrorInternal".into(),
            )?;
        }
        libparsec::ImportRecoveryDeviceError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ImportRecoveryDeviceErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::ImportRecoveryDeviceError::InvalidData { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ImportRecoveryDeviceErrorInvalidData".into(),
            )?;
        }
        libparsec::ImportRecoveryDeviceError::InvalidPassphrase { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ImportRecoveryDeviceErrorInvalidPassphrase".into(),
            )?;
        }
        libparsec::ImportRecoveryDeviceError::InvalidPath { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ImportRecoveryDeviceErrorInvalidPath".into(),
            )?;
        }
        libparsec::ImportRecoveryDeviceError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ImportRecoveryDeviceErrorOffline".into(),
            )?;
        }
        libparsec::ImportRecoveryDeviceError::RemoteOpaqueKeyUploadFailed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ImportRecoveryDeviceErrorRemoteOpaqueKeyUploadFailed".into(),
            )?;
        }
        libparsec::ImportRecoveryDeviceError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ImportRecoveryDeviceErrorStopped".into(),
            )?;
        }
        libparsec::ImportRecoveryDeviceError::StorageNotAvailable { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ImportRecoveryDeviceErrorStorageNotAvailable".into(),
            )?;
        }
        libparsec::ImportRecoveryDeviceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ImportRecoveryDeviceErrorTimestampOutOfBallpark".into(),
            )?;
            let js_server_timestamp = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
    }
    Ok(js_obj)
}

// InviteInfoInvitationCreatedBy

#[allow(dead_code)]
fn variant_invite_info_invitation_created_by_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::InviteInfoInvitationCreatedBy, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "InviteInfoInvitationCreatedByExternalService" => {
            let service_label = {
                let js_val = Reflect::get(&obj, &"serviceLabel".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            Ok(libparsec::InviteInfoInvitationCreatedBy::ExternalService { service_label })
        }
        "InviteInfoInvitationCreatedByUser" => {
            let user_id = {
                let js_val = Reflect::get(&obj, &"userId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                            libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let human_handle = {
                let js_val = Reflect::get(&obj, &"humanHandle".into())?;
                struct_human_handle_js_to_rs(js_val)?
            };
            Ok(libparsec::InviteInfoInvitationCreatedBy::User {
                user_id,
                human_handle,
            })
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a InviteInfoInvitationCreatedBy",
        ))),
    }
}

#[allow(dead_code)]
fn variant_invite_info_invitation_created_by_rs_to_js(
    rs_obj: libparsec::InviteInfoInvitationCreatedBy,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::InviteInfoInvitationCreatedBy::ExternalService { service_label, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"InviteInfoInvitationCreatedByExternalService".into(),
            )?;
            let js_service_label = service_label.into();
            Reflect::set(&js_obj, &"serviceLabel".into(), &js_service_label)?;
        }
        libparsec::InviteInfoInvitationCreatedBy::User {
            user_id,
            human_handle,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"InviteInfoInvitationCreatedByUser".into(),
            )?;
            let js_user_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(user_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"userId".into(), &js_user_id)?;
            let js_human_handle = struct_human_handle_rs_to_js(human_handle)?;
            Reflect::set(&js_obj, &"humanHandle".into(), &js_human_handle)?;
        }
    }
    Ok(js_obj)
}

// InviteListInvitationCreatedBy

#[allow(dead_code)]
fn variant_invite_list_invitation_created_by_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::InviteListInvitationCreatedBy, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "InviteListInvitationCreatedByExternalService" => {
            let service_label = {
                let js_val = Reflect::get(&obj, &"serviceLabel".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))?
            };
            Ok(libparsec::InviteListInvitationCreatedBy::ExternalService { service_label })
        }
        "InviteListInvitationCreatedByUser" => {
            let user_id = {
                let js_val = Reflect::get(&obj, &"userId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                            libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let human_handle = {
                let js_val = Reflect::get(&obj, &"humanHandle".into())?;
                struct_human_handle_js_to_rs(js_val)?
            };
            Ok(libparsec::InviteListInvitationCreatedBy::User {
                user_id,
                human_handle,
            })
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a InviteListInvitationCreatedBy",
        ))),
    }
}

#[allow(dead_code)]
fn variant_invite_list_invitation_created_by_rs_to_js(
    rs_obj: libparsec::InviteListInvitationCreatedBy,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::InviteListInvitationCreatedBy::ExternalService { service_label, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"InviteListInvitationCreatedByExternalService".into(),
            )?;
            let js_service_label = service_label.into();
            Reflect::set(&js_obj, &"serviceLabel".into(), &js_service_label)?;
        }
        libparsec::InviteListInvitationCreatedBy::User {
            user_id,
            human_handle,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"InviteListInvitationCreatedByUser".into(),
            )?;
            let js_user_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(user_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"userId".into(), &js_user_id)?;
            let js_human_handle = struct_human_handle_rs_to_js(human_handle)?;
            Reflect::set(&js_obj, &"humanHandle".into(), &js_human_handle)?;
        }
    }
    Ok(js_obj)
}

// InviteListItem

#[allow(dead_code)]
fn variant_invite_list_item_js_to_rs(obj: JsValue) -> Result<libparsec::InviteListItem, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "InviteListItemDevice" => {
            let addr = {
                let js_val = Reflect::get(&obj, &"addr".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::ParsecInvitationAddr::from_any(&s).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
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
                    })?
            };
            let created_on = {
                let js_val = Reflect::get(&obj, &"createdOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let created_by = {
                let js_val = Reflect::get(&obj, &"createdBy".into())?;
                variant_invite_list_invitation_created_by_js_to_rs(js_val)?
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
                created_by,
                status,
            })
        }
        "InviteListItemShamirRecovery" => {
            let addr = {
                let js_val = Reflect::get(&obj, &"addr".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::ParsecInvitationAddr::from_any(&s).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
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
                    })?
            };
            let created_on = {
                let js_val = Reflect::get(&obj, &"createdOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let created_by = {
                let js_val = Reflect::get(&obj, &"createdBy".into())?;
                variant_invite_list_invitation_created_by_js_to_rs(js_val)?
            };
            let claimer_user_id = {
                let js_val = Reflect::get(&obj, &"claimerUserId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                            libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let shamir_recovery_created_on = {
                let js_val = Reflect::get(&obj, &"shamirRecoveryCreatedOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
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
                let js_val = Reflect::get(&obj, &"addr".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::ParsecInvitationAddr::from_any(&s).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
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
                    })?
            };
            let created_on = {
                let js_val = Reflect::get(&obj, &"createdOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let created_by = {
                let js_val = Reflect::get(&obj, &"createdBy".into())?;
                variant_invite_list_invitation_created_by_js_to_rs(js_val)?
            };
            let claimer_email = {
                let js_val = Reflect::get(&obj, &"claimerEmail".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::EmailAddress::from_str(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
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
                created_by,
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
            created_by,
            status,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"InviteListItemDevice".into())?;
            let js_addr = JsValue::from_str({
                let custom_to_rs_string =
                    |addr: libparsec::ParsecInvitationAddr| -> Result<String, &'static str> {
                        Ok(addr.to_url().into())
                    };
                match custom_to_rs_string(addr) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"addr".into(), &js_addr)?;
            let js_token = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
            let js_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
            let js_created_by = variant_invite_list_invitation_created_by_rs_to_js(created_by)?;
            Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
            let js_status = JsValue::from_str(enum_invitation_status_rs_to_js(status));
            Reflect::set(&js_obj, &"status".into(), &js_status)?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"InviteListItemShamirRecovery".into(),
            )?;
            let js_addr = JsValue::from_str({
                let custom_to_rs_string =
                    |addr: libparsec::ParsecInvitationAddr| -> Result<String, &'static str> {
                        Ok(addr.to_url().into())
                    };
                match custom_to_rs_string(addr) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"addr".into(), &js_addr)?;
            let js_token = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
            let js_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
            let js_created_by = variant_invite_list_invitation_created_by_rs_to_js(created_by)?;
            Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
            let js_claimer_user_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(claimer_user_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"claimerUserId".into(), &js_claimer_user_id)?;
            let js_shamir_recovery_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(shamir_recovery_created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(
                &js_obj,
                &"shamirRecoveryCreatedOn".into(),
                &js_shamir_recovery_created_on,
            )?;
            let js_status = JsValue::from_str(enum_invitation_status_rs_to_js(status));
            Reflect::set(&js_obj, &"status".into(), &js_status)?;
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
            Reflect::set(&js_obj, &"tag".into(), &"InviteListItemUser".into())?;
            let js_addr = JsValue::from_str({
                let custom_to_rs_string =
                    |addr: libparsec::ParsecInvitationAddr| -> Result<String, &'static str> {
                        Ok(addr.to_url().into())
                    };
                match custom_to_rs_string(addr) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"addr".into(), &js_addr)?;
            let js_token = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
            let js_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
            let js_created_by = variant_invite_list_invitation_created_by_rs_to_js(created_by)?;
            Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
            let js_claimer_email = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::EmailAddress| -> Result<_, &'static str> { Ok(x.to_string()) };
                match custom_to_rs_string(claimer_email) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"claimerEmail".into(), &js_claimer_email)?;
            let js_status = JsValue::from_str(enum_invitation_status_rs_to_js(status));
            Reflect::set(&js_obj, &"status".into(), &js_status)?;
        }
    }
    Ok(js_obj)
}

// ListAvailableDeviceError

#[allow(dead_code)]
fn variant_list_available_device_error_rs_to_js(
    rs_obj: libparsec::ListAvailableDeviceError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ListAvailableDeviceError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ListAvailableDeviceErrorInternal".into(),
            )?;
        }
        libparsec::ListAvailableDeviceError::StorageNotAvailable { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ListAvailableDeviceErrorStorageNotAvailable".into(),
            )?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ListInvitationsErrorInternal".into(),
            )?;
        }
        libparsec::ListInvitationsError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ListInvitationsErrorOffline".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// MountpointMountStrategy

#[allow(dead_code)]
fn variant_mountpoint_mount_strategy_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::MountpointMountStrategy, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "MountpointMountStrategyDirectory" => {
            let base_dir = {
                let js_val = Reflect::get(&obj, &"baseDir".into())?;
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
                    })?
            };
            Ok(libparsec::MountpointMountStrategy::Directory { base_dir })
        }
        "MountpointMountStrategyDisabled" => Ok(libparsec::MountpointMountStrategy::Disabled),
        "MountpointMountStrategyDriveLetter" => Ok(libparsec::MountpointMountStrategy::DriveLetter),
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a MountpointMountStrategy",
        ))),
    }
}

#[allow(dead_code)]
fn variant_mountpoint_mount_strategy_rs_to_js(
    rs_obj: libparsec::MountpointMountStrategy,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::MountpointMountStrategy::Directory { base_dir, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"MountpointMountStrategyDirectory".into(),
            )?;
            let js_base_dir = JsValue::from_str({
                let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                };
                match custom_to_rs_string(base_dir) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"baseDir".into(), &js_base_dir)?;
        }
        libparsec::MountpointMountStrategy::Disabled => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"MountpointMountStrategyDisabled".into(),
            )?;
        }
        libparsec::MountpointMountStrategy::DriveLetter => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"MountpointMountStrategyDriveLetter".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// MountpointToOsPathError

#[allow(dead_code)]
fn variant_mountpoint_to_os_path_error_rs_to_js(
    rs_obj: libparsec::MountpointToOsPathError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::MountpointToOsPathError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"MountpointToOsPathErrorInternal".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// MountpointUnmountError

#[allow(dead_code)]
fn variant_mountpoint_unmount_error_rs_to_js(
    rs_obj: libparsec::MountpointUnmountError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::MountpointUnmountError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"MountpointUnmountErrorInternal".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// MoveEntryMode

#[allow(dead_code)]
fn variant_move_entry_mode_js_to_rs(obj: JsValue) -> Result<libparsec::MoveEntryMode, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "MoveEntryModeCanReplace" => Ok(libparsec::MoveEntryMode::CanReplace),
        "MoveEntryModeCanReplaceFileOnly" => Ok(libparsec::MoveEntryMode::CanReplaceFileOnly),
        "MoveEntryModeExchange" => Ok(libparsec::MoveEntryMode::Exchange),
        "MoveEntryModeNoReplace" => Ok(libparsec::MoveEntryMode::NoReplace),
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a MoveEntryMode",
        ))),
    }
}

#[allow(dead_code)]
fn variant_move_entry_mode_rs_to_js(rs_obj: libparsec::MoveEntryMode) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::MoveEntryMode::CanReplace => {
            Reflect::set(&js_obj, &"tag".into(), &"MoveEntryModeCanReplace".into())?;
        }
        libparsec::MoveEntryMode::CanReplaceFileOnly => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"MoveEntryModeCanReplaceFileOnly".into(),
            )?;
        }
        libparsec::MoveEntryMode::Exchange => {
            Reflect::set(&js_obj, &"tag".into(), &"MoveEntryModeExchange".into())?;
        }
        libparsec::MoveEntryMode::NoReplace => {
            Reflect::set(&js_obj, &"tag".into(), &"MoveEntryModeNoReplace".into())?;
        }
    }
    Ok(js_obj)
}

// OtherShamirRecoveryInfo

#[allow(dead_code)]
fn variant_other_shamir_recovery_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::OtherShamirRecoveryInfo, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "OtherShamirRecoveryInfoDeleted" => {
            let user_id = {
                let js_val = Reflect::get(&obj, &"userId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                            libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let created_on = {
                let js_val = Reflect::get(&obj, &"createdOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let created_by = {
                let js_val = Reflect::get(&obj, &"createdBy".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let threshold = {
                let js_val = Reflect::get(&obj, &"threshold".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u8 number")));
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val = Reflect::get(&obj, &"perRecipientShares".into())?;
                {
                    let js_val = js_val
                        .dyn_into::<Map>()
                        .map_err(|_| TypeError::new("Not a Map"))?;
                    let mut converted =
                        std::collections::HashMap::with_capacity(js_val.size() as usize);
                    let js_keys = js_val.keys();
                    let js_values = js_val.values();
                    loop {
                        let next_js_key = js_keys.next()?;
                        let next_js_value = js_values.next()?;
                        if next_js_key.done() {
                            assert!(next_js_value.done());
                            break;
                        }
                        assert!(!next_js_value.done());

                        let js_key = next_js_key.value();
                        let js_value = next_js_value.value();

                        let key = js_key
                            .dyn_into::<JsString>()
                            .ok()
                            .and_then(|s| s.as_string())
                            .ok_or_else(|| TypeError::new("Not a string"))
                            .and_then(|x| {
                                let custom_from_rs_string =
                                    |s: String| -> Result<libparsec::UserID, _> {
                                        libparsec::UserID::from_hex(s.as_str())
                                            .map_err(|e| e.to_string())
                                    };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })?;
                        let value = {
                            let v = js_value
                                .dyn_into::<Number>()
                                .map_err(|_| TypeError::new("Not a number"))?
                                .value_of();
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                return Err(JsValue::from(TypeError::new("Not an u8 number")));
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => {
                                    return Err(JsValue::from(TypeError::new(err.as_ref())))
                                }
                            }
                        };
                        converted.insert(key, value);
                    }
                    converted
                }
            };
            let deleted_on = {
                let js_val = Reflect::get(&obj, &"deletedOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let deleted_by = {
                let js_val = Reflect::get(&obj, &"deletedBy".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
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
                let js_val = Reflect::get(&obj, &"userId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                            libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let created_on = {
                let js_val = Reflect::get(&obj, &"createdOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let created_by = {
                let js_val = Reflect::get(&obj, &"createdBy".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let threshold = {
                let js_val = Reflect::get(&obj, &"threshold".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u8 number")));
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val = Reflect::get(&obj, &"perRecipientShares".into())?;
                {
                    let js_val = js_val
                        .dyn_into::<Map>()
                        .map_err(|_| TypeError::new("Not a Map"))?;
                    let mut converted =
                        std::collections::HashMap::with_capacity(js_val.size() as usize);
                    let js_keys = js_val.keys();
                    let js_values = js_val.values();
                    loop {
                        let next_js_key = js_keys.next()?;
                        let next_js_value = js_values.next()?;
                        if next_js_key.done() {
                            assert!(next_js_value.done());
                            break;
                        }
                        assert!(!next_js_value.done());

                        let js_key = next_js_key.value();
                        let js_value = next_js_value.value();

                        let key = js_key
                            .dyn_into::<JsString>()
                            .ok()
                            .and_then(|s| s.as_string())
                            .ok_or_else(|| TypeError::new("Not a string"))
                            .and_then(|x| {
                                let custom_from_rs_string =
                                    |s: String| -> Result<libparsec::UserID, _> {
                                        libparsec::UserID::from_hex(s.as_str())
                                            .map_err(|e| e.to_string())
                                    };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })?;
                        let value = {
                            let v = js_value
                                .dyn_into::<Number>()
                                .map_err(|_| TypeError::new("Not a number"))?
                                .value_of();
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                return Err(JsValue::from(TypeError::new("Not an u8 number")));
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => {
                                    return Err(JsValue::from(TypeError::new(err.as_ref())))
                                }
                            }
                        };
                        converted.insert(key, value);
                    }
                    converted
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
                let js_val = Reflect::get(&obj, &"userId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                            libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let created_on = {
                let js_val = Reflect::get(&obj, &"createdOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let created_by = {
                let js_val = Reflect::get(&obj, &"createdBy".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let threshold = {
                let js_val = Reflect::get(&obj, &"threshold".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u8 number")));
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val = Reflect::get(&obj, &"perRecipientShares".into())?;
                {
                    let js_val = js_val
                        .dyn_into::<Map>()
                        .map_err(|_| TypeError::new("Not a Map"))?;
                    let mut converted =
                        std::collections::HashMap::with_capacity(js_val.size() as usize);
                    let js_keys = js_val.keys();
                    let js_values = js_val.values();
                    loop {
                        let next_js_key = js_keys.next()?;
                        let next_js_value = js_values.next()?;
                        if next_js_key.done() {
                            assert!(next_js_value.done());
                            break;
                        }
                        assert!(!next_js_value.done());

                        let js_key = next_js_key.value();
                        let js_value = next_js_value.value();

                        let key = js_key
                            .dyn_into::<JsString>()
                            .ok()
                            .and_then(|s| s.as_string())
                            .ok_or_else(|| TypeError::new("Not a string"))
                            .and_then(|x| {
                                let custom_from_rs_string =
                                    |s: String| -> Result<libparsec::UserID, _> {
                                        libparsec::UserID::from_hex(s.as_str())
                                            .map_err(|e| e.to_string())
                                    };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })?;
                        let value = {
                            let v = js_value
                                .dyn_into::<Number>()
                                .map_err(|_| TypeError::new("Not a number"))?
                                .value_of();
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                return Err(JsValue::from(TypeError::new("Not an u8 number")));
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => {
                                    return Err(JsValue::from(TypeError::new(err.as_ref())))
                                }
                            }
                        };
                        converted.insert(key, value);
                    }
                    converted
                }
            };
            let revoked_recipients = {
                let js_val = Reflect::get(&obj, &"revokedRecipients".into())?;
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
                                let custom_from_rs_string =
                                    |s: String| -> Result<libparsec::UserID, _> {
                                        libparsec::UserID::from_hex(s.as_str())
                                            .map_err(|e| e.to_string())
                                    };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })?;
                        converted.push(x_converted);
                    }
                    converted.into_iter().collect()
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
                let js_val = Reflect::get(&obj, &"userId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                            libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let created_on = {
                let js_val = Reflect::get(&obj, &"createdOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let created_by = {
                let js_val = Reflect::get(&obj, &"createdBy".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let threshold = {
                let js_val = Reflect::get(&obj, &"threshold".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u8 number")));
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val = Reflect::get(&obj, &"perRecipientShares".into())?;
                {
                    let js_val = js_val
                        .dyn_into::<Map>()
                        .map_err(|_| TypeError::new("Not a Map"))?;
                    let mut converted =
                        std::collections::HashMap::with_capacity(js_val.size() as usize);
                    let js_keys = js_val.keys();
                    let js_values = js_val.values();
                    loop {
                        let next_js_key = js_keys.next()?;
                        let next_js_value = js_values.next()?;
                        if next_js_key.done() {
                            assert!(next_js_value.done());
                            break;
                        }
                        assert!(!next_js_value.done());

                        let js_key = next_js_key.value();
                        let js_value = next_js_value.value();

                        let key = js_key
                            .dyn_into::<JsString>()
                            .ok()
                            .and_then(|s| s.as_string())
                            .ok_or_else(|| TypeError::new("Not a string"))
                            .and_then(|x| {
                                let custom_from_rs_string =
                                    |s: String| -> Result<libparsec::UserID, _> {
                                        libparsec::UserID::from_hex(s.as_str())
                                            .map_err(|e| e.to_string())
                                    };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })?;
                        let value = {
                            let v = js_value
                                .dyn_into::<Number>()
                                .map_err(|_| TypeError::new("Not a number"))?
                                .value_of();
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                return Err(JsValue::from(TypeError::new("Not an u8 number")));
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => {
                                    return Err(JsValue::from(TypeError::new(err.as_ref())))
                                }
                            }
                        };
                        converted.insert(key, value);
                    }
                    converted
                }
            };
            let revoked_recipients = {
                let js_val = Reflect::get(&obj, &"revokedRecipients".into())?;
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
                                let custom_from_rs_string =
                                    |s: String| -> Result<libparsec::UserID, _> {
                                        libparsec::UserID::from_hex(s.as_str())
                                            .map_err(|e| e.to_string())
                                    };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })?;
                        converted.push(x_converted);
                    }
                    converted.into_iter().collect()
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
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a OtherShamirRecoveryInfo",
        ))),
    }
}

#[allow(dead_code)]
fn variant_other_shamir_recovery_info_rs_to_js(
    rs_obj: libparsec::OtherShamirRecoveryInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"OtherShamirRecoveryInfoDeleted".into(),
            )?;
            let js_user_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(user_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"userId".into(), &js_user_id)?;
            let js_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
            let js_created_by = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
            let js_threshold = {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                let v = match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"threshold".into(), &js_threshold)?;
            let js_per_recipient_shares = {
                let js_map = Map::new();
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    let js_value = {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        let v = match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                        };
                        JsValue::from(v)
                    };
                    js_map.set(&js_key, &js_value);
                }
                js_map.into()
            };
            Reflect::set(
                &js_obj,
                &"perRecipientShares".into(),
                &js_per_recipient_shares,
            )?;
            let js_deleted_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(deleted_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"deletedOn".into(), &js_deleted_on)?;
            let js_deleted_by = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(deleted_by) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"deletedBy".into(), &js_deleted_by)?;
        }
        libparsec::OtherShamirRecoveryInfo::SetupAllValid {
            user_id,
            created_on,
            created_by,
            threshold,
            per_recipient_shares,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"OtherShamirRecoveryInfoSetupAllValid".into(),
            )?;
            let js_user_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(user_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"userId".into(), &js_user_id)?;
            let js_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
            let js_created_by = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
            let js_threshold = {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                let v = match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"threshold".into(), &js_threshold)?;
            let js_per_recipient_shares = {
                let js_map = Map::new();
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    let js_value = {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        let v = match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                        };
                        JsValue::from(v)
                    };
                    js_map.set(&js_key, &js_value);
                }
                js_map.into()
            };
            Reflect::set(
                &js_obj,
                &"perRecipientShares".into(),
                &js_per_recipient_shares,
            )?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"OtherShamirRecoveryInfoSetupButUnusable".into(),
            )?;
            let js_user_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(user_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"userId".into(), &js_user_id)?;
            let js_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
            let js_created_by = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
            let js_threshold = {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                let v = match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"threshold".into(), &js_threshold)?;
            let js_per_recipient_shares = {
                let js_map = Map::new();
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    let js_value = {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        let v = match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                        };
                        JsValue::from(v)
                    };
                    js_map.set(&js_key, &js_value);
                }
                js_map.into()
            };
            Reflect::set(
                &js_obj,
                &"perRecipientShares".into(),
                &js_per_recipient_shares,
            )?;
            let js_revoked_recipients = {
                // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                let js_array = Array::new_with_length(revoked_recipients.len() as u32);
                for (i, elem) in revoked_recipients.into_iter().enumerate() {
                    let js_elem = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(elem) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    js_array.set(i as u32, js_elem);
                }
                js_array.into()
            };
            Reflect::set(&js_obj, &"revokedRecipients".into(), &js_revoked_recipients)?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"OtherShamirRecoveryInfoSetupWithRevokedRecipients".into(),
            )?;
            let js_user_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(user_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"userId".into(), &js_user_id)?;
            let js_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
            let js_created_by = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
            let js_threshold = {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                let v = match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"threshold".into(), &js_threshold)?;
            let js_per_recipient_shares = {
                let js_map = Map::new();
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    let js_value = {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        let v = match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                        };
                        JsValue::from(v)
                    };
                    js_map.set(&js_key, &js_value);
                }
                js_map.into()
            };
            Reflect::set(
                &js_obj,
                &"perRecipientShares".into(),
                &js_per_recipient_shares,
            )?;
            let js_revoked_recipients = {
                // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                let js_array = Array::new_with_length(revoked_recipients.len() as u32);
                for (i, elem) in revoked_recipients.into_iter().enumerate() {
                    let js_elem = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(elem) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    js_array.set(i as u32, js_elem);
                }
                js_array.into()
            };
            Reflect::set(&js_obj, &"revokedRecipients".into(), &js_revoked_recipients)?;
        }
    }
    Ok(js_obj)
}

// ParseParsecAddrError

#[allow(dead_code)]
fn variant_parse_parsec_addr_error_rs_to_js(
    rs_obj: libparsec::ParseParsecAddrError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ParseParsecAddrError::InvalidUrl { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ParseParsecAddrErrorInvalidUrl".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ParsedParsecAddr

#[allow(dead_code)]
fn variant_parsed_parsec_addr_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::ParsedParsecAddr, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "ParsedParsecAddrInvitationDevice" => {
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
                    let v = v as u32;
                    v
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
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::OrganizationID::try_from(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
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
                    })?
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
                    let v = v as u32;
                    v
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
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::OrganizationID::try_from(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
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
                    })?
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
                    let v = v as u32;
                    v
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
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::OrganizationID::try_from(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
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
                    })?
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
                    let v = v as u32;
                    v
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
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::OrganizationID::try_from(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
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
                    let v = v as u32;
                    v
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
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::OrganizationID::try_from(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
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
                    let v = v as u32;
                    v
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
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::OrganizationID::try_from(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
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
                    let v = v as u32;
                    v
                }
            };
            let use_ssl = {
                let js_val = Reflect::get(&obj, &"useSsl".into())?;
                js_val
                    .dyn_into::<Boolean>()
                    .map_err(|_| TypeError::new("Not a boolean"))?
                    .value_of()
            };
            Ok(libparsec::ParsedParsecAddr::Server {
                hostname,
                port,
                use_ssl,
            })
        }
        "ParsedParsecAddrWorkspacePath" => {
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
                    let v = v as u32;
                    v
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
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<_, String> {
                            libparsec::OrganizationID::try_from(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
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
                    })?
            };
            let key_index = {
                let js_val = Reflect::get(&obj, &"keyIndex".into())?;
                {
                    let v = u64::try_from(js_val)
                        .map_err(|_| TypeError::new("Not a BigInt representing an u64 number"))?;
                    v
                }
            };
            let encrypted_path = {
                let js_val = Reflect::get(&obj, &"encryptedPath".into())?;
                js_val
                    .dyn_into::<Uint8Array>()
                    .map_err(|_| TypeError::new("Not a Uint8Array"))?
                    .to_vec()
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
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a ParsedParsecAddr",
        ))),
    }
}

#[allow(dead_code)]
fn variant_parsed_parsec_addr_rs_to_js(
    rs_obj: libparsec::ParsedParsecAddr,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::ParsedParsecAddr::InvitationDevice {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ParsedParsecAddrInvitationDevice".into(),
            )?;
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
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
        }
        libparsec::ParsedParsecAddr::InvitationShamirRecovery {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ParsedParsecAddrInvitationShamirRecovery".into(),
            )?;
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
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
        }
        libparsec::ParsedParsecAddr::InvitationUser {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ParsedParsecAddrInvitationUser".into(),
            )?;
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
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"token".into(), &js_token)?;
        }
        libparsec::ParsedParsecAddr::Organization {
            hostname,
            port,
            use_ssl,
            organization_id,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ParsedParsecAddrOrganization".into(),
            )?;
            let js_hostname = hostname.into();
            Reflect::set(&js_obj, &"hostname".into(), &js_hostname)?;
            let js_port = JsValue::from(port);
            Reflect::set(&js_obj, &"port".into(), &js_port)?;
            let js_use_ssl = use_ssl.into();
            Reflect::set(&js_obj, &"useSsl".into(), &js_use_ssl)?;
            let js_organization_id = JsValue::from_str(organization_id.as_ref());
            Reflect::set(&js_obj, &"organizationId".into(), &js_organization_id)?;
        }
        libparsec::ParsedParsecAddr::OrganizationBootstrap {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ParsedParsecAddrOrganizationBootstrap".into(),
            )?;
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
        libparsec::ParsedParsecAddr::PkiEnrollment {
            hostname,
            port,
            use_ssl,
            organization_id,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ParsedParsecAddrPkiEnrollment".into(),
            )?;
            let js_hostname = hostname.into();
            Reflect::set(&js_obj, &"hostname".into(), &js_hostname)?;
            let js_port = JsValue::from(port);
            Reflect::set(&js_obj, &"port".into(), &js_port)?;
            let js_use_ssl = use_ssl.into();
            Reflect::set(&js_obj, &"useSsl".into(), &js_use_ssl)?;
            let js_organization_id = JsValue::from_str(organization_id.as_ref());
            Reflect::set(&js_obj, &"organizationId".into(), &js_organization_id)?;
        }
        libparsec::ParsedParsecAddr::Server {
            hostname,
            port,
            use_ssl,
            ..
        } => {
            Reflect::set(&js_obj, &"tag".into(), &"ParsedParsecAddrServer".into())?;
            let js_hostname = hostname.into();
            Reflect::set(&js_obj, &"hostname".into(), &js_hostname)?;
            let js_port = JsValue::from(port);
            Reflect::set(&js_obj, &"port".into(), &js_port)?;
            let js_use_ssl = use_ssl.into();
            Reflect::set(&js_obj, &"useSsl".into(), &js_use_ssl)?;
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ParsedParsecAddrWorkspacePath".into(),
            )?;
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
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"workspaceId".into(), &js_workspace_id)?;
            let js_key_index = JsValue::from(key_index);
            Reflect::set(&js_obj, &"keyIndex".into(), &js_key_index)?;
            let js_encrypted_path = JsValue::from(Uint8Array::from(encrypted_path.as_ref()));
            Reflect::set(&js_obj, &"encryptedPath".into(), &js_encrypted_path)?;
        }
    }
    Ok(js_obj)
}

// PkiEnrollmentAcceptError

#[allow(dead_code)]
fn variant_pki_enrollment_accept_error_rs_to_js(
    rs_obj: libparsec::PkiEnrollmentAcceptError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::PkiEnrollmentAcceptError::ActiveUsersLimitReached { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentAcceptErrorActiveUsersLimitReached".into(),
            )?;
        }
        libparsec::PkiEnrollmentAcceptError::AuthorNotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentAcceptErrorAuthorNotAllowed".into(),
            )?;
        }
        libparsec::PkiEnrollmentAcceptError::EnrollmentNoLongerAvailable { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentAcceptErrorEnrollmentNoLongerAvailable".into(),
            )?;
        }
        libparsec::PkiEnrollmentAcceptError::EnrollmentNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentAcceptErrorEnrollmentNotFound".into(),
            )?;
        }
        libparsec::PkiEnrollmentAcceptError::HumanHandleAlreadyTaken { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentAcceptErrorHumanHandleAlreadyTaken".into(),
            )?;
        }
        libparsec::PkiEnrollmentAcceptError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentAcceptErrorInternal".into(),
            )?;
        }
        libparsec::PkiEnrollmentAcceptError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentAcceptErrorOffline".into(),
            )?;
        }
        libparsec::PkiEnrollmentAcceptError::PkiOperationError { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentAcceptErrorPkiOperationError".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// PkiEnrollmentListError

#[allow(dead_code)]
fn variant_pki_enrollment_list_error_rs_to_js(
    rs_obj: libparsec::PkiEnrollmentListError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::PkiEnrollmentListError::AuthorNotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentListErrorAuthorNotAllowed".into(),
            )?;
        }
        libparsec::PkiEnrollmentListError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentListErrorInternal".into(),
            )?;
        }
        libparsec::PkiEnrollmentListError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentListErrorOffline".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// PkiEnrollmentRejectError

#[allow(dead_code)]
fn variant_pki_enrollment_reject_error_rs_to_js(
    rs_obj: libparsec::PkiEnrollmentRejectError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::PkiEnrollmentRejectError::AuthorNotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentRejectErrorAuthorNotAllowed".into(),
            )?;
        }
        libparsec::PkiEnrollmentRejectError::EnrollmentNoLongerAvailable { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentRejectErrorEnrollmentNoLongerAvailable".into(),
            )?;
        }
        libparsec::PkiEnrollmentRejectError::EnrollmentNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentRejectErrorEnrollmentNotFound".into(),
            )?;
        }
        libparsec::PkiEnrollmentRejectError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentRejectErrorInternal".into(),
            )?;
        }
        libparsec::PkiEnrollmentRejectError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentRejectErrorOffline".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// PkiEnrollmentSubmitError

#[allow(dead_code)]
fn variant_pki_enrollment_submit_error_rs_to_js(
    rs_obj: libparsec::PkiEnrollmentSubmitError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::PkiEnrollmentSubmitError::AlreadyEnrolled { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentSubmitErrorAlreadyEnrolled".into(),
            )?;
        }
        libparsec::PkiEnrollmentSubmitError::AlreadySubmitted { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentSubmitErrorAlreadySubmitted".into(),
            )?;
        }
        libparsec::PkiEnrollmentSubmitError::EmailAlreadyUsed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentSubmitErrorEmailAlreadyUsed".into(),
            )?;
        }
        libparsec::PkiEnrollmentSubmitError::IdAlreadyUsed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentSubmitErrorIdAlreadyUsed".into(),
            )?;
        }
        libparsec::PkiEnrollmentSubmitError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentSubmitErrorInternal".into(),
            )?;
        }
        libparsec::PkiEnrollmentSubmitError::InvalidPayload { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentSubmitErrorInvalidPayload".into(),
            )?;
        }
        libparsec::PkiEnrollmentSubmitError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentSubmitErrorOffline".into(),
            )?;
        }
        libparsec::PkiEnrollmentSubmitError::PkiOperationError { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"PkiEnrollmentSubmitErrorPkiOperationError".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// SelfShamirRecoveryInfo

#[allow(dead_code)]
fn variant_self_shamir_recovery_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::SelfShamirRecoveryInfo, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "SelfShamirRecoveryInfoDeleted" => {
            let created_on = {
                let js_val = Reflect::get(&obj, &"createdOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let created_by = {
                let js_val = Reflect::get(&obj, &"createdBy".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let threshold = {
                let js_val = Reflect::get(&obj, &"threshold".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u8 number")));
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val = Reflect::get(&obj, &"perRecipientShares".into())?;
                {
                    let js_val = js_val
                        .dyn_into::<Map>()
                        .map_err(|_| TypeError::new("Not a Map"))?;
                    let mut converted =
                        std::collections::HashMap::with_capacity(js_val.size() as usize);
                    let js_keys = js_val.keys();
                    let js_values = js_val.values();
                    loop {
                        let next_js_key = js_keys.next()?;
                        let next_js_value = js_values.next()?;
                        if next_js_key.done() {
                            assert!(next_js_value.done());
                            break;
                        }
                        assert!(!next_js_value.done());

                        let js_key = next_js_key.value();
                        let js_value = next_js_value.value();

                        let key = js_key
                            .dyn_into::<JsString>()
                            .ok()
                            .and_then(|s| s.as_string())
                            .ok_or_else(|| TypeError::new("Not a string"))
                            .and_then(|x| {
                                let custom_from_rs_string =
                                    |s: String| -> Result<libparsec::UserID, _> {
                                        libparsec::UserID::from_hex(s.as_str())
                                            .map_err(|e| e.to_string())
                                    };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })?;
                        let value = {
                            let v = js_value
                                .dyn_into::<Number>()
                                .map_err(|_| TypeError::new("Not a number"))?
                                .value_of();
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                return Err(JsValue::from(TypeError::new("Not an u8 number")));
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => {
                                    return Err(JsValue::from(TypeError::new(err.as_ref())))
                                }
                            }
                        };
                        converted.insert(key, value);
                    }
                    converted
                }
            };
            let deleted_on = {
                let js_val = Reflect::get(&obj, &"deletedOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let deleted_by = {
                let js_val = Reflect::get(&obj, &"deletedBy".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
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
                let js_val = Reflect::get(&obj, &"createdOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let created_by = {
                let js_val = Reflect::get(&obj, &"createdBy".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let threshold = {
                let js_val = Reflect::get(&obj, &"threshold".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u8 number")));
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val = Reflect::get(&obj, &"perRecipientShares".into())?;
                {
                    let js_val = js_val
                        .dyn_into::<Map>()
                        .map_err(|_| TypeError::new("Not a Map"))?;
                    let mut converted =
                        std::collections::HashMap::with_capacity(js_val.size() as usize);
                    let js_keys = js_val.keys();
                    let js_values = js_val.values();
                    loop {
                        let next_js_key = js_keys.next()?;
                        let next_js_value = js_values.next()?;
                        if next_js_key.done() {
                            assert!(next_js_value.done());
                            break;
                        }
                        assert!(!next_js_value.done());

                        let js_key = next_js_key.value();
                        let js_value = next_js_value.value();

                        let key = js_key
                            .dyn_into::<JsString>()
                            .ok()
                            .and_then(|s| s.as_string())
                            .ok_or_else(|| TypeError::new("Not a string"))
                            .and_then(|x| {
                                let custom_from_rs_string =
                                    |s: String| -> Result<libparsec::UserID, _> {
                                        libparsec::UserID::from_hex(s.as_str())
                                            .map_err(|e| e.to_string())
                                    };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })?;
                        let value = {
                            let v = js_value
                                .dyn_into::<Number>()
                                .map_err(|_| TypeError::new("Not a number"))?
                                .value_of();
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                return Err(JsValue::from(TypeError::new("Not an u8 number")));
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => {
                                    return Err(JsValue::from(TypeError::new(err.as_ref())))
                                }
                            }
                        };
                        converted.insert(key, value);
                    }
                    converted
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
                let js_val = Reflect::get(&obj, &"createdOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let created_by = {
                let js_val = Reflect::get(&obj, &"createdBy".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let threshold = {
                let js_val = Reflect::get(&obj, &"threshold".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u8 number")));
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val = Reflect::get(&obj, &"perRecipientShares".into())?;
                {
                    let js_val = js_val
                        .dyn_into::<Map>()
                        .map_err(|_| TypeError::new("Not a Map"))?;
                    let mut converted =
                        std::collections::HashMap::with_capacity(js_val.size() as usize);
                    let js_keys = js_val.keys();
                    let js_values = js_val.values();
                    loop {
                        let next_js_key = js_keys.next()?;
                        let next_js_value = js_values.next()?;
                        if next_js_key.done() {
                            assert!(next_js_value.done());
                            break;
                        }
                        assert!(!next_js_value.done());

                        let js_key = next_js_key.value();
                        let js_value = next_js_value.value();

                        let key = js_key
                            .dyn_into::<JsString>()
                            .ok()
                            .and_then(|s| s.as_string())
                            .ok_or_else(|| TypeError::new("Not a string"))
                            .and_then(|x| {
                                let custom_from_rs_string =
                                    |s: String| -> Result<libparsec::UserID, _> {
                                        libparsec::UserID::from_hex(s.as_str())
                                            .map_err(|e| e.to_string())
                                    };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })?;
                        let value = {
                            let v = js_value
                                .dyn_into::<Number>()
                                .map_err(|_| TypeError::new("Not a number"))?
                                .value_of();
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                return Err(JsValue::from(TypeError::new("Not an u8 number")));
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => {
                                    return Err(JsValue::from(TypeError::new(err.as_ref())))
                                }
                            }
                        };
                        converted.insert(key, value);
                    }
                    converted
                }
            };
            let revoked_recipients = {
                let js_val = Reflect::get(&obj, &"revokedRecipients".into())?;
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
                                let custom_from_rs_string =
                                    |s: String| -> Result<libparsec::UserID, _> {
                                        libparsec::UserID::from_hex(s.as_str())
                                            .map_err(|e| e.to_string())
                                    };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })?;
                        converted.push(x_converted);
                    }
                    converted.into_iter().collect()
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
                let js_val = Reflect::get(&obj, &"createdOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let created_by = {
                let js_val = Reflect::get(&obj, &"createdBy".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let threshold = {
                let js_val = Reflect::get(&obj, &"threshold".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u8 number")));
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                }
            };
            let per_recipient_shares = {
                let js_val = Reflect::get(&obj, &"perRecipientShares".into())?;
                {
                    let js_val = js_val
                        .dyn_into::<Map>()
                        .map_err(|_| TypeError::new("Not a Map"))?;
                    let mut converted =
                        std::collections::HashMap::with_capacity(js_val.size() as usize);
                    let js_keys = js_val.keys();
                    let js_values = js_val.values();
                    loop {
                        let next_js_key = js_keys.next()?;
                        let next_js_value = js_values.next()?;
                        if next_js_key.done() {
                            assert!(next_js_value.done());
                            break;
                        }
                        assert!(!next_js_value.done());

                        let js_key = next_js_key.value();
                        let js_value = next_js_value.value();

                        let key = js_key
                            .dyn_into::<JsString>()
                            .ok()
                            .and_then(|s| s.as_string())
                            .ok_or_else(|| TypeError::new("Not a string"))
                            .and_then(|x| {
                                let custom_from_rs_string =
                                    |s: String| -> Result<libparsec::UserID, _> {
                                        libparsec::UserID::from_hex(s.as_str())
                                            .map_err(|e| e.to_string())
                                    };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })?;
                        let value = {
                            let v = js_value
                                .dyn_into::<Number>()
                                .map_err(|_| TypeError::new("Not a number"))?
                                .value_of();
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                return Err(JsValue::from(TypeError::new("Not an u8 number")));
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => {
                                    return Err(JsValue::from(TypeError::new(err.as_ref())))
                                }
                            }
                        };
                        converted.insert(key, value);
                    }
                    converted
                }
            };
            let revoked_recipients = {
                let js_val = Reflect::get(&obj, &"revokedRecipients".into())?;
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
                                let custom_from_rs_string =
                                    |s: String| -> Result<libparsec::UserID, _> {
                                        libparsec::UserID::from_hex(s.as_str())
                                            .map_err(|e| e.to_string())
                                    };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })?;
                        converted.push(x_converted);
                    }
                    converted.into_iter().collect()
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
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a SelfShamirRecoveryInfo",
        ))),
    }
}

#[allow(dead_code)]
fn variant_self_shamir_recovery_info_rs_to_js(
    rs_obj: libparsec::SelfShamirRecoveryInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"SelfShamirRecoveryInfoDeleted".into(),
            )?;
            let js_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
            let js_created_by = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
            let js_threshold = {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                let v = match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"threshold".into(), &js_threshold)?;
            let js_per_recipient_shares = {
                let js_map = Map::new();
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    let js_value = {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        let v = match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                        };
                        JsValue::from(v)
                    };
                    js_map.set(&js_key, &js_value);
                }
                js_map.into()
            };
            Reflect::set(
                &js_obj,
                &"perRecipientShares".into(),
                &js_per_recipient_shares,
            )?;
            let js_deleted_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(deleted_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"deletedOn".into(), &js_deleted_on)?;
            let js_deleted_by = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(deleted_by) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"deletedBy".into(), &js_deleted_by)?;
        }
        libparsec::SelfShamirRecoveryInfo::NeverSetup { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"SelfShamirRecoveryInfoNeverSetup".into(),
            )?;
        }
        libparsec::SelfShamirRecoveryInfo::SetupAllValid {
            created_on,
            created_by,
            threshold,
            per_recipient_shares,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"SelfShamirRecoveryInfoSetupAllValid".into(),
            )?;
            let js_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
            let js_created_by = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
            let js_threshold = {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                let v = match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"threshold".into(), &js_threshold)?;
            let js_per_recipient_shares = {
                let js_map = Map::new();
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    let js_value = {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        let v = match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                        };
                        JsValue::from(v)
                    };
                    js_map.set(&js_key, &js_value);
                }
                js_map.into()
            };
            Reflect::set(
                &js_obj,
                &"perRecipientShares".into(),
                &js_per_recipient_shares,
            )?;
        }
        libparsec::SelfShamirRecoveryInfo::SetupButUnusable {
            created_on,
            created_by,
            threshold,
            per_recipient_shares,
            revoked_recipients,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"SelfShamirRecoveryInfoSetupButUnusable".into(),
            )?;
            let js_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
            let js_created_by = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
            let js_threshold = {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                let v = match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"threshold".into(), &js_threshold)?;
            let js_per_recipient_shares = {
                let js_map = Map::new();
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    let js_value = {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        let v = match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                        };
                        JsValue::from(v)
                    };
                    js_map.set(&js_key, &js_value);
                }
                js_map.into()
            };
            Reflect::set(
                &js_obj,
                &"perRecipientShares".into(),
                &js_per_recipient_shares,
            )?;
            let js_revoked_recipients = {
                // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                let js_array = Array::new_with_length(revoked_recipients.len() as u32);
                for (i, elem) in revoked_recipients.into_iter().enumerate() {
                    let js_elem = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(elem) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    js_array.set(i as u32, js_elem);
                }
                js_array.into()
            };
            Reflect::set(&js_obj, &"revokedRecipients".into(), &js_revoked_recipients)?;
        }
        libparsec::SelfShamirRecoveryInfo::SetupWithRevokedRecipients {
            created_on,
            created_by,
            threshold,
            per_recipient_shares,
            revoked_recipients,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"SelfShamirRecoveryInfoSetupWithRevokedRecipients".into(),
            )?;
            let js_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"createdOn".into(), &js_created_on)?;
            let js_created_by = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(created_by) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"createdBy".into(), &js_created_by)?;
            let js_threshold = {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                let v = match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"threshold".into(), &js_threshold)?;
            let js_per_recipient_shares = {
                let js_map = Map::new();
                for (key, value) in per_recipient_shares.into_iter() {
                    let js_key = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    let js_value = {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        let v = match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                        };
                        JsValue::from(v)
                    };
                    js_map.set(&js_key, &js_value);
                }
                js_map.into()
            };
            Reflect::set(
                &js_obj,
                &"perRecipientShares".into(),
                &js_per_recipient_shares,
            )?;
            let js_revoked_recipients = {
                // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                let js_array = Array::new_with_length(revoked_recipients.len() as u32);
                for (i, elem) in revoked_recipients.into_iter().enumerate() {
                    let js_elem = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(elem) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    js_array.set(i as u32, js_elem);
                }
                js_array.into()
            };
            Reflect::set(&js_obj, &"revokedRecipients".into(), &js_revoked_recipients)?;
        }
    }
    Ok(js_obj)
}

// ShamirRecoveryClaimAddShareError

#[allow(dead_code)]
fn variant_shamir_recovery_claim_add_share_error_rs_to_js(
    rs_obj: libparsec::ShamirRecoveryClaimAddShareError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ShamirRecoveryClaimAddShareError::CorruptedSecret { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimAddShareErrorCorruptedSecret".into(),
            )?;
        }
        libparsec::ShamirRecoveryClaimAddShareError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimAddShareErrorInternal".into(),
            )?;
        }
        libparsec::ShamirRecoveryClaimAddShareError::RecipientNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimAddShareErrorRecipientNotFound".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ShamirRecoveryClaimMaybeFinalizeInfo

#[allow(dead_code)]
fn variant_shamir_recovery_claim_maybe_finalize_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::ShamirRecoveryClaimMaybeFinalizeInfo, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "ShamirRecoveryClaimMaybeFinalizeInfoFinalize" => {
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
                    let v = v as u32;
                    v
                }
            };
            Ok(libparsec::ShamirRecoveryClaimMaybeFinalizeInfo::Finalize { handle })
        }
        "ShamirRecoveryClaimMaybeFinalizeInfoOffline" => {
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
                    let v = v as u32;
                    v
                }
            };
            Ok(libparsec::ShamirRecoveryClaimMaybeFinalizeInfo::Offline { handle })
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a ShamirRecoveryClaimMaybeFinalizeInfo",
        ))),
    }
}

#[allow(dead_code)]
fn variant_shamir_recovery_claim_maybe_finalize_info_rs_to_js(
    rs_obj: libparsec::ShamirRecoveryClaimMaybeFinalizeInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::ShamirRecoveryClaimMaybeFinalizeInfo::Finalize { handle, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimMaybeFinalizeInfoFinalize".into(),
            )?;
            let js_handle = JsValue::from(handle);
            Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
        }
        libparsec::ShamirRecoveryClaimMaybeFinalizeInfo::Offline { handle, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimMaybeFinalizeInfoOffline".into(),
            )?;
            let js_handle = JsValue::from(handle);
            Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
        }
    }
    Ok(js_obj)
}

// ShamirRecoveryClaimMaybeRecoverDeviceInfo

#[allow(dead_code)]
fn variant_shamir_recovery_claim_maybe_recover_device_info_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::ShamirRecoveryClaimMaybeRecoverDeviceInfo, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient" => {
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
                    let v = v as u32;
                    v
                }
            };
            let claimer_user_id = {
                let js_val = Reflect::get(&obj, &"claimerUserId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                            libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let claimer_human_handle = {
                let js_val = Reflect::get(&obj, &"claimerHumanHandle".into())?;
                struct_human_handle_js_to_rs(js_val)?
            };
            let shamir_recovery_created_on = {
                let js_val = Reflect::get(&obj, &"shamirRecoveryCreatedOn".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let recipients = {
                let js_val = Reflect::get(&obj, &"recipients".into())?;
                {
                    let js_val = js_val
                        .dyn_into::<Array>()
                        .map_err(|_| TypeError::new("Not an array"))?;
                    let mut converted = Vec::with_capacity(js_val.length() as usize);
                    for x in js_val.iter() {
                        let x_converted = struct_shamir_recovery_recipient_js_to_rs(x)?;
                        converted.push(x_converted);
                    }
                    converted
                }
            };
            let threshold = {
                let js_val = Reflect::get(&obj, &"threshold".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u8 number")));
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                }
            };
            let recovered_shares = {
                let js_val = Reflect::get(&obj, &"recoveredShares".into())?;
                {
                    let js_val = js_val
                        .dyn_into::<Map>()
                        .map_err(|_| TypeError::new("Not a Map"))?;
                    let mut converted =
                        std::collections::HashMap::with_capacity(js_val.size() as usize);
                    let js_keys = js_val.keys();
                    let js_values = js_val.values();
                    loop {
                        let next_js_key = js_keys.next()?;
                        let next_js_value = js_values.next()?;
                        if next_js_key.done() {
                            assert!(next_js_value.done());
                            break;
                        }
                        assert!(!next_js_value.done());

                        let js_key = next_js_key.value();
                        let js_value = next_js_value.value();

                        let key = js_key
                            .dyn_into::<JsString>()
                            .ok()
                            .and_then(|s| s.as_string())
                            .ok_or_else(|| TypeError::new("Not a string"))
                            .and_then(|x| {
                                let custom_from_rs_string =
                                    |s: String| -> Result<libparsec::UserID, _> {
                                        libparsec::UserID::from_hex(s.as_str())
                                            .map_err(|e| e.to_string())
                                    };
                                custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                            })?;
                        let value = {
                            let v = js_value
                                .dyn_into::<Number>()
                                .map_err(|_| TypeError::new("Not a number"))?
                                .value_of();
                            if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                                return Err(JsValue::from(TypeError::new("Not an u8 number")));
                            }
                            let v = v as u8;
                            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                            };
                            match custom_from_rs_u8(v) {
                                Ok(val) => val,
                                Err(err) => {
                                    return Err(JsValue::from(TypeError::new(err.as_ref())))
                                }
                            }
                        };
                        converted.insert(key, value);
                    }
                    converted
                }
            };
            let is_recoverable = {
                let js_val = Reflect::get(&obj, &"isRecoverable".into())?;
                js_val
                    .dyn_into::<Boolean>()
                    .map_err(|_| TypeError::new("Not a boolean"))?
                    .value_of()
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
                let js_val = Reflect::get(&obj, &"handle".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u32 number")));
                    }
                    let v = v as u32;
                    v
                }
            };
            let claimer_user_id = {
                let js_val = Reflect::get(&obj, &"claimerUserId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                            libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let claimer_human_handle = {
                let js_val = Reflect::get(&obj, &"claimerHumanHandle".into())?;
                struct_human_handle_js_to_rs(js_val)?
            };
            Ok(
                libparsec::ShamirRecoveryClaimMaybeRecoverDeviceInfo::RecoverDevice {
                    handle,
                    claimer_user_id,
                    claimer_human_handle,
                },
            )
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a ShamirRecoveryClaimMaybeRecoverDeviceInfo",
        ))),
    }
}

#[allow(dead_code)]
fn variant_shamir_recovery_claim_maybe_recover_device_info_rs_to_js(
    rs_obj: libparsec::ShamirRecoveryClaimMaybeRecoverDeviceInfo,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient".into(),
            )?;
            let js_handle = JsValue::from(handle);
            Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
            let js_claimer_user_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(claimer_user_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"claimerUserId".into(), &js_claimer_user_id)?;
            let js_claimer_human_handle = struct_human_handle_rs_to_js(claimer_human_handle)?;
            Reflect::set(
                &js_obj,
                &"claimerHumanHandle".into(),
                &js_claimer_human_handle,
            )?;
            let js_shamir_recovery_created_on = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(shamir_recovery_created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(
                &js_obj,
                &"shamirRecoveryCreatedOn".into(),
                &js_shamir_recovery_created_on,
            )?;
            let js_recipients = {
                // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                let js_array = Array::new_with_length(recipients.len() as u32);
                for (i, elem) in recipients.into_iter().enumerate() {
                    let js_elem = struct_shamir_recovery_recipient_rs_to_js(elem)?;
                    js_array.set(i as u32, js_elem);
                }
                js_array.into()
            };
            Reflect::set(&js_obj, &"recipients".into(), &js_recipients)?;
            let js_threshold = {
                let custom_to_rs_u8 =
                    |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                let v = match custom_to_rs_u8(threshold) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"threshold".into(), &js_threshold)?;
            let js_recovered_shares = {
                let js_map = Map::new();
                for (key, value) in recovered_shares.into_iter() {
                    let js_key = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(key) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    let js_value = {
                        let custom_to_rs_u8 =
                            |x: std::num::NonZeroU8| -> Result<u8, &'static str> { Ok(x.get()) };
                        let v = match custom_to_rs_u8(value) {
                            Ok(ok) => ok,
                            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                        };
                        JsValue::from(v)
                    };
                    js_map.set(&js_key, &js_value);
                }
                js_map.into()
            };
            Reflect::set(&js_obj, &"recoveredShares".into(), &js_recovered_shares)?;
            let js_is_recoverable = is_recoverable.into();
            Reflect::set(&js_obj, &"isRecoverable".into(), &js_is_recoverable)?;
        }
        libparsec::ShamirRecoveryClaimMaybeRecoverDeviceInfo::RecoverDevice {
            handle,
            claimer_user_id,
            claimer_human_handle,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice".into(),
            )?;
            let js_handle = JsValue::from(handle);
            Reflect::set(&js_obj, &"handle".into(), &js_handle)?;
            let js_claimer_user_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(claimer_user_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"claimerUserId".into(), &js_claimer_user_id)?;
            let js_claimer_human_handle = struct_human_handle_rs_to_js(claimer_human_handle)?;
            Reflect::set(
                &js_obj,
                &"claimerHumanHandle".into(),
                &js_claimer_human_handle,
            )?;
        }
    }
    Ok(js_obj)
}

// ShamirRecoveryClaimPickRecipientError

#[allow(dead_code)]
fn variant_shamir_recovery_claim_pick_recipient_error_rs_to_js(
    rs_obj: libparsec::ShamirRecoveryClaimPickRecipientError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ShamirRecoveryClaimPickRecipientError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimPickRecipientErrorInternal".into(),
            )?;
        }
        libparsec::ShamirRecoveryClaimPickRecipientError::RecipientAlreadyPicked { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimPickRecipientErrorRecipientAlreadyPicked".into(),
            )?;
        }
        libparsec::ShamirRecoveryClaimPickRecipientError::RecipientNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimPickRecipientErrorRecipientNotFound".into(),
            )?;
        }
        libparsec::ShamirRecoveryClaimPickRecipientError::RecipientRevoked { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimPickRecipientErrorRecipientRevoked".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ShamirRecoveryClaimRecoverDeviceError

#[allow(dead_code)]
fn variant_shamir_recovery_claim_recover_device_error_rs_to_js(
    rs_obj: libparsec::ShamirRecoveryClaimRecoverDeviceError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ShamirRecoveryClaimRecoverDeviceError::AlreadyUsed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimRecoverDeviceErrorAlreadyUsed".into(),
            )?;
        }
        libparsec::ShamirRecoveryClaimRecoverDeviceError::CipheredDataNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimRecoverDeviceErrorCipheredDataNotFound".into(),
            )?;
        }
        libparsec::ShamirRecoveryClaimRecoverDeviceError::CorruptedCipheredData { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimRecoverDeviceErrorCorruptedCipheredData".into(),
            )?;
        }
        libparsec::ShamirRecoveryClaimRecoverDeviceError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimRecoverDeviceErrorInternal".into(),
            )?;
        }
        libparsec::ShamirRecoveryClaimRecoverDeviceError::NotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimRecoverDeviceErrorNotFound".into(),
            )?;
        }
        libparsec::ShamirRecoveryClaimRecoverDeviceError::OrganizationExpired { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimRecoverDeviceErrorOrganizationExpired".into(),
            )?;
        }
        libparsec::ShamirRecoveryClaimRecoverDeviceError::RegisterNewDeviceError { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShamirRecoveryClaimRecoverDeviceErrorRegisterNewDeviceError".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// ShowCertificateSelectionDialogError

#[allow(dead_code)]
fn variant_show_certificate_selection_dialog_error_rs_to_js(
    rs_obj: libparsec::ShowCertificateSelectionDialogError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::ShowCertificateSelectionDialogError::CannotGetCertificateInfo { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShowCertificateSelectionDialogErrorCannotGetCertificateInfo".into(),
            )?;
        }
        libparsec::ShowCertificateSelectionDialogError::CannotOpenStore { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"ShowCertificateSelectionDialogErrorCannotOpenStore".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// TestbedError

#[allow(dead_code)]
fn variant_testbed_error_rs_to_js(rs_obj: libparsec::TestbedError) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::TestbedError::Disabled { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"TestbedErrorDisabled".into())?;
        }
        libparsec::TestbedError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"TestbedErrorInternal".into())?;
        }
    }
    Ok(js_obj)
}

// UpdateDeviceError

#[allow(dead_code)]
fn variant_update_device_error_rs_to_js(
    rs_obj: libparsec::UpdateDeviceError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::UpdateDeviceError::DecryptionFailed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"UpdateDeviceErrorDecryptionFailed".into(),
            )?;
        }
        libparsec::UpdateDeviceError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"UpdateDeviceErrorInternal".into())?;
        }
        libparsec::UpdateDeviceError::InvalidData { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"UpdateDeviceErrorInvalidData".into(),
            )?;
        }
        libparsec::UpdateDeviceError::InvalidPath { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"UpdateDeviceErrorInvalidPath".into(),
            )?;
        }
        libparsec::UpdateDeviceError::RemoteOpaqueKeyOperationFailed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"UpdateDeviceErrorRemoteOpaqueKeyOperationFailed".into(),
            )?;
        }
        libparsec::UpdateDeviceError::RemoteOpaqueKeyOperationOffline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"UpdateDeviceErrorRemoteOpaqueKeyOperationOffline".into(),
            )?;
        }
        libparsec::UpdateDeviceError::StorageNotAvailable { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"UpdateDeviceErrorStorageNotAvailable".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// UserClaimListInitialInfosError

#[allow(dead_code)]
fn variant_user_claim_list_initial_infos_error_rs_to_js(
    rs_obj: libparsec::UserClaimListInitialInfosError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::UserClaimListInitialInfosError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"UserClaimListInitialInfosErrorInternal".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WaitForDeviceAvailableError

#[allow(dead_code)]
fn variant_wait_for_device_available_error_rs_to_js(
    rs_obj: libparsec::WaitForDeviceAvailableError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WaitForDeviceAvailableError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WaitForDeviceAvailableErrorInternal".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceCreateFileError

#[allow(dead_code)]
fn variant_workspace_create_file_error_rs_to_js(
    rs_obj: libparsec::WorkspaceCreateFileError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceCreateFileError::EntryExists { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFileErrorEntryExists".into(),
            )?;
        }
        libparsec::WorkspaceCreateFileError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFileErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceCreateFileError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFileErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceCreateFileError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFileErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceCreateFileError::InvalidManifest { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFileErrorInvalidManifest".into(),
            )?;
        }
        libparsec::WorkspaceCreateFileError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFileErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceCreateFileError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFileErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceCreateFileError::ParentNotAFolder { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFileErrorParentNotAFolder".into(),
            )?;
        }
        libparsec::WorkspaceCreateFileError::ParentNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFileErrorParentNotFound".into(),
            )?;
        }
        libparsec::WorkspaceCreateFileError::ReadOnlyRealm { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFileErrorReadOnlyRealm".into(),
            )?;
        }
        libparsec::WorkspaceCreateFileError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFileErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceCreateFolderError

#[allow(dead_code)]
fn variant_workspace_create_folder_error_rs_to_js(
    rs_obj: libparsec::WorkspaceCreateFolderError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceCreateFolderError::EntryExists { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFolderErrorEntryExists".into(),
            )?;
        }
        libparsec::WorkspaceCreateFolderError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFolderErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceCreateFolderError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFolderErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceCreateFolderError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFolderErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceCreateFolderError::InvalidManifest { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFolderErrorInvalidManifest".into(),
            )?;
        }
        libparsec::WorkspaceCreateFolderError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFolderErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceCreateFolderError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFolderErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceCreateFolderError::ParentNotAFolder { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFolderErrorParentNotAFolder".into(),
            )?;
        }
        libparsec::WorkspaceCreateFolderError::ParentNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFolderErrorParentNotFound".into(),
            )?;
        }
        libparsec::WorkspaceCreateFolderError::ReadOnlyRealm { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFolderErrorReadOnlyRealm".into(),
            )?;
        }
        libparsec::WorkspaceCreateFolderError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceCreateFolderErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceDecryptPathAddrError

#[allow(dead_code)]
fn variant_workspace_decrypt_path_addr_error_rs_to_js(
    rs_obj: libparsec::WorkspaceDecryptPathAddrError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceDecryptPathAddrError::CorruptedData { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceDecryptPathAddrErrorCorruptedData".into(),
            )?;
        }
        libparsec::WorkspaceDecryptPathAddrError::CorruptedKey { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceDecryptPathAddrErrorCorruptedKey".into(),
            )?;
        }
        libparsec::WorkspaceDecryptPathAddrError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceDecryptPathAddrErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceDecryptPathAddrError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceDecryptPathAddrErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceDecryptPathAddrError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceDecryptPathAddrErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceDecryptPathAddrError::KeyNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceDecryptPathAddrErrorKeyNotFound".into(),
            )?;
        }
        libparsec::WorkspaceDecryptPathAddrError::NotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceDecryptPathAddrErrorNotAllowed".into(),
            )?;
        }
        libparsec::WorkspaceDecryptPathAddrError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceDecryptPathAddrErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceDecryptPathAddrError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceDecryptPathAddrErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceFdCloseError

#[allow(dead_code)]
fn variant_workspace_fd_close_error_rs_to_js(
    rs_obj: libparsec::WorkspaceFdCloseError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceFdCloseError::BadFileDescriptor { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdCloseErrorBadFileDescriptor".into(),
            )?;
        }
        libparsec::WorkspaceFdCloseError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdCloseErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceFdCloseError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdCloseErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceFdFlushError

#[allow(dead_code)]
fn variant_workspace_fd_flush_error_rs_to_js(
    rs_obj: libparsec::WorkspaceFdFlushError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceFdFlushError::BadFileDescriptor { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdFlushErrorBadFileDescriptor".into(),
            )?;
        }
        libparsec::WorkspaceFdFlushError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdFlushErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceFdFlushError::NotInWriteMode { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdFlushErrorNotInWriteMode".into(),
            )?;
        }
        libparsec::WorkspaceFdFlushError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdFlushErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceFdReadError

#[allow(dead_code)]
fn variant_workspace_fd_read_error_rs_to_js(
    rs_obj: libparsec::WorkspaceFdReadError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceFdReadError::BadFileDescriptor { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdReadErrorBadFileDescriptor".into(),
            )?;
        }
        libparsec::WorkspaceFdReadError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdReadErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceFdReadError::InvalidBlockAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdReadErrorInvalidBlockAccess".into(),
            )?;
        }
        libparsec::WorkspaceFdReadError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdReadErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceFdReadError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdReadErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceFdReadError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdReadErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceFdReadError::NotInReadMode { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdReadErrorNotInReadMode".into(),
            )?;
        }
        libparsec::WorkspaceFdReadError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdReadErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceFdReadError::ServerBlockstoreUnavailable { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdReadErrorServerBlockstoreUnavailable".into(),
            )?;
        }
        libparsec::WorkspaceFdReadError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdReadErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceFdResizeError

#[allow(dead_code)]
fn variant_workspace_fd_resize_error_rs_to_js(
    rs_obj: libparsec::WorkspaceFdResizeError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceFdResizeError::BadFileDescriptor { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdResizeErrorBadFileDescriptor".into(),
            )?;
        }
        libparsec::WorkspaceFdResizeError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdResizeErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceFdResizeError::NotInWriteMode { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdResizeErrorNotInWriteMode".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceFdStatError

#[allow(dead_code)]
fn variant_workspace_fd_stat_error_rs_to_js(
    rs_obj: libparsec::WorkspaceFdStatError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceFdStatError::BadFileDescriptor { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdStatErrorBadFileDescriptor".into(),
            )?;
        }
        libparsec::WorkspaceFdStatError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdStatErrorInternal".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceFdWriteError

#[allow(dead_code)]
fn variant_workspace_fd_write_error_rs_to_js(
    rs_obj: libparsec::WorkspaceFdWriteError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceFdWriteError::BadFileDescriptor { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdWriteErrorBadFileDescriptor".into(),
            )?;
        }
        libparsec::WorkspaceFdWriteError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdWriteErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceFdWriteError::NotInWriteMode { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceFdWriteErrorNotInWriteMode".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceGeneratePathAddrError

#[allow(dead_code)]
fn variant_workspace_generate_path_addr_error_rs_to_js(
    rs_obj: libparsec::WorkspaceGeneratePathAddrError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceGeneratePathAddrError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceGeneratePathAddrErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceGeneratePathAddrError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceGeneratePathAddrErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceGeneratePathAddrError::NoKey { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceGeneratePathAddrErrorNoKey".into(),
            )?;
        }
        libparsec::WorkspaceGeneratePathAddrError::NotAllowed { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceGeneratePathAddrErrorNotAllowed".into(),
            )?;
        }
        libparsec::WorkspaceGeneratePathAddrError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceGeneratePathAddrErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceGeneratePathAddrError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceGeneratePathAddrErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryEntryStat

#[allow(dead_code)]
fn variant_workspace_history_entry_stat_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::WorkspaceHistoryEntryStat, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "WorkspaceHistoryEntryStatFile" => {
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
                    })?
            };
            let parent = {
                let js_val = Reflect::get(&obj, &"parent".into())?;
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
                    })?
            };
            let created = {
                let js_val = Reflect::get(&obj, &"created".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
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
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let version = {
                let js_val = Reflect::get(&obj, &"version".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u32 number")));
                    }
                    let v = v as u32;
                    v
                }
            };
            let size = {
                let js_val = Reflect::get(&obj, &"size".into())?;
                {
                    let v = u64::try_from(js_val)
                        .map_err(|_| TypeError::new("Not a BigInt representing an u64 number"))?;
                    v
                }
            };
            let last_updater = {
                let js_val = Reflect::get(&obj, &"lastUpdater".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            Ok(libparsec::WorkspaceHistoryEntryStat::File {
                id,
                parent,
                created,
                updated,
                version,
                size,
                last_updater,
            })
        }
        "WorkspaceHistoryEntryStatFolder" => {
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
                    })?
            };
            let parent = {
                let js_val = Reflect::get(&obj, &"parent".into())?;
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
                    })?
            };
            let created = {
                let js_val = Reflect::get(&obj, &"created".into())?;
                {
                    let v = js_val.dyn_into::<Number>()?.value_of();
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
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
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    let v = custom_from_rs_f64(v).map_err(|e| TypeError::new(e.as_ref()))?;
                    v
                }
            };
            let version = {
                let js_val = Reflect::get(&obj, &"version".into())?;
                {
                    let v = js_val
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u32::MIN as f64) || (u32::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u32 number")));
                    }
                    let v = v as u32;
                    v
                }
            };
            let last_updater = {
                let js_val = Reflect::get(&obj, &"lastUpdater".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            Ok(libparsec::WorkspaceHistoryEntryStat::Folder {
                id,
                parent,
                created,
                updated,
                version,
                last_updater,
            })
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a WorkspaceHistoryEntryStat",
        ))),
    }
}

#[allow(dead_code)]
fn variant_workspace_history_entry_stat_rs_to_js(
    rs_obj: libparsec::WorkspaceHistoryEntryStat,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::WorkspaceHistoryEntryStat::File {
            id,
            parent,
            created,
            updated,
            version,
            size,
            last_updater,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryEntryStatFile".into(),
            )?;
            let js_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"id".into(), &js_id)?;
            let js_parent = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(parent) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"parent".into(), &js_parent)?;
            let js_created = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(updated) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"updated".into(), &js_updated)?;
            let js_version = JsValue::from(version);
            Reflect::set(&js_obj, &"version".into(), &js_version)?;
            let js_size = JsValue::from(size);
            Reflect::set(&js_obj, &"size".into(), &js_size)?;
            let js_last_updater = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(last_updater) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"lastUpdater".into(), &js_last_updater)?;
        }
        libparsec::WorkspaceHistoryEntryStat::Folder {
            id,
            parent,
            created,
            updated,
            version,
            last_updater,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryEntryStatFolder".into(),
            )?;
            let js_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"id".into(), &js_id)?;
            let js_parent = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(parent) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"parent".into(), &js_parent)?;
            let js_created = {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
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
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                let v = match custom_to_rs_f64(updated) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                };
                JsValue::from(v)
            };
            Reflect::set(&js_obj, &"updated".into(), &js_updated)?;
            let js_version = JsValue::from(version);
            Reflect::set(&js_obj, &"version".into(), &js_version)?;
            let js_last_updater = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(last_updater) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(&js_obj, &"lastUpdater".into(), &js_last_updater)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryFdCloseError

#[allow(dead_code)]
fn variant_workspace_history_fd_close_error_rs_to_js(
    rs_obj: libparsec::WorkspaceHistoryFdCloseError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceHistoryFdCloseError::BadFileDescriptor { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryFdCloseErrorBadFileDescriptor".into(),
            )?;
        }
        libparsec::WorkspaceHistoryFdCloseError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryFdCloseErrorInternal".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryFdReadError

#[allow(dead_code)]
fn variant_workspace_history_fd_read_error_rs_to_js(
    rs_obj: libparsec::WorkspaceHistoryFdReadError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceHistoryFdReadError::BadFileDescriptor { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryFdReadErrorBadFileDescriptor".into(),
            )?;
        }
        libparsec::WorkspaceHistoryFdReadError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryFdReadErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceHistoryFdReadError::InvalidBlockAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryFdReadErrorInvalidBlockAccess".into(),
            )?;
        }
        libparsec::WorkspaceHistoryFdReadError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryFdReadErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceHistoryFdReadError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryFdReadErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceHistoryFdReadError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryFdReadErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceHistoryFdReadError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryFdReadErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceHistoryFdReadError::ServerBlockstoreUnavailable { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryFdReadErrorServerBlockstoreUnavailable".into(),
            )?;
        }
        libparsec::WorkspaceHistoryFdReadError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryFdReadErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryFdStatError

#[allow(dead_code)]
fn variant_workspace_history_fd_stat_error_rs_to_js(
    rs_obj: libparsec::WorkspaceHistoryFdStatError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceHistoryFdStatError::BadFileDescriptor { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryFdStatErrorBadFileDescriptor".into(),
            )?;
        }
        libparsec::WorkspaceHistoryFdStatError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryFdStatErrorInternal".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryInternalOnlyError

#[allow(dead_code)]
fn variant_workspace_history_internal_only_error_rs_to_js(
    rs_obj: libparsec::WorkspaceHistoryInternalOnlyError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceHistoryInternalOnlyError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryInternalOnlyErrorInternal".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryOpenFileError

#[allow(dead_code)]
fn variant_workspace_history_open_file_error_rs_to_js(
    rs_obj: libparsec::WorkspaceHistoryOpenFileError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceHistoryOpenFileError::EntryNotAFile { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryOpenFileErrorEntryNotAFile".into(),
            )?;
        }
        libparsec::WorkspaceHistoryOpenFileError::EntryNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryOpenFileErrorEntryNotFound".into(),
            )?;
        }
        libparsec::WorkspaceHistoryOpenFileError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryOpenFileErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceHistoryOpenFileError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryOpenFileErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceHistoryOpenFileError::InvalidHistory { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryOpenFileErrorInvalidHistory".into(),
            )?;
        }
        libparsec::WorkspaceHistoryOpenFileError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryOpenFileErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceHistoryOpenFileError::InvalidManifest { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryOpenFileErrorInvalidManifest".into(),
            )?;
        }
        libparsec::WorkspaceHistoryOpenFileError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryOpenFileErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceHistoryOpenFileError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryOpenFileErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceHistoryOpenFileError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryOpenFileErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryRealmExportDecryptor

#[allow(dead_code)]
fn variant_workspace_history_realm_export_decryptor_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::WorkspaceHistoryRealmExportDecryptor, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "WorkspaceHistoryRealmExportDecryptorSequesterService" => {
            let sequester_service_id = {
                let js_val = Reflect::get(&obj, &"sequesterServiceId".into())?;
                js_val
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string =
                            |s: String| -> Result<libparsec::SequesterServiceID, _> {
                                libparsec::SequesterServiceID::from_hex(s.as_str())
                                    .map_err(|e| e.to_string())
                            };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            let private_key_pem_path = {
                let js_val = Reflect::get(&obj, &"privateKeyPemPath".into())?;
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
                    })?
            };
            Ok(
                libparsec::WorkspaceHistoryRealmExportDecryptor::SequesterService {
                    sequester_service_id,
                    private_key_pem_path,
                },
            )
        }
        "WorkspaceHistoryRealmExportDecryptorUser" => {
            let access = {
                let js_val = Reflect::get(&obj, &"access".into())?;
                variant_device_access_strategy_js_to_rs(js_val)?
            };
            Ok(libparsec::WorkspaceHistoryRealmExportDecryptor::User { access })
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a WorkspaceHistoryRealmExportDecryptor",
        ))),
    }
}

#[allow(dead_code)]
fn variant_workspace_history_realm_export_decryptor_rs_to_js(
    rs_obj: libparsec::WorkspaceHistoryRealmExportDecryptor,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::WorkspaceHistoryRealmExportDecryptor::SequesterService {
            sequester_service_id,
            private_key_pem_path,
            ..
        } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryRealmExportDecryptorSequesterService".into(),
            )?;
            let js_sequester_service_id = JsValue::from_str({
                let custom_to_rs_string =
                    |x: libparsec::SequesterServiceID| -> Result<String, &'static str> {
                        Ok(x.hex())
                    };
                match custom_to_rs_string(sequester_service_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(
                &js_obj,
                &"sequesterServiceId".into(),
                &js_sequester_service_id,
            )?;
            let js_private_key_pem_path = JsValue::from_str({
                let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                };
                match custom_to_rs_string(private_key_pem_path) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                }
                .as_ref()
            });
            Reflect::set(
                &js_obj,
                &"privateKeyPemPath".into(),
                &js_private_key_pem_path,
            )?;
        }
        libparsec::WorkspaceHistoryRealmExportDecryptor::User { access, .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryRealmExportDecryptorUser".into(),
            )?;
            let js_access = variant_device_access_strategy_rs_to_js(access)?;
            Reflect::set(&js_obj, &"access".into(), &js_access)?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistorySetTimestampOfInterestError

#[allow(dead_code)]
fn variant_workspace_history_set_timestamp_of_interest_error_rs_to_js(
    rs_obj: libparsec::WorkspaceHistorySetTimestampOfInterestError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceHistorySetTimestampOfInterestError::EntryNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistorySetTimestampOfInterestErrorEntryNotFound".into(),
            )?;
        }
        libparsec::WorkspaceHistorySetTimestampOfInterestError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistorySetTimestampOfInterestErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceHistorySetTimestampOfInterestError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistorySetTimestampOfInterestErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceHistorySetTimestampOfInterestError::InvalidHistory { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistorySetTimestampOfInterestErrorInvalidHistory".into(),
            )?;
        }
        libparsec::WorkspaceHistorySetTimestampOfInterestError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistorySetTimestampOfInterestErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceHistorySetTimestampOfInterestError::InvalidManifest { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistorySetTimestampOfInterestErrorInvalidManifest".into(),
            )?;
        }
        libparsec::WorkspaceHistorySetTimestampOfInterestError::NewerThanHigherBound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistorySetTimestampOfInterestErrorNewerThanHigherBound".into(),
            )?;
        }
        libparsec::WorkspaceHistorySetTimestampOfInterestError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistorySetTimestampOfInterestErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceHistorySetTimestampOfInterestError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistorySetTimestampOfInterestErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceHistorySetTimestampOfInterestError::OlderThanLowerBound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistorySetTimestampOfInterestErrorOlderThanLowerBound".into(),
            )?;
        }
        libparsec::WorkspaceHistorySetTimestampOfInterestError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistorySetTimestampOfInterestErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryStartError

#[allow(dead_code)]
fn variant_workspace_history_start_error_rs_to_js(
    rs_obj: libparsec::WorkspaceHistoryStartError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceHistoryStartError::CannotOpenRealmExportDatabase { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStartErrorCannotOpenRealmExportDatabase".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStartError::IncompleteRealmExportDatabase { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStartErrorIncompleteRealmExportDatabase".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStartError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStartErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStartError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStartErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStartError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStartErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStartError::InvalidManifest { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStartErrorInvalidManifest".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStartError::InvalidRealmExportDatabase { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStartErrorInvalidRealmExportDatabase".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStartError::NoHistory { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStartErrorNoHistory".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStartError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStartErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStartError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStartErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStartError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStartErrorStopped".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStartError::UnsupportedRealmExportDatabaseVersion { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStartErrorUnsupportedRealmExportDatabaseVersion".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryStatEntryError

#[allow(dead_code)]
fn variant_workspace_history_stat_entry_error_rs_to_js(
    rs_obj: libparsec::WorkspaceHistoryStatEntryError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceHistoryStatEntryError::EntryNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatEntryErrorEntryNotFound".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatEntryError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatEntryErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatEntryError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatEntryErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatEntryError::InvalidHistory { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatEntryErrorInvalidHistory".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatEntryError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatEntryErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatEntryError::InvalidManifest { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatEntryErrorInvalidManifest".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatEntryError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatEntryErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatEntryError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatEntryErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatEntryError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatEntryErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceHistoryStatFolderChildrenError

#[allow(dead_code)]
fn variant_workspace_history_stat_folder_children_error_rs_to_js(
    rs_obj: libparsec::WorkspaceHistoryStatFolderChildrenError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceHistoryStatFolderChildrenError::EntryIsFile { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatFolderChildrenErrorEntryIsFile".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::EntryNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatFolderChildrenErrorEntryNotFound".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatFolderChildrenErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatFolderChildrenErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::InvalidHistory { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatFolderChildrenErrorInvalidHistory".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatFolderChildrenErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::InvalidManifest { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatFolderChildrenErrorInvalidManifest".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatFolderChildrenErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatFolderChildrenErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceHistoryStatFolderChildrenError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceHistoryStatFolderChildrenErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceInfoError

#[allow(dead_code)]
fn variant_workspace_info_error_rs_to_js(
    rs_obj: libparsec::WorkspaceInfoError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceInfoError::Internal { .. } => {
            Reflect::set(&js_obj, &"tag".into(), &"WorkspaceInfoErrorInternal".into())?;
        }
    }
    Ok(js_obj)
}

// WorkspaceIsFileContentLocalError

#[allow(dead_code)]
fn variant_workspace_is_file_content_local_error_rs_to_js(
    rs_obj: libparsec::WorkspaceIsFileContentLocalError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceIsFileContentLocalError::EntryNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceIsFileContentLocalErrorEntryNotFound".into(),
            )?;
        }
        libparsec::WorkspaceIsFileContentLocalError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceIsFileContentLocalErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceIsFileContentLocalError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceIsFileContentLocalErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceIsFileContentLocalError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceIsFileContentLocalErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceIsFileContentLocalError::InvalidManifest { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceIsFileContentLocalErrorInvalidManifest".into(),
            )?;
        }
        libparsec::WorkspaceIsFileContentLocalError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceIsFileContentLocalErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceIsFileContentLocalError::NotAFile { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceIsFileContentLocalErrorNotAFile".into(),
            )?;
        }
        libparsec::WorkspaceIsFileContentLocalError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceIsFileContentLocalErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceIsFileContentLocalError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceIsFileContentLocalErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceMountError

#[allow(dead_code)]
fn variant_workspace_mount_error_rs_to_js(
    rs_obj: libparsec::WorkspaceMountError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceMountError::Disabled { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceMountErrorDisabled".into(),
            )?;
        }
        libparsec::WorkspaceMountError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceMountErrorInternal".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceMoveEntryError

#[allow(dead_code)]
fn variant_workspace_move_entry_error_rs_to_js(
    rs_obj: libparsec::WorkspaceMoveEntryError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceMoveEntryError::CannotMoveRoot { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceMoveEntryErrorCannotMoveRoot".into(),
            )?;
        }
        libparsec::WorkspaceMoveEntryError::DestinationExists { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceMoveEntryErrorDestinationExists".into(),
            )?;
        }
        libparsec::WorkspaceMoveEntryError::DestinationNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceMoveEntryErrorDestinationNotFound".into(),
            )?;
        }
        libparsec::WorkspaceMoveEntryError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceMoveEntryErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceMoveEntryError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceMoveEntryErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceMoveEntryError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceMoveEntryErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceMoveEntryError::InvalidManifest { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceMoveEntryErrorInvalidManifest".into(),
            )?;
        }
        libparsec::WorkspaceMoveEntryError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceMoveEntryErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceMoveEntryError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceMoveEntryErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceMoveEntryError::ReadOnlyRealm { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceMoveEntryErrorReadOnlyRealm".into(),
            )?;
        }
        libparsec::WorkspaceMoveEntryError::SourceNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceMoveEntryErrorSourceNotFound".into(),
            )?;
        }
        libparsec::WorkspaceMoveEntryError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceMoveEntryErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceOpenFileError

#[allow(dead_code)]
fn variant_workspace_open_file_error_rs_to_js(
    rs_obj: libparsec::WorkspaceOpenFileError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceOpenFileError::EntryExistsInCreateNewMode { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceOpenFileErrorEntryExistsInCreateNewMode".into(),
            )?;
        }
        libparsec::WorkspaceOpenFileError::EntryNotAFile { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceOpenFileErrorEntryNotAFile".into(),
            )?;
        }
        libparsec::WorkspaceOpenFileError::EntryNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceOpenFileErrorEntryNotFound".into(),
            )?;
        }
        libparsec::WorkspaceOpenFileError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceOpenFileErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceOpenFileError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceOpenFileErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceOpenFileError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceOpenFileErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceOpenFileError::InvalidManifest { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceOpenFileErrorInvalidManifest".into(),
            )?;
        }
        libparsec::WorkspaceOpenFileError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceOpenFileErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceOpenFileError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceOpenFileErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceOpenFileError::ReadOnlyRealm { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceOpenFileErrorReadOnlyRealm".into(),
            )?;
        }
        libparsec::WorkspaceOpenFileError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceOpenFileErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceRemoveEntryError

#[allow(dead_code)]
fn variant_workspace_remove_entry_error_rs_to_js(
    rs_obj: libparsec::WorkspaceRemoveEntryError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceRemoveEntryError::CannotRemoveRoot { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceRemoveEntryErrorCannotRemoveRoot".into(),
            )?;
        }
        libparsec::WorkspaceRemoveEntryError::EntryIsFile { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceRemoveEntryErrorEntryIsFile".into(),
            )?;
        }
        libparsec::WorkspaceRemoveEntryError::EntryIsFolder { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceRemoveEntryErrorEntryIsFolder".into(),
            )?;
        }
        libparsec::WorkspaceRemoveEntryError::EntryIsNonEmptyFolder { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceRemoveEntryErrorEntryIsNonEmptyFolder".into(),
            )?;
        }
        libparsec::WorkspaceRemoveEntryError::EntryNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceRemoveEntryErrorEntryNotFound".into(),
            )?;
        }
        libparsec::WorkspaceRemoveEntryError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceRemoveEntryErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceRemoveEntryError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceRemoveEntryErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceRemoveEntryError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceRemoveEntryErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceRemoveEntryError::InvalidManifest { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceRemoveEntryErrorInvalidManifest".into(),
            )?;
        }
        libparsec::WorkspaceRemoveEntryError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceRemoveEntryErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceRemoveEntryError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceRemoveEntryErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceRemoveEntryError::ReadOnlyRealm { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceRemoveEntryErrorReadOnlyRealm".into(),
            )?;
        }
        libparsec::WorkspaceRemoveEntryError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceRemoveEntryErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceStatEntryError

#[allow(dead_code)]
fn variant_workspace_stat_entry_error_rs_to_js(
    rs_obj: libparsec::WorkspaceStatEntryError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceStatEntryError::EntryNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatEntryErrorEntryNotFound".into(),
            )?;
        }
        libparsec::WorkspaceStatEntryError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatEntryErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceStatEntryError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatEntryErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceStatEntryError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatEntryErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceStatEntryError::InvalidManifest { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatEntryErrorInvalidManifest".into(),
            )?;
        }
        libparsec::WorkspaceStatEntryError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatEntryErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceStatEntryError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatEntryErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceStatEntryError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatEntryErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceStatFolderChildrenError

#[allow(dead_code)]
fn variant_workspace_stat_folder_children_error_rs_to_js(
    rs_obj: libparsec::WorkspaceStatFolderChildrenError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceStatFolderChildrenError::EntryIsFile { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatFolderChildrenErrorEntryIsFile".into(),
            )?;
        }
        libparsec::WorkspaceStatFolderChildrenError::EntryNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatFolderChildrenErrorEntryNotFound".into(),
            )?;
        }
        libparsec::WorkspaceStatFolderChildrenError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatFolderChildrenErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceStatFolderChildrenError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatFolderChildrenErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceStatFolderChildrenError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatFolderChildrenErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceStatFolderChildrenError::InvalidManifest { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatFolderChildrenErrorInvalidManifest".into(),
            )?;
        }
        libparsec::WorkspaceStatFolderChildrenError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatFolderChildrenErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceStatFolderChildrenError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatFolderChildrenErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceStatFolderChildrenError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStatFolderChildrenErrorStopped".into(),
            )?;
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
            Reflect::set(&js_obj, &"tag".into(), &"WorkspaceStopErrorInternal".into())?;
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
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "WorkspaceStorageCacheSizeCustom" => {
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
                    let v = v as u32;
                    v
                }
            };
            Ok(libparsec::WorkspaceStorageCacheSize::Custom { size })
        }
        "WorkspaceStorageCacheSizeDefault" => Ok(libparsec::WorkspaceStorageCacheSize::Default {}),
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
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStorageCacheSizeCustom".into(),
            )?;
            let js_size = JsValue::from(size);
            Reflect::set(&js_obj, &"size".into(), &js_size)?;
        }
        libparsec::WorkspaceStorageCacheSize::Default { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceStorageCacheSizeDefault".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// WorkspaceWatchEntryOneShotError

#[allow(dead_code)]
fn variant_workspace_watch_entry_one_shot_error_rs_to_js(
    rs_obj: libparsec::WorkspaceWatchEntryOneShotError,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    let js_display = &rs_obj.to_string();
    Reflect::set(&js_obj, &"error".into(), &js_display.into())?;
    match rs_obj {
        libparsec::WorkspaceWatchEntryOneShotError::EntryNotFound { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceWatchEntryOneShotErrorEntryNotFound".into(),
            )?;
        }
        libparsec::WorkspaceWatchEntryOneShotError::Internal { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceWatchEntryOneShotErrorInternal".into(),
            )?;
        }
        libparsec::WorkspaceWatchEntryOneShotError::InvalidCertificate { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceWatchEntryOneShotErrorInvalidCertificate".into(),
            )?;
        }
        libparsec::WorkspaceWatchEntryOneShotError::InvalidKeysBundle { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceWatchEntryOneShotErrorInvalidKeysBundle".into(),
            )?;
        }
        libparsec::WorkspaceWatchEntryOneShotError::InvalidManifest { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceWatchEntryOneShotErrorInvalidManifest".into(),
            )?;
        }
        libparsec::WorkspaceWatchEntryOneShotError::NoRealmAccess { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceWatchEntryOneShotErrorNoRealmAccess".into(),
            )?;
        }
        libparsec::WorkspaceWatchEntryOneShotError::Offline { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceWatchEntryOneShotErrorOffline".into(),
            )?;
        }
        libparsec::WorkspaceWatchEntryOneShotError::Stopped { .. } => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"WorkspaceWatchEntryOneShotErrorStopped".into(),
            )?;
        }
    }
    Ok(js_obj)
}

// X509URIFlavorValue

#[allow(dead_code)]
fn variant_x509_uri_flavor_value_js_to_rs(
    obj: JsValue,
) -> Result<libparsec::X509URIFlavorValue, JsValue> {
    let tag = Reflect::get(&obj, &"tag".into())?;
    let tag = tag
        .as_string()
        .ok_or_else(|| JsValue::from(TypeError::new("tag isn't a string")))?;
    match tag.as_str() {
        "X509URIFlavorValuePKCS11" => {
            let x1 = {
                let js_val = Reflect::get(&obj, &"x1".into())?;
                {
                    let _ = js_val;
                    libparsec::X509Pkcs11URI
                }
            };
            Ok(libparsec::X509URIFlavorValue::PKCS11(x1))
        }
        "X509URIFlavorValueWindowsCNG" => {
            let x1 = {
                let js_val = Reflect::get(&obj, &"x1".into())?;
                js_val
                    .dyn_into::<Uint8Array>()
                    .map(|x| x.to_vec())
                    .map_err(|_| TypeError::new("Not a Uint8Array"))
                    .and_then(|x| {
                        let custom_from_rs_bytes = |v: &[u8]| -> Result<_, String> {
                            Ok(libparsec::Bytes::copy_from_slice(v).into())
                        };
                        custom_from_rs_bytes(&x).map_err(|e| TypeError::new(e.as_ref()))
                    })?
            };
            Ok(libparsec::X509URIFlavorValue::WindowsCNG(x1))
        }
        _ => Err(JsValue::from(TypeError::new(
            "Object is not a X509URIFlavorValue",
        ))),
    }
}

#[allow(dead_code)]
fn variant_x509_uri_flavor_value_rs_to_js(
    rs_obj: libparsec::X509URIFlavorValue,
) -> Result<JsValue, JsValue> {
    let js_obj = Object::new().into();
    match rs_obj {
        libparsec::X509URIFlavorValue::PKCS11(x1, ..) => {
            Reflect::set(&js_obj, &"tag".into(), &"X509URIFlavorValuePKCS11".into())?;
            let js_x1 = {
                let _ = x1;
                JsValue::UNDEFINED
            };
            Reflect::set(&js_obj, &"x1".into(), &js_x1.into())?;
        }
        libparsec::X509URIFlavorValue::WindowsCNG(x1, ..) => {
            Reflect::set(
                &js_obj,
                &"tag".into(),
                &"X509URIFlavorValueWindowsCNG".into(),
            )?;
            let js_x1 = JsValue::from(Uint8Array::from({
                let custom_to_rs_bytes =
                    |v: libparsec::X509WindowsCngURI| -> Result<Vec<u8>, String> { Ok(v.into()) };
                match custom_to_rs_bytes(x1) {
                    Ok(ok) => ok,
                    Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                }
                .as_ref()
            }));
            Reflect::set(&js_obj, &"x1".into(), &js_x1.into())?;
        }
    }
    Ok(js_obj)
}

// account_create_1_send_validation_email
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountCreate1SendValidationEmail(
    config_dir: String,
    addr: String,
    email: String,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config_dir = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(config_dir).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(addr).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let email = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::EmailAddress::from_str(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(email).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::account_create_1_send_validation_email(&config_dir, addr, email).await;
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
                let js_err = variant_account_create_send_validation_email_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_create_2_check_validation_code
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountCreate2CheckValidationCode(
    config_dir: String,
    addr: String,
    validation_code: String,
    email: String,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config_dir = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(config_dir).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(addr).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let email = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::EmailAddress::from_str(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(email).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::account_create_2_check_validation_code(
            &config_dir,
            addr,
            &validation_code,
            email,
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
                let js_err = variant_account_create_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_create_3_proceed
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountCreate3Proceed(
    config_dir: String,
    addr: String,
    validation_code: String,
    human_handle: Object,
    auth_method_strategy: Object,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config_dir = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(config_dir).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(addr).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let human_handle = human_handle.into();
        let human_handle = struct_human_handle_js_to_rs(human_handle)?;

        let auth_method_strategy = auth_method_strategy.into();
        let auth_method_strategy =
            variant_account_auth_method_strategy_js_to_rs(auth_method_strategy)?;

        let ret = libparsec::account_create_3_proceed(
            &config_dir,
            addr,
            &validation_code,
            human_handle,
            auth_method_strategy,
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
                let js_err = variant_account_create_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_create_auth_method
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountCreateAuthMethod(account: u32, auth_method_strategy: Object) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let auth_method_strategy = auth_method_strategy.into();
        let auth_method_strategy =
            variant_account_auth_method_strategy_js_to_rs(auth_method_strategy)?;

        let ret = libparsec::account_create_auth_method(account, auth_method_strategy).await;
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
                let js_err = variant_account_create_auth_method_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_create_registration_device
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountCreateRegistrationDevice(
    account: u32,
    existing_local_device_access: Object,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let existing_local_device_access = existing_local_device_access.into();
        let existing_local_device_access =
            variant_device_access_strategy_js_to_rs(existing_local_device_access)?;

        let ret =
            libparsec::account_create_registration_device(account, existing_local_device_access)
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
                let js_err = variant_account_create_registration_device_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_delete_1_send_validation_email
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountDelete1SendValidationEmail(account: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::account_delete_1_send_validation_email(account).await;
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
                let js_err = variant_account_delete_send_validation_email_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_delete_2_proceed
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountDelete2Proceed(account: u32, validation_code: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::account_delete_2_proceed(account, &validation_code).await;
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
                let js_err = variant_account_delete_proceed_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_disable_auth_method
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountDisableAuthMethod(account: u32, auth_method_id: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let auth_method_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::AccountAuthMethodID, _> {
                libparsec::AccountAuthMethodID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(auth_method_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::account_disable_auth_method(account, auth_method_id).await;
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
                let js_err = variant_account_disable_auth_method_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_info
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountInfo(account: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::account_info(account);
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_account_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_account_info_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_list_auth_methods
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountListAuthMethods(account: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::account_list_auth_methods(account).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(value.len() as u32);
                    for (i, elem) in value.into_iter().enumerate() {
                        let js_elem = struct_auth_method_info_rs_to_js(elem)?;
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
                let js_err = variant_account_list_auth_methods_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_list_invitations
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountListInvitations(account: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::account_list_invitations(account).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(value.len() as u32);
                    for (i, elem) in value.into_iter().enumerate() {
                        let js_elem = {
                            let (x1, x2, x3, x4) = elem;
                            // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                            let js_array = Array::new_with_length(4);
                            let js_value = JsValue::from_str({
                                let custom_to_rs_string = |addr: libparsec::ParsecInvitationAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) };
                                match custom_to_rs_string(x1) {
                                    Ok(ok) => ok,
                                    Err(err) => {
                                        return Err(JsValue::from(TypeError::new(&err.to_string())))
                                    }
                                }
                                .as_ref()
                            });
                            js_array.set(0, js_value);
                            let js_value = JsValue::from_str(x2.as_ref());
                            js_array.set(1, js_value);
                            let js_value = JsValue::from_str({
                                let custom_to_rs_string = |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                                match custom_to_rs_string(x3) {
                                    Ok(ok) => ok,
                                    Err(err) => {
                                        return Err(JsValue::from(TypeError::new(&err.to_string())))
                                    }
                                }
                                .as_ref()
                            });
                            js_array.set(2, js_value);
                            let js_value = JsValue::from_str(enum_invitation_type_rs_to_js(x4));
                            js_array.set(3, js_value);
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
                let js_err = variant_account_list_invitations_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_list_organizations
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountListOrganizations(account: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::account_list_organizations(account).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_account_organizations_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_account_list_organizations_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_list_registration_devices
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountListRegistrationDevices(account: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::account_list_registration_devices(account).await;
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
                            // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                            let js_array = Array::new_with_length(2);
                            let js_value = JsValue::from_str(x1.as_ref());
                            js_array.set(0, js_value);
                            let js_value = JsValue::from_str({
                                let custom_to_rs_string =
                                    |x: libparsec::UserID| -> Result<String, &'static str> {
                                        Ok(x.hex())
                                    };
                                match custom_to_rs_string(x2) {
                                    Ok(ok) => ok,
                                    Err(err) => {
                                        return Err(JsValue::from(TypeError::new(&err.to_string())))
                                    }
                                }
                                .as_ref()
                            });
                            js_array.set(1, js_value);
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
                let js_err = variant_account_list_registration_devices_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_login
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountLogin(config_dir: String, addr: String, login_strategy: Object) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config_dir = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(config_dir).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(addr).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let login_strategy = login_strategy.into();
        let login_strategy = variant_account_login_strategy_js_to_rs(login_strategy)?;

        let ret = libparsec::account_login(config_dir, addr, login_strategy).await;
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
                let js_err = variant_account_login_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_logout
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountLogout(account: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::account_logout(account);
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
                let js_err = variant_account_logout_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_recover_1_send_validation_email
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountRecover1SendValidationEmail(
    config_dir: String,
    addr: String,
    email: String,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config_dir = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(config_dir).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(addr).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let email = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::EmailAddress::from_str(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(email).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret =
            libparsec::account_recover_1_send_validation_email(&config_dir, addr, email).await;
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
                let js_err = variant_account_recover_send_validation_email_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_recover_2_proceed
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountRecover2Proceed(
    config_dir: String,
    addr: String,
    validation_code: String,
    email: String,
    auth_method_strategy: Object,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config_dir = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(config_dir).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(addr).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let email = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::EmailAddress::from_str(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(email).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let auth_method_strategy = auth_method_strategy.into();
        let auth_method_strategy =
            variant_account_auth_method_strategy_js_to_rs(auth_method_strategy)?;

        let ret = libparsec::account_recover_2_proceed(
            &config_dir,
            addr,
            &validation_code,
            email,
            auth_method_strategy,
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
                let js_err = variant_account_recover_proceed_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// account_register_new_device
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn accountRegisterNewDevice(
    account: u32,
    organization_id: String,
    user_id: String,
    new_device_label: String,
    save_strategy: Object,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let organization_id = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(organization_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let user_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(user_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let new_device_label = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(new_device_label).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let save_strategy = save_strategy.into();
        let save_strategy = variant_device_save_strategy_js_to_rs(save_strategy)?;

        let ret = libparsec::account_register_new_device(
            account,
            organization_id,
            user_id,
            new_device_label,
            save_strategy,
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
                let js_err = variant_account_register_new_device_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// archive_device
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn archiveDevice(config_dir: String, device_path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config_dir = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(config_dir).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let device_path = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(device_path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::archive_device(&config_dir, &device_path).await;
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
                let js_err = variant_archive_device_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// bootstrap_organization
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn bootstrapOrganization(
    config: Object,
    bootstrap_organization_addr: String,
    save_strategy: Object,
    human_handle: Object,
    device_label: String,
    sequester_authority_verify_key_pem: Option<String>,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config = config.into();
        let config = struct_client_config_js_to_rs(config)?;

        let bootstrap_organization_addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecOrganizationBootstrapAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(bootstrap_organization_addr)
                .map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let save_strategy = save_strategy.into();
        let save_strategy = variant_device_save_strategy_js_to_rs(save_strategy)?;

        let human_handle = human_handle.into();
        let human_handle = struct_human_handle_js_to_rs(human_handle)?;

        let device_label = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(device_label).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let sequester_authority_verify_key_pem = match sequester_authority_verify_key_pem {
            Some(sequester_authority_verify_key_pem) => Some(sequester_authority_verify_key_pem),
            None => None,
        };

        let ret = libparsec::bootstrap_organization(
            config,
            bootstrap_organization_addr,
            save_strategy,
            human_handle,
            device_label,
            sequester_authority_verify_key_pem.as_deref(),
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
    }))
}

// build_parsec_organization_bootstrap_addr
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn buildParsecOrganizationBootstrapAddr(addr: String, organization_id: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(addr).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let organization_id = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::OrganizationID::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(organization_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::build_parsec_organization_bootstrap_addr(addr, organization_id);
        Ok(JsValue::from_str({
            let custom_to_rs_string =
                |addr: libparsec::ParsecOrganizationBootstrapAddr| -> Result<String, &'static str> {
                    Ok(addr.to_url().into())
                };
            match custom_to_rs_string(ret) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
            }
            .as_ref()
        }))
    }))
}

// cancel
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn cancel(canceller: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// claimer_device_finalize_save_local_device
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerDeviceFinalizeSaveLocalDevice(handle: u32, save_strategy: Object) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
                let js_err = variant_claim_finalize_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// claimer_device_in_progress_1_do_deny_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerDeviceInProgress1DoDenyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::claimer_device_in_progress_1_do_deny_trust(canceller, handle).await;
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
                let js_err = variant_claim_in_progress_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// claimer_device_in_progress_1_do_signify_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerDeviceInProgress1DoSignifyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// claimer_device_in_progress_2_do_wait_peer_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerDeviceInProgress2DoWaitPeerTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// claimer_device_in_progress_3_do_claim
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerDeviceInProgress3DoClaim(
    canceller: u32,
    handle: u32,
    requested_device_label: String,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let requested_device_label = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(requested_device_label).map_err(|e| TypeError::new(e.as_ref()))
        }?;
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
    }))
}

// claimer_device_initial_do_wait_peer
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerDeviceInitialDoWaitPeer(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// claimer_greeter_abort_operation
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerGreeterAbortOperation(handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::claimer_greeter_abort_operation(handle).await;
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
    }))
}

// claimer_retrieve_info
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerRetrieveInfo(config: Object, addr: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config = config.into();
        let config = struct_client_config_js_to_rs(config)?;

        let addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecInvitationAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(addr).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::claimer_retrieve_info(config, addr).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = variant_any_claim_retrieved_info_rs_to_js(value)?;
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
    }))
}

// claimer_shamir_recovery_add_share
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerShamirRecoveryAddShare(recipient_pick_handle: u32, share_handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::claimer_shamir_recovery_add_share(recipient_pick_handle, share_handle);
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value =
                    variant_shamir_recovery_claim_maybe_recover_device_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_shamir_recovery_claim_add_share_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// claimer_shamir_recovery_finalize_save_local_device
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerShamirRecoveryFinalizeSaveLocalDevice(handle: u32, save_strategy: Object) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let save_strategy = save_strategy.into();
        let save_strategy = variant_device_save_strategy_js_to_rs(save_strategy)?;

        let ret =
            libparsec::claimer_shamir_recovery_finalize_save_local_device(handle, save_strategy)
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
                let js_err = variant_claim_finalize_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// claimer_shamir_recovery_in_progress_1_do_deny_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerShamirRecoveryInProgress1DoDenyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret =
            libparsec::claimer_shamir_recovery_in_progress_1_do_deny_trust(canceller, handle).await;
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
                let js_err = variant_claim_in_progress_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// claimer_shamir_recovery_in_progress_1_do_signify_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerShamirRecoveryInProgress1DoSignifyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret =
            libparsec::claimer_shamir_recovery_in_progress_1_do_signify_trust(canceller, handle)
                .await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_shamir_recovery_claim_in_progress2_info_rs_to_js(value)?;
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
    }))
}

// claimer_shamir_recovery_in_progress_2_do_wait_peer_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerShamirRecoveryInProgress2DoWaitPeerTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret =
            libparsec::claimer_shamir_recovery_in_progress_2_do_wait_peer_trust(canceller, handle)
                .await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_shamir_recovery_claim_in_progress3_info_rs_to_js(value)?;
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
    }))
}

// claimer_shamir_recovery_in_progress_3_do_claim
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerShamirRecoveryInProgress3DoClaim(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret =
            libparsec::claimer_shamir_recovery_in_progress_3_do_claim(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_shamir_recovery_claim_share_info_rs_to_js(value)?;
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
    }))
}

// claimer_shamir_recovery_initial_do_wait_peer
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerShamirRecoveryInitialDoWaitPeer(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::claimer_shamir_recovery_initial_do_wait_peer(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_shamir_recovery_claim_in_progress1_info_rs_to_js(value)?;
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
    }))
}

// claimer_shamir_recovery_pick_recipient
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerShamirRecoveryPickRecipient(handle: u32, recipient_user_id: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let recipient_user_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(recipient_user_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::claimer_shamir_recovery_pick_recipient(handle, recipient_user_id);
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_shamir_recovery_claim_initial_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_shamir_recovery_claim_pick_recipient_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// claimer_shamir_recovery_recover_device
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerShamirRecoveryRecoverDevice(handle: u32, requested_device_label: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let requested_device_label = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(requested_device_label).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret =
            libparsec::claimer_shamir_recovery_recover_device(handle, requested_device_label).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = variant_shamir_recovery_claim_maybe_finalize_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_shamir_recovery_claim_recover_device_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// claimer_user_finalize_save_local_device
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerUserFinalizeSaveLocalDevice(handle: u32, save_strategy: Object) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
                let js_err = variant_claim_finalize_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// claimer_user_in_progress_1_do_deny_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerUserInProgress1DoDenyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::claimer_user_in_progress_1_do_deny_trust(canceller, handle).await;
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
                let js_err = variant_claim_in_progress_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// claimer_user_in_progress_1_do_signify_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerUserInProgress1DoSignifyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// claimer_user_in_progress_2_do_wait_peer_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerUserInProgress2DoWaitPeerTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// claimer_user_in_progress_3_do_claim
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerUserInProgress3DoClaim(
    canceller: u32,
    handle: u32,
    requested_device_label: String,
    requested_human_handle: Object,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let requested_device_label = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(requested_device_label).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let requested_human_handle = requested_human_handle.into();
        let requested_human_handle = struct_human_handle_js_to_rs(requested_human_handle)?;

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
    }))
}

// claimer_user_initial_do_wait_peer
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerUserInitialDoWaitPeer(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// claimer_user_list_initial_info
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerUserListInitialInfo(handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::claimer_user_list_initial_info(handle);
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(value.len() as u32);
                    for (i, elem) in value.into_iter().enumerate() {
                        let js_elem = struct_user_claim_initial_info_rs_to_js(elem)?;
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
                let js_err = variant_user_claim_list_initial_infos_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// claimer_user_wait_all_peers
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn claimerUserWaitAllPeers(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::claimer_user_wait_all_peers(canceller, handle).await;
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
    }))
}

// client_accept_tos
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientAcceptTos(client: u32, tos_updated_on: f64) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let tos_updated_on = {
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            custom_from_rs_f64(tos_updated_on).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::client_accept_tos(client, tos_updated_on).await;
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
                let js_err = variant_client_accept_tos_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_cancel_invitation
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientCancelInvitation(client: u32, token: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let token = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::InvitationToken, _> {
                libparsec::InvitationToken::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(token).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_cancel_invitation(client, token).await;
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
                let js_err = variant_client_cancel_invitation_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_create_workspace
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientCreateWorkspace(client: u32, name: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
                        Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
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
    }))
}

// client_delete_shamir_recovery
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientDeleteShamirRecovery(client_handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::client_delete_shamir_recovery(client_handle).await;
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
                let js_err = variant_client_delete_shamir_recovery_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_export_recovery_device
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientExportRecoveryDevice(client_handle: u32, device_label: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let device_label = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(device_label).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_export_recovery_device(client_handle, device_label).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let (x1, x2) = value;
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(2);
                    let js_value = x1.into();
                    js_array.set(0, js_value);
                    let js_value = JsValue::from(Uint8Array::from(x2.as_ref()));
                    js_array.set(1, js_value);
                    js_array.into()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_client_export_recovery_device_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_forget_all_certificates
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientForgetAllCertificates(client: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::client_forget_all_certificates(client).await;
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
                let js_err = variant_client_forget_all_certificates_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_get_organization_bootstrap_date
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientGetOrganizationBootstrapDate(client_handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::client_get_organization_bootstrap_date(client_handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                        Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                    };
                    let v = match custom_to_rs_f64(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    };
                    JsValue::from(v)
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_client_get_organization_bootstrap_date_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_get_self_shamir_recovery
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientGetSelfShamirRecovery(client_handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::client_get_self_shamir_recovery(client_handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = variant_self_shamir_recovery_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_client_get_self_shamir_recovery_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_get_tos
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientGetTos(client: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::client_get_tos(client).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_tos_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_client_get_tos_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_get_user_device
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientGetUserDevice(client: u32, device: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let device = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
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
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(2);
                    let js_value = struct_user_info_rs_to_js(x1)?;
                    js_array.set(0, js_value);
                    let js_value = struct_device_info_rs_to_js(x2)?;
                    js_array.set(1, js_value);
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
    }))
}

// client_get_user_info
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientGetUserInfo(client: u32, user_id: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let user_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(user_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_get_user_info(client, user_id).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_user_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_client_get_user_info_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_info
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientInfo(client: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// client_list_frozen_users
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientListFrozenUsers(client_handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::client_list_frozen_users(client_handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(value.len() as u32);
                    for (i, elem) in value.into_iter().enumerate() {
                        let js_elem = JsValue::from_str({
                            let custom_to_rs_string =
                                |x: libparsec::UserID| -> Result<String, &'static str> {
                                    Ok(x.hex())
                                };
                            match custom_to_rs_string(elem) {
                                Ok(ok) => ok,
                                Err(err) => {
                                    return Err(JsValue::from(TypeError::new(&err.to_string())))
                                }
                            }
                            .as_ref()
                        });
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
                let js_err = variant_client_list_frozen_users_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_list_invitations
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientListInvitations(client: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// client_list_shamir_recoveries_for_others
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientListShamirRecoveriesForOthers(client_handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::client_list_shamir_recoveries_for_others(client_handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(value.len() as u32);
                    for (i, elem) in value.into_iter().enumerate() {
                        let js_elem = variant_other_shamir_recovery_info_rs_to_js(elem)?;
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
                let js_err = variant_client_list_shamir_recoveries_for_others_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_list_user_devices
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientListUserDevices(client: u32, user: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let user = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(user).map_err(|e| TypeError::new(e.as_ref()))
        }?;
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
    }))
}

// client_list_users
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientListUsers(client: u32, skip_revoked: bool) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// client_list_workspace_users
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientListWorkspaceUsers(client: u32, realm_id: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// client_list_workspaces
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientListWorkspaces(client: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// client_new_device_invitation
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientNewDeviceInvitation(client: u32, send_email: bool) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
                let js_err = variant_client_new_device_invitation_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_new_shamir_recovery_invitation
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientNewShamirRecoveryInvitation(
    client: u32,
    claimer_user_id: String,
    send_email: bool,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let claimer_user_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(claimer_user_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret =
            libparsec::client_new_shamir_recovery_invitation(client, claimer_user_id, send_email)
                .await;
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
                let js_err = variant_client_new_shamir_recovery_invitation_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_new_user_invitation
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientNewUserInvitation(client: u32, claimer_email: String, send_email: bool) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let claimer_email = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::EmailAddress::from_str(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(claimer_email).map_err(|e| TypeError::new(e.as_ref()))
        }?;

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
                let js_err = variant_client_new_user_invitation_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_organization_info
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientOrganizationInfo(client_handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::client_organization_info(client_handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_organization_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_client_organization_info_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_pki_enrollment_accept
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientPkiEnrollmentAccept(
    client_handle: u32,
    profile: String,
    enrollment_id: String,
    human_handle: Object,
    cert_ref: Object,
    submit_payload: Uint8Array,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let profile = enum_user_profile_js_to_rs(&profile)?;

        let enrollment_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::EnrollmentID, _> {
                libparsec::EnrollmentID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(enrollment_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let human_handle = human_handle.into();
        let human_handle = struct_human_handle_js_to_rs(human_handle)?;

        let cert_ref = cert_ref.into();
        let cert_ref = struct_x509_certificate_reference_js_to_rs(cert_ref)?;

        let submit_payload = {
            let custom_from_rs_bytes =
                |v: &[u8]| -> Result<libparsec::Bytes, String> { Ok(v.to_vec().into()) };
            custom_from_rs_bytes(&submit_payload.to_vec()).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_pki_enrollment_accept(
            client_handle,
            profile,
            enrollment_id,
            &human_handle,
            &cert_ref,
            submit_payload,
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
                let js_err = variant_pki_enrollment_accept_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_pki_enrollment_reject
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientPkiEnrollmentReject(client_handle: u32, enrollment_id: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let enrollment_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::EnrollmentID, _> {
                libparsec::EnrollmentID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(enrollment_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_pki_enrollment_reject(client_handle, enrollment_id).await;
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
                let js_err = variant_pki_enrollment_reject_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_rename_workspace
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientRenameWorkspace(client: u32, realm_id: String, new_name: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// client_revoke_user
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientRevokeUser(client: u32, user: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let user = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(user).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_revoke_user(client, user).await;
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
                let js_err = variant_client_revoke_user_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_setup_shamir_recovery
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientSetupShamirRecovery(
    client_handle: u32,
    per_recipient_shares: Object,
    threshold: u8,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let per_recipient_shares = {
            let js_map = per_recipient_shares;
            let mut d = std::collections::HashMap::with_capacity(
                Reflect::get(&js_map, &"size".into())?
                    .dyn_into::<Number>()?
                    .value_of() as usize,
            );

            let js_keys = Reflect::get(&js_map, &"keys".into())?
                .dyn_into::<Function>()?
                .call0(&js_map)?;
            let js_values = Reflect::get(&js_map, &"values".into())?
                .dyn_into::<Function>()?
                .call0(&js_map)?;

            let js_literal_next = "next".into();
            let js_keys_next_cb =
                Reflect::get(&js_keys, &js_literal_next)?.dyn_into::<Function>()?;
            let js_values_next_cb =
                Reflect::get(&js_values, &js_literal_next)?.dyn_into::<Function>()?;

            let js_literal_done = "done".into();
            let js_literal_value = "value".into();
            loop {
                let next_js_key = js_keys_next_cb.call0(&js_keys)?;
                let next_js_value = js_values_next_cb.call0(&js_values)?;

                let keys_done = Reflect::get(&next_js_key, &js_literal_done)?
                    .dyn_into::<Boolean>()?
                    .value_of();
                let values_done = Reflect::get(&next_js_value, &js_literal_done)?
                    .dyn_into::<Boolean>()?
                    .value_of();
                match (keys_done, values_done) {
                    (true, true) => break,
                    (false, false) => (),
                    _ => unreachable!(),
                }

                let js_key =
                    Reflect::get(&next_js_key, &js_literal_value)?.dyn_into::<JsString>()?;
                let js_value =
                    Reflect::get(&next_js_value, &js_literal_value)?.dyn_into::<JsString>()?;

                let rs_key = js_key
                    .dyn_into::<JsString>()
                    .ok()
                    .and_then(|s| s.as_string())
                    .ok_or_else(|| TypeError::new("Not a string"))
                    .and_then(|x| {
                        let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                            libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                        };
                        custom_from_rs_string(x).map_err(|e| TypeError::new(e.as_ref()))
                    })?;
                let rs_value = {
                    let v = js_value
                        .dyn_into::<Number>()
                        .map_err(|_| TypeError::new("Not a number"))?
                        .value_of();
                    if v < (u8::MIN as f64) || (u8::MAX as f64) < v {
                        return Err(JsValue::from(TypeError::new("Not an u8 number")));
                    }
                    let v = v as u8;
                    let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                        std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_u8(v) {
                        Ok(val) => val,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    }
                };

                d.insert(rs_key, rs_value);
            }
            d
        };

        let threshold = {
            let custom_from_rs_u8 = |x: u8| -> Result<std::num::NonZeroU8, _> {
                std::num::NonZeroU8::try_from(x).map_err(|e| e.to_string())
            };
            custom_from_rs_u8(threshold).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret =
            libparsec::client_setup_shamir_recovery(client_handle, per_recipient_shares, threshold)
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
                let js_err = variant_client_setup_shamir_recovery_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
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
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let realm_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(realm_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let recipient = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(recipient).map_err(|e| TypeError::new(e.as_ref()))
        }?;
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
    }))
}

// client_start
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientStart(config: Object, access: Object) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config = config.into();
        let config = struct_client_config_js_to_rs(config)?;

        let access = access.into();
        let access = variant_device_access_strategy_js_to_rs(access)?;

        let ret = libparsec::client_start(config, access).await;
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
    }))
}

// client_start_device_invitation_greet
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientStartDeviceInvitationGreet(client: u32, token: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// client_start_shamir_recovery_invitation_greet
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientStartShamirRecoveryInvitationGreet(client: u32, token: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let token = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::InvitationToken, _> {
                libparsec::InvitationToken::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(token).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_start_shamir_recovery_invitation_greet(client, token).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_shamir_recovery_greet_initial_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err =
                    variant_client_start_shamir_recovery_invitation_greet_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_start_user_invitation_greet
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientStartUserInvitationGreet(client: u32, token: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// client_start_workspace
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientStartWorkspace(client: u32, realm_id: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// client_start_workspace_history
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientStartWorkspaceHistory(client: u32, realm_id: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let realm_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(realm_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::client_start_workspace_history(client, realm_id).await;
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
                let js_err = variant_workspace_history_start_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// client_stop
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientStop(client: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// client_update_user_profile
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn clientUpdateUserProfile(client_handle: u32, user: String, new_profile: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let user = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(user).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let new_profile = enum_user_profile_js_to_rs(&new_profile)?;

        let ret = libparsec::client_update_user_profile(client_handle, user, new_profile).await;
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
                let js_err = variant_client_user_update_profile_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// get_default_config_dir
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn getDefaultConfigDir() -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::get_default_config_dir();
        Ok(JsValue::from_str({
            let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                path.into_os_string()
                    .into_string()
                    .map_err(|_| "Path contains non-utf8 characters")
            };
            match custom_to_rs_string(ret) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
            }
            .as_ref()
        }))
    }))
}

// get_default_data_base_dir
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn getDefaultDataBaseDir() -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::get_default_data_base_dir();
        Ok(JsValue::from_str({
            let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                path.into_os_string()
                    .into_string()
                    .map_err(|_| "Path contains non-utf8 characters")
            };
            match custom_to_rs_string(ret) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
            }
            .as_ref()
        }))
    }))
}

// get_default_mountpoint_base_dir
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn getDefaultMountpointBaseDir() -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::get_default_mountpoint_base_dir();
        Ok(JsValue::from_str({
            let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                path.into_os_string()
                    .into_string()
                    .map_err(|_| "Path contains non-utf8 characters")
            };
            match custom_to_rs_string(ret) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
            }
            .as_ref()
        }))
    }))
}

// get_platform
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn getPlatform() -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::get_platform();
        Ok(JsValue::from_str(enum_platform_rs_to_js(ret)))
    }))
}

// greeter_device_in_progress_1_do_wait_peer_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterDeviceInProgress1DoWaitPeerTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// greeter_device_in_progress_2_do_deny_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterDeviceInProgress2DoDenyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::greeter_device_in_progress_2_do_deny_trust(canceller, handle).await;
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
    }))
}

// greeter_device_in_progress_2_do_signify_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterDeviceInProgress2DoSignifyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// greeter_device_in_progress_3_do_get_claim_requests
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterDeviceInProgress3DoGetClaimRequests(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// greeter_device_in_progress_4_do_create
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterDeviceInProgress4DoCreate(
    canceller: u32,
    handle: u32,
    device_label: String,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let device_label = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(device_label).map_err(|e| TypeError::new(e.as_ref()))
        }?;
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
    }))
}

// greeter_device_initial_do_wait_peer
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterDeviceInitialDoWaitPeer(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// greeter_shamir_recovery_in_progress_1_do_wait_peer_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterShamirRecoveryInProgress1DoWaitPeerTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret =
            libparsec::greeter_shamir_recovery_in_progress_1_do_wait_peer_trust(canceller, handle)
                .await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_shamir_recovery_greet_in_progress2_info_rs_to_js(value)?;
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
    }))
}

// greeter_shamir_recovery_in_progress_2_do_deny_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterShamirRecoveryInProgress2DoDenyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret =
            libparsec::greeter_shamir_recovery_in_progress_2_do_deny_trust(canceller, handle).await;
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
    }))
}

// greeter_shamir_recovery_in_progress_2_do_signify_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterShamirRecoveryInProgress2DoSignifyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret =
            libparsec::greeter_shamir_recovery_in_progress_2_do_signify_trust(canceller, handle)
                .await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_shamir_recovery_greet_in_progress3_info_rs_to_js(value)?;
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
    }))
}

// greeter_shamir_recovery_in_progress_3_do_get_claim_requests
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterShamirRecoveryInProgress3DoGetClaimRequests(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::greeter_shamir_recovery_in_progress_3_do_get_claim_requests(
            canceller, handle,
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
    }))
}

// greeter_shamir_recovery_initial_do_wait_peer
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterShamirRecoveryInitialDoWaitPeer(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::greeter_shamir_recovery_initial_do_wait_peer(canceller, handle).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_shamir_recovery_greet_in_progress1_info_rs_to_js(value)?;
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
    }))
}

// greeter_user_in_progress_1_do_wait_peer_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterUserInProgress1DoWaitPeerTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// greeter_user_in_progress_2_do_deny_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterUserInProgress2DoDenyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::greeter_user_in_progress_2_do_deny_trust(canceller, handle).await;
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
    }))
}

// greeter_user_in_progress_2_do_signify_trust
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterUserInProgress2DoSignifyTrust(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// greeter_user_in_progress_3_do_get_claim_requests
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterUserInProgress3DoGetClaimRequests(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// greeter_user_in_progress_4_do_create
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterUserInProgress4DoCreate(
    canceller: u32,
    handle: u32,
    human_handle: Object,
    device_label: String,
    profile: String,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let human_handle = human_handle.into();
        let human_handle = struct_human_handle_js_to_rs(human_handle)?;

        let device_label = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(device_label).map_err(|e| TypeError::new(e.as_ref()))
        }?;
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
    }))
}

// greeter_user_initial_do_wait_peer
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn greeterUserInitialDoWaitPeer(canceller: u32, handle: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// import_recovery_device
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn importRecoveryDevice(
    config: Object,
    recovery_device: Uint8Array,
    passphrase: String,
    device_label: String,
    save_strategy: Object,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config = config.into();
        let config = struct_client_config_js_to_rs(config)?;

        let recovery_device = recovery_device.to_vec();

        let device_label = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(device_label).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let save_strategy = save_strategy.into();
        let save_strategy = variant_device_save_strategy_js_to_rs(save_strategy)?;

        let ret = libparsec::import_recovery_device(
            config,
            &recovery_device.to_vec(),
            passphrase,
            device_label,
            save_strategy,
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
                let js_err = variant_import_recovery_device_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// is_keyring_available
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn isKeyringAvailable() -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::is_keyring_available();
        Ok(ret.into())
    }))
}

// is_pki_available
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn isPkiAvailable() -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::is_pki_available().await;
        Ok(ret.into())
    }))
}

// libparsec_init_native_only_init
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn libparsecInitNativeOnlyInit(config: Object) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config = config.into();
        let config = struct_client_config_js_to_rs(config)?;

        libparsec::libparsec_init_native_only_init(config).await;
        Ok(JsValue::NULL)
    }))
}

// libparsec_init_set_on_event_callback
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn libparsecInitSetOnEventCallback(on_event_callback: Function) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let on_event_callback = std::sync::Arc::new(
            move |handle: libparsec::Handle, event: libparsec::ClientEvent| {
                // TODO: Better error handling here (log error ?)
                let js_event =
                    variant_client_event_rs_to_js(event).expect("event type conversion error");
                on_event_callback
                    .call2(&JsValue::NULL, &Number::from(handle), &js_event)
                    .expect("error in event callback");
            },
        )
            as std::sync::Arc<dyn Fn(libparsec::Handle, libparsec::ClientEvent)>;
        libparsec::libparsec_init_set_on_event_callback(on_event_callback);
        Ok(JsValue::NULL)
    }))
}

// list_available_devices
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn listAvailableDevices(path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::list_available_devices(&path).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(value.len() as u32);
                    for (i, elem) in value.into_iter().enumerate() {
                        let js_elem = struct_available_device_rs_to_js(elem)?;
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
                let js_err = variant_list_available_device_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// list_started_accounts
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn listStartedAccounts() -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::list_started_accounts();
        Ok({
            // Array::new_with_length allocates with `undefined` value, that's why we `set` value
            let js_array = Array::new_with_length(ret.len() as u32);
            for (i, elem) in ret.into_iter().enumerate() {
                let js_elem = JsValue::from(elem);
                js_array.set(i as u32, js_elem);
            }
            js_array.into()
        })
    }))
}

// list_started_clients
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn listStartedClients() -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::list_started_clients();
        Ok({
            // Array::new_with_length allocates with `undefined` value, that's why we `set` value
            let js_array = Array::new_with_length(ret.len() as u32);
            for (i, elem) in ret.into_iter().enumerate() {
                let js_elem = {
                    let (x1, x2) = elem;
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(2);
                    let js_value = JsValue::from(x1);
                    js_array.set(0, js_value);
                    let js_value = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::DeviceID| -> Result<String, &'static str> {
                                Ok(x.hex())
                            };
                        match custom_to_rs_string(x2) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    js_array.set(1, js_value);
                    js_array.into()
                };
                js_array.set(i as u32, js_elem);
            }
            js_array.into()
        })
    }))
}

// mountpoint_to_os_path
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn mountpointToOsPath(mountpoint: u32, parsec_path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let parsec_path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(parsec_path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::mountpoint_to_os_path(mountpoint, parsec_path).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from_str({
                    let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                        path.into_os_string()
                            .into_string()
                            .map_err(|_| "Path contains non-utf8 characters")
                    };
                    match custom_to_rs_string(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                    }
                    .as_ref()
                });
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_mountpoint_to_os_path_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// mountpoint_unmount
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn mountpointUnmount(mountpoint: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::mountpoint_unmount(mountpoint).await;
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
                let js_err = variant_mountpoint_unmount_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// new_canceller
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn newCanceller() -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::new_canceller();
        Ok(JsValue::from(ret))
    }))
}

// parse_parsec_addr
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn parseParsecAddr(url: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::parse_parsec_addr(&url);
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = variant_parsed_parsec_addr_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_parse_parsec_addr_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// path_filename
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn pathFilename(path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::path_filename(&path);
        Ok(match ret {
            Some(val) => JsValue::from_str(val.as_ref()),
            None => JsValue::NULL,
        })
    }))
}

// path_join
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn pathJoin(parent: String, child: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let parent = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(parent).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let child = {
            let custom_from_rs_string = |s: String| -> Result<_, _> {
                s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(child).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::path_join(&parent, &child);
        Ok(JsValue::from_str({
            let custom_to_rs_string =
                |path: libparsec::FsPath| -> Result<_, &'static str> { Ok(path.to_string()) };
            match custom_to_rs_string(ret) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
            }
            .as_ref()
        }))
    }))
}

// path_normalize
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn pathNormalize(path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::path_normalize(path);
        Ok(JsValue::from_str({
            let custom_to_rs_string =
                |path: libparsec::FsPath| -> Result<_, &'static str> { Ok(path.to_string()) };
            match custom_to_rs_string(ret) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
            }
            .as_ref()
        }))
    }))
}

// path_parent
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn pathParent(path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::path_parent(&path);
        Ok(JsValue::from_str({
            let custom_to_rs_string =
                |path: libparsec::FsPath| -> Result<_, &'static str> { Ok(path.to_string()) };
            match custom_to_rs_string(ret) {
                Ok(ok) => ok,
                Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
            }
            .as_ref()
        }))
    }))
}

// path_split
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn pathSplit(path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::path_split(&path);
        Ok({
            // Array::new_with_length allocates with `undefined` value, that's why we `set` value
            let js_array = Array::new_with_length(ret.len() as u32);
            for (i, elem) in ret.into_iter().enumerate() {
                let js_elem = JsValue::from_str(elem.as_ref());
                js_array.set(i as u32, js_elem);
            }
            js_array.into()
        })
    }))
}

// pki_enrollment_submit
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn pkiEnrollmentSubmit(
    config: Object,
    addr: String,
    cert_ref: Object,
    human_handle: Object,
    device_label: String,
    force: bool,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config = config.into();
        let config = struct_client_config_js_to_rs(config)?;

        let addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecPkiEnrollmentAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(addr).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let cert_ref = cert_ref.into();
        let cert_ref = struct_x509_certificate_reference_js_to_rs(cert_ref)?;

        let human_handle = human_handle.into();
        let human_handle = struct_human_handle_js_to_rs(human_handle)?;

        let device_label = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::DeviceLabel::try_from(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(device_label).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::pki_enrollment_submit(
            config,
            addr,
            cert_ref,
            human_handle,
            device_label,
            force,
        )
        .await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                        Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                    };
                    let v = match custom_to_rs_f64(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    };
                    JsValue::from(v)
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_pki_enrollment_submit_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// show_certificate_selection_dialog_windows_only
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn showCertificateSelectionDialogWindowsOnly() -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::show_certificate_selection_dialog_windows_only().await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = match value {
                    Some(val) => struct_x509_certificate_reference_rs_to_js(val)?,
                    None => JsValue::NULL,
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_show_certificate_selection_dialog_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// test_check_mailbox
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn testCheckMailbox(server_addr: String, email: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let server_addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(server_addr).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let email = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::EmailAddress::from_str(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(email).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::test_check_mailbox(&server_addr, &email).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(value.len() as u32);
                    for (i, elem) in value.into_iter().enumerate() {
                        let js_elem = {
                            let (x1, x2, x3) = elem;
                            // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                            let js_array = Array::new_with_length(3);
                            let js_value = JsValue::from_str({
                                let custom_to_rs_string =
                                    |x: libparsec::EmailAddress| -> Result<_, &'static str> {
                                        Ok(x.to_string())
                                    };
                                match custom_to_rs_string(x1) {
                                    Ok(ok) => ok,
                                    Err(err) => {
                                        return Err(JsValue::from(TypeError::new(&err.to_string())))
                                    }
                                }
                                .as_ref()
                            });
                            js_array.set(0, js_value);
                            let js_value = {
                                let custom_to_rs_f64 =
                                    |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                                        Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                                    };
                                let v = match custom_to_rs_f64(x2) {
                                    Ok(ok) => ok,
                                    Err(err) => {
                                        return Err(JsValue::from(TypeError::new(err.as_ref())))
                                    }
                                };
                                JsValue::from(v)
                            };
                            js_array.set(1, js_value);
                            let js_value = x3.into();
                            js_array.set(2, js_value);
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
                let js_err = variant_testbed_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// test_drop_testbed
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn testDropTestbed(path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::test_drop_testbed(&path).await;
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
                let js_err = variant_testbed_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// test_get_testbed_bootstrap_organization_addr
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn testGetTestbedBootstrapOrganizationAddr(discriminant_dir: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let discriminant_dir = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(discriminant_dir).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::test_get_testbed_bootstrap_organization_addr(&discriminant_dir);
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = match value {
                    Some(val) => JsValue::from_str({
                        let custom_to_rs_string = |addr: libparsec::ParsecOrganizationBootstrapAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) };
                        match custom_to_rs_string(val) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    }),
                    None => JsValue::NULL,
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_testbed_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// test_get_testbed_organization_id
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn testGetTestbedOrganizationId(discriminant_dir: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let discriminant_dir = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(discriminant_dir).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::test_get_testbed_organization_id(&discriminant_dir);
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = match value {
                    Some(val) => JsValue::from_str(val.as_ref()),
                    None => JsValue::NULL,
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_testbed_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// test_new_account
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn testNewAccount(server_addr: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let server_addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(server_addr).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::test_new_account(&server_addr).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let (x1, x2) = value;
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(2);
                    let js_value = struct_human_handle_rs_to_js(x1)?;
                    js_array.set(0, js_value);
                    let js_value = JsValue::from(Uint8Array::from(x2.as_ref()));
                    js_array.set(1, js_value);
                    js_array.into()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_testbed_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// test_new_testbed
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn testNewTestbed(template: String, test_server: Option<String>) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let test_server = match test_server {
            Some(test_server) => {
                let test_server = {
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
                    };
                    custom_from_rs_string(test_server).map_err(|e| TypeError::new(e.as_ref()))
                }?;

                Some(test_server)
            }
            None => None,
        };

        let ret = libparsec::test_new_testbed(&template, test_server.as_ref()).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from_str({
                    let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                        path.into_os_string()
                            .into_string()
                            .map_err(|_| "Path contains non-utf8 characters")
                    };
                    match custom_to_rs_string(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                    }
                    .as_ref()
                });
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_testbed_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// update_device_change_authentication
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn updateDeviceChangeAuthentication(
    config_dir: String,
    current_auth: Object,
    new_auth: Object,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config_dir = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(config_dir).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let current_auth = current_auth.into();
        let current_auth = variant_device_access_strategy_js_to_rs(current_auth)?;

        let new_auth = new_auth.into();
        let new_auth = variant_device_save_strategy_js_to_rs(new_auth)?;

        let ret =
            libparsec::update_device_change_authentication(&config_dir, current_auth, new_auth)
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
                let js_err = variant_update_device_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// update_device_overwrite_server_addr
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn updateDeviceOverwriteServerAddr(
    config_dir: String,
    access: Object,
    new_server_addr: String,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config_dir = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(config_dir).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let access = access.into();
        let access = variant_device_access_strategy_js_to_rs(access)?;

        let new_server_addr = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(new_server_addr).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret =
            libparsec::update_device_overwrite_server_addr(&config_dir, access, new_server_addr)
                .await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from_str({
                    let custom_to_rs_string =
                        |addr: libparsec::ParsecAddr| -> Result<String, &'static str> {
                            Ok(addr.to_url().into())
                        };
                    match custom_to_rs_string(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                    }
                    .as_ref()
                });
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_update_device_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// validate_device_label
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn validateDeviceLabel(raw: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::validate_device_label(&raw);
        Ok(ret.into())
    }))
}

// validate_email
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn validateEmail(raw: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::validate_email(&raw);
        Ok(ret.into())
    }))
}

// validate_entry_name
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn validateEntryName(raw: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::validate_entry_name(&raw);
        Ok(ret.into())
    }))
}

// validate_human_handle_label
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn validateHumanHandleLabel(raw: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::validate_human_handle_label(&raw);
        Ok(ret.into())
    }))
}

// validate_invitation_token
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn validateInvitationToken(raw: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::validate_invitation_token(&raw);
        Ok(ret.into())
    }))
}

// validate_organization_id
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn validateOrganizationId(raw: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::validate_organization_id(&raw);
        Ok(ret.into())
    }))
}

// validate_path
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn validatePath(raw: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::validate_path(&raw);
        Ok(ret.into())
    }))
}

// wait_for_device_available
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn waitForDeviceAvailable(config_dir: String, device_id: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config_dir = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(config_dir).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let device_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(device_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::wait_for_device_available(&config_dir, device_id).await;
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
                let js_err = variant_wait_for_device_available_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_create_file
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceCreateFile(workspace: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_create_file(workspace, path).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from_str({
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                    }
                    .as_ref()
                });
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_create_file_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_create_folder
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceCreateFolder(workspace: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_create_folder(workspace, path).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from_str({
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                    }
                    .as_ref()
                });
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_create_folder_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_create_folder_all
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceCreateFolderAll(workspace: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_create_folder_all(workspace, path).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from_str({
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                    }
                    .as_ref()
                });
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_create_folder_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_decrypt_path_addr
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceDecryptPathAddr(workspace: u32, link: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let link = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecWorkspacePathAddr::from_any(&s).map_err(|e| e.to_string())
            };
            custom_from_rs_string(link).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_decrypt_path_addr(workspace, &link).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from_str({
                    let custom_to_rs_string = |path: libparsec::FsPath| -> Result<_, &'static str> {
                        Ok(path.to_string())
                    };
                    match custom_to_rs_string(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                    }
                    .as_ref()
                });
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_decrypt_path_addr_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_fd_close
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceFdClose(workspace: u32, fd: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let fd = {
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            custom_from_rs_u32(fd).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_fd_close(workspace, fd).await;
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
                let js_err = variant_workspace_fd_close_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_fd_flush
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceFdFlush(workspace: u32, fd: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let fd = {
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            custom_from_rs_u32(fd).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_fd_flush(workspace, fd).await;
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
                let js_err = variant_workspace_fd_flush_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_fd_read
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceFdRead(workspace: u32, fd: u32, offset: u64, size: u64) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let fd = {
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            custom_from_rs_u32(fd).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_fd_read(workspace, fd, offset, size).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from(Uint8Array::from(value.as_ref()));
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_fd_read_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_fd_resize
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceFdResize(workspace: u32, fd: u32, length: u64, truncate_only: bool) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let fd = {
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            custom_from_rs_u32(fd).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_fd_resize(workspace, fd, length, truncate_only).await;
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
                let js_err = variant_workspace_fd_resize_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_fd_stat
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceFdStat(workspace: u32, fd: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let fd = {
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            custom_from_rs_u32(fd).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_fd_stat(workspace, fd).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_file_stat_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_fd_stat_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_fd_write
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceFdWrite(workspace: u32, fd: u32, offset: u64, data: Uint8Array) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let fd = {
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            custom_from_rs_u32(fd).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let data = data.to_vec();

        let ret = libparsec::workspace_fd_write(workspace, fd, offset, &data.to_vec()).await;
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
                let js_err = variant_workspace_fd_write_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_fd_write_constrained_io
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceFdWriteConstrainedIo(
    workspace: u32,
    fd: u32,
    offset: u64,
    data: Uint8Array,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let fd = {
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            custom_from_rs_u32(fd).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let data = data.to_vec();

        let ret =
            libparsec::workspace_fd_write_constrained_io(workspace, fd, offset, &data.to_vec())
                .await;
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
                let js_err = variant_workspace_fd_write_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_fd_write_start_eof
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceFdWriteStartEof(workspace: u32, fd: u32, data: Uint8Array) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let fd = {
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            custom_from_rs_u32(fd).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let data = data.to_vec();

        let ret = libparsec::workspace_fd_write_start_eof(workspace, fd, &data.to_vec()).await;
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
                let js_err = variant_workspace_fd_write_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_generate_path_addr
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceGeneratePathAddr(workspace: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_generate_path_addr(workspace, &path).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from_str({
                    let custom_to_rs_string =
                        |addr: libparsec::ParsecWorkspacePathAddr| -> Result<String, &'static str> {
                            Ok(addr.to_url().into())
                        };
                    match custom_to_rs_string(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                    }
                    .as_ref()
                });
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_generate_path_addr_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_fd_close
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistoryFdClose(workspace_history: u32, fd: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let fd = {
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            custom_from_rs_u32(fd).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_history_fd_close(workspace_history, fd);
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
                let js_err = variant_workspace_history_fd_close_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_fd_read
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistoryFdRead(workspace_history: u32, fd: u32, offset: u64, size: u64) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let fd = {
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            custom_from_rs_u32(fd).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_history_fd_read(workspace_history, fd, offset, size).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from(Uint8Array::from(value.as_ref()));
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_history_fd_read_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_fd_stat
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistoryFdStat(workspace_history: u32, fd: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let fd = {
            let custom_from_rs_u32 =
                |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
            custom_from_rs_u32(fd).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_history_fd_stat(workspace_history, fd).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_workspace_history_file_stat_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_history_fd_stat_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_get_timestamp_higher_bound
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistoryGetTimestampHigherBound(workspace_history: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::workspace_history_get_timestamp_higher_bound(workspace_history);
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                        Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                    };
                    let v = match custom_to_rs_f64(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    };
                    JsValue::from(v)
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_history_internal_only_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_get_timestamp_lower_bound
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistoryGetTimestampLowerBound(workspace_history: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::workspace_history_get_timestamp_lower_bound(workspace_history);
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                        Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                    };
                    let v = match custom_to_rs_f64(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    };
                    JsValue::from(v)
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_history_internal_only_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_get_timestamp_of_interest
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistoryGetTimestampOfInterest(workspace_history: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::workspace_history_get_timestamp_of_interest(workspace_history);
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                        Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                    };
                    let v = match custom_to_rs_f64(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    };
                    JsValue::from(v)
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_history_internal_only_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_open_file
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistoryOpenFile(workspace_history: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_history_open_file(workspace_history, path).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let custom_to_rs_u32 =
                        |fd: libparsec::FileDescriptor| -> Result<_, &'static str> { Ok(fd.0) };
                    let v = match custom_to_rs_u32(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    };
                    JsValue::from(v)
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_history_open_file_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_open_file_and_get_id
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistoryOpenFileAndGetId(workspace_history: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_history_open_file_and_get_id(workspace_history, path).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let (x1, x2) = value;
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(2);
                    let js_value = {
                        let custom_to_rs_u32 =
                            |fd: libparsec::FileDescriptor| -> Result<_, &'static str> { Ok(fd.0) };
                        let v = match custom_to_rs_u32(x1) {
                            Ok(ok) => ok,
                            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                        };
                        JsValue::from(v)
                    };
                    js_array.set(0, js_value);
                    let js_value = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(x2) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    js_array.set(1, js_value);
                    js_array.into()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_history_open_file_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_open_file_by_id
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistoryOpenFileById(workspace_history: u32, entry_id: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let entry_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(entry_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_history_open_file_by_id(workspace_history, entry_id).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let custom_to_rs_u32 =
                        |fd: libparsec::FileDescriptor| -> Result<_, &'static str> { Ok(fd.0) };
                    let v = match custom_to_rs_u32(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    };
                    JsValue::from(v)
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_history_open_file_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_set_timestamp_of_interest
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistorySetTimestampOfInterest(workspace_history: u32, toi: f64) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let toi = {
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            custom_from_rs_f64(toi).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret =
            libparsec::workspace_history_set_timestamp_of_interest(workspace_history, toi).await;
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
                let js_err =
                    variant_workspace_history_set_timestamp_of_interest_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_start_with_realm_export
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistoryStartWithRealmExport(
    config: Object,
    export_db_path: String,
    decryptors: Vec<Object>,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let config = config.into();
        let config = struct_client_config_js_to_rs(config)?;

        let export_db_path = {
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            custom_from_rs_string(export_db_path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let mut decryptors_converted = Vec::with_capacity(decryptors.len());
        for js_elem in decryptors.iter() {
            let rs_elem =
                variant_workspace_history_realm_export_decryptor_js_to_rs(js_elem.into())?;
            decryptors_converted.push(rs_elem);
        }
        let decryptors = decryptors_converted;

        let ret = libparsec::workspace_history_start_with_realm_export(
            config,
            export_db_path,
            decryptors,
        )
        .await;
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
                let js_err = variant_workspace_history_start_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_stat_entry
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistoryStatEntry(workspace_history: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_history_stat_entry(workspace_history, &path).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = variant_workspace_history_entry_stat_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_history_stat_entry_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_stat_entry_by_id
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistoryStatEntryById(workspace_history: u32, entry_id: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let entry_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(entry_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_history_stat_entry_by_id(workspace_history, entry_id).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = variant_workspace_history_entry_stat_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_history_stat_entry_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_stat_folder_children
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistoryStatFolderChildren(workspace_history: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_history_stat_folder_children(workspace_history, &path).await;
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
                            // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                            let js_array = Array::new_with_length(2);
                            let js_value = JsValue::from_str(x1.as_ref());
                            js_array.set(0, js_value);
                            let js_value = variant_workspace_history_entry_stat_rs_to_js(x2)?;
                            js_array.set(1, js_value);
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
                let js_err = variant_workspace_history_stat_folder_children_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_stat_folder_children_by_id
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistoryStatFolderChildrenById(workspace_history: u32, entry_id: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let entry_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(entry_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret =
            libparsec::workspace_history_stat_folder_children_by_id(workspace_history, entry_id)
                .await;
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
                            // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                            let js_array = Array::new_with_length(2);
                            let js_value = JsValue::from_str(x1.as_ref());
                            js_array.set(0, js_value);
                            let js_value = variant_workspace_history_entry_stat_rs_to_js(x2)?;
                            js_array.set(1, js_value);
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
                let js_err = variant_workspace_history_stat_folder_children_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_history_stop
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceHistoryStop(workspace_history: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::workspace_history_stop(workspace_history);
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
                let js_err = variant_workspace_history_internal_only_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_info
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceInfo(workspace: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::workspace_info(workspace).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = struct_started_workspace_info_rs_to_js(value)?;
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_info_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_is_file_content_local
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceIsFileContentLocal(workspace: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_is_file_content_local(workspace, path).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = value.into();
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_is_file_content_local_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_mount
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceMount(workspace: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let ret = libparsec::workspace_mount(workspace).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let (x1, x2) = value;
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(2);
                    let js_value = JsValue::from(x1);
                    js_array.set(0, js_value);
                    let js_value = JsValue::from_str({
                        let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                            path.into_os_string()
                                .into_string()
                                .map_err(|_| "Path contains non-utf8 characters")
                        };
                        match custom_to_rs_string(x2) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    js_array.set(1, js_value);
                    js_array.into()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_mount_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_move_entry
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceMoveEntry(workspace: u32, src: String, dst: String, mode: Object) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let src = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(src).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let dst = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(dst).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let mode = mode.into();
        let mode = variant_move_entry_mode_js_to_rs(mode)?;

        let ret = libparsec::workspace_move_entry(workspace, src, dst, mode).await;
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
                let js_err = variant_workspace_move_entry_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_open_file
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceOpenFile(workspace: u32, path: String, mode: Object) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let mode = mode.into();
        let mode = struct_open_options_js_to_rs(mode)?;

        let ret = libparsec::workspace_open_file(workspace, path, mode).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let custom_to_rs_u32 =
                        |fd: libparsec::FileDescriptor| -> Result<_, &'static str> { Ok(fd.0) };
                    let v = match custom_to_rs_u32(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    };
                    JsValue::from(v)
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_open_file_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_open_file_and_get_id
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceOpenFileAndGetId(workspace: u32, path: String, mode: Object) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let mode = mode.into();
        let mode = struct_open_options_js_to_rs(mode)?;

        let ret = libparsec::workspace_open_file_and_get_id(workspace, path, mode).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let (x1, x2) = value;
                    // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                    let js_array = Array::new_with_length(2);
                    let js_value = {
                        let custom_to_rs_u32 =
                            |fd: libparsec::FileDescriptor| -> Result<_, &'static str> { Ok(fd.0) };
                        let v = match custom_to_rs_u32(x1) {
                            Ok(ok) => ok,
                            Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                        };
                        JsValue::from(v)
                    };
                    js_array.set(0, js_value);
                    let js_value = JsValue::from_str({
                        let custom_to_rs_string =
                            |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                        match custom_to_rs_string(x2) {
                            Ok(ok) => ok,
                            Err(err) => {
                                return Err(JsValue::from(TypeError::new(&err.to_string())))
                            }
                        }
                        .as_ref()
                    });
                    js_array.set(1, js_value);
                    js_array.into()
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_open_file_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_open_file_by_id
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceOpenFileById(workspace: u32, entry_id: String, mode: Object) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let entry_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(entry_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let mode = mode.into();
        let mode = struct_open_options_js_to_rs(mode)?;

        let ret = libparsec::workspace_open_file_by_id(workspace, entry_id, mode).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = {
                    let custom_to_rs_u32 =
                        |fd: libparsec::FileDescriptor| -> Result<_, &'static str> { Ok(fd.0) };
                    let v = match custom_to_rs_u32(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(err.as_ref()))),
                    };
                    JsValue::from(v)
                };
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_open_file_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_remove_entry
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceRemoveEntry(workspace: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_remove_entry(workspace, path).await;
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
                let js_err = variant_workspace_remove_entry_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_remove_file
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceRemoveFile(workspace: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_remove_file(workspace, path).await;
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
                let js_err = variant_workspace_remove_entry_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_remove_folder
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceRemoveFolder(workspace: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_remove_folder(workspace, path).await;
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
                let js_err = variant_workspace_remove_entry_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_remove_folder_all
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceRemoveFolderAll(workspace: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_remove_folder_all(workspace, path).await;
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
                let js_err = variant_workspace_remove_entry_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_rename_entry_by_id
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceRenameEntryById(
    workspace: u32,
    src_parent_id: String,
    src_name: String,
    dst_name: String,
    mode: Object,
) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let src_parent_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(src_parent_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let src_name = {
            let custom_from_rs_string = |s: String| -> Result<_, _> {
                s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(src_name).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let dst_name = {
            let custom_from_rs_string = |s: String| -> Result<_, _> {
                s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(dst_name).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let mode = mode.into();
        let mode = variant_move_entry_mode_js_to_rs(mode)?;

        let ret = libparsec::workspace_rename_entry_by_id(
            workspace,
            src_parent_id,
            src_name,
            dst_name,
            mode,
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
                let js_err = variant_workspace_move_entry_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_stat_entry
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceStatEntry(workspace: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
                let js_err = variant_workspace_stat_entry_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_stat_entry_by_id
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceStatEntryById(workspace: u32, entry_id: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let entry_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(entry_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_stat_entry_by_id(workspace, entry_id).await;
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
                let js_err = variant_workspace_stat_entry_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_stat_entry_by_id_ignore_confinement_point
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceStatEntryByIdIgnoreConfinementPoint(workspace: u32, entry_id: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let entry_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(entry_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret =
            libparsec::workspace_stat_entry_by_id_ignore_confinement_point(workspace, entry_id)
                .await;
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
                let js_err = variant_workspace_stat_entry_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_stat_folder_children
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceStatFolderChildren(workspace: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;

        let ret = libparsec::workspace_stat_folder_children(workspace, &path).await;
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
                            // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                            let js_array = Array::new_with_length(2);
                            let js_value = JsValue::from_str(x1.as_ref());
                            js_array.set(0, js_value);
                            let js_value = variant_entry_stat_rs_to_js(x2)?;
                            js_array.set(1, js_value);
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
                let js_err = variant_workspace_stat_folder_children_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_stat_folder_children_by_id
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceStatFolderChildrenById(workspace: u32, entry_id: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let entry_id = {
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            custom_from_rs_string(entry_id).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_stat_folder_children_by_id(workspace, entry_id).await;
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
                            // Array::new_with_length allocates with `undefined` value, that's why we `set` value
                            let js_array = Array::new_with_length(2);
                            let js_value = JsValue::from_str(x1.as_ref());
                            js_array.set(0, js_value);
                            let js_value = variant_entry_stat_rs_to_js(x2)?;
                            js_array.set(1, js_value);
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
                let js_err = variant_workspace_stat_folder_children_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

// workspace_stop
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceStop(workspace: u32) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
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
    }))
}

// workspace_watch_entry_oneshot
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn workspaceWatchEntryOneshot(workspace: u32, path: String) -> Promise {
    future_to_promise(libparsec::WithTaskIDFuture::from(async move {
        let path = {
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
            };
            custom_from_rs_string(path).map_err(|e| TypeError::new(e.as_ref()))
        }?;
        let ret = libparsec::workspace_watch_entry_oneshot(workspace, path).await;
        Ok(match ret {
            Ok(value) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &true.into())?;
                let js_value = JsValue::from_str({
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(value) {
                        Ok(ok) => ok,
                        Err(err) => return Err(JsValue::from(TypeError::new(&err.to_string()))),
                    }
                    .as_ref()
                });
                Reflect::set(&js_obj, &"value".into(), &js_value)?;
                js_obj
            }
            Err(err) => {
                let js_obj = Object::new().into();
                Reflect::set(&js_obj, &"ok".into(), &false.into())?;
                let js_err = variant_workspace_watch_entry_one_shot_error_rs_to_js(err)?;
                Reflect::set(&js_obj, &"error".into(), &js_err)?;
                js_obj
            }
        })
    }))
}

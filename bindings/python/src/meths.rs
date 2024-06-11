// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

#![allow(
    clippy::redundant_closure,
    clippy::needless_lifetimes,
    clippy::too_many_arguments,
    clippy::useless_asref
)]

#[allow(unused_imports)]
use pyo3::{conversion::ToPyObject, exceptions::*, intern, prelude::*, types::*};

// DeviceFileType

#[allow(dead_code, unused_variables)]
fn enum_device_file_type_py_to_rs<'a>(
    py: Python<'a>,
    raw_value: &str,
) -> PyResult<libparsec::DeviceFileType> {
    match raw_value {
        "DeviceFileTypeKeyring" => Ok(libparsec::DeviceFileType::Keyring),
        "DeviceFileTypePassword" => Ok(libparsec::DeviceFileType::Password),
        "DeviceFileTypeRecovery" => Ok(libparsec::DeviceFileType::Recovery),
        "DeviceFileTypeSmartcard" => Ok(libparsec::DeviceFileType::Smartcard),
        _ => Err(PyValueError::new_err(
            "Invalid value `{raw_value}` for enum DeviceFileType",
        )),
    }
}

#[allow(dead_code, unused_variables)]
fn enum_device_file_type_rs_to_py(value: libparsec::DeviceFileType) -> &'static str {
    match value {
        libparsec::DeviceFileType::Keyring => "DeviceFileTypeKeyring",
        libparsec::DeviceFileType::Password => "DeviceFileTypePassword",
        libparsec::DeviceFileType::Recovery => "DeviceFileTypeRecovery",
        libparsec::DeviceFileType::Smartcard => "DeviceFileTypeSmartcard",
    }
}

// InvitationEmailSentStatus

#[allow(dead_code, unused_variables)]
fn enum_invitation_email_sent_status_py_to_rs<'a>(
    py: Python<'a>,
    raw_value: &str,
) -> PyResult<libparsec::InvitationEmailSentStatus> {
    match raw_value {
        "InvitationEmailSentStatusRecipientRefused" => {
            Ok(libparsec::InvitationEmailSentStatus::RecipientRefused)
        }
        "InvitationEmailSentStatusServerUnavailable" => {
            Ok(libparsec::InvitationEmailSentStatus::ServerUnavailable)
        }
        "InvitationEmailSentStatusSuccess" => Ok(libparsec::InvitationEmailSentStatus::Success),
        _ => Err(PyValueError::new_err(
            "Invalid value `{raw_value}` for enum InvitationEmailSentStatus",
        )),
    }
}

#[allow(dead_code, unused_variables)]
fn enum_invitation_email_sent_status_rs_to_py(
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

#[allow(dead_code, unused_variables)]
fn enum_invitation_status_py_to_rs<'a>(
    py: Python<'a>,
    raw_value: &str,
) -> PyResult<libparsec::InvitationStatus> {
    match raw_value {
        "InvitationStatusCancelled" => Ok(libparsec::InvitationStatus::Cancelled),
        "InvitationStatusFinished" => Ok(libparsec::InvitationStatus::Finished),
        "InvitationStatusIdle" => Ok(libparsec::InvitationStatus::Idle),
        "InvitationStatusReady" => Ok(libparsec::InvitationStatus::Ready),
        _ => Err(PyValueError::new_err(
            "Invalid value `{raw_value}` for enum InvitationStatus",
        )),
    }
}

#[allow(dead_code, unused_variables)]
fn enum_invitation_status_rs_to_py(value: libparsec::InvitationStatus) -> &'static str {
    match value {
        libparsec::InvitationStatus::Cancelled => "InvitationStatusCancelled",
        libparsec::InvitationStatus::Finished => "InvitationStatusFinished",
        libparsec::InvitationStatus::Idle => "InvitationStatusIdle",
        libparsec::InvitationStatus::Ready => "InvitationStatusReady",
    }
}

// Platform

#[allow(dead_code, unused_variables)]
fn enum_platform_py_to_rs<'a>(py: Python<'a>, raw_value: &str) -> PyResult<libparsec::Platform> {
    match raw_value {
        "PlatformAndroid" => Ok(libparsec::Platform::Android),
        "PlatformLinux" => Ok(libparsec::Platform::Linux),
        "PlatformMacOS" => Ok(libparsec::Platform::MacOS),
        "PlatformWeb" => Ok(libparsec::Platform::Web),
        "PlatformWindows" => Ok(libparsec::Platform::Windows),
        _ => Err(PyValueError::new_err(
            "Invalid value `{raw_value}` for enum Platform",
        )),
    }
}

#[allow(dead_code, unused_variables)]
fn enum_platform_rs_to_py(value: libparsec::Platform) -> &'static str {
    match value {
        libparsec::Platform::Android => "PlatformAndroid",
        libparsec::Platform::Linux => "PlatformLinux",
        libparsec::Platform::MacOS => "PlatformMacOS",
        libparsec::Platform::Web => "PlatformWeb",
        libparsec::Platform::Windows => "PlatformWindows",
    }
}

// RealmRole

#[allow(dead_code, unused_variables)]
fn enum_realm_role_py_to_rs<'a>(py: Python<'a>, raw_value: &str) -> PyResult<libparsec::RealmRole> {
    match raw_value {
        "RealmRoleContributor" => Ok(libparsec::RealmRole::Contributor),
        "RealmRoleManager" => Ok(libparsec::RealmRole::Manager),
        "RealmRoleOwner" => Ok(libparsec::RealmRole::Owner),
        "RealmRoleReader" => Ok(libparsec::RealmRole::Reader),
        _ => Err(PyValueError::new_err(
            "Invalid value `{raw_value}` for enum RealmRole",
        )),
    }
}

#[allow(dead_code, unused_variables)]
fn enum_realm_role_rs_to_py(value: libparsec::RealmRole) -> &'static str {
    match value {
        libparsec::RealmRole::Contributor => "RealmRoleContributor",
        libparsec::RealmRole::Manager => "RealmRoleManager",
        libparsec::RealmRole::Owner => "RealmRoleOwner",
        libparsec::RealmRole::Reader => "RealmRoleReader",
    }
}

// UserProfile

#[allow(dead_code, unused_variables)]
fn enum_user_profile_py_to_rs<'a>(
    py: Python<'a>,
    raw_value: &str,
) -> PyResult<libparsec::UserProfile> {
    match raw_value {
        "UserProfileAdmin" => Ok(libparsec::UserProfile::Admin),
        "UserProfileOutsider" => Ok(libparsec::UserProfile::Outsider),
        "UserProfileStandard" => Ok(libparsec::UserProfile::Standard),
        _ => Err(PyValueError::new_err(
            "Invalid value `{raw_value}` for enum UserProfile",
        )),
    }
}

#[allow(dead_code, unused_variables)]
fn enum_user_profile_rs_to_py(value: libparsec::UserProfile) -> &'static str {
    match value {
        libparsec::UserProfile::Admin => "UserProfileAdmin",
        libparsec::UserProfile::Outsider => "UserProfileOutsider",
        libparsec::UserProfile::Standard => "UserProfileStandard",
    }
}

// AvailableDevice

#[allow(dead_code, unused_variables)]
fn struct_available_device_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::AvailableDevice> {
    let key_file_path = {
        let py_val_any = obj
            .get_item("key_file_path")?
            .ok_or_else(|| PyKeyError::new_err("key_file_path"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let created_on = {
        let py_val_any = obj
            .get_item("created_on")?
            .ok_or_else(|| PyKeyError::new_err("created_on"))?;
        let py_val_downcasted = py_val_any.downcast::<PyFloat>()?;
        {
            let v = py_val_downcasted.extract::<f64>()?;
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let protected_on = {
        let py_val_any = obj
            .get_item("protected_on")?
            .ok_or_else(|| PyKeyError::new_err("protected_on"))?;
        let py_val_downcasted = py_val_any.downcast::<PyFloat>()?;
        {
            let v = py_val_downcasted.extract::<f64>()?;
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let server_url = {
        let py_val_any = obj
            .get_item("server_url")?
            .ok_or_else(|| PyKeyError::new_err("server_url"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            py_val_str.to_str()?.to_owned()
        }
    };
    let organization_id = {
        let py_val_any = obj
            .get_item("organization_id")?
            .ok_or_else(|| PyKeyError::new_err("organization_id"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            match py_val_str.to_str()?.parse() {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let user_id = {
        let py_val_any = obj
            .get_item("user_id")?
            .ok_or_else(|| PyKeyError::new_err("user_id"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let device_id = {
        let py_val_any = obj
            .get_item("device_id")?
            .ok_or_else(|| PyKeyError::new_err("device_id"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let human_handle = {
        let py_val_any = obj
            .get_item("human_handle")?
            .ok_or_else(|| PyKeyError::new_err("human_handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyDict>()?;
        {
            let py_val_dict: &Bound<'_, PyDict> = py_val_downcasted;
            struct_human_handle_py_to_rs(py, py_val_dict)?
        }
    };
    let device_label = {
        let py_val_any = obj
            .get_item("device_label")?
            .ok_or_else(|| PyKeyError::new_err("device_label"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            match py_val_str.to_str()?.parse() {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let ty = {
        let py_val_any = obj
            .get_item("ty")?
            .ok_or_else(|| PyKeyError::new_err("ty"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let raw_value = py_val_downcasted.extract::<&str>()?;
            enum_device_file_type_py_to_rs(py, raw_value)?
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

#[allow(dead_code, unused_variables)]
fn struct_available_device_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::AvailableDevice,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_key_file_path = PyString::new_bound(py, &{
        let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        };
        match custom_to_rs_string(rs_obj.key_file_path) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("key_file_path", py_key_file_path)?;
    let py_created_on = PyFloat::new_bound(py, {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        match custom_to_rs_f64(rs_obj.created_on) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("created_on", py_created_on)?;
    let py_protected_on = PyFloat::new_bound(py, {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        match custom_to_rs_f64(rs_obj.protected_on) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("protected_on", py_protected_on)?;
    let py_server_url = PyString::new_bound(py, rs_obj.server_url.as_ref()).into_any();
    py_obj.set_item("server_url", py_server_url)?;
    let py_organization_id = PyString::new_bound(py, rs_obj.organization_id.as_ref()).into_any();
    py_obj.set_item("organization_id", py_organization_id)?;
    let py_user_id = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.user_id) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("user_id", py_user_id)?;
    let py_device_id = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.device_id) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("device_id", py_device_id)?;
    let py_human_handle = struct_human_handle_rs_to_py(py, rs_obj.human_handle)?.into_any();
    py_obj.set_item("human_handle", py_human_handle)?;
    let py_device_label = PyString::new_bound(py, rs_obj.device_label.as_ref()).into_any();
    py_obj.set_item("device_label", py_device_label)?;
    let py_ty = PyString::new_bound(py, enum_device_file_type_rs_to_py(rs_obj.ty)).into_any();
    py_obj.set_item("ty", py_ty)?;
    Ok(py_obj)
}

// ClientConfig

#[allow(dead_code, unused_variables)]
fn struct_client_config_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::ClientConfig> {
    let config_dir = {
        let py_val_any = obj
            .get_item("config_dir")?
            .ok_or_else(|| PyKeyError::new_err("config_dir"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let data_base_dir = {
        let py_val_any = obj
            .get_item("data_base_dir")?
            .ok_or_else(|| PyKeyError::new_err("data_base_dir"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string =
                |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let mountpoint_mount_strategy = {
        let py_val_any = obj
            .get_item("mountpoint_mount_strategy")?
            .ok_or_else(|| PyKeyError::new_err("mountpoint_mount_strategy"))?;
        let py_val_downcasted = py_val_any.downcast::<PyDict>()?;
        {
            let py_val_dict: &Bound<'_, PyDict> = py_val_downcasted;
            variant_mountpoint_mount_strategy_py_to_rs(py, py_val_dict)?
        }
    };
    let workspace_storage_cache_size = {
        let py_val_any = obj
            .get_item("workspace_storage_cache_size")?
            .ok_or_else(|| PyKeyError::new_err("workspace_storage_cache_size"))?;
        let py_val_downcasted = py_val_any.downcast::<PyDict>()?;
        {
            let py_val_dict: &Bound<'_, PyDict> = py_val_downcasted;
            variant_workspace_storage_cache_size_py_to_rs(py, py_val_dict)?
        }
    };
    let with_monitors = {
        let py_val_any = obj
            .get_item("with_monitors")?
            .ok_or_else(|| PyKeyError::new_err("with_monitors"))?;
        let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
        py_val_downcasted.extract()?
    };
    Ok(libparsec::ClientConfig {
        config_dir,
        data_base_dir,
        mountpoint_mount_strategy,
        workspace_storage_cache_size,
        with_monitors,
    })
}

#[allow(dead_code, unused_variables)]
fn struct_client_config_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientConfig,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_config_dir = PyString::new_bound(py, &{
        let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        };
        match custom_to_rs_string(rs_obj.config_dir) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("config_dir", py_config_dir)?;
    let py_data_base_dir = PyString::new_bound(py, &{
        let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        };
        match custom_to_rs_string(rs_obj.data_base_dir) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("data_base_dir", py_data_base_dir)?;
    let py_mountpoint_mount_strategy =
        variant_mountpoint_mount_strategy_rs_to_py(py, rs_obj.mountpoint_mount_strategy)?
            .into_any();
    py_obj.set_item("mountpoint_mount_strategy", py_mountpoint_mount_strategy)?;
    let py_workspace_storage_cache_size =
        variant_workspace_storage_cache_size_rs_to_py(py, rs_obj.workspace_storage_cache_size)?
            .into_any();
    py_obj.set_item(
        "workspace_storage_cache_size",
        py_workspace_storage_cache_size,
    )?;
    let py_with_monitors = PyBool::new_bound(py, rs_obj.with_monitors)
        .to_owned()
        .into_any();
    py_obj.set_item("with_monitors", py_with_monitors)?;
    Ok(py_obj)
}

// ClientInfo

#[allow(dead_code, unused_variables)]
fn struct_client_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::ClientInfo> {
    let organization_addr = {
        let py_val_any = obj
            .get_item("organization_addr")?
            .ok_or_else(|| PyKeyError::new_err("organization_addr"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecOrganizationAddr::from_any(&s).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let organization_id = {
        let py_val_any = obj
            .get_item("organization_id")?
            .ok_or_else(|| PyKeyError::new_err("organization_id"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            match py_val_str.to_str()?.parse() {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let device_id = {
        let py_val_any = obj
            .get_item("device_id")?
            .ok_or_else(|| PyKeyError::new_err("device_id"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let user_id = {
        let py_val_any = obj
            .get_item("user_id")?
            .ok_or_else(|| PyKeyError::new_err("user_id"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let device_label = {
        let py_val_any = obj
            .get_item("device_label")?
            .ok_or_else(|| PyKeyError::new_err("device_label"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            match py_val_str.to_str()?.parse() {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let human_handle = {
        let py_val_any = obj
            .get_item("human_handle")?
            .ok_or_else(|| PyKeyError::new_err("human_handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyDict>()?;
        {
            let py_val_dict: &Bound<'_, PyDict> = py_val_downcasted;
            struct_human_handle_py_to_rs(py, py_val_dict)?
        }
    };
    let current_profile = {
        let py_val_any = obj
            .get_item("current_profile")?
            .ok_or_else(|| PyKeyError::new_err("current_profile"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let raw_value = py_val_downcasted.extract::<&str>()?;
            enum_user_profile_py_to_rs(py, raw_value)?
        }
    };
    let server_config = {
        let py_val_any = obj
            .get_item("server_config")?
            .ok_or_else(|| PyKeyError::new_err("server_config"))?;
        let py_val_downcasted = py_val_any.downcast::<PyDict>()?;
        {
            let py_val_dict: &Bound<'_, PyDict> = py_val_downcasted;
            struct_server_config_py_to_rs(py, py_val_dict)?
        }
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

#[allow(dead_code, unused_variables)]
fn struct_client_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientInfo,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_organization_addr = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |addr: libparsec::ParsecOrganizationAddr| -> Result<String, &'static str> {
                Ok(addr.to_url().into())
            };
        match custom_to_rs_string(rs_obj.organization_addr) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("organization_addr", py_organization_addr)?;
    let py_organization_id = PyString::new_bound(py, rs_obj.organization_id.as_ref()).into_any();
    py_obj.set_item("organization_id", py_organization_id)?;
    let py_device_id = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.device_id) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("device_id", py_device_id)?;
    let py_user_id = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.user_id) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("user_id", py_user_id)?;
    let py_device_label = PyString::new_bound(py, rs_obj.device_label.as_ref()).into_any();
    py_obj.set_item("device_label", py_device_label)?;
    let py_human_handle = struct_human_handle_rs_to_py(py, rs_obj.human_handle)?.into_any();
    py_obj.set_item("human_handle", py_human_handle)?;
    let py_current_profile =
        PyString::new_bound(py, enum_user_profile_rs_to_py(rs_obj.current_profile)).into_any();
    py_obj.set_item("current_profile", py_current_profile)?;
    let py_server_config = struct_server_config_rs_to_py(py, rs_obj.server_config)?.into_any();
    py_obj.set_item("server_config", py_server_config)?;
    Ok(py_obj)
}

// DeviceClaimFinalizeInfo

#[allow(dead_code, unused_variables)]
fn struct_device_claim_finalize_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::DeviceClaimFinalizeInfo> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    Ok(libparsec::DeviceClaimFinalizeInfo { handle })
}

#[allow(dead_code, unused_variables)]
fn struct_device_claim_finalize_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::DeviceClaimFinalizeInfo,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    Ok(py_obj)
}

// DeviceClaimInProgress1Info

#[allow(dead_code, unused_variables)]
fn struct_device_claim_in_progress1_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::DeviceClaimInProgress1Info> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    let greeter_sas = {
        let py_val_any = obj
            .get_item("greeter_sas")?
            .ok_or_else(|| PyKeyError::new_err("greeter_sas"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            match py_val_str.to_str()?.parse() {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let greeter_sas_choices = {
        let py_val_any = obj
            .get_item("greeter_sas_choices")?
            .ok_or_else(|| PyKeyError::new_err("greeter_sas_choices"))?;
        let py_val_downcasted = py_val_any.downcast::<PyList>()?;
        {
            let py_val_list: &Bound<'_, PyList> = py_val_downcasted;
            let size = py_val_list.len();
            let mut v = Vec::with_capacity(size);
            for i in 0..size {
                let py_item_any = py_val_list.get_item(i)?;
                let py_item = py_item_any.downcast::<PyString>()?;
                v.push({
                    let py_val_str: &Bound<'_, PyString> = py_item;
                    match py_val_str.to_str()?.parse() {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
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

#[allow(dead_code, unused_variables)]
fn struct_device_claim_in_progress1_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::DeviceClaimInProgress1Info,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    let py_greeter_sas = PyString::new_bound(py, rs_obj.greeter_sas.as_ref()).into_any();
    py_obj.set_item("greeter_sas", py_greeter_sas)?;
    let py_greeter_sas_choices = {
        let py_list = PyList::empty_bound(py);
        for rs_elem in rs_obj.greeter_sas_choices.into_iter() {
            let py_elem = PyString::new_bound(py, rs_elem.as_ref()).into_any();
            py_list.append(py_elem)?;
        }
        py_list.to_owned().into_any()
    };
    py_obj.set_item("greeter_sas_choices", py_greeter_sas_choices)?;
    Ok(py_obj)
}

// DeviceClaimInProgress2Info

#[allow(dead_code, unused_variables)]
fn struct_device_claim_in_progress2_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::DeviceClaimInProgress2Info> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    let claimer_sas = {
        let py_val_any = obj
            .get_item("claimer_sas")?
            .ok_or_else(|| PyKeyError::new_err("claimer_sas"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            match py_val_str.to_str()?.parse() {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    Ok(libparsec::DeviceClaimInProgress2Info {
        handle,
        claimer_sas,
    })
}

#[allow(dead_code, unused_variables)]
fn struct_device_claim_in_progress2_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::DeviceClaimInProgress2Info,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    let py_claimer_sas = PyString::new_bound(py, rs_obj.claimer_sas.as_ref()).into_any();
    py_obj.set_item("claimer_sas", py_claimer_sas)?;
    Ok(py_obj)
}

// DeviceClaimInProgress3Info

#[allow(dead_code, unused_variables)]
fn struct_device_claim_in_progress3_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::DeviceClaimInProgress3Info> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    Ok(libparsec::DeviceClaimInProgress3Info { handle })
}

#[allow(dead_code, unused_variables)]
fn struct_device_claim_in_progress3_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::DeviceClaimInProgress3Info,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    Ok(py_obj)
}

// DeviceGreetInProgress1Info

#[allow(dead_code, unused_variables)]
fn struct_device_greet_in_progress1_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::DeviceGreetInProgress1Info> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    let greeter_sas = {
        let py_val_any = obj
            .get_item("greeter_sas")?
            .ok_or_else(|| PyKeyError::new_err("greeter_sas"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            match py_val_str.to_str()?.parse() {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    Ok(libparsec::DeviceGreetInProgress1Info {
        handle,
        greeter_sas,
    })
}

#[allow(dead_code, unused_variables)]
fn struct_device_greet_in_progress1_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::DeviceGreetInProgress1Info,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    let py_greeter_sas = PyString::new_bound(py, rs_obj.greeter_sas.as_ref()).into_any();
    py_obj.set_item("greeter_sas", py_greeter_sas)?;
    Ok(py_obj)
}

// DeviceGreetInProgress2Info

#[allow(dead_code, unused_variables)]
fn struct_device_greet_in_progress2_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::DeviceGreetInProgress2Info> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    let claimer_sas = {
        let py_val_any = obj
            .get_item("claimer_sas")?
            .ok_or_else(|| PyKeyError::new_err("claimer_sas"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            match py_val_str.to_str()?.parse() {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let claimer_sas_choices = {
        let py_val_any = obj
            .get_item("claimer_sas_choices")?
            .ok_or_else(|| PyKeyError::new_err("claimer_sas_choices"))?;
        let py_val_downcasted = py_val_any.downcast::<PyList>()?;
        {
            let py_val_list: &Bound<'_, PyList> = py_val_downcasted;
            let size = py_val_list.len();
            let mut v = Vec::with_capacity(size);
            for i in 0..size {
                let py_item_any = py_val_list.get_item(i)?;
                let py_item = py_item_any.downcast::<PyString>()?;
                v.push({
                    let py_val_str: &Bound<'_, PyString> = py_item;
                    match py_val_str.to_str()?.parse() {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
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

#[allow(dead_code, unused_variables)]
fn struct_device_greet_in_progress2_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::DeviceGreetInProgress2Info,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    let py_claimer_sas = PyString::new_bound(py, rs_obj.claimer_sas.as_ref()).into_any();
    py_obj.set_item("claimer_sas", py_claimer_sas)?;
    let py_claimer_sas_choices = {
        let py_list = PyList::empty_bound(py);
        for rs_elem in rs_obj.claimer_sas_choices.into_iter() {
            let py_elem = PyString::new_bound(py, rs_elem.as_ref()).into_any();
            py_list.append(py_elem)?;
        }
        py_list.to_owned().into_any()
    };
    py_obj.set_item("claimer_sas_choices", py_claimer_sas_choices)?;
    Ok(py_obj)
}

// DeviceGreetInProgress3Info

#[allow(dead_code, unused_variables)]
fn struct_device_greet_in_progress3_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::DeviceGreetInProgress3Info> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    Ok(libparsec::DeviceGreetInProgress3Info { handle })
}

#[allow(dead_code, unused_variables)]
fn struct_device_greet_in_progress3_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::DeviceGreetInProgress3Info,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    Ok(py_obj)
}

// DeviceGreetInProgress4Info

#[allow(dead_code, unused_variables)]
fn struct_device_greet_in_progress4_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::DeviceGreetInProgress4Info> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    let requested_device_label = {
        let py_val_any = obj
            .get_item("requested_device_label")?
            .ok_or_else(|| PyKeyError::new_err("requested_device_label"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            match py_val_str.to_str()?.parse() {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    Ok(libparsec::DeviceGreetInProgress4Info {
        handle,
        requested_device_label,
    })
}

#[allow(dead_code, unused_variables)]
fn struct_device_greet_in_progress4_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::DeviceGreetInProgress4Info,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    let py_requested_device_label =
        PyString::new_bound(py, rs_obj.requested_device_label.as_ref()).into_any();
    py_obj.set_item("requested_device_label", py_requested_device_label)?;
    Ok(py_obj)
}

// DeviceGreetInitialInfo

#[allow(dead_code, unused_variables)]
fn struct_device_greet_initial_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::DeviceGreetInitialInfo> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    Ok(libparsec::DeviceGreetInitialInfo { handle })
}

#[allow(dead_code, unused_variables)]
fn struct_device_greet_initial_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::DeviceGreetInitialInfo,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    Ok(py_obj)
}

// DeviceInfo

#[allow(dead_code, unused_variables)]
fn struct_device_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::DeviceInfo> {
    let id = {
        let py_val_any = obj
            .get_item("id")?
            .ok_or_else(|| PyKeyError::new_err("id"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let device_label = {
        let py_val_any = obj
            .get_item("device_label")?
            .ok_or_else(|| PyKeyError::new_err("device_label"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            match py_val_str.to_str()?.parse() {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let created_on = {
        let py_val_any = obj
            .get_item("created_on")?
            .ok_or_else(|| PyKeyError::new_err("created_on"))?;
        let py_val_downcasted = py_val_any.downcast::<PyFloat>()?;
        {
            let v = py_val_downcasted.extract::<f64>()?;
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let created_by = {
        let py_val_any = obj
            .get_item("created_by")?
            .ok_or_else(|| PyKeyError::new_err("created_by"))?;
        let py_val_downcasted = py_val_any.downcast::<PyAny>()?;
        {
            if py_val_downcasted.is_none() {
                None
            } else {
                let py_val_nested = py_val_downcasted.downcast::<PyString>()?;
                Some({
                    let py_val_str: &Bound<'_, PyString> = py_val_nested;
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
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

#[allow(dead_code, unused_variables)]
fn struct_device_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::DeviceInfo,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_id = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.id) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("id", py_id)?;
    let py_device_label = PyString::new_bound(py, rs_obj.device_label.as_ref()).into_any();
    py_obj.set_item("device_label", py_device_label)?;
    let py_created_on = PyFloat::new_bound(py, {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        match custom_to_rs_f64(rs_obj.created_on) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("created_on", py_created_on)?;
    let py_created_by = match rs_obj.created_by {
        Some(elem) => PyString::new_bound(py, &{
            let custom_to_rs_string =
                |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
            match custom_to_rs_string(elem) {
                Ok(ok) => ok,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        })
        .into_any(),
        None => PyNone::get_bound(py).to_owned().into_any(),
    };
    py_obj.set_item("created_by", py_created_by)?;
    Ok(py_obj)
}

// HumanHandle

#[allow(dead_code, unused_variables)]
fn struct_human_handle_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::HumanHandle> {
    let email = {
        let py_val_any = obj
            .get_item("email")?
            .ok_or_else(|| PyKeyError::new_err("email"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            py_val_str.to_str()?.to_owned()
        }
    };
    let label = {
        let py_val_any = obj
            .get_item("label")?
            .ok_or_else(|| PyKeyError::new_err("label"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            py_val_str.to_str()?.to_owned()
        }
    };
    {
        let custom_init = |email: String, label: String| -> Result<_, String> {
            libparsec::HumanHandle::new(&email, &label).map_err(|e| e.to_string())
        };
        custom_init(email, label).map_err(|e| PyValueError::new_err(e))
    }
}

#[allow(dead_code, unused_variables)]
fn struct_human_handle_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::HumanHandle,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_email = {
        let custom_getter = |obj| {
            fn access(obj: &libparsec::HumanHandle) -> &str {
                obj.email()
            }
            access(obj)
        };
        PyString::new_bound(py, custom_getter(&rs_obj).as_ref()).into_any()
    };
    py_obj.set_item("email", py_email)?;
    let py_label = {
        let custom_getter = |obj| {
            fn access(obj: &libparsec::HumanHandle) -> &str {
                obj.label()
            }
            access(obj)
        };
        PyString::new_bound(py, custom_getter(&rs_obj).as_ref()).into_any()
    };
    py_obj.set_item("label", py_label)?;
    Ok(py_obj)
}

// NewInvitationInfo

#[allow(dead_code, unused_variables)]
fn struct_new_invitation_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::NewInvitationInfo> {
    let addr = {
        let py_val_any = obj
            .get_item("addr")?
            .ok_or_else(|| PyKeyError::new_err("addr"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecInvitationAddr::from_any(&s).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let token = {
        let py_val_any = obj
            .get_item("token")?
            .ok_or_else(|| PyKeyError::new_err("token"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string = |s: String| -> Result<libparsec::InvitationToken, _> {
                libparsec::InvitationToken::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let email_sent_status = {
        let py_val_any = obj
            .get_item("email_sent_status")?
            .ok_or_else(|| PyKeyError::new_err("email_sent_status"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let raw_value = py_val_downcasted.extract::<&str>()?;
            enum_invitation_email_sent_status_py_to_rs(py, raw_value)?
        }
    };
    Ok(libparsec::NewInvitationInfo {
        addr,
        token,
        email_sent_status,
    })
}

#[allow(dead_code, unused_variables)]
fn struct_new_invitation_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::NewInvitationInfo,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_addr = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |addr: libparsec::ParsecInvitationAddr| -> Result<String, &'static str> {
                Ok(addr.to_url().into())
            };
        match custom_to_rs_string(rs_obj.addr) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("addr", py_addr)?;
    let py_token = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.token) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("token", py_token)?;
    let py_email_sent_status = PyString::new_bound(
        py,
        enum_invitation_email_sent_status_rs_to_py(rs_obj.email_sent_status),
    )
    .into_any();
    py_obj.set_item("email_sent_status", py_email_sent_status)?;
    Ok(py_obj)
}

// OpenOptions

#[allow(dead_code, unused_variables)]
fn struct_open_options_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::OpenOptions> {
    let read = {
        let py_val_any = obj
            .get_item("read")?
            .ok_or_else(|| PyKeyError::new_err("read"))?;
        let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
        py_val_downcasted.extract()?
    };
    let write = {
        let py_val_any = obj
            .get_item("write")?
            .ok_or_else(|| PyKeyError::new_err("write"))?;
        let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
        py_val_downcasted.extract()?
    };
    let truncate = {
        let py_val_any = obj
            .get_item("truncate")?
            .ok_or_else(|| PyKeyError::new_err("truncate"))?;
        let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
        py_val_downcasted.extract()?
    };
    let create = {
        let py_val_any = obj
            .get_item("create")?
            .ok_or_else(|| PyKeyError::new_err("create"))?;
        let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
        py_val_downcasted.extract()?
    };
    let create_new = {
        let py_val_any = obj
            .get_item("create_new")?
            .ok_or_else(|| PyKeyError::new_err("create_new"))?;
        let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
        py_val_downcasted.extract()?
    };
    Ok(libparsec::OpenOptions {
        read,
        write,
        truncate,
        create,
        create_new,
    })
}

#[allow(dead_code, unused_variables)]
fn struct_open_options_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::OpenOptions,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_read = PyBool::new_bound(py, rs_obj.read).to_owned().into_any();
    py_obj.set_item("read", py_read)?;
    let py_write = PyBool::new_bound(py, rs_obj.write).to_owned().into_any();
    py_obj.set_item("write", py_write)?;
    let py_truncate = PyBool::new_bound(py, rs_obj.truncate).to_owned().into_any();
    py_obj.set_item("truncate", py_truncate)?;
    let py_create = PyBool::new_bound(py, rs_obj.create).to_owned().into_any();
    py_obj.set_item("create", py_create)?;
    let py_create_new = PyBool::new_bound(py, rs_obj.create_new)
        .to_owned()
        .into_any();
    py_obj.set_item("create_new", py_create_new)?;
    Ok(py_obj)
}

// ServerConfig

#[allow(dead_code, unused_variables)]
fn struct_server_config_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::ServerConfig> {
    let user_profile_outsider_allowed = {
        let py_val_any = obj
            .get_item("user_profile_outsider_allowed")?
            .ok_or_else(|| PyKeyError::new_err("user_profile_outsider_allowed"))?;
        let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
        py_val_downcasted.extract()?
    };
    let active_users_limit = {
        let py_val_any = obj
            .get_item("active_users_limit")?
            .ok_or_else(|| PyKeyError::new_err("active_users_limit"))?;
        let py_val_downcasted = py_val_any.downcast::<PyDict>()?;
        {
            let py_val_dict: &Bound<'_, PyDict> = py_val_downcasted;
            variant_active_users_limit_py_to_rs(py, py_val_dict)?
        }
    };
    Ok(libparsec::ServerConfig {
        user_profile_outsider_allowed,
        active_users_limit,
    })
}

#[allow(dead_code, unused_variables)]
fn struct_server_config_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ServerConfig,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_user_profile_outsider_allowed =
        PyBool::new_bound(py, rs_obj.user_profile_outsider_allowed)
            .to_owned()
            .into_any();
    py_obj.set_item(
        "user_profile_outsider_allowed",
        py_user_profile_outsider_allowed,
    )?;
    let py_active_users_limit =
        variant_active_users_limit_rs_to_py(py, rs_obj.active_users_limit)?.into_any();
    py_obj.set_item("active_users_limit", py_active_users_limit)?;
    Ok(py_obj)
}

// StartedWorkspaceInfo

#[allow(dead_code, unused_variables)]
fn struct_started_workspace_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::StartedWorkspaceInfo> {
    let client = {
        let py_val_any = obj
            .get_item("client")?
            .ok_or_else(|| PyKeyError::new_err("client"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    let id = {
        let py_val_any = obj
            .get_item("id")?
            .ok_or_else(|| PyKeyError::new_err("id"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let current_name = {
        let py_val_any = obj
            .get_item("current_name")?
            .ok_or_else(|| PyKeyError::new_err("current_name"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string = |s: String| -> Result<_, _> {
                s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let current_self_role = {
        let py_val_any = obj
            .get_item("current_self_role")?
            .ok_or_else(|| PyKeyError::new_err("current_self_role"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let raw_value = py_val_downcasted.extract::<&str>()?;
            enum_realm_role_py_to_rs(py, raw_value)?
        }
    };
    let mountpoints = {
        let py_val_any = obj
            .get_item("mountpoints")?
            .ok_or_else(|| PyKeyError::new_err("mountpoints"))?;
        let py_val_downcasted = py_val_any.downcast::<PyList>()?;
        {
            let py_val_list: &Bound<'_, PyList> = py_val_downcasted;
            let size = py_val_list.len();
            let mut v = Vec::with_capacity(size);
            for i in 0..size {
                let py_item_any = py_val_list.get_item(i)?;
                let py_item = py_item_any.downcast::<PyTuple>()?;
                v.push({
                    let py_val_tuple: &Bound<'_, PyTuple> = py_item;
                    let size = py_val_tuple.len();
                    (
                        {
                            let py_item_any = py_item.get_item(0)?;
                            let py_item = py_item_any.downcast::<PyInt>()?;
                            {
                                py_item.extract::<u32>()?
                            }
                        },
                        {
                            let py_item_any = py_item.get_item(1)?;
                            let py_item = py_item_any.downcast::<PyString>()?;
                            {
                                let py_val_str: &Bound<'_, PyString> = py_item;
                                let custom_from_rs_string = |s: String| -> Result<_, &'static str> {
                                    Ok(std::path::PathBuf::from(s))
                                };
                                match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                                    Ok(val) => val,
                                    Err(err) => return Err(PyTypeError::new_err(err)),
                                }
                            }
                        },
                    )
                });
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

#[allow(dead_code, unused_variables)]
fn struct_started_workspace_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::StartedWorkspaceInfo,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_client = (rs_obj.client).to_object(py).into_bound(py);
    py_obj.set_item("client", py_client)?;
    let py_id = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.id) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("id", py_id)?;
    let py_current_name = PyString::new_bound(py, rs_obj.current_name.as_ref()).into_any();
    py_obj.set_item("current_name", py_current_name)?;
    let py_current_self_role =
        PyString::new_bound(py, enum_realm_role_rs_to_py(rs_obj.current_self_role)).into_any();
    py_obj.set_item("current_self_role", py_current_self_role)?;
    let py_mountpoints = {
        let py_list = PyList::empty_bound(py);
        for rs_elem in rs_obj.mountpoints.into_iter() {
            let py_elem = {
                let (x0, x1) = rs_elem;
                let py_x0 = (x0).to_object(py).into_bound(py);
                let py_x1 = PyString::new_bound(py, &{
                    let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                        path.into_os_string()
                            .into_string()
                            .map_err(|_| "Path contains non-utf8 characters")
                    };
                    match custom_to_rs_string(x1) {
                        Ok(ok) => ok,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                })
                .into_any();
                PyTuple::new_bound(py, [py_x0, py_x1]).into_any()
            };
            py_list.append(py_elem)?;
        }
        py_list.to_owned().into_any()
    };
    py_obj.set_item("mountpoints", py_mountpoints)?;
    Ok(py_obj)
}

// UserClaimFinalizeInfo

#[allow(dead_code, unused_variables)]
fn struct_user_claim_finalize_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::UserClaimFinalizeInfo> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    Ok(libparsec::UserClaimFinalizeInfo { handle })
}

#[allow(dead_code, unused_variables)]
fn struct_user_claim_finalize_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::UserClaimFinalizeInfo,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    Ok(py_obj)
}

// UserClaimInProgress1Info

#[allow(dead_code, unused_variables)]
fn struct_user_claim_in_progress1_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::UserClaimInProgress1Info> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    let greeter_sas = {
        let py_val_any = obj
            .get_item("greeter_sas")?
            .ok_or_else(|| PyKeyError::new_err("greeter_sas"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            match py_val_str.to_str()?.parse() {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let greeter_sas_choices = {
        let py_val_any = obj
            .get_item("greeter_sas_choices")?
            .ok_or_else(|| PyKeyError::new_err("greeter_sas_choices"))?;
        let py_val_downcasted = py_val_any.downcast::<PyList>()?;
        {
            let py_val_list: &Bound<'_, PyList> = py_val_downcasted;
            let size = py_val_list.len();
            let mut v = Vec::with_capacity(size);
            for i in 0..size {
                let py_item_any = py_val_list.get_item(i)?;
                let py_item = py_item_any.downcast::<PyString>()?;
                v.push({
                    let py_val_str: &Bound<'_, PyString> = py_item;
                    match py_val_str.to_str()?.parse() {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
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

#[allow(dead_code, unused_variables)]
fn struct_user_claim_in_progress1_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::UserClaimInProgress1Info,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    let py_greeter_sas = PyString::new_bound(py, rs_obj.greeter_sas.as_ref()).into_any();
    py_obj.set_item("greeter_sas", py_greeter_sas)?;
    let py_greeter_sas_choices = {
        let py_list = PyList::empty_bound(py);
        for rs_elem in rs_obj.greeter_sas_choices.into_iter() {
            let py_elem = PyString::new_bound(py, rs_elem.as_ref()).into_any();
            py_list.append(py_elem)?;
        }
        py_list.to_owned().into_any()
    };
    py_obj.set_item("greeter_sas_choices", py_greeter_sas_choices)?;
    Ok(py_obj)
}

// UserClaimInProgress2Info

#[allow(dead_code, unused_variables)]
fn struct_user_claim_in_progress2_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::UserClaimInProgress2Info> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    let claimer_sas = {
        let py_val_any = obj
            .get_item("claimer_sas")?
            .ok_or_else(|| PyKeyError::new_err("claimer_sas"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            match py_val_str.to_str()?.parse() {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    Ok(libparsec::UserClaimInProgress2Info {
        handle,
        claimer_sas,
    })
}

#[allow(dead_code, unused_variables)]
fn struct_user_claim_in_progress2_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::UserClaimInProgress2Info,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    let py_claimer_sas = PyString::new_bound(py, rs_obj.claimer_sas.as_ref()).into_any();
    py_obj.set_item("claimer_sas", py_claimer_sas)?;
    Ok(py_obj)
}

// UserClaimInProgress3Info

#[allow(dead_code, unused_variables)]
fn struct_user_claim_in_progress3_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::UserClaimInProgress3Info> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    Ok(libparsec::UserClaimInProgress3Info { handle })
}

#[allow(dead_code, unused_variables)]
fn struct_user_claim_in_progress3_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::UserClaimInProgress3Info,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    Ok(py_obj)
}

// UserGreetInProgress1Info

#[allow(dead_code, unused_variables)]
fn struct_user_greet_in_progress1_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::UserGreetInProgress1Info> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    let greeter_sas = {
        let py_val_any = obj
            .get_item("greeter_sas")?
            .ok_or_else(|| PyKeyError::new_err("greeter_sas"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            match py_val_str.to_str()?.parse() {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    Ok(libparsec::UserGreetInProgress1Info {
        handle,
        greeter_sas,
    })
}

#[allow(dead_code, unused_variables)]
fn struct_user_greet_in_progress1_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::UserGreetInProgress1Info,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    let py_greeter_sas = PyString::new_bound(py, rs_obj.greeter_sas.as_ref()).into_any();
    py_obj.set_item("greeter_sas", py_greeter_sas)?;
    Ok(py_obj)
}

// UserGreetInProgress2Info

#[allow(dead_code, unused_variables)]
fn struct_user_greet_in_progress2_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::UserGreetInProgress2Info> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    let claimer_sas = {
        let py_val_any = obj
            .get_item("claimer_sas")?
            .ok_or_else(|| PyKeyError::new_err("claimer_sas"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            match py_val_str.to_str()?.parse() {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let claimer_sas_choices = {
        let py_val_any = obj
            .get_item("claimer_sas_choices")?
            .ok_or_else(|| PyKeyError::new_err("claimer_sas_choices"))?;
        let py_val_downcasted = py_val_any.downcast::<PyList>()?;
        {
            let py_val_list: &Bound<'_, PyList> = py_val_downcasted;
            let size = py_val_list.len();
            let mut v = Vec::with_capacity(size);
            for i in 0..size {
                let py_item_any = py_val_list.get_item(i)?;
                let py_item = py_item_any.downcast::<PyString>()?;
                v.push({
                    let py_val_str: &Bound<'_, PyString> = py_item;
                    match py_val_str.to_str()?.parse() {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
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

#[allow(dead_code, unused_variables)]
fn struct_user_greet_in_progress2_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::UserGreetInProgress2Info,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    let py_claimer_sas = PyString::new_bound(py, rs_obj.claimer_sas.as_ref()).into_any();
    py_obj.set_item("claimer_sas", py_claimer_sas)?;
    let py_claimer_sas_choices = {
        let py_list = PyList::empty_bound(py);
        for rs_elem in rs_obj.claimer_sas_choices.into_iter() {
            let py_elem = PyString::new_bound(py, rs_elem.as_ref()).into_any();
            py_list.append(py_elem)?;
        }
        py_list.to_owned().into_any()
    };
    py_obj.set_item("claimer_sas_choices", py_claimer_sas_choices)?;
    Ok(py_obj)
}

// UserGreetInProgress3Info

#[allow(dead_code, unused_variables)]
fn struct_user_greet_in_progress3_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::UserGreetInProgress3Info> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    Ok(libparsec::UserGreetInProgress3Info { handle })
}

#[allow(dead_code, unused_variables)]
fn struct_user_greet_in_progress3_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::UserGreetInProgress3Info,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    Ok(py_obj)
}

// UserGreetInProgress4Info

#[allow(dead_code, unused_variables)]
fn struct_user_greet_in_progress4_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::UserGreetInProgress4Info> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    let requested_human_handle = {
        let py_val_any = obj
            .get_item("requested_human_handle")?
            .ok_or_else(|| PyKeyError::new_err("requested_human_handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyDict>()?;
        {
            let py_val_dict: &Bound<'_, PyDict> = py_val_downcasted;
            struct_human_handle_py_to_rs(py, py_val_dict)?
        }
    };
    let requested_device_label = {
        let py_val_any = obj
            .get_item("requested_device_label")?
            .ok_or_else(|| PyKeyError::new_err("requested_device_label"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            match py_val_str.to_str()?.parse() {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    Ok(libparsec::UserGreetInProgress4Info {
        handle,
        requested_human_handle,
        requested_device_label,
    })
}

#[allow(dead_code, unused_variables)]
fn struct_user_greet_in_progress4_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::UserGreetInProgress4Info,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    let py_requested_human_handle =
        struct_human_handle_rs_to_py(py, rs_obj.requested_human_handle)?.into_any();
    py_obj.set_item("requested_human_handle", py_requested_human_handle)?;
    let py_requested_device_label =
        PyString::new_bound(py, rs_obj.requested_device_label.as_ref()).into_any();
    py_obj.set_item("requested_device_label", py_requested_device_label)?;
    Ok(py_obj)
}

// UserGreetInitialInfo

#[allow(dead_code, unused_variables)]
fn struct_user_greet_initial_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::UserGreetInitialInfo> {
    let handle = {
        let py_val_any = obj
            .get_item("handle")?
            .ok_or_else(|| PyKeyError::new_err("handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
        {
            py_val_downcasted.extract::<u32>()?
        }
    };
    Ok(libparsec::UserGreetInitialInfo { handle })
}

#[allow(dead_code, unused_variables)]
fn struct_user_greet_initial_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::UserGreetInitialInfo,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_handle = (rs_obj.handle).to_object(py).into_bound(py);
    py_obj.set_item("handle", py_handle)?;
    Ok(py_obj)
}

// UserInfo

#[allow(dead_code, unused_variables)]
fn struct_user_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::UserInfo> {
    let id = {
        let py_val_any = obj
            .get_item("id")?
            .ok_or_else(|| PyKeyError::new_err("id"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let human_handle = {
        let py_val_any = obj
            .get_item("human_handle")?
            .ok_or_else(|| PyKeyError::new_err("human_handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyDict>()?;
        {
            let py_val_dict: &Bound<'_, PyDict> = py_val_downcasted;
            struct_human_handle_py_to_rs(py, py_val_dict)?
        }
    };
    let current_profile = {
        let py_val_any = obj
            .get_item("current_profile")?
            .ok_or_else(|| PyKeyError::new_err("current_profile"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let raw_value = py_val_downcasted.extract::<&str>()?;
            enum_user_profile_py_to_rs(py, raw_value)?
        }
    };
    let created_on = {
        let py_val_any = obj
            .get_item("created_on")?
            .ok_or_else(|| PyKeyError::new_err("created_on"))?;
        let py_val_downcasted = py_val_any.downcast::<PyFloat>()?;
        {
            let v = py_val_downcasted.extract::<f64>()?;
            let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                    .map_err(|_| "Out-of-bound datetime")
            };
            match custom_from_rs_f64(v) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let created_by = {
        let py_val_any = obj
            .get_item("created_by")?
            .ok_or_else(|| PyKeyError::new_err("created_by"))?;
        let py_val_downcasted = py_val_any.downcast::<PyAny>()?;
        {
            if py_val_downcasted.is_none() {
                None
            } else {
                let py_val_nested = py_val_downcasted.downcast::<PyString>()?;
                Some({
                    let py_val_str: &Bound<'_, PyString> = py_val_nested;
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                })
            }
        }
    };
    let revoked_on = {
        let py_val_any = obj
            .get_item("revoked_on")?
            .ok_or_else(|| PyKeyError::new_err("revoked_on"))?;
        let py_val_downcasted = py_val_any.downcast::<PyAny>()?;
        {
            if py_val_downcasted.is_none() {
                None
            } else {
                let py_val_nested = py_val_downcasted.downcast::<PyFloat>()?;
                Some({
                    let v = py_val_nested.extract::<f64>()?;
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                })
            }
        }
    };
    let revoked_by = {
        let py_val_any = obj
            .get_item("revoked_by")?
            .ok_or_else(|| PyKeyError::new_err("revoked_by"))?;
        let py_val_downcasted = py_val_any.downcast::<PyAny>()?;
        {
            if py_val_downcasted.is_none() {
                None
            } else {
                let py_val_nested = py_val_downcasted.downcast::<PyString>()?;
                Some({
                    let py_val_str: &Bound<'_, PyString> = py_val_nested;
                    let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
                        libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
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

#[allow(dead_code, unused_variables)]
fn struct_user_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::UserInfo,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_id = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.id) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("id", py_id)?;
    let py_human_handle = struct_human_handle_rs_to_py(py, rs_obj.human_handle)?.into_any();
    py_obj.set_item("human_handle", py_human_handle)?;
    let py_current_profile =
        PyString::new_bound(py, enum_user_profile_rs_to_py(rs_obj.current_profile)).into_any();
    py_obj.set_item("current_profile", py_current_profile)?;
    let py_created_on = PyFloat::new_bound(py, {
        let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
            Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
        };
        match custom_to_rs_f64(rs_obj.created_on) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("created_on", py_created_on)?;
    let py_created_by = match rs_obj.created_by {
        Some(elem) => PyString::new_bound(py, &{
            let custom_to_rs_string =
                |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
            match custom_to_rs_string(elem) {
                Ok(ok) => ok,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        })
        .into_any(),
        None => PyNone::get_bound(py).to_owned().into_any(),
    };
    py_obj.set_item("created_by", py_created_by)?;
    let py_revoked_on = match rs_obj.revoked_on {
        Some(elem) => PyFloat::new_bound(py, {
            let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
            };
            match custom_to_rs_f64(elem) {
                Ok(ok) => ok,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        })
        .into_any(),
        None => PyNone::get_bound(py).to_owned().into_any(),
    };
    py_obj.set_item("revoked_on", py_revoked_on)?;
    let py_revoked_by = match rs_obj.revoked_by {
        Some(elem) => PyString::new_bound(py, &{
            let custom_to_rs_string =
                |x: libparsec::DeviceID| -> Result<String, &'static str> { Ok(x.hex()) };
            match custom_to_rs_string(elem) {
                Ok(ok) => ok,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        })
        .into_any(),
        None => PyNone::get_bound(py).to_owned().into_any(),
    };
    py_obj.set_item("revoked_by", py_revoked_by)?;
    Ok(py_obj)
}

// WorkspaceInfo

#[allow(dead_code, unused_variables)]
fn struct_workspace_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::WorkspaceInfo> {
    let id = {
        let py_val_any = obj
            .get_item("id")?
            .ok_or_else(|| PyKeyError::new_err("id"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let current_name = {
        let py_val_any = obj
            .get_item("current_name")?
            .ok_or_else(|| PyKeyError::new_err("current_name"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string = |s: String| -> Result<_, _> {
                s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
            };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let current_self_role = {
        let py_val_any = obj
            .get_item("current_self_role")?
            .ok_or_else(|| PyKeyError::new_err("current_self_role"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let raw_value = py_val_downcasted.extract::<&str>()?;
            enum_realm_role_py_to_rs(py, raw_value)?
        }
    };
    let is_started = {
        let py_val_any = obj
            .get_item("is_started")?
            .ok_or_else(|| PyKeyError::new_err("is_started"))?;
        let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
        py_val_downcasted.extract()?
    };
    let is_bootstrapped = {
        let py_val_any = obj
            .get_item("is_bootstrapped")?
            .ok_or_else(|| PyKeyError::new_err("is_bootstrapped"))?;
        let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
        py_val_downcasted.extract()?
    };
    Ok(libparsec::WorkspaceInfo {
        id,
        current_name,
        current_self_role,
        is_started,
        is_bootstrapped,
    })
}

#[allow(dead_code, unused_variables)]
fn struct_workspace_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceInfo,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_id = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.id) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("id", py_id)?;
    let py_current_name = PyString::new_bound(py, rs_obj.current_name.as_ref()).into_any();
    py_obj.set_item("current_name", py_current_name)?;
    let py_current_self_role =
        PyString::new_bound(py, enum_realm_role_rs_to_py(rs_obj.current_self_role)).into_any();
    py_obj.set_item("current_self_role", py_current_self_role)?;
    let py_is_started = PyBool::new_bound(py, rs_obj.is_started)
        .to_owned()
        .into_any();
    py_obj.set_item("is_started", py_is_started)?;
    let py_is_bootstrapped = PyBool::new_bound(py, rs_obj.is_bootstrapped)
        .to_owned()
        .into_any();
    py_obj.set_item("is_bootstrapped", py_is_bootstrapped)?;
    Ok(py_obj)
}

// WorkspaceUserAccessInfo

#[allow(dead_code, unused_variables)]
fn struct_workspace_user_access_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::WorkspaceUserAccessInfo> {
    let user_id = {
        let py_val_any = obj
            .get_item("user_id")?
            .ok_or_else(|| PyKeyError::new_err("user_id"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
            let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        }
    };
    let human_handle = {
        let py_val_any = obj
            .get_item("human_handle")?
            .ok_or_else(|| PyKeyError::new_err("human_handle"))?;
        let py_val_downcasted = py_val_any.downcast::<PyDict>()?;
        {
            let py_val_dict: &Bound<'_, PyDict> = py_val_downcasted;
            struct_human_handle_py_to_rs(py, py_val_dict)?
        }
    };
    let current_profile = {
        let py_val_any = obj
            .get_item("current_profile")?
            .ok_or_else(|| PyKeyError::new_err("current_profile"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let raw_value = py_val_downcasted.extract::<&str>()?;
            enum_user_profile_py_to_rs(py, raw_value)?
        }
    };
    let current_role = {
        let py_val_any = obj
            .get_item("current_role")?
            .ok_or_else(|| PyKeyError::new_err("current_role"))?;
        let py_val_downcasted = py_val_any.downcast::<PyString>()?;
        {
            let raw_value = py_val_downcasted.extract::<&str>()?;
            enum_realm_role_py_to_rs(py, raw_value)?
        }
    };
    Ok(libparsec::WorkspaceUserAccessInfo {
        user_id,
        human_handle,
        current_profile,
        current_role,
    })
}

#[allow(dead_code, unused_variables)]
fn struct_workspace_user_access_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceUserAccessInfo,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_user_id = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
        match custom_to_rs_string(rs_obj.user_id) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();
    py_obj.set_item("user_id", py_user_id)?;
    let py_human_handle = struct_human_handle_rs_to_py(py, rs_obj.human_handle)?.into_any();
    py_obj.set_item("human_handle", py_human_handle)?;
    let py_current_profile =
        PyString::new_bound(py, enum_user_profile_rs_to_py(rs_obj.current_profile)).into_any();
    py_obj.set_item("current_profile", py_current_profile)?;
    let py_current_role =
        PyString::new_bound(py, enum_realm_role_rs_to_py(rs_obj.current_role)).into_any();
    py_obj.set_item("current_role", py_current_role)?;
    Ok(py_obj)
}

// ActiveUsersLimit

#[allow(dead_code, unused_variables)]
fn variant_active_users_limit_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::ActiveUsersLimit> {
    let tag = obj
        .get_item("tag")?
        .ok_or_else(|| PyKeyError::new_err("tag"))?;
    match tag.downcast::<PyString>()?.to_str()? {
        "ActiveUsersLimitLimitedTo" => {
            let x0 = {
                let py_val_any = obj
                    .get_item("x0")?
                    .ok_or_else(|| PyKeyError::new_err("x0"))?;
                let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
                {
                    py_val_downcasted.extract::<u64>()?
                }
            };
            Ok(libparsec::ActiveUsersLimit::LimitedTo(x0))
        }
        "ActiveUsersLimitNoLimit" => Ok(libparsec::ActiveUsersLimit::NoLimit {}),
        _ => Err(PyTypeError::new_err("Object is not a ActiveUsersLimit")),
    }
}

#[allow(dead_code, unused_variables)]
fn variant_active_users_limit_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ActiveUsersLimit,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    match rs_obj {
        libparsec::ActiveUsersLimit::LimitedTo(x0, ..) => {
            let py_tag = PyString::new_bound(py, "LimitedTo");
            py_obj.set_item("tag", py_tag)?;
            let py_x0 = (x0).to_object(py).into_bound(py);
            py_obj.set_item("x0", py_x0)?;
        }
        libparsec::ActiveUsersLimit::NoLimit { .. } => {
            let py_tag = PyString::new_bound(py, "ActiveUsersLimitNoLimit");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// BootstrapOrganizationError

#[allow(dead_code, unused_variables)]
fn variant_bootstrap_organization_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::BootstrapOrganizationError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::BootstrapOrganizationError::AlreadyUsedToken { .. } => {
            let py_tag = PyString::new_bound(py, "BootstrapOrganizationErrorAlreadyUsedToken");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::BootstrapOrganizationError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "BootstrapOrganizationErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::BootstrapOrganizationError::InvalidToken { .. } => {
            let py_tag = PyString::new_bound(py, "BootstrapOrganizationErrorInvalidToken");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::BootstrapOrganizationError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "BootstrapOrganizationErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::BootstrapOrganizationError::OrganizationExpired { .. } => {
            let py_tag = PyString::new_bound(py, "BootstrapOrganizationErrorOrganizationExpired");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::BootstrapOrganizationError::SaveDeviceError { .. } => {
            let py_tag = PyString::new_bound(py, "BootstrapOrganizationErrorSaveDeviceError");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::BootstrapOrganizationError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let py_tag =
                PyString::new_bound(py, "BootstrapOrganizationErrorTimestampOutOfBallpark");
            py_obj.set_item("tag", py_tag)?;
            let py_server_timestamp = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("server_timestamp", py_server_timestamp)?;
            let py_client_timestamp = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("client_timestamp", py_client_timestamp)?;
            let py_ballpark_client_early_offset =
                PyFloat::new_bound(py, ballpark_client_early_offset).into_any();
            py_obj.set_item(
                "ballpark_client_early_offset",
                py_ballpark_client_early_offset,
            )?;
            let py_ballpark_client_late_offset =
                PyFloat::new_bound(py, ballpark_client_late_offset).into_any();
            py_obj.set_item(
                "ballpark_client_late_offset",
                py_ballpark_client_late_offset,
            )?;
        }
    }
    Ok(py_obj)
}

// CancelError

#[allow(dead_code, unused_variables)]
fn variant_cancel_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::CancelError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::CancelError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "CancelErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::CancelError::NotBound { .. } => {
            let py_tag = PyString::new_bound(py, "CancelErrorNotBound");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClaimInProgressError

#[allow(dead_code, unused_variables)]
fn variant_claim_in_progress_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClaimInProgressError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClaimInProgressError::ActiveUsersLimitReached { .. } => {
            let py_tag = PyString::new_bound(py, "ClaimInProgressErrorActiveUsersLimitReached");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClaimInProgressError::AlreadyUsed { .. } => {
            let py_tag = PyString::new_bound(py, "ClaimInProgressErrorAlreadyUsed");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClaimInProgressError::Cancelled { .. } => {
            let py_tag = PyString::new_bound(py, "ClaimInProgressErrorCancelled");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClaimInProgressError::CorruptedConfirmation { .. } => {
            let py_tag = PyString::new_bound(py, "ClaimInProgressErrorCorruptedConfirmation");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClaimInProgressError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClaimInProgressErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClaimInProgressError::NotFound { .. } => {
            let py_tag = PyString::new_bound(py, "ClaimInProgressErrorNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClaimInProgressError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "ClaimInProgressErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClaimInProgressError::OrganizationExpired { .. } => {
            let py_tag = PyString::new_bound(py, "ClaimInProgressErrorOrganizationExpired");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClaimInProgressError::PeerReset { .. } => {
            let py_tag = PyString::new_bound(py, "ClaimInProgressErrorPeerReset");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClaimerGreeterAbortOperationError

#[allow(dead_code, unused_variables)]
fn variant_claimer_greeter_abort_operation_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClaimerGreeterAbortOperationError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClaimerGreeterAbortOperationError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClaimerGreeterAbortOperationErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClaimerRetrieveInfoError

#[allow(dead_code, unused_variables)]
fn variant_claimer_retrieve_info_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClaimerRetrieveInfoError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClaimerRetrieveInfoError::AlreadyUsed { .. } => {
            let py_tag = PyString::new_bound(py, "ClaimerRetrieveInfoErrorAlreadyUsed");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClaimerRetrieveInfoError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClaimerRetrieveInfoErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClaimerRetrieveInfoError::NotFound { .. } => {
            let py_tag = PyString::new_bound(py, "ClaimerRetrieveInfoErrorNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClaimerRetrieveInfoError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "ClaimerRetrieveInfoErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientCancelInvitationError

#[allow(dead_code, unused_variables)]
fn variant_client_cancel_invitation_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientCancelInvitationError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientCancelInvitationError::AlreadyDeleted { .. } => {
            let py_tag = PyString::new_bound(py, "ClientCancelInvitationErrorAlreadyDeleted");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientCancelInvitationError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientCancelInvitationErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientCancelInvitationError::NotFound { .. } => {
            let py_tag = PyString::new_bound(py, "ClientCancelInvitationErrorNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientCancelInvitationError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "ClientCancelInvitationErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientChangeAuthenticationError

#[allow(dead_code, unused_variables)]
fn variant_client_change_authentication_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientChangeAuthenticationError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientChangeAuthenticationError::DecryptionFailed { .. } => {
            let py_tag = PyString::new_bound(py, "ClientChangeAuthenticationErrorDecryptionFailed");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientChangeAuthenticationError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientChangeAuthenticationErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientChangeAuthenticationError::InvalidData { .. } => {
            let py_tag = PyString::new_bound(py, "ClientChangeAuthenticationErrorInvalidData");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientChangeAuthenticationError::InvalidPath { .. } => {
            let py_tag = PyString::new_bound(py, "ClientChangeAuthenticationErrorInvalidPath");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientCreateWorkspaceError

#[allow(dead_code, unused_variables)]
fn variant_client_create_workspace_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientCreateWorkspaceError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientCreateWorkspaceError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientCreateWorkspaceErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientCreateWorkspaceError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "ClientCreateWorkspaceErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientEvent

#[allow(dead_code, unused_variables)]
fn variant_client_event_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::ClientEvent> {
    let tag = obj
        .get_item("tag")?
        .ok_or_else(|| PyKeyError::new_err("tag"))?;
    match tag.downcast::<PyString>()?.to_str()? {
        "ClientEventExpiredOrganization" => Ok(libparsec::ClientEvent::ExpiredOrganization {}),
        "ClientEventIncompatibleServer" => {
            let detail = {
                let py_val_any = obj
                    .get_item("detail")?
                    .ok_or_else(|| PyKeyError::new_err("detail"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    py_val_str.to_str()?.to_owned()
                }
            };
            Ok(libparsec::ClientEvent::IncompatibleServer { detail })
        }
        "ClientEventInvitationChanged" => {
            let token = {
                let py_val_any = obj
                    .get_item("token")?
                    .ok_or_else(|| PyKeyError::new_err("token"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string =
                        |s: String| -> Result<libparsec::InvitationToken, _> {
                            libparsec::InvitationToken::from_hex(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let status = {
                let py_val_any = obj
                    .get_item("status")?
                    .ok_or_else(|| PyKeyError::new_err("status"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let raw_value = py_val_downcasted.extract::<&str>()?;
                    enum_invitation_status_py_to_rs(py, raw_value)?
                }
            };
            Ok(libparsec::ClientEvent::InvitationChanged { token, status })
        }
        "ClientEventOffline" => Ok(libparsec::ClientEvent::Offline {}),
        "ClientEventOnline" => Ok(libparsec::ClientEvent::Online {}),
        "ClientEventPing" => {
            let ping = {
                let py_val_any = obj
                    .get_item("ping")?
                    .ok_or_else(|| PyKeyError::new_err("ping"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    py_val_str.to_str()?.to_owned()
                }
            };
            Ok(libparsec::ClientEvent::Ping { ping })
        }
        "ClientEventRevokedSelfUser" => Ok(libparsec::ClientEvent::RevokedSelfUser {}),
        "ClientEventServerConfigChanged" => Ok(libparsec::ClientEvent::ServerConfigChanged {}),
        "ClientEventTooMuchDriftWithServerClock" => {
            let server_timestamp = {
                let py_val_any = obj
                    .get_item("server_timestamp")?
                    .ok_or_else(|| PyKeyError::new_err("server_timestamp"))?;
                let py_val_downcasted = py_val_any.downcast::<PyFloat>()?;
                {
                    let v = py_val_downcasted.extract::<f64>()?;
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let client_timestamp = {
                let py_val_any = obj
                    .get_item("client_timestamp")?
                    .ok_or_else(|| PyKeyError::new_err("client_timestamp"))?;
                let py_val_downcasted = py_val_any.downcast::<PyFloat>()?;
                {
                    let v = py_val_downcasted.extract::<f64>()?;
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let ballpark_client_early_offset = {
                let py_val_any = obj
                    .get_item("ballpark_client_early_offset")?
                    .ok_or_else(|| PyKeyError::new_err("ballpark_client_early_offset"))?;
                let py_val_downcasted = py_val_any.downcast::<PyFloat>()?;
                py_val_downcasted.extract()?
            };
            let ballpark_client_late_offset = {
                let py_val_any = obj
                    .get_item("ballpark_client_late_offset")?
                    .ok_or_else(|| PyKeyError::new_err("ballpark_client_late_offset"))?;
                let py_val_downcasted = py_val_any.downcast::<PyFloat>()?;
                py_val_downcasted.extract()?
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
        "ClientEventWorkspacesSelfAccessChanged" => {
            Ok(libparsec::ClientEvent::WorkspacesSelfAccessChanged {})
        }
        _ => Err(PyTypeError::new_err("Object is not a ClientEvent")),
    }
}

#[allow(dead_code, unused_variables)]
fn variant_client_event_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientEvent,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    match rs_obj {
        libparsec::ClientEvent::ExpiredOrganization { .. } => {
            let py_tag = PyString::new_bound(py, "ClientEventExpiredOrganization");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientEvent::IncompatibleServer { detail, .. } => {
            let py_tag = PyString::new_bound(py, "ClientEventIncompatibleServer");
            py_obj.set_item("tag", py_tag)?;
            let py_detail = PyString::new_bound(py, detail.as_ref()).into_any();
            py_obj.set_item("detail", py_detail)?;
        }
        libparsec::ClientEvent::InvitationChanged { token, status, .. } => {
            let py_tag = PyString::new_bound(py, "ClientEventInvitationChanged");
            py_obj.set_item("tag", py_tag)?;
            let py_token = PyString::new_bound(py, &{
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("token", py_token)?;
            let py_status =
                PyString::new_bound(py, enum_invitation_status_rs_to_py(status)).into_any();
            py_obj.set_item("status", py_status)?;
        }
        libparsec::ClientEvent::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "ClientEventOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientEvent::Online { .. } => {
            let py_tag = PyString::new_bound(py, "ClientEventOnline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientEvent::Ping { ping, .. } => {
            let py_tag = PyString::new_bound(py, "ClientEventPing");
            py_obj.set_item("tag", py_tag)?;
            let py_ping = PyString::new_bound(py, ping.as_ref()).into_any();
            py_obj.set_item("ping", py_ping)?;
        }
        libparsec::ClientEvent::RevokedSelfUser { .. } => {
            let py_tag = PyString::new_bound(py, "ClientEventRevokedSelfUser");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientEvent::ServerConfigChanged { .. } => {
            let py_tag = PyString::new_bound(py, "ClientEventServerConfigChanged");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientEvent::TooMuchDriftWithServerClock {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let py_tag = PyString::new_bound(py, "ClientEventTooMuchDriftWithServerClock");
            py_obj.set_item("tag", py_tag)?;
            let py_server_timestamp = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("server_timestamp", py_server_timestamp)?;
            let py_client_timestamp = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("client_timestamp", py_client_timestamp)?;
            let py_ballpark_client_early_offset =
                PyFloat::new_bound(py, ballpark_client_early_offset).into_any();
            py_obj.set_item(
                "ballpark_client_early_offset",
                py_ballpark_client_early_offset,
            )?;
            let py_ballpark_client_late_offset =
                PyFloat::new_bound(py, ballpark_client_late_offset).into_any();
            py_obj.set_item(
                "ballpark_client_late_offset",
                py_ballpark_client_late_offset,
            )?;
        }
        libparsec::ClientEvent::WorkspaceLocallyCreated { .. } => {
            let py_tag = PyString::new_bound(py, "ClientEventWorkspaceLocallyCreated");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientEvent::WorkspacesSelfAccessChanged { .. } => {
            let py_tag = PyString::new_bound(py, "ClientEventWorkspacesSelfAccessChanged");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientGetUserDeviceError

#[allow(dead_code, unused_variables)]
fn variant_client_get_user_device_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientGetUserDeviceError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientGetUserDeviceError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientGetUserDeviceErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientGetUserDeviceError::NonExisting { .. } => {
            let py_tag = PyString::new_bound(py, "ClientGetUserDeviceErrorNonExisting");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientGetUserDeviceError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "ClientGetUserDeviceErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientInfoError

#[allow(dead_code, unused_variables)]
fn variant_client_info_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientInfoError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientInfoError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientInfoErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientInfoError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "ClientInfoErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientListUserDevicesError

#[allow(dead_code, unused_variables)]
fn variant_client_list_user_devices_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientListUserDevicesError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientListUserDevicesError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientListUserDevicesErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientListUserDevicesError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "ClientListUserDevicesErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientListUsersError

#[allow(dead_code, unused_variables)]
fn variant_client_list_users_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientListUsersError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientListUsersError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientListUsersErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientListUsersError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "ClientListUsersErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientListWorkspaceUsersError

#[allow(dead_code, unused_variables)]
fn variant_client_list_workspace_users_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientListWorkspaceUsersError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientListWorkspaceUsersError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientListWorkspaceUsersErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientListWorkspaceUsersError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "ClientListWorkspaceUsersErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientListWorkspacesError

#[allow(dead_code, unused_variables)]
fn variant_client_list_workspaces_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientListWorkspacesError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientListWorkspacesError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientListWorkspacesErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientNewDeviceInvitationError

#[allow(dead_code, unused_variables)]
fn variant_client_new_device_invitation_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientNewDeviceInvitationError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientNewDeviceInvitationError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientNewDeviceInvitationErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientNewDeviceInvitationError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "ClientNewDeviceInvitationErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientNewUserInvitationError

#[allow(dead_code, unused_variables)]
fn variant_client_new_user_invitation_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientNewUserInvitationError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientNewUserInvitationError::AlreadyMember { .. } => {
            let py_tag = PyString::new_bound(py, "ClientNewUserInvitationErrorAlreadyMember");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientNewUserInvitationError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientNewUserInvitationErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientNewUserInvitationError::NotAllowed { .. } => {
            let py_tag = PyString::new_bound(py, "ClientNewUserInvitationErrorNotAllowed");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientNewUserInvitationError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "ClientNewUserInvitationErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientRenameWorkspaceError

#[allow(dead_code, unused_variables)]
fn variant_client_rename_workspace_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientRenameWorkspaceError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientRenameWorkspaceError::AuthorNotAllowed { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRenameWorkspaceErrorAuthorNotAllowed");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRenameWorkspaceErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::InvalidCertificate { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRenameWorkspaceErrorInvalidCertificate");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::InvalidEncryptedRealmName { .. } => {
            let py_tag =
                PyString::new_bound(py, "ClientRenameWorkspaceErrorInvalidEncryptedRealmName");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::InvalidKeysBundle { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRenameWorkspaceErrorInvalidKeysBundle");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::NoKey { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRenameWorkspaceErrorNoKey");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRenameWorkspaceErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRenameWorkspaceErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRenameWorkspaceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let py_tag =
                PyString::new_bound(py, "ClientRenameWorkspaceErrorTimestampOutOfBallpark");
            py_obj.set_item("tag", py_tag)?;
            let py_server_timestamp = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("server_timestamp", py_server_timestamp)?;
            let py_client_timestamp = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("client_timestamp", py_client_timestamp)?;
            let py_ballpark_client_early_offset =
                PyFloat::new_bound(py, ballpark_client_early_offset).into_any();
            py_obj.set_item(
                "ballpark_client_early_offset",
                py_ballpark_client_early_offset,
            )?;
            let py_ballpark_client_late_offset =
                PyFloat::new_bound(py, ballpark_client_late_offset).into_any();
            py_obj.set_item(
                "ballpark_client_late_offset",
                py_ballpark_client_late_offset,
            )?;
        }
        libparsec::ClientRenameWorkspaceError::WorkspaceNotFound { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRenameWorkspaceErrorWorkspaceNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientRevokeUserError

#[allow(dead_code, unused_variables)]
fn variant_client_revoke_user_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientRevokeUserError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientRevokeUserError::AuthorNotAllowed { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRevokeUserErrorAuthorNotAllowed");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRevokeUserError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRevokeUserErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRevokeUserError::InvalidCertificate { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRevokeUserErrorInvalidCertificate");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRevokeUserError::InvalidKeysBundle { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRevokeUserErrorInvalidKeysBundle");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRevokeUserError::NoKey { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRevokeUserErrorNoKey");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRevokeUserError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRevokeUserErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRevokeUserError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRevokeUserErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRevokeUserError::TimestampOutOfBallpark { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRevokeUserErrorTimestampOutOfBallpark");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRevokeUserError::UserIsSelf { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRevokeUserErrorUserIsSelf");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientRevokeUserError::UserNotFound { .. } => {
            let py_tag = PyString::new_bound(py, "ClientRevokeUserErrorUserNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientShareWorkspaceError

#[allow(dead_code, unused_variables)]
fn variant_client_share_workspace_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientShareWorkspaceError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientShareWorkspaceError::AuthorNotAllowed { .. } => {
            let py_tag = PyString::new_bound(py, "ClientShareWorkspaceErrorAuthorNotAllowed");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientShareWorkspaceError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientShareWorkspaceErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientShareWorkspaceError::InvalidCertificate { .. } => {
            let py_tag = PyString::new_bound(py, "ClientShareWorkspaceErrorInvalidCertificate");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientShareWorkspaceError::InvalidKeysBundle { .. } => {
            let py_tag = PyString::new_bound(py, "ClientShareWorkspaceErrorInvalidKeysBundle");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientShareWorkspaceError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "ClientShareWorkspaceErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientShareWorkspaceError::RecipientIsSelf { .. } => {
            let py_tag = PyString::new_bound(py, "ClientShareWorkspaceErrorRecipientIsSelf");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientShareWorkspaceError::RecipientNotFound { .. } => {
            let py_tag = PyString::new_bound(py, "ClientShareWorkspaceErrorRecipientNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientShareWorkspaceError::RecipientRevoked { .. } => {
            let py_tag = PyString::new_bound(py, "ClientShareWorkspaceErrorRecipientRevoked");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientShareWorkspaceError::RoleIncompatibleWithOutsider { .. } => {
            let py_tag =
                PyString::new_bound(py, "ClientShareWorkspaceErrorRoleIncompatibleWithOutsider");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientShareWorkspaceError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "ClientShareWorkspaceErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientShareWorkspaceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let py_tag = PyString::new_bound(py, "ClientShareWorkspaceErrorTimestampOutOfBallpark");
            py_obj.set_item("tag", py_tag)?;
            let py_server_timestamp = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("server_timestamp", py_server_timestamp)?;
            let py_client_timestamp = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("client_timestamp", py_client_timestamp)?;
            let py_ballpark_client_early_offset =
                PyFloat::new_bound(py, ballpark_client_early_offset).into_any();
            py_obj.set_item(
                "ballpark_client_early_offset",
                py_ballpark_client_early_offset,
            )?;
            let py_ballpark_client_late_offset =
                PyFloat::new_bound(py, ballpark_client_late_offset).into_any();
            py_obj.set_item(
                "ballpark_client_late_offset",
                py_ballpark_client_late_offset,
            )?;
        }
        libparsec::ClientShareWorkspaceError::WorkspaceNotFound { .. } => {
            let py_tag = PyString::new_bound(py, "ClientShareWorkspaceErrorWorkspaceNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientStartError

#[allow(dead_code, unused_variables)]
fn variant_client_start_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientStartError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientStartError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientStartErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientStartError::LoadDeviceDecryptionFailed { .. } => {
            let py_tag = PyString::new_bound(py, "ClientStartErrorLoadDeviceDecryptionFailed");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientStartError::LoadDeviceInvalidData { .. } => {
            let py_tag = PyString::new_bound(py, "ClientStartErrorLoadDeviceInvalidData");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientStartError::LoadDeviceInvalidPath { .. } => {
            let py_tag = PyString::new_bound(py, "ClientStartErrorLoadDeviceInvalidPath");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientStartInvitationGreetError

#[allow(dead_code, unused_variables)]
fn variant_client_start_invitation_greet_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientStartInvitationGreetError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientStartInvitationGreetError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientStartInvitationGreetErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientStartWorkspaceError

#[allow(dead_code, unused_variables)]
fn variant_client_start_workspace_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientStartWorkspaceError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientStartWorkspaceError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientStartWorkspaceErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ClientStartWorkspaceError::WorkspaceNotFound { .. } => {
            let py_tag = PyString::new_bound(py, "ClientStartWorkspaceErrorWorkspaceNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ClientStopError

#[allow(dead_code, unused_variables)]
fn variant_client_stop_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ClientStopError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ClientStopError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ClientStopErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// DeviceAccessStrategy

#[allow(dead_code, unused_variables)]
fn variant_device_access_strategy_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::DeviceAccessStrategy> {
    let tag = obj
        .get_item("tag")?
        .ok_or_else(|| PyKeyError::new_err("tag"))?;
    match tag.downcast::<PyString>()?.to_str()? {
        "DeviceAccessStrategyKeyring" => {
            let key_file = {
                let py_val_any = obj
                    .get_item("key_file")?
                    .ok_or_else(|| PyKeyError::new_err("key_file"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string =
                        |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            Ok(libparsec::DeviceAccessStrategy::Keyring { key_file })
        }
        "DeviceAccessStrategyPassword" => {
            let password = {
                let py_val_any = obj
                    .get_item("password")?
                    .ok_or_else(|| PyKeyError::new_err("password"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string = |s: String| -> Result<_, String> { Ok(s.into()) };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let key_file = {
                let py_val_any = obj
                    .get_item("key_file")?
                    .ok_or_else(|| PyKeyError::new_err("key_file"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string =
                        |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            Ok(libparsec::DeviceAccessStrategy::Password { password, key_file })
        }
        "DeviceAccessStrategySmartcard" => {
            let key_file = {
                let py_val_any = obj
                    .get_item("key_file")?
                    .ok_or_else(|| PyKeyError::new_err("key_file"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string =
                        |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            Ok(libparsec::DeviceAccessStrategy::Smartcard { key_file })
        }
        _ => Err(PyTypeError::new_err("Object is not a DeviceAccessStrategy")),
    }
}

#[allow(dead_code, unused_variables)]
fn variant_device_access_strategy_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::DeviceAccessStrategy,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    match rs_obj {
        libparsec::DeviceAccessStrategy::Keyring { key_file, .. } => {
            let py_tag = PyString::new_bound(py, "DeviceAccessStrategyKeyring");
            py_obj.set_item("tag", py_tag)?;
            let py_key_file = PyString::new_bound(py, &{
                let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                };
                match custom_to_rs_string(key_file) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("key_file", py_key_file)?;
        }
        libparsec::DeviceAccessStrategy::Password {
            password, key_file, ..
        } => {
            let py_tag = PyString::new_bound(py, "DeviceAccessStrategyPassword");
            py_obj.set_item("tag", py_tag)?;
            let py_password = PyString::new_bound(py, password.as_ref()).into_any();
            py_obj.set_item("password", py_password)?;
            let py_key_file = PyString::new_bound(py, &{
                let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                };
                match custom_to_rs_string(key_file) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("key_file", py_key_file)?;
        }
        libparsec::DeviceAccessStrategy::Smartcard { key_file, .. } => {
            let py_tag = PyString::new_bound(py, "DeviceAccessStrategySmartcard");
            py_obj.set_item("tag", py_tag)?;
            let py_key_file = PyString::new_bound(py, &{
                let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                };
                match custom_to_rs_string(key_file) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("key_file", py_key_file)?;
        }
    }
    Ok(py_obj)
}

// DeviceSaveStrategy

#[allow(dead_code, unused_variables)]
fn variant_device_save_strategy_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::DeviceSaveStrategy> {
    let tag = obj
        .get_item("tag")?
        .ok_or_else(|| PyKeyError::new_err("tag"))?;
    match tag.downcast::<PyString>()?.to_str()? {
        "DeviceSaveStrategyKeyring" => Ok(libparsec::DeviceSaveStrategy::Keyring {}),
        "DeviceSaveStrategyPassword" => {
            let password = {
                let py_val_any = obj
                    .get_item("password")?
                    .ok_or_else(|| PyKeyError::new_err("password"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string = |s: String| -> Result<_, String> { Ok(s.into()) };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            Ok(libparsec::DeviceSaveStrategy::Password { password })
        }
        "DeviceSaveStrategySmartcard" => Ok(libparsec::DeviceSaveStrategy::Smartcard {}),
        _ => Err(PyTypeError::new_err("Object is not a DeviceSaveStrategy")),
    }
}

#[allow(dead_code, unused_variables)]
fn variant_device_save_strategy_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::DeviceSaveStrategy,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    match rs_obj {
        libparsec::DeviceSaveStrategy::Keyring { .. } => {
            let py_tag = PyString::new_bound(py, "DeviceSaveStrategyKeyring");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::DeviceSaveStrategy::Password { password, .. } => {
            let py_tag = PyString::new_bound(py, "DeviceSaveStrategyPassword");
            py_obj.set_item("tag", py_tag)?;
            let py_password = PyString::new_bound(py, password.as_ref()).into_any();
            py_obj.set_item("password", py_password)?;
        }
        libparsec::DeviceSaveStrategy::Smartcard { .. } => {
            let py_tag = PyString::new_bound(py, "DeviceSaveStrategySmartcard");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// EntryStat

#[allow(dead_code, unused_variables)]
fn variant_entry_stat_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::EntryStat> {
    let tag = obj
        .get_item("tag")?
        .ok_or_else(|| PyKeyError::new_err("tag"))?;
    match tag.downcast::<PyString>()?.to_str()? {
        "EntryStatFile" => {
            let confinement_point = {
                let py_val_any = obj
                    .get_item("confinement_point")?
                    .ok_or_else(|| PyKeyError::new_err("confinement_point"))?;
                let py_val_downcasted = py_val_any.downcast::<PyAny>()?;
                {
                    if py_val_downcasted.is_none() {
                        None
                    } else {
                        let py_val_nested = py_val_downcasted.downcast::<PyString>()?;
                        Some({
                            let py_val_str: &Bound<'_, PyString> = py_val_nested;
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::VlobID, _> {
                                    libparsec::VlobID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                                Ok(val) => val,
                                Err(err) => return Err(PyTypeError::new_err(err)),
                            }
                        })
                    }
                }
            };
            let id = {
                let py_val_any = obj
                    .get_item("id")?
                    .ok_or_else(|| PyKeyError::new_err("id"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                        libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let parent = {
                let py_val_any = obj
                    .get_item("parent")?
                    .ok_or_else(|| PyKeyError::new_err("parent"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                        libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let created = {
                let py_val_any = obj
                    .get_item("created")?
                    .ok_or_else(|| PyKeyError::new_err("created"))?;
                let py_val_downcasted = py_val_any.downcast::<PyFloat>()?;
                {
                    let v = py_val_downcasted.extract::<f64>()?;
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let updated = {
                let py_val_any = obj
                    .get_item("updated")?
                    .ok_or_else(|| PyKeyError::new_err("updated"))?;
                let py_val_downcasted = py_val_any.downcast::<PyFloat>()?;
                {
                    let v = py_val_downcasted.extract::<f64>()?;
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let base_version = {
                let py_val_any = obj
                    .get_item("base_version")?
                    .ok_or_else(|| PyKeyError::new_err("base_version"))?;
                let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
                {
                    py_val_downcasted.extract::<u32>()?
                }
            };
            let is_placeholder = {
                let py_val_any = obj
                    .get_item("is_placeholder")?
                    .ok_or_else(|| PyKeyError::new_err("is_placeholder"))?;
                let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
                py_val_downcasted.extract()?
            };
            let need_sync = {
                let py_val_any = obj
                    .get_item("need_sync")?
                    .ok_or_else(|| PyKeyError::new_err("need_sync"))?;
                let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
                py_val_downcasted.extract()?
            };
            let size = {
                let py_val_any = obj
                    .get_item("size")?
                    .ok_or_else(|| PyKeyError::new_err("size"))?;
                let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
                {
                    py_val_downcasted.extract::<u64>()?
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
                let py_val_any = obj
                    .get_item("confinement_point")?
                    .ok_or_else(|| PyKeyError::new_err("confinement_point"))?;
                let py_val_downcasted = py_val_any.downcast::<PyAny>()?;
                {
                    if py_val_downcasted.is_none() {
                        None
                    } else {
                        let py_val_nested = py_val_downcasted.downcast::<PyString>()?;
                        Some({
                            let py_val_str: &Bound<'_, PyString> = py_val_nested;
                            let custom_from_rs_string =
                                |s: String| -> Result<libparsec::VlobID, _> {
                                    libparsec::VlobID::from_hex(s.as_str())
                                        .map_err(|e| e.to_string())
                                };
                            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                                Ok(val) => val,
                                Err(err) => return Err(PyTypeError::new_err(err)),
                            }
                        })
                    }
                }
            };
            let id = {
                let py_val_any = obj
                    .get_item("id")?
                    .ok_or_else(|| PyKeyError::new_err("id"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                        libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let parent = {
                let py_val_any = obj
                    .get_item("parent")?
                    .ok_or_else(|| PyKeyError::new_err("parent"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                        libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let created = {
                let py_val_any = obj
                    .get_item("created")?
                    .ok_or_else(|| PyKeyError::new_err("created"))?;
                let py_val_downcasted = py_val_any.downcast::<PyFloat>()?;
                {
                    let v = py_val_downcasted.extract::<f64>()?;
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let updated = {
                let py_val_any = obj
                    .get_item("updated")?
                    .ok_or_else(|| PyKeyError::new_err("updated"))?;
                let py_val_downcasted = py_val_any.downcast::<PyFloat>()?;
                {
                    let v = py_val_downcasted.extract::<f64>()?;
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let base_version = {
                let py_val_any = obj
                    .get_item("base_version")?
                    .ok_or_else(|| PyKeyError::new_err("base_version"))?;
                let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
                {
                    py_val_downcasted.extract::<u32>()?
                }
            };
            let is_placeholder = {
                let py_val_any = obj
                    .get_item("is_placeholder")?
                    .ok_or_else(|| PyKeyError::new_err("is_placeholder"))?;
                let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
                py_val_downcasted.extract()?
            };
            let need_sync = {
                let py_val_any = obj
                    .get_item("need_sync")?
                    .ok_or_else(|| PyKeyError::new_err("need_sync"))?;
                let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
                py_val_downcasted.extract()?
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
        _ => Err(PyTypeError::new_err("Object is not a EntryStat")),
    }
}

#[allow(dead_code, unused_variables)]
fn variant_entry_stat_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::EntryStat,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
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
            let py_tag = PyString::new_bound(py, "EntryStatFile");
            py_obj.set_item("tag", py_tag)?;
            let py_confinement_point = match confinement_point {
                Some(elem) => PyString::new_bound(py, &{
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(elem) {
                        Ok(ok) => ok,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                })
                .into_any(),
                None => PyNone::get_bound(py).to_owned().into_any(),
            };
            py_obj.set_item("confinement_point", py_confinement_point)?;
            let py_id = PyString::new_bound(py, &{
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("id", py_id)?;
            let py_parent = PyString::new_bound(py, &{
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(parent) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("parent", py_parent)?;
            let py_created = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("created", py_created)?;
            let py_updated = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(updated) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("updated", py_updated)?;
            let py_base_version = (base_version).to_object(py).into_bound(py);
            py_obj.set_item("base_version", py_base_version)?;
            let py_is_placeholder = PyBool::new_bound(py, is_placeholder).to_owned().into_any();
            py_obj.set_item("is_placeholder", py_is_placeholder)?;
            let py_need_sync = PyBool::new_bound(py, need_sync).to_owned().into_any();
            py_obj.set_item("need_sync", py_need_sync)?;
            let py_size = (size).to_object(py).into_bound(py);
            py_obj.set_item("size", py_size)?;
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
            let py_tag = PyString::new_bound(py, "EntryStatFolder");
            py_obj.set_item("tag", py_tag)?;
            let py_confinement_point = match confinement_point {
                Some(elem) => PyString::new_bound(py, &{
                    let custom_to_rs_string =
                        |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                    match custom_to_rs_string(elem) {
                        Ok(ok) => ok,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                })
                .into_any(),
                None => PyNone::get_bound(py).to_owned().into_any(),
            };
            py_obj.set_item("confinement_point", py_confinement_point)?;
            let py_id = PyString::new_bound(py, &{
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("id", py_id)?;
            let py_parent = PyString::new_bound(py, &{
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(parent) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("parent", py_parent)?;
            let py_created = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("created", py_created)?;
            let py_updated = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(updated) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("updated", py_updated)?;
            let py_base_version = (base_version).to_object(py).into_bound(py);
            py_obj.set_item("base_version", py_base_version)?;
            let py_is_placeholder = PyBool::new_bound(py, is_placeholder).to_owned().into_any();
            py_obj.set_item("is_placeholder", py_is_placeholder)?;
            let py_need_sync = PyBool::new_bound(py, need_sync).to_owned().into_any();
            py_obj.set_item("need_sync", py_need_sync)?;
        }
    }
    Ok(py_obj)
}

// GreetInProgressError

#[allow(dead_code, unused_variables)]
fn variant_greet_in_progress_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::GreetInProgressError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::GreetInProgressError::ActiveUsersLimitReached { .. } => {
            let py_tag = PyString::new_bound(py, "GreetInProgressErrorActiveUsersLimitReached");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::GreetInProgressError::AlreadyDeleted { .. } => {
            let py_tag = PyString::new_bound(py, "GreetInProgressErrorAlreadyDeleted");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::GreetInProgressError::Cancelled { .. } => {
            let py_tag = PyString::new_bound(py, "GreetInProgressErrorCancelled");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::GreetInProgressError::CorruptedInviteUserData { .. } => {
            let py_tag = PyString::new_bound(py, "GreetInProgressErrorCorruptedInviteUserData");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::GreetInProgressError::DeviceAlreadyExists { .. } => {
            let py_tag = PyString::new_bound(py, "GreetInProgressErrorDeviceAlreadyExists");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::GreetInProgressError::HumanHandleAlreadyTaken { .. } => {
            let py_tag = PyString::new_bound(py, "GreetInProgressErrorHumanHandleAlreadyTaken");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::GreetInProgressError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "GreetInProgressErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::GreetInProgressError::NonceMismatch { .. } => {
            let py_tag = PyString::new_bound(py, "GreetInProgressErrorNonceMismatch");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::GreetInProgressError::NotFound { .. } => {
            let py_tag = PyString::new_bound(py, "GreetInProgressErrorNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::GreetInProgressError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "GreetInProgressErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::GreetInProgressError::PeerReset { .. } => {
            let py_tag = PyString::new_bound(py, "GreetInProgressErrorPeerReset");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::GreetInProgressError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let py_tag = PyString::new_bound(py, "GreetInProgressErrorTimestampOutOfBallpark");
            py_obj.set_item("tag", py_tag)?;
            let py_server_timestamp = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(server_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("server_timestamp", py_server_timestamp)?;
            let py_client_timestamp = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(client_timestamp) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("client_timestamp", py_client_timestamp)?;
            let py_ballpark_client_early_offset =
                PyFloat::new_bound(py, ballpark_client_early_offset).into_any();
            py_obj.set_item(
                "ballpark_client_early_offset",
                py_ballpark_client_early_offset,
            )?;
            let py_ballpark_client_late_offset =
                PyFloat::new_bound(py, ballpark_client_late_offset).into_any();
            py_obj.set_item(
                "ballpark_client_late_offset",
                py_ballpark_client_late_offset,
            )?;
        }
        libparsec::GreetInProgressError::UserAlreadyExists { .. } => {
            let py_tag = PyString::new_bound(py, "GreetInProgressErrorUserAlreadyExists");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::GreetInProgressError::UserCreateNotAllowed { .. } => {
            let py_tag = PyString::new_bound(py, "GreetInProgressErrorUserCreateNotAllowed");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// InviteListItem

#[allow(dead_code, unused_variables)]
fn variant_invite_list_item_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::InviteListItem> {
    let tag = obj
        .get_item("tag")?
        .ok_or_else(|| PyKeyError::new_err("tag"))?;
    match tag.downcast::<PyString>()?.to_str()? {
        "InviteListItemDevice" => {
            let addr = {
                let py_val_any = obj
                    .get_item("addr")?
                    .ok_or_else(|| PyKeyError::new_err("addr"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::ParsecInvitationAddr::from_any(&s).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let token = {
                let py_val_any = obj
                    .get_item("token")?
                    .ok_or_else(|| PyKeyError::new_err("token"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string =
                        |s: String| -> Result<libparsec::InvitationToken, _> {
                            libparsec::InvitationToken::from_hex(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let created_on = {
                let py_val_any = obj
                    .get_item("created_on")?
                    .ok_or_else(|| PyKeyError::new_err("created_on"))?;
                let py_val_downcasted = py_val_any.downcast::<PyFloat>()?;
                {
                    let v = py_val_downcasted.extract::<f64>()?;
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let status = {
                let py_val_any = obj
                    .get_item("status")?
                    .ok_or_else(|| PyKeyError::new_err("status"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let raw_value = py_val_downcasted.extract::<&str>()?;
                    enum_invitation_status_py_to_rs(py, raw_value)?
                }
            };
            Ok(libparsec::InviteListItem::Device {
                addr,
                token,
                created_on,
                status,
            })
        }
        "InviteListItemUser" => {
            let addr = {
                let py_val_any = obj
                    .get_item("addr")?
                    .ok_or_else(|| PyKeyError::new_err("addr"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string = |s: String| -> Result<_, String> {
                        libparsec::ParsecInvitationAddr::from_any(&s).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let token = {
                let py_val_any = obj
                    .get_item("token")?
                    .ok_or_else(|| PyKeyError::new_err("token"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string =
                        |s: String| -> Result<libparsec::InvitationToken, _> {
                            libparsec::InvitationToken::from_hex(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let created_on = {
                let py_val_any = obj
                    .get_item("created_on")?
                    .ok_or_else(|| PyKeyError::new_err("created_on"))?;
                let py_val_downcasted = py_val_any.downcast::<PyFloat>()?;
                {
                    let v = py_val_downcasted.extract::<f64>()?;
                    let custom_from_rs_f64 = |n: f64| -> Result<_, &'static str> {
                        libparsec::DateTime::from_timestamp_micros((n * 1_000_000f64) as i64)
                            .map_err(|_| "Out-of-bound datetime")
                    };
                    match custom_from_rs_f64(v) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let claimer_email = {
                let py_val_any = obj
                    .get_item("claimer_email")?
                    .ok_or_else(|| PyKeyError::new_err("claimer_email"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    py_val_str.to_str()?.to_owned()
                }
            };
            let status = {
                let py_val_any = obj
                    .get_item("status")?
                    .ok_or_else(|| PyKeyError::new_err("status"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let raw_value = py_val_downcasted.extract::<&str>()?;
                    enum_invitation_status_py_to_rs(py, raw_value)?
                }
            };
            Ok(libparsec::InviteListItem::User {
                addr,
                token,
                created_on,
                claimer_email,
                status,
            })
        }
        _ => Err(PyTypeError::new_err("Object is not a InviteListItem")),
    }
}

#[allow(dead_code, unused_variables)]
fn variant_invite_list_item_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::InviteListItem,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    match rs_obj {
        libparsec::InviteListItem::Device {
            addr,
            token,
            created_on,
            status,
            ..
        } => {
            let py_tag = PyString::new_bound(py, "InviteListItemDevice");
            py_obj.set_item("tag", py_tag)?;
            let py_addr = PyString::new_bound(py, &{
                let custom_to_rs_string =
                    |addr: libparsec::ParsecInvitationAddr| -> Result<String, &'static str> {
                        Ok(addr.to_url().into())
                    };
                match custom_to_rs_string(addr) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("addr", py_addr)?;
            let py_token = PyString::new_bound(py, &{
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("token", py_token)?;
            let py_created_on = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("created_on", py_created_on)?;
            let py_status =
                PyString::new_bound(py, enum_invitation_status_rs_to_py(status)).into_any();
            py_obj.set_item("status", py_status)?;
        }
        libparsec::InviteListItem::User {
            addr,
            token,
            created_on,
            claimer_email,
            status,
            ..
        } => {
            let py_tag = PyString::new_bound(py, "InviteListItemUser");
            py_obj.set_item("tag", py_tag)?;
            let py_addr = PyString::new_bound(py, &{
                let custom_to_rs_string =
                    |addr: libparsec::ParsecInvitationAddr| -> Result<String, &'static str> {
                        Ok(addr.to_url().into())
                    };
                match custom_to_rs_string(addr) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("addr", py_addr)?;
            let py_token = PyString::new_bound(py, &{
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("token", py_token)?;
            let py_created_on = PyFloat::new_bound(py, {
                let custom_to_rs_f64 = |dt: libparsec::DateTime| -> Result<f64, &'static str> {
                    Ok((dt.as_timestamp_micros() as f64) / 1_000_000f64)
                };
                match custom_to_rs_f64(created_on) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("created_on", py_created_on)?;
            let py_claimer_email = PyString::new_bound(py, claimer_email.as_ref()).into_any();
            py_obj.set_item("claimer_email", py_claimer_email)?;
            let py_status =
                PyString::new_bound(py, enum_invitation_status_rs_to_py(status)).into_any();
            py_obj.set_item("status", py_status)?;
        }
    }
    Ok(py_obj)
}

// ListInvitationsError

#[allow(dead_code, unused_variables)]
fn variant_list_invitations_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ListInvitationsError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ListInvitationsError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "ListInvitationsErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::ListInvitationsError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "ListInvitationsErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// MountpointMountStrategy

#[allow(dead_code, unused_variables)]
fn variant_mountpoint_mount_strategy_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::MountpointMountStrategy> {
    let tag = obj
        .get_item("tag")?
        .ok_or_else(|| PyKeyError::new_err("tag"))?;
    match tag.downcast::<PyString>()?.to_str()? {
        "MountpointMountStrategyDirectory" => {
            let base_dir = {
                let py_val_any = obj
                    .get_item("base_dir")?
                    .ok_or_else(|| PyKeyError::new_err("base_dir"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string =
                        |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            Ok(libparsec::MountpointMountStrategy::Directory { base_dir })
        }
        "MountpointMountStrategyDisabled" => Ok(libparsec::MountpointMountStrategy::Disabled),
        "MountpointMountStrategyDriveLetter" => Ok(libparsec::MountpointMountStrategy::DriveLetter),
        _ => Err(PyTypeError::new_err(
            "Object is not a MountpointMountStrategy",
        )),
    }
}

#[allow(dead_code, unused_variables)]
fn variant_mountpoint_mount_strategy_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::MountpointMountStrategy,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    match rs_obj {
        libparsec::MountpointMountStrategy::Directory { base_dir, .. } => {
            let py_tag = PyString::new_bound(py, "MountpointMountStrategyDirectory");
            py_obj.set_item("tag", py_tag)?;
            let py_base_dir = PyString::new_bound(py, &{
                let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                    path.into_os_string()
                        .into_string()
                        .map_err(|_| "Path contains non-utf8 characters")
                };
                match custom_to_rs_string(base_dir) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("base_dir", py_base_dir)?;
        }
        libparsec::MountpointMountStrategy::Disabled => {
            let py_tag = PyString::new_bound(py, "Disabled");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::MountpointMountStrategy::DriveLetter => {
            let py_tag = PyString::new_bound(py, "DriveLetter");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// MountpointToOsPathError

#[allow(dead_code, unused_variables)]
fn variant_mountpoint_to_os_path_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::MountpointToOsPathError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::MountpointToOsPathError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "MountpointToOsPathErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// MountpointUnmountError

#[allow(dead_code, unused_variables)]
fn variant_mountpoint_unmount_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::MountpointUnmountError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::MountpointUnmountError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "MountpointUnmountErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// MoveEntryMode

#[allow(dead_code, unused_variables)]
fn variant_move_entry_mode_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::MoveEntryMode> {
    let tag = obj
        .get_item("tag")?
        .ok_or_else(|| PyKeyError::new_err("tag"))?;
    match tag.downcast::<PyString>()?.to_str()? {
        "MoveEntryModeCanReplace" => Ok(libparsec::MoveEntryMode::CanReplace),
        "MoveEntryModeExchange" => Ok(libparsec::MoveEntryMode::Exchange),
        "MoveEntryModeNoReplace" => Ok(libparsec::MoveEntryMode::NoReplace),
        _ => Err(PyTypeError::new_err("Object is not a MoveEntryMode")),
    }
}

#[allow(dead_code, unused_variables)]
fn variant_move_entry_mode_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::MoveEntryMode,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    match rs_obj {
        libparsec::MoveEntryMode::CanReplace => {
            let py_tag = PyString::new_bound(py, "CanReplace");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::MoveEntryMode::Exchange => {
            let py_tag = PyString::new_bound(py, "Exchange");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::MoveEntryMode::NoReplace => {
            let py_tag = PyString::new_bound(py, "NoReplace");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ParseParsecAddrError

#[allow(dead_code, unused_variables)]
fn variant_parse_parsec_addr_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ParseParsecAddrError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::ParseParsecAddrError::InvalidUrl { .. } => {
            let py_tag = PyString::new_bound(py, "ParseParsecAddrErrorInvalidUrl");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// ParsedParsecAddr

#[allow(dead_code, unused_variables)]
fn variant_parsed_parsec_addr_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::ParsedParsecAddr> {
    let tag = obj
        .get_item("tag")?
        .ok_or_else(|| PyKeyError::new_err("tag"))?;
    match tag.downcast::<PyString>()?.to_str()? {
        "ParsedParsecAddrInvitationDevice" => {
            let hostname = {
                let py_val_any = obj
                    .get_item("hostname")?
                    .ok_or_else(|| PyKeyError::new_err("hostname"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    py_val_str.to_str()?.to_owned()
                }
            };
            let port = {
                let py_val_any = obj
                    .get_item("port")?
                    .ok_or_else(|| PyKeyError::new_err("port"))?;
                let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
                {
                    py_val_downcasted.extract::<u32>()?
                }
            };
            let use_ssl = {
                let py_val_any = obj
                    .get_item("use_ssl")?
                    .ok_or_else(|| PyKeyError::new_err("use_ssl"))?;
                let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
                py_val_downcasted.extract()?
            };
            let organization_id = {
                let py_val_any = obj
                    .get_item("organization_id")?
                    .ok_or_else(|| PyKeyError::new_err("organization_id"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    match py_val_str.to_str()?.parse() {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let token = {
                let py_val_any = obj
                    .get_item("token")?
                    .ok_or_else(|| PyKeyError::new_err("token"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string =
                        |s: String| -> Result<libparsec::InvitationToken, _> {
                            libparsec::InvitationToken::from_hex(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
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
        "ParsedParsecAddrInvitationUser" => {
            let hostname = {
                let py_val_any = obj
                    .get_item("hostname")?
                    .ok_or_else(|| PyKeyError::new_err("hostname"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    py_val_str.to_str()?.to_owned()
                }
            };
            let port = {
                let py_val_any = obj
                    .get_item("port")?
                    .ok_or_else(|| PyKeyError::new_err("port"))?;
                let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
                {
                    py_val_downcasted.extract::<u32>()?
                }
            };
            let use_ssl = {
                let py_val_any = obj
                    .get_item("use_ssl")?
                    .ok_or_else(|| PyKeyError::new_err("use_ssl"))?;
                let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
                py_val_downcasted.extract()?
            };
            let organization_id = {
                let py_val_any = obj
                    .get_item("organization_id")?
                    .ok_or_else(|| PyKeyError::new_err("organization_id"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    match py_val_str.to_str()?.parse() {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let token = {
                let py_val_any = obj
                    .get_item("token")?
                    .ok_or_else(|| PyKeyError::new_err("token"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string =
                        |s: String| -> Result<libparsec::InvitationToken, _> {
                            libparsec::InvitationToken::from_hex(s.as_str())
                                .map_err(|e| e.to_string())
                        };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
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
                let py_val_any = obj
                    .get_item("hostname")?
                    .ok_or_else(|| PyKeyError::new_err("hostname"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    py_val_str.to_str()?.to_owned()
                }
            };
            let port = {
                let py_val_any = obj
                    .get_item("port")?
                    .ok_or_else(|| PyKeyError::new_err("port"))?;
                let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
                {
                    py_val_downcasted.extract::<u32>()?
                }
            };
            let use_ssl = {
                let py_val_any = obj
                    .get_item("use_ssl")?
                    .ok_or_else(|| PyKeyError::new_err("use_ssl"))?;
                let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
                py_val_downcasted.extract()?
            };
            let organization_id = {
                let py_val_any = obj
                    .get_item("organization_id")?
                    .ok_or_else(|| PyKeyError::new_err("organization_id"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    match py_val_str.to_str()?.parse() {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
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
                let py_val_any = obj
                    .get_item("hostname")?
                    .ok_or_else(|| PyKeyError::new_err("hostname"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    py_val_str.to_str()?.to_owned()
                }
            };
            let port = {
                let py_val_any = obj
                    .get_item("port")?
                    .ok_or_else(|| PyKeyError::new_err("port"))?;
                let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
                {
                    py_val_downcasted.extract::<u32>()?
                }
            };
            let use_ssl = {
                let py_val_any = obj
                    .get_item("use_ssl")?
                    .ok_or_else(|| PyKeyError::new_err("use_ssl"))?;
                let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
                py_val_downcasted.extract()?
            };
            let organization_id = {
                let py_val_any = obj
                    .get_item("organization_id")?
                    .ok_or_else(|| PyKeyError::new_err("organization_id"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    match py_val_str.to_str()?.parse() {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let token = {
                let py_val_any = obj
                    .get_item("token")?
                    .ok_or_else(|| PyKeyError::new_err("token"))?;
                let py_val_downcasted = py_val_any.downcast::<PyAny>()?;
                {
                    if py_val_downcasted.is_none() {
                        None
                    } else {
                        let py_val_nested = py_val_downcasted.downcast::<PyString>()?;
                        Some({
                            let py_val_str: &Bound<'_, PyString> = py_val_nested;
                            py_val_str.to_str()?.to_owned()
                        })
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
                let py_val_any = obj
                    .get_item("hostname")?
                    .ok_or_else(|| PyKeyError::new_err("hostname"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    py_val_str.to_str()?.to_owned()
                }
            };
            let port = {
                let py_val_any = obj
                    .get_item("port")?
                    .ok_or_else(|| PyKeyError::new_err("port"))?;
                let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
                {
                    py_val_downcasted.extract::<u32>()?
                }
            };
            let use_ssl = {
                let py_val_any = obj
                    .get_item("use_ssl")?
                    .ok_or_else(|| PyKeyError::new_err("use_ssl"))?;
                let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
                py_val_downcasted.extract()?
            };
            let organization_id = {
                let py_val_any = obj
                    .get_item("organization_id")?
                    .ok_or_else(|| PyKeyError::new_err("organization_id"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    match py_val_str.to_str()?.parse() {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
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
                let py_val_any = obj
                    .get_item("hostname")?
                    .ok_or_else(|| PyKeyError::new_err("hostname"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    py_val_str.to_str()?.to_owned()
                }
            };
            let port = {
                let py_val_any = obj
                    .get_item("port")?
                    .ok_or_else(|| PyKeyError::new_err("port"))?;
                let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
                {
                    py_val_downcasted.extract::<u32>()?
                }
            };
            let use_ssl = {
                let py_val_any = obj
                    .get_item("use_ssl")?
                    .ok_or_else(|| PyKeyError::new_err("use_ssl"))?;
                let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
                py_val_downcasted.extract()?
            };
            Ok(libparsec::ParsedParsecAddr::Server {
                hostname,
                port,
                use_ssl,
            })
        }
        "ParsedParsecAddrWorkspacePath" => {
            let hostname = {
                let py_val_any = obj
                    .get_item("hostname")?
                    .ok_or_else(|| PyKeyError::new_err("hostname"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    py_val_str.to_str()?.to_owned()
                }
            };
            let port = {
                let py_val_any = obj
                    .get_item("port")?
                    .ok_or_else(|| PyKeyError::new_err("port"))?;
                let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
                {
                    py_val_downcasted.extract::<u32>()?
                }
            };
            let use_ssl = {
                let py_val_any = obj
                    .get_item("use_ssl")?
                    .ok_or_else(|| PyKeyError::new_err("use_ssl"))?;
                let py_val_downcasted = py_val_any.downcast::<PyBool>()?;
                py_val_downcasted.extract()?
            };
            let organization_id = {
                let py_val_any = obj
                    .get_item("organization_id")?
                    .ok_or_else(|| PyKeyError::new_err("organization_id"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    match py_val_str.to_str()?.parse() {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let workspace_id = {
                let py_val_any = obj
                    .get_item("workspace_id")?
                    .ok_or_else(|| PyKeyError::new_err("workspace_id"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
                        libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let key_index = {
                let py_val_any = obj
                    .get_item("key_index")?
                    .ok_or_else(|| PyKeyError::new_err("key_index"))?;
                let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
                {
                    py_val_downcasted.extract::<u64>()?
                }
            };
            let encrypted_path = {
                let py_val_any = obj
                    .get_item("encrypted_path")?
                    .ok_or_else(|| PyKeyError::new_err("encrypted_path"))?;
                let py_val_downcasted = py_val_any.downcast::<PyBytes>()?;
                {
                    let py_val_bytes: &Bound<'_, PyBytes> = py_val_downcasted;
                    py_val_bytes.as_bytes().to_owned()
                }
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
        _ => Err(PyTypeError::new_err("Object is not a ParsedParsecAddr")),
    }
}

#[allow(dead_code, unused_variables)]
fn variant_parsed_parsec_addr_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::ParsedParsecAddr,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    match rs_obj {
        libparsec::ParsedParsecAddr::InvitationDevice {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            let py_tag = PyString::new_bound(py, "ParsedParsecAddrInvitationDevice");
            py_obj.set_item("tag", py_tag)?;
            let py_hostname = PyString::new_bound(py, hostname.as_ref()).into_any();
            py_obj.set_item("hostname", py_hostname)?;
            let py_port = (port).to_object(py).into_bound(py);
            py_obj.set_item("port", py_port)?;
            let py_use_ssl = PyBool::new_bound(py, use_ssl).to_owned().into_any();
            py_obj.set_item("use_ssl", py_use_ssl)?;
            let py_organization_id = PyString::new_bound(py, organization_id.as_ref()).into_any();
            py_obj.set_item("organization_id", py_organization_id)?;
            let py_token = PyString::new_bound(py, &{
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("token", py_token)?;
        }
        libparsec::ParsedParsecAddr::InvitationUser {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            let py_tag = PyString::new_bound(py, "ParsedParsecAddrInvitationUser");
            py_obj.set_item("tag", py_tag)?;
            let py_hostname = PyString::new_bound(py, hostname.as_ref()).into_any();
            py_obj.set_item("hostname", py_hostname)?;
            let py_port = (port).to_object(py).into_bound(py);
            py_obj.set_item("port", py_port)?;
            let py_use_ssl = PyBool::new_bound(py, use_ssl).to_owned().into_any();
            py_obj.set_item("use_ssl", py_use_ssl)?;
            let py_organization_id = PyString::new_bound(py, organization_id.as_ref()).into_any();
            py_obj.set_item("organization_id", py_organization_id)?;
            let py_token = PyString::new_bound(py, &{
                let custom_to_rs_string =
                    |x: libparsec::InvitationToken| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(token) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("token", py_token)?;
        }
        libparsec::ParsedParsecAddr::Organization {
            hostname,
            port,
            use_ssl,
            organization_id,
            ..
        } => {
            let py_tag = PyString::new_bound(py, "ParsedParsecAddrOrganization");
            py_obj.set_item("tag", py_tag)?;
            let py_hostname = PyString::new_bound(py, hostname.as_ref()).into_any();
            py_obj.set_item("hostname", py_hostname)?;
            let py_port = (port).to_object(py).into_bound(py);
            py_obj.set_item("port", py_port)?;
            let py_use_ssl = PyBool::new_bound(py, use_ssl).to_owned().into_any();
            py_obj.set_item("use_ssl", py_use_ssl)?;
            let py_organization_id = PyString::new_bound(py, organization_id.as_ref()).into_any();
            py_obj.set_item("organization_id", py_organization_id)?;
        }
        libparsec::ParsedParsecAddr::OrganizationBootstrap {
            hostname,
            port,
            use_ssl,
            organization_id,
            token,
            ..
        } => {
            let py_tag = PyString::new_bound(py, "ParsedParsecAddrOrganizationBootstrap");
            py_obj.set_item("tag", py_tag)?;
            let py_hostname = PyString::new_bound(py, hostname.as_ref()).into_any();
            py_obj.set_item("hostname", py_hostname)?;
            let py_port = (port).to_object(py).into_bound(py);
            py_obj.set_item("port", py_port)?;
            let py_use_ssl = PyBool::new_bound(py, use_ssl).to_owned().into_any();
            py_obj.set_item("use_ssl", py_use_ssl)?;
            let py_organization_id = PyString::new_bound(py, organization_id.as_ref()).into_any();
            py_obj.set_item("organization_id", py_organization_id)?;
            let py_token = match token {
                Some(elem) => PyString::new_bound(py, elem.as_ref()).into_any(),
                None => PyNone::get_bound(py).to_owned().into_any(),
            };
            py_obj.set_item("token", py_token)?;
        }
        libparsec::ParsedParsecAddr::PkiEnrollment {
            hostname,
            port,
            use_ssl,
            organization_id,
            ..
        } => {
            let py_tag = PyString::new_bound(py, "ParsedParsecAddrPkiEnrollment");
            py_obj.set_item("tag", py_tag)?;
            let py_hostname = PyString::new_bound(py, hostname.as_ref()).into_any();
            py_obj.set_item("hostname", py_hostname)?;
            let py_port = (port).to_object(py).into_bound(py);
            py_obj.set_item("port", py_port)?;
            let py_use_ssl = PyBool::new_bound(py, use_ssl).to_owned().into_any();
            py_obj.set_item("use_ssl", py_use_ssl)?;
            let py_organization_id = PyString::new_bound(py, organization_id.as_ref()).into_any();
            py_obj.set_item("organization_id", py_organization_id)?;
        }
        libparsec::ParsedParsecAddr::Server {
            hostname,
            port,
            use_ssl,
            ..
        } => {
            let py_tag = PyString::new_bound(py, "ParsedParsecAddrServer");
            py_obj.set_item("tag", py_tag)?;
            let py_hostname = PyString::new_bound(py, hostname.as_ref()).into_any();
            py_obj.set_item("hostname", py_hostname)?;
            let py_port = (port).to_object(py).into_bound(py);
            py_obj.set_item("port", py_port)?;
            let py_use_ssl = PyBool::new_bound(py, use_ssl).to_owned().into_any();
            py_obj.set_item("use_ssl", py_use_ssl)?;
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
            let py_tag = PyString::new_bound(py, "ParsedParsecAddrWorkspacePath");
            py_obj.set_item("tag", py_tag)?;
            let py_hostname = PyString::new_bound(py, hostname.as_ref()).into_any();
            py_obj.set_item("hostname", py_hostname)?;
            let py_port = (port).to_object(py).into_bound(py);
            py_obj.set_item("port", py_port)?;
            let py_use_ssl = PyBool::new_bound(py, use_ssl).to_owned().into_any();
            py_obj.set_item("use_ssl", py_use_ssl)?;
            let py_organization_id = PyString::new_bound(py, organization_id.as_ref()).into_any();
            py_obj.set_item("organization_id", py_organization_id)?;
            let py_workspace_id = PyString::new_bound(py, &{
                let custom_to_rs_string =
                    |x: libparsec::VlobID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(workspace_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("workspace_id", py_workspace_id)?;
            let py_key_index = (key_index).to_object(py).into_bound(py);
            py_obj.set_item("key_index", py_key_index)?;
            let py_encrypted_path = PyBytes::new_bound(py, &encrypted_path).into_any();
            py_obj.set_item("encrypted_path", py_encrypted_path)?;
        }
    }
    Ok(py_obj)
}

// TestbedError

#[allow(dead_code, unused_variables)]
fn variant_testbed_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::TestbedError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::TestbedError::Disabled { .. } => {
            let py_tag = PyString::new_bound(py, "TestbedErrorDisabled");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::TestbedError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "TestbedErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// UserOrDeviceClaimInitialInfo

#[allow(dead_code, unused_variables)]
fn variant_user_or_device_claim_initial_info_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::UserOrDeviceClaimInitialInfo> {
    let tag = obj
        .get_item("tag")?
        .ok_or_else(|| PyKeyError::new_err("tag"))?;
    match tag.downcast::<PyString>()?.to_str()? {
        "UserOrDeviceClaimInitialInfoDevice" => {
            let handle = {
                let py_val_any = obj
                    .get_item("handle")?
                    .ok_or_else(|| PyKeyError::new_err("handle"))?;
                let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
                {
                    py_val_downcasted.extract::<u32>()?
                }
            };
            let greeter_user_id = {
                let py_val_any = obj
                    .get_item("greeter_user_id")?
                    .ok_or_else(|| PyKeyError::new_err("greeter_user_id"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                        libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let greeter_human_handle = {
                let py_val_any = obj
                    .get_item("greeter_human_handle")?
                    .ok_or_else(|| PyKeyError::new_err("greeter_human_handle"))?;
                let py_val_downcasted = py_val_any.downcast::<PyDict>()?;
                {
                    let py_val_dict: &Bound<'_, PyDict> = py_val_downcasted;
                    struct_human_handle_py_to_rs(py, py_val_dict)?
                }
            };
            Ok(libparsec::UserOrDeviceClaimInitialInfo::Device {
                handle,
                greeter_user_id,
                greeter_human_handle,
            })
        }
        "UserOrDeviceClaimInitialInfoUser" => {
            let handle = {
                let py_val_any = obj
                    .get_item("handle")?
                    .ok_or_else(|| PyKeyError::new_err("handle"))?;
                let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
                {
                    py_val_downcasted.extract::<u32>()?
                }
            };
            let claimer_email = {
                let py_val_any = obj
                    .get_item("claimer_email")?
                    .ok_or_else(|| PyKeyError::new_err("claimer_email"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    py_val_str.to_str()?.to_owned()
                }
            };
            let greeter_user_id = {
                let py_val_any = obj
                    .get_item("greeter_user_id")?
                    .ok_or_else(|| PyKeyError::new_err("greeter_user_id"))?;
                let py_val_downcasted = py_val_any.downcast::<PyString>()?;
                {
                    let py_val_str: &Bound<'_, PyString> = py_val_downcasted;
                    let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
                        libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
                    };
                    match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                        Ok(val) => val,
                        Err(err) => return Err(PyTypeError::new_err(err)),
                    }
                }
            };
            let greeter_human_handle = {
                let py_val_any = obj
                    .get_item("greeter_human_handle")?
                    .ok_or_else(|| PyKeyError::new_err("greeter_human_handle"))?;
                let py_val_downcasted = py_val_any.downcast::<PyDict>()?;
                {
                    let py_val_dict: &Bound<'_, PyDict> = py_val_downcasted;
                    struct_human_handle_py_to_rs(py, py_val_dict)?
                }
            };
            Ok(libparsec::UserOrDeviceClaimInitialInfo::User {
                handle,
                claimer_email,
                greeter_user_id,
                greeter_human_handle,
            })
        }
        _ => Err(PyTypeError::new_err(
            "Object is not a UserOrDeviceClaimInitialInfo",
        )),
    }
}

#[allow(dead_code, unused_variables)]
fn variant_user_or_device_claim_initial_info_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::UserOrDeviceClaimInitialInfo,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    match rs_obj {
        libparsec::UserOrDeviceClaimInitialInfo::Device {
            handle,
            greeter_user_id,
            greeter_human_handle,
            ..
        } => {
            let py_tag = PyString::new_bound(py, "UserOrDeviceClaimInitialInfoDevice");
            py_obj.set_item("tag", py_tag)?;
            let py_handle = (handle).to_object(py).into_bound(py);
            py_obj.set_item("handle", py_handle)?;
            let py_greeter_user_id = PyString::new_bound(py, &{
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(greeter_user_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("greeter_user_id", py_greeter_user_id)?;
            let py_greeter_human_handle =
                struct_human_handle_rs_to_py(py, greeter_human_handle)?.into_any();
            py_obj.set_item("greeter_human_handle", py_greeter_human_handle)?;
        }
        libparsec::UserOrDeviceClaimInitialInfo::User {
            handle,
            claimer_email,
            greeter_user_id,
            greeter_human_handle,
            ..
        } => {
            let py_tag = PyString::new_bound(py, "UserOrDeviceClaimInitialInfoUser");
            py_obj.set_item("tag", py_tag)?;
            let py_handle = (handle).to_object(py).into_bound(py);
            py_obj.set_item("handle", py_handle)?;
            let py_claimer_email = PyString::new_bound(py, claimer_email.as_ref()).into_any();
            py_obj.set_item("claimer_email", py_claimer_email)?;
            let py_greeter_user_id = PyString::new_bound(py, &{
                let custom_to_rs_string =
                    |x: libparsec::UserID| -> Result<String, &'static str> { Ok(x.hex()) };
                match custom_to_rs_string(greeter_user_id) {
                    Ok(ok) => ok,
                    Err(err) => return Err(PyTypeError::new_err(err)),
                }
            })
            .into_any();
            py_obj.set_item("greeter_user_id", py_greeter_user_id)?;
            let py_greeter_human_handle =
                struct_human_handle_rs_to_py(py, greeter_human_handle)?.into_any();
            py_obj.set_item("greeter_human_handle", py_greeter_human_handle)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceCreateFileError

#[allow(dead_code, unused_variables)]
fn variant_workspace_create_file_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceCreateFileError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceCreateFileError::EntryExists { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFileErrorEntryExists");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFileError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFileErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFileError::InvalidCertificate { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFileErrorInvalidCertificate");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFileError::InvalidKeysBundle { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFileErrorInvalidKeysBundle");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFileError::InvalidManifest { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFileErrorInvalidManifest");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFileError::NoRealmAccess { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFileErrorNoRealmAccess");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFileError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFileErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFileError::ParentNotAFolder { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFileErrorParentNotAFolder");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFileError::ParentNotFound { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFileErrorParentNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFileError::ReadOnlyRealm { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFileErrorReadOnlyRealm");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFileError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFileErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceCreateFolderError

#[allow(dead_code, unused_variables)]
fn variant_workspace_create_folder_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceCreateFolderError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceCreateFolderError::EntryExists { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFolderErrorEntryExists");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFolderErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::InvalidCertificate { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFolderErrorInvalidCertificate");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::InvalidKeysBundle { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFolderErrorInvalidKeysBundle");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::InvalidManifest { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFolderErrorInvalidManifest");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::NoRealmAccess { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFolderErrorNoRealmAccess");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFolderErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::ParentNotAFolder { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFolderErrorParentNotAFolder");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::ParentNotFound { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFolderErrorParentNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::ReadOnlyRealm { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFolderErrorReadOnlyRealm");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceCreateFolderError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceCreateFolderErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceDecryptPathAddrError

#[allow(dead_code, unused_variables)]
fn variant_workspace_decrypt_path_addr_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceDecryptPathAddrError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceDecryptPathAddrError::CorruptedData { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceDecryptPathAddrErrorCorruptedData");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::CorruptedKey { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceDecryptPathAddrErrorCorruptedKey");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceDecryptPathAddrErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::InvalidCertificate { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceDecryptPathAddrErrorInvalidCertificate");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::InvalidKeysBundle { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceDecryptPathAddrErrorInvalidKeysBundle");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::KeyNotFound { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceDecryptPathAddrErrorKeyNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::NotAllowed { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceDecryptPathAddrErrorNotAllowed");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceDecryptPathAddrErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceDecryptPathAddrError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceDecryptPathAddrErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceFdCloseError

#[allow(dead_code, unused_variables)]
fn variant_workspace_fd_close_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceFdCloseError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceFdCloseError::BadFileDescriptor { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdCloseErrorBadFileDescriptor");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdCloseError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdCloseErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdCloseError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdCloseErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceFdFlushError

#[allow(dead_code, unused_variables)]
fn variant_workspace_fd_flush_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceFdFlushError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceFdFlushError::BadFileDescriptor { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdFlushErrorBadFileDescriptor");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdFlushError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdFlushErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdFlushError::NotInWriteMode { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdFlushErrorNotInWriteMode");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdFlushError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdFlushErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceFdReadError

#[allow(dead_code, unused_variables)]
fn variant_workspace_fd_read_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceFdReadError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceFdReadError::BadFileDescriptor { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdReadErrorBadFileDescriptor");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdReadError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdReadErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdReadError::InvalidBlockAccess { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdReadErrorInvalidBlockAccess");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdReadError::InvalidCertificate { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdReadErrorInvalidCertificate");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdReadError::InvalidKeysBundle { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdReadErrorInvalidKeysBundle");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdReadError::NoRealmAccess { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdReadErrorNoRealmAccess");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdReadError::NotInReadMode { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdReadErrorNotInReadMode");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdReadError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdReadErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdReadError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdReadErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceFdResizeError

#[allow(dead_code, unused_variables)]
fn variant_workspace_fd_resize_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceFdResizeError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceFdResizeError::BadFileDescriptor { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdResizeErrorBadFileDescriptor");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdResizeError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdResizeErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdResizeError::NotInWriteMode { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdResizeErrorNotInWriteMode");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceFdWriteError

#[allow(dead_code, unused_variables)]
fn variant_workspace_fd_write_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceFdWriteError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceFdWriteError::BadFileDescriptor { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdWriteErrorBadFileDescriptor");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdWriteError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdWriteErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceFdWriteError::NotInWriteMode { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceFdWriteErrorNotInWriteMode");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceGeneratePathAddrError

#[allow(dead_code, unused_variables)]
fn variant_workspace_generate_path_addr_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceGeneratePathAddrError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceGeneratePathAddrError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceGeneratePathAddrErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceGeneratePathAddrError::InvalidKeysBundle { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceGeneratePathAddrErrorInvalidKeysBundle");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceGeneratePathAddrError::NoKey { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceGeneratePathAddrErrorNoKey");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceGeneratePathAddrError::NotAllowed { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceGeneratePathAddrErrorNotAllowed");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceGeneratePathAddrError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceGeneratePathAddrErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceGeneratePathAddrError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceGeneratePathAddrErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceInfoError

#[allow(dead_code, unused_variables)]
fn variant_workspace_info_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceInfoError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceInfoError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceInfoErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceMountError

#[allow(dead_code, unused_variables)]
fn variant_workspace_mount_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceMountError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceMountError::Disabled { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceMountErrorDisabled");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceMountError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceMountErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceMoveEntryError

#[allow(dead_code, unused_variables)]
fn variant_workspace_move_entry_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceMoveEntryError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceMoveEntryError::CannotMoveRoot { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceMoveEntryErrorCannotMoveRoot");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::DestinationExists { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceMoveEntryErrorDestinationExists");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::DestinationNotFound { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceMoveEntryErrorDestinationNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceMoveEntryErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::InvalidCertificate { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceMoveEntryErrorInvalidCertificate");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::InvalidKeysBundle { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceMoveEntryErrorInvalidKeysBundle");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::InvalidManifest { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceMoveEntryErrorInvalidManifest");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::NoRealmAccess { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceMoveEntryErrorNoRealmAccess");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceMoveEntryErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::ReadOnlyRealm { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceMoveEntryErrorReadOnlyRealm");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::SourceNotFound { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceMoveEntryErrorSourceNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceMoveEntryError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceMoveEntryErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceOpenFileError

#[allow(dead_code, unused_variables)]
fn variant_workspace_open_file_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceOpenFileError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceOpenFileError::EntryExistsInCreateNewMode { .. } => {
            let py_tag =
                PyString::new_bound(py, "WorkspaceOpenFileErrorEntryExistsInCreateNewMode");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceOpenFileError::EntryNotAFile { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceOpenFileErrorEntryNotAFile");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceOpenFileError::EntryNotFound { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceOpenFileErrorEntryNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceOpenFileError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceOpenFileErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceOpenFileError::InvalidCertificate { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceOpenFileErrorInvalidCertificate");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceOpenFileError::InvalidKeysBundle { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceOpenFileErrorInvalidKeysBundle");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceOpenFileError::InvalidManifest { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceOpenFileErrorInvalidManifest");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceOpenFileError::NoRealmAccess { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceOpenFileErrorNoRealmAccess");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceOpenFileError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceOpenFileErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceOpenFileError::ReadOnlyRealm { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceOpenFileErrorReadOnlyRealm");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceOpenFileError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceOpenFileErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceRemoveEntryError

#[allow(dead_code, unused_variables)]
fn variant_workspace_remove_entry_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceRemoveEntryError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceRemoveEntryError::CannotRemoveRoot { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceRemoveEntryErrorCannotRemoveRoot");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::EntryIsFile { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceRemoveEntryErrorEntryIsFile");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::EntryIsFolder { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceRemoveEntryErrorEntryIsFolder");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::EntryIsNonEmptyFolder { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceRemoveEntryErrorEntryIsNonEmptyFolder");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::EntryNotFound { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceRemoveEntryErrorEntryNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceRemoveEntryErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::InvalidCertificate { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceRemoveEntryErrorInvalidCertificate");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::InvalidKeysBundle { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceRemoveEntryErrorInvalidKeysBundle");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::InvalidManifest { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceRemoveEntryErrorInvalidManifest");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::NoRealmAccess { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceRemoveEntryErrorNoRealmAccess");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceRemoveEntryErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::ReadOnlyRealm { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceRemoveEntryErrorReadOnlyRealm");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceRemoveEntryError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceRemoveEntryErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceStatEntryError

#[allow(dead_code, unused_variables)]
fn variant_workspace_stat_entry_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceStatEntryError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceStatEntryError::EntryNotFound { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStatEntryErrorEntryNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceStatEntryError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStatEntryErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceStatEntryError::InvalidCertificate { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStatEntryErrorInvalidCertificate");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceStatEntryError::InvalidKeysBundle { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStatEntryErrorInvalidKeysBundle");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceStatEntryError::InvalidManifest { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStatEntryErrorInvalidManifest");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceStatEntryError::NoRealmAccess { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStatEntryErrorNoRealmAccess");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceStatEntryError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStatEntryErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceStatEntryError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStatEntryErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceStatFolderChildrenError

#[allow(dead_code, unused_variables)]
fn variant_workspace_stat_folder_children_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceStatFolderChildrenError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceStatFolderChildrenError::EntryIsFile { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStatFolderChildrenErrorEntryIsFile");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::EntryNotFound { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStatFolderChildrenErrorEntryNotFound");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStatFolderChildrenErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::InvalidCertificate { .. } => {
            let py_tag =
                PyString::new_bound(py, "WorkspaceStatFolderChildrenErrorInvalidCertificate");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::InvalidKeysBundle { .. } => {
            let py_tag =
                PyString::new_bound(py, "WorkspaceStatFolderChildrenErrorInvalidKeysBundle");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::InvalidManifest { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStatFolderChildrenErrorInvalidManifest");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::NoRealmAccess { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStatFolderChildrenErrorNoRealmAccess");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::Offline { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStatFolderChildrenErrorOffline");
            py_obj.set_item("tag", py_tag)?;
        }
        libparsec::WorkspaceStatFolderChildrenError::Stopped { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStatFolderChildrenErrorStopped");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceStopError

#[allow(dead_code, unused_variables)]
fn variant_workspace_stop_error_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceStopError,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    let py_display = PyString::new_bound(py, &rs_obj.to_string());
    py_obj.set_item("error", py_display)?;
    match rs_obj {
        libparsec::WorkspaceStopError::Internal { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStopErrorInternal");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// WorkspaceStorageCacheSize

#[allow(dead_code, unused_variables)]
fn variant_workspace_storage_cache_size_py_to_rs<'a>(
    py: Python<'a>,
    obj: &Bound<'a, PyDict>,
) -> PyResult<libparsec::WorkspaceStorageCacheSize> {
    let tag = obj
        .get_item("tag")?
        .ok_or_else(|| PyKeyError::new_err("tag"))?;
    match tag.downcast::<PyString>()?.to_str()? {
        "WorkspaceStorageCacheSizeCustom" => {
            let size = {
                let py_val_any = obj
                    .get_item("size")?
                    .ok_or_else(|| PyKeyError::new_err("size"))?;
                let py_val_downcasted = py_val_any.downcast::<PyInt>()?;
                {
                    py_val_downcasted.extract::<u32>()?
                }
            };
            Ok(libparsec::WorkspaceStorageCacheSize::Custom { size })
        }
        "WorkspaceStorageCacheSizeDefault" => Ok(libparsec::WorkspaceStorageCacheSize::Default {}),
        _ => Err(PyTypeError::new_err(
            "Object is not a WorkspaceStorageCacheSize",
        )),
    }
}

#[allow(dead_code, unused_variables)]
fn variant_workspace_storage_cache_size_rs_to_py<'a>(
    py: Python<'a>,
    rs_obj: libparsec::WorkspaceStorageCacheSize,
) -> PyResult<Bound<'a, PyDict>> {
    let py_obj = PyDict::new_bound(py);
    match rs_obj {
        libparsec::WorkspaceStorageCacheSize::Custom { size, .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStorageCacheSizeCustom");
            py_obj.set_item("tag", py_tag)?;
            let py_size = (size).to_object(py).into_bound(py);
            py_obj.set_item("size", py_size)?;
        }
        libparsec::WorkspaceStorageCacheSize::Default { .. } => {
            let py_tag = PyString::new_bound(py, "WorkspaceStorageCacheSizeDefault");
            py_obj.set_item("tag", py_tag)?;
        }
    }
    Ok(py_obj)
}

// bootstrap_organization
#[pyfunction]
fn bootstrap_organization<'py>(
    py: Python<'py>,
    config: &Bound<'py, PyDict>,
    on_event_callback: &Bound<'py, PyFunction>,
    bootstrap_organization_addr: &Bound<'py, PyString>,
    save_strategy: &Bound<'py, PyDict>,
    human_handle: &Bound<'py, PyDict>,
    device_label: &Bound<'py, PyString>,
    sequester_authority_verify_key: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    let config = {
        let py_val_dict: &Bound<'_, PyDict> = config;
        struct_client_config_py_to_rs(py, py_val_dict)?
    };
    let on_event_callback = {
        // Wrap the Python callback so that its reference counter is decreased on drop.
        struct Callback {
            py_fn: Option<Py<PyFunction>>,
        }
        impl Drop for Callback {
            fn drop(&mut self) {
                if let Some(py_fn) = self.py_fn.take() {
                    Python::with_gil(|py| {
                        // Return the py object to the py runtime to avoid memory leak
                        py_fn.drop_ref(py);
                    });
                }
            }
        }
        let py_fn: &Bound<'_, PyFunction> = on_event_callback;
        let callback = Callback {
            py_fn: Some(py_fn.to_owned().unbind()),
        };
        std::sync::Arc::new(move |event: libparsec::ClientEvent| {
            if let Some(ref py_fn) = callback.py_fn {
                Python::with_gil(|py| {
                    // TODO: log an error instead of panic ? (it is a bit harsh to crash
                    // the current task if an unrelated event handler has a bug...)
                    let py_event =
                        variant_client_event_rs_to_py(py, event).expect("event conversion failed");
                    py_fn
                        .call1(py, (py_event,))
                        .expect("event callback call failed");
                });
            }
        }) as std::sync::Arc<dyn Fn(libparsec::ClientEvent) + Send + Sync>
    };
    let bootstrap_organization_addr = {
        let py_val_str: &Bound<'_, PyString> = bootstrap_organization_addr;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            libparsec::ParsecOrganizationBootstrapAddr::from_any(&s).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let save_strategy = {
        let py_val_dict: &Bound<'_, PyDict> = save_strategy;
        variant_device_save_strategy_py_to_rs(py, py_val_dict)?
    };
    let human_handle = {
        let py_val_dict: &Bound<'_, PyDict> = human_handle;
        struct_human_handle_py_to_rs(py, py_val_dict)?
    };
    let device_label = {
        let py_val_str: &Bound<'_, PyString> = device_label;
        match py_val_str.to_str()?.parse() {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let sequester_authority_verify_key = if sequester_authority_verify_key.is_none() {
        let py_val_nested = sequester_authority_verify_key.downcast::<PyBytes>()?;
        Some({
            let py_val_bytes: &Bound<'_, PyBytes> = py_val_nested;
            #[allow(clippy::unnecessary_mut_passed)]
            match py_val_bytes.as_bytes().try_into() {
                Ok(val) => val,
                // err can't infer type in some case, because of the previous `try_into`
                Err(err) => return Err(PyTypeError::new_err(format!("{}", err))),
            }
        })
    } else {
        None
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
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

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = struct_available_device_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_bootstrap_organization_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// build_parsec_organization_bootstrap_addr
#[pyfunction]
fn build_parsec_organization_bootstrap_addr<'py>(
    py: Python<'py>,
    addr: &Bound<'py, PyString>,
    organization_id: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let addr = {
        let py_val_str: &Bound<'_, PyString> = addr;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let organization_id = {
        let py_val_str: &Bound<'_, PyString> = organization_id;
        match py_val_str.to_str()?.parse() {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };

    let ret = py.allow_threads(|| {
        libparsec::build_parsec_organization_bootstrap_addr(addr, organization_id)
    });

    let py_ret = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |addr: libparsec::ParsecOrganizationBootstrapAddr| -> Result<String, &'static str> {
                Ok(addr.to_url().into())
            };
        match custom_to_rs_string(ret) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();

    Ok(py_ret)
}

// cancel
#[pyfunction]
fn cancel<'py>(py: Python<'py>, canceller: &Bound<'py, PyInt>) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };

    let ret = py.allow_threads(|| libparsec::cancel(canceller));

    let py_ret = match ret {
        Ok(ok) => {
            let py_obj = PyDict::new_bound(py);
            py_obj.set_item("ok", true)?;
            let py_value = {
                #[allow(clippy::let_unit_value)]
                let _ = ok;
                PyNone::get_bound(py).to_owned().into_any()
            };
            py_obj.set_item("value", py_value)?;
            py_obj.into_any()
        }
        Err(err) => {
            let py_obj = PyDict::new_bound(py);
            py_obj.set_item("ok", false)?;
            let py_err = variant_cancel_error_rs_to_py(py, err)?.into_any();
            py_obj.set_item("error", py_err)?;
            py_obj.into_any()
        }
    };

    Ok(py_ret)
}

// claimer_device_finalize_save_local_device
#[pyfunction]
fn claimer_device_finalize_save_local_device<'py>(
    py: Python<'py>,
    handle: &Bound<'py, PyInt>,
    save_strategy: &Bound<'py, PyDict>,
) -> PyResult<Bound<'py, PyAny>> {
    let handle = { handle.extract::<u32>()? };
    let save_strategy = {
        let py_val_dict: &Bound<'_, PyDict> = save_strategy;
        variant_device_save_strategy_py_to_rs(py, py_val_dict)?
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::claimer_device_finalize_save_local_device(handle, save_strategy).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = struct_available_device_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_claim_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// claimer_device_in_progress_1_do_signify_trust
#[pyfunction]
fn claimer_device_in_progress_1_do_signify_trust<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::claimer_device_in_progress_1_do_signify_trust(canceller, handle).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_device_claim_in_progress2_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_claim_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// claimer_device_in_progress_2_do_wait_peer_trust
#[pyfunction]
fn claimer_device_in_progress_2_do_wait_peer_trust<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret =
            libparsec::claimer_device_in_progress_2_do_wait_peer_trust(canceller, handle).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_device_claim_in_progress3_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_claim_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// claimer_device_in_progress_3_do_claim
#[pyfunction]
fn claimer_device_in_progress_3_do_claim<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
    requested_device_label: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let requested_device_label = {
        let py_val_str: &Bound<'_, PyString> = requested_device_label;
        match py_val_str.to_str()?.parse() {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::claimer_device_in_progress_3_do_claim(
            canceller,
            handle,
            requested_device_label,
        )
        .await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_device_claim_finalize_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_claim_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// claimer_device_initial_do_wait_peer
#[pyfunction]
fn claimer_device_initial_do_wait_peer<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::claimer_device_initial_do_wait_peer(canceller, handle).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_device_claim_in_progress1_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_claim_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// claimer_greeter_abort_operation
#[pyfunction]
fn claimer_greeter_abort_operation<'py>(
    py: Python<'py>,
    handle: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let handle = { handle.extract::<u32>()? };

    let ret = py.allow_threads(|| libparsec::claimer_greeter_abort_operation(handle));

    let py_ret = match ret {
        Ok(ok) => {
            let py_obj = PyDict::new_bound(py);
            py_obj.set_item("ok", true)?;
            let py_value = {
                #[allow(clippy::let_unit_value)]
                let _ = ok;
                PyNone::get_bound(py).to_owned().into_any()
            };
            py_obj.set_item("value", py_value)?;
            py_obj.into_any()
        }
        Err(err) => {
            let py_obj = PyDict::new_bound(py);
            py_obj.set_item("ok", false)?;
            let py_err =
                variant_claimer_greeter_abort_operation_error_rs_to_py(py, err)?.into_any();
            py_obj.set_item("error", py_err)?;
            py_obj.into_any()
        }
    };

    Ok(py_ret)
}

// claimer_retrieve_info
#[pyfunction]
fn claimer_retrieve_info<'py>(
    py: Python<'py>,
    config: &Bound<'py, PyDict>,
    on_event_callback: &Bound<'py, PyFunction>,
    addr: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let config = {
        let py_val_dict: &Bound<'_, PyDict> = config;
        struct_client_config_py_to_rs(py, py_val_dict)?
    };
    let on_event_callback = {
        // Wrap the Python callback so that its reference counter is decreased on drop.
        struct Callback {
            py_fn: Option<Py<PyFunction>>,
        }
        impl Drop for Callback {
            fn drop(&mut self) {
                if let Some(py_fn) = self.py_fn.take() {
                    Python::with_gil(|py| {
                        // Return the py object to the py runtime to avoid memory leak
                        py_fn.drop_ref(py);
                    });
                }
            }
        }
        let py_fn: &Bound<'_, PyFunction> = on_event_callback;
        let callback = Callback {
            py_fn: Some(py_fn.to_owned().unbind()),
        };
        std::sync::Arc::new(move |event: libparsec::ClientEvent| {
            if let Some(ref py_fn) = callback.py_fn {
                Python::with_gil(|py| {
                    // TODO: log an error instead of panic ? (it is a bit harsh to crash
                    // the current task if an unrelated event handler has a bug...)
                    let py_event =
                        variant_client_event_rs_to_py(py, event).expect("event conversion failed");
                    py_fn
                        .call1(py, (py_event,))
                        .expect("event callback call failed");
                });
            }
        }) as std::sync::Arc<dyn Fn(libparsec::ClientEvent) + Send + Sync>
    };
    let addr = {
        let py_val_str: &Bound<'_, PyString> = addr;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            libparsec::ParsecInvitationAddr::from_any(&s).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::claimer_retrieve_info(config, on_event_callback, addr).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            variant_user_or_device_claim_initial_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_claimer_retrieve_info_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// claimer_user_finalize_save_local_device
#[pyfunction]
fn claimer_user_finalize_save_local_device<'py>(
    py: Python<'py>,
    handle: &Bound<'py, PyInt>,
    save_strategy: &Bound<'py, PyDict>,
) -> PyResult<Bound<'py, PyAny>> {
    let handle = { handle.extract::<u32>()? };
    let save_strategy = {
        let py_val_dict: &Bound<'_, PyDict> = save_strategy;
        variant_device_save_strategy_py_to_rs(py, py_val_dict)?
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::claimer_user_finalize_save_local_device(handle, save_strategy).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = struct_available_device_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_claim_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// claimer_user_in_progress_1_do_signify_trust
#[pyfunction]
fn claimer_user_in_progress_1_do_signify_trust<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::claimer_user_in_progress_1_do_signify_trust(canceller, handle).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_user_claim_in_progress2_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_claim_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// claimer_user_in_progress_2_do_wait_peer_trust
#[pyfunction]
fn claimer_user_in_progress_2_do_wait_peer_trust<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::claimer_user_in_progress_2_do_wait_peer_trust(canceller, handle).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_user_claim_in_progress3_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_claim_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// claimer_user_in_progress_3_do_claim
#[pyfunction]
fn claimer_user_in_progress_3_do_claim<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
    requested_device_label: &Bound<'py, PyString>,
    requested_human_handle: &Bound<'py, PyDict>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let requested_device_label = {
        let py_val_str: &Bound<'_, PyString> = requested_device_label;
        match py_val_str.to_str()?.parse() {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let requested_human_handle = {
        let py_val_dict: &Bound<'_, PyDict> = requested_human_handle;
        struct_human_handle_py_to_rs(py, py_val_dict)?
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::claimer_user_in_progress_3_do_claim(
            canceller,
            handle,
            requested_device_label,
            requested_human_handle,
        )
        .await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = struct_user_claim_finalize_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_claim_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// claimer_user_initial_do_wait_peer
#[pyfunction]
fn claimer_user_initial_do_wait_peer<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::claimer_user_initial_do_wait_peer(canceller, handle).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_user_claim_in_progress1_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_claim_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_cancel_invitation
#[pyfunction]
fn client_cancel_invitation<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
    token: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let token = {
        let py_val_str: &Bound<'_, PyString> = token;
        let custom_from_rs_string = |s: String| -> Result<libparsec::InvitationToken, _> {
            libparsec::InvitationToken::from_hex(s.as_str()).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_cancel_invitation(client, token).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_client_cancel_invitation_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_change_authentication
#[pyfunction]
fn client_change_authentication<'py>(
    py: Python<'py>,
    client_config: &Bound<'py, PyDict>,
    current_auth: &Bound<'py, PyDict>,
    new_auth: &Bound<'py, PyDict>,
) -> PyResult<Bound<'py, PyAny>> {
    let client_config = {
        let py_val_dict: &Bound<'_, PyDict> = client_config;
        struct_client_config_py_to_rs(py, py_val_dict)?
    };
    let current_auth = {
        let py_val_dict: &Bound<'_, PyDict> = current_auth;
        variant_device_access_strategy_py_to_rs(py, py_val_dict)?
    };
    let new_auth = {
        let py_val_dict: &Bound<'_, PyDict> = new_auth;
        variant_device_save_strategy_py_to_rs(py, py_val_dict)?
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret =
            libparsec::client_change_authentication(client_config, current_auth, new_auth).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_client_change_authentication_error_rs_to_py(py, err)?
                            .into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_create_workspace
#[pyfunction]
fn client_create_workspace<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
    name: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let name = {
        let py_val_str: &Bound<'_, PyString> = name;
        let custom_from_rs_string = |s: String| -> Result<_, _> {
            s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_create_workspace(client, name).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = PyString::new_bound(py, &{
                            let custom_to_rs_string =
                                |x: libparsec::VlobID| -> Result<String, &'static str> {
                                    Ok(x.hex())
                                };
                            match custom_to_rs_string(ok) {
                                Ok(ok) => ok,
                                Err(err) => return Err(PyTypeError::new_err(err)),
                            }
                        })
                        .into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_client_create_workspace_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_get_user_device
#[pyfunction]
fn client_get_user_device<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
    device: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let device = {
        let py_val_str: &Bound<'_, PyString> = device;
        let custom_from_rs_string = |s: String| -> Result<libparsec::DeviceID, _> {
            libparsec::DeviceID::from_hex(s.as_str()).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_get_user_device(client, device).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            let (x0, x1) = ok;
                            let py_x0 = struct_user_info_rs_to_py(py, x0)?.into_any();
                            let py_x1 = struct_device_info_rs_to_py(py, x1)?.into_any();
                            PyTuple::new_bound(py, [py_x0, py_x1]).into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_client_get_user_device_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_info
#[pyfunction]
fn client_info<'py>(py: Python<'py>, client: &Bound<'py, PyInt>) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_info(client).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = struct_client_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_client_info_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_list_invitations
#[pyfunction]
fn client_list_invitations<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_list_invitations(client).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            let py_list = PyList::empty_bound(py);
                            for rs_elem in ok.into_iter() {
                                let py_elem =
                                    variant_invite_list_item_rs_to_py(py, rs_elem)?.into_any();
                                py_list.append(py_elem)?;
                            }
                            py_list.to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_list_invitations_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_list_user_devices
#[pyfunction]
fn client_list_user_devices<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
    user: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let user = {
        let py_val_str: &Bound<'_, PyString> = user;
        let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
            libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_list_user_devices(client, user).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            let py_list = PyList::empty_bound(py);
                            for rs_elem in ok.into_iter() {
                                let py_elem = struct_device_info_rs_to_py(py, rs_elem)?.into_any();
                                py_list.append(py_elem)?;
                            }
                            py_list.to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_client_list_user_devices_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_list_users
#[pyfunction]
fn client_list_users<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
    skip_revoked: &Bound<'py, PyBool>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let skip_revoked = skip_revoked.extract()?;
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_list_users(client, skip_revoked).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            let py_list = PyList::empty_bound(py);
                            for rs_elem in ok.into_iter() {
                                let py_elem = struct_user_info_rs_to_py(py, rs_elem)?.into_any();
                                py_list.append(py_elem)?;
                            }
                            py_list.to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_client_list_users_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_list_workspace_users
#[pyfunction]
fn client_list_workspace_users<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
    realm_id: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let realm_id = {
        let py_val_str: &Bound<'_, PyString> = realm_id;
        let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
            libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_list_workspace_users(client, realm_id).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            let py_list = PyList::empty_bound(py);
                            for rs_elem in ok.into_iter() {
                                let py_elem =
                                    struct_workspace_user_access_info_rs_to_py(py, rs_elem)?
                                        .into_any();
                                py_list.append(py_elem)?;
                            }
                            py_list.to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_client_list_workspace_users_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_list_workspaces
#[pyfunction]
fn client_list_workspaces<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_list_workspaces(client).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            let py_list = PyList::empty_bound(py);
                            for rs_elem in ok.into_iter() {
                                let py_elem =
                                    struct_workspace_info_rs_to_py(py, rs_elem)?.into_any();
                                py_list.append(py_elem)?;
                            }
                            py_list.to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_client_list_workspaces_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_new_device_invitation
#[pyfunction]
fn client_new_device_invitation<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
    send_email: &Bound<'py, PyBool>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let send_email = send_email.extract()?;
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_new_device_invitation(client, send_email).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = struct_new_invitation_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_client_new_device_invitation_error_rs_to_py(py, err)?
                            .into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_new_user_invitation
#[pyfunction]
fn client_new_user_invitation<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
    claimer_email: &Bound<'py, PyString>,
    send_email: &Bound<'py, PyBool>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let claimer_email = {
        let py_val_str: &Bound<'_, PyString> = claimer_email;
        py_val_str.to_str()?.to_owned()
    };
    let send_email = send_email.extract()?;
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_new_user_invitation(client, claimer_email, send_email).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = struct_new_invitation_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_client_new_user_invitation_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_rename_workspace
#[pyfunction]
fn client_rename_workspace<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
    realm_id: &Bound<'py, PyString>,
    new_name: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let realm_id = {
        let py_val_str: &Bound<'_, PyString> = realm_id;
        let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
            libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let new_name = {
        let py_val_str: &Bound<'_, PyString> = new_name;
        let custom_from_rs_string = |s: String| -> Result<_, _> {
            s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_rename_workspace(client, realm_id, new_name).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_client_rename_workspace_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_revoke_user
#[pyfunction]
fn client_revoke_user<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
    user: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let user = {
        let py_val_str: &Bound<'_, PyString> = user;
        let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
            libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_revoke_user(client, user).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_client_revoke_user_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_share_workspace
#[pyfunction]
fn client_share_workspace<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
    realm_id: &Bound<'py, PyString>,
    recipient: &Bound<'py, PyString>,
    role: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let realm_id = {
        let py_val_str: &Bound<'_, PyString> = realm_id;
        let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
            libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let recipient = {
        let py_val_str: &Bound<'_, PyString> = recipient;
        let custom_from_rs_string = |s: String| -> Result<libparsec::UserID, _> {
            libparsec::UserID::from_hex(s.as_str()).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let role = if role.is_none() {
        let py_val_nested = role.downcast::<PyString>()?;
        Some({
            let raw_value = py_val_nested.extract::<&str>()?;
            enum_realm_role_py_to_rs(py, raw_value)?
        })
    } else {
        None
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_share_workspace(client, realm_id, recipient, role).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_client_share_workspace_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_start
#[pyfunction]
fn client_start<'py>(
    py: Python<'py>,
    config: &Bound<'py, PyDict>,
    on_event_callback: &Bound<'py, PyFunction>,
    access: &Bound<'py, PyDict>,
) -> PyResult<Bound<'py, PyAny>> {
    let config = {
        let py_val_dict: &Bound<'_, PyDict> = config;
        struct_client_config_py_to_rs(py, py_val_dict)?
    };
    let on_event_callback = {
        // Wrap the Python callback so that its reference counter is decreased on drop.
        struct Callback {
            py_fn: Option<Py<PyFunction>>,
        }
        impl Drop for Callback {
            fn drop(&mut self) {
                if let Some(py_fn) = self.py_fn.take() {
                    Python::with_gil(|py| {
                        // Return the py object to the py runtime to avoid memory leak
                        py_fn.drop_ref(py);
                    });
                }
            }
        }
        let py_fn: &Bound<'_, PyFunction> = on_event_callback;
        let callback = Callback {
            py_fn: Some(py_fn.to_owned().unbind()),
        };
        std::sync::Arc::new(move |event: libparsec::ClientEvent| {
            if let Some(ref py_fn) = callback.py_fn {
                Python::with_gil(|py| {
                    // TODO: log an error instead of panic ? (it is a bit harsh to crash
                    // the current task if an unrelated event handler has a bug...)
                    let py_event =
                        variant_client_event_rs_to_py(py, event).expect("event conversion failed");
                    py_fn
                        .call1(py, (py_event,))
                        .expect("event callback call failed");
                });
            }
        }) as std::sync::Arc<dyn Fn(libparsec::ClientEvent) + Send + Sync>
    };
    let access = {
        let py_val_dict: &Bound<'_, PyDict> = access;
        variant_device_access_strategy_py_to_rs(py, py_val_dict)?
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_start(config, on_event_callback, access).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = (ok).to_object(py).into_bound(py);
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_client_start_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_start_device_invitation_greet
#[pyfunction]
fn client_start_device_invitation_greet<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
    token: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let token = {
        let py_val_str: &Bound<'_, PyString> = token;
        let custom_from_rs_string = |s: String| -> Result<libparsec::InvitationToken, _> {
            libparsec::InvitationToken::from_hex(s.as_str()).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_start_device_invitation_greet(client, token).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_device_greet_initial_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_client_start_invitation_greet_error_rs_to_py(py, err)?
                            .into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_start_user_invitation_greet
#[pyfunction]
fn client_start_user_invitation_greet<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
    token: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let token = {
        let py_val_str: &Bound<'_, PyString> = token;
        let custom_from_rs_string = |s: String| -> Result<libparsec::InvitationToken, _> {
            libparsec::InvitationToken::from_hex(s.as_str()).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_start_user_invitation_greet(client, token).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = struct_user_greet_initial_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_client_start_invitation_greet_error_rs_to_py(py, err)?
                            .into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_start_workspace
#[pyfunction]
fn client_start_workspace<'py>(
    py: Python<'py>,
    client: &Bound<'py, PyInt>,
    realm_id: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let realm_id = {
        let py_val_str: &Bound<'_, PyString> = realm_id;
        let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
            libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_start_workspace(client, realm_id).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = (ok).to_object(py).into_bound(py);
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_client_start_workspace_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// client_stop
#[pyfunction]
fn client_stop<'py>(py: Python<'py>, client: &Bound<'py, PyInt>) -> PyResult<Bound<'py, PyAny>> {
    let client = { client.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::client_stop(client).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_client_stop_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// fd_close
#[pyfunction]
fn fd_close<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    fd: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let fd = {
        let v = fd.extract::<u32>()?;
        let custom_from_rs_u32 =
            |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
        match custom_from_rs_u32(v) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::fd_close(workspace, fd).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_workspace_fd_close_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// fd_flush
#[pyfunction]
fn fd_flush<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    fd: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let fd = {
        let v = fd.extract::<u32>()?;
        let custom_from_rs_u32 =
            |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
        match custom_from_rs_u32(v) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::fd_flush(workspace, fd).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_workspace_fd_flush_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// fd_read
#[pyfunction]
fn fd_read<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    fd: &Bound<'py, PyInt>,
    offset: &Bound<'py, PyInt>,
    size: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let fd = {
        let v = fd.extract::<u32>()?;
        let custom_from_rs_u32 =
            |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
        match custom_from_rs_u32(v) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let offset = { offset.extract::<u64>()? };
    let size = { size.extract::<u64>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::fd_read(workspace, fd, offset, size).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = PyBytes::new_bound(py, &ok).into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_workspace_fd_read_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// fd_resize
#[pyfunction]
fn fd_resize<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    fd: &Bound<'py, PyInt>,
    length: &Bound<'py, PyInt>,
    truncate_only: &Bound<'py, PyBool>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let fd = {
        let v = fd.extract::<u32>()?;
        let custom_from_rs_u32 =
            |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
        match custom_from_rs_u32(v) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let length = { length.extract::<u64>()? };
    let truncate_only = truncate_only.extract()?;
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::fd_resize(workspace, fd, length, truncate_only).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_workspace_fd_resize_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// fd_write
#[pyfunction]
fn fd_write<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    fd: &Bound<'py, PyInt>,
    offset: &Bound<'py, PyInt>,
    data: &Bound<'py, PyBytes>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let fd = {
        let v = fd.extract::<u32>()?;
        let custom_from_rs_u32 =
            |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
        match custom_from_rs_u32(v) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let offset = { offset.extract::<u64>()? };
    let data = {
        let py_val_bytes: &Bound<'_, PyBytes> = data;
        py_val_bytes.as_bytes().to_owned()
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::fd_write(workspace, fd, offset, &data).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = (ok).to_object(py).into_bound(py);
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_workspace_fd_write_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// fd_write_constrained_io
#[pyfunction]
fn fd_write_constrained_io<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    fd: &Bound<'py, PyInt>,
    offset: &Bound<'py, PyInt>,
    data: &Bound<'py, PyBytes>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let fd = {
        let v = fd.extract::<u32>()?;
        let custom_from_rs_u32 =
            |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
        match custom_from_rs_u32(v) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let offset = { offset.extract::<u64>()? };
    let data = {
        let py_val_bytes: &Bound<'_, PyBytes> = data;
        py_val_bytes.as_bytes().to_owned()
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::fd_write_constrained_io(workspace, fd, offset, &data).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = (ok).to_object(py).into_bound(py);
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_workspace_fd_write_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// fd_write_start_eof
#[pyfunction]
fn fd_write_start_eof<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    fd: &Bound<'py, PyInt>,
    data: &Bound<'py, PyBytes>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let fd = {
        let v = fd.extract::<u32>()?;
        let custom_from_rs_u32 =
            |raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) };
        match custom_from_rs_u32(v) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let data = {
        let py_val_bytes: &Bound<'_, PyBytes> = data;
        py_val_bytes.as_bytes().to_owned()
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::fd_write_start_eof(workspace, fd, &data).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = (ok).to_object(py).into_bound(py);
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_workspace_fd_write_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// get_default_config_dir
#[pyfunction]
fn get_default_config_dir<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
    let ret = py.allow_threads(|| libparsec::get_default_config_dir());

    let py_ret = PyString::new_bound(py, &{
        let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        };
        match custom_to_rs_string(ret) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();

    Ok(py_ret)
}

// get_default_data_base_dir
#[pyfunction]
fn get_default_data_base_dir<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
    let ret = py.allow_threads(|| libparsec::get_default_data_base_dir());

    let py_ret = PyString::new_bound(py, &{
        let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        };
        match custom_to_rs_string(ret) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();

    Ok(py_ret)
}

// get_default_mountpoint_base_dir
#[pyfunction]
fn get_default_mountpoint_base_dir<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
    let ret = py.allow_threads(|| libparsec::get_default_mountpoint_base_dir());

    let py_ret = PyString::new_bound(py, &{
        let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
            path.into_os_string()
                .into_string()
                .map_err(|_| "Path contains non-utf8 characters")
        };
        match custom_to_rs_string(ret) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();

    Ok(py_ret)
}

// get_platform
#[pyfunction]
fn get_platform<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
    let ret = py.allow_threads(|| libparsec::get_platform());

    let py_ret = PyString::new_bound(py, enum_platform_rs_to_py(ret)).into_any();

    Ok(py_ret)
}

// greeter_device_in_progress_1_do_wait_peer_trust
#[pyfunction]
fn greeter_device_in_progress_1_do_wait_peer_trust<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret =
            libparsec::greeter_device_in_progress_1_do_wait_peer_trust(canceller, handle).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_device_greet_in_progress2_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_greet_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// greeter_device_in_progress_2_do_signify_trust
#[pyfunction]
fn greeter_device_in_progress_2_do_signify_trust<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::greeter_device_in_progress_2_do_signify_trust(canceller, handle).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_device_greet_in_progress3_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_greet_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// greeter_device_in_progress_3_do_get_claim_requests
#[pyfunction]
fn greeter_device_in_progress_3_do_get_claim_requests<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret =
            libparsec::greeter_device_in_progress_3_do_get_claim_requests(canceller, handle).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_device_greet_in_progress4_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_greet_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// greeter_device_in_progress_4_do_create
#[pyfunction]
fn greeter_device_in_progress_4_do_create<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
    device_label: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let device_label = {
        let py_val_str: &Bound<'_, PyString> = device_label;
        match py_val_str.to_str()?.parse() {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret =
            libparsec::greeter_device_in_progress_4_do_create(canceller, handle, device_label)
                .await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_greet_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// greeter_device_initial_do_wait_peer
#[pyfunction]
fn greeter_device_initial_do_wait_peer<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::greeter_device_initial_do_wait_peer(canceller, handle).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_device_greet_in_progress1_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_greet_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// greeter_user_in_progress_1_do_wait_peer_trust
#[pyfunction]
fn greeter_user_in_progress_1_do_wait_peer_trust<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::greeter_user_in_progress_1_do_wait_peer_trust(canceller, handle).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_user_greet_in_progress2_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_greet_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// greeter_user_in_progress_2_do_signify_trust
#[pyfunction]
fn greeter_user_in_progress_2_do_signify_trust<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::greeter_user_in_progress_2_do_signify_trust(canceller, handle).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_user_greet_in_progress3_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_greet_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// greeter_user_in_progress_3_do_get_claim_requests
#[pyfunction]
fn greeter_user_in_progress_3_do_get_claim_requests<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret =
            libparsec::greeter_user_in_progress_3_do_get_claim_requests(canceller, handle).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_user_greet_in_progress4_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_greet_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// greeter_user_in_progress_4_do_create
#[pyfunction]
fn greeter_user_in_progress_4_do_create<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
    human_handle: &Bound<'py, PyDict>,
    device_label: &Bound<'py, PyString>,
    profile: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let human_handle = {
        let py_val_dict: &Bound<'_, PyDict> = human_handle;
        struct_human_handle_py_to_rs(py, py_val_dict)?
    };
    let device_label = {
        let py_val_str: &Bound<'_, PyString> = device_label;
        match py_val_str.to_str()?.parse() {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let profile = {
        let raw_value = profile.extract::<&str>()?;
        enum_user_profile_py_to_rs(py, raw_value)?
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::greeter_user_in_progress_4_do_create(
            canceller,
            handle,
            human_handle,
            device_label,
            profile,
        )
        .await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_greet_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// greeter_user_initial_do_wait_peer
#[pyfunction]
fn greeter_user_initial_do_wait_peer<'py>(
    py: Python<'py>,
    canceller: &Bound<'py, PyInt>,
    handle: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let canceller = { canceller.extract::<u32>()? };
    let handle = { handle.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::greeter_user_initial_do_wait_peer(canceller, handle).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value =
                            struct_user_greet_in_progress1_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_greet_in_progress_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// is_keyring_available
#[pyfunction]
fn is_keyring_available<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
    let ret = py.allow_threads(|| libparsec::is_keyring_available());

    let py_ret = PyBool::new_bound(py, ret).to_owned().into_any();

    Ok(py_ret)
}

// list_available_devices
#[pyfunction]
fn list_available_devices<'py>(
    py: Python<'py>,
    path: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string =
            |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::list_available_devices(&path).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = {
                    let py_list = PyList::empty_bound(py);
                    for rs_elem in ret.into_iter() {
                        let py_elem = struct_available_device_rs_to_py(py, rs_elem)?.into_any();
                        py_list.append(py_elem)?;
                    }
                    py_list.to_owned().into_any()
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// mountpoint_to_os_path
#[pyfunction]
fn mountpoint_to_os_path<'py>(
    py: Python<'py>,
    mountpoint: &Bound<'py, PyInt>,
    parsec_path: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let mountpoint = { mountpoint.extract::<u32>()? };
    let parsec_path = {
        let py_val_str: &Bound<'_, PyString> = parsec_path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::mountpoint_to_os_path(mountpoint, parsec_path).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = PyString::new_bound(py, &{
                            let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                                path.into_os_string()
                                    .into_string()
                                    .map_err(|_| "Path contains non-utf8 characters")
                            };
                            match custom_to_rs_string(ok) {
                                Ok(ok) => ok,
                                Err(err) => return Err(PyTypeError::new_err(err)),
                            }
                        })
                        .into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_mountpoint_to_os_path_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// mountpoint_unmount
#[pyfunction]
fn mountpoint_unmount<'py>(
    py: Python<'py>,
    mountpoint: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let mountpoint = { mountpoint.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::mountpoint_unmount(mountpoint).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_mountpoint_unmount_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// new_canceller
#[pyfunction]
fn new_canceller<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
    let ret = py.allow_threads(|| libparsec::new_canceller());

    let py_ret = (ret).to_object(py).into_bound(py);

    Ok(py_ret)
}

// parse_parsec_addr
#[pyfunction]
fn parse_parsec_addr<'py>(
    py: Python<'py>,
    url: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let url = {
        let py_val_str: &Bound<'_, PyString> = url;
        py_val_str.to_str()?.to_owned()
    };

    let ret = py.allow_threads(|| libparsec::parse_parsec_addr(&url));

    let py_ret = match ret {
        Ok(ok) => {
            let py_obj = PyDict::new_bound(py);
            py_obj.set_item("ok", true)?;
            let py_value = variant_parsed_parsec_addr_rs_to_py(py, ok)?.into_any();
            py_obj.set_item("value", py_value)?;
            py_obj.into_any()
        }
        Err(err) => {
            let py_obj = PyDict::new_bound(py);
            py_obj.set_item("ok", false)?;
            let py_err = variant_parse_parsec_addr_error_rs_to_py(py, err)?.into_any();
            py_obj.set_item("error", py_err)?;
            py_obj.into_any()
        }
    };

    Ok(py_ret)
}

// path_filename
#[pyfunction]
fn path_filename<'py>(py: Python<'py>, path: &Bound<'py, PyString>) -> PyResult<Bound<'py, PyAny>> {
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };

    let ret = py.allow_threads(|| libparsec::path_filename(&path));

    let py_ret = match ret {
        Some(elem) => PyString::new_bound(py, elem.as_ref()).into_any(),
        None => PyNone::get_bound(py).to_owned().into_any(),
    };

    Ok(py_ret)
}

// path_join
#[pyfunction]
fn path_join<'py>(
    py: Python<'py>,
    parent: &Bound<'py, PyString>,
    child: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let parent = {
        let py_val_str: &Bound<'_, PyString> = parent;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let child = {
        let py_val_str: &Bound<'_, PyString> = child;
        let custom_from_rs_string = |s: String| -> Result<_, _> {
            s.parse::<libparsec::EntryName>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };

    let ret = py.allow_threads(|| libparsec::path_join(&parent, &child));

    let py_ret = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |path: libparsec::FsPath| -> Result<_, &'static str> { Ok(path.to_string()) };
        match custom_to_rs_string(ret) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();

    Ok(py_ret)
}

// path_normalize
#[pyfunction]
fn path_normalize<'py>(
    py: Python<'py>,
    path: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };

    let ret = py.allow_threads(|| libparsec::path_normalize(path));

    let py_ret = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |path: libparsec::FsPath| -> Result<_, &'static str> { Ok(path.to_string()) };
        match custom_to_rs_string(ret) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();

    Ok(py_ret)
}

// path_parent
#[pyfunction]
fn path_parent<'py>(py: Python<'py>, path: &Bound<'py, PyString>) -> PyResult<Bound<'py, PyAny>> {
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };

    let ret = py.allow_threads(|| libparsec::path_parent(&path));

    let py_ret = PyString::new_bound(py, &{
        let custom_to_rs_string =
            |path: libparsec::FsPath| -> Result<_, &'static str> { Ok(path.to_string()) };
        match custom_to_rs_string(ret) {
            Ok(ok) => ok,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    })
    .into_any();

    Ok(py_ret)
}

// path_split
#[pyfunction]
fn path_split<'py>(py: Python<'py>, path: &Bound<'py, PyString>) -> PyResult<Bound<'py, PyAny>> {
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };

    let ret = py.allow_threads(|| libparsec::path_split(&path));

    let py_ret = {
        let py_list = PyList::empty_bound(py);
        for rs_elem in ret.into_iter() {
            let py_elem = PyString::new_bound(py, rs_elem.as_ref()).into_any();
            py_list.append(py_elem)?;
        }
        py_list.to_owned().into_any()
    };

    Ok(py_ret)
}

// test_drop_testbed
#[pyfunction]
fn test_drop_testbed<'py>(
    py: Python<'py>,
    path: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string =
            |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::test_drop_testbed(&path).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_testbed_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// test_get_testbed_bootstrap_organization_addr
#[pyfunction]
fn test_get_testbed_bootstrap_organization_addr<'py>(
    py: Python<'py>,
    discriminant_dir: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let discriminant_dir = {
        let py_val_str: &Bound<'_, PyString> = discriminant_dir;
        let custom_from_rs_string =
            |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };

    let ret = py.allow_threads(|| {
        libparsec::test_get_testbed_bootstrap_organization_addr(&discriminant_dir)
    });

    let py_ret = match ret {
        Ok(ok) => {
            let py_obj = PyDict::new_bound(py);
            py_obj.set_item("ok", true)?;
            let py_value = match ok {
    Some(elem) => {
        PyString::new_bound(py,&{
    let custom_to_rs_string = |addr: libparsec::ParsecOrganizationBootstrapAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) };
    match custom_to_rs_string(elem) {
        Ok(ok) => ok,
        Err(err) => return Err(PyTypeError::new_err(err)),
    }
}).into_any()
    }
    None => PyNone::get_bound(py).to_owned().into_any(),
};
            py_obj.set_item("value", py_value)?;
            py_obj.into_any()
        }
        Err(err) => {
            let py_obj = PyDict::new_bound(py);
            py_obj.set_item("ok", false)?;
            let py_err = variant_testbed_error_rs_to_py(py, err)?.into_any();
            py_obj.set_item("error", py_err)?;
            py_obj.into_any()
        }
    };

    Ok(py_ret)
}

// test_get_testbed_organization_id
#[pyfunction]
fn test_get_testbed_organization_id<'py>(
    py: Python<'py>,
    discriminant_dir: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let discriminant_dir = {
        let py_val_str: &Bound<'_, PyString> = discriminant_dir;
        let custom_from_rs_string =
            |s: String| -> Result<_, &'static str> { Ok(std::path::PathBuf::from(s)) };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };

    let ret = py.allow_threads(|| libparsec::test_get_testbed_organization_id(&discriminant_dir));

    let py_ret = match ret {
        Ok(ok) => {
            let py_obj = PyDict::new_bound(py);
            py_obj.set_item("ok", true)?;
            let py_value = match ok {
                Some(elem) => PyString::new_bound(py, elem.as_ref()).into_any(),
                None => PyNone::get_bound(py).to_owned().into_any(),
            };
            py_obj.set_item("value", py_value)?;
            py_obj.into_any()
        }
        Err(err) => {
            let py_obj = PyDict::new_bound(py);
            py_obj.set_item("ok", false)?;
            let py_err = variant_testbed_error_rs_to_py(py, err)?.into_any();
            py_obj.set_item("error", py_err)?;
            py_obj.into_any()
        }
    };

    Ok(py_ret)
}

// test_new_testbed
#[pyfunction]
fn test_new_testbed<'py>(
    py: Python<'py>,
    template: &Bound<'py, PyString>,
    test_server: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    let template = {
        let py_val_str: &Bound<'_, PyString> = template;
        py_val_str.to_str()?.to_owned()
    };
    let test_server = if test_server.is_none() {
        let py_val_nested = test_server.downcast::<PyString>()?;
        Some({
            let py_val_str: &Bound<'_, PyString> = py_val_nested;
            let custom_from_rs_string = |s: String| -> Result<_, String> {
                libparsec::ParsecAddr::from_any(&s).map_err(|e| e.to_string())
            };
            match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
                Ok(val) => val,
                Err(err) => return Err(PyTypeError::new_err(err)),
            }
        })
    } else {
        None
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::test_new_testbed(&template, test_server.as_ref()).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = PyString::new_bound(py, &{
                            let custom_to_rs_string = |path: std::path::PathBuf| -> Result<_, _> {
                                path.into_os_string()
                                    .into_string()
                                    .map_err(|_| "Path contains non-utf8 characters")
                            };
                            match custom_to_rs_string(ok) {
                                Ok(ok) => ok,
                                Err(err) => return Err(PyTypeError::new_err(err)),
                            }
                        })
                        .into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_testbed_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// validate_device_label
#[pyfunction]
fn validate_device_label<'py>(
    py: Python<'py>,
    raw: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let raw = {
        let py_val_str: &Bound<'_, PyString> = raw;
        py_val_str.to_str()?.to_owned()
    };

    let ret = py.allow_threads(|| libparsec::validate_device_label(&raw));

    let py_ret = PyBool::new_bound(py, ret).to_owned().into_any();

    Ok(py_ret)
}

// validate_email
#[pyfunction]
fn validate_email<'py>(py: Python<'py>, raw: &Bound<'py, PyString>) -> PyResult<Bound<'py, PyAny>> {
    let raw = {
        let py_val_str: &Bound<'_, PyString> = raw;
        py_val_str.to_str()?.to_owned()
    };

    let ret = py.allow_threads(|| libparsec::validate_email(&raw));

    let py_ret = PyBool::new_bound(py, ret).to_owned().into_any();

    Ok(py_ret)
}

// validate_entry_name
#[pyfunction]
fn validate_entry_name<'py>(
    py: Python<'py>,
    raw: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let raw = {
        let py_val_str: &Bound<'_, PyString> = raw;
        py_val_str.to_str()?.to_owned()
    };

    let ret = py.allow_threads(|| libparsec::validate_entry_name(&raw));

    let py_ret = PyBool::new_bound(py, ret).to_owned().into_any();

    Ok(py_ret)
}

// validate_human_handle_label
#[pyfunction]
fn validate_human_handle_label<'py>(
    py: Python<'py>,
    raw: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let raw = {
        let py_val_str: &Bound<'_, PyString> = raw;
        py_val_str.to_str()?.to_owned()
    };

    let ret = py.allow_threads(|| libparsec::validate_human_handle_label(&raw));

    let py_ret = PyBool::new_bound(py, ret).to_owned().into_any();

    Ok(py_ret)
}

// validate_invitation_token
#[pyfunction]
fn validate_invitation_token<'py>(
    py: Python<'py>,
    raw: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let raw = {
        let py_val_str: &Bound<'_, PyString> = raw;
        py_val_str.to_str()?.to_owned()
    };

    let ret = py.allow_threads(|| libparsec::validate_invitation_token(&raw));

    let py_ret = PyBool::new_bound(py, ret).to_owned().into_any();

    Ok(py_ret)
}

// validate_organization_id
#[pyfunction]
fn validate_organization_id<'py>(
    py: Python<'py>,
    raw: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let raw = {
        let py_val_str: &Bound<'_, PyString> = raw;
        py_val_str.to_str()?.to_owned()
    };

    let ret = py.allow_threads(|| libparsec::validate_organization_id(&raw));

    let py_ret = PyBool::new_bound(py, ret).to_owned().into_any();

    Ok(py_ret)
}

// validate_path
#[pyfunction]
fn validate_path<'py>(py: Python<'py>, raw: &Bound<'py, PyString>) -> PyResult<Bound<'py, PyAny>> {
    let raw = {
        let py_val_str: &Bound<'_, PyString> = raw;
        py_val_str.to_str()?.to_owned()
    };

    let ret = py.allow_threads(|| libparsec::validate_path(&raw));

    let py_ret = PyBool::new_bound(py, ret).to_owned().into_any();

    Ok(py_ret)
}

// workspace_create_file
#[pyfunction]
fn workspace_create_file<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    path: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_create_file(workspace, path).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = PyString::new_bound(py, &{
                            let custom_to_rs_string =
                                |x: libparsec::VlobID| -> Result<String, &'static str> {
                                    Ok(x.hex())
                                };
                            match custom_to_rs_string(ok) {
                                Ok(ok) => ok,
                                Err(err) => return Err(PyTypeError::new_err(err)),
                            }
                        })
                        .into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_workspace_create_file_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_create_folder
#[pyfunction]
fn workspace_create_folder<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    path: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_create_folder(workspace, path).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = PyString::new_bound(py, &{
                            let custom_to_rs_string =
                                |x: libparsec::VlobID| -> Result<String, &'static str> {
                                    Ok(x.hex())
                                };
                            match custom_to_rs_string(ok) {
                                Ok(ok) => ok,
                                Err(err) => return Err(PyTypeError::new_err(err)),
                            }
                        })
                        .into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_workspace_create_folder_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_create_folder_all
#[pyfunction]
fn workspace_create_folder_all<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    path: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_create_folder_all(workspace, path).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = PyString::new_bound(py, &{
                            let custom_to_rs_string =
                                |x: libparsec::VlobID| -> Result<String, &'static str> {
                                    Ok(x.hex())
                                };
                            match custom_to_rs_string(ok) {
                                Ok(ok) => ok,
                                Err(err) => return Err(PyTypeError::new_err(err)),
                            }
                        })
                        .into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_workspace_create_folder_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_decrypt_path_addr
#[pyfunction]
fn workspace_decrypt_path_addr<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    link: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let link = {
        let py_val_str: &Bound<'_, PyString> = link;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            libparsec::ParsecWorkspacePathAddr::from_any(&s).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_decrypt_path_addr(workspace, &link).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = PyString::new_bound(py, &{
                            let custom_to_rs_string =
                                |path: libparsec::FsPath| -> Result<_, &'static str> {
                                    Ok(path.to_string())
                                };
                            match custom_to_rs_string(ok) {
                                Ok(ok) => ok,
                                Err(err) => return Err(PyTypeError::new_err(err)),
                            }
                        })
                        .into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_workspace_decrypt_path_addr_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_generate_path_addr
#[pyfunction]
fn workspace_generate_path_addr<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    path: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {

        let ret = libparsec::workspace_generate_path_addr(
            workspace,
            &path,
        ).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
    Ok(ok) => {
        let py_obj = PyDict::new_bound(py);
        py_obj.set_item("ok", true)?;
        let py_value = PyString::new_bound(py,&{
    let custom_to_rs_string = |addr: libparsec::ParsecWorkspacePathAddr| -> Result<String, &'static str> { Ok(addr.to_url().into()) };
    match custom_to_rs_string(ok) {
        Ok(ok) => ok,
        Err(err) => return Err(PyTypeError::new_err(err)),
    }
}).into_any();
        py_obj.set_item("value", py_value)?;
        py_obj.into_any()
    }
    Err(err) => {
        let py_obj = PyDict::new_bound(py);
        py_obj.set_item("ok", false)?;
        let py_err = variant_workspace_generate_path_addr_error_rs_to_py(py, err)?.into_any();
        py_obj.set_item("error", py_err)?;
        py_obj.into_any()
    }
};
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(intern!(py, "call_soon_threadsafe"), (future_set_result, py_ret), None)
            })().expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_info
#[pyfunction]
fn workspace_info<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_info(workspace).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = struct_started_workspace_info_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_workspace_info_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_mount
#[pyfunction]
fn workspace_mount<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_mount(workspace).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            let (x0, x1) = ok;
                            let py_x0 = (x0).to_object(py).into_bound(py);
                            let py_x1 = PyString::new_bound(py, &{
                                let custom_to_rs_string =
                                    |path: std::path::PathBuf| -> Result<_, _> {
                                        path.into_os_string()
                                            .into_string()
                                            .map_err(|_| "Path contains non-utf8 characters")
                                    };
                                match custom_to_rs_string(x1) {
                                    Ok(ok) => ok,
                                    Err(err) => return Err(PyTypeError::new_err(err)),
                                }
                            })
                            .into_any();
                            PyTuple::new_bound(py, [py_x0, py_x1]).into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_workspace_mount_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_move_entry
#[pyfunction]
fn workspace_move_entry<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    src: &Bound<'py, PyString>,
    dst: &Bound<'py, PyString>,
    mode: &Bound<'py, PyDict>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let src = {
        let py_val_str: &Bound<'_, PyString> = src;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let dst = {
        let py_val_str: &Bound<'_, PyString> = dst;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let mode = {
        let py_val_dict: &Bound<'_, PyDict> = mode;
        variant_move_entry_mode_py_to_rs(py, py_val_dict)?
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_move_entry(workspace, src, dst, mode).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_workspace_move_entry_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_open_file
#[pyfunction]
fn workspace_open_file<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    path: &Bound<'py, PyString>,
    mode: &Bound<'py, PyDict>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let mode = {
        let py_val_dict: &Bound<'_, PyDict> = mode;
        struct_open_options_py_to_rs(py, py_val_dict)?
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_open_file(workspace, path, mode).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = ({
                            let custom_to_rs_u32 =
                                |fd: libparsec::FileDescriptor| -> Result<_, &'static str> {
                                    Ok(fd.0)
                                };
                            match custom_to_rs_u32(ok) {
                                Ok(ok) => ok,
                                Err(err) => return Err(PyTypeError::new_err(err)),
                            }
                        })
                        .to_object(py)
                        .into_bound(py);
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_workspace_open_file_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_remove_entry
#[pyfunction]
fn workspace_remove_entry<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    path: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_remove_entry(workspace, path).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_workspace_remove_entry_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_remove_file
#[pyfunction]
fn workspace_remove_file<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    path: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_remove_file(workspace, path).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_workspace_remove_entry_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_remove_folder
#[pyfunction]
fn workspace_remove_folder<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    path: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_remove_folder(workspace, path).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_workspace_remove_entry_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_remove_folder_all
#[pyfunction]
fn workspace_remove_folder_all<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    path: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_remove_folder_all(workspace, path).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_workspace_remove_entry_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_stat_entry
#[pyfunction]
fn workspace_stat_entry<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    path: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_stat_entry(workspace, &path).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = variant_entry_stat_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_workspace_stat_entry_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_stat_entry_by_id
#[pyfunction]
fn workspace_stat_entry_by_id<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    entry_id: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let entry_id = {
        let py_val_str: &Bound<'_, PyString> = entry_id;
        let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
            libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_stat_entry_by_id(workspace, entry_id).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = variant_entry_stat_rs_to_py(py, ok)?.into_any();
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_workspace_stat_entry_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_stat_folder_children
#[pyfunction]
fn workspace_stat_folder_children<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    path: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let path = {
        let py_val_str: &Bound<'_, PyString> = path;
        let custom_from_rs_string = |s: String| -> Result<_, String> {
            s.parse::<libparsec::FsPath>().map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_stat_folder_children(workspace, &path).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            let py_list = PyList::empty_bound(py);
                            for rs_elem in ok.into_iter() {
                                let py_elem = {
                                    let (x0, x1) = rs_elem;
                                    let py_x0 = PyString::new_bound(py, x0.as_ref()).into_any();
                                    let py_x1 = variant_entry_stat_rs_to_py(py, x1)?.into_any();
                                    PyTuple::new_bound(py, [py_x0, py_x1]).into_any()
                                };
                                py_list.append(py_elem)?;
                            }
                            py_list.to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_workspace_stat_folder_children_error_rs_to_py(py, err)?
                                .into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_stat_folder_children_by_id
#[pyfunction]
fn workspace_stat_folder_children_by_id<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
    entry_id: &Bound<'py, PyString>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let entry_id = {
        let py_val_str: &Bound<'_, PyString> = entry_id;
        let custom_from_rs_string = |s: String| -> Result<libparsec::VlobID, _> {
            libparsec::VlobID::from_hex(s.as_str()).map_err(|e| e.to_string())
        };
        match custom_from_rs_string(py_val_str.to_str()?.to_owned()) {
            Ok(val) => val,
            Err(err) => return Err(PyTypeError::new_err(err)),
        }
    };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_stat_folder_children_by_id(workspace, entry_id).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            let py_list = PyList::empty_bound(py);
                            for rs_elem in ok.into_iter() {
                                let py_elem = {
                                    let (x0, x1) = rs_elem;
                                    let py_x0 = PyString::new_bound(py, x0.as_ref()).into_any();
                                    let py_x1 = variant_entry_stat_rs_to_py(py, x1)?.into_any();
                                    PyTuple::new_bound(py, [py_x0, py_x1]).into_any()
                                };
                                py_list.append(py_elem)?;
                            }
                            py_list.to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err =
                            variant_workspace_stat_folder_children_error_rs_to_py(py, err)?
                                .into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

// workspace_stop
#[pyfunction]
fn workspace_stop<'py>(
    py: Python<'py>,
    workspace: &Bound<'py, PyInt>,
) -> PyResult<Bound<'py, PyAny>> {
    let workspace = { workspace.extract::<u32>()? };
    let future = crate::asyncio(py).call_method0(intern!(py, "Future"))?;
    let future_in_tokio = future.to_object(py);

    // TODO: Support cancellation ?
    let _handle = crate::tokio_runtime().spawn(async move {
        let ret = libparsec::workspace_stop(workspace).await;

        Python::with_gil(|py| {
            (move || -> PyResult<Bound<'_, PyAny>> {
                let py_ret = match ret {
                    Ok(ok) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", true)?;
                        let py_value = {
                            #[allow(clippy::let_unit_value)]
                            let _ = ok;
                            PyNone::get_bound(py).to_owned().into_any()
                        };
                        py_obj.set_item("value", py_value)?;
                        py_obj.into_any()
                    }
                    Err(err) => {
                        let py_obj = PyDict::new_bound(py);
                        py_obj.set_item("ok", false)?;
                        let py_err = variant_workspace_stop_error_rs_to_py(py, err)?.into_any();
                        py_obj.set_item("error", py_err)?;
                        py_obj.into_any()
                    }
                };
                let future = future_in_tokio.bind(py);
                let future_set_result = future.getattr(intern!(py, "set_result"))?;
                let asyncio_loop = future.call_method0(intern!(py, "get_loop"))?;
                asyncio_loop.call_method(
                    intern!(py, "call_soon_threadsafe"),
                    (future_set_result, py_ret),
                    None,
                )
            })()
            .expect("future callback call has failed");
        });
    });

    Ok(future)
}

pub fn register_meths(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bootstrap_organization, m)?)?;
    m.add_function(wrap_pyfunction!(
        build_parsec_organization_bootstrap_addr,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(cancel, m)?)?;
    m.add_function(wrap_pyfunction!(
        claimer_device_finalize_save_local_device,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        claimer_device_in_progress_1_do_signify_trust,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        claimer_device_in_progress_2_do_wait_peer_trust,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(claimer_device_in_progress_3_do_claim, m)?)?;
    m.add_function(wrap_pyfunction!(claimer_device_initial_do_wait_peer, m)?)?;
    m.add_function(wrap_pyfunction!(claimer_greeter_abort_operation, m)?)?;
    m.add_function(wrap_pyfunction!(claimer_retrieve_info, m)?)?;
    m.add_function(wrap_pyfunction!(
        claimer_user_finalize_save_local_device,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        claimer_user_in_progress_1_do_signify_trust,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        claimer_user_in_progress_2_do_wait_peer_trust,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(claimer_user_in_progress_3_do_claim, m)?)?;
    m.add_function(wrap_pyfunction!(claimer_user_initial_do_wait_peer, m)?)?;
    m.add_function(wrap_pyfunction!(client_cancel_invitation, m)?)?;
    m.add_function(wrap_pyfunction!(client_change_authentication, m)?)?;
    m.add_function(wrap_pyfunction!(client_create_workspace, m)?)?;
    m.add_function(wrap_pyfunction!(client_get_user_device, m)?)?;
    m.add_function(wrap_pyfunction!(client_info, m)?)?;
    m.add_function(wrap_pyfunction!(client_list_invitations, m)?)?;
    m.add_function(wrap_pyfunction!(client_list_user_devices, m)?)?;
    m.add_function(wrap_pyfunction!(client_list_users, m)?)?;
    m.add_function(wrap_pyfunction!(client_list_workspace_users, m)?)?;
    m.add_function(wrap_pyfunction!(client_list_workspaces, m)?)?;
    m.add_function(wrap_pyfunction!(client_new_device_invitation, m)?)?;
    m.add_function(wrap_pyfunction!(client_new_user_invitation, m)?)?;
    m.add_function(wrap_pyfunction!(client_rename_workspace, m)?)?;
    m.add_function(wrap_pyfunction!(client_revoke_user, m)?)?;
    m.add_function(wrap_pyfunction!(client_share_workspace, m)?)?;
    m.add_function(wrap_pyfunction!(client_start, m)?)?;
    m.add_function(wrap_pyfunction!(client_start_device_invitation_greet, m)?)?;
    m.add_function(wrap_pyfunction!(client_start_user_invitation_greet, m)?)?;
    m.add_function(wrap_pyfunction!(client_start_workspace, m)?)?;
    m.add_function(wrap_pyfunction!(client_stop, m)?)?;
    m.add_function(wrap_pyfunction!(fd_close, m)?)?;
    m.add_function(wrap_pyfunction!(fd_flush, m)?)?;
    m.add_function(wrap_pyfunction!(fd_read, m)?)?;
    m.add_function(wrap_pyfunction!(fd_resize, m)?)?;
    m.add_function(wrap_pyfunction!(fd_write, m)?)?;
    m.add_function(wrap_pyfunction!(fd_write_constrained_io, m)?)?;
    m.add_function(wrap_pyfunction!(fd_write_start_eof, m)?)?;
    m.add_function(wrap_pyfunction!(get_default_config_dir, m)?)?;
    m.add_function(wrap_pyfunction!(get_default_data_base_dir, m)?)?;
    m.add_function(wrap_pyfunction!(get_default_mountpoint_base_dir, m)?)?;
    m.add_function(wrap_pyfunction!(get_platform, m)?)?;
    m.add_function(wrap_pyfunction!(
        greeter_device_in_progress_1_do_wait_peer_trust,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        greeter_device_in_progress_2_do_signify_trust,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        greeter_device_in_progress_3_do_get_claim_requests,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(greeter_device_in_progress_4_do_create, m)?)?;
    m.add_function(wrap_pyfunction!(greeter_device_initial_do_wait_peer, m)?)?;
    m.add_function(wrap_pyfunction!(
        greeter_user_in_progress_1_do_wait_peer_trust,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        greeter_user_in_progress_2_do_signify_trust,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        greeter_user_in_progress_3_do_get_claim_requests,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(greeter_user_in_progress_4_do_create, m)?)?;
    m.add_function(wrap_pyfunction!(greeter_user_initial_do_wait_peer, m)?)?;
    m.add_function(wrap_pyfunction!(is_keyring_available, m)?)?;
    m.add_function(wrap_pyfunction!(list_available_devices, m)?)?;
    m.add_function(wrap_pyfunction!(mountpoint_to_os_path, m)?)?;
    m.add_function(wrap_pyfunction!(mountpoint_unmount, m)?)?;
    m.add_function(wrap_pyfunction!(new_canceller, m)?)?;
    m.add_function(wrap_pyfunction!(parse_parsec_addr, m)?)?;
    m.add_function(wrap_pyfunction!(path_filename, m)?)?;
    m.add_function(wrap_pyfunction!(path_join, m)?)?;
    m.add_function(wrap_pyfunction!(path_normalize, m)?)?;
    m.add_function(wrap_pyfunction!(path_parent, m)?)?;
    m.add_function(wrap_pyfunction!(path_split, m)?)?;
    m.add_function(wrap_pyfunction!(test_drop_testbed, m)?)?;
    m.add_function(wrap_pyfunction!(
        test_get_testbed_bootstrap_organization_addr,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(test_get_testbed_organization_id, m)?)?;
    m.add_function(wrap_pyfunction!(test_new_testbed, m)?)?;
    m.add_function(wrap_pyfunction!(validate_device_label, m)?)?;
    m.add_function(wrap_pyfunction!(validate_email, m)?)?;
    m.add_function(wrap_pyfunction!(validate_entry_name, m)?)?;
    m.add_function(wrap_pyfunction!(validate_human_handle_label, m)?)?;
    m.add_function(wrap_pyfunction!(validate_invitation_token, m)?)?;
    m.add_function(wrap_pyfunction!(validate_organization_id, m)?)?;
    m.add_function(wrap_pyfunction!(validate_path, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_create_file, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_create_folder, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_create_folder_all, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_decrypt_path_addr, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_generate_path_addr, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_info, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_mount, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_move_entry, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_open_file, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_remove_entry, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_remove_file, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_remove_folder, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_remove_folder_all, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_stat_entry, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_stat_entry_by_id, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_stat_folder_children, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_stat_folder_children_by_id, m)?)?;
    m.add_function(wrap_pyfunction!(workspace_stop, m)?)?;
    Ok(())
}

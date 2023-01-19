// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use once_cell::sync::OnceCell;

use libparsec_client_types::LocalDevice;
use libparsec_platform_async::Mutex;
use libparsec_platform_device_loader::load_device_with_password;
use libparsec_types::DeviceID;

use crate::{LoggedCoreError, LoggedCoreResult};

static CORE: OnceCell<Mutex<Vec<Option<LoggedCore>>>> = OnceCell::new();

/// i32 is because javascript number is a 64 bits float
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoggedCoreHandle(i32);

impl From<i32> for LoggedCoreHandle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<LoggedCoreHandle> for i32 {
    fn from(value: LoggedCoreHandle) -> Self {
        value.0
    }
}

impl From<LoggedCoreHandle> for usize {
    fn from(value: LoggedCoreHandle) -> Self {
        value.0 as usize
    }
}

#[derive(Debug, Clone)]
struct LoggedCore {
    device: LocalDevice,
}

pub async fn login(key: &str, password: &str) -> LoggedCoreResult<LoggedCoreHandle> {
    let mut core = CORE.get_or_init(|| Mutex::new(Vec::new())).lock().await;

    let index = core.len() as i32;

    let device =
        load_device_with_password(key, password).map_err(|e| LoggedCoreError::LoginFailed {
            help: e.to_string(),
        })?;

    // Store in memory
    core.push(Some(LoggedCore { device }));

    Ok(LoggedCoreHandle(index))
}

pub async fn logged_core_get_device_id(handle: LoggedCoreHandle) -> LoggedCoreResult<DeviceID> {
    if let Some(logged_core) = CORE
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .await
        .get(usize::from(handle))
    {
        return if let Some(logged_core) = logged_core {
            Ok(logged_core.device.device_id.clone())
        } else {
            Err(LoggedCoreError::Disconnected)
        };
    }

    Err(LoggedCoreError::InvalidHandle { handle })
}

pub async fn logged_core_get_device_display(handle: LoggedCoreHandle) -> LoggedCoreResult<String> {
    if let Some(logged_core) = CORE
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .await
        .get(usize::from(handle))
    {
        return if let Some(logged_core) = logged_core {
            Ok(logged_core.device.device_display().into())
        } else {
            Err(LoggedCoreError::Disconnected)
        };
    }

    Err(LoggedCoreError::InvalidHandle { handle })
}

pub async fn logout(handle: LoggedCoreHandle) -> LoggedCoreResult<()> {
    if let Some(logged_core) = CORE
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .await
        .get_mut(usize::from(handle))
    {
        *logged_core = None;
        return Ok(());
    }

    Err(LoggedCoreError::InvalidHandle { handle })
}

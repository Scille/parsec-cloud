// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use widestring::U16CStr;
use windows_sys::Win32::{
    Foundation::GetLastError,
    Networking::WinHttp::{
        WinHttpGetIEProxyConfigForCurrentUser, WINHTTP_CURRENT_USER_IE_PROXY_CONFIG,
    },
    System::Memory::GlobalFree,
};

use libparsec_types::anyhow;

use crate::ProxyConfig;

impl ProxyConfig {
    pub(crate) fn with_register(self) -> anyhow::Result<Self> {
        let mut cfg = self;

        let mut proxy_config = WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
            fAutoDetect: 0,
            lpszAutoConfigUrl: std::ptr::null_mut(),
            lpszProxy: std::ptr::null_mut(),
            lpszProxyBypass: std::ptr::null_mut(),
        };

        // Safety: The only thing we should ensure is that the pointer provided
        // is a WINHTTP_CURRENT_USER_IE_PROXY_CONFIG for the correct size but I
        // don't cast anything (no other unsafe block) and rust provides the
        // check on type
        unsafe {
            if WinHttpGetIEProxyConfigForCurrentUser(&mut proxy_config) == 0 {
                return Err(anyhow::anyhow!(
                    "WinHttpGetProxyForUrl failed with code: {}",
                    GetLastError()
                ));
            }
        }

        // Proxy Auto Config enabled
        if !proxy_config.lpszAutoConfigUrl.is_null() {
            // Safety: We free a pointer that was allocated by windows and we don't use it after.
            // According to Windows doc, we should free it.
            // https://learn.microsoft.com/en-us/windows/win32/api/winhttp/nf-winhttp-winhttpgetieproxyconfigforcurrentuser#remarks
            unsafe {
                GlobalFree(proxy_config.lpszAutoConfigUrl as isize);
            }
        }

        // Proxy enabled
        if !proxy_config.lpszProxy.is_null() {
            // Safety: According to Windows doc, this parameter is a nul terminated widestring
            let proxy = unsafe { U16CStr::from_ptr_str(proxy_config.lpszProxy) };
            let proxy = String::from_utf16(proxy.as_slice())?;

            cfg = if proxy.starts_with("https") {
                cfg.with_https_proxy(proxy)
            } else if proxy.starts_with("http") {
                cfg.with_http_proxy(proxy)
            } else {
                return Err(anyhow::anyhow!(
                    "Proxy server doesn't start with http or https"
                ));
            }?;

            // Safety: We free a pointer that was allocated by windows and we don't use it after.
            // According to Windows doc, we should free it.
            // https://learn.microsoft.com/en-us/windows/win32/api/winhttp/nf-winhttp-winhttpgetieproxyconfigforcurrentuser#remarks
            unsafe {
                GlobalFree(proxy_config.lpszProxy as isize);
            }
        }

        // Proxy Bypass enabled
        // TODO: We don't handle ProxyBypass for the moment
        if !proxy_config.lpszProxyBypass.is_null() {
            // Safety: We free a pointer that was allocated by windows and we don't use it after.
            // According to Windows doc, we should free it.
            // https://learn.microsoft.com/en-us/windows/win32/api/winhttp/nf-winhttp-winhttpgetieproxyconfigforcurrentuser#remarks
            unsafe {
                GlobalFree(proxy_config.lpszProxyBypass as isize);
            }
        }

        Ok(cfg)
    }
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::ffi::c_void;

use widestring::{U16CStr, U16CString};
use windows_sys::Win32::{
    Foundation::{GetLastError, GlobalFree},
    Networking::WinHttp::{
        WinHttpCloseHandle, WinHttpGetIEProxyConfigForCurrentUser, WinHttpGetProxyForUrl,
        WinHttpOpen, WINHTTP_ACCESS_TYPE_DEFAULT_PROXY, WINHTTP_AUTOPROXY_CONFIG_URL,
        WINHTTP_AUTOPROXY_OPTIONS, WINHTTP_AUTO_DETECT_TYPE_DHCP, WINHTTP_AUTO_DETECT_TYPE_DNS_A,
        WINHTTP_CURRENT_USER_IE_PROXY_CONFIG, WINHTTP_PROXY_INFO,
    },
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
            // Safety: According to Windows doc, this parameter is a nul terminated widestring
            let pac_url = unsafe { U16CString::from_ptr_str(proxy_config.lpszAutoConfigUrl) };
            cfg.pac_url = Some(pac_url);

            // Safety: We free a pointer that was allocated by windows and we don't use it after.
            // According to Windows doc, we should free it.
            // https://learn.microsoft.com/en-us/windows/win32/api/winhttp/nf-winhttp-winhttpgetieproxyconfigforcurrentuser#remarks
            unsafe {
                GlobalFree(proxy_config.lpszAutoConfigUrl as *mut c_void);
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
                GlobalFree(proxy_config.lpszProxy as *mut c_void);
            }
        }

        // Proxy Bypass enabled
        // TODO: We don't handle ProxyBypass for the moment
        if !proxy_config.lpszProxyBypass.is_null() {
            // Safety: We free a pointer that was allocated by windows and we don't use it after.
            // According to Windows doc, we should free it.
            // https://learn.microsoft.com/en-us/windows/win32/api/winhttp/nf-winhttp-winhttpgetieproxyconfigforcurrentuser#remarks
            unsafe {
                GlobalFree(proxy_config.lpszProxyBypass as *mut c_void);
            }
        }

        Ok(cfg)
    }

    pub fn with_proxy_pac(&self, request: &str) -> anyhow::Result<Option<String>> {
        if let Some(pac_url) = &self.pac_url {
            let mut request = request.chars().map(|c| c as u8 as u16).collect::<Vec<_>>();
            // Need nul terminated
            request.push(0);
            let req = U16CStr::from_slice(&request).map_err(|e| anyhow::anyhow!("{e}"))?;

            // Safety: Nothing unsafe here
            let handle = unsafe {
                WinHttpOpen(
                    std::ptr::null(),
                    WINHTTP_ACCESS_TYPE_DEFAULT_PROXY,
                    std::ptr::null(),
                    std::ptr::null(),
                    0,
                )
            };

            if handle.is_null() {
                return Err(anyhow::anyhow!("Cannot call WinHttpOpen"));
            }

            let mut auto_proxy_options = WINHTTP_AUTOPROXY_OPTIONS {
                // Here we use the provided pac url
                dwFlags: WINHTTP_AUTOPROXY_CONFIG_URL,
                dwAutoDetectFlags: WINHTTP_AUTO_DETECT_TYPE_DHCP | WINHTTP_AUTO_DETECT_TYPE_DNS_A,
                lpszAutoConfigUrl: pac_url.as_ptr(),
                lpvReserved: std::ptr::null_mut(),
                dwReserved: 0,
                fAutoLogonIfChallenged: 1,
            };

            let mut proxy_info = WINHTTP_PROXY_INFO {
                dwAccessType: 0,
                lpszProxy: std::ptr::null_mut(),
                lpszProxyBypass: std::ptr::null_mut(),
            };

            // Run the pac script and determines which proxy must be used
            // Safety: Nothing unsafe here
            unsafe {
                if WinHttpGetProxyForUrl(
                    handle,
                    req.as_ptr(),
                    &mut auto_proxy_options,
                    &mut proxy_info,
                ) == 0
                {
                    return Err(anyhow::anyhow!(
                        "WinHttpGetProxyForUrl failed with code: {}",
                        GetLastError()
                    ));
                }
            }

            // TODO: We don't handle ProxyBypass for the moment
            if !proxy_info.lpszProxyBypass.is_null() {
                // Safety: We free a pointer that was allocated by windows and we don't use it after.
                // According to Windows doc, we should free it.
                // https://learn.microsoft.com/en-us/windows/win32/api/winhttp/nf-winhttp-winhttpgetproxyforurl#parameters
                unsafe {
                    GlobalFree(proxy_info.lpszProxyBypass as *mut c_void);
                }
            }

            // Determines if there is a proxy which must be used
            let res = if !proxy_info.lpszProxy.is_null() {
                // Safety: According to Windows doc, this parameter is a nul terminated widestring
                let proxy = unsafe { U16CStr::from_ptr_str(proxy_info.lpszProxy) };
                let proxy = String::from_utf16(proxy.as_slice())?;

                // Safety: We free a pointer that was allocated by windows and we don't use it after.
                // According to Windows doc, we should free it.
                // https://learn.microsoft.com/en-us/windows/win32/api/winhttp/nf-winhttp-winhttpgetproxyforurl#parameters
                unsafe {
                    GlobalFree(proxy_info.lpszProxy as *mut c_void);
                }

                Some(proxy)
            } else {
                None
            };

            // Safety: We free a pointer that was allocated by windows and we don't use it after.
            // According to Windows doc, we should close it.
            // https://learn.microsoft.com/en-us/windows/win32/api/winhttp/nf-winhttp-winhttpopen#remarks
            unsafe {
                WinHttpCloseHandle(handle);
            }

            return Ok(res);
        }

        Ok(None)
    }
}

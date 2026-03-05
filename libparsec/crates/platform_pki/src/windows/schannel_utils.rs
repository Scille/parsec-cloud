// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use schannel::{
    cert_context::CertContext, cert_store::CertStore, ncrypt_key::NcryptKey, RawPointer,
};
use windows_sys::Win32::Security::Cryptography;

pub(super) fn get_raw_store(store: &CertStore) -> Cryptography::HCERTSTORE {
    // SAFETY: Ideally we would use `schannel::Inner::as_inner` to get the already typed type, but it's a
    // private trait.
    // Instead, the following that require to cast to the correct type.
    (unsafe { RawPointer::as_ptr(store) }) as Cryptography::HCERTSTORE
}

pub(super) unsafe fn cert_context_from_raw(
    raw_context: *mut Cryptography::CERT_CONTEXT,
) -> CertContext {
    RawPointer::from_ptr(raw_context as *mut std::os::raw::c_void)
}

pub(super) fn cert_context_to_raw(cert_context: &CertContext) -> *const Cryptography::CERT_CONTEXT {
    // SAFETY: Ideally we would use `schannel::Inner::as_inner` to get the already typed type, but it's a
    // private trait.
    // Instead, the following that require to cast to the correct type.
    (unsafe { RawPointer::as_ptr(cert_context) }) as *const Cryptography::CERT_CONTEXT
}

pub(super) fn ncrypt_key_to_ptr(ncrypt_key: &NcryptKey) -> Cryptography::NCRYPT_KEY_HANDLE {
    // SAFETY: NcryptKey is obtained from an NCRYPT_KEY_HANDLE. RawPointer::as_ptr() returns
    // a *mut c_void pointing to the inner handle field. We cast to *mut usize to read the
    // actual handle value without moving the non-Copy c_void.
    let ptr = unsafe { RawPointer::as_ptr(ncrypt_key) };
    log::trace!("ptr  (address of handle field) : {:p}", ptr);
    ptr as Cryptography::NCRYPT_KEY_HANDLE
}

pub(super) struct PrivateKey {
    handle: Cryptography::NCRYPT_KEY_HANDLE,
    must_free: bool,
}

impl Drop for PrivateKey {
    fn drop(&mut self) {
        if self.must_free {
            // TODO:
            #[expect(clippy::undocumented_unsafe_blocks)]
            unsafe {
                Cryptography::NCryptFreeObject(self.handle);
            }
        }
    }
}

impl PrivateKey {
    pub fn handle(&self) -> Cryptography::NCRYPT_KEY_HANDLE {
        self.handle
    }
}

/// Return a private key handle from a certificate context
pub(super) fn acquire_private_key(context: &CertContext) -> std::io::Result<PrivateKey> {
    // TODO:
    #[expect(clippy::undocumented_unsafe_blocks)]
    let raw_context = unsafe { RawPointer::as_ptr(context) as *const Cryptography::CERT_CONTEXT };
    let mut handle = Cryptography::NCRYPT_KEY_HANDLE::default();
    let flags = Cryptography::CRYPT_ACQUIRE_COMPARE_KEY_FLAG // Ensure that the private key correspond to the
    // certificate
    | Cryptography::CRYPT_ACQUIRE_PREFER_NCRYPT_KEY_FLAG // Only return a Ncrypt key handle (since we
    // do not cater to Windows SRV 2003 nor Windows XP)
    ;
    let mut spec = 0;
    let mut must_free = windows_sys::core::BOOL::default();
    // TODO:
    #[expect(clippy::undocumented_unsafe_blocks)]
    // Doc: https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-cryptacquirecertificateprivatekey
    let res = unsafe {
        Cryptography::CryptAcquireCertificatePrivateKey(
            raw_context,
            flags,
            std::ptr::null_mut(),
            &mut handle,
            &mut spec,
            &mut must_free,
        )
    };

    if res == 0 {
        // `res` correspond to `BOOL`, and is set to `FALSE` on failure and the error reason is
        // obtain by a call to `GetLastError`.
        return Err(std::io::Error::last_os_error());
    }

    // Check if the returned key is of expected spec
    if spec == Cryptography::CERT_NCRYPT_KEY_SPEC {
        Ok(PrivateKey {
            handle,
            must_free: must_free == 1,
        })
    } else {
        log::debug!("Got an HCRYPTPROV handle, trying to translate it");
        let mut ncrypt_handle = Cryptography::NCRYPT_KEY_HANDLE::default();
        // TODO:
        #[expect(clippy::undocumented_unsafe_blocks)]
        let res = unsafe {
            Cryptography::NCryptTranslateHandle(
                std::ptr::null_mut(),
                &mut ncrypt_handle,
                handle,
                0, // We do not have an handle to HCRYPTKEY
                spec,
                0,
            )
        };

        if must_free != 0 {
            // TODO:
            #[expect(clippy::undocumented_unsafe_blocks)]
            unsafe {
                Cryptography::CryptReleaseContext(handle, 0)
            };
        }

        if res != 0 {
            log::warn!("Failed to translate legacy handle to NCRYPT");
            return Err(std::io::Error::from_raw_os_error(res));
        }

        Ok(PrivateKey {
            handle: ncrypt_handle,
            must_free: true,
        })
    }
}

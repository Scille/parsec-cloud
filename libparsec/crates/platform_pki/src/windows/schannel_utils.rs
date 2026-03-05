// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use schannel::{cert_context::CertContext, cert_store::CertStore, RawPointer};
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

pub(super) struct PrivateKey {
    handle: Cryptography::NCRYPT_KEY_HANDLE,
    must_free: bool,
}

impl Drop for PrivateKey {
    fn drop(&mut self) {
        if self.must_free {
            // SAFETY: The handle to free is obtain by a previous call to
            // `CryptAcquireCertificatePrivateKey` where proper check where made.
            // Furthermore, we only free the handle if asked to by
            // `CryptAcquireCertificatePrivateKey` (via `must_free` param).
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
    let raw_context = cert_context_to_raw(context);
    let mut handle = Cryptography::HCRYPTPROV_OR_NCRYPT_KEY_HANDLE::default();
    let flags = Cryptography::CRYPT_ACQUIRE_COMPARE_KEY_FLAG // Ensure that the private key correspond to the
    // certificate
    | Cryptography::CRYPT_ACQUIRE_PREFER_NCRYPT_KEY_FLAG // Prefer Ncrypt key handle for legacy
    // handle, we will try to convert them to Ncrypt.
    ;
    let mut spec = Cryptography::CERT_KEY_SPEC::default();
    let mut must_free = windows_sys::core::BOOL::default();
    // SAFETY: We pass the expected parameters following the documentation.
    // The certificate context handle is a valid handle obtained by converting `CertContext`.
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

    let must_free = must_free == 1;
    // Check if the returned key is of expected spec
    if spec == Cryptography::CERT_NCRYPT_KEY_SPEC {
        Ok(PrivateKey { handle, must_free })
    } else {
        convert_legacy_hcryptprov_to_ncrypt_key(handle, spec, must_free)
    }
}

/// Function that convert a legacy hcryptprov to ncrypt key handle.
///
/// This is a best effort conversion to try to support legacy smartcard reader not implementing
/// CNG API.
fn convert_legacy_hcryptprov_to_ncrypt_key(
    handle: Cryptography::HCRYPTPROV_LEGACY,
    spec: Cryptography::CERT_KEY_SPEC,
    must_free: bool,
) -> std::io::Result<PrivateKey> {
    log::trace!("Trying to convert HCRYPTPROV to NCRYPT_KEY_HANDLE (spec: {spec:#x})");
    let mut ncrypt_handle = Cryptography::NCRYPT_KEY_HANDLE::default();
    // SAFETY: Outside of `handle` & `spec` the parameters are correct according to the
    // documentation.
    // Doc: https://learn.microsoft.com/en-us/windows/win32/api/ncrypt/nf-ncrypt-ncrypttranslatehandle
    let res = unsafe {
        Cryptography::NCryptTranslateHandle(
            std::ptr::null_mut(), // We do not need a handle to the NCrypt provider.
            &mut ncrypt_handle,
            handle,
            0, // We do not have a handle to HCRYPTKEY.
            spec,
            0,
        )
    };

    if must_free {
        // SAFETY: We free the handle that we tried to translate.
        // Error or not it's no longer needed.
        // Doc: https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-cryptreleasecontext
        unsafe { Cryptography::CryptReleaseContext(handle, 0) };
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

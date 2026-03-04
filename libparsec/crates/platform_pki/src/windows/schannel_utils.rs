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
    let mut handle = Cryptography::NCRYPT_KEY_HANDLE::default();
    let flags = Cryptography::CRYPT_ACQUIRE_COMPARE_KEY_FLAG // Ensure that the private key correspond to the
    // certificate
    | Cryptography::CRYPT_ACQUIRE_ONLY_NCRYPT_KEY_FLAG // Only return a Ncrypt key handle (since we
    // do not cater to Windows SRV 2003 nor Windows XP)
    ;
    let mut spec = 0;
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

    // Check if the returned key is of expected spec
    if spec != Cryptography::CERT_NCRYPT_KEY_SPEC {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("invalid returned key spec ({spec:#x})"),
        ));
    }

    Ok(PrivateKey {
        handle,
        must_free: must_free == 1,
    })
}

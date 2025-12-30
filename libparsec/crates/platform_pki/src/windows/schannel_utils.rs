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
    // SAFETY: NcryptKey is obtain from an NCRYPT_KEY_HANDLE, here we retrieve the underlying
    // handle.
    (unsafe { RawPointer::as_ptr(ncrypt_key) }) as Cryptography::NCRYPT_KEY_HANDLE
}

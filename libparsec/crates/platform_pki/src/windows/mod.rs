// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::ffi::c_void;

use crate::{
    CertificateDer, CertificateHash, CertificateReference, CertificateReferenceIdOrHash,
    GetDerEncodedCertificateError,
};
use bytes::Bytes;
use schannel::{cert_context::CertContext, cert_context::HashAlgorithm, cert_store::CertStore};
use windows_sys::Win32::Security::Cryptography::{
    CertGetCertificateContextProperty, CERT_CONTEXT, CERT_KEY_PROV_INFO_PROP_ID, HCERTSTORE,
    UI::CryptUIDlgSelectCertificateFromStore,
};

fn open_store() -> std::io::Result<CertStore> {
    CertStore::open_current_user("My")
}

impl CertificateHash {
    fn get_hash_algo(&self) -> HashAlgorithm {
        match self {
            CertificateHash::SHA256(..) => HashAlgorithm::sha256(),
        }
    }
}

impl PartialEq<CertContext> for CertificateHash {
    fn eq(&self, other: &CertContext) -> bool {
        let Ok(cert_hash) = other.fingerprint(self.get_hash_algo()) else {
            return false;
        };
        match self {
            CertificateHash::SHA256(expected) => expected.as_ref() == cert_hash.as_slice(),
        }
    }
}

impl CertificateHash {
    fn from_certificate_context(context: &CertContext) -> std::io::Result<Self> {
        context
            .fingerprint(HashAlgorithm::sha256())
            .and_then(|buf| {
                buf.try_into().map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Not a sha256 hash")
                })
            })
            .map(Self::SHA256)
    }
}

fn find_certificate(
    store: &CertStore,
    certificate_ref: &CertificateReference,
) -> Option<CertContext> {
    let matcher: Box<dyn Fn(&CertContext) -> bool> = match certificate_ref {
        CertificateReference::Id(id) => Box::new(|candidate: &CertContext| {
            cert_cmp_id(candidate, id.as_ref()).unwrap_or_default()
        }),
        CertificateReference::Hash(hash) => {
            Box::new(move |candidate: &CertContext| hash == candidate)
        }
        CertificateReference::IdOrHash(id_or_hash) => Box::new(move |candidate: &CertContext| {
            cert_cmp_id(candidate, id_or_hash.id.as_ref()).unwrap_or_default()
                || &id_or_hash.hash == candidate
        }),
    };

    store.certs().find(matcher)
}

fn cert_cmp_id(cert_context: &CertContext, expected_id: &[u8]) -> std::io::Result<bool> {
    get_certificate_id(cert_context).map(|cert_id| cert_id == expected_id)
}

fn get_certificate_id(cert_context: &CertContext) -> std::io::Result<Vec<u8>> {
    // SAFETY: We use `CertGetCertificateContextProperty` by using a correct windows pointer and
    // first getting the required size to allocate a buffer.
    unsafe {
        let raw_context = schannel::RawPointer::as_ptr(cert_context) as *const CERT_CONTEXT;
        // 1. Get size of cert id
        let mut len = 0u32;
        // Documentation can be found here:
        // https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-certgetcertificatecontextproperty
        // Rust doc:
        // https://docs.rs/windows-sys/latest/windows_sys/Win32/Security/Cryptography/fn.CertGetCertificateContextProperty.html
        let ret = CertGetCertificateContextProperty(
            raw_context,
            CERT_KEY_PROV_INFO_PROP_ID,
            std::ptr::null_mut(),
            &mut len,
        );
        if ret == 0 {
            return Err(std::io::Error::last_os_error());
        }
        // 2. Get actual cert id
        let mut buf = vec![0u8; len as usize];
        let ret = CertGetCertificateContextProperty(
            raw_context,
            CERT_KEY_PROV_INFO_PROP_ID,
            buf.as_mut_ptr() as *mut c_void,
            &mut len,
        );
        if ret == 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(buf)
        }
    }
}

pub fn get_der_encoded_certificate(
    certificate_ref: &CertificateReference,
) -> Result<CertificateDer, GetDerEncodedCertificateError> {
    let store = open_store().map_err(GetDerEncodedCertificateError::CannotOpenStore)?;
    let cert_context =
        find_certificate(&store, certificate_ref).ok_or(GetDerEncodedCertificateError::NotFound)?;

    let reference = get_id_and_hash_from_cert_context(&cert_context)
        .map_err(GetDerEncodedCertificateError::CannotGetCertificateInfo)?;
    let der_content = Bytes::copy_from_slice(cert_context.to_der());

    Ok(CertificateDer {
        cert_ref: reference,
        der_content,
    })
}

fn get_id_and_hash_from_cert_context(
    context: &CertContext,
) -> std::io::Result<CertificateReferenceIdOrHash> {
    let id = get_certificate_id(context)?.into();
    let hash = CertificateHash::from_certificate_context(context)?;

    Ok(CertificateReferenceIdOrHash { id, hash })
}

#[derive(Debug, thiserror::Error)]
pub enum ShowCertificateSelectionDialogError {
    #[error("Cannot open certificate store: {}", .0)]
    CannotOpenStore(std::io::Error),
    #[error("Cannot get certificate info: {}", .0)]
    CannotGetCertificateInfo(std::io::Error),
}

// TODO: This is specific to windows, it cannot be replicated on other platform.
// Instead, we likely need to go the manual way and show a custom dialog on the client side with a
// list of certificate that we retrieve from the platform certstore.
pub fn show_certificate_selection_dialog(
) -> Result<Option<CertificateReferenceIdOrHash>, ShowCertificateSelectionDialogError> {
    let store = open_store().map_err(ShowCertificateSelectionDialogError::CannotOpenStore)?;
    ask_user_to_select_certificate(&store)
        .as_ref()
        .map(get_id_and_hash_from_cert_context)
        .transpose()
        .map_err(ShowCertificateSelectionDialogError::CannotGetCertificateInfo)
}

fn ask_user_to_select_certificate(store: &CertStore) -> Option<CertContext> {
    // SAFETY: Ideally we would use `schannel::Inner::as_inner` to get the already typed type, but it's a
    // private trait.
    // Instead, the following that require to cast to the correct type.
    let raw_store = unsafe { schannel::RawPointer::as_ptr(store) } as HCERTSTORE;

    // SAFETY: We use `CryptUIDlgSelectCertificateFromStore` by providing it with a correct pointer
    // that come from windows, and use default value (NULL) as detailed by the documentation.
    //
    // Documentation of the below function:
    // https://learn.microsoft.com/en-us/windows/win32/api/cryptuiapi/nf-cryptuiapi-cryptuidlgselectcertificatefromstore
    // Rust doc:
    // https://docs.rs/windows-sys/latest/windows_sys/Win32/Security/Cryptography/UI/fn.CryptUIDlgSelectCertificateFromStore.html
    unsafe {
        let raw_cert_context = CryptUIDlgSelectCertificateFromStore(
            raw_store,
            core::ptr::null_mut(), // Will create a dedicated window
            core::ptr::null(),     // Use default title
            core::ptr::null(),     // Use default dialog phrase
            0,                     // Do not hide any column
            0,                     // Not used by windows per documentation
            core::ptr::null(),     // Reversed for future use per documentation
        );
        if raw_cert_context.is_null() {
            None
        } else {
            Some(schannel::RawPointer::from_ptr(
                raw_cert_context as *mut std::os::raw::c_void,
            ))
        }
    }
}

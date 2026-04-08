// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod certificate;
mod find_in_store;
mod private_key;
mod schannel_utils;
use schannel::{
    cert_context::{CertContext, HashAlgorithm},
    cert_store::CertStore,
};
use windows_sys::Win32::Security::Cryptography::UI::CryptUIDlgSelectCertificateFromStore;

use libparsec_types::prelude::*;

use crate::{
    AvailablePkiCertificate, PkiCertificate, PkiPrivateKey, PkiScwsConfig, PkiSystemInitError,
    PkiSystemListUserCertificateError, PkiSystemOpenCertificateError,
    ShowCertificateSelectionDialogError, X509CertificateDer, X509TrustAnchor,
};

pub(crate) use certificate::PlatformPkiCertificate;
pub(crate) use private_key::PlatformPkiPrivateKey;

#[derive(Debug)]
pub struct PlatformPkiSystem {
    my_cert_store: CertStore,
}

impl PlatformPkiSystem {
    pub async fn init(_scws_config: Option<PkiScwsConfig>) -> Result<Self, PkiSystemInitError> {
        CertStore::open_current_user("My")
            .map(|store| Self {
                my_cert_store: store,
            })
            .context("Cannot open windows cert store")
            .map_err(PkiSystemInitError::Internal)
    }

    pub async fn open_certificate(
        &self,
        cert_ref: &X509CertificateReference,
    ) -> Result<Option<PkiCertificate>, PkiSystemOpenCertificateError> {
        Ok(find_certificate(&self.my_cert_store, cert_ref)
            .map(PlatformPkiCertificate::from)
            .map(wrap_platform_certificate))
    }

    pub async fn list_user_certificates(
        &self,
    ) -> Result<Vec<AvailablePkiCertificate>, PkiSystemListUserCertificateError> {
        Ok(self
            .my_cert_store
            .certs()
            .map(|cert| AvailablePkiCertificate::load_der(&cert.to_der()))
            .collect())
    }
}

#[cfg(not(feature = "test-with-testbed"))]
pub(super) fn wrap_platform_certificate(platform: PlatformPkiCertificate) -> PkiCertificate {
    PkiCertificate { platform }
}

#[cfg(feature = "test-with-testbed")]
pub(super) fn wrap_platform_certificate(platform: PlatformPkiCertificate) -> PkiCertificate {
    PkiCertificate {
        platform: crate::testbed::MaybeWithTestbed::WithPlatform(platform),
    }
}

#[cfg(not(feature = "test-with-testbed"))]
pub(super) fn wrap_platform_private_key(platform: PlatformPkiPrivateKey) -> PkiPrivateKey {
    PkiPrivateKey { platform }
}

#[cfg(feature = "test-with-testbed")]
pub(super) fn wrap_platform_private_key(platform: PlatformPkiPrivateKey) -> PkiPrivateKey {
    PkiPrivateKey {
        platform: crate::testbed::MaybeWithTestbed::WithPlatform(platform),
    }
}

fn open_store() -> std::io::Result<CertStore> {
    CertStore::open_current_user("My")
}

fn get_hash_algo(hash: &X509CertificateHash) -> HashAlgorithm {
    match hash {
        X509CertificateHash::SHA256(..) => HashAlgorithm::sha256(),
    }
}

fn hash_from_certificate_context(context: &CertContext) -> std::io::Result<X509CertificateHash> {
    context
        .fingerprint(HashAlgorithm::sha256())
        .and_then(|buf| {
            buf.try_into().map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Not a sha256 hash")
            })
        })
        .map(X509CertificateHash::SHA256)
}

fn find_certificate(
    store: &CertStore,
    certificate_ref: &X509CertificateReference,
) -> Option<CertContext> {
    certificate_ref
        .get_uri::<X509WindowsCngURI>()
        .and_then(|uri| {
            log::trace!("Looking for cert with uri: {uri}");
            let mut uri = uri.clone();
            find_in_store::find_cert_in_store(store, find_in_store::CertFilter::cert_id(&mut uri))
                .next()
        })
        .inspect(|_v| log::trace!("Certificate found using uri"))
        .or_else(|| {
            log::trace!("Certificate not found by uri, trying by fingerprint");
            let hash_algo = get_hash_algo(&certificate_ref.hash);
            store.certs().find(|candidate: &CertContext| {
                let Ok(cert_hash) = candidate.fingerprint(hash_algo) else {
                    return false;
                };
                match &certificate_ref.hash {
                    X509CertificateHash::SHA256(data) => data.as_ref() == cert_hash.as_slice(),
                }
            })
        })
}

fn get_certificate_uri(cert_context: &CertContext) -> X509WindowsCngURI {
    let raw_context = schannel_utils::cert_context_to_raw(cert_context);
    // SAFETY: The raw pointer come from the inner valid pointer of `cert_context`
    // that is of type `Cryptography::CERT_CONTEXT`
    let cert_info = unsafe { *(*raw_context).pCertInfo };

    // SAFETY: Issuer is of type `CRYPT_INTEGER_BLOB` and is obtain from a valid cert_context.
    let issuer = unsafe {
        std::slice::from_raw_parts(cert_info.Issuer.pbData, cert_info.Issuer.cbData as usize)
    }
    .to_vec();
    // SAFETY: SerialNumber is of type `CRYPT_INTEGER_BLOB` and is obtain from a valid cert_context.
    let serial_number = unsafe {
        std::slice::from_raw_parts(
            cert_info.SerialNumber.pbData,
            cert_info.SerialNumber.cbData as usize,
        )
    }
    .to_vec();

    X509WindowsCngURI {
        issuer,
        serial_number,
    }
}

fn get_id_and_hash_from_cert_context(
    context: &CertContext,
) -> std::io::Result<X509CertificateReference> {
    let uri = get_certificate_uri(context);
    let hash = hash_from_certificate_context(context)?;

    Ok(X509CertificateReference::from(hash).add_or_replace_uri(uri))
}

#[derive(Debug, thiserror::Error)]
enum ListCertificatesError {
    #[error("Cannot open certificate store: {0}")]
    CannotOpenStore(std::io::Error),
}

type ListTrustedRootCertificatesError = ListCertificatesError;

async fn list_trusted_root_certificate_anchors(
) -> Result<Vec<X509TrustAnchor<'static>>, ListTrustedRootCertificatesError> {
    let store = CertStore::open_current_user("Root")
        .map_err(ListTrustedRootCertificatesError::CannotOpenStore)?;

    let res = store
        .certs()
        // NOTE: We could filter the root certificate on their valid usages, but we could easily
        // miss out the expected root certificate as the init script
        // `../../examples/init_windows_credstore.ps1` generate a test CA cert with the type
        // `SSLServerAuthentication` (the default value).
        //
        // .filter(|ctx| match ctx.valid_uses() {
        //     Ok(ValidUses::All) => true,
        //     Ok(ValidUses::Oids(oids)) => {
        //         log::trace!(cert_name:?=ctx.friendly_name().ok(), oids:?; "Certificate valid uses");
        //         false
        //     }
        //     Err(err) => { log::trace!(cert_name:?=ctx.friendly_name().ok(), err:?; "Certificate with no valid use, skipping"); false },
        // })
        .filter_map(|ctx| {
            webpki::anchor_from_trusted_cert(&ctx.to_der().into())
                .map(|anchor| anchor.to_owned())
                .inspect_err(|err| {
                    log::warn!(
                        "Invalid root certificate: name={:?}, fingerprint={:?}, err={err}",
                        ctx.friendly_name().ok(),
                        ctx.fingerprint(HashAlgorithm::sha256()).ok()
                    )
                })
                .ok()
        })
        .collect::<Vec<_>>();

    Ok(res)
}

type ListIntermediateCertificatesError = ListCertificatesError;

async fn list_intermediate_certificates(
) -> Result<Vec<X509CertificateDer<'static>>, ListIntermediateCertificatesError> {
    let store = CertStore::open_current_user("CA")
        .map_err(ListIntermediateCertificatesError::CannotOpenStore)?;

    Ok(store
        .certs()
        .map(|ctx| X509CertificateDer::from(ctx.to_der()).into_owned())
        .collect::<Vec<_>>())
}

pub fn show_certificate_selection_dialog_windows_only(
) -> Result<Option<X509CertificateReference>, ShowCertificateSelectionDialogError> {
    let store = open_store().map_err(ShowCertificateSelectionDialogError::CannotOpenStore)?;
    ask_user_to_select_certificate(&store)
        .as_ref()
        .map(get_id_and_hash_from_cert_context)
        .transpose()
        .map_err(ShowCertificateSelectionDialogError::CannotGetCertificateInfo)
}

fn ask_user_to_select_certificate(store: &CertStore) -> Option<CertContext> {
    let raw_store = schannel_utils::get_raw_store(store);

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
            Some(schannel_utils::cert_context_from_raw(raw_cert_context))
        }
    }
}

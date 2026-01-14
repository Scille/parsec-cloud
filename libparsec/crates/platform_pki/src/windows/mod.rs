// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod find_in_store;
mod schannel_utils;

use crate::{
    CertificateDer, DecryptMessageError, DecryptedMessage, EncryptMessageError, EncryptedMessage,
    GetDerEncodedCertificateError, ListIntermediateCertificatesError,
    ListTrustedRootCertificatesError, ShowCertificateSelectionDialogError, SignMessageError,
    SignedMessageFromPki,
};

use bytes::Bytes;
use schannel::{
    cert_context::{CertContext, HashAlgorithm, PrivateKey},
    cert_store::CertStore,
    ncrypt_key::NcryptKey,
};
use sha2::Digest as _;
use windows_sys::Win32::Security::Cryptography::{
    NCryptDecrypt, NCryptEncrypt, NCryptSignHash, BCRYPT_OAEP_PADDING_INFO,
    BCRYPT_PSS_PADDING_INFO, NCRYPT_PAD_OAEP_FLAG, NCRYPT_PAD_PSS_FLAG, NCRYPT_SHA256_ALGORITHM,
    UI::CryptUIDlgSelectCertificateFromStore,
};

use libparsec_types::prelude::*;

pub fn is_available() -> bool {
    open_store().is_ok()
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

pub fn get_der_encoded_certificate(
    certificate_ref: &X509CertificateReference,
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
) -> std::io::Result<X509CertificateReference> {
    let uri = get_certificate_uri(context);
    let hash = hash_from_certificate_context(context)?;

    Ok(X509CertificateReference::from(hash).add_or_replace_uri(uri))
}

pub fn list_trusted_root_certificate_anchors(
) -> Result<Vec<rustls_pki_types::TrustAnchor<'static>>, ListTrustedRootCertificatesError> {
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

pub fn list_intermediate_certificates(
) -> Result<Vec<rustls_pki_types::CertificateDer<'static>>, ListIntermediateCertificatesError> {
    let store = CertStore::open_current_user("CA")
        .map_err(ListIntermediateCertificatesError::CannotOpenStore)?;

    Ok(store
        .certs()
        .map(|ctx| rustls_pki_types::CertificateDer::from(ctx.to_der()).into_owned())
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

pub fn sign_message(
    message: &[u8],
    certificate_ref: &X509CertificateReference,
) -> Result<SignedMessageFromPki, SignMessageError> {
    let store = open_store().map_err(SignMessageError::CannotOpenStore)?;
    let cert_context =
        find_certificate(&store, certificate_ref).ok_or(SignMessageError::NotFound)?;
    let reference = get_id_and_hash_from_cert_context(&cert_context)
        .map_err(SignMessageError::CannotGetCertificateInfo)?;
    let keypair = get_keypair(&cert_context).map_err(SignMessageError::CannotAcquireKeypair)?;
    let (algo, signature) = match keypair {
        // We do not support a CryptoAPI provider as its API is marked for depreciation by windows.
        PrivateKey::CryptProv(..) => {
            todo!("Use CryptGetUserKey to get the keypair")
        }
        // Handle to a CryptoGraphy Next Generation (CNG) API
        PrivateKey::NcryptKey(handle) => ncrypt_sign_message_with_rsa(message, &handle),
    }
    .map_err(SignMessageError::CannotSign)?;

    Ok(SignedMessageFromPki {
        algo,
        signature: signature.into(),
        cert_ref: reference,
    })
}

fn get_keypair(context: &CertContext) -> Result<PrivateKey, std::io::Error> {
    let mut acq_keypair = context.private_key();
    acq_keypair
        // Ensure private key correspond to the certificate public key.
        .compare_key(true)
        .acquire()
}

fn ncrypt_sign_message_with_rsa(
    message: &[u8],
    handle: &NcryptKey,
) -> std::io::Result<(PkiSignatureAlgorithm, Vec<u8>)> {
    const ALGO: PkiSignatureAlgorithm = PkiSignatureAlgorithm::RsassaPssSha256;
    let hash = sha2::Sha256::digest(message);
    let raw_handle = schannel_utils::ncrypt_key_to_ptr(handle);

    // SAFETY: We follow the windows documentation by correctly passing the correct flags according
    // to padding_info type, and the other pointer are either coming from allocated buffer or null
    // pointer with their correct associated sizes.
    //
    // Documentation of the below function:
    // https://learn.microsoft.com/en-us/windows/win32/api/ncrypt/nf-ncrypt-ncryptsignhash
    // Rust doc:
    // https://docs.rs/windows-sys/latest/windows_sys/Win32/Security/Cryptography/fn.NCryptSignHash.html
    unsafe {
        // We sign the hash using RSA_PSS_SHA256 algo.
        let flags = NCRYPT_PAD_PSS_FLAG;
        // https://learn.microsoft.com/en-us/windows/win32/api/bcrypt/ns-bcrypt-bcrypt_pss_padding_info
        let padding_info = BCRYPT_PSS_PADDING_INFO {
            pszAlgId: NCRYPT_SHA256_ALGORITHM,
            // This determine the size of the random salt, from WolfSSL it's the convention to use
            // a salt of the same size of the hash.
            cbSalt: hash.len() as u32,
        };
        let padding_info_ptr = (&raw const padding_info) as *const std::os::raw::c_void;
        let mut len = 0_u32;

        // 1. Get the size of the resulting signature
        let res = NCryptSignHash(
            raw_handle,
            padding_info_ptr,
            hash.as_ptr(),
            hash.len() as u32,
            std::ptr::null_mut(),
            0,
            &mut len,
            flags,
        );

        if res != 0 {
            return Err(std::io::Error::last_os_error());
        }

        // 2. Fill the actual signature in a buffer
        let mut buff = vec![0_u8; len as usize];
        let res = NCryptSignHash(
            raw_handle,
            padding_info_ptr,
            hash.as_ptr(),
            hash.len() as u32,
            buff.as_mut_ptr(),
            buff.len() as u32,
            &mut len,
            flags,
        );

        if res != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok((ALGO, buff))
    }
}

pub fn encrypt_message(
    message: &[u8],
    certificate_ref: &X509CertificateReference,
) -> Result<EncryptedMessage, EncryptMessageError> {
    let store = open_store().map_err(EncryptMessageError::CannotOpenStore)?;
    let cert_context =
        find_certificate(&store, certificate_ref).ok_or(EncryptMessageError::NotFound)?;
    let reference = get_id_and_hash_from_cert_context(&cert_context)
        .map_err(EncryptMessageError::CannotGetCertificateInfo)?;
    let keypair = get_keypair(&cert_context).map_err(EncryptMessageError::CannotAcquireKeypair)?;
    let (algo, ciphered) = match keypair {
        // We do not support a CryptoAPI provider as its API is marked for depreciation by windows.
        PrivateKey::CryptProv(..) => {
            todo!("Use CryptGetUserKey to get the keypair")
        }
        // Handle to a CryptoGraphy Next Generation (CNG) API
        PrivateKey::NcryptKey(handle) => ncrypt_encrypt_message_with_rsa(message, &handle),
    }
    .map_err(EncryptMessageError::CannotEncrypt)?;

    Ok(EncryptedMessage {
        algo,
        ciphered: ciphered.into(),
        cert_ref: reference,
    })
}

fn ncrypt_encrypt_message_with_rsa(
    message: &[u8],
    handle: &NcryptKey,
) -> std::io::Result<(PKIEncryptionAlgorithm, Vec<u8>)> {
    const ALGO: PKIEncryptionAlgorithm = PKIEncryptionAlgorithm::RsaesOaepSha256;
    let raw_handle = schannel_utils::ncrypt_key_to_ptr(handle);

    // SAFETY: We follow the windows documentation by correctly passing the correct flags according
    // to padding_info type, and the other pointer are either coming from allocated buffer or null
    // pointer with their correct associated sizes.
    //
    // Documentation of the below function:
    // https://learn.microsoft.com/en-us/windows/win32/api/ncrypt/nf-ncrypt-ncryptencrypt
    // Rust doc:
    // https://docs.rs/windows-sys/latest/windows_sys/Win32/Security/Cryptography/fn.NCryptEncrypt.html
    unsafe {
        // We pad the ciphered data using OAEP.
        let flags = NCRYPT_PAD_OAEP_FLAG;
        // https://learn.microsoft.com/en-us/windows/win32/api/bcrypt/ns-bcrypt-bcrypt_oaep_padding_info
        let padding_info = BCRYPT_OAEP_PADDING_INFO {
            pszAlgId: NCRYPT_SHA256_ALGORITHM,
            pbLabel: std::ptr::null_mut(),
            cbLabel: 0,
        };
        let padding_info_ptr = (&raw const padding_info) as *const std::os::raw::c_void;
        let mut len = 0_u32;

        // 1. Get size of the resulting ciphered data.
        let res = NCryptEncrypt(
            raw_handle,
            message.as_ptr(),
            message.len() as u32,
            padding_info_ptr,
            std::ptr::null_mut(),
            0,
            &mut len,
            flags,
        );
        if res != 0 {
            return Err(std::io::Error::last_os_error());
        }

        // 2. Actually perform the encryption.
        let mut buff = vec![0_u8; len as usize];
        let res = NCryptEncrypt(
            raw_handle,
            message.as_ptr(),
            message.len() as u32,
            padding_info_ptr,
            buff.as_mut_ptr(),
            buff.len() as u32,
            &mut len,
            flags,
        );
        if res != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok((ALGO, buff))
    }
}

pub fn decrypt_message(
    algo: PKIEncryptionAlgorithm,
    encrypted_message: &[u8],
    certificate_ref: &X509CertificateReference,
) -> Result<DecryptedMessage, DecryptMessageError> {
    let store = open_store().map_err(DecryptMessageError::CannotOpenStore)?;
    let cert_context =
        find_certificate(&store, certificate_ref).ok_or(DecryptMessageError::NotFound)?;
    let reference = get_id_and_hash_from_cert_context(&cert_context)
        .map_err(DecryptMessageError::CannotGetCertificateInfo)?;
    let keypair = get_keypair(&cert_context).map_err(DecryptMessageError::CannotAcquireKeypair)?;
    let data = match keypair {
        // We do not support a CryptoAPI provider as its API is marked for depreciation by windows.
        PrivateKey::CryptProv(..) => {
            todo!("Use CryptGetUserKey to get the keypair")
        }
        // Handle to a CryptoGraphy Next Generation (CNG) API
        PrivateKey::NcryptKey(handle) => {
            if algo != PKIEncryptionAlgorithm::RsaesOaepSha256 {
                todo!("Unsupported encryption algo '{algo}'");
            }
            ncrypt_decrypt_message_with_rsa(encrypted_message, &handle).map(Into::into)
        }
    }
    .map_err(DecryptMessageError::CannotDecrypt)?;

    Ok(DecryptedMessage {
        data,
        cert_ref: reference,
    })
}

fn ncrypt_decrypt_message_with_rsa(
    encrypted_message: &[u8],
    handle: &NcryptKey,
) -> std::io::Result<Vec<u8>> {
    let raw_handle = schannel_utils::ncrypt_key_to_ptr(handle);

    // SAFETY: We follow the windows documentation by correctly passing the correct flags according
    // to padding_info type, and the other pointer are either coming from allocated buffer or null
    // pointer with their correct associated sizes.
    //
    // Documentation of the below function:
    // https://learn.microsoft.com/en-us/windows/win32/api/ncrypt/nf-ncrypt-ncryptdecrypt
    // Rust doc:
    // https://docs.rs/windows-sys/latest/windows_sys/Win32/Security/Cryptography/fn.NCryptDecrypt.html
    unsafe {
        // We have padded the ciphered data using OAEP.
        let flags = NCRYPT_PAD_OAEP_FLAG;
        // https://learn.microsoft.com/en-us/windows/win32/api/bcrypt/ns-bcrypt-bcrypt_oaep_padding_info
        let padding_info = BCRYPT_OAEP_PADDING_INFO {
            pszAlgId: NCRYPT_SHA256_ALGORITHM,
            pbLabel: std::ptr::null_mut(),
            cbLabel: 0,
        };
        let padding_info_ptr = (&raw const padding_info) as *const std::os::raw::c_void;
        let mut len = 0_u32;

        // 1. Get size of the resulting ciphered data.
        let res = NCryptDecrypt(
            raw_handle,
            encrypted_message.as_ptr(),
            encrypted_message.len() as u32,
            padding_info_ptr,
            std::ptr::null_mut(),
            0,
            &mut len,
            flags,
        );
        if res != 0 {
            return Err(std::io::Error::last_os_error());
        }

        // 2. Actually perform the decryption.
        let mut buff = vec![0_u8; len as usize];
        let res = NCryptDecrypt(
            raw_handle,
            encrypted_message.as_ptr(),
            encrypted_message.len() as u32,
            padding_info_ptr,
            buff.as_mut_ptr(),
            buff.len() as u32,
            &mut len,
            flags,
        );
        if res != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(buff)
    }
}

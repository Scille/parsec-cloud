// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::ffi::c_void;

use crate::{
    CertificateDer, DecryptMessageError, DecryptedMessage, EncryptMessageError, EncryptedMessage,
    EncryptionAlgorithm, GetDerEncodedCertificateError, ListTrustedRootCertificatesError,
    ShowCertificateSelectionDialogError, SignMessageError, SignatureAlgorithm,
    SignedMessageFromPki,
};
use bytes::Bytes;
use libparsec_types::{X509CertificateHash, X509CertificateReference, X509WindowsCngURI};
use schannel::{
    cert_context::{CertContext, HashAlgorithm, PrivateKey},
    cert_store::CertStore,
    ncrypt_key::NcryptKey,
    RawPointer,
};
use sha2::Digest as _;
use windows_sys::Win32::Security::Cryptography::{
    CertGetCertificateContextProperty, NCryptDecrypt, NCryptEncrypt, NCryptSignHash,
    BCRYPT_OAEP_PADDING_INFO, BCRYPT_PSS_PADDING_INFO, CERT_CONTEXT, CERT_KEY_PROV_INFO_PROP_ID,
    HCERTSTORE, NCRYPT_KEY_HANDLE, NCRYPT_PAD_OAEP_FLAG, NCRYPT_PAD_PSS_FLAG,
    NCRYPT_SHA256_ALGORITHM, UI::CryptUIDlgSelectCertificateFromStore,
};

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

fn cert_cmp_hash(hash: &X509CertificateHash, other: &CertContext) -> bool {
    let Ok(cert_hash) = other.fingerprint(get_hash_algo(hash)) else {
        return false;
    };
    match hash {
        X509CertificateHash::SHA256(data) => data.as_ref() == cert_hash.as_slice(),
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
    let matcher: Box<dyn Fn(&CertContext) -> bool> =
        match certificate_ref.get_uri::<X509WindowsCngURI>() {
            None => Box::new(move |candidate: &CertContext| {
                cert_cmp_hash(&certificate_ref.hash, candidate)
            }),
            Some(id) => Box::new(move |candidate: &CertContext| {
                cert_cmp_id(candidate, id).unwrap_or_default()
                    || cert_cmp_hash(&certificate_ref.hash, candidate)
            }),
        };

    store.certs().find(matcher)
}

fn cert_cmp_id(
    cert_context: &CertContext,
    expected_id: &X509WindowsCngURI,
) -> std::io::Result<bool> {
    get_certificate_id(cert_context).map(|cert_id| cert_id == expected_id.as_ref())
}

fn get_certificate_id(cert_context: &CertContext) -> std::io::Result<Vec<u8>> {
    // SAFETY: We use `CertGetCertificateContextProperty` by using a correct windows pointer and
    // first getting the required size to allocate a buffer.
    unsafe {
        let raw_context = RawPointer::as_ptr(cert_context) as *const CERT_CONTEXT;
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
    let id = X509WindowsCngURI::from(Bytes::from(get_certificate_id(context)?));
    let hash = hash_from_certificate_context(context)?;

    Ok(X509CertificateReference::from(hash).add_or_replace_uri(id))
}

pub fn list_trusted_root_certificate_der(
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
        .map(|ctx| {
            webpki::anchor_from_trusted_cert(&ctx.to_der().into()).map(|anchor| anchor.to_owned())
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(ListTrustedRootCertificatesError::InvalidRootCertificate)?;

    Ok(res)
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
    // SAFETY: Ideally we would use `schannel::Inner::as_inner` to get the already typed type, but it's a
    // private trait.
    // Instead, the following that require to cast to the correct type.
    let raw_store = unsafe { RawPointer::as_ptr(store) } as HCERTSTORE;

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

pub fn sign_message(
    message: &[u8],
    certificate_ref: &X509CertificateReference,
) -> Result<SignedMessageFromPki, SignMessageError> {
    let store = open_store().map_err(SignMessageError::CannotOpenStore)?;
    let cert_context =
        find_certificate(&store, certificate_ref).ok_or(SignMessageError::NotFound)?;
    let reference = get_id_and_hash_from_cert_context(&cert_context)
        .map_err(SignMessageError::CannotGetCertificateInfo)?;
    let keypair = get_keypair(&cert_context)?;
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

fn get_keypair(context: &CertContext) -> Result<PrivateKey, crate::errors::BaseKeyPairError> {
    let mut acq_keypair = context.private_key();
    acq_keypair
        // Ensure private key correspond to the certificate public key.
        .compare_key(true)
        .acquire()
        .map_err(crate::errors::BaseKeyPairError::CannotAcquireKeypair)
}

fn ncrypt_sign_message_with_rsa(
    message: &[u8],
    handle: &NcryptKey,
) -> std::io::Result<(SignatureAlgorithm, Vec<u8>)> {
    const ALGO: SignatureAlgorithm = SignatureAlgorithm::RsassaPssSha256;
    let hash = sha2::Sha256::digest(message);
    // SAFETY: NcryptKey is obtain from an NCRYPT_KEY_HANDLE, here we retrieve the underlying
    // handle.
    let raw_handle = unsafe { RawPointer::as_ptr(handle) } as NCRYPT_KEY_HANDLE;

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
    let keypair = get_keypair(&cert_context)?;
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
) -> std::io::Result<(EncryptionAlgorithm, Vec<u8>)> {
    const ALGO: EncryptionAlgorithm = EncryptionAlgorithm::RsaesOaepSha256;
    // SAFETY: NcryptKey is obtain from an NCRYPT_KEY_HANDLE, here we retrieve the underlying
    // handle.
    let raw_handle = unsafe { RawPointer::as_ptr(handle) } as NCRYPT_KEY_HANDLE;

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
    algo: EncryptionAlgorithm,
    encrypted_message: &[u8],
    certificate_ref: &X509CertificateReference,
) -> Result<DecryptedMessage, DecryptMessageError> {
    let store = open_store().map_err(DecryptMessageError::CannotOpenStore)?;
    let cert_context =
        find_certificate(&store, certificate_ref).ok_or(DecryptMessageError::NotFound)?;
    let reference = get_id_and_hash_from_cert_context(&cert_context)
        .map_err(DecryptMessageError::CannotGetCertificateInfo)?;
    let keypair = get_keypair(&cert_context)?;
    let data = match keypair {
        // We do not support a CryptoAPI provider as its API is marked for depreciation by windows.
        PrivateKey::CryptProv(..) => {
            todo!("Use CryptGetUserKey to get the keypair")
        }
        // Handle to a CryptoGraphy Next Generation (CNG) API
        PrivateKey::NcryptKey(handle) => {
            if algo != EncryptionAlgorithm::RsaesOaepSha256 {
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
    // SAFETY: NcryptKey is obtain from an NCRYPT_KEY_HANDLE, here we retrieve the underlying
    // handle.
    let raw_handle = unsafe { RawPointer::as_ptr(handle) } as NCRYPT_KEY_HANDLE;

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

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::{anyhow, Bytes, PKIEncryptionAlgorithm, PkiSignatureAlgorithm};
use schannel::cert_context::PrivateKey as SchannelPKey;
use sha2::Digest as _;
use windows_sys::Win32::Security::Cryptography::{
    self, NCryptDecrypt, NCryptSignHash, BCRYPT_OAEP_PADDING_INFO, BCRYPT_PSS_PADDING_INFO,
    NCRYPT_PAD_OAEP_FLAG, NCRYPT_PAD_PSS_FLAG, NCRYPT_SHA256_ALGORITHM,
};

pub struct X509PrivateKey(SchannelPKey);

impl From<SchannelPKey> for X509PrivateKey {
    fn from(value: SchannelPKey) -> Self {
        Self(value)
    }
}

impl X509PrivateKey {
    pub async fn sign(
        &self,
        message: &[u8],
    ) -> Result<(PkiSignatureAlgorithm, Bytes), crate::SignError> {
        match &self.0 {
            SchannelPKey::CryptProv(_crypt) => Err(crate::SignError::UnsupportedAlgorithm),
            SchannelPKey::NcryptKey(ncrypt) => sign_with_ncrypt(ncrypt, message),
        }
    }

    pub async fn decrypt(
        &self,
        algorithm: PKIEncryptionAlgorithm,
        ciphertext: &[u8],
    ) -> Result<Bytes, crate::DecryptError> {
        if algorithm != PKIEncryptionAlgorithm::RsaesOaepSha256 {
            return Err(crate::DecryptError::UnsupportedAlgorithm);
        }
        match &self.0 {
            SchannelPKey::CryptProv(_crypt) => Err(crate::DecryptError::Internal(anyhow::anyhow!(
                "Unsupported private key handle type"
            ))),
            SchannelPKey::NcryptKey(ncrypt) => decrypt_with_ncrypt(ncrypt, ciphertext),
        }
    }
}

fn sign_with_ncrypt(
    pkey: &schannel::ncrypt_key::NcryptKey,
    message: &[u8],
) -> Result<(PkiSignatureAlgorithm, Bytes), crate::SignError> {
    const ALGO: PkiSignatureAlgorithm = PkiSignatureAlgorithm::RsassaPssSha256;
    let hash = sha2::Sha256::digest(message);
    // SAFETY: We retrieve the inner NCRYPT_KEY_HANDLE wrapped by NcryptKey.
    let raw_handle =
        unsafe { schannel::RawPointer::as_ptr(pkey) as Cryptography::NCRYPT_KEY_HANDLE };

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
            return Err(crate::SignError::Sign(
                std::io::Error::from_raw_os_error(res).into(),
            ));
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
            return Err(crate::SignError::Sign(
                std::io::Error::from_raw_os_error(res).into(),
            ));
        }
        Ok((ALGO, buff.into()))
    }
}

fn decrypt_with_ncrypt(
    pkey: &schannel::ncrypt_key::NcryptKey,
    ciphertext: &[u8],
) -> Result<Bytes, crate::DecryptError> {
    // SAFETY: We retrieve the inner NCRYPT_KEY_HANDLE wrapped by NcryptKey.
    let raw_handle =
        unsafe { schannel::RawPointer::as_ptr(pkey) as Cryptography::NCRYPT_KEY_HANDLE };

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
            ciphertext.as_ptr(),
            ciphertext.len() as u32,
            padding_info_ptr,
            std::ptr::null_mut(),
            0,
            &mut len,
            flags,
        );
        if res != 0 {
            return Err(crate::DecryptError::Decrypt(
                std::io::Error::from_raw_os_error(res).into(),
            ));
        }

        // 2. Actually perform the decryption.
        let mut buff = vec![0_u8; len as usize];
        let res = NCryptDecrypt(
            raw_handle,
            ciphertext.as_ptr(),
            ciphertext.len() as u32,
            padding_info_ptr,
            buff.as_mut_ptr(),
            buff.len() as u32,
            &mut len,
            flags,
        );
        if res != 0 {
            return Err(crate::DecryptError::Decrypt(
                std::io::Error::from_raw_os_error(res).into(),
            ));
        }
        Ok(buff.into())
    }
}

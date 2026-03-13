// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashMap,
    env,
    num::ParseIntError,
    str::{FromStr, Utf8Error},
    string::FromUtf8Error,
};

use crate::{
    errors::{ListTrustedRootCertificatesError, ListUserCertificatesError},
    shared::{CertificateDetails, CertificateWithDetails, InvalidCertificateReason},
    DecryptMessageError, EncryptMessageError, GetDerEncodedCertificateError,
    ListIntermediateCertificatesError, ShowCertificateSelectionDialogError, SignMessageError,
};
use cryptoki::{
    context::Pkcs11,
    object::{Attribute, AttributeInfo, AttributeType, ObjectClass},
    session,
};
use libparsec_types::prelude::*;
use sha2::Digest;

pub async fn get_der_encoded_certificate(
    certificate_ref: &X509CertificateReference,
) -> Result<Bytes, GetDerEncodedCertificateError> {
    let _ = certificate_ref;
    unimplemented!("platform not supported")
}

pub async fn list_trusted_root_certificate_anchors(
) -> Result<Vec<rustls_pki_types::TrustAnchor<'static>>, ListTrustedRootCertificatesError> {
    unimplemented!("platform not supported")
}

pub async fn list_intermediate_certificates(
) -> Result<Vec<rustls_pki_types::CertificateDer<'static>>, ListIntermediateCertificatesError> {
    unimplemented!("platform not supported")
}

pub async fn sign_message(
    message: &[u8],
    certificate_ref: &X509CertificateReference,
) -> Result<(PkiSignatureAlgorithm, Bytes), SignMessageError> {
    let _ = message;
    let _ = certificate_ref;
    unimplemented!("platform not supported")
}

pub async fn encrypt_message(
    message: &[u8],
    certificate_ref: &X509CertificateReference,
) -> Result<(PKIEncryptionAlgorithm, Bytes), EncryptMessageError> {
    let _ = (message, certificate_ref);
    unimplemented!("platform not supported")
}

pub async fn decrypt_message(
    algo: PKIEncryptionAlgorithm,
    encrypted_message: &[u8],
    certificate_ref: &X509CertificateReference,
) -> Result<Bytes, DecryptMessageError> {
    let _ = (algo, encrypted_message, certificate_ref);
    unimplemented!("platform not supported")
}

pub fn show_certificate_selection_dialog_windows_only(
) -> Result<Option<X509CertificateReference>, ShowCertificateSelectionDialogError> {
    unimplemented!("platform not supported")
}

pub fn is_available() -> bool {
    false
}

// Get information about available attribute
const DESIRED_ATTRS: &[AttributeType] = &[
    // Key identifier for public/private key pair
    AttributeType::Id,
    // DER-encoding of the certificate serial number
    AttributeType::SerialNumber,
    // True if the key support encryption
    AttributeType::Encrypt,
    // True if the key support description
    AttributeType::Decrypt,
    // True if the key support signature
    AttributeType::Sign,
    // True if the key support verification
    AttributeType::Verify,
    // DER-encoding of the SubjectPublicKeyInfo for the public key contained in the
    // certificate.
    AttributeType::PublicKeyInfo,
    AttributeType::HashOfIssuerPublicKey,
    AttributeType::HashOfSubjectPublicKey,
    // Type of certificate
    AttributeType::CertificateType,
    // Type of key
    AttributeType::KeyType,
    // True if the object is private (value not available)
    AttributeType::Private,
    // BER-encoding of the certificate
    AttributeType::Value,
    // DER-encoding of the certificate issuer
    AttributeType::Issuer,
    // Description of the object
    AttributeType::Label,
    // Type of object
    AttributeType::Class,
    // DER-encoding of the certificate subject name
    AttributeType::Subject,
];

pub async fn list_user_certificates_with_details(
) -> Result<Vec<CertificateWithDetails>, ListUserCertificatesError> {
    let pkcs11 = Pkcs11::new(
        env::var("TEST_PKCS11_MODULE")
            .unwrap_or_else(|_| "/usr/local/lib/softhsm/libsofthsm2.so".to_string()),
    )?;
    pkcs11.initialize(cryptoki::context::CInitializeArgs::new(
        cryptoki::context::CInitializeFlags::OS_LOCKING_OK,
    ))?;

    let slots = pkcs11.get_slots_with_token()?;

    let mut res = Vec::with_capacity(slots.len());

    'slot_iterator: for slot in slots.into_iter() {
        let session = pkcs11.open_ro_session(slot)?;
        let slot_info = pkcs11.get_slot_info(slot)?; // filter on key capabilities ?
        'handle_iterator: for handle in session.iter_objects(&[])? {
            let handle = handle?;
            let attr_info = session.get_attribute_info(handle, DESIRED_ATTRS)?;
            let attr = session.get_attributes(handle, DESIRED_ATTRS)?;
            let attr_hash_map: HashMap<_, _> = DESIRED_ATTRS
                .iter()
                .zip(attr.iter().zip(attr_info.iter()))
                .collect();
            // this is in a block because after getting the handle and the label
            // we want to add a invalid certificate details, not ignore nor return
            let cert_details = 'details: {
                // make sure we're looking at a certificate
                let class = match attr_hash_map.get(&AttributeType::Class) {
                    Some((Attribute::Class(class), _)) if *class == ObjectClass::CERTIFICATE => {}
                    _ => continue 'handle_iterator,
                };
                let cert_handle = X509CertificateHash::fake_sha256(); // TODO

                let friendly_name = match attr_hash_map.get(&AttributeType::Label) {
                    Some((Attribute::Label(label), _)) => {
                        Some(String::from_utf8(label.clone()).expect("todo"))
                    }
                    Some(_) => unreachable!(),
                    None => None,
                };

                let value = match attr_hash_map.get(&AttributeType::Value) {
                    Some((Attribute::Value(value), _)) => value,
                    _ => {
                        break 'details CertificateWithDetails::Invalid {
                            handle: cert_handle.into(),
                            friendly_name,
                            invalid_reason: InvalidCertificateReason::UnableToGetAttribute(
                                "value".to_string(),
                            ),
                        };
                    }
                };

                // parsing cert value if possible
                let digest = sha2::Sha256::digest(value);
                let hash = X509CertificateHash::SHA256(Box::new(digest.into()));

                let cert_handle: X509CertificateReference = hash.into();

                match get_cert_details(attr_hash_map) {
                    Ok(details) => CertificateWithDetails::Valid {
                        handle: cert_handle,
                        friendly_name,
                        details,
                    },
                    Err(invalid_reason) => CertificateWithDetails::Invalid {
                        handle: cert_handle,
                        friendly_name,
                        invalid_reason,
                    },
                }
            };

            res.push(cert_details);
        }
    }
    Ok(res)
}

fn get_cert_details(
    attr_hash_map: HashMap<&AttributeType, (&Attribute, &AttributeInfo)>,
) -> Result<CertificateDetails, InvalidCertificateReason> {
    let name = todo!();

    let subject = todo!();
    let issuer = todo!();
    let not_before = match attr_hash_map.get(&AttributeType::StartDate) {
        Some((Attribute::StartDate(value), _)) => cryptoki_date_to_datetime(value)
            .map_err(|_| InvalidCertificateReason::UnableToParseTime)?,
        _ => {
            return Err(InvalidCertificateReason::UnableToGetAttribute(
                "start date".to_string(),
            ))
        }
    };
    let not_after = match attr_hash_map.get(&AttributeType::EndDate) {
        Some((Attribute::EndDate(value), _)) => cryptoki_date_to_datetime(value)
            .map_err(|_| InvalidCertificateReason::UnableToParseTime)?,
        _ => {
            return Err(InvalidCertificateReason::UnableToGetAttribute(
                "end date".to_string(),
            ))
        }
    };
    let serial = todo!();
    let emails = todo!();
    let can_sign = todo!();
    let can_encrypt = todo!();
    Ok(CertificateDetails {
        name,
        subject,
        issuer,
        not_before,
        not_after,
        serial,
        emails,
        can_sign,
        can_encrypt,
    })
}

enum TimeConversionError {}

impl From<FromUtf8Error> for TimeConversionError {
    fn from(value: FromUtf8Error) -> Self {
        todo!()
    }
}

impl From<ParseIntError> for TimeConversionError {
    fn from(value: ParseIntError) -> Self {
        todo!()
    }
}

impl From<DatetimeFromTimestampMicrosError> for TimeConversionError {
    fn from(value: DatetimeFromTimestampMicrosError) -> Self {
        todo!()
    }
}

fn cryptoki_date_to_datetime(
    date: &cryptoki::types::Date,
) -> Result<DateTime, TimeConversionError> {
    let year = String::from_utf8(date.year.to_vec())?.parse::<i32>()?;
    let month = String::from_utf8(date.month.to_vec())?.parse::<u32>()?;
    let day = String::from_utf8(date.day.to_vec())?.parse::<u32>()?;

    Ok(DateTime::from_ymd_hms_us(year, month, day, 0, 0, 0, 0)?)
}

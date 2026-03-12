// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod subject_alt_names;

use x509_cert::{
    der::{
        flagset::FlagSet,
        oid::db::rfc5280::{ID_CE_EXT_KEY_USAGE, ID_CE_KEY_USAGE, ID_CE_SUBJECT_ALT_NAME},
        Decode, Error as DERError, SliceReader,
    },
    ext::{self, pkix::KeyUsage},
};

use subject_alt_names::parse_san_octet_string;
pub use subject_alt_names::SubjectAltName;

#[derive(Debug, Clone)]
pub struct Extensions {
    pub subject_alt_names: Vec<SubjectAltName>,
    pub key_usage: KeyUsage,
}

impl Default for Extensions {
    fn default() -> Self {
        Self {
            subject_alt_names: Default::default(),
            key_usage: FlagSet::empty().into(),
        }
    }
}

impl Extensions {
    /// if key usage is not specified, returns true
    pub fn can_encrypt(&self) -> bool {
        self.key_usage.0.is_empty() || self.key_usage.data_encipherment()
    }

    /// if key usage is not specified, returns true
    pub fn can_sign(&self) -> bool {
        self.key_usage.0.is_empty() || self.key_usage.digital_signature()
    }
}

impl TryFrom<Vec<ext::Extension>> for Extensions {
    type Error = DERError;

    fn try_from(value: Vec<ext::Extension>) -> Result<Self, Self::Error> {
        let mut extensions = Self::default();
        value
            .into_iter()
            .try_for_each(|ext| -> Result<(), DERError> {
                log::trace!(
                    "Certificate extensions: id={}, value={:?}, critical={}",
                    ext.extn_id,
                    ext.extn_value,
                    ext.critical
                );
                match ext.extn_id {
                    // Certificate alternative names
                    // https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.6
                    ID_CE_SUBJECT_ALT_NAME => {
                        extensions.subject_alt_names =
                            parse_san_octet_string(ext.extn_value.as_bytes())?;
                    }
                    // Certificate key usage
                    // https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.3
                    ID_CE_KEY_USAGE => {
                        let mut reader = SliceReader::new(ext.extn_value.as_bytes())?;
                        extensions.key_usage = KeyUsage::decode(&mut reader)?;
                    }
                    // Certificate Additional key usage
                    // https://datatracker.ietf.org/doc/html/rfc5280#section-4.2.1.12
                    ID_CE_EXT_KEY_USAGE => {}
                    _ => {}
                }
                Ok(())
            })?;
        Ok(extensions)
    }
}

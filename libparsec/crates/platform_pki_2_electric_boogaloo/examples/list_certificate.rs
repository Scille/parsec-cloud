// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#[cfg(target_os = "linux")]
mod unix_only {
    use core::panic;
    use std::{
        collections::HashMap,
        fmt::{Debug, Display},
        path::PathBuf,
    };

    use anyhow::Context;
    use clap::Parser;
    use cryptoki::{
        context::Pkcs11,
        object::{Attribute, AttributeInfo, AttributeType, ObjectClass},
    };
    use libparsec_types::X509CertificateHash;
    use percent_encoding::{percent_encode, utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
    use sha2::Digest;

    /// The unreserved charset from the URI [RFC-3986] (meaning available chars to the user), the ABNF notation is like so:
    /// ```ABNF
    /// unreserved           = ALPHA / DIGIT / "-" / "." / "_" / "~"
    /// ```
    const UNRESERVED: AsciiSet = NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'.')
        .remove(b'_')
        .remove(b'~')
        .complement();
    /// On top of [`UNRESERVED`], pkcs11 allow additional chars to be used:
    ///
    /// ```ABNF
    /// pk11-res-avail       = ":" / "[" / "]" / "@" / "!" / "$" /
    ///                        "'" / "(" / ")" / "*" / "+" / "," / "="
    /// ```
    const PK11_RES_AVAIL: AsciiSet = AsciiSet::EMPTY
        .add(b':')
        .add(b'[')
        .add(b']')
        .add(b'@')
        .add(b'!')
        .add(b'$')
        .add(b'\'')
        .add(b'(')
        .add(b')')
        .add(b'*')
        .add(b'+')
        .add(b',')
        .add(b'=');
    /// Specific to path value, pkcs11 allow on top of [`PK11_RES_AVAIL`] using the `&` char:
    /// ```ABNF
    /// pk11-path-res-avail  = pk11-res-avail / "&"
    /// ```
    const PK11_PATH_RES_AVAIL: AsciiSet = PK11_RES_AVAIL.add(b'&');
    /// Ascii set to percent-encode value for the `pkcs11` URI.
    ///
    /// The set is composed of `unreserved` (from URI [RFC-3986]) and _pk11-path-res-avail_ chars (we
    /// do not list `percent-encode` charset as it's implicitly included)
    const PKCS11_PATH_FRAGMENT: &AsciiSet = &UNRESERVED.union(PK11_PATH_RES_AVAIL).complement();
    // // pk11-query-res-avail = pk11-res-avail / "/" / "?" / "|"
    // const PK11_QUERY_RES_AVAIL: AsciiSet = PK11_RES_AVAIL.remove(b'/').remove(b'?').remove(b'|');
    // // pk11-qchar           = unreserved / pk11-query-res-avail / pct-encoded
    // const PKCS11_QUERY_FRAGMENT: &AsciiSet = &UNRESERVED.union(PK11_QUERY_RES_AVAIL);
    /// Id is a special case where it should be entirely percent-encoded
    const PKCS11_ID_FRAGMENT: &AsciiSet = &AsciiSet::EMPTY.complement();

    #[derive(Debug, Parser)]
    struct Args {
        /// Path to a dynamic library providing pkcs11 interface
        #[clap(long, env = "TEST_PKCS11_MODULE")]
        module: PathBuf,
        /// User pin to unlock the smartcard, if not provided not all information could be retrieved
        /// from it.
        #[clap(env = "SLOT_USER_PIN")]
        user_pin: Option<String>,
    }

    pub fn main() -> anyhow::Result<()> {
        let args = Args::parse();
        println!("args={args:?}");
        let pkcs11 = Pkcs11::new(args.module)?;
        pkcs11.initialize(cryptoki::context::CInitializeArgs::new(
            cryptoki::context::CInitializeFlags::OS_LOCKING_OK,
        ))?;

        let slots = pkcs11
            .get_slots_with_token()
            .context("Failed to get slots")?;

        slots
        .iter()
        .enumerate()
        .try_for_each(|(n, slot)| -> anyhow::Result<()> {
            println!("Slot #{n}: {}", slot.id());
            let slot_info = pkcs11.get_slot_info(*slot).context("Get slot info")?;
            println!("  description: {}", slot_info.slot_description());
            let token_info = if slot_info.token_present() {
                let token_info = pkcs11.get_token_info(*slot).context("Get token info")?;
                println!("  label: {}", token_info.label());
                println!("  token manufacturer: {}", token_info.manufacturer_id());
                println!("  slot manufacturer: {}", slot_info.manufacturer_id());
                println!("  model: {}", token_info.model());
                println!("  serial: {}", token_info.serial_number());
                if !token_info.token_initialized() {
                    return Ok(());
                }
                token_info
            } else {
                println!("  No token present!");
                return Ok(());
            };
            let session = pkcs11
                .open_ro_session(*slot)
                .context("Cannot open session")?;

            // Not required to login, but that allow access to more sensible items (like private
            // key).
            if let Some(user_pin) = &args.user_pin {
                session.login(cryptoki::session::UserType::User, Some(&user_pin.as_str().into())).context("Cannot login")?;
            }

            for (j, res) in session.iter_objects(&[])?.enumerate() {
                use cryptoki::object::AttributeType::*;
                let handle = res?;
                println!("  - handle #{j}");
                // Get information about available attribute
                const DESIRED_ATTRS: &[AttributeType] = &[
                    // Key identifier for public/private key pair
                    Id,
                    // DER-encoding of the certificate serial number
                    SerialNumber,
                    // True if the key support encryption
                    Encrypt,
                    // True if the key support description
                    Decrypt,
                    // True if the key support signature
                    Sign,
                    // True if the key support verification
                    Verify,
                    // DER-encoding of the SubjectPublicKeyInfo for the public key contained in the
                    // certificate.
                    PublicKeyInfo,
                    HashOfIssuerPublicKey,
                    HashOfSubjectPublicKey,
                    // Type of certificate
                    CertificateType,
                    // Type of key
                    KeyType,
                    // True if the object is private (value not available)
                    Private,
                    // BER-encoding of the certificate
                    Value,
                    // DER-encoding of the certificate issuer
                    Issuer,
                    // Description of the object
                    Label,
                    // // DER-encoding of the certificate owner
                    // Owner,
                    // // Optional value where complete certificate can be retrieved
                    // Url,
                    // Type of object
                    Class,
                    // // DER-encoding of the object identifier
                    // ObjectId,
                    // DER-encoding of the certificate subject name
                    Subject,
                ];
                let attrs_availability = session.get_attribute_info(handle, DESIRED_ATTRS)?;
                let available_attrs = DESIRED_ATTRS
                    .iter()
                    .zip(attrs_availability)
                    .filter_map(|(ty, status)| {
                        if matches!(status, AttributeInfo::Available(_)) {
                            Some(*ty)
                        } else {
                            println!("    {ty}: Not available ({status:?})");
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                let attrs = session.get_attributes(handle, &available_attrs)?;

                let attrs = available_attrs
                    .iter()
                    .zip(attrs)
                    .inspect(|(ty, value)| println!("    {ty}: {:x}", AttributeDisplay(value)))
                    .collect::<HashMap<_, _>>();

                let Attribute::Id(id)= &attrs[&Id] else {
                    panic!("Invalid attribute `id`");
                };
                let Attribute::Class(class) = &attrs[&Class] else {
                    panic!("Invalid attribute `class`");
                };
                // Example to generate a pkcs11, we could add more attribute if needed.
                let pkcs11_uri = format!(
                    "pkcs11:model={model};manufacturer={manufacturer};serial={token_serial};token={token};id={id};type={class}",
                    model = utf8_percent_encode(token_info.model(), PKCS11_PATH_FRAGMENT),
                    manufacturer = utf8_percent_encode(token_info.manufacturer_id(), PKCS11_PATH_FRAGMENT),
                    token = utf8_percent_encode(token_info.label(), PKCS11_PATH_FRAGMENT),
                    token_serial = utf8_percent_encode(token_info.serial_number(),PKCS11_PATH_FRAGMENT),
                    id = percent_encode(id, PKCS11_ID_FRAGMENT),
                    class = normalized_type(class),
                );

                println!("    URI: {pkcs11_uri}");
                if *class == ObjectClass::CERTIFICATE {
                    let Attribute::Value(value) = &attrs[&Value] else {
                        panic!("Invalid attribute `value`");
                    };
                    let digest = sha2::Sha256::digest(value);
                    let hash = X509CertificateHash::SHA256( Box::new(digest.into()) );
                    println!("    fingerprint-sha256: {hash}")
                }
            }

            Ok(())
        })?;

        Ok(())
    }

    fn normalized_type(class: &ObjectClass) -> &str {
        match *class {
            ObjectClass::CERTIFICATE => "cert",
            ObjectClass::DATA => "data",
            ObjectClass::PRIVATE_KEY => "private",
            ObjectClass::PUBLIC_KEY => "public",
            ObjectClass::SECRET_KEY => "secret-key",
            _ => unreachable!("RFC does not define other mapping"),
        }
    }

    struct AttributeDisplay<'a>(&'a Attribute);

    impl<'a> std::fmt::Display for AttributeDisplay<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use Attribute::*;
            match self.0 {
                // raw string
                Label(raw) => str::from_utf8(raw)
                    .map(|s| f.write_str(s))
                    .unwrap_or_else(|_| Debug::fmt(raw, f)),
                // bool
                Private(b) | Sign(b) | Verify(b) | Encrypt(b) | Decrypt(b) => Display::fmt(b, f),
                // base64 formatting
                SerialNumber(raw)
                | Id(raw)
                | Value(raw)
                | Subject(raw)
                | Owner(raw)
                | Issuer(raw)
                | PublicKeyInfo(raw)
                | HashOfSubjectPublicKey(raw)
                | HashOfIssuerPublicKey(raw) => {
                    if raw.is_empty() {
                        f.write_str("<empty>")
                    } else {
                        data_encoding::BASE64.encode_write(raw.as_ref(), f)
                    }
                }
                CertificateType(cert_type) => Display::fmt(cert_type, f),
                KeyType(key_type) => Display::fmt(key_type, f),
                // Type
                Class(class) => f.write_str(normalized_type(class)),
                v => write!(f, "{v:x?}"),
            }
        }
    }

    impl<'a> std::fmt::LowerHex for AttributeDisplay<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Display::fmt(self, f)
        }
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    #[cfg(target_os = "linux")]
    unix_only::main()?;
    Ok(())
}

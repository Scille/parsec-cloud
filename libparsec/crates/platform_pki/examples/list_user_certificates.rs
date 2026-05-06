// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]
mod utils;

use anyhow::Context;
use libparsec_platform_pki::{AvailablePkiCertificate, PkiSystem};
use x509_cert::der::{Decode, Encode};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Config dir is only used for testbed, we can safely provide an empty path here
    let pki = PkiSystem::init(std::path::Path::new(""), None)
        .await
        .context("Failed to initialize PKI system")?;

    let certs = pki
        .list_user_certificates()
        .await
        .context("Failed to list user certificates")?;

    println!("Found {} user certificates", certs.len());

    for (i, cert) in certs.iter().enumerate() {
        match cert {
            AvailablePkiCertificate::Valid {
                reference,
                friendly_name,
                ..
            } => {
                println!(
                    "Certificate #{i} (AKA {friendly_name}) fingerprint={}",
                    reference.hash
                );
                let cert = pki
                    .open_certificate(reference)
                    .await
                    .context("Failed to load certificate")?;

                let der = cert.get_der().await.context("Failed to get DER")?;
                println!(
                    "  content: {}",
                    data_encoding::BASE64.encode_display(der.as_ref())
                );
                println!("  reference: {}", utils::EncodedCertRef(reference.clone()));
                let (issuer, serial) = cert.get_issuer_and_serial().await;
                let decoded_cert = x509_cert::der::SliceReader::new(&der)
                    .and_then(|mut r| x509_cert::Certificate::decode(&mut r))
                    .expect("Cannot decode certificate DER");
                let serial = x509_cert::serial_number::SerialNumber::<
                    x509_cert::certificate::Rfc5280,
                >::new(&serial)
                .unwrap();
                println!(
                    "  issuer from cert info: {}",
                    data_encoding::BASE64.encode_display(&issuer)
                );
                println!(
                    "  issuer from cert der : {}",
                    data_encoding::BASE64.encode_display(
                        &decoded_cert
                            .tbs_certificate
                            .issuer
                            .to_der()
                            .unwrap_or_default()
                    )
                );
                println!(
                    "  serial from cert info: {}",
                    data_encoding::BASE64.encode_display(serial.as_bytes())
                );
                println!(
                    "  serial from cert der : {}",
                    data_encoding::BASE64
                        .encode_display(decoded_cert.tbs_certificate.serial_number.as_bytes())
                );
            }

            AvailablePkiCertificate::Invalid {
                reference,
                invalid_reason,
            } => {
                println!(
                    "Certificate #{i} fingerprint={} INVALID (reason: {})",
                    reference.hash, invalid_reason
                );
            }
        }
        println!();
    }
    Ok(())
}

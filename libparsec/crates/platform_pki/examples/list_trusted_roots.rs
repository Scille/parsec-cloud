// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use anyhow::Context;
use libparsec_platform_pki::{AvailablePkiCertificate, PkiSystem};

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
            AvailablePkiCertificate::Valid { reference, .. } => {
                println!("Certificate #{i} fingerprint={}", reference.hash);
                let cert = pki
                    .open_certificate(reference)
                    .await
                    .context("Failed to load certificate")?
                    .context("Certificate not found")?;
                let der = cert.get_der().await.context("Failed to get DER")?;
                println!(
                    "  content: {}",
                    data_encoding::BASE64.encode_display(der.as_ref())
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

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use anyhow::Context;
use libparsec_platform_pki::PkiSystem;

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
        let cert_ref = cert
            .to_reference()
            .await
            .context("Failed to get reference")?;
        let der = cert.get_der().await.context("Failed to get DER")?;
        println!("Certificate #{i} fingerprint={}", cert_ref.hash);
        println!(
            "  content: {}",
            data_encoding::BASE64.encode_display(der.as_ref())
        );
        println!();
    }
    Ok(())
}

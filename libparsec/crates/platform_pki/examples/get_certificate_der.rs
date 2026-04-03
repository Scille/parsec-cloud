// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

mod utils;

use std::fmt::Debug;

use anyhow::Context;
use clap::Parser;
use libparsec_platform_pki::PkiSystem;
use libparsec_types::X509CertificateHash;
use sha2::Digest;

#[derive(Debug, Parser)]
struct Args {
    /// Hash of the certificate to get content of.
    /// If not provided, lists all user certificates and shows the first one.
    #[arg(value_parser = utils::CertificateSRIHashParser)]
    certificate_hash: Option<X509CertificateHash>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();
    log::debug!("args={args:x?}");

    // Config dir is only used for testbed, we can safely provide an empty path here
    let pki = PkiSystem::init(std::path::Path::new(""), None)
        .await
        .context("Failed to initialize PKI system")?;

    let cert = match args.certificate_hash {
        Some(hash) => {
            let cert_ref = hash.into();
            pki.find_certificate(&cert_ref)
                .await
                .context("Failed to find certificate")?
                .context("Certificate not found")?
        }
        None => {
            let certs = pki
                .list_user_certificates()
                .await
                .context("Failed to list user certificates")?;
            if certs.is_empty() {
                return Err(anyhow::anyhow!("No user certificates found"));
            }
            println!("Found {} user certificate(s)", certs.len());
            certs.into_iter().next().unwrap()
        }
    };

    let cert_ref = cert
        .to_reference()
        .await
        .context("Failed to get certificate reference")?;
    println!("fingerprint: {}", cert_ref.hash);

    let certificate = cert.get_der().await.context("Get certificate")?;
    let der_bytes = certificate.as_ref();

    {
        let digest = sha2::Sha256::digest(der_bytes);
        let hash = X509CertificateHash::SHA256(Box::new(digest.into()));
        println!("Manually calculated fingerprint: {hash}");
    }
    println!(
        "content: {}",
        data_encoding::BASE64.encode_display(der_bytes)
    );

    Ok(())
}

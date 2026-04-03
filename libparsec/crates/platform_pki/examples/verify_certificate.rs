// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

mod utils;

use anyhow::Context;
use clap::Parser;
use libparsec_platform_pki::PkiSystem;

#[derive(Debug, Parser)]
struct Args {
    /// Hash of the certificate to verify.
    #[arg(value_parser = utils::CertificateSRIHashParser)]
    certificate_hash: libparsec_types::X509CertificateHash,
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();
    log::debug!("args={args:?}");

    // Config dir is only used for testbed, we can safely provide an empty path here
    let pki = PkiSystem::init(std::path::Path::new(""), None)
        .await
        .context("Failed to initialize PKI system")?;

    let cert_ref: libparsec_types::X509CertificateReference = args.certificate_hash.into();
    let cert = pki
        .find_certificate(&cert_ref)
        .await
        .context("Failed to find certificate")?
        .context("Certificate not found")?;

    let path = cert
        .get_validation_path()
        .await
        .context("Cannot trust certificate")?;

    println!("Certificate is trusted!");
    println!(
        "  root: subject={}",
        utils::display_x509_raw_name(&path.root.subject)
    );
    if !path.intermediates.is_empty() {
        println!("  intermediate certificates:");
        for (i, cert_der) in path.intermediates.iter().enumerate() {
            println!("  {:02}. (DER {} bytes)", i + 1, cert_der.as_ref().len());
        }
    }
    println!("  leaf: (DER {} bytes)", path.leaf.as_ref().len());

    Ok(())
}

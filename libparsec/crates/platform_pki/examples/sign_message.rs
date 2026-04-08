// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

mod utils;

use anyhow::Context;
use clap::Parser;
use libparsec_platform_pki::PkiSystem;
use libparsec_types::X509CertificateHash;

#[derive(Debug, Parser)]
struct Args {
    /// Hash of the certificate to use for the signature.
    #[arg(value_parser = utils::CertificateSRIHashParser)]
    certificate_hash: X509CertificateHash,
    #[command(flatten)]
    content: utils::ContentOpts,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();
    log::debug!("args={args:?}");

    let cert_ref = args.certificate_hash.into();
    let data = args.content.into_bytes()?;

    // Config dir is only used for testbed, we can safely provide an empty path here
    let pki = PkiSystem::init(std::path::Path::new(""), None)
        .await
        .context("Failed to initialize PKI system")?;
    let cert = pki
        .open_certificate(&cert_ref)
        .await
        .context("Failed to find certificate")?
        .context("Certificate not found")?;
    let key = cert
        .request_private_key()
        .await
        .context("Failed to get private key")?;
    let (algo, signature) = key.sign(&data).await.context("Failed to sign message")?;

    println!("Signed with algorithm {}", algo);
    println!("Signed by cert with fingerprint: {}", cert_ref.hash);
    println!(
        "Signature: {}",
        data_encoding::BASE64.encode_display(&signature)
    );

    Ok(())
}

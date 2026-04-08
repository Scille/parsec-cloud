// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

mod utils;

use anyhow::Context;
use bytes::Bytes;
use clap::Parser;
use libparsec_platform_pki::{verify_message, PkiSystem};
use libparsec_types::{DateTime, PkiSignatureAlgorithm, X509CertificateHash};

#[derive(Debug, Parser)]
struct Args {
    /// Hash of the certificate that signed the message.
    #[arg(value_parser = utils::CertificateSRIHashParser)]
    certificate_hash: X509CertificateHash,
    #[command(flatten)]
    content: utils::ContentOpts,
    /// The signature algorithm used.
    signature_header: PkiSignatureAlgorithm,
    /// The signature in base64 format.
    #[arg(value_parser = utils::Base64Parser)]
    signature: Bytes,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();
    log::debug!("args={args:?}");

    let data = args.content.into_bytes()?;

    // Config dir is only used for testbed, we can safely provide an empty path here
    let pki = PkiSystem::init(std::path::Path::new(""), None)
        .await
        .context("Failed to initialize PKI system")?;

    let cert_ref = args.certificate_hash.into();
    let cert = pki
        .open_certificate(&cert_ref)
        .await
        .context("Failed to find certificate")?
        .context("Certificate not found")?;

    let path = cert
        .get_validation_path()
        .await
        .context("Failed to get validation path")?;

    match verify_message(
        &data,
        &args.signature,
        args.signature_header,
        path.leaf.as_ref(),
        path.intermediates.iter().map(|c| c.as_ref()),
        &[path.root],
        DateTime::now(),
    ) {
        Ok(_) => println!("The message has a correct signature"),
        Err(e) => println!("The message has an incorrect signature: {e}"),
    }

    Ok(())
}

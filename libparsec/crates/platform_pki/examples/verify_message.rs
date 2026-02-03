// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use anyhow::Context;
use bytes::Bytes;
use clap::Parser;
use libparsec_platform_pki::{verify_message, SignedMessage};
use libparsec_types::{PkiSignatureAlgorithm, X509CertificateHash};
use sha2::Digest;

mod utils;

#[derive(Debug, Parser)]
struct Args {
    #[command(flatten)]
    cert: utils::CertificateOrRef,
    #[command(flatten)]
    content: utils::ContentOpts,
    signature_header: PkiSignatureAlgorithm,
    #[arg(value_parser = utils::Base64Parser)]
    signature: Bytes,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();
    log::debug!("args={args:?}");

    let data = args.content.into_bytes()?;

    let cert = args.cert.get_certificate().await?;

    {
        let fingerprint =
            X509CertificateHash::SHA256(Box::new(sha2::Sha256::digest(cert.as_ref()).into()));
        println!("Certificate fingerprint: {fingerprint}");
    }

    let signed_message = SignedMessage {
        algo: args.signature_header,
        signature: args.signature,
        message: data,
    };

    match verify_message(
        &signed_message,
        &cert.to_end_certificate().context("Invalid certificate")?,
    ) {
        Ok(_) => {
            println!("The message as a correct signature")
        }
        Err(e) => println!("The message as an incorrect signature: {e}"),
    }

    Ok(())
}

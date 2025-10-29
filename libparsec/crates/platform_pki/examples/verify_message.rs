// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use anyhow::Context;
use clap::Parser;
use libparsec_platform_pki::{verify_message, SignatureAlgorithm, SignedMessage};
use libparsec_types::X509CertificateHash;
use sha2::Digest;

mod utils;

#[derive(Debug, Parser)]
struct Args {
    #[command(flatten)]
    cert: utils::CertificateOrRef,
    #[command(flatten)]
    content: utils::ContentOpts,
    signature_header: SignatureAlgorithm,
    /// Signature in base64
    signature: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("args={args:?}");

    let data = args.content.into_bytes()?;

    let signature = data_encoding::BASE64
        .decode(args.signature.as_bytes())
        .context("Invalid signature format")?;

    let cert = args.cert.get_certificate()?;

    {
        let fingerprint =
            X509CertificateHash::SHA256(Box::new(sha2::Sha256::digest(cert.as_ref()).into()));
        println!("Certificate fingerprint: {fingerprint}");
    }

    let signed_message = SignedMessage {
        algo: args.signature_header,
        signature,
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

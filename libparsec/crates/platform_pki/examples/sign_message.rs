// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

mod utils;

use anyhow::Context;
use clap::Parser;
use libparsec_platform_pki::sign_message;
use libparsec_types::X509CertificateHash;

#[derive(Debug, Parser)]
struct Args {
    /// Hash of the certificate to use for the signature.
    #[arg(value_parser = utils::CertificateSRIHashParser)]
    certificate_hash: X509CertificateHash,
    #[command(flatten)]
    content: utils::ContentOpts,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("args={args:?}");

    let cert_ref = args.certificate_hash.into();
    let data = args.content.into_bytes()?;
    let res = sign_message(&data, &cert_ref).context("Failed to sign message")?;

    println!(
        "Signed by cert with id {{{}}} with algorithm {}",
        &res.cert_ref.uris().next().unwrap(),
        res.algo
    );
    println!("Signed by cert with fingerprint: {}", res.cert_ref.hash);
    println!(
        "Signature: {}",
        data_encoding::BASE64.encode_display(&res.signature)
    );

    Ok(())
}

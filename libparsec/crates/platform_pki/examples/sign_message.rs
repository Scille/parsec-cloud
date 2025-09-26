// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod utils;

use anyhow::Context;
use clap::Parser;
use libparsec_platform_pki::{sign_message, CertificateHash, CertificateReference};

#[derive(Debug, Parser)]
struct Args {
    /// Hash of the certificate to use for the signature.
    #[arg(value_parser = utils::CertificateSRIHashParser)]
    certificate_hash: CertificateHash,
    #[command(flatten)]
    content: utils::ContentOpts,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("args={args:?}");

    let cert_ref = CertificateReference::Hash(args.certificate_hash);
    let data: Vec<u8> = args.content.into_bytes()?;
    let res = sign_message(&data, &cert_ref).context("Failed to sign message")?;

    println!(
        "Signed by cert with id {{{}}} with algorithm {}",
        data_encoding::BASE64.encode_display(&res.cert_ref.id),
        res.algo
    );
    #[cfg(feature = "hash-sri-display")]
    println!("Signed by cert with fingerprint: {}", res.cert_ref.hash);
    println!(
        "Signature: {}",
        data_encoding::BASE64.encode_display(&res.signature)
    );

    Ok(())
}

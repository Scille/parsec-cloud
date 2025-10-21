// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

mod utils;

use anyhow::Context;
use clap::Parser;
use libparsec_platform_pki::encrypt_message;
use libparsec_types::X509CertificateHash;

#[derive(Debug, Parser)]
struct Args {
    /// Hash of the certificate to use for the encryption.
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

    let res = encrypt_message(&data, &cert_ref).context("Failed to encrypt message")?;

    println!(
        "Encrypted by cert with id {{{}}} using the algorithm {}",
        data_encoding::BASE64.encode_display(res.cert_ref.uris().next().unwrap()),
        res.algo
    );
    println!("Encrypted by cert with fingerprint: {}", res.cert_ref.hash);
    println!(
        "Encrypted data: {}",
        data_encoding::BASE64.encode_display(&res.ciphered)
    );

    Ok(())
}

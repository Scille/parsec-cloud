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
    env_logger::init();
    let args = Args::parse();
    log::debug!("args={args:?}");

    let cert_ref = args.certificate_hash.into();
    let data = args.content.into_bytes()?;

    let (algo, ciphered) =
        encrypt_message(&data, &cert_ref).context("Failed to encrypt message")?;

    println!("Encrypted using the algorithm {}", algo);
    println!("Encrypted by cert with fingerprint: {}", cert_ref.hash);
    println!(
        "Encrypted data: {}",
        data_encoding::BASE64.encode_display(&ciphered)
    );

    Ok(())
}

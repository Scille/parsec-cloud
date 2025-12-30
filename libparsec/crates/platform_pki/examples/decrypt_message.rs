// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

mod utils;

use anyhow::Context;
use clap::Parser;
use libparsec_platform_pki::decrypt_message;
use libparsec_types::{PKIEncryptionAlgorithm, X509CertificateHash};

#[derive(Debug, Parser)]
struct Args {
    /// Hash of the certificate to use for the encryption.
    #[arg(value_parser = utils::CertificateSRIHashParser)]
    certificate_hash: X509CertificateHash,
    #[command(flatten)]
    content: utils::ContentOpts,
    /// The algorithm used to encrypt the content.
    #[arg(long, default_value_t = PKIEncryptionAlgorithm::RsaesOaepSha256)]
    algorithm: PKIEncryptionAlgorithm,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();
    log::debug!("args={args:?}");

    let cert_ref = args.certificate_hash.into();
    let b64_data = args.content.into_bytes()?;
    let data = data_encoding::BASE64
        .decode(&b64_data)
        .context("Failed to decode hex encoded data")?;

    let res =
        decrypt_message(args.algorithm, &data, &cert_ref).context("Failed to decrypt message")?;

    println!(
        "Decrypted by cert with id {{{}}} with algo {}",
        &res.cert_ref.uris().next().unwrap(),
        args.algorithm
    );
    println!("Decrypted by cert with fingerprint: {}", res.cert_ref.hash);
    println!(
        "Decrypted data: {}",
        data_encoding::BASE64.encode_display(&res.data)
    );

    Ok(())
}

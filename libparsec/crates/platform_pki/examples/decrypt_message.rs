// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod utils;
use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use libparsec_platform_pki::{decrypt_message, CertificateHash, CertificateReference};

#[derive(Debug, Parser)]
struct Args {
    /// Hash of the certificate to use for the encryption.
    #[arg(value_parser = utils::CertificateSRIHashParser)]
    certificate_hash: CertificateHash,
    #[command(flatten)]
    content: ContentOpts,
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = true, multiple = false)]
struct ContentOpts {
    #[arg(long, conflicts_with = "content_file")]
    content: Option<String>,
    #[arg(long)]
    content_file: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("args={args:?}");

    let cert_ref = CertificateReference::Hash(args.certificate_hash);
    let b64_data: Vec<u8> = match (args.content.content, args.content.content_file) {
        (Some(content), None) => content.into(),
        (None, Some(filepath)) => std::fs::read(filepath).context("Failed to read file")?,
        (Some(_), Some(_)) | (None, None) => unreachable!("Handled by clap"),
    };
    let data = data_encoding::BASE64
        .decode(&b64_data)
        .context("Failed to decode hex encoded data")?;

    let res = decrypt_message(&data, &cert_ref).context("Failed to decrypt message")?;

    println!(
        "Decrypted by cert with id: {}",
        data_encoding::BASE64.encode_display(&res.cert_ref.id)
    );
    #[cfg(feature = "hash-sri-display")]
    println!("Decrypted by cert with fingerprint: {}", res.cert_ref.hash);
    println!(
        "Decrypted data: {}",
        data_encoding::BASE64.encode_display(&res.data)
    );

    Ok(())
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use libparsec_platform_pki::{
    get_der_encoded_certificate, verify_message, Certificate, CertificateHash, SignatureAlgorithm,
    SignedMessage,
};
use sha2::Digest;

mod utils;

#[derive(Debug, Parser)]
struct Args {
    #[command(flatten)]
    cert: CertificateOrRef,
    #[command(flatten)]
    content: utils::ContentOpts,
    signature_header: SignatureAlgorithm,
    /// Signature in base64
    signature: String,
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = true, multiple = false)]
struct CertificateOrRef {
    /// Hash of the certificate from the store to use to verify the signature.
    #[arg(long, value_parser = utils::CertificateSRIHashParser)]
    certificate_hash: Option<CertificateHash>,
    /// Path to a file containing the certificate in DER format.
    #[arg(long)]
    der_file: Option<PathBuf>,
    /// Path to a file containing the certificate in PEM format.
    #[arg(long)]
    pem_file: Option<PathBuf>,
    /// Certificate in PEM format but without headers.
    #[arg(long)]
    pem: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("args={args:?}");

    let data = args.content.into_bytes()?;

    let signature = data_encoding::BASE64
        .decode(args.signature.as_bytes())
        .context("Invalid signature format")?;

    let cert = if let Some(hash) = args.cert.certificate_hash {
        let res =
            get_der_encoded_certificate(&libparsec_platform_pki::CertificateReference::Hash(hash))?;
        println!(
            "Will verify signature using cert with id {{{}}}",
            data_encoding::BASE64.encode_display(&res.cert_ref.id)
        );
        Certificate::from_der_owned(res.der_content.into())
    } else if let Some(der_file) = args.cert.der_file {
        let raw = std::fs::read(der_file).context("Failed to read file")?;
        Certificate::from_der_owned(raw)
    } else if let Some(pem_file) = args.cert.pem_file {
        let raw = std::fs::read(pem_file).context("Failed to read file")?;
        Certificate::try_from_pem(&r#raw)?.into_owned()
    } else if let Some(pem) = args.cert.pem {
        let raw = data_encoding::BASE64
            .decode(pem.as_bytes())
            .context("Invalid pem base64")?;
        Certificate::from_der_owned(raw)
    } else {
        unreachable!("Should be handle by clap")
    };

    #[cfg(feature = "hash-sri-display")]
    {
        let fingerprint =
            CertificateHash::SHA256(Box::new(sha2::Sha256::digest(cert.as_ref()).into()));
        println!("Certificate fingerprint: {fingerprint}");
    }

    let signed_message = SignedMessage {
        algo: args.signature_header,
        signature,
        message: data,
    };

    match verify_message(&signed_message, cert) {
        Ok(_) => {
            println!("The message as a correct signature")
        }
        Err(e) => println!("The message as an incorrect signature: {e}"),
    }

    Ok(())
}

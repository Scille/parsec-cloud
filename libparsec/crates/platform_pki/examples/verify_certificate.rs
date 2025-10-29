// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

mod utils;

use anyhow::Context;
use clap::Parser;
use libparsec_platform_pki::{list_trusted_root_certificate_anchor, verify_certificate};
use libparsec_types::DateTime;

#[derive(Debug, Parser)]
struct Args {
    #[command(flatten)]
    cert: utils::CertificateOrRef,
}

pub fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("args={args:?}");

    let trusted_roots =
        list_trusted_root_certificate_anchor().context("Cannot list trusted root certificates")?;
    let untrusted_certificate = args
        .cert
        .get_certificate()
        .context("Cannot get certificate")?;

    let end_cert = untrusted_certificate
        .to_end_certificate()
        .context("Invalid certificate")?;
    println!("Untrusted certificate: {}", utils::display_cert(&end_cert));

    let path = verify_certificate(
        &end_cert,
        &trusted_roots,
        &[],
        DateTime::now(),
        webpki::KeyUsage::client_auth(),
    )
    .context("Cannot trust certificate")?;

    println!("trusted path:");
    println!(
        "  root: subject={}",
        utils::display_x509_raw_name(&path.anchor().subject)
    );
    let mut intermediate = path.intermediate_certificates().enumerate().peekable();
    if intermediate.peek().is_some() {
        println!("  intermediate certificates:");
        for (i, cert) in intermediate {
            println!("  {:02}. {}", i + 1, utils::display_cert(cert));
        }
    }
    let trusted_cert = path.end_entity();
    println!("  leaf: {}", utils::display_cert(trusted_cert));

    Ok(())
}

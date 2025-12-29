// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

mod utils;

use anyhow::Context;
use clap::Parser;
use libparsec_types::DateTime;

#[derive(Debug, Parser)]
struct Args {
    #[command(flatten)]
    cert: utils::CertificateOrRef,
}

pub fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();
    log::debug!("args={args:?}");

    let trusted_roots = libparsec_platform_pki::list_trusted_root_certificate_anchors()
        .context("Cannot list trusted root certificates")?;
    println!("Found {} trusted roots", trusted_roots.len());

    let intermediate_certificates = libparsec_platform_pki::list_intermediate_certificates()
        .context("Cannot list intermediate certificates")?;
    println!(
        "Found {} intermediate certificates",
        intermediate_certificates.len()
    );

    let untrusted_certificate = args
        .cert
        .get_certificate()
        .context("Cannot get certificate")?;

    let end_cert = untrusted_certificate
        .to_end_certificate()
        .context("Invalid certificate")?;
    println!("Untrusted certificate: {}", utils::display_cert(&end_cert));

    let path = libparsec_platform_pki::verify_certificate(
        &end_cert,
        &trusted_roots,
        &intermediate_certificates,
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

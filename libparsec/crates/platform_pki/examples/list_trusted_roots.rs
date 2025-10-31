// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

mod utils;

use libparsec_platform_pki::list_trusted_root_certificate_anchor;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let roots = list_trusted_root_certificate_anchor()?;

    println!("Found {} trusted roots", roots.len());

    for (i, anchor) in roots.iter().enumerate() {
        println!(
            "Root Certificate #{i} subject={}",
            utils::display_x509_raw_name(anchor.subject.as_ref())
        );
        println!(
            "  raw subject: {}",
            data_encoding::BASE64.encode_display(&anchor.subject)
        );
        println!();
    }
    Ok(())
}

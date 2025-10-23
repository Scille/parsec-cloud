// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use libparsec_platform_pki::{list_trusted_root_certificate_der, x509};
use x509_cert::der::{DecodeValue, Header, SliceReader, Tag};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let roots = list_trusted_root_certificate_der()?;

    println!("Found {} trusted roots", roots.len());

    for (i, anchor) in roots.iter().enumerate() {
        println!(
            "Root Certificate #{i} subject: {:?}",
            x509_cert::name::Name::decode_value(
                &mut SliceReader::new(&anchor.subject).unwrap(),
                Header::new(Tag::Sequence, anchor.subject.len()).unwrap()
            )
            .map(x509::extract_dn_list_from_rnd_seq)
            .unwrap_or_default()
        );
        println!(
            "  raw subject: {}",
            data_encoding::BASE64.encode_display(&anchor.subject)
        );
        println!();
    }
    Ok(())
}

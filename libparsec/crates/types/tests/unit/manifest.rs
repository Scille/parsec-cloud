// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use rstest::rstest;

use crate::{DataError, Manifest};

#[rstest]
#[case::blocksize(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   type: "file_manifest"
    //   author: "alice@dev1"
    //   timestamp: ext(1, 1638618643.208821)
    //   id: ext(2, hex!("87c6b5fd3b454c94bab51d6af1c6930b"))
    //   version: 42
    //   created: ext(1, 1638618643.208821)
    //   updated: ext(1, 1638618643.208821)
    //   blocks: []
    //   blocksize: 2
    //   parent: ext(2, hex!("07748fbf67a646428427865fd730bf3e"))
    //   size: 700
    &hex!(
        "789ceb5e5e965a549c999fa7b5ac20b12835afe406137b49fffef4656e4e2dea6df1d70df6"
        "db2d4f2e4a4d2c494db9cee87823eb6acbd9d8e04599293798da8f6dfd6bedea3365d756d9"
        "ac8fc726732f2f2d484155b6a4a4b220756d5a664e6a7c6e625e665a6a71c9b2a49cfce4ec"
        "e2094b8a33ab52cf32ed5996585a92915fb42a31273339d52125b5cc706549662e5061626e"
        "01c2a095105d402d4c0066eb5178"
    ),
    DataError::Serialization
)]
#[case::dummy(b"dummy", DataError::Compression)]
#[case::dummy_compressed(
    &hex!("789c4b29cdcdad04000667022d"),
    DataError::Serialization
)]
fn invalid_deserialize_data(#[case] data: &[u8], #[case] error: DataError) {
    let manifest = Manifest::deserialize_data(data);

    assert_eq!(manifest, Err(error));
}

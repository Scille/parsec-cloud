// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;
use libparsec_types::fixtures::{alice, Device};
use libparsec_types::prelude::*;

use std::collections::HashMap;
use std::num::NonZeroU64;

#[rstest]
fn test_file_manifest_1_blocks(alice: &Device) {
    test_file_manifest_x_blocks(alice, 1);
}

#[rstest]
fn test_file_manifest_10_blocks(alice: &Device) {
    test_file_manifest_x_blocks(alice, 10);
}

#[rstest]
fn test_file_manifest_1000_blocks(alice: &Device) {
    test_file_manifest_x_blocks(alice, 1000);
}

fn test_file_manifest_x_blocks(alice: &Device, blocks_count: usize) {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let mut blocks = vec![];
    for i in 0..blocks_count {
        blocks.push(BlockAccess {
            id: BlockID::default(),
            digest: HashDigest::from_data(&format!("block {}", i).as_bytes()),
            offset: (i * 512) as u64,
            size: NonZeroU64::try_from(512).unwrap(),
        });
    }
    let manifest = FileManifest {
        author: alice.device_id.to_owned(),
        timestamp: now,
        id: VlobID::default(),
        version: 42,
        created: now,
        updated: now,
        blocks,
        blocksize: Blocksize::try_from(512).unwrap(),
        parent: VlobID::default(),
        size: 700,
    };

    use std::time::Instant;
    let now = Instant::now();
    for _ in 0..1000 {
        let data = manifest.dump_sign_and_encrypt(&alice.signing_key, &key);
        FileManifest::decrypt_verify_and_load(
            &data,
            &key,
            &alice.verify_key(),
            &alice.device_id,
            manifest.timestamp,
            Some(manifest.id),
            Some(manifest.version),
        )
        .unwrap();
    }
    let data = manifest.dump_sign_and_encrypt(&alice.signing_key, &key);
    println!(
        "file manifest with {} children done in {:?}, size: {:?}",
        blocks_count,
        now.elapsed(),
        data.len()
    );
}

#[rstest]
fn test_folder_manifest_1_blocks(alice: &Device) {
    test_folder_manifest_x_children(alice, 1);
}

#[rstest]
fn test_folder_manifest_10_blocks(alice: &Device) {
    test_folder_manifest_x_children(alice, 10);
}

#[rstest]
fn test_folder_manifest_1000_blocks(alice: &Device) {
    test_folder_manifest_x_children(alice, 1000);
}

fn test_folder_manifest_x_children(alice: &Device, children_count: usize) {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let mut children = vec![];
    for i in 0..children_count {
        children.push((format!("entry{}", i).parse().unwrap(), VlobID::default()));
    }
    let manifest = FolderManifest {
        author: alice.device_id.to_owned(),
        timestamp: now,
        id: VlobID::default(),
        version: 42,
        created: now,
        updated: now,
        children: HashMap::from_iter(children.into_iter()),
        parent: VlobID::default(),
    };

    use std::time::Instant;
    let now = Instant::now();
    for _ in 0..1000 {
        let data = manifest.dump_sign_and_encrypt(&alice.signing_key, &key);
        FolderManifest::decrypt_verify_and_load(
            &data,
            &key,
            &alice.verify_key(),
            &alice.device_id,
            manifest.timestamp,
            Some(manifest.id),
            Some(manifest.version),
        )
        .unwrap();
    }
    let data = manifest.dump_sign_and_encrypt(&alice.signing_key, &key);
    println!(
        "folder manifest with {} children done in {:?}, size: {:?}",
        children_count,
        now.elapsed(),
        data.len()
    );
}

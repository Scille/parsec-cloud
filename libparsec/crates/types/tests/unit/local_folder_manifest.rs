// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Functions using rstest parametrize ignores `#[warn(clippy::too_many_arguments)]`
// decorator, so we must do global ignore instead :(
#![allow(clippy::too_many_arguments)]

use std::collections::{HashMap, HashSet};

use crate::fixtures::{alice, timestamp, Device};
use crate::prelude::*;
use libparsec_tests_lite::prelude::*;

type AliceLocalFolderManifest = Box<dyn FnOnce(&Device) -> (&'static [u8], LocalFolderManifest)>;

#[rstest]
#[case::folder_manifest(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'local_folder_manifest'
    //   base:
    //     type: 'folder_manifest',
    //     author: ext(2, 0xde10a11cec0010000000000000000000),
    //     timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b),
    //     parent: ext(2, 0x07748fbf67a646428427865fd730bf3e),
    //     version: 42,
    //     created: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     children: {
    //       wksp1: ext(2, 0xb82954f1138b4d719b7f5bd78915d20f),
    //     },
    //   }
    //   parent: ext(2, 0x40c8fe8cd69742479f418f1a6d54ea7a)
    //   need_sync: True
    //   updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   children: { wksp2: ext(2, 0xd7e3af6a03e1414db0f4682901e9aa4b), }
    //   local_confinement_points: [ ext(2, 0xd7e3af6a03e1414db0f4682901e9aa4b), ]
    //   remote_confinement_points: [ ext(2, 0xb82954f1138b4d719b7f5bd78915d20f), ]
    //   speculative: False
    &hex!(
    "7335c86779af273389ee30d90a70c9f95e2bc395a97ef804bb84abe0f08b6411abe5b7"
    "cb9fda6d14a16375c488254473a60ca456fa1d735bedf2d8f73ab5cdafcbc1121a6def"
    "62c2b7b8e02b35fffc12fc41ee9cfb2b6cbdcf2f08b5ed869179d2287b74810714a570"
    "fe81f558dba5a7c86f69008e6b34caef79c3ed8cafa55ba5d5fdc63f3a076f583dae5b"
    "da729b02f0e3481b929182c4a459b95b5e4fd45d6ebc07d0619fc1cecdb01df1a819ff"
    "c532a33768f2971421672cdd3582279350b739735d7027cbe6bba65d60f39d74a63a0d"
    "25acff95df7c87a7022ec6aebd7ff758407ffc82bb203725be5ed6cdd2d90a842cf043"
    "181c322dde2e3ccdcbe59921fe1011b09451fd905dc2c4b6f35fd8e69d003f29752e60"
    "68cff472236aa2a92edfe8d207bd71469207f9b3816c21c88e59e1f16c65197124877a"
    "af1e0b4e5354f483946219287a03b4396bac37549f52b30145067750"
    )[..],
        LocalFolderManifest {
            parent: VlobID::from_hex("40c8fe8cd69742479f418f1a6d54ea7a").unwrap(),
            updated: now,
            base: FolderManifest {
                author: alice.device_id,
                timestamp: now,
                id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
                parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
                version: 42,
                created: now,
                updated: now,
                children: HashMap::from([
                    ("wksp1".parse().unwrap(), VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap())
                ]),
            },
            children: HashMap::from([
                ("wksp2".parse().unwrap(), VlobID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap())
            ]),
            local_confinement_points: HashSet::from([VlobID::from_hex("d7e3af6a03e1414db0f4682901e9aa4b").unwrap()]),
            remote_confinement_points: HashSet::from([VlobID::from_hex("b82954f1138b4d719b7f5bd78915d20f").unwrap()]),
            need_sync: true,
            speculative: false,
        }
    )
}))]
#[case::folder_manifest_speculative(Box::new(|alice: &Device| {
    let now = "2021-12-04T11:50:43.208821Z".parse().unwrap();
    (
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   type: 'local_folder_manifest'
    //   base: { type: 'folder_manifest',
    //     author: ext(2, 0xde10a11cec0010000000000000000000),
    //     timestamp: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b),
    //     parent: ext(2, 0x07748fbf67a646428427865fd730bf3e),
    //     version: 0,
    //     created: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z,
    //     children: { },
    //   }
    //   parent: ext(2, 0x40c8fe8cd69742479f418f1a6d54ea7a)
    //   need_sync: True
    //   updated: ext(1, 1638618643208821) i.e. 2021-12-04T12:50:43.208821Z
    //   children: { }
    //   local_confinement_points: [ ]
    //   remote_confinement_points: [ ]
    //   speculative: True
    &hex!(
    "9f3fa66cd290bd883d465ffd4d69a0022de3821265ff8a9d73ffe6cec71648b13fd8a1"
    "f12828d8fa7f546e6e3e522db59de612d99eb78e916c27c352e702710f16d0284f2ad7"
    "8a0d637c404c3813cca6192df094581c9536f8c2739a2eac516ad0a46a6e61f5fe06c1"
    "54dc26b88334fbfaaecc1278a9938298275acd178df7320bf07e0d55162213dfce122d"
    "5aee5c0cd336ddb7e19d512bed90c22d7ad4901b80f3929b1668eab56867009c981fa3"
    "2637e55bb4527769882d618fa05158aae94307ee875b3f752cca7acdc6fc92652ab3f9"
    "5fea223fe98aef3c5180e50030c24d8175dd44dc3eeb70713a14073fb0ad390732c440"
    "0c43f1ab71e646212ae8a454221fcf3afa90b1ea7babdb04399319f4ec100308edcf7f"
    "f54217977fcd2784e05e5ae9b5d9"
    )[..],
        LocalFolderManifest {
            parent: VlobID::from_hex("40c8fe8cd69742479f418f1a6d54ea7a").unwrap(),
            updated: now,
            base: FolderManifest {
                author: alice.device_id,
                timestamp: now,
                id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
                parent: VlobID::from_hex("07748fbf67a646428427865fd730bf3e").unwrap(),
                version: 0,
                created: now,
                updated: now,
                children: HashMap::new(),
            },
            children: HashMap::new(),
            local_confinement_points: HashSet::new(),
            remote_confinement_points: HashSet::new(),
            need_sync: true,
            speculative: true,
        }
    )
}))]
fn serde_local_folder_manifest(
    alice: &Device,
    #[case] generate_data_and_expected: AliceLocalFolderManifest,
) {
    let (data, expected) = generate_data_and_expected(alice);
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let manifest = LocalChildManifest::decrypt_and_load(data, &key).unwrap();

    p_assert_eq!(manifest, LocalChildManifest::Folder(expected.clone()));

    // Also test serialization round trip
    let folder_manifest: LocalFolderManifest = manifest.try_into().unwrap();
    let data2 = folder_manifest.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to encryption and keys order
    let manifest2 = LocalChildManifest::decrypt_and_load(&data2, &key).unwrap();

    p_assert_eq!(manifest2, LocalChildManifest::Folder(expected));
}

#[rstest]
fn local_folder_manifest_new(timestamp: DateTime) {
    let author = DeviceID::default();
    let parent = VlobID::default();
    let lfm = LocalFolderManifest::new(author, parent, timestamp);

    p_assert_eq!(lfm.base.author, author);
    p_assert_eq!(lfm.base.timestamp, timestamp);
    p_assert_eq!(lfm.base.parent, parent);
    p_assert_eq!(lfm.base.version, 0);
    p_assert_eq!(lfm.base.created, timestamp);
    p_assert_eq!(lfm.base.updated, timestamp);
    assert!(lfm.need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.children.len(), 0);
    p_assert_eq!(lfm.local_confinement_points.len(), 0);
    p_assert_eq!(lfm.remote_confinement_points.len(), 0);
    assert!(!lfm.speculative);
}

#[rstest]
fn local_folder_manifest_new_root(timestamp: DateTime) {
    let author = DeviceID::default();
    let realm = VlobID::default();
    let lfm = LocalFolderManifest::new_root(author, realm, timestamp, true);

    p_assert_eq!(lfm.base.author, author);
    p_assert_eq!(lfm.base.timestamp, timestamp);
    p_assert_eq!(lfm.base.id, realm);
    p_assert_eq!(lfm.base.parent, realm);
    p_assert_eq!(lfm.base.version, 0);
    p_assert_eq!(lfm.base.created, timestamp);
    p_assert_eq!(lfm.base.updated, timestamp);
    assert!(lfm.need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.children.len(), 0);
    p_assert_eq!(lfm.local_confinement_points.len(), 0);
    p_assert_eq!(lfm.remote_confinement_points.len(), 0);
    assert!(lfm.speculative);
}

#[rstest]
#[case::empty((
    HashMap::new(),
    HashMap::new(),
    0,
    "",
))]
#[case::children_filtered((
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    HashMap::new(),
    1,
    ".+",
))]
#[case::children((
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    0,
    ".mp4",
))]
fn local_folder_manifest_from_remote(
    timestamp: DateTime,
    #[case] input: (
        HashMap<EntryName, VlobID>,
        HashMap<EntryName, VlobID>,
        usize,
        &str,
    ),
) {
    let (children, expected_children, filtered, regex) = input;
    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children,
    };

    let lfm = LocalFolderManifest::from_remote(fm.clone(), &Regex::from_regex_str(regex).unwrap());

    p_assert_eq!(lfm.base, fm);
    assert!(!lfm.need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.children, expected_children);
    p_assert_eq!(lfm.local_confinement_points.len(), 0);
    p_assert_eq!(lfm.remote_confinement_points.len(), filtered);
    assert!(!lfm.speculative);
}

#[rstest]
#[case::empty(HashMap::new(), HashMap::new(), HashMap::new(), 0, 0, false, "")]
#[case::children_filtered(
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    HashMap::new(),
    HashMap::new(),
    1,
    0,
    false,
    ".+",
)]
#[case::children(
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    HashMap::new(),
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    0,
    0,
    false,
    ".mp4",
)]
#[case::children_merged(
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()),
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()),
    ]),
    0,
    1,
    false,
    ".mp4",
)]
#[case::need_sync(
    HashMap::new(),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()),
    ]),
    0,
    0,
    true,
    ".png",
)]
fn local_folder_manifest_from_remote_with_local_context(
    timestamp: DateTime,
    #[case] children: HashMap<EntryName, VlobID>,
    #[case] local_children: HashMap<EntryName, VlobID>,
    #[case] expected_children: HashMap<EntryName, VlobID>,
    #[case] filtered: usize,
    #[case] merged: usize,
    #[case] need_sync: bool,
    #[case] regex: &str,
) {
    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children,
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        need_sync: false,
        updated: timestamp,
        children: local_children.clone(),
        local_confinement_points: local_children.into_values().collect(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };

    let lfm = LocalFolderManifest::from_remote_with_local_context(
        fm.clone(),
        &Regex::from_regex_str(regex).unwrap(),
        &lfm,
        timestamp,
    );

    p_assert_eq!(lfm.base, fm);
    p_assert_eq!(lfm.need_sync, need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.children, expected_children);
    p_assert_eq!(lfm.local_confinement_points.len(), merged);
    p_assert_eq!(lfm.remote_confinement_points.len(), filtered);
    assert!(!lfm.speculative);
}

#[rstest]
fn local_folder_manifest_to_remote(timestamp: DateTime) {
    let t1 = timestamp;
    let t2 = t1.add_us(1);
    let author = DeviceID::default();
    let parent = VlobID::default();
    let mut lfm = LocalFolderManifest::new(author, parent, t1);

    lfm.children
        .insert("file1.png".parse().unwrap(), VlobID::default());
    lfm.updated = t2;

    let author = DeviceID::default();
    let fm = lfm.to_remote(author, timestamp);

    p_assert_eq!(fm.author, author);
    p_assert_eq!(fm.timestamp, timestamp);
    p_assert_eq!(fm.id, lfm.base.id);
    p_assert_eq!(fm.parent, lfm.base.parent);
    p_assert_eq!(fm.version, lfm.base.version + 1);
    p_assert_eq!(fm.created, lfm.base.created);
    p_assert_eq!(fm.updated, lfm.updated);
    p_assert_eq!(fm.children, lfm.children);
}

#[rstest]
#[case::empty(HashMap::new(), HashMap::new(), HashMap::new(), 0, false, "")]
#[case::no_data(
    HashMap::new(),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    0,
    false,
    ".mp4",
)]
#[case::data(
    HashMap::from([
        ("file1.png".parse().unwrap(), Some(VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())),
        ("file2.mp4".parse().unwrap(), Some(VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())),
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()),
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()),
    ]),
    1,
    true,
    ".png",
)]
fn local_folder_manifest_evolve_children_and_mark_updated(
    timestamp: DateTime,
    #[case] data: HashMap<EntryName, Option<VlobID>>,
    #[case] children: HashMap<EntryName, VlobID>,
    #[case] expected_children: HashMap<EntryName, VlobID>,
    #[case] merged: usize,
    #[case] need_sync: bool,
    #[case] regex: &str,
) {
    let prevent_sync_pattern = Regex::from_regex_str(regex).unwrap();

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children: HashMap::new(),
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        need_sync: false,
        updated: timestamp,
        children,
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    }
    .evolve_children_and_mark_updated(data, &prevent_sync_pattern, timestamp);

    p_assert_eq!(lfm.base, fm);
    p_assert_eq!(lfm.need_sync, need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.children, expected_children);
    p_assert_eq!(lfm.local_confinement_points.len(), merged);
    p_assert_eq!(lfm.remote_confinement_points.len(), 0);
}

// TODO
#[rstest]
fn local_folder_manifest_apply_prevent_sync_pattern(timestamp: DateTime) {
    let prevent_sync_pattern = Regex::from_regex_str("").unwrap();

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: timestamp,
        updated: timestamp,
        children: HashMap::new(),
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        need_sync: false,
        updated: timestamp,
        children: HashMap::new(),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    }
    .apply_prevent_sync_pattern(&prevent_sync_pattern, timestamp);

    p_assert_eq!(lfm.base, fm);
    assert!(!lfm.need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.children, HashMap::new());
    p_assert_eq!(lfm.local_confinement_points, HashSet::new());
    p_assert_eq!(lfm.remote_confinement_points, HashSet::new());
}

// TODO: Add integrity tests for:
// - `LocalFolderManifest` with the following failing invariants:
//   * the manifest ID is different from the parent ID (when loaded as a child manifest)

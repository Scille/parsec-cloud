// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Functions using rstest parametrize ignores `#[warn(clippy::too_many_arguments)]`
// decorator, so we must do global ignore instead :(
#![allow(clippy::too_many_arguments)]

use std::collections::{HashMap, HashSet};

use crate::fixtures::{alice, timestamp, Device};
use crate::prelude::*;
use libparsec_tests_lite::prelude::*;

type AliceLocalFolderManifest = Box<dyn FnOnce(&Device) -> (&'static [u8], LocalFolderManifest)>;

const PREVENT_SYNC_PATTERN_TMP: &str = r"\.tmp$";

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
fn new(timestamp: DateTime) {
    let expected_author = DeviceID::default();
    let expected_parent = VlobID::default();
    let lfm = LocalFolderManifest::new(expected_author, expected_parent, timestamp);

    // Destruct manifests to ensure this code with fail to compile whenever a new field is introduced.
    let LocalFolderManifest {
        base:
            FolderManifest {
                author: base_author,
                timestamp: base_timestamp,
                id: base_id,
                parent: base_parent,
                version: base_version,
                created: base_created,
                updated: base_updated,
                children: base_children,
            },
        parent,
        need_sync,
        updated,
        children,
        local_confinement_points,
        remote_confinement_points,
        speculative,
    } = lfm;

    p_assert_ne!(base_id, expected_parent);
    p_assert_eq!(base_author, expected_author);
    p_assert_eq!(base_timestamp, timestamp);
    p_assert_eq!(base_parent, expected_parent);
    p_assert_eq!(base_version, 0);
    p_assert_eq!(base_created, timestamp);
    p_assert_eq!(base_updated, timestamp);
    p_assert_eq!(base_children, HashMap::new());

    p_assert_eq!(parent, expected_parent);
    assert!(need_sync);
    p_assert_eq!(updated, timestamp);
    p_assert_eq!(children.len(), 0);
    p_assert_eq!(local_confinement_points.len(), 0);
    p_assert_eq!(remote_confinement_points.len(), 0);
    assert!(!speculative);
}

#[rstest]
fn new_root(#[values(true, false)] speculative: bool, timestamp: DateTime) {
    let expected_speculative = speculative;
    let expected_author = DeviceID::default();
    let expected_realm = VlobID::default();
    let lfm = LocalFolderManifest::new_root(
        expected_author,
        expected_realm,
        timestamp,
        expected_speculative,
    );

    // Destruct manifests to ensure this code with fail to compile whenever a new field is introduced.
    let LocalFolderManifest {
        base:
            FolderManifest {
                author: base_author,
                timestamp: base_timestamp,
                id: base_id,
                parent: base_parent,
                version: base_version,
                created: base_created,
                updated: base_updated,
                children: base_children,
            },
        parent,
        need_sync,
        updated,
        children,
        local_confinement_points,
        remote_confinement_points,
        speculative,
    } = lfm;

    p_assert_eq!(base_id, expected_realm);
    p_assert_eq!(base_author, expected_author);
    p_assert_eq!(base_timestamp, timestamp);
    p_assert_eq!(base_parent, expected_realm);
    p_assert_eq!(base_version, 0);
    p_assert_eq!(base_created, timestamp);
    p_assert_eq!(base_updated, timestamp);
    p_assert_eq!(base_children, HashMap::new());

    p_assert_eq!(parent, expected_realm);
    assert!(need_sync);
    p_assert_eq!(updated, timestamp);
    p_assert_eq!(children.len(), 0);
    p_assert_eq!(local_confinement_points.len(), 0);
    p_assert_eq!(remote_confinement_points.len(), 0);
    p_assert_eq!(speculative, expected_speculative);
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
    r"\.mp4$",
))]
fn from_remote(
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

    let lfm = LocalFolderManifest::from_remote(
        fm.clone(),
        &PreventSyncPattern::from_regex(regex).unwrap(),
    );

    p_assert_eq!(lfm.base, fm);
    assert!(!lfm.need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.children, expected_children);
    p_assert_eq!(lfm.local_confinement_points.len(), 0);
    p_assert_eq!(lfm.remote_confinement_points.len(), filtered);
    assert!(!lfm.speculative);
}

#[rstest]
#[case::empty(
    HashMap::new(),
    HashMap::new(),
    HashSet::new(),
    HashMap::new(),
    HashSet::new(),
    HashSet::new(),
    false,
    ""
)]
#[case::remote_children_confined(
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    HashMap::new(),
    HashSet::new(),
    HashMap::new(),
    HashSet::from_iter([VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()]),
    HashSet::new(),
    false,
    PREVENT_SYNC_PATTERN_TMP,
)]
#[case::remote_children_not_confined(
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    HashMap::new(),
    HashSet::new(),
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    HashSet::new(),
    HashSet::new(),
    false,
    PREVENT_SYNC_PATTERN_TMP,
)]
#[case::local_children_confined(
    HashMap::new(),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashSet::from_iter([VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()]),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()),
    ]),
    HashSet::new(),
    HashSet::from_iter([VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()]),
    false,
    PREVENT_SYNC_PATTERN_TMP,
)]
#[case::local_children_confined_with_outdated_pattern(
    HashMap::new(),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashSet::from_iter([VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()]),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()),
    ]),
    HashSet::new(),
    HashSet::new(),
    true,
    // The pattern doesn't match the one used in the local manifest, hence the need_sync
    // is set to true (the previously confined data must now be synchronized !) and there
    // is no local confinement points anymore
    r"\.tmp~$",
)]
// Note there is no `remote_children_confined_with_outdated_pattern` test given the
// remote confinement points are just ignored by `LocalFolderManifest::from_remote_with_restored_local_confinement_points`
#[case::local_children_not_confined(
    HashMap::new(),
    // Local data are not confined, hence they are just ignored
    HashMap::from([
        ("file1.png".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashSet::new(),
    HashMap::new(),
    HashSet::new(),
    HashSet::new(),
    false,
    PREVENT_SYNC_PATTERN_TMP,
)]
#[case::remote_and_local_both_confined_on_same_name(
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashSet::from_iter([VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()]),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()),
    ]),
    HashSet::from_iter([VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()]),
    HashSet::from_iter([VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()]),
    false,
    PREVENT_SYNC_PATTERN_TMP,
)]
#[case::remote_and_local_both_confined_with_additional_local_changes(
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("4990FFDAF37848449546ADF309B09B04").unwrap()),
        ("file2.png".parse().unwrap(), VlobID::from_hex("732CAFF939B04ADE99028CE9D4D90F00").unwrap())
    ]),
    HashMap::from([
        ("fileA.tmp".parse().unwrap(), VlobID::from_hex("187A239DBAA44343ADFC3742BA5882C2").unwrap()),
        // fileB is not confined, hence it is going to be simply ignored
        ("fileB.png".parse().unwrap(), VlobID::from_hex("73F6A33B09904CBD84BC6572A44CB3E5").unwrap())
    ]),
    HashSet::from_iter([VlobID::from_hex("187A239DBAA44343ADFC3742BA5882C2").unwrap()]),
    HashMap::from([
        ("file2.png".parse().unwrap(), VlobID::from_hex("732CAFF939B04ADE99028CE9D4D90F00").unwrap()),
        ("fileA.tmp".parse().unwrap(), VlobID::from_hex("187A239DBAA44343ADFC3742BA5882C2").unwrap()),
    ]),
    HashSet::from_iter([VlobID::from_hex("4990FFDAF37848449546ADF309B09B04").unwrap()]),
    HashSet::from_iter([VlobID::from_hex("187A239DBAA44343ADFC3742BA5882C2").unwrap()]),
    // need_sync = false given everything not confined is ignored
    false,
    PREVENT_SYNC_PATTERN_TMP,
)]
fn from_remote_with_restored_local_confinement_points(
    timestamp: DateTime,
    #[case] remote_children: HashMap<EntryName, VlobID>,
    #[case] local_children: HashMap<EntryName, VlobID>,
    #[case] local_local_confinement_points: HashSet<VlobID>,
    #[case] expected_children: HashMap<EntryName, VlobID>,
    #[case] expected_remote_confinement_points: HashSet<VlobID>,
    #[case] expected_local_confinement_points: HashSet<VlobID>,
    #[case] expected_need_sync: bool,
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
        children: remote_children,
    };

    let lfm = LocalFolderManifest {
        // Note we always use the same base for all tests, which is also the remote
        // later use to call `LocalFolderManifest::from_remote_with_restored_local_confinement_points`.
        // This isn't a big deal given the tested method never use the local manifest's base
        // (i.e. this `base` field).
        // Of course this is not great to rely on implementation details in tests, but
        // alas this is an improvement for another time...
        base: fm.clone(),
        parent: fm.parent,
        need_sync: false,
        updated: timestamp,
        children: local_children.clone(),
        local_confinement_points: local_local_confinement_points,
        // Just like for `base`, we don't care about the remote confinement points
        // given `LocalFolderManifest::from_remote_with_restored_local_confinement_points`
        // ignores it and rebuilds it from the remote manifest only.
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };

    let lfm = LocalFolderManifest::from_remote_with_restored_local_confinement_points(
        fm.clone(),
        &PreventSyncPattern::from_regex(regex).unwrap(),
        &lfm,
        timestamp,
    );

    p_assert_eq!(lfm.base, fm);
    p_assert_eq!(lfm.need_sync, expected_need_sync);
    p_assert_eq!(lfm.updated, timestamp);
    p_assert_eq!(lfm.children, expected_children);
    p_assert_eq!(
        lfm.local_confinement_points,
        expected_local_confinement_points
    );
    p_assert_eq!(
        lfm.remote_confinement_points,
        expected_remote_confinement_points
    );
    assert!(!lfm.speculative);
}

#[rstest]
fn to_remote(
    #[values(
        "no_confinement",
        "with_local_confined",
        "with_remote_confined",
        "with_local_and_remote_confined_on_same_entry"
    )]
    kind: &str,
) {
    let t1 = "2000-01-01T00:00:00Z".parse().unwrap();
    let t2 = "2000-01-02T00:00:00Z".parse().unwrap();
    let t3 = "2000-01-03T00:00:00Z".parse().unwrap();
    let t4 = "2000-01-04T00:00:00Z".parse().unwrap();
    let expected_author = DeviceID::default();
    let expected_parent = VlobID::default();
    let mut lfm = LocalFolderManifest::new(expected_author, expected_parent, t1);

    let remote_child_id = VlobID::from_hex("8708d8c6263d42f99ab34a7051b7160b").unwrap();
    let local_child_id = VlobID::from_hex("3e0bfa6f20aa49eb9cb6b16db8e7be70").unwrap();

    lfm.base
        .children
        .insert("remote.png".parse().unwrap(), remote_child_id);
    lfm.base.updated = t2;
    lfm.base.version = 3;

    lfm.children
        .insert("local.png".parse().unwrap(), local_child_id);
    lfm.updated = t3;

    let expected_author = DeviceID::default();
    let mut expected_children = lfm.children.clone();

    match kind {
        "no_confinement" => (),
        "with_local_confined" => {
            let confined_id = VlobID::from_hex("58bf714d79454de39bf070c7e47f7fd2").unwrap();
            lfm.children
                .insert("local_confined.tmp".parse().unwrap(), confined_id);
            lfm.local_confinement_points.insert(confined_id);
        }
        "with_remote_confined" => {
            let confined_id = VlobID::from_hex("58bf714d79454de39bf070c7e47f7fd2").unwrap();
            lfm.base
                .children
                .insert("remote_confined.tmp".parse().unwrap(), confined_id);
            lfm.remote_confinement_points.insert(confined_id);
            expected_children.insert("remote_confined.tmp".parse().unwrap(), confined_id);
        }
        "with_local_and_remote_confined_on_same_entry" => {
            let local_confined_id = VlobID::from_hex("58bf714d79454de39bf070c7e47f7fd2").unwrap();
            let remote_confined_id = VlobID::from_hex("f13cb3bb9c1542cb87d0c690e3183999").unwrap();

            lfm.children
                .insert("local_confined.tmp".parse().unwrap(), local_confined_id);
            lfm.local_confinement_points.insert(local_confined_id);
            lfm.base
                .children
                .insert("remote_confined.tmp".parse().unwrap(), remote_confined_id);
            lfm.remote_confinement_points.insert(remote_confined_id);
            expected_children.insert("remote_confined.tmp".parse().unwrap(), remote_confined_id);
        }
        unknown => panic!("Unknown kind: {unknown}"),
    }

    let fm = lfm.to_remote(expected_author, t4);
    // Destruct manifests to ensure this code with fail to compile whenever a new field is introduced.
    let FolderManifest {
        author: fm_author,
        timestamp: fm_timestamp,
        id: fm_id,
        parent: fm_parent,
        version: fm_version,
        created: fm_created,
        updated: fm_updated,
        children: fm_children,
    } = fm;

    p_assert_eq!(fm_author, expected_author);
    p_assert_eq!(fm_timestamp, t4);
    p_assert_eq!(fm_id, lfm.base.id);
    p_assert_eq!(fm_parent, expected_parent);
    p_assert_eq!(fm_version, lfm.base.version + 1);
    p_assert_eq!(fm_created, lfm.base.created);
    p_assert_eq!(fm_updated, lfm.updated);
    p_assert_eq!(fm_children, expected_children);
}

#[rstest]
#[case::empty(HashMap::new(), HashMap::new(), HashMap::new(), HashSet::new(), false)]
#[case::no_data_and_existing_children_with_local_confinement(
    HashMap::new(),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()),
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()),
    ]),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()),
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()),
    ]),
    HashSet::from([VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()]),
    false,
)]
#[case::no_data_and_only_children_with_local_confinement(
    HashMap::new(),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashSet::from([VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()]),
    false,
)]
#[case::remove_non_confined_entry(
    HashMap::from([
        ("file2.mp4".parse().unwrap(), None),
    ]),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()),
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()),
    ]),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashSet::from([VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()]),
    true,
)]
#[case::remove_confined_entry(
    HashMap::from([
        ("file1.tmp".parse().unwrap(), None),
    ]),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()),
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()),
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()),
    ]),
    HashSet::new(),
    false,
)]
#[case::add_confined_entry(
    HashMap::from([
        ("file1.tmp".parse().unwrap(), Some(VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())),
    ]),
    HashMap::new(),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap())
    ]),
    HashSet::from([VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()]),
    false,
)]
#[case::add_non_confined_entry(
    HashMap::from([
        ("file2.mp4".parse().unwrap(), Some(VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())),
    ]),
    HashMap::new(),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap())
    ]),
    HashSet::new(),
    true,
)]
#[case::replace_confined_entry(
    HashMap::from([
        ("file1.tmp".parse().unwrap(), Some(VlobID::from_hex("58083131379C48909A13E239E2408921").unwrap())),
    ]),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()),
    ]),
    HashMap::from([
        ("file1.tmp".parse().unwrap(), VlobID::from_hex("58083131379C48909A13E239E2408921").unwrap())
    ]),
    HashSet::from([VlobID::from_hex("58083131379C48909A13E239E2408921").unwrap()]),
    false,
)]
#[case::replace_non_confined_entry(
    HashMap::from([
        ("file2.mp4".parse().unwrap(), Some(VlobID::from_hex("58083131379C48909A13E239E2408921").unwrap())),
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()),
    ]),
    HashMap::from([
        ("file2.mp4".parse().unwrap(), VlobID::from_hex("58083131379C48909A13E239E2408921").unwrap())
    ]),
    HashSet::new(),
    true,
)]
fn evolve_children_and_mark_updated(
    #[case] data: HashMap<EntryName, Option<VlobID>>,
    #[case] children: HashMap<EntryName, VlobID>,
    #[case] expected_children: HashMap<EntryName, VlobID>,
    #[case] expected_local_confinement_points: HashSet<VlobID>,
    #[case] expected_need_sync: bool,
) {
    let prevent_sync_pattern = PreventSyncPattern::from_regex(PREVENT_SYNC_PATTERN_TMP).unwrap();
    let t1 = "2000-01-01T00:00:00Z".parse().unwrap();
    let t2 = "2000-01-02T00:00:00Z".parse().unwrap();

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp: t1,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: t1,
        updated: t1,
        children: HashMap::new(),
    };

    let mut lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        need_sync: false,
        updated: t1,
        local_confinement_points: HashSet::from_iter(children.iter().filter_map(|(name, id)| {
            if prevent_sync_pattern.is_match(name.as_ref()) {
                Some(*id)
            } else {
                None
            }
        })),
        children,
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };
    // Actual method tested
    lfm.evolve_children_and_mark_updated(data, &prevent_sync_pattern, t2);

    p_assert_eq!(lfm.base, fm);
    p_assert_eq!(lfm.need_sync, expected_need_sync);
    let expected_updated = if expected_need_sync { t2 } else { t1 };
    p_assert_eq!(lfm.updated, expected_updated);
    p_assert_eq!(lfm.children, expected_children);
    p_assert_eq!(
        lfm.local_confinement_points,
        expected_local_confinement_points
    );
    p_assert_eq!(lfm.remote_confinement_points.len(), 0);
}

#[test]
fn apply_prevent_sync_pattern_nothing_confined() {
    let t1 = "2000-01-01T00:00:00Z".parse().unwrap();
    let t2 = "2000-01-02T00:00:00Z".parse().unwrap();
    let t3 = "2000-01-03T00:00:00Z".parse().unwrap();

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp: t1,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: t1,
        updated: t1,
        children: HashMap::from_iter([
            (
                "file1.png".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                // Removed in local
                "file2.txt".parse().unwrap(),
                VlobID::from_hex("F0F3AD570E7D4A7C9C2CCB3DD00414E1").unwrap(),
            ),
            (
                // Renamed in local
                "file3.txt".parse().unwrap(),
                VlobID::from_hex("80583ECB218A490AAB6ECDA237D850EA").unwrap(),
            ),
        ]),
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        need_sync: true,
        updated: t2,
        children: HashMap::from_iter([
            (
                "file1.png".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                "fileA.mp4".parse().unwrap(),
                VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(),
            ),
            (
                "file3-renamed.txt".parse().unwrap(),
                VlobID::from_hex("80583ECB218A490AAB6ECDA237D850EA").unwrap(),
            ),
        ]),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };

    // New prevent sync pattern doesn't match any entry, so nothing should change
    let new_prevent_sync_pattern =
        PreventSyncPattern::from_regex(PREVENT_SYNC_PATTERN_TMP).unwrap();
    let lfm2 = lfm.apply_prevent_sync_pattern(&new_prevent_sync_pattern, t3);

    p_assert_eq!(lfm2, lfm);
}

#[test]
fn apply_prevent_sync_pattern_stability_with_confined() {
    let t1 = "2000-01-01T00:00:00Z".parse().unwrap();
    let t2 = "2000-01-02T00:00:00Z".parse().unwrap();
    let t3 = "2000-01-03T00:00:00Z".parse().unwrap();

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp: t1,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: t1,
        updated: t1,
        children: HashMap::from_iter([
            (
                "file1.png".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                // Removed in local
                "file2.txt".parse().unwrap(),
                VlobID::from_hex("F0F3AD570E7D4A7C9C2CCB3DD00414E1").unwrap(),
            ),
            (
                // Renamed in local
                "file3.txt".parse().unwrap(),
                VlobID::from_hex("80583ECB218A490AAB6ECDA237D850EA").unwrap(),
            ),
            (
                // Remote confined
                "file4.tmp".parse().unwrap(),
                VlobID::from_hex("198762BA0C744DC0B45B2B17678C51CE").unwrap(),
            ),
        ]),
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        need_sync: true,
        updated: t2,
        children: HashMap::from_iter([
            (
                "file1.png".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                "file3-renamed.txt".parse().unwrap(),
                VlobID::from_hex("80583ECB218A490AAB6ECDA237D850EA").unwrap(),
            ),
            (
                "fileA.mp4".parse().unwrap(),
                VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(),
            ),
            (
                // Local confined
                "fileB.tmp".parse().unwrap(),
                VlobID::from_hex("B0C37F14927244FA8550EDAECEA09E96").unwrap(),
            ),
        ]),
        // Current prevent sync pattern is `\.tmp$`
        local_confinement_points: HashSet::from_iter([VlobID::from_hex(
            "B0C37F14927244FA8550EDAECEA09E96",
        )
        .unwrap()]),
        remote_confinement_points: HashSet::from_iter([VlobID::from_hex(
            "198762BA0C744DC0B45B2B17678C51CE",
        )
        .unwrap()]),
        speculative: false,
    };

    // We re-apply the same `\.tmp$` prevent sync pattern, so nothing should change
    let current_prevent_sync_pattern =
        PreventSyncPattern::from_regex(PREVENT_SYNC_PATTERN_TMP).unwrap();
    let lfm2 = lfm.apply_prevent_sync_pattern(&current_prevent_sync_pattern, t3);

    p_assert_eq!(lfm2, lfm);
}

#[test]
fn apply_prevent_sync_pattern_with_non_confined_local_children_matching_future_pattern() {
    let t1 = "2000-01-01T00:00:00Z".parse().unwrap();
    let t2 = "2000-01-02T00:00:00Z".parse().unwrap();
    let t3 = "2000-01-03T00:00:00Z".parse().unwrap();

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp: t1,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: t1,
        updated: t1,
        children: HashMap::new(),
    };

    // Create a local folder manifest without any confinement points (the local children
    // have names ending in `.tmp`, but we can consider the current prevent sync pattern
    // is something else for now).
    let lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        need_sync: true,
        updated: t2,
        children: HashMap::from_iter([
            (
                // Not currently confined !
                "fileA.tmp".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                "fileB.mp4".parse().unwrap(),
                VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(),
            ),
        ]),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };

    // Now we change the prevent sync pattern to something that matches some of the local children
    let new_prevent_sync_pattern =
        PreventSyncPattern::from_regex(PREVENT_SYNC_PATTERN_TMP).unwrap();
    let lfm = lfm.apply_prevent_sync_pattern(&new_prevent_sync_pattern, t3);

    p_assert_eq!(lfm.remote_confinement_points, HashSet::new());
    p_assert_eq!(
        lfm.local_confinement_points,
        HashSet::from_iter([VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()])
    );
    p_assert_eq!(
        lfm.children,
        HashMap::from_iter([
            (
                "fileA.tmp".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()
            ),
            (
                "fileB.mp4".parse().unwrap(),
                VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()
            ),
        ])
    );
    p_assert_eq!(lfm.need_sync, true);
    // The last update is actually from t2.
    // The `apply_prevent_sync_pattern` call at t3 merely flagged fileA as confined.
    p_assert_eq!(lfm.updated, t2);
}

#[test]
fn apply_prevent_sync_pattern_with_non_confined_remote_children_matching_future_pattern() {
    let t1 = "2000-01-01T00:00:00Z".parse().unwrap();
    let t2 = "2000-01-02T00:00:00Z".parse().unwrap();
    let t3 = "2000-01-03T00:00:00Z".parse().unwrap();

    // Create a local folder manifest without any confinement points (the local children
    // have names ending in `.tmp`, but we can consider the current prevent sync pattern
    // is something else for now).

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp: t1,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: t1,
        updated: t1,
        children: HashMap::from_iter([
            (
                "file1.png".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                // Removed in local
                "file2.txt".parse().unwrap(),
                VlobID::from_hex("F0F3AD570E7D4A7C9C2CCB3DD00414E1").unwrap(),
            ),
            (
                // Not currently confined !
                "file3.tmp".parse().unwrap(),
                VlobID::from_hex("198762BA0C744DC0B45B2B17678C51CE").unwrap(),
            ),
        ]),
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        need_sync: true,
        updated: t2,
        children: HashMap::from_iter([
            (
                "file1.png".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                "file3.tmp".parse().unwrap(),
                VlobID::from_hex("198762BA0C744DC0B45B2B17678C51CE").unwrap(),
            ),
        ]),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };

    // Now we change the prevent sync pattern to something that matches some of the remote children
    let new_prevent_sync_pattern =
        PreventSyncPattern::from_regex(PREVENT_SYNC_PATTERN_TMP).unwrap();
    let lfm = lfm.apply_prevent_sync_pattern(&new_prevent_sync_pattern, t3);

    p_assert_eq!(
        lfm.remote_confinement_points,
        HashSet::from_iter([VlobID::from_hex("198762BA0C744DC0B45B2B17678C51CE").unwrap()])
    );
    p_assert_eq!(
        lfm.local_confinement_points,
        HashSet::from_iter([VlobID::from_hex("198762BA0C744DC0B45B2B17678C51CE").unwrap()])
    );
    p_assert_eq!(
        lfm.children,
        HashMap::from_iter([
            (
                "file1.png".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                "file3.tmp".parse().unwrap(),
                VlobID::from_hex("198762BA0C744DC0B45B2B17678C51CE").unwrap(),
            ),
        ])
    );
    p_assert_eq!(lfm.need_sync, true);
    // The last update is actually from t2.
    // The `apply_prevent_sync_pattern` call at t3 merely filtered `file3.tmp`.
    p_assert_eq!(lfm.updated, t2);
}

#[test]
fn apply_prevent_sync_pattern_with_confined_local_children_turning_non_confined() {
    let t1 = "2000-01-01T00:00:00Z".parse().unwrap();
    let t2 = "2000-01-02T00:00:00Z".parse().unwrap();
    let t3 = "2000-01-02T00:00:00Z".parse().unwrap();

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp: t1,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: t1,
        updated: t1,
        children: HashMap::new(),
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        need_sync: true,
        updated: t2,
        children: HashMap::from_iter([
            (
                "fileA.tmp".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                "fileB.mp4".parse().unwrap(),
                VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(),
            ),
        ]),
        // Current prevent sync pattern is `\.tmp$`
        local_confinement_points: HashSet::from_iter([VlobID::from_hex(
            "3DF3AC53967C43D889860AE2F459F42B",
        )
        .unwrap()]),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };

    // The new prevent sync pattern doesn't match any entry, hence `fileA.tmp` is
    // no longer confined, hence manifest's `updated` field should also get updated.
    let new_prevent_sync_pattern = PreventSyncPattern::from_regex(r"\.tmp~$").unwrap();
    let lfm = lfm.apply_prevent_sync_pattern(&new_prevent_sync_pattern, t3);

    p_assert_eq!(lfm.remote_confinement_points, HashSet::new());
    p_assert_eq!(lfm.local_confinement_points, HashSet::new());
    p_assert_eq!(
        lfm.children,
        HashMap::from_iter([
            (
                "fileA.tmp".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap()
            ),
            (
                "fileB.mp4".parse().unwrap(),
                VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap()
            ),
        ])
    );
    p_assert_eq!(lfm.need_sync, true);
    p_assert_eq!(lfm.updated, t3);
}

#[test]
fn apply_prevent_sync_pattern_with_local_changes_and_confined_remote_children_turning_non_confined()
{
    let t1 = "2000-01-01T00:00:00Z".parse().unwrap();
    let t2 = "2000-01-02T00:00:00Z".parse().unwrap();
    let t3 = "2000-01-03T00:00:00Z".parse().unwrap();

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp: t1,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: t1,
        updated: t1,
        children: HashMap::from_iter([
            (
                "file1.png".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                // Removed in local
                "file2.txt".parse().unwrap(),
                VlobID::from_hex("F0F3AD570E7D4A7C9C2CCB3DD00414E1").unwrap(),
            ),
            (
                // Remote confined
                "file3.tmp".parse().unwrap(),
                VlobID::from_hex("198762BA0C744DC0B45B2B17678C51CE").unwrap(),
            ),
        ]),
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        // The local manifest has remove an entry from the remote, so there is some changes
        need_sync: true,
        updated: t2,
        children: HashMap::from_iter([(
            "file1.png".parse().unwrap(),
            VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
        )]),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::from_iter([VlobID::from_hex(
            "198762BA0C744DC0B45B2B17678C51CE",
        )
        .unwrap()]),
        speculative: false,
    };

    // The new prevent sync pattern doesn't match any entry, hence `file3.tmp` is
    // no longer confined.
    // Manifest is still need sync due to the removal of `file2.txt`, however the
    // `updated` shouldn't has changed since the change in confinement is on
    // an entry that is already in remote manifest !
    let new_prevent_sync_pattern = PreventSyncPattern::from_regex(r"\.tmp~$").unwrap();
    let lfm = lfm.apply_prevent_sync_pattern(&new_prevent_sync_pattern, t3);

    p_assert_eq!(lfm.remote_confinement_points, HashSet::new());
    p_assert_eq!(lfm.local_confinement_points, HashSet::new());
    p_assert_eq!(
        lfm.children,
        HashMap::from_iter([
            (
                "file1.png".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                "file3.tmp".parse().unwrap(),
                VlobID::from_hex("198762BA0C744DC0B45B2B17678C51CE").unwrap(),
            ),
        ])
    );
    p_assert_eq!(lfm.need_sync, true);
    p_assert_eq!(lfm.updated, t2);
}

#[test]
fn apply_prevent_sync_pattern_with_only_confined_remote_children_turning_non_confined() {
    let t1 = "2000-01-01T00:00:00Z".parse().unwrap();
    let t2 = "2000-01-02T00:00:00Z".parse().unwrap();

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp: t1,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: t1,
        updated: t1,
        children: HashMap::from_iter([
            (
                "file1.png".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                // Remote confined
                "file3.tmp".parse().unwrap(),
                VlobID::from_hex("198762BA0C744DC0B45B2B17678C51CE").unwrap(),
            ),
        ]),
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        // The local manifest has no changes compared to the remote
        need_sync: false,
        updated: t1,
        children: HashMap::from_iter([(
            "file1.png".parse().unwrap(),
            VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
        )]),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::from_iter([VlobID::from_hex(
            "198762BA0C744DC0B45B2B17678C51CE",
        )
        .unwrap()]),
        speculative: false,
    };

    // The new prevent sync pattern doesn't match any entry, hence `file3.tmp` is
    // no longer confined, but manifest doesn't need to be sync since this
    // entry is already in remote manifest !
    let new_prevent_sync_pattern = PreventSyncPattern::from_regex(r"\.tmp~$").unwrap();
    let lfm = lfm.apply_prevent_sync_pattern(&new_prevent_sync_pattern, t2);

    p_assert_eq!(lfm.remote_confinement_points, HashSet::new());
    p_assert_eq!(lfm.local_confinement_points, HashSet::new());
    p_assert_eq!(
        lfm.children,
        HashMap::from_iter([
            (
                "file1.png".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                "file3.tmp".parse().unwrap(),
                VlobID::from_hex("198762BA0C744DC0B45B2B17678C51CE").unwrap(),
            ),
        ])
    );
    p_assert_eq!(lfm.need_sync, false);
    p_assert_eq!(lfm.updated, t1);
}

#[test]
fn apply_prevent_sync_pattern_with_broader_prevent_sync_pattern() {
    let t1 = "2000-01-01T00:00:00Z".parse().unwrap();
    let t2 = "2000-01-02T00:00:00Z".parse().unwrap();
    let t3 = "2000-01-02T00:00:00Z".parse().unwrap();

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp: t1,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: t1,
        updated: t1,
        children: HashMap::from_iter([
            (
                "file1.png".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                // Removed in local
                "file2.txt".parse().unwrap(),
                VlobID::from_hex("F0F3AD570E7D4A7C9C2CCB3DD00414E1").unwrap(),
            ),
            (
                // Remote confined
                "file3.tmp".parse().unwrap(),
                VlobID::from_hex("198762BA0C744DC0B45B2B17678C51CE").unwrap(),
            ),
        ]),
    };

    let lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        need_sync: true,
        updated: t2,
        children: HashMap::from_iter([
            (
                "file1.png".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                "fileA.mp4".parse().unwrap(),
                VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(),
            ),
            (
                // Local confined
                "fileB.tmp".parse().unwrap(),
                VlobID::from_hex("B0C37F14927244FA8550EDAECEA09E96").unwrap(),
            ),
        ]),
        // Current prevent sync pattern is `\.tmp$`
        local_confinement_points: HashSet::from_iter([VlobID::from_hex(
            "B0C37F14927244FA8550EDAECEA09E96",
        )
        .unwrap()]),
        remote_confinement_points: HashSet::from_iter([VlobID::from_hex(
            "198762BA0C744DC0B45B2B17678C51CE",
        )
        .unwrap()]),
        speculative: false,
    };

    // `.+` is a superset of the previous `\.tmp$` pattern, all entries should
    // be confined now
    let new_prevent_sync_pattern = PreventSyncPattern::from_regex(".+").unwrap();
    let lfm = lfm.apply_prevent_sync_pattern(&new_prevent_sync_pattern, t3);

    p_assert_eq!(
        lfm.remote_confinement_points,
        HashSet::from_iter([
            VlobID::from_hex("F0F3AD570E7D4A7C9C2CCB3DD00414E1").unwrap(),
            VlobID::from_hex("198762BA0C744DC0B45B2B17678C51CE").unwrap(),
            VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
        ])
    );
    p_assert_eq!(
        lfm.local_confinement_points,
        HashSet::from_iter([
            VlobID::from_hex("B0C37F14927244FA8550EDAECEA09E96").unwrap(),
            VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(),
            VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
        ])
    );
    p_assert_eq!(
        lfm.children,
        HashMap::from_iter([
            (
                "file1.png".parse().unwrap(),
                VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap(),
            ),
            (
                "fileA.mp4".parse().unwrap(),
                VlobID::from_hex("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(),
            ),
            (
                "fileB.tmp".parse().unwrap(),
                VlobID::from_hex("B0C37F14927244FA8550EDAECEA09E96").unwrap(),
            ),
        ])
    );
    p_assert_eq!(lfm.need_sync, true);
    p_assert_eq!(lfm.updated, t3);
}

#[rstest]
fn apply_prevent_sync_pattern_on_renamed_entry(
    #[values(
        "no_confinement",
        "remote_name_matching_current_prevent_sync_pattern",
        "local_name_matching_current_prevent_sync_pattern",
        "remote_and_local_names_matching_current_prevent_sync_pattern",
        "remote_name_matching_future_prevent_sync_pattern",
        "local_name_matching_future_prevent_sync_pattern",
        "remote_and_local_names_matching_future_prevent_sync_pattern"
    )]
    kind: &str,
) {
    let t1 = "2000-01-01T00:00:00Z".parse().unwrap();
    let t2 = "2000-01-02T00:00:00Z".parse().unwrap();
    let t3 = "2000-01-03T00:00:00Z".parse().unwrap();
    let child_id = VlobID::from_hex("3DF3AC53967C43D889860AE2F459F42B").unwrap();

    let mut local_confinement_points = HashSet::new();
    let mut expected_local_confinement_points = HashSet::new();
    let mut remote_confinement_points = HashSet::new();
    let mut expected_remote_confinement_points = HashSet::new();

    let (remote_name, local_name, expected_updated): (EntryName, EntryName, DateTime) = match kind {
        "no_confinement" => {
            let remote_name = "file1.txt".parse().unwrap();
            let local_name = "file1-renamed.txt".parse().unwrap();
            (remote_name, local_name, t2)
        }
        "remote_name_matching_current_prevent_sync_pattern" => {
            let remote_name = "file1.tmp".parse().unwrap();
            let local_name = "file1-renamed.txt".parse().unwrap();
            remote_confinement_points.insert(child_id);
            (remote_name, local_name, t2)
        }
        "local_name_matching_current_prevent_sync_pattern" => {
            let remote_name = "file1.txt".parse().unwrap();
            let local_name = "file1-renamed.tmp".parse().unwrap();
            local_confinement_points.insert(child_id);
            (remote_name, local_name, t3)
        }
        "remote_and_local_names_matching_current_prevent_sync_pattern" => {
            let remote_name = "file1.tmp".parse().unwrap();
            let local_name = "file1-renamed.tmp".parse().unwrap();
            remote_confinement_points.insert(child_id);
            local_confinement_points.insert(child_id);
            (remote_name, local_name, t3)
        }
        "remote_name_matching_future_prevent_sync_pattern" => {
            let remote_name = "file1.tmp~".parse().unwrap();
            let local_name = "file1-renamed.txt".parse().unwrap();
            expected_remote_confinement_points.insert(child_id);
            (remote_name, local_name, t2)
        }
        "local_name_matching_future_prevent_sync_pattern" => {
            let remote_name = "file1.txt".parse().unwrap();
            let local_name = "file1-renamed.tmp~".parse().unwrap();
            expected_local_confinement_points.insert(child_id);
            (remote_name, local_name, t2)
        }
        "remote_and_local_names_matching_future_prevent_sync_pattern" => {
            let remote_name = "file1.tmp~".parse().unwrap();
            let local_name = "file1-renamed.tmp~".parse().unwrap();
            expected_remote_confinement_points.insert(child_id);
            expected_local_confinement_points.insert(child_id);
            (remote_name, local_name, t2)
        }
        unknown => panic!("Unknown kind: {unknown}"),
    };

    let fm = FolderManifest {
        author: DeviceID::default(),
        timestamp: t1,
        id: VlobID::default(),
        parent: VlobID::default(),
        version: 0,
        created: t1,
        updated: t1,
        children: HashMap::from_iter([(remote_name.clone(), child_id)]),
    };
    let lfm = LocalFolderManifest {
        base: fm.clone(),
        parent: fm.parent,
        need_sync: true,
        updated: t2,
        children: HashMap::from_iter([(local_name.clone(), child_id)]),
        local_confinement_points,
        remote_confinement_points,
        speculative: false,
    };

    // Now apply the new prevent sync pattern...
    let new_prevent_sync_pattern = PreventSyncPattern::from_regex(r"\.tmp~$").unwrap();
    let lfm = lfm.apply_prevent_sync_pattern(&new_prevent_sync_pattern, t3);

    p_assert_eq!(
        lfm.remote_confinement_points,
        expected_remote_confinement_points
    );
    p_assert_eq!(
        lfm.local_confinement_points,
        expected_local_confinement_points
    );
    p_assert_eq!(lfm.children, HashMap::from_iter([(local_name, child_id)]));
    p_assert_eq!(lfm.need_sync, true);
    p_assert_eq!(lfm.updated, expected_updated);
}

#[rstest]
fn workspace_manifest_check_integrity(timestamp: DateTime) {
    let author = DeviceID::default();
    let realm = VlobID::default();
    let speculative = false;

    // Good integrity

    let mut lwm = LocalWorkspaceManifest::new(author, realm, timestamp, speculative);
    lwm.check_data_integrity().unwrap();

    // Bad integrity: different ID than parent
    lwm.0.parent = VlobID::default();
    p_assert_eq!(
        lwm.check_data_integrity().unwrap_err(),
        DataError::DataIntegrity {
            data_type: "libparsec_types::local_manifest::folder::LocalFolderManifest",
            invariant: "id and parent are the same for root manifest"
        }
    );
}

#[rstest]
fn child_folder_manifest_check_integrity(timestamp: DateTime) {
    let author = DeviceID::default();
    let realm = VlobID::default();

    // Good integrity

    let lcm = LocalChildManifest::Folder(LocalFolderManifest::new(author, realm, timestamp));
    lcm.check_data_integrity().unwrap();

    // Bad integrity: same ID as parent
    let lcm = {
        let mut manifest = LocalFolderManifest::new(author, realm, timestamp);
        manifest.parent = manifest.base.id;
        LocalChildManifest::Folder(manifest)
    };
    p_assert_eq!(
        lcm.check_data_integrity().unwrap_err(),
        DataError::DataIntegrity {
            data_type: "libparsec_types::local_manifest::folder::LocalFolderManifest",
            invariant: "id and parent are different for child manifest"
        }
    );
}

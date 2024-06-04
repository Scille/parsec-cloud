// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use bytes::Bytes;
use hex_literal::hex;
use pretty_assertions::assert_eq;

use libparsec_serialization_format::parsec_data_from_contents;

#[path = "./common/libparsec_types_mock.rs"]
mod libparsec_types;

#[test]
fn simple() {
    parsec_data_from_contents!(
        r#"{
            "label": "FooManifest",
            "type": "foo_manifest",
            "other_fields": [
                {
                    "name": "author",
                    "type": "DeviceID"
                },
                {
                    "name": "version",
                    "type": "Integer"
                },
                {
                    "name": "certificate",
                    "type": "Bytes"
                }
            ]
        }"#
    );

    // Check round-trip serialize/deserialize

    let data = FooManifestData {
        ty: FooManifestDataType,
        author: libparsec_types::DeviceID("alice@pc1".to_owned()),
        version: 1,
        certificate: Bytes::from_static(b"whatever"),
    };
    let dumped = rmp_serde::to_vec_named(&data).unwrap();
    let reloaded: FooManifestData = rmp_serde::from_slice(&dumped).unwrap();
    assert_eq!(reloaded, data,);
}

#[test]
fn with_default_field() {
    fn generate_default_device_id() -> libparsec_types::DeviceID {
        libparsec_types::DeviceID("default".to_owned())
    }
    parsec_data_from_contents!(
        r#"{
            "label": "FooManifest",
            "type": "foo_manifest",
            "other_fields": [
                {
                    "name": "author",
                    "type": "DeviceID",
                    "default": "generate_default_device_id"
                },
                {
                    "name": "version",
                    "type": "Integer"
                }
            ]
        }"#
    );

    // `{"type": "foo_manifest", "version": 42}` in msgpack
    let raw = hex!("82a474797065ac666f6f5f6d616e6966657374a776657273696f6e2a");
    let data: FooManifestData = rmp_serde::from_slice(&raw).unwrap();
    let expected = FooManifestData {
        ty: FooManifestDataType,
        author: libparsec_types::DeviceID("default".to_owned()),
        version: 42,
    };
    assert_eq!(data, expected)
}

// Supporting empty type is currently disabled (as all our schemes have a `type` field).
// This code is kept just in case we eventually have an exotic need for it...

// #[test]
// fn empty_type_field() {
//     parsec_data_from_contents!(
//         r#"{
//             "label": "FooManifest",
//             "other_fields": [
//                 {
//                     "name": "author",
//                     "type": "DeviceID"
//                 },
//                 {
//                     "name": "version",
//                     "type": "Integer"
//                 }
//             ]
//         }"#
//     );

//     // Check round-trip serialize/deserialize

//     let data = FooManifestData {
//         author: libparsec_types::DeviceID("alice@pc1".to_owned()),
//         version: 1,
//     };
//     let dumped = rmp_serde::to_vec_named(&data).unwrap();
//     let reloaded: FooManifestData = rmp_serde::from_slice(&dumped).unwrap();
//     assert_eq!(reloaded, data,);
// }

#[test]
fn introduce_in_field() {
    parsec_data_from_contents!(
        r#"{
            "label": "FooManifest",
            "type": "foo_manifest",
            "other_fields": [
                {
                    "name": "author",
                    "type": "DeviceID"
                },
                {
                    "name": "is_cool_guy",
                    "type": "Boolean",
                    "introduced_in_revision": 115
                }
            ]
        }"#
    );

    let data_with = FooManifestData {
        ty: FooManifestDataType,
        author: libparsec_types::DeviceID("alice@pc1".to_owned()),
        is_cool_guy: libparsec_types::Maybe::Present(true),
    };
    let data_without = FooManifestData {
        ty: FooManifestDataType,
        author: libparsec_types::DeviceID("alice@pc1".to_owned()),
        is_cool_guy: libparsec_types::Maybe::Absent,
    };

    // `{"type": "foo_manifest", "author": "alice@pc1"}` in msgpack
    let raw = hex!("82a474797065ac666f6f5f6d616e6966657374a6617574686f72a9616c69636540706331");
    let loaded_without: FooManifestData = rmp_serde::from_slice(&raw).unwrap();
    assert_eq!(loaded_without, data_without);

    // `{"type": "foo_manifest", "author": "alice@pc1", "is_cool_guy": true}` in msgpack
    let raw = hex!("83a474797065ac666f6f5f6d616e6966657374a6617574686f72a9616c69636540706331ab69735f636f6f6c5f677579c3");
    let loaded_without: FooManifestData = rmp_serde::from_slice(&raw).unwrap();
    assert_eq!(loaded_without, data_with);

    // Check round-trip serialize/deserialize

    let dumped_with = rmp_serde::to_vec_named(&data_with).unwrap();
    let reloaded_with: FooManifestData = rmp_serde::from_slice(&dumped_with).unwrap();
    assert_eq!(reloaded_with, data_with);

    let dumped_without = rmp_serde::to_vec_named(&data_without).unwrap();
    let reloaded_without: FooManifestData = rmp_serde::from_slice(&dumped_without).unwrap();
    assert_eq!(reloaded_without, data_without);
}

#[test]
fn deprecate_in_field() {
    parsec_data_from_contents!(
        r#"{
            "label": "FooManifest",
            "type": "foo_manifest",
            "other_fields": [
                {
                    "name": "author",
                    "type": "DeviceID"
                },
                {
                    "name": "is_cool_guy",
                    "type": "Boolean",
                    "deprecated_in_revision": 300
                }
            ]
        }"#
    );

    let data = FooManifestData {
        ty: FooManifestDataType,
        author: libparsec_types::DeviceID("alice@pc1".to_owned()),
    };

    // `{"type": "foo_manifest", "author": "alice@pc1"}` in msgpack
    let raw = hex!("82a474797065ac666f6f5f6d616e6966657374a6617574686f72a9616c69636540706331");
    let loaded_without: FooManifestData = rmp_serde::from_slice(&raw).unwrap();
    assert_eq!(loaded_without, data);

    // `{"type": "foo_manifest", "author": "alice@pc1", "is_cool_guy": true}` in msgpack
    let raw = hex!("83a474797065ac666f6f5f6d616e6966657374a6617574686f72a9616c69636540706331ab69735f636f6f6c5f677579c3");
    let loaded_without: FooManifestData = rmp_serde::from_slice(&raw).unwrap();
    assert_eq!(loaded_without, data);

    // Check round-trip serialize/deserialize

    let dumped = rmp_serde::to_vec_named(&data).unwrap();
    let reloaded: FooManifestData = rmp_serde::from_slice(&dumped).unwrap();
    assert_eq!(reloaded, data);
}

#[test]
fn introduce_then_deprecate_in_field() {
    parsec_data_from_contents!(
        r#"{
            "label": "FooManifest",
            "type": "foo_manifest",
            "other_fields": [
                {
                    "name": "author",
                    "type": "DeviceID"
                },
                {
                    "name": "is_cool_guy",
                    "type": "Boolean",
                    "introduced_in_revision": 115,
                    "deprecated_in_revision": 300
                }
            ]
        }"#
    );

    let data = FooManifestData {
        ty: FooManifestDataType,
        author: libparsec_types::DeviceID("alice@pc1".to_owned()),
    };

    // `{"type": "foo_manifest", "author": "alice@pc1"}` in msgpack
    let raw = hex!("82a474797065ac666f6f5f6d616e6966657374a6617574686f72a9616c69636540706331");
    let loaded_without: FooManifestData = rmp_serde::from_slice(&raw).unwrap();
    assert_eq!(loaded_without, data);

    // `{"type": "foo_manifest", "author": "alice@pc1", "is_cool_guy": true}` in msgpack
    let raw = hex!("83a474797065ac666f6f5f6d616e6966657374a6617574686f72a9616c69636540706331ab69735f636f6f6c5f677579c3");
    let loaded_without: FooManifestData = rmp_serde::from_slice(&raw).unwrap();
    assert_eq!(loaded_without, data);

    // Check round-trip serialize/deserialize

    let dumped = rmp_serde::to_vec_named(&data).unwrap();
    let reloaded: FooManifestData = rmp_serde::from_slice(&dumped).unwrap();
    assert_eq!(reloaded, data);
}

#[test]
fn introduce_deprecate_in_root() {
    parsec_data_from_contents!(
        r#"{
            "label": "FooManifest",
            "type": "foo_manifest",
            "other_fields": [],
            "introduced_in_revision": 115,
            "deprecated_in_revision": 300
        }"#
    );
    // Introduced/deprecated at root level are only for documentation and don't do anything
}

#[test]
fn nested_type() {
    parsec_data_from_contents!(
        r#"{
            "label": "FooManifest",
            "type": "foo_manifest",
            "other_fields": [
                {"name": "e", "type": "EnumNestedType"},
                {"name": "u", "type": "UnionNestedType"},
                {"name": "s", "type": "StructNestedType"}
            ],
            "nested_types": [
                {
                    "name": "EnumNestedType",
                    "variants": [
                        {
                            "name": "E1",
                            "discriminant_value": "e1"
                        },
                        {
                            "name": "E2",
                            "discriminant_value": "e2"
                        }
                    ]
                },
                {
                    "name": "UnionNestedType",
                    "discriminant_field": "type",
                    "variants": [
                        {
                            "name": "U1",
                            "discriminant_value": "u1",
                            "fields": [
                                {
                                    "name": "f",
                                    "type": "Integer"
                                }
                            ]
                        },
                        {
                            "name": "U2",
                            "discriminant_value": "u2",
                            "fields": []
                        }
                    ]
                },
                {
                    "name": "StructNestedType",
                    "fields": [
                        {
                            "name": "f",
                            "type": "Integer"
                        }
                    ]
                }
            ]
        }"#
    );

    let data = FooManifestData {
        ty: FooManifestDataType,
        e: EnumNestedType::E1,
        u: UnionNestedType::U1 { f: 42 },
        s: StructNestedType { f: 42 },
    };

    // `{"type": "foo_manifest", "e": "e1", "u": {"type": "u1", "f": 42}, "s": {"f": 42}}` in msgpack
    let raw = hex!(
        "84a474797065ac666f6f5f6d616e6966657374a165a26531a17582a474797065a27531a1662aa17381a1662a"
    );
    let loaded: FooManifestData = rmp_serde::from_slice(&raw).unwrap();
    assert_eq!(loaded, data);

    // Check round-trip serialize/deserialize

    let dumped = rmp_serde::to_vec_named(&data).unwrap();
    let reloaded: FooManifestData = rmp_serde::from_slice(&dumped).unwrap();
    assert_eq!(reloaded, data);
}

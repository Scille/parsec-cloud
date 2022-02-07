// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use hex_literal::hex;
use rstest::*;
use serde_test::{assert_tokens, Configure, Token};
use std::collections::HashMap;

use parsec_api_crypto::*;
use parsec_api_types::*;

const SAMPLE_UUID: [u8; 16] = hex!("b574912de0cf4afd8b1795eee9411a58");
const SAMPLE_DATETIME: &str = "2000-01-01T12:00:00.123456Z";
const SAMPLE_SECRETKEY: [u8; 32] =
    hex!("6ad96422ec0bc18fbbf4ead7e45e8119790006d6b3d14f7d9516672cca32d0d0");
const SAMPLE_HASHDIGEST: [u8; 32] =
    hex!("2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae");

#[rstest]
fn serde_on_manifest_content(
    #[values(
        (
            ManifestContent::File {
                author: "alice@pc1".parse().unwrap(),
                timestamp: SAMPLE_DATETIME.parse().unwrap(),
                id: EntryID::from(SAMPLE_UUID),
                parent: EntryID::from(SAMPLE_UUID),
                version: 2,
                created: SAMPLE_DATETIME.parse().unwrap(),
                updated: SAMPLE_DATETIME.parse().unwrap(),
                size: 30,
                blocksize: 16,
                blocks: vec![
                    BlockAccess {
                        id: BlockID::from(SAMPLE_UUID),
                        key: SecretKey::from(SAMPLE_SECRETKEY),
                        offset: 0,
                        size: 16,
                        digest: HashDigest::from(SAMPLE_HASHDIGEST),
                    },
                    BlockAccess {
                        id: BlockID::from(SAMPLE_UUID),
                        key: SecretKey::from(SAMPLE_SECRETKEY),
                        offset: 16,
                        size: 14,
                        digest: HashDigest::from(SAMPLE_HASHDIGEST),
                    },
                ],
            },
            vec![
                Token::Struct { name: "ManifestContent", len: 11 },

                Token::Str("type"),
                Token::Str("file_manifest"),

                Token::Str("author"),
                Token::BorrowedStr("alice@pc1"),

                Token::Str("timestamp"),
                Token::F64(946728000.123456),

                Token::Str("id"),
                Token::Bytes(&SAMPLE_UUID),

                Token::Str("parent"),
                Token::Bytes(&SAMPLE_UUID),

                Token::Str("version"),
                Token::U32(2),

                Token::Str("created"),
                Token::F64(946728000.123456),

                Token::Str("updated"),
                Token::F64(946728000.123456),

                Token::Str("size"),
                Token::U32(30),

                Token::Str("blocksize"),
                Token::U32(16),

                Token::Str("blocks"),
                Token::Seq { len: Some(2) },

                Token::Struct { name: "BlockAccess", len: 5 },

                Token::Str("id"),
                Token::Bytes(&SAMPLE_UUID),

                Token::Str("key"),
                Token::Bytes(&SAMPLE_SECRETKEY),

                Token::Str("offset"),
                Token::U32(0),

                Token::Str("size"),
                Token::U32(16),

                Token::Str("digest"),
                Token::Bytes(&SAMPLE_HASHDIGEST),

                Token::StructEnd,

                Token::Struct { name: "BlockAccess", len: 5 },

                Token::Str("id"),
                Token::Bytes(&SAMPLE_UUID),

                Token::Str("key"),
                Token::Bytes(&SAMPLE_SECRETKEY),

                Token::Str("offset"),
                Token::U32(16),

                Token::Str("size"),
                Token::U32(14),

                Token::Str("digest"),
                Token::Bytes(&SAMPLE_HASHDIGEST),

                Token::StructEnd,

                Token::SeqEnd,
                Token::StructEnd,
            ]
        ),

        (
            ManifestContent::Folder {
                author: "alice@pc1".parse().unwrap(),
                timestamp: SAMPLE_DATETIME.parse().unwrap(),
                id: EntryID::from(SAMPLE_UUID),
                parent: EntryID::from(SAMPLE_UUID),
                version: 2,
                created: SAMPLE_DATETIME.parse().unwrap(),
                updated: SAMPLE_DATETIME.parse().unwrap(),
                children: HashMap::from([
                    (
                        EntryName::try_from("foo").unwrap(),
                        ManifestEntry(EntryID::from(SAMPLE_UUID)),
                    ),
                ])
            },
            vec![
                Token::Struct { name: "ManifestContent", len: 9 },

                Token::Str("type"),
                Token::Str("folder_manifest"),

                Token::Str("author"),
                Token::BorrowedStr("alice@pc1"),

                Token::Str("timestamp"),
                Token::F64(946728000.123456),

                Token::Str("id"),
                Token::Bytes(&SAMPLE_UUID),

                Token::Str("parent"),
                Token::Bytes(&SAMPLE_UUID),

                Token::Str("version"),
                Token::U32(2),

                Token::Str("created"),
                Token::F64(946728000.123456),

                Token::Str("updated"),
                Token::F64(946728000.123456),

                Token::Str("children"),
                Token::Map { len: Some(1) },

                Token::BorrowedStr("foo"),
                Token::Bytes(&SAMPLE_UUID),

                Token::MapEnd,

                Token::StructEnd,
            ]
        ),

        (
            ManifestContent::Workspace {
                author: "alice@pc1".parse().unwrap(),
                timestamp: SAMPLE_DATETIME.parse().unwrap(),
                id: EntryID::from(SAMPLE_UUID),
                version: 2,
                created: SAMPLE_DATETIME.parse().unwrap(),
                updated: SAMPLE_DATETIME.parse().unwrap(),
                children: HashMap::from([
                    (
                        EntryName::try_from("foo").unwrap(),
                        ManifestEntry(EntryID::from(SAMPLE_UUID)),
                    ),
                ])
            },
            vec![
                Token::Struct { name: "ManifestContent", len: 8 },

                Token::Str("type"),
                Token::Str("workspace_manifest"),

                Token::Str("author"),
                Token::BorrowedStr("alice@pc1"),

                Token::Str("timestamp"),
                Token::F64(946728000.123456),

                Token::Str("id"),
                Token::Bytes(&SAMPLE_UUID),

                Token::Str("version"),
                Token::U32(2),

                Token::Str("created"),
                Token::F64(946728000.123456),

                Token::Str("updated"),
                Token::F64(946728000.123456),

                Token::Str("children"),
                Token::Map { len: Some(1) },

                Token::BorrowedStr("foo"),
                Token::Bytes(&SAMPLE_UUID),

                Token::MapEnd,

                Token::StructEnd,
            ]
        ),

        (
            ManifestContent::User {
                author: "alice@pc1".parse().unwrap(),
                timestamp: SAMPLE_DATETIME.parse().unwrap(),
                id: EntryID::from(SAMPLE_UUID),
                version: 2,
                created: SAMPLE_DATETIME.parse().unwrap(),
                updated: SAMPLE_DATETIME.parse().unwrap(),
                last_processed_message: 42,
                workspaces: vec![
                    WorkspaceEntry {
                        id: EntryID::from(SAMPLE_UUID),
                        name: EntryName::try_from("foo").unwrap(),
                        key: SecretKey::from(SAMPLE_SECRETKEY),
                        encryption_revision: 2,
                        encrypted_on: SAMPLE_DATETIME.parse().unwrap(),
                        role_cached_on: SAMPLE_DATETIME.parse().unwrap(),
                        role: Some(RealmRole::Owner),
                    },
                    WorkspaceEntry {
                        id: EntryID::from(SAMPLE_UUID),
                        name: EntryName::try_from("bar").unwrap(),
                        key: SecretKey::from(SAMPLE_SECRETKEY),
                        encryption_revision: 2,
                        encrypted_on: SAMPLE_DATETIME.parse().unwrap(),
                        role_cached_on: SAMPLE_DATETIME.parse().unwrap(),
                        role: None,
                    },
                ]
            },
            vec![
                Token::Struct { name: "ManifestContent", len: 9 },

                Token::Str("type"),
                Token::Str("user_manifest"),

                Token::Str("author"),
                Token::BorrowedStr("alice@pc1"),

                Token::Str("timestamp"),
                Token::F64(946728000.123456),

                Token::Str("id"),
                Token::Bytes(&SAMPLE_UUID),

                Token::Str("version"),
                Token::U32(2),

                Token::Str("created"),
                Token::F64(946728000.123456),

                Token::Str("updated"),
                Token::F64(946728000.123456),

                Token::Str("last_processed_message"),
                Token::U32(42),

                Token::Str("workspaces"),
                Token::Seq { len: Some(2) },

                Token::Struct { name: "WorkspaceEntry", len: 7 },

                Token::Str("id"),
                Token::Bytes(&SAMPLE_UUID),

                Token::Str("name"),
                Token::BorrowedStr("foo"),

                Token::Str("key"),
                Token::Bytes(&SAMPLE_SECRETKEY),

                Token::Str("encryption_revision"),
                Token::U32(2),

                Token::Str("encrypted_on"),
                Token::F64(946728000.123456),

                Token::Str("role_cached_on"),
                Token::F64(946728000.123456),

                Token::Str("role"),
                Token::Some,
                Token::UnitVariant { name: "RealmRole", variant: "OWNER" },

                Token::StructEnd,

                Token::Struct { name: "WorkspaceEntry", len: 7 },

                Token::Str("id"),
                Token::Bytes(&SAMPLE_UUID),

                Token::Str("name"),
                Token::BorrowedStr("bar"),

                Token::Str("key"),
                Token::Bytes(&SAMPLE_SECRETKEY),

                Token::Str("encryption_revision"),
                Token::U32(2),

                Token::Str("encrypted_on"),
                Token::F64(946728000.123456),

                Token::Str("role_cached_on"),
                Token::F64(946728000.123456),

                Token::Str("role"),
                Token::None,

                Token::StructEnd,

                Token::SeqEnd,

                Token::StructEnd,
            ]
        ),
    )]
    data: (ManifestContent, Vec<Token>),
) {
    let (msg, tokens) = data;
    assert_tokens(&msg.compact(), &tokens);
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::prelude::*;
use hex_literal::hex;
use rstest::*;
use serde_test::{assert_tokens, Configure, Token};

use parsec_api_crypto::*;
use parsec_api_types::*;

const SAMPLE_UUID: [u8; 16] = hex!("b574912de0cf4afd8b1795eee9411a58");
const SAMPLE_DATETIME: &str = "2000-01-01T12:00:00.123456Z";
const SAMPLE_SECRETKEY: [u8; 32] =
    hex!("6ad96422ec0bc18fbbf4ead7e45e8119790006d6b3d14f7d9516672cca32d0d0");

#[rstest]
fn serde_on_message_content(
    #[values(
        (
            MessageContent::SharingGranted {
                name: EntryName::try_from("foo").unwrap(),
                id: EntryID::from(SAMPLE_UUID),
                encryption_revision: 2,
                encrypted_on: SAMPLE_DATETIME.parse::<DateTime<Utc>>().unwrap(),
                key: SecretKey::from(SAMPLE_SECRETKEY),
            },
            vec![
                Token::Struct { name: "MessageContent", len: 6 },

                Token::Str("type"),
                Token::Str("sharing.granted"),

                Token::Str("name"),
                Token::BorrowedStr("foo"),

                Token::Str("id"),
                Token::Bytes(&SAMPLE_UUID),

                Token::Str("encryption_revision"),
                Token::U32(2),

                Token::Str("encrypted_on"),
                Token::F64(946728000.123456),

                Token::Str("key"),
                Token::Bytes(&SAMPLE_SECRETKEY),

                Token::StructEnd,
            ]
        ),

        (
            MessageContent::SharingReencrypted {
                name: EntryName::try_from("foo").unwrap(),
                id: EntryID::from(SAMPLE_UUID),
                encryption_revision: 2,
                encrypted_on: SAMPLE_DATETIME.parse::<DateTime<Utc>>().unwrap(),
                key: SecretKey::from(SAMPLE_SECRETKEY),
            },
            vec![
                Token::Struct { name: "MessageContent", len: 6 },

                Token::Str("type"),
                Token::Str("sharing.reencrypted"),

                Token::Str("name"),
                Token::BorrowedStr("foo"),

                Token::Str("id"),
                Token::Bytes(&SAMPLE_UUID),

                Token::Str("encryption_revision"),
                Token::U32(2),

                Token::Str("encrypted_on"),
                Token::F64(946728000.123456),

                Token::Str("key"),
                Token::Bytes(&SAMPLE_SECRETKEY),

                Token::StructEnd,
            ]
        ),

        (
            MessageContent::SharingRevoked {id: EntryID::from(SAMPLE_UUID)},
            vec![
                Token::Struct { name: "MessageContent", len: 2 },

                Token::Str("type"),
                Token::Str("sharing.revoked"),

                Token::Str("id"),
                Token::Bytes(&SAMPLE_UUID),

                Token::StructEnd,
            ]
        ),

        (
            MessageContent::Ping {ping: "foo".to_string()},
            vec![
                Token::Struct { name: "MessageContent", len: 2 },

                Token::Str("type"),
                Token::Str("ping"),

                Token::Str("ping"),
                Token::Str("foo"),

                Token::StructEnd,
            ]
        ),
    )]
    data: (MessageContent, Vec<Token>),
) {
    let (msg, tokens) = data;
    assert_tokens(&msg.compact(), &tokens);
}

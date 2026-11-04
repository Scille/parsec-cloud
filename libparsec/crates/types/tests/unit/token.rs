// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use super::{AccessToken, AccessTokenDecodeError, TOKEN_SIZE};

#[test]
fn default_random() {
    let token = AccessToken::default();
    p_assert_eq!(token.as_bytes().len(), TOKEN_SIZE);
}

#[test]
fn eq() {
    let token = AccessToken::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
    let token2 = AccessToken::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
    p_assert_eq!(token, token2);
}

#[test]
fn ne() {
    let token = AccessToken::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
    let token2 = AccessToken::from_hex("000102030405060708090a0b0c0d0e0e").unwrap();
    assert_ne!(token, token2);
}

#[test]
fn hex() {
    let token = AccessToken::default();
    let hex = token.hex();
    let from_hex = AccessToken::from_hex(&hex).unwrap();
    p_assert_eq!(token, from_hex);
    p_assert_matches!(
        AccessToken::from_hex("#~!"),
        Err(AccessTokenDecodeError::InvalidHex),
    );
    p_assert_matches!(
        AccessToken::from_hex("00010203"),
        Err(AccessTokenDecodeError::InvalidSize { .. }),
    );
}

#[test]
fn debug() {
    let token = AccessToken::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
    p_assert_eq!(
        format!("{:?}", token),
        concat!(
            stringify!(AccessToken),
            "(\"000102030405060708090a0b0c0d0e0f\")"
        )
    );
}

#[test]
fn display() {
    let token = AccessToken::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
    p_assert_eq!(format!("{}", token), "000102030405060708090a0b0c0d0e0f");
}

#[test]
fn try_from_str() {
    let token: AccessToken = AccessToken::try_from("000102030405060708090a0b0c0d0e0f").unwrap();
    let expected = AccessToken::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
    p_assert_eq!(token, expected);
    p_assert_matches!(
        AccessToken::try_from("#~!"),
        Err(AccessTokenDecodeError::InvalidHex),
    );
    p_assert_matches!(
        AccessToken::try_from("00010203"),
        Err(AccessTokenDecodeError::InvalidSize { .. }),
    );
}

#[test]
fn parse() {
    let token: AccessToken = "000102030405060708090a0b0c0d0e0f".parse().unwrap();
    let expected = AccessToken::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
    p_assert_eq!(token, expected);
    p_assert_matches!(
        "#~!".parse::<AccessToken>(),
        Err(AccessTokenDecodeError::InvalidHex),
    );
    p_assert_matches!(
        "00010203".parse::<AccessToken>(),
        Err(AccessTokenDecodeError::InvalidSize { .. }),
    );
}

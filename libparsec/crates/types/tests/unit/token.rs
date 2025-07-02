// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use super::*;

macro_rules! base_token_tests {
    ($mod: ident, $name: ident) => {
        mod $mod {
            use super::*;

            #[test]
            fn default_random() {
                let token = $name::default();
                p_assert_eq!(token.as_bytes().len(), TOKEN_SIZE);
            }

            #[test]
            fn eq() {
                let token = $name::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
                let token2 = $name::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();

                p_assert_eq!(token, token2);
            }

            #[test]
            fn ne() {
                let token = $name::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
                let token2 = $name::from_hex("000102030405060708090a0b0c0d0e0e").unwrap();

                assert_ne!(token, token2);
            }

            #[test]
            fn hex() {
                let token = $name::default();

                let hex = token.hex();

                let from_hex = $name::from_hex(&hex).unwrap();

                p_assert_eq!(token, from_hex);

                p_assert_matches!($name::from_hex("#~!"), Err(TokenDecodeError::InvalidHex),);
                p_assert_matches!(
                    $name::from_hex("00010203"),
                    Err(TokenDecodeError::InvalidSize { .. }),
                );
            }

            #[test]
            fn debug() {
                let token = $name::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();

                p_assert_eq!(
                    format!("{:?}", token),
                    concat!(stringify!($name), "(\"000102030405060708090a0b0c0d0e0f\")")
                );
            }

            #[test]
            fn display() {
                let token = $name::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();

                p_assert_eq!(format!("{}", token), "000102030405060708090a0b0c0d0e0f");
            }

            #[test]
            fn try_from_str() {
                let token: $name = $name::try_from("000102030405060708090a0b0c0d0e0f").unwrap();
                let expected = $name::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();

                p_assert_eq!(token, expected);

                p_assert_matches!($name::try_from("#~!"), Err(TokenDecodeError::InvalidHex),);
                p_assert_matches!(
                    $name::try_from("00010203"),
                    Err(TokenDecodeError::InvalidSize { .. }),
                );
            }

            #[test]
            fn parse() {
                let token: $name = "000102030405060708090a0b0c0d0e0f".parse().unwrap();
                let expected = $name::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();

                p_assert_eq!(token, expected);

                p_assert_matches!("#~!".parse::<$name>(), Err(TokenDecodeError::InvalidHex),);
                p_assert_matches!(
                    "00010203".parse::<$name>(),
                    Err(TokenDecodeError::InvalidSize { .. }),
                );
            }
        }
    };
}

base_token_tests!(invitation_token, InvitationToken);
base_token_tests!(bootstrap_token, BootstrapToken);

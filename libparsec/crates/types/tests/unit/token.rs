// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::*;

macro_rules! base_token_tests {
    ($mod: ident, $name: ident) => {
        mod $mod {
            use super::*;

            #[test]
            fn test_default_random() {
                let token = $name::default();
                assert_eq!(token.as_bytes().len(), TOKEN_SIZE);
            }

            #[test]
            fn test_eq() {
                let token = $name::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
                let token2 = $name::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();

                assert_eq!(token, token2);
            }

            #[test]
            fn test_ne() {
                let token = $name::from_hex("000102030405060708090a0b0c0d0e0f").unwrap();
                let token2 = $name::from_hex("000102030405060708090a0b0c0d0e0e").unwrap();

                assert_ne!(token, token2);
            }

            #[test]
            fn test_hex() {
                let token = $name::default();

                let hex = token.hex();

                let from_hex = $name::from_hex(&hex).unwrap();

                assert_eq!(token, from_hex);
            }
        }
    };
}

base_token_tests!(invitation_token, InvitationToken);
base_token_tests!(bootstrap_token, BootstrapToken);

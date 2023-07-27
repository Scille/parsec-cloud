// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

macro_rules! impl_secret_key {
    ($key: ident) => {
        crate::impl_key_debug!($key);

        impl $key {
            /// Encode the key as a human-readable string, for instance:
            ///     D5VR-53YO-QYJW-VJ4A-4DQR-4LVC-W425-3CXN-F3AQ-J6X2-YVPZ-XBAO-NU4Q
            /// Yes, it looks like a good old CD key *insert keygen music*
            ///
            /// We want to provide the user with an easier to type format for the recovery password.
            /// We use base32 so all characters are in one case, and the alphabet contains less
            /// colliding letters (i.e. `0` and `O`) then form 4 letters groups separated by dashes.
            /// When decoding, we remove any characters not included in the alphabet (spaces, new lines, ...)
            /// and decode to get our password back.
            pub fn generate_recovery_passphrase() -> (crate::SecretKeyPassphrase, Self) {
                use ::zeroize::Zeroize;

                let key = Self::generate();

                let mut b32 =
                    ::base32::encode(::base32::Alphabet::RFC4648 { padding: false }, key.as_ref());

                // Add `-` grouping separators
                let passphrase = b32
                    .as_bytes()
                    .chunks(4)
                    .map(std::str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()
                    .expect("Unreachable because variable b32 is a utf8 string")
                    .join("-");

                b32.zeroize();

                (passphrase.into(), key)
            }

            /// `passphrase` parameter is passed mutable so that we zeroize it once used.
            pub fn from_recovery_passphrase(
                mut passphrase: crate::SecretKeyPassphrase,
            ) -> crate::CryptoResult<Self> {
                use ::zeroize::Zeroizing;

                // Lowercase is not allowed in theory, but it's too tempting to fix this here ;-)
                passphrase.make_ascii_uppercase();

                // Filter out any unknown characters, this is typically useful to remove
                // the `-` and whitespaces.
                // Note we also "correct" possible typos from the user, this is done by two means:
                // - Very common 0 vs O and 1 vs I typos are taken care of
                // - Other not allowed characters (e.g. `8` or `9`) are simply removed
                let b32 = {
                    let b32 = passphrase
                        .chars()
                        .filter_map(|c| match c {
                            '0' => Some('O'),
                            '1' => Some('I'),
                            'A'..='Z' | '2'..='7' => Some(c),
                            _ => None,
                        })
                        .collect::<String>();

                    // Note we have stripped any `=` padding character that may have been
                    // present (`::base32::decode` works just fine without them)

                    Zeroizing::new(b32)
                };

                // Actual base32 decoding
                let rawkey = {
                    let rawkey =
                        ::base32::decode(::base32::Alphabet::RFC4648 { padding: true }, &b32)
                            .expect("Always valid base32 payload");

                    Zeroizing::new(rawkey)
                };

                // Final conversion
                Self::try_from(&rawkey[..])
            }
        }

        impl AsRef<[u8]> for $key {
            fn as_ref(&self) -> &[u8] {
                &self.0.as_ref()
            }
        }

        impl TryFrom<&[u8]> for $key {
            type Error = CryptoError;
            fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
                <[u8; Self::SIZE]>::try_from(data)
                    .map(Self::from)
                    .map_err(|_| CryptoError::KeySize {
                        expected: Self::SIZE,
                        got: data.len(),
                    })
            }
        }

        impl TryFrom<&::serde_bytes::Bytes> for SecretKey {
            type Error = CryptoError;
            fn try_from(data: &Bytes) -> Result<Self, Self::Error> {
                Self::try_from(data.as_ref())
            }
        }

        impl ::serde::Serialize for SecretKey {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_bytes(self.as_ref())
            }
        }
    };
}

pub(crate) use impl_secret_key;

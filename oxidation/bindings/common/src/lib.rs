// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use base64::DecodeError;

use libparsec::crypto::SecretKey;
pub use libparsec::{create_context, RuntimeContext, StrPath};

#[cfg(not(target_arch = "wasm32"))]
use libparsec::client_types::list_available_devices;
/// We can't access file system in web environment
#[cfg(target_arch = "wasm32")]
fn list_available_devices(_config_dir: &std::path::Path) -> Result<(), ()> {
    Err(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cmd {
    Version,
    Encrypt(SecretKey, Vec<u8>),
    Decrypt(SecretKey, Vec<u8>),
    ListAvailableDevices(StrPath),
}

impl Cmd {
    pub fn decode(cmd: &str, payload: &str) -> Result<Self, String> {
        let payload = payload
            .split(':')
            .map(base64::decode)
            .collect::<Result<Vec<_>, DecodeError>>()
            .map_err(|err| format!("bad_params_encoding: {err:?}"))?;

        Ok(match (cmd, &payload[..]) {
            ("version", _) => Self::Version,
            ("encrypt", [raw_key, data]) => match SecretKey::try_from(&raw_key[..]) {
                Ok(key) => Self::Encrypt(key, data.to_vec()),
                Err(err) => return Err(format!("bad_params_value: {err:?}")),
            },
            ("decrypt", [raw_key, data]) => match SecretKey::try_from(&raw_key[..]) {
                Ok(key) => Self::Decrypt(key, data.to_vec()),
                Err(err) => return Err(format!("bad_params_value: {err:?}")),
            },
            ("list_available_devices", [config_dir]) => {
                let config_dir = std::str::from_utf8(config_dir).unwrap();
                Self::ListAvailableDevices(StrPath::from(config_dir))
            }
            (unknown_cmd, payload) => {
                return Err(format!(
                    "unknown_command or invalid_payload_length: {unknown_cmd} {}",
                    payload.len()
                ))
            }
        })
    }

    pub fn execute(self) -> Result<String, String> {
        Ok(match self {
            Self::Version => "v0.0.1".into(),
            Self::Encrypt(key, data) => {
                let encrypted = key.encrypt(&data);
                base64::encode(encrypted)
            }
            Self::Decrypt(key, data) => match key.decrypt(&data) {
                Ok(cleartext) => base64::encode(cleartext),
                Err(err) => return Err(format!("decryption_error: {err:?}")),
            },
            Self::ListAvailableDevices(config_dir) => {
                let devices = list_available_devices(&config_dir).unwrap_or_default();
                serde_json::to_string(&devices).unwrap()
            }
        })
    }
}

#[test]
fn test_version() {
    let version = Cmd::decode("version", "").unwrap().execute().unwrap();
    assert_eq!(version, "v0.0.1")
}

#[test]
fn test_encrypt_decrypt() {
    let secret = SecretKey::generate();
    let data = b"secret data";

    let payload = base64::encode(&secret) + ":" + &base64::encode(data);
    let encrypted = Cmd::decode("encrypt", &payload).unwrap().execute().unwrap();

    let payload = base64::encode(&secret) + ":" + &encrypted;
    let decrypted = Cmd::decode("decrypt", &payload).unwrap().execute().unwrap();

    let cleartext = base64::decode(decrypted).unwrap();
    assert_eq!(cleartext, b"secret data");
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use base64::DecodeError;
use libparsec::SecretKey;
pub use libparsec::{create_context, RuntimeContext};

pub fn decode_and_execute(cmd: &str, payload: &str) -> Result<String, String> {
    let version = "v0.0.1".into();
    let payload = payload
        .split(':')
        .map(base64::decode)
        .collect::<Result<Vec<_>, DecodeError>>()
        .map_err(|err| format!("bad_params_encoding: {err:?}"))?;

    match (cmd, &payload[..]) {
        ("version", _) => Ok(version),
        ("encrypt", [raw_key, data]) => match SecretKey::try_from(&raw_key[..]) {
            Ok(key) => {
                let encrypted = key.encrypt(data);
                Ok(base64::encode(encrypted))
            }
            Err(err) => Err(format!("bad_params_value: {err:?}")),
        },
        ("decrypt", [raw_key, data]) => match SecretKey::try_from(&raw_key[..]) {
            Ok(key) => match key.decrypt(data) {
                Ok(cleartext) => Ok(base64::encode(cleartext)),
                Err(err) => Err(format!("decryption_error: {err:?}")),
            },
            Err(err) => Err(format!("bad_params_value: {err:?}")),
        },
        (unknown_cmd, payload) => Err(format!(
            "unknown_command or invalid_payload_length: {unknown_cmd} {}",
            payload.len()
        )),
    }
}

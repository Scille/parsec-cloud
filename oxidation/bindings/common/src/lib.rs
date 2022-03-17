pub use libparsec::{create_context, RuntimeContext};

use libparsec::SecretKey;

pub fn decode_and_execute(cmd: &str, payload: &str) -> Result<String, String>
{
    fn _get_key_and_data(payload: &str) -> Result<(SecretKey, Vec<u8>), String> {
            let mut splitted  = payload.split(':');
            let b64_key = splitted.next();  // key as base64
            let b64_data = splitted.next();  // message as base64
            match (b64_key, b64_data, splitted.next()) {
                (Some(b64_key), Some(b64_data), None) => {
                    match (base64::decode(b64_key), base64::decode(b64_data)) {
                        (Ok(raw_key), Ok(data)) => {
                            match SecretKey::try_from(&raw_key[..]) {
                                Ok(key) => Ok((key, data)),
                                err => Err(format!("bad_params_value:{:?}", err)),
                            }
                        },
                        err => Err(format!("bad_params_encoding:{:?}", err)),
                    }
                },
                err => {
                    Err(format!("bad_params_number:{:?}", err))
                }
            }
    }

    match cmd {
        "encrypt" => {
            _get_key_and_data(payload).map(|(key, data)| {
                let encrypted = key.encrypt(&data);
                base64::encode(encrypted)
            })
        },
        "decrypt" => {
            _get_key_and_data(payload).and_then(|(key, data)| {
                match key.decrypt(&data) {
                    Ok(cleartext) => Ok(base64::encode(cleartext)),
                    Err(err) => Err(format!("decryption_error:{:?}", err)),
                }
            })
        },
        err => Err(format!("unknown_command:{}", err)),
    }
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `libparsec_zstd` is just a shim over `zstd` crate to provide a simpler-to-build
// pure Rust implementation when compiling for WASM on Windows/MacOS.
use libparsec_zstd as zstd;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatError {
    BadSerialization {
        format: Option<u8>,
        step: &'static str,
    },
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError::BadSerialization { format, step } => {
                let format_str = match format {
                    Some(format) => format!("{format}"),
                    None => "<unknown>".to_string(),
                };
                write!(
                    f,
                    "Invalid serialization: format {} step <{step}>",
                    format_str
                )
            }
        }
    }
}

impl std::error::Error for FormatError {}

/// Format v0: `0x00` + zstd(msgpack(<data>))
#[allow(dead_code)]
const FORMAT_V0_VERSION_BYTE: u8 = 0;

/// Format v0: msgpack + zstd
#[allow(dead_code)]
pub(crate) fn format_v0_dump<T>(obj: &T) -> Vec<u8>
where
    T: serde::Serialize + ?Sized,
{
    // Step 1 is msgpack serialization

    let step1_output = rmp_serde::to_vec_named(obj).expect("unexpected serialization error");

    // Step 2 is format version byte + zstd compression

    let mut step2_output = Vec::with_capacity(step1_output.len() + 1);
    step2_output.push(FORMAT_V0_VERSION_BYTE);

    let mut step2_output_cursor = std::io::Cursor::new(&mut step2_output);
    step2_output_cursor.set_position(1);

    zstd::stream::copy_encode(std::io::Cursor::new(step1_output), step2_output_cursor, 0)
        .expect("unexpected compression error");

    step2_output
}

#[allow(dead_code)]
pub(crate) fn format_vx_load<T>(raw: &[u8]) -> Result<T, FormatError>
where
    T: for<'a> serde::Deserialize<'a>,
{
    match raw.first() {
        Some(&FORMAT_V0_VERSION_BYTE) => {
            let step1_input = {
                zstd::stream::decode_all(&raw[1..]).map_err(|_| FormatError::BadSerialization {
                    format: Some(FORMAT_V0_VERSION_BYTE),
                    step: "zstd",
                })?
            };

            rmp_serde::from_slice(&step1_input).map_err(|_| FormatError::BadSerialization {
                format: Some(FORMAT_V0_VERSION_BYTE),
                step: "msgpack+validation",
            })
        }

        Some(_) | None => Err(FormatError::BadSerialization {
            format: None,
            step: "format detection",
        }),
    }
}

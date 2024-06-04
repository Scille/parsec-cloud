// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::DataError;

const FORMAT_V0_VERSION_BYTE: u8 = 0;

/// Format v0: msgpack + zstd
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

pub(crate) fn format_vx_load<T>(raw: &[u8]) -> Result<T, DataError>
where
    T: for<'a> serde::Deserialize<'a>,
{
    match raw.first() {
        Some(&FORMAT_V0_VERSION_BYTE) => {
            let step1_input =
                zstd::stream::decode_all(&raw[1..]).map_err(|_| DataError::BadSerialization {
                    format: Some(FORMAT_V0_VERSION_BYTE),
                    step: "zstd",
                })?;

            rmp_serde::from_slice(&step1_input).map_err(|_| DataError::BadSerialization {
                format: Some(FORMAT_V0_VERSION_BYTE),
                step: "msgpack+validation",
            })
        }

        Some(_) | None => Err(DataError::BadSerialization {
            format: None,
            step: "format detection",
        }),
    }
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::DataError;

#[cfg(feature = "use-zstd-in-serialization-format")]
/// Format v0: `0x00` + zstd(msgpack(<data>))
const FORMAT_V0_VERSION_BYTE: u8 = 0;
/// TODO: remove me once zstd WASM compilation on macOS/Windows is fixed
/// Temporary format for dev used as workaround for issue with zstd support in WASM:
/// Format vFF: `0xFF` + msgpack(<data>)
const FORMAT_VFF_VERSION_BYTE: u8 = 0xFF;

/// Format v0: msgpack + zstd
pub(crate) fn format_v0_dump<T>(obj: &T) -> Vec<u8>
where
    T: serde::Serialize + ?Sized,
{
    // Step 1 is msgpack serialization

    let step1_output = rmp_serde::to_vec_named(obj).expect("unexpected serialization error");

    #[cfg(feature = "use-zstd-in-serialization-format")]
    {
        // Step 2 is format version byte + zstd compression

        let mut step2_output = Vec::with_capacity(step1_output.len() + 1);
        step2_output.push(FORMAT_V0_VERSION_BYTE);

        let mut step2_output_cursor = std::io::Cursor::new(&mut step2_output);
        step2_output_cursor.set_position(1);

        zstd::stream::copy_encode(std::io::Cursor::new(step1_output), step2_output_cursor, 0)
            .expect("unexpected compression error");

        step2_output
    }

    // TODO: `use-zstd-in-serialization-format` is a temporary feature to disable the
    //       use of zstd, as its compilation with WASM is buggy on some host (e.g.
    //       MacOS/Windows/Ubuntu 20.04).
    //       TL;DR: ALWAYS ENABLE `use-zstd-in-serialization-format` FOR PRODUCTION !!!
    #[cfg(not(feature = "use-zstd-in-serialization-format"))]
    {
        // Step 2 is format version byte

        let mut step2_output = Vec::with_capacity(step1_output.len() + 1);
        step2_output.push(FORMAT_VFF_VERSION_BYTE);

        let mut step2_output_cursor = std::io::Cursor::new(&mut step2_output);
        step2_output_cursor.set_position(1);

        use std::io::Write;

        step2_output_cursor
            .write_all(&step1_output)
            .expect("write to vec never fails");

        step2_output
    }
}

pub(crate) fn format_vx_load<T>(raw: &[u8]) -> Result<T, DataError>
where
    T: for<'a> serde::Deserialize<'a>,
{
    match raw.first() {
        #[cfg(feature = "use-zstd-in-serialization-format")]
        Some(&FORMAT_V0_VERSION_BYTE) => {
            let step1_input = {
                zstd::stream::decode_all(&raw[1..]).map_err(|_| DataError::BadSerialization {
                    format: Some(FORMAT_V0_VERSION_BYTE),
                    step: "zstd",
                })?
            };

            rmp_serde::from_slice(&step1_input).map_err(|_| DataError::BadSerialization {
                format: Some(FORMAT_V0_VERSION_BYTE),
                step: "msgpack+validation",
            })
        }

        // Allow decoding of vFF format even if `use-zstd-in-serialization-format` is enabled,
        // this is needed to be able to communicate between a wasm client and the non-wasm
        // testbed server (note the trick here is that the data are only produced by the
        // client, so the client doesn't have to support format v0 as long as there is
        // only wasm client which is reasonable in a dev scenario).
        Some(&FORMAT_VFF_VERSION_BYTE) => {
            rmp_serde::from_slice(&raw[1..]).map_err(|_| DataError::BadSerialization {
                format: Some(FORMAT_VFF_VERSION_BYTE),
                step: "msgpack+validation",
            })
        }

        Some(_) | None => Err(DataError::BadSerialization {
            format: None,
            step: "format detection",
        }),
    }
}

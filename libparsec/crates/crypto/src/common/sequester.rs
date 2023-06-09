// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use crate::{CryptoError, CryptoResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum SequesterKeySize {
    _1024Bits = 1024,
    _2048Bits = 2048,
    _3072Bits = 3072,
    _4096Bits = 4096,
}

/// Here we avoid unnecessary allocation & enforce output has `key_size`
pub(crate) fn serialize_with_armor(
    output: &[u8],
    data: &[u8],
    key_size_bytes: usize,
    algo: &str,
) -> Vec<u8> {
    // It is important `output` has a constant size given this
    // is how it is retrieved during decryption
    assert!(output.len() <= key_size_bytes);
    let mut res = vec![0; algo.len() + 1 + key_size_bytes + data.len()];

    let (algorithm_part, others) = res.split_at_mut(algo.len());
    let (colon, others) = others.split_at_mut(1);
    // Here we enforce output has key size with zeros
    // Using RSA, we should end up with a number as big as the key size and
    // provided as big endian bytes.
    // However it is possible the number can be represented with less bytes if
    // enough of its most significant bits are equal to zero.
    // In such case, certain implementations (at least oscrypto on macOS 12) trim
    // the null bytes and hence return a bytes array smaller than the key size.
    //
    // For instance, considering a 16bits RSA key size (state-of-the-art security ^^)
    // and a RSA output `42`, the output can be represented as b"\x42" or b"\x00\x42)
    //
    // Long story short, we want to make sure our RSA output are always of the
    // same size than the key, this simplify splitting messages and protect us
    // if an RSA implementation on another platform is picky about output size.
    let (_zeros, others) = others.split_at_mut(key_size_bytes - output.len());
    let (output_part, data_part) = others.split_at_mut(output.len());

    algorithm_part.copy_from_slice(algo.as_bytes());
    colon[0] = b':';
    output_part.copy_from_slice(output);
    data_part.copy_from_slice(data);

    res
}

pub(crate) fn deserialize_with_armor<'a>(
    data: &'a [u8],
    key_size_bytes: usize,
    algo: &str,
) -> CryptoResult<(&'a [u8], &'a [u8])> {
    let index = data
        .iter()
        .position(|x| *x == b':')
        .ok_or(CryptoError::Decryption)?;
    let (algorithm, output_and_data) = data.split_at(index + 1);

    if &algorithm[..index] != algo.as_bytes() {
        return Err(CryptoError::Algorithm(
            String::from_utf8_lossy(&algorithm[..index]).into(),
        ));
    }

    Ok(output_and_data.split_at(key_size_bytes))
}

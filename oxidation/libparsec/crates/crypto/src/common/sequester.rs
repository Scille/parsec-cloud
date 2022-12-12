// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use crate::{CryptoError, CryptoResult};

pub(crate) trait EnforceSerialize {
    const ALGORITHM: &'static [u8];
    fn size_in_bytes(&self) -> usize;

    /// Here we avoid uncessary allocation & enforce output has `key_size`
    fn serialize(&self, output: &[u8], data: &[u8]) -> Vec<u8> {
        assert!(output.len() <= self.size_in_bytes());
        let mut res = vec![0; Self::ALGORITHM.len() + 1 + self.size_in_bytes() + data.len()];

        let (algorithm_part, others) = res.split_at_mut(Self::ALGORITHM.len());
        let (colon, others) = others.split_at_mut(1);
        // Here we enforce output has key size with zeros
        let (_zeros, others) = others.split_at_mut(self.size_in_bytes() - output.len());
        let (output_part, data_part) = others.split_at_mut(output.len());

        algorithm_part.copy_from_slice(Self::ALGORITHM);
        colon[0] = b':';
        output_part.copy_from_slice(output);
        data_part.copy_from_slice(data);

        res
    }
}

pub(crate) trait EnforceDeserialize {
    const ALGORITHM: &'static [u8];
    fn size_in_bytes(&self) -> usize;

    fn deserialize<'a>(&self, data: &'a [u8]) -> CryptoResult<(&'a [u8], &'a [u8])> {
        let index = data
            .iter()
            .position(|x| *x == b':')
            .ok_or(CryptoError::Decryption)?;
        let (algo, output_and_data) = data.split_at(index + 1);

        if &algo[..index] != Self::ALGORITHM {
            return Err(CryptoError::Algorithm(
                String::from_utf8_lossy(&algo[..index]).into(),
            ));
        }

        Ok(output_and_data.split_at(self.size_in_bytes()))
    }
}

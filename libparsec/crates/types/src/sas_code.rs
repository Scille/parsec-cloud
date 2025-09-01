// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::str::FromStr;

use rand::{seq::SliceRandom, Rng};

use libparsec_crypto::SecretKey;

// (Note I/1 and 0/O are skipped to avoid visual confusion)
const SAS_CODE_CHARS: &[u8; 32] = crate::BASE32_ALPHABET;
const SAS_CODE_LEN: usize = 4;
const SAS_CODE_BITS: usize = 20;
const SAS_CODE_MASK: usize = (1 << SAS_CODE_BITS) - 1;
const SAS_SUBCODE_BITS: usize = 5;
const SAS_SUBCODE_MASK: usize = (1 << SAS_SUBCODE_BITS) - 1;

/// SAS code is composed of 4 hexadecimal characters
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct SASCode(String);

impl std::fmt::Display for SASCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Clone, Debug)]
pub struct SASCodeParseError;

impl std::error::Error for SASCodeParseError {}

impl std::fmt::Display for SASCodeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid SAS code")
    }
}

impl FromStr for SASCode {
    type Err = SASCodeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valid = s.len() == 4 && s.as_bytes().iter().all(|c| SAS_CODE_CHARS.contains(c));

        if valid {
            Ok(Self(s.to_string()))
        } else {
            Err(SASCodeParseError)
        }
    }
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Clone, Copy)]
pub struct SASCodeValueTooLarge;

impl std::error::Error for SASCodeValueTooLarge {}

impl std::fmt::Display for SASCodeValueTooLarge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Provided integer is too large")
    }
}

impl TryFrom<u32> for SASCode {
    type Error = SASCodeValueTooLarge;
    fn try_from(num: u32) -> Result<SASCode, Self::Error> {
        let mut num = num as usize;
        if num >= 1 << SAS_CODE_BITS {
            // The valid range number should not exceed 20 bit long
            // because subcode is 5 bits long (remainder by SAS_CODE_CHARS.len() [32])
            // and SAS_CODE_LEN is 4
            return Err(SASCodeValueTooLarge);
        }

        let mut str = String::with_capacity(SAS_CODE_LEN);

        for _ in 0..SAS_CODE_LEN {
            let subcode = num & SAS_SUBCODE_MASK;
            let char = SAS_CODE_CHARS[subcode] as char;
            str.push(char);
            num >>= SAS_SUBCODE_BITS;
        }

        Ok(Self(str))
    }
}

impl std::convert::AsRef<str> for SASCode {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<SASCode> for String {
    fn from(item: SASCode) -> String {
        item.0
    }
}

impl SASCode {
    pub fn generate_sas_code_candidates(&self, size: usize) -> Vec<SASCode> {
        if size == 0 {
            return vec![];
        }

        let mut sas_codes = Vec::<SASCode>::with_capacity(size);

        sas_codes.push(SASCode(self.to_string()));
        while sas_codes.len() < size {
            let num = rand::thread_rng().gen_range(0..=SAS_CODE_MASK);
            let candidate = SASCode::try_from(num as u32)
                .expect("SASCode number should not exceed `SAS_CODE_BITS` bit long");
            if &candidate != self {
                sas_codes.push(candidate);
            }
        }
        sas_codes.shuffle(&mut rand::thread_rng());
        sas_codes
    }

    pub fn generate_sas_codes(
        claimer_nonce: &[u8],
        greeter_nonce: &[u8],
        shared_secret_key: &SecretKey,
    ) -> (SASCode, SASCode) {
        // Computes combined HMAC
        let mut combined_nonce = Vec::with_capacity(claimer_nonce.len() + greeter_nonce.len());
        combined_nonce.extend_from_slice(claimer_nonce);
        combined_nonce.extend_from_slice(greeter_nonce);

        // Digest size of 5 bytes so we can split it between two 20bits SAS
        // Note we have to store is as a 8bytes array to be able to convert it into u64
        let mut combined_hmac = [0; 8];
        // Blake2b is technically able to produce a digest of any size between 1 and 64 bytes,
        // however libsodium only allows a minimum of 16 bytes.
        // We need only 5 bytes here, but it's perfectly safe to truncate a larger digest.
        // Hence we just compute a 64 bytes digest (most convenient size since it is already
        // used elsewhere in the codebase).
        combined_hmac[3..8].copy_from_slice(&shared_secret_key.mac_512(&combined_nonce)[..5]);
        let hmac_as_int = u64::from_be_bytes(combined_hmac);

        let claimer_part_as_int = (hmac_as_int & SAS_CODE_MASK as u64) as u32;
        let greeter_part_as_int = ((hmac_as_int >> SAS_CODE_BITS) & SAS_CODE_MASK as u64) as u32;

        // Big endian number extracted from bits [0, 20[
        let claimer_sas = SASCode::try_from(claimer_part_as_int)
            .expect("SASCode number should not exceed `SAS_CODE_BITS` bit long");
        // Big endian number extracted from bits [20, 40[
        let greeter_sas = SASCode::try_from(greeter_part_as_int)
            .expect("SASCode number should not exceed `SAS_CODE_BITS` bit long");

        (claimer_sas, greeter_sas)
    }
}

#[cfg(test)]
#[path = "../tests/unit/sas_code.rs"]
mod tests;

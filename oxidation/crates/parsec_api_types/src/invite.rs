// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use super::ext_types::new_uuid_type;
use rand::Rng;
use regex::Regex;
use std::str::FromStr;

use parsec_api_crypto::SecretKey;

/*
 * InvitationType
 */

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InvitationType {
    User,
    Device,
}

impl FromStr for InvitationType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "USER" => Ok(Self::User),
            "DEVICE" => Ok(Self::Device),
            _ => Err("Invalid InvitationType"),
        }
    }
}

impl ToString for InvitationType {
    fn to_string(&self) -> String {
        match self {
            Self::User => String::from("USER"),
            Self::Device => String::from("DEVICE"),
        }
    }
}

/*
 * InvitationToken
 */

new_uuid_type!(pub InvitationToken);

/*
 * SASCode
 */

// SAS code is composed of 4 hexadecimal characters
macro_rules! SAS_CODE_CHARS {
    () => {
        "ABCDEFGHJKLMNPQRSTUVWXYZ23456789"
    };
}
const SAS_CODE_CHARS: &str = SAS_CODE_CHARS!();
const SAS_CODE_PATTERN: &str = concat!("^[", SAS_CODE_CHARS!(), "]{4}$");
const SAS_CODE_LEN: usize = 4;
const SAS_CODE_BITS: u32 = 20;

#[derive(Debug, PartialEq, Eq)]
pub struct SASCode(String);

impl std::fmt::Display for SASCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for SASCode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(SAS_CODE_PATTERN).unwrap();
        }
        if PATTERN.is_match(s) {
            Ok(Self(s.to_string()))
        } else {
            Err("Invalid SAS code")
        }
    }
}

impl TryFrom<u32> for SASCode {
    type Error = &'static str;
    fn try_from(mut num: u32) -> Result<SASCode, Self::Error> {
        let mut str = String::with_capacity(SAS_CODE_LEN);

        for _ in 0..SAS_CODE_LEN {
            let subcode = num % SAS_CODE_CHARS.len() as u32;
            let char = SAS_CODE_CHARS.chars().nth(subcode as usize).unwrap();
            str.push(char);
            num /= SAS_CODE_CHARS.len() as u32;
        }
        if num != 0 {
            Err("Provided integer is too large")
        } else {
            Ok(Self(str))
        }
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
        let mut sas_codes = vec![];
        while sas_codes.len() < size {
            let num = rand::thread_rng().gen_range(0..(2u32.pow(SAS_CODE_BITS) - 1));
            let candidate = SASCode::try_from(num).unwrap_or_else(|_| unreachable!());
            if &candidate != self {
                sas_codes.push(candidate);
            }
        }
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

        // Digest size of 5 bytes so we can split it beween two 20bits SAS
        // Note we have to store is as a 8bytes array to be able to convert it into u64
        let mut combined_hmac = [0; 8];
        combined_hmac[3..8].clone_from_slice(&shared_secret_key.hmac(&combined_nonce, 5)[..]);
        let hmac_as_int = u64::from_be_bytes(combined_hmac);

        let claimer_part_as_int = (hmac_as_int % 2u64.pow(SAS_CODE_BITS) as u64) as u32;
        let greeter_part_as_int =
            ((hmac_as_int >> SAS_CODE_BITS) % 2u64.pow(SAS_CODE_BITS) as u64) as u32;

        // Big endian number extracted from bits [0, 20[
        let claimer_sas = SASCode::try_from(claimer_part_as_int).unwrap_or_else(|_| unreachable!());
        // Big endian number extracted from bits [20, 40[
        let greeter_sas = SASCode::try_from(greeter_part_as_int).unwrap_or_else(|_| unreachable!());

        (claimer_sas, greeter_sas)
    }
}

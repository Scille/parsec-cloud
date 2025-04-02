// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
// cspell:words AIAA KBWM

use libparsec_tests_lite::prelude::*;

use crate::{SASCode, SASCodeParseError, SecretKey};

use super::SASCodeValueTooLarge;

#[test]
fn generate() {
    let claimer_nonce = hex!(
        "f4b8b6327c5d49580d34fdb0512cbcb9b794511eb33631258dfdbcf7ebccc875909e79"
        "ae346d6eefbf775fed9fe9027cae0f56bb9269f0aae3eaf614cd89e77e"
    );
    let greeter_nonce = hex!(
        "031a533ea63a2d85f5ca69720dfdde2799982aebff22cca1d97b433770492f291721c5"
        "9b088afcecb80385ce313b58902ea3bac3857856194af4d08c8f0b8c3c"
    );
    let shared_secret_key = SecretKey::from(hex!(
        "1452fb69a5eed5c1e432c3e666d7b17bdf21b33088086682ddc45b6fa33a0460"
    ));

    let (claimer_sas, greeter_sas) =
        SASCode::generate_sas_codes(&claimer_nonce, &greeter_nonce, &shared_secret_key);

    p_assert_eq!(claimer_sas.as_ref(), "25PA");
    p_assert_eq!(greeter_sas.as_ref(), "KBWM");

    p_assert_eq!(claimer_sas, "25PA".parse().unwrap());
    p_assert_eq!(greeter_sas, "KBWM".parse().unwrap());
}

#[rstest]
#[case::min(0, Ok("AAAA".parse().unwrap()))]
#[case::typical(123456, Ok("AU2D".parse().unwrap()))]
#[case::max(2u32.pow(20) - 1, Ok("9999".parse().unwrap()))]
#[case::too_large(2u32.pow(20), Err(SASCodeValueTooLarge))]
fn from_int(#[case] val: u32, #[case] result: Result<SASCode, SASCodeValueTooLarge>) {
    p_assert_eq!(SASCode::try_from(val), result);
}

#[test]
fn from_str_good() {
    p_assert_eq!("AAAA".parse::<SASCode>().unwrap().as_ref(), "AAAA");
    p_assert_eq!("9999".parse::<SASCode>().unwrap().as_ref(), "9999");
}

#[rstest]
#[case::too_short("AAA")]
#[case::too_long("AAAAA")]
#[case::bad_char_i("AIAA")]
#[case::bad_char_1("AA1A")]
#[case::bad_char_bang("#AAA")]
fn from_str_bad(#[case] val: &str) {
    p_assert_eq!(val.parse::<SASCode>(), Err(SASCodeParseError))
}

#[test]
fn generate_sas_code_candidates() {
    let sas: SASCode = "25PA".parse().unwrap();

    p_assert_eq!(sas.generate_sas_code_candidates(0), vec![]);

    p_assert_eq!(sas.generate_sas_code_candidates(1), vec![sas.clone()]);

    let candidates = sas.generate_sas_code_candidates(10);
    p_assert_eq!(candidates.len(), 10);
    assert!(candidates.contains(&sas));
}

#[test]
fn from_sas_into_string() {
    let sas: SASCode = "25PA".parse().unwrap();

    p_assert_eq!(String::from(sas), "25PA".to_string());
}

#[test]
fn debug() {
    let sas: SASCode = "25PA".parse().unwrap();

    p_assert_eq!(format!("{:?}", sas), "SASCode(\"25PA\")".to_string());
}

#[test]
fn display() {
    let sas: SASCode = "25PA".parse().unwrap();

    p_assert_eq!(format!("{}", sas), "25PA".to_string());
}

#[test]
fn eq_and_partial_ord() {
    let sas1: SASCode = "25PA".parse().unwrap();
    let sas2: SASCode = "25PA".parse().unwrap();
    let sas3: SASCode = "35PA".parse().unwrap();

    p_assert_eq!(sas1, sas2);
    p_assert_ne!(sas1, sas3);
    assert!(sas1 < sas3);
}

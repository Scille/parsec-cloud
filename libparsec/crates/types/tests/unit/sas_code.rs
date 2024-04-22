// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use crate::{SASCode, SecretKey};

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
#[case::too_large(2u32.pow(20), Err("Provided integer is too large"))]
fn from_int(#[case] val: u32, #[case] result: Result<SASCode, &'static str>) {
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
    p_assert_matches!(
        val.parse::<SASCode>(),
        Err(err) if err == "Invalid SAS code"
    )
}

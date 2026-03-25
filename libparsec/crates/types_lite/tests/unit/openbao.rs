// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use super::OpenBaoAuthType;

#[test]
fn open_bao_auth_type() {
    let raw = OpenBaoAuthType::Hexagone.as_str();
    p_assert_matches!(
        OpenBaoAuthType::try_from(raw).unwrap(),
        OpenBaoAuthType::Hexagone
    );

    let raw = OpenBaoAuthType::ProConnect.as_str();
    p_assert_matches!(
        OpenBaoAuthType::try_from(raw).unwrap(),
        OpenBaoAuthType::ProConnect
    );

    p_assert_matches!(OpenBaoAuthType::try_from("DUMMY"), Err(()));
}

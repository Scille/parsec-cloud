// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::sorted_drive_letters;

use libparsec_tests_fixtures::prelude::*;

#[test]
fn ok() {
    p_assert_eq!(
        sorted_drive_letters(0, 5).take(5).collect::<String>(),
        // cspell:disable-next-line
        "PUZLQ"
    );

    p_assert_eq!(
        sorted_drive_letters(1, 5).take(5).collect::<String>(),
        // cspell:disable-next-line
        "QVHMR"
    );

    p_assert_eq!(
        sorted_drive_letters(2, 5).take(5).collect::<String>(),
        // cspell:disable-next-line
        "RWINS"
    );

    p_assert_eq!(
        sorted_drive_letters(3, 5).take(5).collect::<String>(),
        // cspell:disable-next-line
        "SXJOT"
    );

    p_assert_eq!(
        sorted_drive_letters(4, 5).take(5).collect::<String>(),
        // cspell:disable-next-line
        "TYKPU"
    );
}

#[test]
fn same_within_grouping() {
    let g1 = sorted_drive_letters(0, 1).collect::<Vec<_>>();
    let g2 = sorted_drive_letters(0, 2).collect::<Vec<_>>();
    let g3 = sorted_drive_letters(0, 3).collect::<Vec<_>>();
    let g4 = sorted_drive_letters(0, 4).collect::<Vec<_>>();
    let g5 = sorted_drive_letters(0, 5).collect::<Vec<_>>();
    let g6 = sorted_drive_letters(0, 6).collect::<Vec<_>>();

    p_assert_eq!(g1, g5);
    p_assert_eq!(g2, g5);
    p_assert_eq!(g3, g5);
    p_assert_eq!(g4, g5);
    p_assert_ne!(g5, g6);
}

use std::{num::NonZeroU8, str::FromStr};

use libparsec::{tmp_path, TmpPath};
use predicates::prelude::PredicateBooleanExt;

use crate::{
    bootstrap_cli_test,
    testenv_utils::{TestOrganization, DEFAULT_DEVICE_PASSWORD},
};

use parsec_cli::commands::shared_recovery::create::WeightedEmail;
use parsec_cli::commands::shared_recovery::create::WeightedEmailParseError;

#[rstest::rstest]
#[tokio::test]
async fn weighted_email_from_str() {
    let expected = Ok(WeightedEmail {
        email: "alice@example.com".parse().unwrap(),
        weight: NonZeroU8::new(1).expect("always valid"),
    });
    // Explicit call
    assert_eq!(WeightedEmail::from_str("alice@example.com:1"), expected);
    assert_eq!(WeightedEmail::from_str("alice@example.com"), expected);
    // Implicit calls, through parse
    assert_eq!("alice@example.com:1".parse(), expected);
    assert_eq!("alice@example.com".parse(), expected);
    assert_eq!("alice@example.com:1".parse::<WeightedEmail>(), expected);
    assert_eq!("alice@example.com".parse::<WeightedEmail>(), expected);
    // Invalid input string
    assert_eq!(
        // cspell: disable-next-line
        WeightedEmail::from_str("aliceexample.com:1"),
        Err(WeightedEmailParseError::InvalidEmail)
    );
    assert_eq!(
        WeightedEmail::from_str("alice@example.com:a"),
        Err(WeightedEmailParseError::InvalidWeight)
    );
    assert_eq!(
        WeightedEmail::from_str("alice@example.com:"),
        Err(WeightedEmailParseError::InvalidWeight)
    );
}

#[rstest::rstest]
#[tokio::test]
async fn create_shared_recovery_ok(tmp_path: TmpPath) {
    let (
        _,
        TestOrganization {
            alice, bob, toto, ..
        },
        _,
    ) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "create",
        "--device",
        &alice.device_id.hex(),
        "--recipients",
        &bob.human_handle.email().to_string(),
        &toto.human_handle.email().to_string(),
        "--threshold",
        "1",
        "--no-confirmation"
    )
    .stdout(predicates::str::contains(
        "Shared recovery setup has been created",
    ));
}

#[rstest::rstest]
#[tokio::test]
async fn create_shared_recovery_with_weights(tmp_path: TmpPath) {
    let (
        _,
        TestOrganization {
            alice, bob, toto, ..
        },
        _,
    ) = bootstrap_cli_test(&tmp_path).await.unwrap();

    crate::assert_cmd_success!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "create",
        "--device",
        &alice.device_id.hex(),
        "--recipients",
        &format!("{}:{}", &bob.human_handle.email(), "2"),
        &format!("{}:{}", &toto.human_handle.email(), "3"),
        "--threshold",
        "2",
        "--no-confirmation"
    )
    .stdout(
        predicates::str::contains("Shared recovery setup has been created")
            .and(predicates::str::contains(format!(
                "• User {} will have 2 shares",
                &bob.human_handle
            )))
            .and(predicates::str::contains(format!(
                "• User {} will have 3 shares",
                &toto.human_handle
            ))),
    );
}

#[rstest::rstest]
#[tokio::test]
async fn create_shared_recovery_inexistent_email(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();
    // a non existent recipient

    crate::assert_cmd_failure!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "create",
        "--device",
        &alice.device_id.hex(),
        "--recipients",
        "not-here@example.com",
        "--threshold",
        "1",
        "--no-confirmation"
    )
    .stderr(predicates::str::contains(
        "A user with email not-here@example.com not found",
    ));
}

#[cfg(target_family = "unix")] // rexpect doesn't support Windows
#[rstest::rstest]
#[tokio::test]
async fn create_shared_recovery_default(tmp_path: TmpPath) {
    let (_, TestOrganization { bob, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();
    let cmd = crate::std_cmd!(
        "shared-recovery",
        "create",
        "--device",
        &bob.device_id.hex()
    );

    let mut p = crate::spawn_interactive_command(cmd, Some(1500)).unwrap();

    p.exp_string("Enter password for the device:").unwrap();
    p.send_line(DEFAULT_DEVICE_PASSWORD).unwrap();

    // Wait for the intermediate spinners to make rexpect wait a bunch to prevent timeout when waiting for the "threshold" prompt
    p.exp_regex(".*Poll server for new certificates.*").unwrap();
    p.exp_regex(".*Creating shared recovery setup.*").unwrap();

    p.exp_regex(".*The threshold is the minimum number of shares that one must gather to recover the account.*").unwrap();
    p.send_line("1").unwrap();
    p.exp_string("The following shared recovery setup will be created")
        .unwrap();
    p.send_line("y").unwrap();
    p.exp_regex(".*Shared recovery setup has been created.*")
        .unwrap();
    p.exp_eof().unwrap();
}

#[rstest::rstest]
#[tokio::test]
async fn create_shared_recovery_no_recipient(tmp_path: TmpPath) {
    let (_, TestOrganization { alice, .. }, _) = bootstrap_cli_test(&tmp_path).await.unwrap();

    // Alice is the only admin
    crate::assert_cmd_failure!(
        with_password = DEFAULT_DEVICE_PASSWORD,
        "shared-recovery",
        "create",
        "--device",
        &alice.device_id.hex()
    )
    .stderr(predicates::str::contains("No default recipient available"));
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use rstest::rstest;

use libparsec_core::{TrustchainContext, TrustchainError};
use libparsec_protocol::authenticated_cmds::v2::user_get::Trustchain;
use libparsec_types::{
    CertificateSignerOwned, DateTime, DeviceCertificate, DeviceID, RevokedUserCertificate,
    UserCertificate, UserProfile,
};

use tests_fixtures::{
    alice, alice_device_certif, alice_revoked_user_certif, alice_user_certif, bob,
    bob_device_certif, bob_user_certif, coolorg, mallory, mallory_device_certif,
    mallory_revoked_user_certif, mallory_user_certif, Device, Organization,
};

#[rstest]
fn test_bad_expected_user(
    alice: &Device,
    bob: &Device,
    alice_user_certif: &UserCertificate,
    alice_device_certif: &DeviceCertificate,
    coolorg: &Organization,
) {
    let mut ctx = TrustchainContext::new(
        alice.root_verify_key().clone(),
        alice.time_provider.clone(),
        1,
    );

    let err = ctx
        .load_user_and_devices(
            Trustchain {
                users: vec![],
                devices: vec![],
                revoked_users: vec![],
            },
            alice_user_certif.dump_and_sign(&coolorg.signing_key),
            None,
            vec![alice_device_certif.dump_and_sign(&coolorg.signing_key)],
            Some(bob.user_id()),
        )
        .unwrap_err();

    assert_eq!(
        err,
        TrustchainError::UnexpectedCertificate {
            expected: bob.user_id().clone(),
            got: alice.user_id().clone()
        }
    );
}

#[rstest]
fn test_verify_no_trustchain(
    alice: &Device,
    alice_user_certif: &UserCertificate,
    alice_device_certif: &DeviceCertificate,
    coolorg: &Organization,
) {
    let mut ctx = TrustchainContext::new(
        alice.root_verify_key().clone(),
        alice.time_provider.clone(),
        1,
    );

    let mut alice2_device_certif = alice_device_certif.clone();
    let alice2_device_id: DeviceID = "alice@dev2".parse().unwrap();
    alice2_device_certif.author = CertificateSignerOwned::User(alice.device_id.clone());
    alice2_device_certif.device_id = alice2_device_id.clone();

    let mut alice3_device_certif = alice_device_certif.clone();
    let alice3_device_id: DeviceID = "alice@dev3".parse().unwrap();
    alice3_device_certif.author = CertificateSignerOwned::User(alice2_device_id.clone());
    alice3_device_certif.device_id = alice3_device_id.clone();

    let (user, revoked_user, devices) = ctx
        .load_user_and_devices(
            Trustchain {
                users: vec![],
                devices: vec![],
                revoked_users: vec![],
            },
            alice_user_certif.dump_and_sign(&coolorg.signing_key),
            None,
            vec![
                alice_device_certif.dump_and_sign(&coolorg.signing_key),
                alice2_device_certif.dump_and_sign(&alice.signing_key),
                alice3_device_certif.dump_and_sign(&alice.signing_key),
            ],
            Some(alice.user_id()),
        )
        .unwrap();

    assert_eq!(&user, ctx.get_user(alice.user_id()).unwrap());
    assert_eq!(revoked_user, None);

    let devices = devices.iter().collect::<Vec<_>>();
    assert!(devices.contains(&ctx.get_device(&alice.device_id).unwrap()));
    assert!(devices.contains(&ctx.get_device(&alice2_device_id).unwrap()));
    assert!(devices.contains(&ctx.get_device(&alice3_device_id).unwrap()));
}

#[rstest]
fn test_bad_user_self_signed(
    alice: &Device,
    alice_user_certif: &UserCertificate,
    alice_device_certif: &DeviceCertificate,
    coolorg: &Organization,
) {
    let mut ctx = TrustchainContext::new(
        alice.root_verify_key().clone(),
        alice.time_provider.clone(),
        1,
    );

    let mut alice_user_certif = alice_user_certif.clone();
    alice_user_certif.author = CertificateSignerOwned::User(alice.device_id.clone());

    let err = ctx
        .load_user_and_devices(
            Trustchain {
                users: vec![],
                devices: vec![],
                revoked_users: vec![],
            },
            alice_user_certif.dump_and_sign(&coolorg.signing_key),
            None,
            vec![alice_device_certif.dump_and_sign(&coolorg.signing_key)],
            Some(alice.user_id()),
        )
        .unwrap_err();

    assert_eq!(
        err,
        TrustchainError::InvalidSelfSignedUserCertificate {
            user_id: alice.user_id().clone(),
        }
    );
}

#[rstest]
fn test_bad_revoked_user_self_signed(
    alice: &Device,
    alice_user_certif: &UserCertificate,
    alice_device_certif: &DeviceCertificate,
    alice_revoked_user_certif: &RevokedUserCertificate,
    coolorg: &Organization,
) {
    let mut ctx = TrustchainContext::new(
        alice.root_verify_key().clone(),
        alice.time_provider.clone(),
        1,
    );
    let mut alice_revoked_user_certif = alice_revoked_user_certif.clone();
    alice_revoked_user_certif.author = alice.device_id.clone();

    let err = ctx
        .load_user_and_devices(
            Trustchain {
                users: vec![],
                devices: vec![],
                revoked_users: vec![],
            },
            alice_user_certif.dump_and_sign(&coolorg.signing_key),
            Some(alice_revoked_user_certif.dump_and_sign(&alice.signing_key)),
            vec![alice_device_certif.dump_and_sign(&coolorg.signing_key)],
            Some(alice.user_id()),
        )
        .unwrap_err();

    assert_eq!(
        err,
        TrustchainError::InvalidSelfSignedUserRevocationCertificate {
            user_id: alice.user_id().clone(),
        }
    );
}

#[rstest]
fn test_invalid_loop_on_device_certif_trustchain_error(
    alice: &Device,
    bob: &Device,
    alice_user_certif: &UserCertificate,
    alice_device_certif: &DeviceCertificate,
    bob_device_certif: &DeviceCertificate,
    coolorg: &Organization,
) {
    let mut ctx = TrustchainContext::new(
        alice.root_verify_key().clone(),
        alice.time_provider.clone(),
        1,
    );

    let mut bob_device_certif = bob_device_certif.clone();
    bob_device_certif.author = CertificateSignerOwned::User(alice.device_id.clone());

    let mut alice_device_loop_certif = alice_device_certif.clone();
    alice_device_loop_certif.author = CertificateSignerOwned::User(bob.device_id.clone());

    let err = ctx
        .load_user_and_devices(
            Trustchain {
                devices: vec![bob_device_certif.dump_and_sign(&alice.signing_key)],
                users: vec![],
                revoked_users: vec![],
            },
            alice_user_certif.dump_and_sign(&coolorg.signing_key),
            None,
            vec![
                alice_device_certif.dump_and_sign(&coolorg.signing_key),
                alice_device_loop_certif.dump_and_sign(&bob.signing_key),
            ],
            Some(alice.user_id()),
        )
        .unwrap_err();

    assert!(
        err == TrustchainError::InvalidSignatureLoopDetected {
            path: "alice@dev1 <-sign- bob@dev1 <-sign- alice@dev1".into(),
        } || err
            == TrustchainError::InvalidSignatureLoopDetected {
                path: "bob@dev1 <-sign- alice@dev1 <-sign- bob@dev1".into(),
            }
    );
}

#[rstest]
#[allow(clippy::too_many_arguments)]
fn test_device_signature_while_revoked(
    alice: &Device,
    bob: &Device,
    mallory: &Device,
    alice_user_certif: &UserCertificate,
    bob_user_certif: &UserCertificate,
    mallory_user_certif: &UserCertificate,
    alice_device_certif: &DeviceCertificate,
    bob_device_certif: &DeviceCertificate,
    mallory_device_certif: &DeviceCertificate,
    alice_revoked_user_certif: &RevokedUserCertificate,
    coolorg: &Organization,
) {
    let d1 = DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap();
    let d2 = DateTime::from_ymd_hms_us(2000, 1, 2, 0, 0, 0, 0).unwrap();
    let mut ctx = TrustchainContext::new(
        mallory.root_verify_key().clone(),
        mallory.time_provider.clone(),
        1,
    );

    let mut mallory_device_certif = mallory_device_certif.clone();
    mallory_device_certif.author = CertificateSignerOwned::User(alice.device_id.clone());
    mallory_device_certif.timestamp = d2;

    let err = ctx
        .load_user_and_devices(
            Trustchain {
                devices: vec![
                    alice_device_certif.dump_and_sign(&coolorg.signing_key),
                    bob_device_certif.dump_and_sign(&coolorg.signing_key),
                ],
                users: vec![
                    alice_user_certif.dump_and_sign(&coolorg.signing_key),
                    bob_user_certif.dump_and_sign(&coolorg.signing_key),
                ],
                revoked_users: vec![alice_revoked_user_certif.dump_and_sign(&bob.signing_key)],
            },
            mallory_user_certif.dump_and_sign(&coolorg.signing_key),
            None,
            vec![mallory_device_certif.dump_and_sign(&alice.signing_key)],
            Some(mallory.user_id()),
        )
        .unwrap_err();

    assert_eq!(
        err,
        TrustchainError::SignaturePosteriorUserRevocation {
            path: "mallory@dev1 <-sign- alice@dev1".into(),
            verified_timestamp: d2,
            user_timestamp: d1,
        }
    );
}

#[rstest]
#[allow(clippy::too_many_arguments)]
fn test_user_signature_while_revoked(
    alice: &Device,
    bob: &Device,
    mallory: &Device,
    alice_user_certif: &UserCertificate,
    bob_user_certif: &UserCertificate,
    mallory_user_certif: &UserCertificate,
    alice_device_certif: &DeviceCertificate,
    bob_device_certif: &DeviceCertificate,
    mallory_device_certif: &DeviceCertificate,
    alice_revoked_user_certif: &RevokedUserCertificate,
    coolorg: &Organization,
) {
    let d1 = DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap();
    let d2 = DateTime::from_ymd_hms_us(2000, 1, 2, 0, 0, 0, 0).unwrap();
    let mut ctx = TrustchainContext::new(
        mallory.root_verify_key().clone(),
        mallory.time_provider.clone(),
        1,
    );

    let mut mallory_user_certif = mallory_user_certif.clone();
    mallory_user_certif.author = CertificateSignerOwned::User(alice.device_id.clone());
    mallory_user_certif.timestamp = d2;

    let err = ctx
        .load_user_and_devices(
            Trustchain {
                devices: vec![
                    alice_device_certif.dump_and_sign(&coolorg.signing_key),
                    bob_device_certif.dump_and_sign(&coolorg.signing_key),
                ],
                users: vec![
                    alice_user_certif.dump_and_sign(&coolorg.signing_key),
                    bob_user_certif.dump_and_sign(&coolorg.signing_key),
                ],
                revoked_users: vec![alice_revoked_user_certif.dump_and_sign(&bob.signing_key)],
            },
            mallory_user_certif.dump_and_sign(&alice.signing_key),
            None,
            vec![mallory_device_certif.dump_and_sign(&coolorg.signing_key)],
            Some(mallory.user_id()),
        )
        .unwrap_err();

    assert_eq!(
        err,
        TrustchainError::SignaturePosteriorUserRevocation {
            path: "mallory's creation <-sign- alice@dev1".into(),
            verified_timestamp: d2,
            user_timestamp: d1,
        }
    );
}

#[rstest]
#[allow(clippy::too_many_arguments)]
fn test_revoked_user_signature_while_revoked(
    alice: &Device,
    bob: &Device,
    mallory: &Device,
    alice_user_certif: &UserCertificate,
    bob_user_certif: &UserCertificate,
    mallory_user_certif: &UserCertificate,
    alice_device_certif: &DeviceCertificate,
    bob_device_certif: &DeviceCertificate,
    mallory_device_certif: &DeviceCertificate,
    alice_revoked_user_certif: &RevokedUserCertificate,
    mallory_revoked_user_certif: &RevokedUserCertificate,
    coolorg: &Organization,
) {
    let d1 = DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap();
    let d2 = DateTime::from_ymd_hms_us(2000, 1, 2, 0, 0, 0, 0).unwrap();
    let mut ctx = TrustchainContext::new(
        mallory.root_verify_key().clone(),
        mallory.time_provider.clone(),
        1,
    );

    let mut bob_user_certif = bob_user_certif.clone();
    bob_user_certif.profile = UserProfile::Admin;

    let mut mallory_revoked_user_certif = mallory_revoked_user_certif.clone();
    mallory_revoked_user_certif.timestamp = d2;

    let err = ctx
        .load_user_and_devices(
            Trustchain {
                devices: vec![
                    alice_device_certif.dump_and_sign(&coolorg.signing_key),
                    bob_device_certif.dump_and_sign(&coolorg.signing_key),
                ],
                users: vec![
                    alice_user_certif.dump_and_sign(&coolorg.signing_key),
                    bob_user_certif.dump_and_sign(&coolorg.signing_key),
                ],
                revoked_users: vec![
                    alice_revoked_user_certif.dump_and_sign(&bob.signing_key),
                    mallory_revoked_user_certif.dump_and_sign(&alice.signing_key),
                ],
            },
            mallory_user_certif.dump_and_sign(&coolorg.signing_key),
            None,
            vec![mallory_device_certif.dump_and_sign(&coolorg.signing_key)],
            Some(mallory.user_id()),
        )
        .unwrap_err();

    assert_eq!(
        err,
        TrustchainError::SignaturePosteriorUserRevocation {
            path: "mallory's revocation <-sign- alice@dev1".into(),
            verified_timestamp: d2,
            user_timestamp: d1,
        }
    );
}

#[rstest]
#[allow(clippy::too_many_arguments)]
fn test_create_user_not_admin(
    alice: &Device,
    bob: &Device,
    mallory: &Device,
    alice_user_certif: &UserCertificate,
    bob_user_certif: &UserCertificate,
    alice_device_certif: &DeviceCertificate,
    bob_device_certif: &DeviceCertificate,
    coolorg: &Organization,
) {
    let mut ctx = TrustchainContext::new(
        mallory.root_verify_key().clone(),
        mallory.time_provider.clone(),
        1,
    );

    let mut alice_user_certif = alice_user_certif.clone();
    alice_user_certif.author = CertificateSignerOwned::User(bob.device_id.clone());

    let mut bob_user_certif = bob_user_certif.clone();
    bob_user_certif.profile = UserProfile::Standard;

    let err = ctx
        .load_user_and_devices(
            Trustchain {
                devices: vec![bob_device_certif.dump_and_sign(&coolorg.signing_key)],
                users: vec![bob_user_certif.dump_and_sign(&coolorg.signing_key)],
                revoked_users: vec![],
            },
            alice_user_certif.dump_and_sign(&bob.signing_key),
            None,
            vec![alice_device_certif.dump_and_sign(&coolorg.signing_key)],
            Some(alice.user_id()),
        )
        .unwrap_err();

    assert_eq!(
        err,
        TrustchainError::InvalidSignatureGiven {
            path: "alice's creation <-sign- bob@dev1".into(),
            user_id: bob.user_id().clone(),
        }
    );
}

#[rstest]
#[allow(clippy::too_many_arguments)]
fn test_revoked_user_not_admin(
    alice: &Device,
    bob: &Device,
    mallory: &Device,
    alice_user_certif: &UserCertificate,
    bob_user_certif: &UserCertificate,
    alice_device_certif: &DeviceCertificate,
    bob_device_certif: &DeviceCertificate,
    alice_revoked_user_certif: &RevokedUserCertificate,
    coolorg: &Organization,
) {
    let mut ctx = TrustchainContext::new(
        mallory.root_verify_key().clone(),
        mallory.time_provider.clone(),
        1,
    );

    let mut bob_user_certif = bob_user_certif.clone();
    bob_user_certif.profile = UserProfile::Standard;

    let err = ctx
        .load_user_and_devices(
            Trustchain {
                devices: vec![bob_device_certif.dump_and_sign(&coolorg.signing_key)],
                users: vec![bob_user_certif.dump_and_sign(&coolorg.signing_key)],
                revoked_users: vec![alice_revoked_user_certif.dump_and_sign(&bob.signing_key)],
            },
            alice_user_certif.dump_and_sign(&coolorg.signing_key),
            None,
            vec![alice_device_certif.dump_and_sign(&coolorg.signing_key)],
            Some(alice.user_id()),
        )
        .unwrap_err();

    assert_eq!(
        err,
        TrustchainError::InvalidSignatureGiven {
            path: "alice's revocation <-sign- bob@dev1".into(),
            user_id: bob.user_id().clone(),
        }
    );
}

#[rstest]
#[allow(clippy::too_many_arguments)]
fn test_verify_user_with_broken_trustchain(
    alice: &Device,
    bob: &Device,
    mallory: &Device,
    alice_user_certif: &UserCertificate,
    bob_user_certif: &UserCertificate,
    mallory_user_certif: &UserCertificate,
    alice_device_certif: &DeviceCertificate,
    bob_device_certif: &DeviceCertificate,
    alice_revoked_user_certif: &RevokedUserCertificate,
    coolorg: &Organization,
) {
    let mut ctx = TrustchainContext::new(
        mallory.root_verify_key().clone(),
        alice.time_provider.clone(),
        1,
    );

    let mut mallory_user_certif = mallory_user_certif.clone();
    mallory_user_certif.profile = UserProfile::Admin;

    let mut bob_device_certif = bob_device_certif.clone();
    bob_device_certif.author = CertificateSignerOwned::User(mallory.device_id.clone());

    let err = ctx
        .load_user_and_devices(
            Trustchain {
                devices: vec![bob_device_certif.dump_and_sign(&mallory.signing_key)],
                users: vec![
                    bob_user_certif.dump_and_sign(&coolorg.signing_key),
                    mallory_user_certif.dump_and_sign(&coolorg.signing_key),
                ],
                revoked_users: vec![alice_revoked_user_certif.dump_and_sign(&bob.signing_key)],
            },
            alice_user_certif.dump_and_sign(&coolorg.signing_key),
            None,
            vec![alice_device_certif.dump_and_sign(&coolorg.signing_key)],
            Some(alice.user_id()),
        )
        .unwrap_err();

    assert_eq!(
        err,
        TrustchainError::MissingDeviceCertificate {
            path: "bob@dev1 <-sign- mallory@dev1".into(),
            device_id: mallory.device_id.clone(),
        }
    );
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use libparsec_client_connection::{
    test_register_low_level_send_hook, test_register_send_hook,
    test_register_sequence_of_send_hooks, HeaderMap, ResponseMock, StatusCode,
};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::{CertifBootstrapWorkspaceError, CertificateBasedActionOutcome};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok_full_bootstrap(env: &TestbedEnv) {
    let realm_id = VlobID::default();
    let env = env.customize(|builder| {
        builder.certificates_storage_fetch_certificates("alice@dev1");
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let new_realm_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle: Arc<Mutex<Option<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle_access: Arc<Mutex<Option<Bytes>>> = Arc::default();
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Realm creation
        {
            let alice = alice.clone();
            let new_realm_certificates = new_realm_certificates.clone();
            move |req: authenticated_cmds::latest::realm_create::Req| {
                RealmRoleCertificate::verify_and_load(
                    &req.realm_role_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
                    Some(realm_id),
                    Some(alice.user_id()),
                )
                .unwrap();
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_role_certificate);
                authenticated_cmds::latest::realm_create::Rep::Ok
            }
        },
        // 2) Initial key rotation
        {
            let alice = alice.clone();
            let new_realm_certificates = new_realm_certificates.clone();
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_access = new_realm_initial_keys_bundle_access.clone();
            move |req: authenticated_cmds::latest::realm_rotate_key::Req| {
                RealmKeyRotationCertificate::verify_and_load(
                    &req.realm_key_rotation_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
                    Some(realm_id),
                )
                .unwrap();
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_key_rotation_certificate);
                *new_realm_initial_keys_bundle.lock().unwrap() = Some(req.keys_bundle);
                let access = req
                    .per_participant_keys_bundle_access
                    .get(&"alice".parse().unwrap())
                    .unwrap()
                    .to_owned();
                *new_realm_initial_keys_bundle_access.lock().unwrap() = Some(access);
                authenticated_cmds::latest::realm_rotate_key::Rep::Ok
            }
        },
        // 3) Fetch new realm certificates (required for initial rename)
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |_req: authenticated_cmds::latest::certificate_get::Req| {
                authenticated_cmds::latest::certificate_get::Rep::Ok {
                    common_certificates: vec![],
                    realm_certificates: HashMap::from([(
                        realm_id,
                        new_realm_certificates.lock().unwrap().clone(),
                    )]),
                    sequester_certificates: vec![],
                    shamir_recovery_certificates: vec![],
                }
            }
        },
        // 4) Fetch keys bundle (required for initial rename)
        {
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_access = new_realm_initial_keys_bundle_access.clone();
            move |_req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle: new_realm_initial_keys_bundle
                        .lock()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .clone(),
                    keys_bundle_access: new_realm_initial_keys_bundle_access
                        .lock()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .clone(),
                }
            }
        },
        // 5) Rename
        {
            let alice = alice.clone();
            let new_realm_certificates = new_realm_certificates.clone();
            move |req: authenticated_cmds::latest::realm_rename::Req| {
                RealmNameCertificate::verify_and_load(
                    &req.realm_name_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
                    Some(realm_id),
                )
                .unwrap();
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_name_certificate);
                authenticated_cmds::latest::realm_rename::Rep::Ok
            }
        }
    );

    let res = ops
        .bootstrap_workspace(realm_id, &"wksp1".parse().unwrap())
        .await
        .unwrap();

    p_assert_matches!(res, CertificateBasedActionOutcome::Uploaded { .. });
}

#[parsec_test(testbed = "minimal")]
async fn ok_partial_bootstrap_realm_created(env: &TestbedEnv) {
    let (env, (realm_id, timestamp)) = env.customize_with_map(|builder| {
        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder
            .new_realm("alice")
            .map(|event| (event.realm_id, event.timestamp))
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let new_realm_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle: Arc<Mutex<Option<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle_access: Arc<Mutex<Option<Bytes>>> = Arc::default();
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Realm creation
        {
            let alice = alice.clone();
            let new_realm_certificates = new_realm_certificates.clone();
            move |req: authenticated_cmds::latest::realm_create::Req| {
                RealmRoleCertificate::verify_and_load(
                    &req.realm_role_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
                    Some(realm_id),
                    Some(alice.user_id()),
                )
                .unwrap();
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_role_certificate);
                // Create realm has been done
                authenticated_cmds::latest::realm_create::Rep::RealmAlreadyExists {
                    last_realm_certificate_timestamp: timestamp,
                }
            }
        },
        // 2) Initial key rotation
        {
            let alice = alice.clone();
            let new_realm_certificates = new_realm_certificates.clone();
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_access = new_realm_initial_keys_bundle_access.clone();
            move |req: authenticated_cmds::latest::realm_rotate_key::Req| {
                RealmKeyRotationCertificate::verify_and_load(
                    &req.realm_key_rotation_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
                    Some(realm_id),
                )
                .unwrap();
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_key_rotation_certificate);
                *new_realm_initial_keys_bundle.lock().unwrap() = Some(req.keys_bundle);
                let access = req
                    .per_participant_keys_bundle_access
                    .get(&"alice".parse().unwrap())
                    .unwrap()
                    .to_owned();
                *new_realm_initial_keys_bundle_access.lock().unwrap() = Some(access);
                authenticated_cmds::latest::realm_rotate_key::Rep::Ok
            }
        },
        // 3) Fetch new realm certificates (required for initial rename)
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |_req: authenticated_cmds::latest::certificate_get::Req| {
                authenticated_cmds::latest::certificate_get::Rep::Ok {
                    common_certificates: vec![],
                    realm_certificates: HashMap::from([(
                        realm_id,
                        new_realm_certificates.lock().unwrap().clone(),
                    )]),
                    sequester_certificates: vec![],
                    shamir_recovery_certificates: vec![],
                }
            }
        },
        // 4) Fetch keys bundle (required for initial rename)
        {
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_access = new_realm_initial_keys_bundle_access.clone();
            move |_req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle: new_realm_initial_keys_bundle
                        .lock()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .clone(),
                    keys_bundle_access: new_realm_initial_keys_bundle_access
                        .lock()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .clone(),
                }
            }
        },
        // 5) Rename
        {
            let alice = alice.clone();
            let new_realm_certificates = new_realm_certificates.clone();
            move |req: authenticated_cmds::latest::realm_rename::Req| {
                RealmNameCertificate::verify_and_load(
                    &req.realm_name_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
                    Some(realm_id),
                )
                .unwrap();
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_name_certificate);
                authenticated_cmds::latest::realm_rename::Rep::Ok
            }
        }
    );

    let res = ops
        .bootstrap_workspace(realm_id, &"wksp1".parse().unwrap())
        .await
        .unwrap();

    p_assert_matches!(res, CertificateBasedActionOutcome::Uploaded { .. });
}

#[parsec_test(testbed = "minimal")]
async fn ok_partial_bootstrap_realm_created_fetched(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder.new_realm("alice").map(|event| event.realm_id);
        builder.certificates_storage_fetch_certificates("alice@dev1");

        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let new_realm_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle: Arc<Mutex<Option<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle_access: Arc<Mutex<Option<Bytes>>> = Arc::default();
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Initial key rotation
        {
            let alice = alice.clone();
            let new_realm_certificates = new_realm_certificates.clone();
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_access = new_realm_initial_keys_bundle_access.clone();
            move |req: authenticated_cmds::latest::realm_rotate_key::Req| {
                RealmKeyRotationCertificate::verify_and_load(
                    &req.realm_key_rotation_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
                    Some(realm_id),
                )
                .unwrap();
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_key_rotation_certificate);
                *new_realm_initial_keys_bundle.lock().unwrap() = Some(req.keys_bundle);
                let access = req
                    .per_participant_keys_bundle_access
                    .get(&"alice".parse().unwrap())
                    .unwrap()
                    .to_owned();
                *new_realm_initial_keys_bundle_access.lock().unwrap() = Some(access);
                authenticated_cmds::latest::realm_rotate_key::Rep::Ok
            }
        },
        // 2) Fetch new realm certificates (required for initial rename)
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |_req: authenticated_cmds::latest::certificate_get::Req| {
                authenticated_cmds::latest::certificate_get::Rep::Ok {
                    common_certificates: vec![],
                    realm_certificates: HashMap::from([(
                        realm_id,
                        new_realm_certificates.lock().unwrap().clone(),
                    )]),
                    sequester_certificates: vec![],
                    shamir_recovery_certificates: vec![],
                }
            }
        },
        // 3) Fetch keys bundle (required for initial rename)
        {
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_access = new_realm_initial_keys_bundle_access.clone();
            move |_req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle: new_realm_initial_keys_bundle
                        .lock()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .clone(),
                    keys_bundle_access: new_realm_initial_keys_bundle_access
                        .lock()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .clone(),
                }
            }
        },
        // 4) Rename
        {
            let alice = alice.clone();
            let new_realm_certificates = new_realm_certificates.clone();
            move |req: authenticated_cmds::latest::realm_rename::Req| {
                RealmNameCertificate::verify_and_load(
                    &req.realm_name_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
                    Some(realm_id),
                )
                .unwrap();
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_name_certificate);
                authenticated_cmds::latest::realm_rename::Rep::Ok
            }
        }
    );

    let res = ops
        .bootstrap_workspace(realm_id, &"wksp1".parse().unwrap())
        .await
        .unwrap();

    p_assert_matches!(res, CertificateBasedActionOutcome::Uploaded { .. });
}

#[parsec_test(testbed = "minimal")]
async fn ok_partial_bootstrap_initial_key_rotation(env: &TestbedEnv) {
    let (env, (realm_id, timestamp)) = env.customize_with_map(|builder| {
        let realm_id = builder.new_realm("alice").map(|event| event.realm_id);
        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder
            .rotate_key_realm(realm_id)
            .map(|event| (realm_id, event.timestamp))
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let new_realm_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle: Arc<Mutex<Option<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle_access: Arc<Mutex<Option<Bytes>>> = Arc::default();
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Initial key rotation
        {
            let alice = alice.clone();
            let new_realm_certificates = new_realm_certificates.clone();
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_access = new_realm_initial_keys_bundle_access.clone();
            move |req: authenticated_cmds::latest::realm_rotate_key::Req| {
                RealmKeyRotationCertificate::verify_and_load(
                    &req.realm_key_rotation_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
                    Some(realm_id),
                )
                .unwrap();
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_key_rotation_certificate);
                *new_realm_initial_keys_bundle.lock().unwrap() = Some(req.keys_bundle);
                let access = req
                    .per_participant_keys_bundle_access
                    .get(&"alice".parse().unwrap())
                    .unwrap()
                    .to_owned();
                *new_realm_initial_keys_bundle_access.lock().unwrap() = Some(access);
                // Rotating realm key has been done
                authenticated_cmds::latest::realm_rotate_key::Rep::BadKeyIndex {
                    last_realm_certificate_timestamp: timestamp,
                }
            }
        },
        // 2) Fetch new realm certificates (required for initial rename)
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |_req: authenticated_cmds::latest::certificate_get::Req| {
                authenticated_cmds::latest::certificate_get::Rep::Ok {
                    common_certificates: vec![],
                    realm_certificates: HashMap::from([(
                        realm_id,
                        new_realm_certificates.lock().unwrap().clone(),
                    )]),
                    sequester_certificates: vec![],
                    shamir_recovery_certificates: vec![],
                }
            }
        },
        // 3) Fetch keys bundle (required for initial rename)
        {
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_access = new_realm_initial_keys_bundle_access.clone();
            move |_req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle: new_realm_initial_keys_bundle
                        .lock()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .clone(),
                    keys_bundle_access: new_realm_initial_keys_bundle_access
                        .lock()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .clone(),
                }
            }
        },
        // 4) Rename
        {
            let alice = alice.clone();
            let new_realm_certificates = new_realm_certificates.clone();
            move |req: authenticated_cmds::latest::realm_rename::Req| {
                RealmNameCertificate::verify_and_load(
                    &req.realm_name_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
                    Some(realm_id),
                )
                .unwrap();
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_name_certificate);
                authenticated_cmds::latest::realm_rename::Rep::Ok
            }
        }
    );

    let res = ops
        .bootstrap_workspace(realm_id, &"wksp1".parse().unwrap())
        .await
        .unwrap();

    p_assert_matches!(res, CertificateBasedActionOutcome::Uploaded { .. });
}

#[parsec_test(testbed = "minimal")]
async fn ok_partial_bootstrap_initial_key_rotation_fetched(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);
        builder.certificates_storage_fetch_certificates("alice@dev1");
        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let keys_bundle = env.get_last_realm_keys_bundle(realm_id);
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, &alice.user_id());
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        {
            move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                p_assert_eq!(req.key_index, 1);
                p_assert_eq!(req.realm_id, realm_id);

                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle,
                    keys_bundle_access,
                }
            }
        },
        {
            move |req: authenticated_cmds::latest::realm_rename::Req| {
                p_assert_eq!(req.initial_name_or_fail, true);
                authenticated_cmds::latest::realm_rename::Rep::Ok
            }
        },
    );

    let res = ops
        .bootstrap_workspace(realm_id, &"wksp1".parse().unwrap())
        .await
        .unwrap();

    p_assert_matches!(res, CertificateBasedActionOutcome::Uploaded { .. });
}

#[parsec_test(testbed = "minimal")]
#[case::author_not_allowed(
    authenticated_cmds::latest::realm_rename::Rep::AuthorNotAllowed,
    |err| p_assert_matches!(err, CertifBootstrapWorkspaceError::AuthorNotAllowed)
)]
#[case::timestamp_out_of_ballpark(
    authenticated_cmds::latest::realm_rename::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 300.,
        ballpark_client_late_offset: 320.,
        client_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
        server_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
    },
    |err| p_assert_matches!(
        err,
        CertifBootstrapWorkspaceError::TimestampOutOfBallpark {
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            client_timestamp,
            server_timestamp,
        }
        if ballpark_client_early_offset == 300.
            && ballpark_client_late_offset == 320.
            && client_timestamp == "2000-01-02T00:00:00Z".parse().unwrap()
            && server_timestamp == "2000-01-02T00:00:00Z".parse().unwrap()
    )
)]
#[case::require_greater_timestamp(
    authenticated_cmds::latest::realm_rename::Rep::RequireGreaterTimestamp {
        strictly_greater_than: "2000-01-02T00:00:00Z".parse().unwrap(),
    },
    |err| p_assert_matches!(err, CertifBootstrapWorkspaceError::Offline)
)]
#[case::realm_not_found(
    authenticated_cmds::latest::realm_rename::Rep::RealmNotFound,
    |err| p_assert_matches!(err, CertifBootstrapWorkspaceError::Internal(_))
)]
#[case::invalid_certificate(
    authenticated_cmds::latest::realm_rename::Rep::InvalidCertificate,
    |err| p_assert_matches!(err, CertifBootstrapWorkspaceError::Internal(_))
)]
#[case::bad_key_index(
    authenticated_cmds::latest::realm_rename::Rep::BadKeyIndex {
        last_realm_certificate_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
    },
    |err| p_assert_matches!(err, CertifBootstrapWorkspaceError::Offline)
)]
#[case::unknown_status(
    authenticated_cmds::latest::realm_rename::Rep::UnknownStatus { unknown_status: "".into(), reason: None },
    |err| p_assert_matches!(err, CertifBootstrapWorkspaceError::Internal(_))
)]
async fn server_error(
    #[case] rep: authenticated_cmds::latest::realm_rename::Rep,
    #[case] assert: impl FnOnce(CertifBootstrapWorkspaceError),
    env: &TestbedEnv,
) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);
        builder.certificates_storage_fetch_certificates("alice@dev1");
        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let keys_bundle = env.get_last_realm_keys_bundle(realm_id);
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, &alice.user_id());
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        {
            move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                p_assert_eq!(req.key_index, 1);
                p_assert_eq!(req.realm_id, realm_id);

                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle,
                    keys_bundle_access,
                }
            }
        },
        {
            move |req: authenticated_cmds::latest::realm_rename::Req| {
                p_assert_eq!(req.initial_name_or_fail, true);
                rep
            }
        },
    );

    let err = ops
        .bootstrap_workspace(realm_id, &"wksp1".parse().unwrap())
        .await
        .unwrap_err();

    assert(err);
}

#[parsec_test(testbed = "minimal")]
async fn server_initial_name_already_exists(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);
        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder.rename_realm(realm_id, "wksp1");
        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let keys_bundle = env.get_last_realm_keys_bundle(realm_id);
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, &alice.user_id());
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        {
            move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                p_assert_eq!(req.key_index, 1);
                p_assert_eq!(req.realm_id, realm_id);

                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle,
                    keys_bundle_access,
                }
            }
        },
        {
            move |req: authenticated_cmds::latest::realm_rename::Req| {
                p_assert_eq!(req.initial_name_or_fail, true);
                authenticated_cmds::latest::realm_rename::Rep::InitialNameAlreadyExists {
                    last_realm_certificate_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
                }
            }
        },
    );

    let res = ops
        .bootstrap_workspace(realm_id, &"wksp1".parse().unwrap())
        .await
        .unwrap();

    p_assert_matches!(
        res,
        CertificateBasedActionOutcome::RemoteIdempotent { certificate_timestamp }
        if certificate_timestamp == "2000-01-02T00:00:00Z".parse().unwrap()
    );
}

#[parsec_test(testbed = "minimal")]
#[case::invalid_keys_bundle(
    |env: &TestbedEnv, realm_id, user_id: &UserID| authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
        keys_bundle: Bytes::from_static(b""),
        keys_bundle_access: env.get_last_realm_keys_bundle_access_for(realm_id, user_id),
    },
    |err| p_assert_matches!(err, CertifBootstrapWorkspaceError::InvalidKeysBundle(_))
)]
#[case::invalid_keys_bundle_access(
    |env: &TestbedEnv, realm_id, _: &UserID| authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
        keys_bundle: env.get_last_realm_keys_bundle(realm_id),
        keys_bundle_access: Bytes::from_static(b""),
    },
    |err| p_assert_matches!(err, CertifBootstrapWorkspaceError::InvalidKeysBundle(_))
)]
async fn invalid_keys_bundle(
    #[case] rep: impl FnOnce(
        &TestbedEnv,
        VlobID,
        &UserID,
    ) -> authenticated_cmds::latest::realm_get_keys_bundle::Rep,
    #[case] assert: impl FnOnce(CertifBootstrapWorkspaceError),
    env: &TestbedEnv,
) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);
        builder.certificates_storage_fetch_certificates("alice@dev1");
        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let rep = rep(&env, realm_id, &alice.user_id());
    test_register_send_hook(&env.discriminant_dir, {
        move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            p_assert_eq!(req.key_index, 1);
            p_assert_eq!(req.realm_id, realm_id);

            rep
        }
    });

    let err = ops
        .bootstrap_workspace(realm_id, &"wksp1".parse().unwrap())
        .await
        .unwrap_err();

    assert(err);
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .bootstrap_workspace(VlobID::default(), &"wksp1".parse().unwrap())
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifBootstrapWorkspaceError::Offline);
}

#[parsec_test(testbed = "minimal")]
async fn invalid_response(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);
        builder.certificates_storage_fetch_certificates("alice@dev1");
        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
        Ok(ResponseMock::Mocked((
            StatusCode::IM_A_TEAPOT,
            HeaderMap::new(),
            Bytes::new(),
        )))
    });

    let err = ops
        .bootstrap_workspace(realm_id, &"wksp1".parse().unwrap())
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifBootstrapWorkspaceError::Internal(_));
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);
        builder.certificates_storage_fetch_certificates("alice@dev1");
        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops
        .bootstrap_workspace(realm_id, &"wksp1".parse().unwrap())
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifBootstrapWorkspaceError::Stopped);
}

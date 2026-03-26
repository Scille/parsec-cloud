// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    test_register_low_level_send_hook, test_register_send_hook,
    test_register_sequence_of_send_hooks, ConnectionError, HeaderMap, ResponseMock, StatusCode,
};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::{CertifArchiveRealmError, CertificateBasedActionOutcome};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(
    #[values(
        "available",
        "archived",
        "deletion_planned_deadline_soon",
        "deletion_planned_deadline_reached"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let configuration = match kind {
        "available" => RealmArchivingConfiguration::Available,
        "archived" => RealmArchivingConfiguration::Archived,
        "deletion_planned_deadline_soon" => RealmArchivingConfiguration::DeletionPlanned {
            // Yeah, like you say, *soon*...
            deletion_date: DateTime::from_ymd_hms_us(2999, 1, 1, 0, 0, 0, 0).unwrap(),
        },
        "deletion_planned_deadline_reached" => RealmArchivingConfiguration::DeletionPlanned {
            deletion_date: DateTime::from_ymd_hms_us(2000, 1, 1, 0, 0, 0, 0).unwrap(),
        },
        unknown => panic!("Unknown kind: {unknown}"),
    };
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(&env.discriminant_dir, {
        move |_req: authenticated_cmds::latest::realm_update_archiving::Req| {
            authenticated_cmds::latest::realm_update_archiving::Rep::Ok
        }
    });

    let res = ops.archive_realm(realm_id, configuration).await.unwrap();

    p_assert_matches!(res, CertificateBasedActionOutcome::Uploaded { .. });
}

#[parsec_test(testbed = "minimal")]
async fn server_error_author_not_allowed(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(&env.discriminant_dir, {
        move |_req: authenticated_cmds::latest::realm_update_archiving::Req| {
            authenticated_cmds::latest::realm_update_archiving::Rep::AuthorNotAllowed
        }
    });

    let err = ops
        .archive_realm(realm_id, RealmArchivingConfiguration::Archived)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifArchiveRealmError::AuthorNotAllowed);
}

#[parsec_test(testbed = "minimal")]
async fn server_error_realm_not_found(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(&env.discriminant_dir, {
        move |_req: authenticated_cmds::latest::realm_update_archiving::Req| {
            authenticated_cmds::latest::realm_update_archiving::Rep::RealmNotFound
        }
    });

    let err = ops
        .archive_realm(realm_id, RealmArchivingConfiguration::Archived)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifArchiveRealmError::UnknownRealm);
}

#[parsec_test(testbed = "minimal")]
async fn server_error_realm_deleted(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(&env.discriminant_dir, {
        move |_req: authenticated_cmds::latest::realm_update_archiving::Req| {
            authenticated_cmds::latest::realm_update_archiving::Rep::RealmDeleted
        }
    });

    let err = ops
        .archive_realm(realm_id, RealmArchivingConfiguration::Archived)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifArchiveRealmError::RealmDeleted);
}

#[parsec_test(testbed = "minimal")]
async fn server_error_archiving_period_too_short(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(&env.discriminant_dir, {
        move |_req: authenticated_cmds::latest::realm_update_archiving::Req| {
            authenticated_cmds::latest::realm_update_archiving::Rep::ArchivingPeriodTooShort
        }
    });

    let err = ops
        .archive_realm(realm_id, RealmArchivingConfiguration::Archived)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifArchiveRealmError::ArchivingPeriodTooShort);
}

#[parsec_test(testbed = "minimal")]
async fn server_error_timestamp_out_of_ballpark(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(&env.discriminant_dir, {
        move |_req: authenticated_cmds::latest::realm_update_archiving::Req| {
            authenticated_cmds::latest::realm_update_archiving::Rep::TimestampOutOfBallpark {
                ballpark_client_early_offset: 300.,
                ballpark_client_late_offset: 320.,
                client_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
                server_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
            }
        }
    });

    let err = ops
        .archive_realm(realm_id, RealmArchivingConfiguration::Archived)
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifArchiveRealmError::TimestampOutOfBallpark {
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            client_timestamp,
            server_timestamp,
        }
        if ballpark_client_early_offset == 300.
            && ballpark_client_late_offset == 320.
            && client_timestamp == "2000-01-02T00:00:00Z".parse().unwrap()
            && server_timestamp == "2000-01-02T00:00:00Z".parse().unwrap(),
    );
}

#[parsec_test(testbed = "minimal")]
async fn server_error_require_greater_timestamp(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(&env.discriminant_dir, {
        move |_req: authenticated_cmds::latest::realm_update_archiving::Req| {
            authenticated_cmds::latest::realm_update_archiving::Rep::RequireGreaterTimestamp {
                strictly_greater_than: "2000-01-02T00:00:00Z".parse().unwrap(),
            }
        }
    });

    let err = ops
        .archive_realm(realm_id, RealmArchivingConfiguration::Archived)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifArchiveRealmError::Offline(_));
}

#[parsec_test(testbed = "minimal")]
async fn server_error_invalid_certificate(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(&env.discriminant_dir, {
        move |_req: authenticated_cmds::latest::realm_update_archiving::Req| {
            authenticated_cmds::latest::realm_update_archiving::Rep::InvalidCertificate
        }
    });

    let err = ops
        .archive_realm(realm_id, RealmArchivingConfiguration::Archived)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifArchiveRealmError::Internal(_));
}

#[parsec_test(testbed = "minimal")]
async fn server_error_unknown_status(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(&env.discriminant_dir, {
        move |_req: authenticated_cmds::latest::realm_update_archiving::Req| {
            authenticated_cmds::latest::realm_update_archiving::Rep::UnknownStatus {
                unknown_status: "".into(),
                reason: None,
            }
        }
    });

    let err = ops
        .archive_realm(realm_id, RealmArchivingConfiguration::Archived)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifArchiveRealmError::Internal(_));
}

#[parsec_test(testbed = "minimal")]
async fn loop_require_greater_timestamp(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::realm_update_archiving::Req| {
            authenticated_cmds::latest::realm_update_archiving::Rep::RequireGreaterTimestamp {
                strictly_greater_than: DateTime::from_ymd_hms_us(2000, 1, 2, 0, 0, 0, 0).unwrap(),
            }
        },
        move |_req: authenticated_cmds::latest::realm_update_archiving::Req| {
            authenticated_cmds::latest::realm_update_archiving::Rep::Ok
        },
    );

    let res = ops
        .archive_realm(realm_id, RealmArchivingConfiguration::Archived)
        .await
        .unwrap();

    p_assert_matches!(res, CertificateBasedActionOutcome::Uploaded { .. });
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .archive_realm(realm_id, RealmArchivingConfiguration::Archived)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifArchiveRealmError::Offline(_));
}

#[parsec_test(testbed = "minimal")]
async fn invalid_response(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
        Ok(ResponseMock::Mocked((
            StatusCode::IM_A_TEAPOT,
            HeaderMap::new(),
            Bytes::new(),
        )))
    });

    let err = ops
        .archive_realm(realm_id, RealmArchivingConfiguration::Archived)
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifArchiveRealmError::Offline(ConnectionError::InvalidResponseStatus(
            StatusCode::IM_A_TEAPOT
        ))
    );
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops
        .archive_realm(realm_id, RealmArchivingConfiguration::Archived)
        .await
        .unwrap_err();

    // This operation is so simple than it can be achieved even if the `CertificateOps` has
    // been stopped, hence why we don't expected a `CertifArchiveRealmError::Stopped` here
    p_assert_matches!(err, CertifArchiveRealmError::Offline(_));
}

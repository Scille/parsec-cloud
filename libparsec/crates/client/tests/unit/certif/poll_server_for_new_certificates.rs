// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, sync::Arc};

use libparsec_client_connection::{
    test_register_low_level_send_hook, test_register_send_hook, HeaderMap, ResponseMock, StatusCode,
};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::CertifPollServerError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn empty(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.common_after, None);
            assert!(req.realm_after.is_empty());
            p_assert_eq!(req.sequester_after, None);
            p_assert_eq!(req.shamir_recovery_after, None);

            authenticated_cmds::latest::certificate_get::Rep::Ok {
                common_certificates: vec![],
                sequester_certificates: vec![],
                shamir_recovery_certificates: vec![],
                realm_certificates: HashMap::default(),
            }
        },
    );

    ops.poll_server_for_new_certificates(None).await.unwrap();
}

async fn poll_testbed_certificates(env: &TestbedEnv, device: Arc<LocalDevice>) {
    let ops = certificates_ops_factory(env, &device).await;

    test_register_send_hook(&env.discriminant_dir, {
        let rep = authenticated_cmds::latest::certificate_get::Rep::Ok {
            common_certificates: env.get_common_certificates_signed(),
            sequester_certificates: env.get_sequester_certificates_signed(),
            shamir_recovery_certificates: env.get_shamir_recovery_certificates_signed(),
            realm_certificates: env.get_realms_certificates_signed(),
        };
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.common_after, None);
            assert!(req.realm_after.is_empty());
            p_assert_eq!(req.sequester_after, None);
            p_assert_eq!(req.shamir_recovery_after, None);
            rep
        }
    });

    ops.poll_server_for_new_certificates(None).await.unwrap();

    // Re-poll, but nothing more to get

    let mut expected_common_after = None;
    let mut expected_shamir_recovery_after: Option<DateTime> = None;
    let mut expected_realm_after: HashMap<VlobID, DateTime> = HashMap::default();
    let mut expected_sequester_after = None;
    for certif in env.template.certificates() {
        match certif.certificate {
            AnyArcCertificate::User(c) => {
                expected_common_after = Some(c.timestamp);
            }
            AnyArcCertificate::Device(c) => {
                expected_common_after = Some(c.timestamp);
            }
            AnyArcCertificate::UserUpdate(c) => {
                expected_common_after = Some(c.timestamp);
            }
            AnyArcCertificate::RevokedUser(c) => {
                expected_common_after = Some(c.timestamp);
            }
            AnyArcCertificate::RealmRole(c) => {
                expected_realm_after.insert(c.realm_id, c.timestamp);
            }
            AnyArcCertificate::RealmName(c) => {
                expected_realm_after.insert(c.realm_id, c.timestamp);
            }
            AnyArcCertificate::RealmKeyRotation(c) => {
                expected_realm_after.insert(c.realm_id, c.timestamp);
            }
            AnyArcCertificate::RealmArchiving(c) => {
                expected_realm_after.insert(c.realm_id, c.timestamp);
            }
            AnyArcCertificate::ShamirRecoveryBrief(c) => {
                expected_shamir_recovery_after = Some(c.timestamp);
            }
            AnyArcCertificate::ShamirRecoveryShare(c) => {
                expected_shamir_recovery_after = Some(c.timestamp);
            }
            AnyArcCertificate::SequesterAuthority(c) => {
                expected_sequester_after = Some(c.timestamp);
            }
            AnyArcCertificate::SequesterService(c) => {
                expected_sequester_after = Some(c.timestamp);
            }
            AnyArcCertificate::SequesterRevokedService(c) => {
                expected_sequester_after = Some(c.timestamp);
            }
        }
    }

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.common_after, expected_common_after);
            p_assert_eq!(req.realm_after, expected_realm_after);
            p_assert_eq!(req.sequester_after, expected_sequester_after);
            p_assert_eq!(req.shamir_recovery_after, expected_shamir_recovery_after);

            authenticated_cmds::latest::certificate_get::Rep::Ok {
                common_certificates: vec![],
                sequester_certificates: vec![],
                shamir_recovery_certificates: vec![],
                realm_certificates: HashMap::default(),
            }
        },
    );

    ops.poll_server_for_new_certificates(None).await.unwrap();
}

#[parsec_test(testbed = "minimal")]
async fn minimal(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    poll_testbed_certificates(env, alice).await;
}

#[parsec_test(testbed = "coolorg")]
async fn coolorg(env: &TestbedEnv) {
    // Cannot use Alice@dev1 here given its certificate storage is not empty
    let alice = env.local_device("alice@dev2");
    poll_testbed_certificates(env, alice).await;
}

#[parsec_test(testbed = "minimal")]
async fn invalid_certif_received(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::certificate_get::Req| {
            authenticated_cmds::latest::certificate_get::Rep::Ok {
                common_certificates: vec![Bytes::from_static(b"foo")],
                sequester_certificates: vec![],
                shamir_recovery_certificates: vec![],
                realm_certificates: HashMap::default(),
            }
        },
    );

    let err = ops
        .poll_server_for_new_certificates(None)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifPollServerError::InvalidCertificate(..));
}

#[parsec_test(testbed = "minimal")]
async fn unknown_status(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::certificate_get::Req| {
            authenticated_cmds::latest::certificate_get::Rep::UnknownStatus {
                unknown_status: "".into(),
                reason: None,
            }
        },
    );

    let err = ops
        .poll_server_for_new_certificates(None)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifPollServerError::Internal(..));
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .poll_server_for_new_certificates(None)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifPollServerError::Offline);
}

#[parsec_test(testbed = "minimal")]
async fn invalid_response(env: &TestbedEnv) {
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
        .poll_server_for_new_certificates(None)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifPollServerError::Internal(..));
}

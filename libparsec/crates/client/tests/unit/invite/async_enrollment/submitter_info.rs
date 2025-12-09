// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{protocol, test_register_send_hook};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::make_config;
use crate::{
    submitter_get_async_enrollment_info, PendingAsyncEnrollmentInfo,
    SubmitterGetAsyncEnrollmentInfoError,
};

#[parsec_test(testbed = "empty")]
async fn ok_submitted(env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    let enrollment_id = AsyncEnrollmentID::default();
    let expected_submitted_on = "2010-01-01T00:00:00Z".parse().unwrap();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::anonymous_cmds::latest::async_enrollment_info::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::anonymous_cmds::latest::async_enrollment_info::Rep::Ok(
                protocol::anonymous_cmds::latest::async_enrollment_info::InfoStatus::Submitted {
                    submitted_on: expected_submitted_on,
                },
            )
        },
    );

    p_assert_matches!(
        submitter_get_async_enrollment_info(config, async_enrollment_addr, enrollment_id).await,
        Ok(PendingAsyncEnrollmentInfo::Submitted{ submitted_on})
        if submitted_on == expected_submitted_on
    );
}

#[parsec_test(testbed = "empty")]
async fn ok_cancelled(env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    let enrollment_id = AsyncEnrollmentID::default();
    let expected_submitted_on = "2010-01-01T00:00:00Z".parse().unwrap();
    let expected_cancelled_on = "2011-01-01T00:00:00Z".parse().unwrap();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::anonymous_cmds::latest::async_enrollment_info::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::anonymous_cmds::latest::async_enrollment_info::Rep::Ok(
                protocol::anonymous_cmds::latest::async_enrollment_info::InfoStatus::Cancelled {
                    submitted_on: expected_submitted_on,
                    cancelled_on: expected_cancelled_on,
                },
            )
        },
    );

    p_assert_matches!(
        submitter_get_async_enrollment_info(config, async_enrollment_addr, enrollment_id).await,
        Ok(PendingAsyncEnrollmentInfo::Cancelled{ submitted_on, cancelled_on })
        if submitted_on == expected_submitted_on && cancelled_on == expected_cancelled_on
    );
}

#[parsec_test(testbed = "empty")]
async fn ok_rejected(env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    let enrollment_id = AsyncEnrollmentID::default();
    let expected_submitted_on = "2010-01-01T00:00:00Z".parse().unwrap();
    let expected_rejected_on = "2011-01-01T00:00:00Z".parse().unwrap();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::anonymous_cmds::latest::async_enrollment_info::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::anonymous_cmds::latest::async_enrollment_info::Rep::Ok(
                protocol::anonymous_cmds::latest::async_enrollment_info::InfoStatus::Rejected {
                    submitted_on: expected_submitted_on,
                    rejected_on: expected_rejected_on,
                },
            )
        },
    );

    p_assert_matches!(
        submitter_get_async_enrollment_info(config, async_enrollment_addr, enrollment_id).await,
        Ok(PendingAsyncEnrollmentInfo::Rejected{ submitted_on, rejected_on })
        if submitted_on == expected_submitted_on && rejected_on == expected_rejected_on
    );
}

#[parsec_test(testbed = "empty")]
async fn ok_accepted(env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    let enrollment_id = AsyncEnrollmentID::default();
    let expected_submitted_on = "2010-01-01T00:00:00Z".parse().unwrap();
    let expected_accepted_on = "2011-01-01T00:00:00Z".parse().unwrap();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::anonymous_cmds::latest::async_enrollment_info::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::anonymous_cmds::latest::async_enrollment_info::Rep::Ok(protocol::anonymous_cmds::latest::async_enrollment_info::InfoStatus::Accepted {
                submitted_on: expected_submitted_on,
                accepted_on: expected_accepted_on,
                accept_payload: Bytes::from_static(b"<accept_payload>"),
                accept_payload_signature: protocol::anonymous_cmds::latest::async_enrollment_info::AcceptPayloadSignature::OpenBao {
                    signature: "vault:v1:kqrnMiRBFelGqTq7J4bmlhkGun09HshMIfOeGVoA8WZEEHkBlqoWQV+rI/WlBItUjRhBKVVm2PIigshKA7Cb+Q==".to_string(),
                    accepter_openbao_entity_id: "81b533c6-e41a-4533-9d50-3188cb88edd8".to_string(),
                }
            })
        },
    );

    p_assert_matches!(
        submitter_get_async_enrollment_info(config, async_enrollment_addr, enrollment_id).await,
        Ok(PendingAsyncEnrollmentInfo::Accepted{ submitted_on, accepted_on })
        if submitted_on == expected_submitted_on && accepted_on == expected_accepted_on
    );
}

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    p_assert_matches!(
        submitter_get_async_enrollment_info(
            config,
            async_enrollment_addr,
            AsyncEnrollmentID::default()
        )
        .await,
        Err(SubmitterGetAsyncEnrollmentInfoError::Offline(_)),
    );
}

#[parsec_test(testbed = "empty")]
async fn enrollment_not_found(env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    let enrollment_id = AsyncEnrollmentID::default();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::anonymous_cmds::latest::async_enrollment_info::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::anonymous_cmds::latest::async_enrollment_info::Rep::EnrollmentNotFound
        },
    );

    p_assert_matches!(
        submitter_get_async_enrollment_info(config, async_enrollment_addr, enrollment_id).await,
        Err(SubmitterGetAsyncEnrollmentInfoError::EnrollmentNotFound),
    );
}

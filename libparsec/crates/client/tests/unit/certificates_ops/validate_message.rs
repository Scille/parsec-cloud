// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::test_register_send_hook;
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certificates_ops::{InvalidMessageError, ValidateMessageError};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let certificates = env
        .template
        .certificates()
        .map(|e| e.signed)
        .collect::<Vec<_>>();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.offset, 0);

            authenticated_cmds::latest::certificate_get::Rep::Ok { certificates }
        },
    );

    let message = MessageContent::Ping {
        author: alice.device_id.clone(),
        timestamp: now,
        ping: "ping".into(),
    };

    let msg = ops
        .validate_message(
            2,
            0,
            &alice.device_id,
            now,
            &message.dump_sign_and_encrypt_for(&alice.signing_key, &alice.public_key()),
        )
        .await
        .unwrap();

    p_assert_eq!(msg, message);

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn ok_sharing_granted(env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let env = env.customize(|builder| {
        builder.new_user_realm("alice").customize(|event| {
            event.realm_id = vlob_id;
            event.timestamp = now;
        });
    });

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(&env, &alice).await;

    let certificates = env
        .template
        .certificates()
        .map(|e| e.signed)
        .collect::<Vec<_>>();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.offset, 0);

            authenticated_cmds::latest::certificate_get::Rep::Ok { certificates }
        },
    );

    let message = MessageContent::SharingGranted {
        author: alice.device_id.clone(),
        timestamp: now,
        id: vlob_id,
        encrypted_on: now,
        name: "wksp1".parse().unwrap(),
        encryption_revision: 0,
        key: alice.local_symkey.clone(),
    };

    let msg = ops
        .validate_message(
            3,
            0,
            &alice.device_id,
            now,
            &message.dump_sign_and_encrypt_for(&alice.signing_key, &alice.public_key()),
        )
        .await
        .unwrap();

    p_assert_eq!(msg, message);

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn ok_sharing_reencrypted(env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let env = env.customize(|builder| {
        builder
            .new_user_realm("alice")
            .customize(|event| event.realm_id = vlob_id);
    });

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(&env, &alice).await;

    let certificates = env
        .template
        .certificates()
        .map(|e| e.signed)
        .collect::<Vec<_>>();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.offset, 0);

            authenticated_cmds::latest::certificate_get::Rep::Ok { certificates }
        },
    );

    let message = MessageContent::SharingReencrypted {
        author: alice.device_id.clone(),
        timestamp: now,
        id: vlob_id,
        encrypted_on: now,
        key: alice.local_symkey.clone(),
        name: "wksp1".parse().unwrap(),
        encryption_revision: 0,
    };

    let msg = ops
        .validate_message(
            3,
            0,
            &alice.device_id,
            now,
            &message.dump_sign_and_encrypt_for(&alice.signing_key, &alice.public_key()),
        )
        .await
        .unwrap();

    p_assert_eq!(msg, message);

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .validate_message(1, 0, &alice.device_id, now, b"")
        .await
        .unwrap_err();

    p_assert_matches!(err, ValidateMessageError::Offline);

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn non_existent_author(env: &TestbedEnv) {
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .validate_message(0, 0, &alice.device_id, now, b"")
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        ValidateMessageError::InvalidMessage(InvalidMessageError::NonExistentAuthor { .. })
    );

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn corrupted(env: &TestbedEnv) {
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let certificates = env
        .template
        .certificates()
        .map(|e| e.signed)
        .collect::<Vec<_>>();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.offset, 0);

            authenticated_cmds::latest::certificate_get::Rep::Ok { certificates }
        },
    );

    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .validate_message(2, 0, &alice.device_id, now, b"")
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        ValidateMessageError::InvalidMessage(InvalidMessageError::Corrupted { .. })
    );

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn no_corresponding_realm_role_certificate(env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let certificates = env
        .template
        .certificates()
        .map(|e| e.signed)
        .collect::<Vec<_>>();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.offset, 0);

            authenticated_cmds::latest::certificate_get::Rep::Ok { certificates }
        },
    );

    let message = MessageContent::SharingRevoked {
        author: alice.device_id.clone(),
        timestamp: now,
        id: vlob_id,
    };

    let err = ops
        .validate_message(
            2,
            0,
            &alice.device_id,
            now,
            &message.dump_sign_and_encrypt_for(&alice.signing_key, &alice.public_key()),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        ValidateMessageError::InvalidMessage(
            InvalidMessageError::NoCorrespondingRealmRoleCertificate {
                index,
                sender,
                timestamp,
            }
        )
        if index == 0 && sender == alice.device_id && timestamp == now,
    );

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn author_not_allowed_to_reencrypt(env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let certificates = env
        .template
        .certificates()
        .map(|e| e.signed)
        .collect::<Vec<_>>();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.offset, 0);

            authenticated_cmds::latest::certificate_get::Rep::Ok { certificates }
        },
    );

    let message = MessageContent::SharingReencrypted {
        author: alice.device_id.clone(),
        timestamp: now,
        id: vlob_id,
        encrypted_on: now,
        key: alice.local_symkey.clone(),
        name: "wksp1".parse().unwrap(),
        encryption_revision: 0,
    };

    let err = ops
        .validate_message(
            2,
            0,
            &alice.device_id,
            now,
            &message.dump_sign_and_encrypt_for(&alice.signing_key, &alice.public_key()),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        ValidateMessageError::InvalidMessage(
            InvalidMessageError::AuthorNotAllowedToReencrypt {
                index,
                sender,
                timestamp,
            }
        )
        if index == 0 && sender == alice.device_id && timestamp == now,
    );

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn revoked(env: &TestbedEnv) {
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.revoke_user("bob");
    });

    let bob = env.local_device("bob@dev1");

    let ops = certificates_ops_factory(&env, &bob).await;

    let certificates = env
        .template
        .certificates()
        .map(|e| e.signed)
        .collect::<Vec<_>>();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.offset, 0);

            authenticated_cmds::latest::certificate_get::Rep::Ok { certificates }
        },
    );

    let message = MessageContent::Ping {
        author: bob.device_id.clone(),
        timestamp: now,
        ping: "ping".into(),
    };

    let err = ops
        .validate_message(
            5,
            0,
            &bob.device_id,
            now,
            &message.dump_sign_and_encrypt_for(&bob.signing_key, &bob.public_key()),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        ValidateMessageError::InvalidMessage(
            InvalidMessageError::RevokedAuthor {
                index,
                sender,
                timestamp,
            }
        )
        if index == 0 && sender == bob.device_id && timestamp == now,
    );

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn sharing_granted_invalid_timestamp(env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let env = env.customize(|builder| {
        builder.new_user_realm("alice").customize(|event| {
            event.realm_id = vlob_id;
        });
    });

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(&env, &alice).await;

    let certificates = env
        .template
        .certificates()
        .map(|e| e.signed)
        .collect::<Vec<_>>();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.offset, 0);

            authenticated_cmds::latest::certificate_get::Rep::Ok { certificates }
        },
    );

    let message = MessageContent::SharingGranted {
        author: alice.device_id.clone(),
        timestamp: now,
        id: vlob_id,
        encrypted_on: now,
        name: "wksp1".parse().unwrap(),
        encryption_revision: 0,
        key: alice.local_symkey.clone(),
    };

    let err = ops
        .validate_message(
            3,
            0,
            &alice.device_id,
            now,
            &message.dump_sign_and_encrypt_for(&alice.signing_key, &alice.public_key()),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        ValidateMessageError::InvalidMessage(
            InvalidMessageError::NoCorrespondingRealmRoleCertificate { .. }
        )
    );

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn sharing_granted_invalid_realm_id(env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let wrong_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000002").unwrap();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let env = env.customize(|builder| {
        builder.new_user_realm("alice").customize(|event| {
            event.realm_id = vlob_id;
            event.timestamp = now;
        });
    });

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(&env, &alice).await;

    let certificates = env
        .template
        .certificates()
        .map(|e| e.signed)
        .collect::<Vec<_>>();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.offset, 0);

            authenticated_cmds::latest::certificate_get::Rep::Ok { certificates }
        },
    );

    let message = MessageContent::SharingGranted {
        author: alice.device_id.clone(),
        timestamp: now,
        id: wrong_id,
        encrypted_on: now,
        name: "wksp1".parse().unwrap(),
        encryption_revision: 0,
        key: alice.local_symkey.clone(),
    };

    let err = ops
        .validate_message(
            3,
            0,
            &alice.device_id,
            now,
            &message.dump_sign_and_encrypt_for(&alice.signing_key, &alice.public_key()),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        ValidateMessageError::InvalidMessage(
            InvalidMessageError::NoCorrespondingRealmRoleCertificate { .. }
        )
    );

    ops.stop().await;
}

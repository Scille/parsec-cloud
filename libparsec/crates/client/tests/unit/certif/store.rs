// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_platform_storage::certificates::{GetCertificateError, PerTopicLastTimestamps, UpTo};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::certificates_store_factory;

#[parsec_test(testbed = "minimal")]
async fn add_user_certificate(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let alice_user_id = alice.user_id();
    let store = certificates_store_factory(env, &alice).await;

    let (alice_certif, alice_signed) = env.get_user_certificate("alice");

    macro_rules! store_get_certif {
        ($store:ident) => {{
            let alice_user_id = alice_user_id.clone();
            $store.for_read(|store| async move {
                store
                    .get_user_certificate(UpTo::Current, alice_user_id)
                    .await
            })
        }};
    }

    // Check that certificate is absent

    let err = store_get_certif!(store).await.unwrap().unwrap_err();
    p_assert_matches!(err, GetCertificateError::NonExisting);

    // Add new certificate

    store
        .for_write({
            let alice_certif = alice_certif.clone();
            |store| async move {
                store
                    .add_next_common_certificate(
                        CommonTopicArcCertificate::User(alice_certif),
                        &alice_signed,
                    )
                    .await
            }
        })
        .await
        .unwrap()
        .unwrap();

    // Check that certificate is present

    let got = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(got, alice_certif);

    store.stop().await.unwrap();

    // Ensure we don't rely on a cache but on data in persistent database

    let store = certificates_store_factory(env, &alice).await;
    let got = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(got, alice_certif);
}

#[parsec_test(testbed = "minimal")]
async fn add_device_certificate(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let alice_device_id = alice.device_id.clone();
    let store = certificates_store_factory(env, &alice).await;

    let (alice_dev1_certif, alice_dev1_signed) = env.get_device_certificate("alice@dev1");

    macro_rules! store_get_certif {
        ($store:ident) => {{
            let alice_device_id = alice_device_id.clone();
            $store.for_read(|store| async move {
                store
                    .get_device_certificate(UpTo::Current, alice_device_id)
                    .await
            })
        }};
    }

    // Check that certificate is absent

    let err = store_get_certif!(store).await.unwrap().unwrap_err();
    p_assert_matches!(err, GetCertificateError::NonExisting);

    // Add new certificate

    store
        .for_write({
            let alice_dev1_certif = alice_dev1_certif.clone();
            |store| async move {
                store
                    .add_next_common_certificate(
                        CommonTopicArcCertificate::Device(alice_dev1_certif),
                        &alice_dev1_signed,
                    )
                    .await
            }
        })
        .await
        .unwrap()
        .unwrap();

    // Check that certificate is present

    let got = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(got, alice_dev1_certif);

    store.stop().await.unwrap();

    // Ensure we don't rely on a cache but on data in persistent database

    let store = certificates_store_factory(env, &alice).await;
    let got = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(got, alice_dev1_certif);
}

#[parsec_test(testbed = "minimal")]
async fn add_user_update_certificate(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Admin);
        builder.update_user_profile("alice", UserProfile::Outsider);
    });

    let alice = env.local_device("alice@dev1");
    let alice_user_id = alice.user_id();
    let store = certificates_store_factory(&env, &alice).await;

    let (alice_update_certif, alice_update_signed) = env.get_last_user_update_certificate("alice");

    macro_rules! store_get_certif {
        ($store:ident) => {{
            let alice_user_id = alice_user_id.clone();
            $store.for_read(|store| async move {
                store
                    .get_last_user_update_certificate(UpTo::Current, alice_user_id)
                    .await
            })
        }};
    }

    // Check that certificate is absent

    let err = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_matches!(err, None);

    // Add new certificate

    store
        .for_write({
            let alice_update_certif = alice_update_certif.clone();
            |store| async move {
                store
                    .add_next_common_certificate(
                        CommonTopicArcCertificate::UserUpdate(alice_update_certif),
                        &alice_update_signed,
                    )
                    .await
            }
        })
        .await
        .unwrap()
        .unwrap();

    // Check that certificate is present

    let got = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(got, Some(alice_update_certif.clone()));

    store.stop().await.unwrap();

    // Ensure we don't rely on a cache but on data in persistent database

    let store = certificates_store_factory(&env, &alice).await;
    let got = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(got, Some(alice_update_certif));
}

#[parsec_test(testbed = "minimal")]
async fn add_revoked_user_certificate(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.revoke_user("bob");
    });

    let alice = env.local_device("alice@dev1");
    let bob_user_id: UserID = "bob".parse().unwrap();
    let store = certificates_store_factory(&env, &alice).await;

    let (bob_revoked_certif, bob_revoked_signed) = env.get_revoked_certificate("bob");

    macro_rules! store_get_certif {
        ($store:ident) => {{
            let bob_user_id = bob_user_id.clone();
            $store.for_read(|store| async move {
                store
                    .get_revoked_user_certificate(UpTo::Current, bob_user_id)
                    .await
            })
        }};
    }

    // Check that certificate is absent

    let err = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_matches!(err, None);

    // Add new certificate

    store
        .for_write({
            let bob_revoked_certif = bob_revoked_certif.clone();
            |store| async move {
                store
                    .add_next_common_certificate(
                        CommonTopicArcCertificate::RevokedUser(bob_revoked_certif),
                        &bob_revoked_signed,
                    )
                    .await
            }
        })
        .await
        .unwrap()
        .unwrap();

    // Check that certificate is present

    let got = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(got, Some(bob_revoked_certif.clone()));

    store.stop().await.unwrap();

    // Ensure we don't rely on a cache but on data in persistent database

    let store = certificates_store_factory(&env, &alice).await;
    let got = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(got, Some(bob_revoked_certif));
}

#[parsec_test(testbed = "minimal")]
async fn add_realm_role_certificate(env: &TestbedEnv) {
    let (env, realm_id) =
        env.customize_with_map(|builder| builder.new_user_realm("alice").map(|e| e.realm_id));

    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(&env, &alice).await;

    let (alice_realm_role_certif, alice_realm_role_signed) =
        env.get_last_realm_role_certificate("alice", realm_id);

    macro_rules! store_get_certif {
        ($store:ident) => {{
            $store.for_read(
                |store| async move { store.get_realm_roles(UpTo::Current, realm_id).await },
            )
        }};
    }

    // Check that certificate is absent

    let err = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(err, vec![]);

    // Add new certificate

    store
        .for_write({
            let alice_realm_role_certif = alice_realm_role_certif.clone();
            |store| async move {
                store
                    .add_next_realm_x_certificate(
                        RealmTopicArcCertificate::RealmRole(alice_realm_role_certif),
                        &alice_realm_role_signed,
                    )
                    .await
            }
        })
        .await
        .unwrap()
        .unwrap();

    // Check that certificate is present

    let got = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(got, vec![alice_realm_role_certif.clone()]);

    store.stop().await.unwrap();

    // Ensure we don't rely on a cache but on data in persistent database

    let store = certificates_store_factory(&env, &alice).await;
    let got = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(got, vec![alice_realm_role_certif]);
}

#[parsec_test(testbed = "empty")]
async fn add_sequester_authority_certificate(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
    });

    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(&env, &alice).await;

    let (authority_certif, authority_signed) = env.get_sequester_authority_certificate();

    macro_rules! store_get_certif {
        ($store:ident) => {
            $store
                .for_read(|store| async move { store.get_sequester_authority_certificate().await })
        };
    }

    // Check that certificate is absent

    let err = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_matches!(err, None);

    // Add new certificate

    store
        .for_write({
            let authority_certif = authority_certif.clone();
            |store| async move {
                store
                    .add_next_sequester_certificate(
                        SequesterTopicArcCertificate::SequesterAuthority(authority_certif),
                        &authority_signed,
                    )
                    .await
            }
        })
        .await
        .unwrap()
        .unwrap();

    // Check that certificate is present

    let got = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(got, Some(authority_certif.clone()));

    store.stop().await.unwrap();

    // Ensure we don't rely on a cache but on data in persistent database

    let store = certificates_store_factory(&env, &alice).await;
    let got = store_get_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(got, Some(authority_certif));
}

#[parsec_test(testbed = "empty")]
async fn add_sequester_service_certificate(env: &TestbedEnv) {
    let (env, service_id) = env.customize_with_map(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder.new_sequester_service().map(|e| e.id)
    });

    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(&env, &alice).await;

    let (authority_certif, _) = env.get_sequester_authority_certificate();
    let (service_certif, service_signed) = env.get_sequester_service_certificate(service_id);

    macro_rules! store_get_service_certif {
        ($store:ident) => {
            $store.for_read(|store| async move {
                store
                    .get_sequester_service_certificates(UpTo::Current)
                    .await
            })
        };
    }

    // Remember store doesn't validate the certificate, hence if we add to the
    // storage the sequester service certificate first, it will be considered
    // as the authority certificate...
    macro_rules! ensure_authority_certif_in_store_hasnt_changed {
        ($store:ident) => {
            let got = $store
                .for_read(|store| async move { store.get_sequester_authority_certificate().await })
                .await
                .unwrap()
                .unwrap();
            p_assert_eq!(got, Some(authority_certif.clone()));
        };
    }

    // Check that certificate is absent

    let err = store_get_service_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(err, vec![]);
    ensure_authority_certif_in_store_hasnt_changed!(store);

    // Add new certificate

    store
        .for_write({
            let service_certif = service_certif.clone();
            |store| async move {
                store
                    .add_next_sequester_certificate(
                        SequesterTopicArcCertificate::SequesterService(service_certif),
                        &service_signed,
                    )
                    .await
            }
        })
        .await
        .unwrap()
        .unwrap();

    // Check that certificate is present

    let got = store_get_service_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(got, vec![service_certif.clone()]);
    ensure_authority_certif_in_store_hasnt_changed!(store);

    store.stop().await.unwrap();

    // Ensure we don't rely on a cache but on data in persistent database

    let store = certificates_store_factory(&env, &alice).await;
    let got = store_get_service_certif!(store).await.unwrap().unwrap();
    p_assert_eq!(got, vec![service_certif]);
    ensure_authority_certif_in_store_hasnt_changed!(store);
}

#[parsec_test(testbed = "minimal")]
async fn get_last_timestamps_common(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(env, &alice).await;

    let (certif, signed) = env.get_user_certificate("alice");

    macro_rules! store_get_last_timestamps {
        ($store:ident) => {
            $store.for_read(|store| async move { store.get_last_timestamps().await })
        };
    }

    let got = store_get_last_timestamps!(store).await.unwrap().unwrap();
    assert_eq!(
        got,
        PerTopicLastTimestamps {
            common: None,
            sequester: None,
            realm: HashMap::default(),
            shamir_recovery: None,
        }
    );

    store
        .for_write({
            let certif = certif.clone();
            |store| async move {
                store
                    .add_next_common_certificate(CommonTopicArcCertificate::User(certif), &signed)
                    .await
            }
        })
        .await
        .unwrap()
        .unwrap();

    // Check the timestamps has changed

    let got = store_get_last_timestamps!(store).await.unwrap().unwrap();
    assert_eq!(
        got,
        PerTopicLastTimestamps {
            common: Some(certif.timestamp),
            sequester: None,
            realm: HashMap::default(),
            shamir_recovery: None,
        }
    );

    store.stop().await.unwrap();

    // Ensure we don't rely on a cache but on data in persistent database

    let store = certificates_store_factory(env, &alice).await;
    let got = store_get_last_timestamps!(store).await.unwrap().unwrap();
    assert_eq!(
        got,
        PerTopicLastTimestamps {
            common: Some(certif.timestamp),
            sequester: None,
            realm: HashMap::default(),
            shamir_recovery: None,
        }
    );
}

#[parsec_test(testbed = "empty")]
async fn get_last_timestamps_sequester(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
    });

    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(&env, &alice).await;

    let (certif, signed) = env.get_sequester_authority_certificate();

    macro_rules! store_get_last_timestamps {
        ($store:ident) => {
            $store.for_read(|store| async move { store.get_last_timestamps().await })
        };
    }

    let got = store_get_last_timestamps!(store).await.unwrap().unwrap();
    assert_eq!(
        got,
        PerTopicLastTimestamps {
            common: None,
            sequester: None,
            realm: HashMap::default(),
            shamir_recovery: None,
        }
    );

    store
        .for_write({
            let certif = certif.clone();
            |store| async move {
                store
                    .add_next_sequester_certificate(
                        SequesterTopicArcCertificate::SequesterAuthority(certif),
                        &signed,
                    )
                    .await
            }
        })
        .await
        .unwrap()
        .unwrap();

    // Check the timestamps has changed

    let got = store_get_last_timestamps!(store).await.unwrap().unwrap();
    assert_eq!(
        got,
        PerTopicLastTimestamps {
            common: None,
            sequester: Some(certif.timestamp),
            realm: HashMap::default(),
            shamir_recovery: None,
        }
    );

    store.stop().await.unwrap();

    // Ensure we don't rely on a cache but on data in persistent database

    let store = certificates_store_factory(&env, &alice).await;
    let got = store_get_last_timestamps!(store).await.unwrap().unwrap();
    assert_eq!(
        got,
        PerTopicLastTimestamps {
            common: None,
            sequester: Some(certif.timestamp),
            realm: HashMap::default(),
            shamir_recovery: None,
        }
    );
}

#[parsec_test(testbed = "minimal")]
async fn get_last_timestamps_realm(env: &TestbedEnv) {
    let (env, realm_id) =
        env.customize_with_map(|builder| builder.new_realm("alice").map(|e| e.realm_id));

    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(&env, &alice).await;

    let (certif, signed) = env.get_last_realm_role_certificate("alice", realm_id);

    macro_rules! store_get_last_timestamps {
        ($store:ident) => {
            $store.for_read(|store| async move { store.get_last_timestamps().await })
        };
    }

    let got = store_get_last_timestamps!(store).await.unwrap().unwrap();
    assert_eq!(
        got,
        PerTopicLastTimestamps {
            common: None,
            sequester: None,
            realm: HashMap::default(),
            shamir_recovery: None,
        }
    );

    store
        .for_write({
            let certif = certif.clone();
            |store| async move {
                store
                    .add_next_realm_x_certificate(
                        RealmTopicArcCertificate::RealmRole(certif),
                        &signed,
                    )
                    .await
            }
        })
        .await
        .unwrap()
        .unwrap();

    // Check the timestamps has changed

    let got = store_get_last_timestamps!(store).await.unwrap().unwrap();
    assert_eq!(
        got,
        PerTopicLastTimestamps {
            common: None,
            sequester: None,
            realm: HashMap::from([(realm_id, certif.timestamp)]),
            shamir_recovery: None,
        }
    );

    store.stop().await.unwrap();

    // Ensure we don't rely on a cache but on data in persistent database

    let store = certificates_store_factory(&env, &alice).await;
    let got = store_get_last_timestamps!(store).await.unwrap().unwrap();
    assert_eq!(
        got,
        PerTopicLastTimestamps {
            common: None,
            sequester: None,
            realm: HashMap::from([(realm_id, certif.timestamp)]),
            shamir_recovery: None,
        }
    );
}

#[parsec_test(testbed = "minimal")]
#[ignore]
async fn get_last_timestamps_shamir(env: &TestbedEnv) {
    // TODO: Shamir not implemented yet !
}

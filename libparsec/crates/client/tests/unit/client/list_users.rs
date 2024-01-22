// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;
use crate::{certif::UserInfo, client::ClientListUsersError};

enum OkKind {
    Vanilla,
    SkipRevoked,
    Offset,
    OffsetTooHigh,
    Limit,
    LimitTooHigh,
    OffsetLimit,
    SkipOffsetLimit,
}

#[parsec_test(testbed = "minimal")]
async fn ok(
    #[values(
        OkKind::Vanilla,
        OkKind::SkipRevoked,
        OkKind::Offset,
        OkKind::OffsetTooHigh,
        OkKind::Limit,
        OkKind::LimitTooHigh,
        OkKind::OffsetLimit,
        OkKind::SkipOffsetLimit
    )]
    kind: OkKind,
    env: &TestbedEnv,
) {
    let env = &env.customize(|builder| {
        // Create additional users with various updates to make sur all those
        // certificates are taken into account
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Standard); // bob@dev1
        builder.revoke_user("bob");
        builder
            .new_user("mallory")
            .with_initial_profile(UserProfile::Outsider); // mallory@dev1
        builder.update_user_profile("mallory", UserProfile::Standard);
        builder.certificates_storage_fetch_certificates("alice@dev1");
    });
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let users = match kind {
        OkKind::Vanilla => client.list_users(false, None, None).await.unwrap(),
        OkKind::SkipRevoked => client.list_users(true, None, None).await.unwrap(),
        OkKind::Limit => client.list_users(false, None, Some(1)).await.unwrap(),
        OkKind::LimitTooHigh => client.list_users(false, None, Some(10)).await.unwrap(),
        OkKind::Offset => client.list_users(false, Some(1), None).await.unwrap(),
        OkKind::OffsetTooHigh => client.list_users(false, Some(10), None).await.unwrap(),
        OkKind::OffsetLimit => client.list_users(false, Some(1), Some(2)).await.unwrap(),
        OkKind::SkipOffsetLimit => client.list_users(true, Some(1), Some(1)).await.unwrap(),
    };

    // Users should be ordered oldest to newest
    let (alice_info, bob_info, mallory_info) = match kind {
        OkKind::Vanilla => {
            p_assert_eq!(users.len(), 3);
            (Some(&users[0]), Some(&users[1]), Some(&users[2]))
        }
        OkKind::SkipRevoked => {
            p_assert_eq!(users.len(), 2);
            (Some(&users[0]), None, Some(&users[1]))
        }
        OkKind::Offset => {
            p_assert_eq!(users.len(), 2);
            (None, Some(&users[0]), Some(&users[1]))
        }
        OkKind::OffsetTooHigh => {
            p_assert_eq!(users.len(), 0);
            (None, None, None)
        }
        OkKind::Limit => {
            p_assert_eq!(users.len(), 1);
            (Some(&users[0]), None, None)
        }
        OkKind::LimitTooHigh => {
            p_assert_eq!(users.len(), 3);
            (Some(&users[0]), Some(&users[1]), Some(&users[2]))
        }
        OkKind::OffsetLimit => {
            p_assert_eq!(users.len(), 2);
            (None, Some(&users[0]), Some(&users[1]))
        }
        OkKind::SkipOffsetLimit => {
            p_assert_eq!(users.len(), 1);
            (None, None, Some(&users[0]))
        }
    };

    // Check Alice
    if let Some(alice_info) = alice_info {
        let UserInfo {
            id,
            human_handle,
            current_profile,
            created_on,
            created_by,
            revoked_on,
            revoked_by,
        } = alice_info;
        p_assert_eq!(*id, "alice".parse().unwrap());
        p_assert_eq!(
            *human_handle,
            "Alicey McAliceFace <alice@example.com>".parse().unwrap()
        );
        p_assert_eq!(*current_profile, UserProfile::Admin);
        p_assert_eq!(*created_on, "2000-01-02T00:00:00Z".parse().unwrap());
        p_assert_eq!(*created_by, None);
        p_assert_eq!(*revoked_on, None);
        p_assert_eq!(*revoked_by, None);
    }

    // Check Bob
    if let Some(bob_info) = bob_info {
        let UserInfo {
            id,
            human_handle,
            current_profile,
            created_on,
            created_by,
            revoked_on,
            revoked_by,
        } = bob_info;
        p_assert_eq!(*id, "bob".parse().unwrap());
        p_assert_eq!(
            *human_handle,
            "Boby McBobFace <bob@example.com>".parse().unwrap()
        );
        p_assert_eq!(*current_profile, UserProfile::Standard);
        p_assert_eq!(*created_on, "2000-01-03T00:00:00Z".parse().unwrap());
        p_assert_eq!(*created_by, Some("alice@dev1".parse().unwrap()));
        p_assert_eq!(*revoked_on, Some("2000-01-04T00:00:00Z".parse().unwrap()));
        p_assert_eq!(*revoked_by, Some("alice@dev1".parse().unwrap()));
    }

    // Check Mallory
    if let Some(mallory_info) = mallory_info {
        let UserInfo {
            id,
            human_handle,
            current_profile,
            created_on,
            created_by,
            revoked_on,
            revoked_by,
        } = mallory_info;
        p_assert_eq!(*id, "mallory".parse().unwrap());
        p_assert_eq!(
            *human_handle,
            "Malloryy McMalloryFace <mallory@example.com>"
                .parse()
                .unwrap()
        );
        p_assert_eq!(*current_profile, UserProfile::Standard);
        p_assert_eq!(*created_on, "2000-01-05T00:00:00Z".parse().unwrap());
        p_assert_eq!(*created_by, Some("alice@dev1".parse().unwrap()));
        p_assert_eq!(*revoked_on, None);
        p_assert_eq!(*revoked_by, None);
    }
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    client.certificates_ops.stop().await.unwrap();

    let err = client.list_users(false, None, None).await.unwrap_err();
    p_assert_matches!(err, ClientListUsersError::Stopped);
}

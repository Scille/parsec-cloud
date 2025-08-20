// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;
use crate::{ClientShareWorkspaceError, WorkspaceUserAccessInfo};

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok_with_author_having_changed_its_role(
    #[values("new_role", "unshared_then_shared_again")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    env.customize(|builder| {
        // Bob is Reader, so he cannot share the realm...

        match kind {
            "new_role" => (),
            "unshared_then_shared_again" => {
                builder.share_realm(wksp1_id, "bob", None);
            }
            unknown => panic!("Unknown kind: {unknown}"),
        }

        // ...but then he gets promoted to Manager
        builder.share_realm(wksp1_id, "bob", RealmRole::Manager);
    })
    .await;

    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    client
        .share_workspace(
            wksp1_id,
            "mallory".parse().unwrap(),
            Some(RealmRole::Reader),
        )
        .await
        .unwrap();
}

#[parsec_test(testbed = "minimal", with_server)]
async fn ok(#[values("author_is_owner", "author_is_manager")] kind: &str, env: &TestbedEnv) {
    let wksp1_id = env
        .customize(|builder| {
            // Alice create the realm, then Bob is the one we will use to share the realm with Mallory
            let wksp1_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation_and_naming("wksp1")
                .map(|e| e.realm);
            builder.new_user("bob");
            builder.new_user("mallory");
            match kind {
                "author_is_owner" => {
                    builder.share_realm(wksp1_id, "bob", RealmRole::Owner);
                }
                "author_is_manager" => {
                    builder.share_realm(wksp1_id, "bob", RealmRole::Manager);
                }
                unknown => panic!("Unknown kind: {unknown}"),
            }
            builder.certificates_storage_fetch_certificates("bob@dev1");
            builder
                .user_storage_local_update("bob@dev1")
                .update_local_workspaces_with_fetched_certificates();
            wksp1_id
        })
        .await;

    let mallory = env.local_device("mallory@dev1");
    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob.clone()).await;

    let cook_access_info =
        |info: Vec<WorkspaceUserAccessInfo>| -> HashMap<UserID, WorkspaceUserAccessInfo> {
            HashMap::from_iter(info.into_iter().map(|info| (info.user_id, info)))
        };

    let mut expected_wksp1_access_info =
        cook_access_info(client.list_workspace_users(wksp1_id).await.unwrap());

    // 1) Give initial access

    client
        .share_workspace(wksp1_id, mallory.user_id, Some(RealmRole::Contributor))
        .await
        .unwrap();

    let wksp1_access_info = cook_access_info(client.list_workspace_users(wksp1_id).await.unwrap());
    expected_wksp1_access_info.insert(
        mallory.user_id,
        WorkspaceUserAccessInfo {
            user_id: mallory.user_id,
            human_handle: mallory.human_handle.clone(),
            current_profile: UserProfile::Standard,
            current_role: RealmRole::Contributor,
        },
    );
    p_assert_eq!(wksp1_access_info, expected_wksp1_access_info);

    // Change again

    client
        .share_workspace(wksp1_id, mallory.user_id, Some(RealmRole::Reader))
        .await
        .unwrap();

    let wksp1_access_info = cook_access_info(client.list_workspace_users(wksp1_id).await.unwrap());
    expected_wksp1_access_info
        .get_mut(&mallory.user_id)
        .unwrap()
        .current_role = RealmRole::Reader;
    p_assert_eq!(wksp1_access_info, expected_wksp1_access_info);

    // Finally unshare

    client
        .share_workspace(wksp1_id, mallory.user_id, None)
        .await
        .unwrap();

    let wksp1_access_info = cook_access_info(client.list_workspace_users(wksp1_id).await.unwrap());
    expected_wksp1_access_info.remove(&mallory.user_id);
    p_assert_eq!(wksp1_access_info, expected_wksp1_access_info);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok_require_bootstrap_before_share(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    // Create a workspace but don't sync it...
    let wid = client
        .create_workspace("wksp2".parse().unwrap())
        .await
        .unwrap();

    client
        .share_workspace(wid, "bob".parse().unwrap(), Some(RealmRole::Reader))
        .await
        .unwrap();
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok_placeholder_workspace(env: &TestbedEnv) {
    let wksp2_id = env
        .customize(|builder| {
            let wksp2_id = builder.new_realm("alice").map(|e| e.realm_id);
            builder.certificates_storage_fetch_certificates("alice@dev1");
            builder
                .user_storage_local_update("alice@dev1")
                .update_local_workspaces_with_fetched_certificates();
            wksp2_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    client
        .share_workspace(wksp2_id, "bob".parse().unwrap(), Some(RealmRole::Reader))
        .await
        .unwrap();
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn to_self(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .share_workspace(wksp1_id, "alice".parse().unwrap(), Some(RealmRole::Reader))
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::RecipientIsSelf);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn to_unknown_user(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .share_workspace(wksp1_id, "mike".parse().unwrap(), Some(RealmRole::Reader))
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::RecipientNotFound);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn to_unknown_workspace(env: &TestbedEnv) {
    let dummy_id: VlobID = VlobID::default();

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .share_workspace(dummy_id, "mallory".parse().unwrap(), Some(RealmRole::Owner))
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::WorkspaceNotFound);
}

// This test requires the server given the check is currently only done on server-side
// (it could be done on client-side as well, but it's not the case for now)
#[parsec_test(testbed = "coolorg", with_server)]
async fn cannot_share_with_revoked_recipient_known_on_client(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    env.customize(|builder| {
        builder.revoke_user("mallory");
        builder.certificates_storage_fetch_certificates("alice@dev1");
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .share_workspace(
            wksp1_id,
            "mallory".parse().unwrap(),
            Some(RealmRole::Reader),
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::RecipientRevoked);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn cannot_share_with_revoked_recipient_unknown_on_client(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    env.customize(|builder| {
        builder.revoke_user("mallory");
        // Client is not aware that Mallory has been revoked !
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .share_workspace(
            wksp1_id,
            "mallory".parse().unwrap(),
            Some(RealmRole::Reader),
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::RecipientRevoked);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn can_unshare_with_revoked_recipient(
    #[values("from_client", "from_server")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    match kind {
        "from_client" => {
            env.customize(|builder| {
                builder.revoke_user("bob");
                builder.certificates_storage_fetch_certificates("alice@dev1");
            })
            .await;
        }
        "from_server" => {
            env.customize(|builder| {
                builder.revoke_user("bob");
            })
            .await;
        }
        unknown => panic!("Unknown kind: {unknown}"),
    }

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    client
        .share_workspace(wksp1_id, "bob".parse().unwrap(), None)
        .await
        .unwrap();

    let wksp1_access_info = client.list_workspace_users(wksp1_id).await.unwrap();
    p_assert_eq!(
        wksp1_access_info,
        [WorkspaceUserAccessInfo {
            user_id: alice.user_id,
            human_handle: alice.human_handle.clone(),
            current_profile: UserProfile::Admin,
            current_role: RealmRole::Owner,
        }]
    );
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn author_not_allowed(#[values("from_client", "from_server")] kind: &str, env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    match kind {
        "from_client" => (),
        "from_server" => {
            env.customize(|builder| {
                // Bob's client thinks he can share the workspace...
                builder.share_realm(wksp1_id, "bob", RealmRole::Manager);
                builder.certificates_storage_fetch_certificates("bob@dev1");
                // ...but his access has been removed in the meantime !
                builder.share_realm(wksp1_id, "bob", None);
            })
            .await;
        }
        unknown => panic!("Unknown kind: {unknown}"),
    };

    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    let err = client
        .share_workspace(
            wksp1_id,
            "mallory".parse().unwrap(),
            Some(RealmRole::Reader),
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::AuthorNotAllowed);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn role_incompatible_with_outsider(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .share_workspace(
            wksp1_id,
            "mallory".parse().unwrap(),
            Some(RealmRole::Manager),
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::RoleIncompatibleWithOutsider);
}

#[parsec_test(testbed = "coolorg")]
async fn offline(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .share_workspace(
            wksp1_id,
            "mallory".parse().unwrap(),
            Some(RealmRole::Reader),
        )
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientShareWorkspaceError::Offline(_));
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn timestamp_out_of_ballpark(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;
    // Mock Alice clock to be 1h too late, but not the one used to communicate with
    // the server (so that only the content is rejected, not the request itself)
    alice.time_provider.mock_time_shifted(-3_600_000_000);
    client.cmds.time_provider.mock_time_shifted(3_600_000_000);

    let err = client
        .share_workspace(
            wksp1_id,
            "mallory".parse().unwrap(),
            Some(RealmRole::Reader),
        )
        .await
        .unwrap_err();
    p_assert_matches!(
        err,
        ClientShareWorkspaceError::TimestampOutOfBallpark { .. }
    );
}

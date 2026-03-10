// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;
use crate::ClientSelfPromoteToWorkspaceOwnerError;

/// Build a workspace where all OWNERs have been revoked so that another user
/// can self-promote.  Returns the realm ID.
///
/// Setup: Alice creates and owns wksp1. Bob is given `bob_role`. Alice is then revoked.
/// Bob should now be eligible to self-promote (if he holds the highest remaining role).
/// Bob is created as Admin so he can sign Alice's revocation certificate.
async fn setup_orphaned_workspace(env: &TestbedEnv, bob_role: RealmRole) -> VlobID {
    env.customize(|builder| {
        let wksp1_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation_and_naming("wksp1")
            .map(|e| e.realm);
        // Bob needs Admin profile to be able to sign Alice's revocation certificate
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Admin);
        builder.share_realm(wksp1_id, "bob", bob_role);
        builder.revoke_user("alice");
        builder.certificates_storage_fetch_certificates("bob@dev1");
        builder
            .user_storage_local_update("bob@dev1")
            .update_local_workspaces_with_fetched_certificates();
        wksp1_id
    })
    .await
}

#[parsec_test(testbed = "minimal", with_server)]
async fn ok(
    #[values("bob_is_manager", "bob_is_contributor", "bob_is_reader")] kind: &str,
    env: &TestbedEnv,
) {
    let bob_role = match kind {
        "bob_is_manager" => RealmRole::Manager,
        "bob_is_contributor" => RealmRole::Contributor,
        "bob_is_reader" => RealmRole::Reader,
        unknown => panic!("Unknown kind: {unknown}"),
    };
    let wksp1_id = setup_orphaned_workspace(env, bob_role).await;

    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    // Before self-promotion, can_self_promote_to_owner should be true
    let workspaces = client.list_workspaces().await;
    let wksp1_info = workspaces.iter().find(|w| w.id == wksp1_id).unwrap();
    p_assert_eq!(wksp1_info.can_self_promote_to_owner, true);

    client
        .self_promote_to_workspace_owner(wksp1_id)
        .await
        .unwrap();

    // After self-promotion, we are now OWNER
    let workspaces = client.list_workspaces().await;
    let wksp1_info = workspaces.iter().find(|w| w.id == wksp1_id).unwrap();
    p_assert_eq!(wksp1_info.current_self_role, RealmRole::Owner);
    p_assert_eq!(wksp1_info.can_self_promote_to_owner, false);
}

#[parsec_test(testbed = "minimal", with_server)]
async fn ok_can_self_promote_is_false_when_non_revoked_owner_exists(env: &TestbedEnv) {
    let wksp1_id = env
        .customize(|builder| {
            let wksp1_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation_and_naming("wksp1")
                .map(|e| e.realm);
            builder.new_user("bob");
            builder.share_realm(wksp1_id, "bob", RealmRole::Manager);
            builder.certificates_storage_fetch_certificates("bob@dev1");
            builder
                .user_storage_local_update("bob@dev1")
                .update_local_workspaces_with_fetched_certificates();
            wksp1_id
        })
        .await;

    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    // Alice is still a non-revoked OWNER, so Bob cannot self-promote
    let workspaces = client.list_workspaces().await;
    let wksp1_info = workspaces.iter().find(|w| w.id == wksp1_id).unwrap();
    p_assert_eq!(wksp1_info.can_self_promote_to_owner, false);
}

#[parsec_test(testbed = "minimal", with_server)]
async fn ok_can_self_promote_is_false_when_not_highest_role(env: &TestbedEnv) {
    // Both Bob (Manager) and Charlie (Reader) are present, Alice is revoked.
    // Only Bob (the highest role) can self-promote, not Charlie.
    let wksp1_id = env
        .customize(|builder| {
            let wksp1_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation_and_naming("wksp1")
                .map(|e| e.realm);
            // Bob needs Admin profile to be able to sign Alice's revocation certificate
            builder
                .new_user("bob")
                .with_initial_profile(UserProfile::Admin);
            builder.new_user("mallory");
            builder.share_realm(wksp1_id, "bob", RealmRole::Manager);
            builder.share_realm(wksp1_id, "mallory", RealmRole::Reader);
            builder.revoke_user("alice");
            builder.certificates_storage_fetch_certificates("mallory@dev1");
            builder
                .user_storage_local_update("mallory@dev1")
                .update_local_workspaces_with_fetched_certificates();
            wksp1_id
        })
        .await;

    let mallory = env.local_device("mallory@dev1");
    let client = client_factory(&env.discriminant_dir, mallory).await;

    // Mallory is Reader while Bob is Manager, so Mallory cannot self-promote
    let workspaces = client.list_workspaces().await;
    let wksp1_info = workspaces.iter().find(|w| w.id == wksp1_id).unwrap();
    p_assert_eq!(wksp1_info.can_self_promote_to_owner, false);
}

#[parsec_test(testbed = "minimal", with_server)]
async fn workspace_not_found(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
    })
    .await;

    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    let dummy_id = VlobID::default();
    let err = client
        .self_promote_to_workspace_owner(dummy_id)
        .await
        .unwrap_err();
    p_assert_matches!(
        err,
        ClientSelfPromoteToWorkspaceOwnerError::WorkspaceNotFound
    );
}

#[parsec_test(testbed = "minimal", with_server)]
async fn author_not_allowed(env: &TestbedEnv) {
    // Setup: Alice (Owner) is revoked. Bob (Admin) is Manager — the highest remaining role.
    // Mallory is Reader — a lower role, so the server will reject her self-promote attempt.
    let wksp1_id = env
        .customize(|builder| {
            let wksp1_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation_and_naming("wksp1")
                .map(|e| e.realm);
            // Bob needs Admin profile to be able to sign Alice's revocation certificate
            builder
                .new_user("bob")
                .with_initial_profile(UserProfile::Admin);
            builder.new_user("mallory");
            builder.share_realm(wksp1_id, "bob", RealmRole::Manager);
            builder.share_realm(wksp1_id, "mallory", RealmRole::Reader);
            builder.revoke_user("alice");
            builder.certificates_storage_fetch_certificates("mallory@dev1");
            builder
                .user_storage_local_update("mallory@dev1")
                .update_local_workspaces_with_fetched_certificates();
            wksp1_id
        })
        .await;

    let mallory = env.local_device("mallory@dev1");
    let client = client_factory(&env.discriminant_dir, mallory).await;

    // Mallory is Reader while Bob is Manager, so the server rejects her self-promote
    let err = client
        .self_promote_to_workspace_owner(wksp1_id)
        .await
        .unwrap_err();
    p_assert_matches!(
        err,
        ClientSelfPromoteToWorkspaceOwnerError::AuthorNotAllowed
    );
}

#[parsec_test(testbed = "minimal", with_server)]
async fn active_owner_already_exists(env: &TestbedEnv) {
    // Setup: Alice (Owner) is revoked. Bob (Admin) and Mallory are both Managers,
    // so both are eligible to self-promote. Bob self-promotes first; then Mallory
    // tries — but Bob is now an active Owner, so the server rejects with
    // ActiveOwnerAlreadyExists.
    let wksp1_id = env
        .customize(|builder| {
            let wksp1_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation_and_naming("wksp1")
                .map(|e| e.realm);
            // Bob needs Admin profile to be able to sign Alice's revocation certificate
            builder
                .new_user("bob")
                .with_initial_profile(UserProfile::Admin);
            builder.new_user("mallory");
            builder.share_realm(wksp1_id, "bob", RealmRole::Manager);
            builder.share_realm(wksp1_id, "mallory", RealmRole::Manager);
            builder.revoke_user("alice");
            // Fetch certs for Bob (knows about Alice's revocation, eligible for self-promote)
            builder.certificates_storage_fetch_certificates("bob@dev1");
            builder
                .user_storage_local_update("bob@dev1")
                .update_local_workspaces_with_fetched_certificates();
            // Fetch certs for Mallory as well (also knows, also eligible)
            builder.certificates_storage_fetch_certificates("mallory@dev1");
            builder
                .user_storage_local_update("mallory@dev1")
                .update_local_workspaces_with_fetched_certificates();
            wksp1_id
        })
        .await;

    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    // Bob self-promotes first — succeeds
    client
        .self_promote_to_workspace_owner(wksp1_id)
        .await
        .unwrap();

    // Mallory's client hasn't fetched Bob's promotion cert yet.
    // When she tries to self-promote the server already has Bob as active Owner.
    let mallory = env.local_device("mallory@dev1");
    let client2 = client_factory(&env.discriminant_dir, mallory).await;

    let err = client2
        .self_promote_to_workspace_owner(wksp1_id)
        .await
        .unwrap_err();
    p_assert_matches!(
        err,
        ClientSelfPromoteToWorkspaceOwnerError::ActiveOwnerAlreadyExists
    );
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let wksp1_id = env
        .customize(|builder| {
            let wksp1_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation_and_naming("wksp1")
                .map(|e| e.realm);
            // Bob needs Admin profile to be able to sign Alice's revocation certificate
            builder
                .new_user("bob")
                .with_initial_profile(UserProfile::Admin);
            builder.share_realm(wksp1_id, "bob", RealmRole::Manager);
            builder.revoke_user("alice");
            builder.certificates_storage_fetch_certificates("bob@dev1");
            builder
                .user_storage_local_update("bob@dev1")
                .update_local_workspaces_with_fetched_certificates();
            wksp1_id
        })
        .await;

    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    let err = client
        .self_promote_to_workspace_owner(wksp1_id)
        .await
        .unwrap_err();
    p_assert_matches!(err, ClientSelfPromoteToWorkspaceOwnerError::Offline(_));
}

#[parsec_test(testbed = "minimal", with_server)]
async fn timestamp_out_of_ballpark(env: &TestbedEnv) {
    let wksp1_id = setup_orphaned_workspace(env, RealmRole::Manager).await;

    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob.clone()).await;
    // Mock Bob's clock to be 1h too late, but not the one used to communicate with
    // the server (so that only the content is rejected, not the request itself)
    bob.time_provider.mock_time_shifted(-3_600_000_000);
    client.cmds.time_provider.mock_time_shifted(3_600_000_000);

    let err = client
        .self_promote_to_workspace_owner(wksp1_id)
        .await
        .unwrap_err();
    p_assert_matches!(
        err,
        ClientSelfPromoteToWorkspaceOwnerError::TimestampOutOfBallpark { .. }
    );
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;
use crate::{certif::WorkspaceUserAccessInfo, client::ClientListWorkspaceUsersError};

enum CertifChange {
    None,
    BobNewUserProfile,
    BobNewRealmRole,
    BobRevoked,
    BobNoRealmRole,
}

#[parsec_test(testbed = "coolorg")]
async fn ok(
    #[values(
        CertifChange::None,
        CertifChange::BobNewUserProfile,
        CertifChange::BobNewRealmRole,
        CertifChange::BobRevoked,
        CertifChange::BobNoRealmRole
    )]
    kind: CertifChange,
    env: &TestbedEnv,
) {
    let env = &env.customize(|builder| {
        match kind {
            CertifChange::None => (),
            CertifChange::BobNewUserProfile => {
                builder.update_user_profile("bob", UserProfile::Outsider);
            }
            CertifChange::BobNewRealmRole => {
                let wksp1_id: VlobID = *builder.get_stuff("wksp1_id");
                builder.share_realm(wksp1_id, "bob", Some(RealmRole::Manager));
            }
            CertifChange::BobRevoked => {
                builder.revoke_user("bob");
            }
            CertifChange::BobNoRealmRole => {
                let wksp1_id: VlobID = *builder.get_stuff("wksp1_id");
                builder.share_realm(wksp1_id, "bob", None);
            }
        }
        builder.certificates_storage_fetch_certificates("alice@dev1");
    });

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let wksp1_users = client.list_workspace_users(wksp1_id).await.unwrap();

    let (alice_entry, bob_entry) = match kind {
        CertifChange::BobRevoked | CertifChange::BobNoRealmRole => {
            p_assert_eq!(wksp1_users.len(), 1);
            (&wksp1_users[0], None)
        }
        _ => {
            p_assert_eq!(wksp1_users.len(), 2);
            let a = &wksp1_users[0];
            let b = &wksp1_users[1];
            if a.user_id.as_ref() == "alice" {
                (a, Some(b))
            } else {
                (b, Some(a))
            }
        }
    };

    // Check Alice
    {
        let WorkspaceUserAccessInfo {
            user_id,
            human_handle,
            current_profile,
            current_role,
        } = alice_entry;
        let (certif, _) = env.get_user_certificate("alice");
        p_assert_eq!(*user_id, certif.user_id);
        p_assert_eq!(
            human_handle,
            match &certif.human_handle {
                MaybeRedacted::Real(human_handle) => human_handle,
                MaybeRedacted::Redacted(_) => unreachable!(),
            }
        );
        p_assert_eq!(*current_profile, UserProfile::Admin);
        p_assert_eq!(*current_role, RealmRole::Owner);
    }

    // Check Bob (if he hasn't been revoked)
    if let Some(bob_entry) = bob_entry {
        let WorkspaceUserAccessInfo {
            user_id,
            human_handle,
            current_profile,
            current_role,
        } = bob_entry;
        let (certif, _) = env.get_user_certificate("bob");
        p_assert_eq!(*user_id, certif.user_id);
        p_assert_eq!(
            human_handle,
            match &certif.human_handle {
                MaybeRedacted::Real(human_handle) => human_handle,
                MaybeRedacted::Redacted(_) => unreachable!(),
            }
        );
        if matches!(kind, CertifChange::BobNewUserProfile) {
            p_assert_eq!(*current_profile, UserProfile::Outsider);
        } else {
            p_assert_eq!(*current_profile, UserProfile::Standard);
        }
        if matches!(kind, CertifChange::BobNewRealmRole) {
            p_assert_eq!(*current_role, RealmRole::Manager);
        } else {
            p_assert_eq!(*current_role, RealmRole::Reader);
        }
    }
}

#[parsec_test(testbed = "minimal")]
async fn unknown_workspace(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let unknown_wksp_id = VlobID::default();

    let unknown_wksp_users = client.list_workspace_users(unknown_wksp_id).await.unwrap();
    p_assert_eq!(unknown_wksp_users.len(), 0);
}

#[parsec_test(testbed = "coolorg")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let client = client_factory(&env.discriminant_dir, alice).await;

    client.certificates_ops.stop().await.unwrap();

    let err: ClientListWorkspaceUsersError =
        client.list_workspace_users(wksp1_id).await.unwrap_err();
    p_assert_matches!(err, ClientListWorkspaceUsersError::Stopped);
}

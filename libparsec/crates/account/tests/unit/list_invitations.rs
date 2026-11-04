// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_client_connection::{
    protocol::authenticated_account_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountListInvitationsError};

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    let expected_invitations = vec![
        ParsecInvitationAddr::new(
            account.cmds.addr().to_owned(),
            "Org1".parse().unwrap(),
            InvitationType::User,
            AccessToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
        ),
        ParsecInvitationAddr::new(
            account.cmds.addr().to_owned(),
            "Org2".parse().unwrap(),
            InvitationType::Device,
            AccessToken::from_hex("e4eaf751ba3546a899a9088002e51918").unwrap(),
        ),
    ];

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let invitations = expected_invitations
            .iter()
            .map(|invitation| {
                (
                    invitation.organization_id().clone(),
                    invitation.token(),
                    invitation.invitation_type(),
                )
            })
            .collect();
        move |_req: authenticated_account_cmds::latest::invite_self_list::Req| {
            authenticated_account_cmds::latest::invite_self_list::Rep::Ok { invitations }
        }
    });

    let invitations = account.list_invitations().await.unwrap();
    p_assert_eq!(invitations, expected_invitations);
}

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    p_assert_matches!(
        account.list_invitations().await,
        Err(AccountListInvitationsError::Offline(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn unknown_server_response(env: &TestbedEnv) {
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: authenticated_account_cmds::latest::invite_self_list::Req| {
            authenticated_account_cmds::latest::invite_self_list::Rep::UnknownStatus {
                unknown_status: "unknown".to_string(),
                reason: None,
            }
        }
    );

    p_assert_matches!(
        account.list_invitations().await,
        Err(AccountListInvitationsError::Internal(err))
        if format!("{err}") == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}

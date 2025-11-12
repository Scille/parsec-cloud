// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_client_connection::{
    protocol::authenticated_account_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    Account, AccountListOrganizationsError, AccountOrganizations, AccountOrganizationsActiveUser,
    AccountOrganizationsOrganizationConfig, AccountOrganizationsRevokedUser,
};

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

    let expected = AccountOrganizations {
        active: vec![
            AccountOrganizationsActiveUser {
                organization_id: "Org1".parse().unwrap(),
                user_id: UserID::from_hex("070da3121be448c0be9c307b6facf856").unwrap(),
                created_on: "2001-01-01T00:00:00Z".parse().unwrap(),
                is_frozen: false,
                current_profile: UserProfile::Admin,
                organization_config: AccountOrganizationsOrganizationConfig {
                    is_expired: false,
                    user_profile_outsider_allowed: true,
                    active_users_limit: ActiveUsersLimit::NoLimit,
                },
            },
            AccountOrganizationsActiveUser {
                organization_id: "Org2".parse().unwrap(),
                user_id: UserID::from_hex("fdc5c8a569074a1daa6526ae34f36109").unwrap(),
                created_on: "2002-01-01T00:00:00Z".parse().unwrap(),
                is_frozen: true,
                current_profile: UserProfile::Standard,
                organization_config: AccountOrganizationsOrganizationConfig {
                    is_expired: true,
                    user_profile_outsider_allowed: false,
                    active_users_limit: ActiveUsersLimit::LimitedTo(8),
                },
            },
        ],
        revoked: vec![AccountOrganizationsRevokedUser {
            organization_id: "Org3".parse().unwrap(),
            user_id: UserID::from_hex("c351954aa0104d74833af419769c45b5").unwrap(),
            created_on: "2003-01-01T00:00:00Z".parse().unwrap(),
            revoked_on: "2003-01-02T00:00:00Z".parse().unwrap(),
            current_profile: UserProfile::Standard,
        }],
    };

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let expected = expected.clone();
        move |_req: authenticated_account_cmds::latest::organization_self_list::Req| {
            authenticated_account_cmds::latest::organization_self_list::Rep::Ok {
                active: expected.active,
                revoked: expected.revoked,
            }
        }
    });

    let orgs = account.list_organizations().await.unwrap();
    p_assert_eq!(orgs, expected);
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
        account.list_organizations().await,
        Err(AccountListOrganizationsError::Offline(_))
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
        move |_req: authenticated_account_cmds::latest::organization_self_list::Req| {
            authenticated_account_cmds::latest::organization_self_list::Rep::UnknownStatus {
                unknown_status: "unknown".to_string(),
                reason: None,
            }
        }
    );

    p_assert_matches!(
        account.list_organizations().await,
        Err(AccountListOrganizationsError::Internal(err))
        if format!("{err}") == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}

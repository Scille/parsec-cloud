// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_vault_item_list,
};
use libparsec_protocol::authenticated_account_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    Account, AccountOrganizationsAccountVaultStrategy, AccountOrganizationsActiveUser,
    AccountOrganizationsAllowedClientAgent, AccountOrganizationsOrganizationConfig,
    AccountUploadOpaqueKeyInVaultError,
};

#[parsec_test(testbed = "empty")]
async fn ok_mocked(env: &TestbedEnv) {
    let org_id: OrganizationID = "Org1".parse().unwrap();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    let vault_key = SecretKey::generate();
    let expected_stuff: Arc<Mutex<Option<(AccountVaultItemOpaqueKeyID, SecretKey)>>> =
        Default::default();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key),
        {
            let active = AccountOrganizationsActiveUser {
                organization_id: org_id.clone(),
                user_id: UserID::from_hex("070da3121be448c0be9c307b6facf856").unwrap(),
                created_on: "2001-01-01T00:00:00Z".parse().unwrap(),
                is_frozen: false,
                current_profile: UserProfile::Admin,
                organization_config: AccountOrganizationsOrganizationConfig {
                    is_expired: false,
                    user_profile_outsider_allowed: true,
                    active_users_limit: ActiveUsersLimit::NoLimit,
                    allowed_client_agent: AccountOrganizationsAllowedClientAgent::NativeOrWeb,
                    account_vault_strategy: AccountOrganizationsAccountVaultStrategy::Allowed,
                },
            };
            move |_req: authenticated_account_cmds::latest::organization_self_list::Req| {
                authenticated_account_cmds::latest::organization_self_list::Rep::Ok {
                    active: vec![active],
                    revoked: vec![],
                }
            }
        },
        {
            let vault_key = vault_key.clone();
            let expected_stuff = expected_stuff.clone();

            move |req: authenticated_account_cmds::latest::vault_item_upload::Req| {
                let item = AccountVaultItem::load(&req.item).unwrap();
                p_assert_eq!(req.item_fingerprint, item.fingerprint());
                match item {
                    AccountVaultItem::OpaqueKey(item) => {
                        let key_access = AccountVaultItemOpaqueKeyEncryptedData::decrypt_and_load(
                            &item.encrypted_data,
                            &vault_key,
                        )
                        .unwrap();
                        p_assert_eq!(item.key_id, key_access.key_id);
                        *expected_stuff.lock().unwrap() = Some((item.key_id, key_access.key));
                    }
                    AccountVaultItem::RegistrationDevice(unexpected) => {
                        unreachable!("{:?}", unexpected)
                    }
                }

                authenticated_account_cmds::latest::vault_item_upload::Rep::Ok
            }
        },
    );

    let (ciphertext_key_id, ciphertext_key) =
        account.upload_opaque_key_in_vault(&org_id).await.unwrap();

    let (expected_ciphertext_key_id, expected_ciphertext_key) =
        expected_stuff.lock().unwrap().clone().unwrap();
    p_assert_eq!(ciphertext_key_id, expected_ciphertext_key_id);
    p_assert_eq!(ciphertext_key, expected_ciphertext_key);
}

#[parsec_test(testbed = "empty")]
async fn offline(
    #[values(
        "during_vault_item_list",
        "during_organization_self_list",
        "during_vault_item_upload"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let org_id: OrganizationID = "Org1".parse().unwrap();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    match kind {
        "during_vault_item_list" => {
            // No send hook, so initial `vault_item_list` will fail
        }

        "during_organization_self_list" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                test_send_hook_vault_item_list!(
                    env,
                    &account.auth_method_secret_key,
                    &SecretKey::generate()
                ),
                // `organization_self_list` is missing !
            );
        }

        "during_vault_item_upload" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                test_send_hook_vault_item_list!(
                    env,
                    &account.auth_method_secret_key,
                    &SecretKey::generate()
                ),
                {
                    let active = AccountOrganizationsActiveUser {
                        organization_id: org_id.clone(),
                        user_id: UserID::from_hex("070da3121be448c0be9c307b6facf856").unwrap(),
                        created_on: "2001-01-01T00:00:00Z".parse().unwrap(),
                        is_frozen: false,
                        current_profile: UserProfile::Admin,
                        organization_config: AccountOrganizationsOrganizationConfig {
                            is_expired: false,
                            user_profile_outsider_allowed: true,
                            active_users_limit: ActiveUsersLimit::NoLimit,
                            allowed_client_agent:
                                AccountOrganizationsAllowedClientAgent::NativeOrWeb,
                            account_vault_strategy:
                                AccountOrganizationsAccountVaultStrategy::Allowed,
                        },
                    };
                    move |_req: authenticated_account_cmds::latest::organization_self_list::Req| {
                        authenticated_account_cmds::latest::organization_self_list::Rep::Ok {
                            active: vec![active],
                            revoked: vec![],
                        }
                    }
                },
                // `vault_item_upload` is missing !
            );
        }

        unknown => panic!("Unknown kind: {unknown}"),
    }

    p_assert_matches!(
        account.upload_opaque_key_in_vault(&org_id).await,
        Err(AccountUploadOpaqueKeyInVaultError::Offline(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn fingerprint_already_exists(env: &TestbedEnv) {
    let org_id: OrganizationID = "Org1".parse().unwrap();
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
        test_send_hook_vault_item_list!(
            env,
            &account.auth_method_secret_key,
            &SecretKey::generate()
        ),
        {
            let active = AccountOrganizationsActiveUser {
                organization_id: org_id.clone(),
                user_id: UserID::from_hex("070da3121be448c0be9c307b6facf856").unwrap(),
                created_on: "2001-01-01T00:00:00Z".parse().unwrap(),
                is_frozen: false,
                current_profile: UserProfile::Admin,
                organization_config: AccountOrganizationsOrganizationConfig {
                    is_expired: false,
                    user_profile_outsider_allowed: true,
                    active_users_limit: ActiveUsersLimit::NoLimit,
                    allowed_client_agent: AccountOrganizationsAllowedClientAgent::NativeOrWeb,
                    account_vault_strategy: AccountOrganizationsAccountVaultStrategy::Allowed,
                },
            };
            move |_req: authenticated_account_cmds::latest::organization_self_list::Req| {
                authenticated_account_cmds::latest::organization_self_list::Rep::Ok {
                    active: vec![active],
                    revoked: vec![],
                }
            }
        },
        move |_req: authenticated_account_cmds::latest::vault_item_upload::Req| {
            authenticated_account_cmds::latest::vault_item_upload::Rep::FingerprintAlreadyExists
        }
    );

    p_assert_matches!(
        account.upload_opaque_key_in_vault(&org_id).await,
        Err(AccountUploadOpaqueKeyInVaultError::Internal(err))
        if format!("{err}") == "Unexpected server response: FingerprintAlreadyExists"
    );
}

#[parsec_test(testbed = "empty")]
async fn bad_vault_key_access(env: &TestbedEnv) {
    let org_id: OrganizationID = "Org1".parse().unwrap();
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
        move |_req: authenticated_account_cmds::latest::vault_item_list::Req| {
            authenticated_account_cmds::latest::vault_item_list::Rep::Ok {
                key_access: Bytes::from_static(b"<invalid>"),
                items: HashMap::new(),
            }
        }
    );

    p_assert_matches!(
        account.upload_opaque_key_in_vault(&org_id).await,
        Err(AccountUploadOpaqueKeyInVaultError::BadVaultKeyAccess(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn unknown_server_response(
    #[values(
        "during_vault_item_list",
        "during_organization_self_list",
        "during_vault_item_upload"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let org_id: OrganizationID = "Org1".parse().unwrap();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    match kind {
        "during_vault_item_list" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                move |_req: authenticated_account_cmds::latest::vault_item_list::Req| {
                    authenticated_account_cmds::latest::vault_item_list::Rep::UnknownStatus {
                        unknown_status: "unknown".to_string(),
                        reason: None,
                    }
                }
            );
        }

        "during_organization_self_list" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                test_send_hook_vault_item_list!(
                    env,
                    &account.auth_method_secret_key,
                    &SecretKey::generate()
                ),
                move |_req: authenticated_account_cmds::latest::organization_self_list::Req| {
                    authenticated_account_cmds::latest::organization_self_list::Rep::UnknownStatus {
                        unknown_status: "unknown".to_string(),
                        reason: None,
                    }
                }
            );
        }

        "during_vault_item_upload" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                test_send_hook_vault_item_list!(
                    env,
                    &account.auth_method_secret_key,
                    &SecretKey::generate()
                ),
                {
                    let active = AccountOrganizationsActiveUser {
                        organization_id: org_id.clone(),
                        user_id: UserID::from_hex("070da3121be448c0be9c307b6facf856").unwrap(),
                        created_on: "2001-01-01T00:00:00Z".parse().unwrap(),
                        is_frozen: false,
                        current_profile: UserProfile::Admin,
                        organization_config: AccountOrganizationsOrganizationConfig {
                            is_expired: false,
                            user_profile_outsider_allowed: true,
                            active_users_limit: ActiveUsersLimit::NoLimit,
                            allowed_client_agent:
                                AccountOrganizationsAllowedClientAgent::NativeOrWeb,
                            account_vault_strategy:
                                AccountOrganizationsAccountVaultStrategy::Allowed,
                        },
                    };
                    move |_req: authenticated_account_cmds::latest::organization_self_list::Req| {
                        authenticated_account_cmds::latest::organization_self_list::Rep::Ok {
                            active: vec![active],
                            revoked: vec![],
                        }
                    }
                },
                move |_req: authenticated_account_cmds::latest::vault_item_upload::Req| {
                    authenticated_account_cmds::latest::vault_item_upload::Rep::UnknownStatus {
                        unknown_status: "unknown".to_string(),
                        reason: None,
                    }
                },
            );
        }

        unknown => panic!("Unknown kind: {unknown}"),
    }

    p_assert_matches!(
        account.upload_opaque_key_in_vault(&org_id).await,
        Err(AccountUploadOpaqueKeyInVaultError::Internal(err))
        if format!("{err}") == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}

#[parsec_test(testbed = "empty")]
async fn cannot_obtain_organization_vault_strategy(env: &TestbedEnv) {
    let org_id: OrganizationID = "Org1".parse().unwrap();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    let vault_key = SecretKey::generate();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key),
        move |_req: authenticated_account_cmds::latest::organization_self_list::Req| {
            authenticated_account_cmds::latest::organization_self_list::Rep::Ok {
                active: vec![],
                revoked: vec![],
            }
        }
    );

    p_assert_matches!(
        account.upload_opaque_key_in_vault(&org_id).await,
        Err(AccountUploadOpaqueKeyInVaultError::CannotObtainOrganizationVaultStrategy)
    );
}

#[parsec_test(testbed = "empty")]
async fn not_allowed_by_organization_vault_strategy(env: &TestbedEnv) {
    let org_id: OrganizationID = "Org1".parse().unwrap();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    let vault_key = SecretKey::generate();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key),
        {
            let active = AccountOrganizationsActiveUser {
                organization_id: org_id.clone(),
                user_id: UserID::from_hex("070da3121be448c0be9c307b6facf856").unwrap(),
                created_on: "2001-01-01T00:00:00Z".parse().unwrap(),
                is_frozen: false,
                current_profile: UserProfile::Admin,
                organization_config: AccountOrganizationsOrganizationConfig {
                    is_expired: false,
                    user_profile_outsider_allowed: true,
                    active_users_limit: ActiveUsersLimit::NoLimit,
                    allowed_client_agent: AccountOrganizationsAllowedClientAgent::NativeOrWeb,
                    account_vault_strategy: AccountOrganizationsAccountVaultStrategy::Forbidden, // !!!
                },
            };
            move |_req: authenticated_account_cmds::latest::organization_self_list::Req| {
                authenticated_account_cmds::latest::organization_self_list::Rep::Ok {
                    active: vec![active],
                    revoked: vec![],
                }
            }
        },
    );

    p_assert_matches!(
        account.upload_opaque_key_in_vault(&org_id).await,
        Err(AccountUploadOpaqueKeyInVaultError::NotAllowedByOrganizationVaultStrategy)
    );
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, sync::Arc};

use super::utils::client_factory;
use libparsec_platform_async::future;
use libparsec_platform_device_loader::{AvailableDeviceType, DeviceSaveStrategy};
use libparsec_protocol::invited_cmds::latest::invite_info::ShamirRecoveryRecipient;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    claimer_retrieve_info, AnyClaimRetrievedInfoCtx, ClaimerRetrieveInfoError, ClientConfig,
    MountpointMountStrategy, ProxyConfig, ShamirRecoveryClaimAddShareError,
    ShamirRecoveryClaimMaybeFinalizeCtx, ShamirRecoveryClaimMaybeRecoverDeviceCtx,
    ShamirRecoveryClaimPickRecipientError, ShamirRecoveryClaimShare, WorkspaceStorageCacheSize,
};

#[parsec_test(testbed = "shamir", with_server)]
async fn shamir_full_greeting(tmp_path: TmpPath, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let mallory = env.local_device("mallory@dev1");
    let mike = env.local_device("mike@dev1");

    // Revoke Mallory first

    let alice_client = client_factory(&env.discriminant_dir, alice.clone()).await;
    alice_client.revoke_user(mallory.user_id).await.unwrap();
    let mallory_revoked_on = alice_client
        .list_users(false, None, None)
        .await
        .unwrap()
        .iter()
        .find(|&u| u.id == mallory.user_id)
        .unwrap()
        .revoked_on;

    // Start the alice claimer workflow

    let alice_token = env
        .template
        .events
        .iter()
        .find_map(|e| match e {
            TestbedEvent::NewShamirRecoveryInvitation(event) if event.claimer == alice.user_id => {
                Some(event.token)
            }
            _ => None,
        })
        .unwrap();

    let addr = ParsecInvitationAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        libparsec_types::InvitationType::User,
        alice_token,
    );

    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        with_monitors: false,
        prevent_sync_pattern: PreventSyncPattern::from_regex(r"\.tmp$").unwrap(),
    });

    let alice_ctx = claimer_retrieve_info(config, addr, None).await.unwrap();
    p_assert_matches!(&alice_ctx, AnyClaimRetrievedInfoCtx::ShamirRecovery(_));
    let alice_recipient_pick_ctx = match alice_ctx {
        AnyClaimRetrievedInfoCtx::ShamirRecovery(alice_ctx) => {
            p_assert_eq!(alice_ctx.claimer_user_id(), alice.user_id);
            p_assert_eq!(*alice_ctx.claimer_human_handle(), alice.human_handle);
            p_assert_eq!(
                *alice_ctx.recipients(),
                vec![
                    ShamirRecoveryRecipient {
                        user_id: bob.user_id,
                        human_handle: bob.human_handle.clone(),
                        shares: 2.try_into().unwrap(),
                        revoked_on: None,
                        online_status: libparsec_protocol::invited_cmds::latest::invite_info::UserOnlineStatus::Unknown,
                    },
                    ShamirRecoveryRecipient {
                        user_id: mallory.user_id,
                        human_handle: mallory.human_handle.clone(),
                        shares: 1.try_into().unwrap(),
                        revoked_on: mallory_revoked_on,
                        online_status: libparsec_protocol::invited_cmds::latest::invite_info::UserOnlineStatus::Unknown,
                    },
                    ShamirRecoveryRecipient {
                        user_id: mike.user_id,
                        human_handle: mike.human_handle.clone(),
                        shares: 1.try_into().unwrap(),
                        revoked_on: None,
                        online_status: libparsec_protocol::invited_cmds::latest::invite_info::UserOnlineStatus::Unknown,
                    }
                ]
            );
            p_assert_eq!(alice_ctx.threshold(), 2.try_into().unwrap());
            p_assert_eq!(alice_ctx.retrieved_shares(), HashMap::new());
            assert!(alice_ctx.is_recoverable());
            alice_ctx
        }
        _ => unreachable!(),
    };

    // Start with Mike

    let alice_with_mike_ctx = alice_recipient_pick_ctx
        .pick_recipient(mike.user_id)
        .unwrap();

    let mike_client = client_factory(&env.discriminant_dir, mike.clone()).await;
    let mike_ctx = mike_client
        .start_shamir_recovery_invitation_greet(alice_token)
        .await
        .unwrap();

    let (first, second) =
        future::join(alice_with_mike_ctx.do_wait_peer(), mike_ctx.do_wait_peer()).await;
    let alice_with_mike_ctx = first.unwrap();
    let mike_ctx = second.unwrap();

    p_assert_eq!(mike_ctx.greeter_sas(), alice_with_mike_ctx.greeter_sas());
    assert!(alice_with_mike_ctx
        .generate_greeter_sas_choices(4)
        .contains(mike_ctx.greeter_sas()));

    let (first, second) = future::join(
        alice_with_mike_ctx.do_signify_trust(),
        mike_ctx.do_wait_peer_trust(),
    )
    .await;
    let alice_with_mike_ctx = first.unwrap();
    let mike_ctx = second.unwrap();

    p_assert_eq!(mike_ctx.claimer_sas(), alice_with_mike_ctx.claimer_sas());
    assert!(mike_ctx
        .generate_claimer_sas_choices(4)
        .contains(alice_with_mike_ctx.claimer_sas()));

    let (first, second) = future::join(
        alice_with_mike_ctx.do_wait_peer_trust(),
        mike_ctx.do_signify_trust(),
    )
    .await;
    let alice_with_mike_ctx = first.unwrap();
    let mike_ctx = second.unwrap();

    let (first, second) = future::join(
        alice_with_mike_ctx.do_recover_share(),
        mike_ctx.do_send_share(),
    )
    .await;
    let alice_share_ctx = first.unwrap();
    second.unwrap();

    let alice_recipient_pick_ctx =
        match alice_recipient_pick_ctx.add_share(alice_share_ctx).unwrap() {
            ShamirRecoveryClaimMaybeRecoverDeviceCtx::PickRecipient(ctx) => ctx,
            _ => panic!("Expected PickRecipient context"),
        };
    p_assert_eq!(alice_recipient_pick_ctx.retrieved_shares().len(), 1);

    // Continue with Bob

    let alice_with_bob_ctx = alice_recipient_pick_ctx
        .pick_recipient(bob.user_id)
        .unwrap();

    let bob_client = client_factory(&env.discriminant_dir, bob.clone()).await;
    let bob_ctx = bob_client
        .start_shamir_recovery_invitation_greet(alice_token)
        .await
        .unwrap();

    let (first, second) =
        future::join(alice_with_bob_ctx.do_wait_peer(), bob_ctx.do_wait_peer()).await;
    let alice_with_bob_ctx = first.unwrap();
    let bob_ctx = second.unwrap();

    p_assert_eq!(bob_ctx.greeter_sas(), alice_with_bob_ctx.greeter_sas());
    assert!(alice_with_bob_ctx
        .generate_greeter_sas_choices(4)
        .contains(bob_ctx.greeter_sas()));

    let (first, second) = future::join(
        alice_with_bob_ctx.do_signify_trust(),
        bob_ctx.do_wait_peer_trust(),
    )
    .await;
    let alice_with_bob_ctx = first.unwrap();
    let bob_ctx = second.unwrap();

    p_assert_eq!(bob_ctx.claimer_sas(), alice_with_bob_ctx.claimer_sas());
    assert!(bob_ctx
        .generate_claimer_sas_choices(4)
        .contains(alice_with_bob_ctx.claimer_sas()));

    let (first, second) = future::join(
        alice_with_bob_ctx.do_wait_peer_trust(),
        bob_ctx.do_signify_trust(),
    )
    .await;
    let alice_with_bob_ctx = first.unwrap();
    let bob_ctx = second.unwrap();

    let (first, second) = future::join(
        alice_with_bob_ctx.do_recover_share(),
        bob_ctx.do_send_share(),
    )
    .await;
    let alice_share_ctx = first.unwrap();
    second.unwrap();

    let alice_recover_device_ctx =
        match alice_recipient_pick_ctx.add_share(alice_share_ctx).unwrap() {
            ShamirRecoveryClaimMaybeRecoverDeviceCtx::RecoverDevice(ctx) => ctx,
            _ => panic!("Expected PickRecipient context"),
        };

    // Recover with Alice

    let device_label: DeviceLabel = "my new device".parse().unwrap();
    let alice_finalize_ctx = match alice_recover_device_ctx
        .recover_device(device_label.clone())
        .await
        .unwrap()
    {
        ShamirRecoveryClaimMaybeFinalizeCtx::Finalize(ctx) => ctx,
        _ => panic!("Expected Finalize context"),
    };

    // Finalize with Alice

    let default_file_key = alice_finalize_ctx.get_default_key_file();
    let expected_default_file_key = env.discriminant_dir.join("devices").join(format!(
        "{}.keys",
        alice_finalize_ctx.new_local_device.device_id.hex()
    ));
    p_assert_eq!(default_file_key, expected_default_file_key);

    let new_local_device = alice_finalize_ctx.new_local_device.clone();
    let key_file = tmp_path.join("device.keys");
    let password: Password = "P@ssw0rd.".to_string().into();
    let save_strategy = DeviceSaveStrategy::Password { password };
    let available_device = alice_finalize_ctx
        .save_local_device(&save_strategy.clone(), &key_file)
        .await
        .unwrap();

    let access = save_strategy.into_access(key_file.clone());
    let expected_server_addr: ParsecAddr = bob.organization_addr.clone().into();
    p_assert_eq!(available_device.key_file_path, tmp_path.join("device.keys"));
    p_assert_eq!(
        available_device.organization_id,
        new_local_device.organization_id().to_owned()
    );
    p_assert_eq!(available_device.device_id, new_local_device.device_id,);
    p_assert_eq!(available_device.device_label, device_label);
    p_assert_eq!(available_device.human_handle, alice.human_handle);
    p_assert_eq!(available_device.ty, AvailableDeviceType::Password);
    p_assert_eq!(
        available_device.organization_id,
        bob.organization_id().clone()
    );
    p_assert_eq!(available_device.server_addr, expected_server_addr);
    p_assert_eq!(available_device.user_id, alice.user_id);

    // Check that the new device can be loaded

    let reloaded_new_alice_device =
        libparsec_platform_device_loader::load_device(&env.discriminant_dir, &access)
            .await
            .unwrap();
    p_assert_eq!(reloaded_new_alice_device, new_local_device);

    let new_alice_client =
        client_factory(&env.discriminant_dir, reloaded_new_alice_device.clone()).await;

    // Test server connection by deleting the shamir recovery

    new_alice_client.delete_shamir_recovery().await.unwrap();
}

#[parsec_test(testbed = "shamir", with_server)]
async fn shamir_invitation_does_not_exist(env: &TestbedEnv) {
    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        with_monitors: false,
        prevent_sync_pattern: PreventSyncPattern::from_regex(r"\.tmp$").unwrap(),
    });

    let addr = ParsecInvitationAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        libparsec_types::InvitationType::User,
        InvitationToken::default(),
    );

    let error = claimer_retrieve_info(config, addr, None).await.unwrap_err();
    p_assert_matches!(&error, ClaimerRetrieveInfoError::NotFound);
}

#[parsec_test(testbed = "shamir", with_server)]
async fn shamir_invitation_has_been_deleted(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");

    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        with_monitors: false,
        prevent_sync_pattern: PreventSyncPattern::from_regex(r"\.tmp$").unwrap(),
    });

    let alice_token = env
        .template
        .events
        .iter()
        .find_map(|e| match e {
            TestbedEvent::NewShamirRecoveryInvitation(event) if event.claimer == alice.user_id => {
                Some(event.token)
            }
            _ => None,
        })
        .unwrap();

    let addr = ParsecInvitationAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        libparsec_types::InvitationType::User,
        alice_token,
    );

    // Bob delete the invitation
    let bob_client = client_factory(&env.discriminant_dir, bob.clone()).await;
    bob_client.cancel_invitation(alice_token).await.unwrap();

    let error = claimer_retrieve_info(config, addr, None).await.unwrap_err();
    p_assert_matches!(&error, ClaimerRetrieveInfoError::AlreadyUsedOrDeleted);
}

#[parsec_test(testbed = "shamir", with_server)]
async fn unrecoverable_recovery(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let mallory = env.local_device("mallory@dev1");
    let mike = env.local_device("mike@dev1");

    // Revoke Mallory and Bob

    let alice_client = client_factory(&env.discriminant_dir, alice.clone()).await;
    alice_client.revoke_user(mallory.user_id).await.unwrap();
    let mallory_revoked_on = alice_client
        .list_users(false, None, None)
        .await
        .unwrap()
        .iter()
        .find(|&u| u.id == mallory.user_id)
        .unwrap()
        .revoked_on;
    alice_client.revoke_user(bob.user_id).await.unwrap();
    let bob_revoked_on = alice_client
        .list_users(false, None, None)
        .await
        .unwrap()
        .iter()
        .find(|&u| u.id == bob.user_id)
        .unwrap()
        .revoked_on;

    // Start the alice claimer workflow

    let alice_token = env
        .template
        .events
        .iter()
        .find_map(|e| match e {
            TestbedEvent::NewShamirRecoveryInvitation(event) if event.claimer == alice.user_id => {
                Some(event.token)
            }
            _ => None,
        })
        .unwrap();

    let addr = ParsecInvitationAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        libparsec_types::InvitationType::User,
        alice_token,
    );

    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        with_monitors: false,
        prevent_sync_pattern: PreventSyncPattern::from_regex(r"\.tmp$").unwrap(),
    });

    let alice_ctx = claimer_retrieve_info(config, addr, None).await.unwrap();
    p_assert_matches!(&alice_ctx, AnyClaimRetrievedInfoCtx::ShamirRecovery(_));
    let alice_recipient_pick_ctx = match alice_ctx {
        AnyClaimRetrievedInfoCtx::ShamirRecovery(alice_ctx) => {
            p_assert_eq!(alice_ctx.claimer_user_id(), alice.user_id);
            p_assert_eq!(*alice_ctx.claimer_human_handle(), alice.human_handle);
            p_assert_eq!(
                *alice_ctx.recipients(),
                vec![
                    ShamirRecoveryRecipient {
                        user_id: bob.user_id,
                        human_handle: bob.human_handle.clone(),
                        shares: 2.try_into().unwrap(),
                        revoked_on: bob_revoked_on,
                        online_status: libparsec_protocol::invited_cmds::latest::invite_info::UserOnlineStatus::Unknown,
                    },
                    ShamirRecoveryRecipient {
                        user_id: mallory.user_id,
                        human_handle: mallory.human_handle.clone(),
                        shares: 1.try_into().unwrap(),
                        revoked_on: mallory_revoked_on,
                        online_status: libparsec_protocol::invited_cmds::latest::invite_info::UserOnlineStatus::Unknown,
                    },
                    ShamirRecoveryRecipient {
                        user_id: mike.user_id,
                        human_handle: mike.human_handle.clone(),
                        shares: 1.try_into().unwrap(),
                        revoked_on: None,
                        online_status: libparsec_protocol::invited_cmds::latest::invite_info::UserOnlineStatus::Unknown,
                    }
                ]
            );
            p_assert_eq!(alice_ctx.threshold(), 2.try_into().unwrap());
            p_assert_eq!(alice_ctx.retrieved_shares(), HashMap::new());
            assert!(!alice_ctx.is_recoverable());
            alice_ctx
        }
        _ => unreachable!(),
    };

    p_assert_matches!(
        alice_recipient_pick_ctx
            .pick_recipient(bob.user_id)
            .unwrap_err(),
        ShamirRecoveryClaimPickRecipientError::RecipientRevoked
    );

    p_assert_matches!(
        alice_recipient_pick_ctx
            .pick_recipient(mallory.user_id)
            .unwrap_err(),
        ShamirRecoveryClaimPickRecipientError::RecipientRevoked
    );

    alice_recipient_pick_ctx
        .pick_recipient(mike.user_id)
        .unwrap();
}

#[parsec_test(testbed = "shamir", with_server)]
async fn already_picked_recipient(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let mike = env.local_device("mike@dev1");

    // Start the alice claimer workflow

    let alice_token = env
        .template
        .events
        .iter()
        .find_map(|e| match e {
            TestbedEvent::NewShamirRecoveryInvitation(event) if event.claimer == alice.user_id => {
                Some(event.token)
            }
            _ => None,
        })
        .unwrap();

    let addr = ParsecInvitationAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        libparsec_types::InvitationType::User,
        alice_token,
    );

    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        with_monitors: false,
        prevent_sync_pattern: PreventSyncPattern::from_regex(r"\.tmp$").unwrap(),
    });

    let alice_ctx = claimer_retrieve_info(config, addr, None).await.unwrap();
    p_assert_matches!(&alice_ctx, AnyClaimRetrievedInfoCtx::ShamirRecovery(_));
    let alice_recipient_pick_ctx = match alice_ctx {
        AnyClaimRetrievedInfoCtx::ShamirRecovery(alice_ctx) => alice_ctx,
        _ => unreachable!(),
    };

    // Start with Mike

    let alice_with_mike_ctx = alice_recipient_pick_ctx
        .pick_recipient(mike.user_id)
        .unwrap();

    let mike_client = client_factory(&env.discriminant_dir, mike.clone()).await;
    let mike_ctx = mike_client
        .start_shamir_recovery_invitation_greet(alice_token)
        .await
        .unwrap();

    let (first, second) =
        future::join(alice_with_mike_ctx.do_wait_peer(), mike_ctx.do_wait_peer()).await;
    let alice_with_mike_ctx = first.unwrap();
    let mike_ctx = second.unwrap();

    p_assert_eq!(mike_ctx.greeter_sas(), alice_with_mike_ctx.greeter_sas());
    assert!(alice_with_mike_ctx
        .generate_greeter_sas_choices(4)
        .contains(mike_ctx.greeter_sas()));

    let (first, second) = future::join(
        alice_with_mike_ctx.do_signify_trust(),
        mike_ctx.do_wait_peer_trust(),
    )
    .await;
    let alice_with_mike_ctx = first.unwrap();
    let mike_ctx = second.unwrap();

    p_assert_eq!(mike_ctx.claimer_sas(), alice_with_mike_ctx.claimer_sas());
    assert!(mike_ctx
        .generate_claimer_sas_choices(4)
        .contains(alice_with_mike_ctx.claimer_sas()));

    let (first, second) = future::join(
        alice_with_mike_ctx.do_wait_peer_trust(),
        mike_ctx.do_signify_trust(),
    )
    .await;
    let alice_with_mike_ctx = first.unwrap();
    let mike_ctx = second.unwrap();

    let (first, second) = future::join(
        alice_with_mike_ctx.do_recover_share(),
        mike_ctx.do_send_share(),
    )
    .await;
    let alice_share_ctx = first.unwrap();
    second.unwrap();

    let alice_recipient_pick_ctx =
        match alice_recipient_pick_ctx.add_share(alice_share_ctx).unwrap() {
            ShamirRecoveryClaimMaybeRecoverDeviceCtx::PickRecipient(ctx) => ctx,
            _ => panic!("Expected PickRecipient context"),
        };
    p_assert_eq!(alice_recipient_pick_ctx.retrieved_shares().len(), 1);

    // Try to pick Mike again

    p_assert_matches!(
        alice_recipient_pick_ctx
            .pick_recipient(mike.user_id)
            .unwrap_err(),
        ShamirRecoveryClaimPickRecipientError::RecipientAlreadyPicked
    );
}

#[parsec_test(testbed = "shamir", with_server)]
async fn recipient_not_found(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    // Start the alice claimer workflow

    let alice_token = env
        .template
        .events
        .iter()
        .find_map(|e| match e {
            TestbedEvent::NewShamirRecoveryInvitation(event) if event.claimer == alice.user_id => {
                Some(event.token)
            }
            _ => None,
        })
        .unwrap();

    let addr = ParsecInvitationAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        libparsec_types::InvitationType::User,
        alice_token,
    );

    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        with_monitors: false,
        prevent_sync_pattern: PreventSyncPattern::from_regex(r"\.tmp$").unwrap(),
    });

    let alice_ctx = claimer_retrieve_info(config, addr, None).await.unwrap();
    p_assert_matches!(&alice_ctx, AnyClaimRetrievedInfoCtx::ShamirRecovery(_));
    let alice_recipient_pick_ctx = match alice_ctx {
        AnyClaimRetrievedInfoCtx::ShamirRecovery(alice_ctx) => alice_ctx,
        _ => unreachable!(),
    };

    // Pick a non existent recipient

    p_assert_matches!(
        alice_recipient_pick_ctx
            .pick_recipient(UserID::default())
            .unwrap_err(),
        ShamirRecoveryClaimPickRecipientError::RecipientNotFound
    );

    // Create a share with an invalid recipient

    let bad_share = ShamirRecoveryClaimShare {
        recipient: UserID::default(),
        weighted_share: vec![],
    };

    p_assert_matches!(
        alice_recipient_pick_ctx.add_share(bad_share).unwrap_err(),
        ShamirRecoveryClaimAddShareError::RecipientNotFound
    );
}

#[parsec_test(testbed = "shamir", with_server)]
async fn add_share_is_idempotent(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let mike = env.local_device("mike@dev1");

    // Start the alice claimer workflow

    let alice_token = env
        .template
        .events
        .iter()
        .find_map(|e| match e {
            TestbedEvent::NewShamirRecoveryInvitation(event) if event.claimer == alice.user_id => {
                Some(event.token)
            }
            _ => None,
        })
        .unwrap();

    let addr = ParsecInvitationAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        libparsec_types::InvitationType::User,
        alice_token,
    );

    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
        mountpoint_mount_strategy: MountpointMountStrategy::Disabled,
        with_monitors: false,
        prevent_sync_pattern: PreventSyncPattern::from_regex(r"\.tmp$").unwrap(),
    });

    let alice_ctx = claimer_retrieve_info(config, addr, None).await.unwrap();
    p_assert_matches!(&alice_ctx, AnyClaimRetrievedInfoCtx::ShamirRecovery(_));
    let alice_recipient_pick_ctx = match alice_ctx {
        AnyClaimRetrievedInfoCtx::ShamirRecovery(alice_ctx) => alice_ctx,
        _ => unreachable!(),
    };

    // Start with Mike

    let alice_with_mike_ctx = alice_recipient_pick_ctx
        .pick_recipient(mike.user_id)
        .unwrap();

    let mike_client = client_factory(&env.discriminant_dir, mike.clone()).await;
    let mike_ctx = mike_client
        .start_shamir_recovery_invitation_greet(alice_token)
        .await
        .unwrap();

    let (first, second) =
        future::join(alice_with_mike_ctx.do_wait_peer(), mike_ctx.do_wait_peer()).await;
    let alice_with_mike_ctx = first.unwrap();
    let mike_ctx = second.unwrap();

    p_assert_eq!(mike_ctx.greeter_sas(), alice_with_mike_ctx.greeter_sas());
    assert!(alice_with_mike_ctx
        .generate_greeter_sas_choices(4)
        .contains(mike_ctx.greeter_sas()));

    let (first, second) = future::join(
        alice_with_mike_ctx.do_signify_trust(),
        mike_ctx.do_wait_peer_trust(),
    )
    .await;
    let alice_with_mike_ctx = first.unwrap();
    let mike_ctx = second.unwrap();

    p_assert_eq!(mike_ctx.claimer_sas(), alice_with_mike_ctx.claimer_sas());
    assert!(mike_ctx
        .generate_claimer_sas_choices(4)
        .contains(alice_with_mike_ctx.claimer_sas()));

    let (first, second) = future::join(
        alice_with_mike_ctx.do_wait_peer_trust(),
        mike_ctx.do_signify_trust(),
    )
    .await;
    let alice_with_mike_ctx = first.unwrap();
    let mike_ctx = second.unwrap();

    let (first, second) = future::join(
        alice_with_mike_ctx.do_recover_share(),
        mike_ctx.do_send_share(),
    )
    .await;
    let alice_share_ctx = first.unwrap();
    second.unwrap();

    // Copy alice share
    let another_alice_share_ctx = ShamirRecoveryClaimShare {
        recipient: alice_share_ctx.recipient,
        weighted_share: alice_share_ctx.weighted_share.clone(),
    };

    // Add the share
    let alice_recipient_pick_ctx =
        match alice_recipient_pick_ctx.add_share(alice_share_ctx).unwrap() {
            ShamirRecoveryClaimMaybeRecoverDeviceCtx::PickRecipient(ctx) => ctx,
            _ => panic!("Expected PickRecipient context"),
        };
    p_assert_eq!(alice_recipient_pick_ctx.retrieved_shares().len(), 1);

    // Add the same share once again

    let alice_recipient_pick_ctx = match alice_recipient_pick_ctx
        .add_share(another_alice_share_ctx)
        .unwrap()
    {
        ShamirRecoveryClaimMaybeRecoverDeviceCtx::PickRecipient(ctx) => ctx,
        _ => panic!("Expected PickRecipient context"),
    };
    p_assert_eq!(alice_recipient_pick_ctx.retrieved_shares().len(), 1);
}

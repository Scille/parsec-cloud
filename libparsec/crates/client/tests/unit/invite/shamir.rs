// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, sync::Arc};

use super::super::super::client::tests::utils::client_factory;
use libparsec_protocol::invited_cmds::v4::invite_info::ShamirRecoveryRecipient;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    claimer_retrieve_info, ClientConfig, MountpointMountStrategy, ProxyConfig,
    ShamirRecoveryClaimMaybeRecoverDeviceCtx, UserOrDeviceClaimInitialCtx,
    WorkspaceStorageCacheSize,
};

#[parsec_test(testbed = "shamir", with_server)]
async fn shamir(tmp_path: TmpPath, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let mallory = env.local_device("mallory@dev1");
    let mike = env.local_device("mike@dev1");

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
    p_assert_matches!(&alice_ctx, UserOrDeviceClaimInitialCtx::ShamirRecovery(_));
    let alice_recipient_pick_ctx = match alice_ctx {
        UserOrDeviceClaimInitialCtx::ShamirRecovery(alice_ctx) => {
            p_assert_eq!(*alice_ctx.claimer_user_id(), alice.user_id);
            p_assert_eq!(*alice_ctx.claimer_human_handle(), alice.human_handle);
            p_assert_eq!(
                *alice_ctx.recipients(),
                vec![
                    ShamirRecoveryRecipient {
                        user_id: bob.user_id,
                        human_handle: bob.human_handle.clone(),
                        shares: 2.try_into().unwrap(),
                    },
                    ShamirRecoveryRecipient {
                        user_id: mallory.user_id,
                        human_handle: mallory.human_handle.clone(),
                        shares: 1.try_into().unwrap(),
                    },
                    ShamirRecoveryRecipient {
                        user_id: mike.user_id,
                        human_handle: mike.human_handle.clone(),
                        shares: 1.try_into().unwrap(),
                    }
                ]
            );
            p_assert_eq!(alice_ctx.threshold(), 2.try_into().unwrap());
            p_assert_eq!(alice_ctx.shares(), HashMap::new());
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
        .start_shamir_recovery_invitation_greet(alice_token, alice.user_id)
        .await
        .unwrap();

    let (first, second) = tokio::join!(alice_with_mike_ctx.do_wait_peer(), mike_ctx.do_wait_peer());
    let alice_with_mike_ctx = first.unwrap();
    let mike_ctx = second.unwrap();

    p_assert_eq!(mike_ctx.greeter_sas(), alice_with_mike_ctx.greeter_sas());
    assert!(alice_with_mike_ctx
        .generate_greeter_sas_choices(4)
        .contains(mike_ctx.greeter_sas()));

    let (first, second) = tokio::join!(
        alice_with_mike_ctx.do_signify_trust(),
        mike_ctx.do_wait_peer_trust()
    );
    let alice_with_mike_ctx = first.unwrap();
    let mike_ctx = second.unwrap();

    p_assert_eq!(mike_ctx.claimer_sas(), alice_with_mike_ctx.claimer_sas());
    assert!(mike_ctx
        .generate_claimer_sas_choices(4)
        .contains(alice_with_mike_ctx.claimer_sas()));

    let (first, second) = tokio::join!(
        alice_with_mike_ctx.do_wait_peer_trust(),
        mike_ctx.do_signify_trust()
    );
    let alice_with_mike_ctx = first.unwrap();
    let mike_ctx = second.unwrap();

    let (first, second) = tokio::join!(
        alice_with_mike_ctx.do_recover_share(),
        mike_ctx.do_send_share()
    );
    let alice_share_ctx = first.unwrap();
    second.unwrap();

    let alice_recipient_pick_ctx =
        match alice_recipient_pick_ctx.add_share(alice_share_ctx).unwrap() {
            ShamirRecoveryClaimMaybeRecoverDeviceCtx::PickRecipient(ctx) => ctx,
            _ => panic!("Expected PickRecipient context"),
        };
    p_assert_eq!(alice_recipient_pick_ctx.shares().len(), 1);

    // Continue with Bob

    let alice_with_bob_ctx = alice_recipient_pick_ctx
        .pick_recipient(bob.user_id)
        .unwrap();

    let bob_client = client_factory(&env.discriminant_dir, bob.clone()).await;
    let bob_ctx = bob_client
        .start_shamir_recovery_invitation_greet(alice_token, alice.user_id)
        .await
        .unwrap();

    let (first, second) = tokio::join!(alice_with_bob_ctx.do_wait_peer(), bob_ctx.do_wait_peer());
    let alice_with_bob_ctx = first.unwrap();
    let bob_ctx = second.unwrap();

    p_assert_eq!(bob_ctx.greeter_sas(), alice_with_bob_ctx.greeter_sas());
    assert!(alice_with_bob_ctx
        .generate_greeter_sas_choices(4)
        .contains(bob_ctx.greeter_sas()));

    let (first, second) = tokio::join!(
        alice_with_bob_ctx.do_signify_trust(),
        bob_ctx.do_wait_peer_trust()
    );
    let alice_with_bob_ctx = first.unwrap();
    let bob_ctx = second.unwrap();

    p_assert_eq!(bob_ctx.claimer_sas(), alice_with_bob_ctx.claimer_sas());
    assert!(bob_ctx
        .generate_claimer_sas_choices(4)
        .contains(alice_with_bob_ctx.claimer_sas()));

    let (first, second) = tokio::join!(
        alice_with_bob_ctx.do_wait_peer_trust(),
        bob_ctx.do_signify_trust()
    );
    let alice_with_bob_ctx = first.unwrap();
    let bob_ctx = second.unwrap();

    let (first, second) = tokio::join!(
        alice_with_bob_ctx.do_recover_share(),
        bob_ctx.do_send_share()
    );
    let alice_share_ctx = first.unwrap();
    second.unwrap();

    let alice_recover_device_ctx =
        match alice_recipient_pick_ctx.add_share(alice_share_ctx).unwrap() {
            ShamirRecoveryClaimMaybeRecoverDeviceCtx::RecoverDevice(ctx) => ctx,
            _ => panic!("Expected PickRecipient context"),
        };

    // Recover with Alice

    let device_label: DeviceLabel = "my new device".parse().unwrap();
    let alice_finalize_ctx = alice_recover_device_ctx
        .recover_device(device_label.clone())
        .await
        .unwrap();

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
    let access = DeviceAccessStrategy::Password { key_file, password };
    let available_device = alice_finalize_ctx.save_local_device(&access).await.unwrap();

    // Checks
    let expected_server_url = ParsecAddr::from(bob.organization_addr.clone())
        .to_http_url(None)
        .to_string();
    p_assert_eq!(available_device.key_file_path, tmp_path.join("device.keys"));
    p_assert_eq!(
        available_device.organization_id,
        new_local_device.organization_id().to_owned()
    );
    p_assert_eq!(available_device.device_id, new_local_device.device_id,);
    p_assert_eq!(available_device.device_label, device_label);
    p_assert_eq!(available_device.human_handle, alice.human_handle);
    p_assert_eq!(available_device.ty, DeviceFileType::Password);
    p_assert_eq!(
        available_device.organization_id,
        bob.organization_id().clone()
    );
    p_assert_eq!(available_device.server_url, expected_server_url);
    p_assert_eq!(available_device.user_id, alice.user_id);
    // created_on and protected_on date times not checked

    // Check device can be loaded
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

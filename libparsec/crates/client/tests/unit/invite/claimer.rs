// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::{Arc, Mutex};

use libparsec_client_connection::{
    protocol, test_register_send_hook, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    claimer_retrieve_info, ClientConfig, ProxyConfig, UserOrDeviceClaimInitialCtx,
    WorkspaceStorageCacheSize,
};

#[parsec_test(testbed = "minimal")]
async fn claimer(tmp_path: TmpPath, env: &TestbedEnv) {
    let addr = ParsecInvitationAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        libparsec_types::InvitationType::User,
        InvitationToken::default(),
    );

    let config = Arc::new(ClientConfig {
        config_dir: env.discriminant_dir.clone(),
        data_base_dir: env.discriminant_dir.clone(),
        mountpoint_base_dir: env.discriminant_dir.clone(),
        workspace_storage_cache_size: WorkspaceStorageCacheSize::Default,
        proxy: ProxyConfig::default(),
    });

    let alice = env.local_device("alice@dev1");

    // Step 0: retrieve info

    test_register_send_hook(&env.discriminant_dir, {
        let alice = alice.clone();
        move |_req: protocol::invited_cmds::latest::invite_info::Req| {
            protocol::invited_cmds::latest::invite_info::Rep::Ok(
                protocol::invited_cmds::latest::invite_info::UserOrDevice::User {
                    claimer_email: "john@example.com".to_owned(),
                    greeter_human_handle: alice.human_handle.clone(),
                    greeter_user_id: alice.user_id().to_owned(),
                },
            )
        }
    });

    let ctx = claimer_retrieve_info(config, addr).await.unwrap();

    p_assert_matches!(&ctx, UserOrDeviceClaimInitialCtx::User(_));
    let ctx = match ctx {
        UserOrDeviceClaimInitialCtx::User(ctx) => {
            p_assert_eq!(ctx.claimer_email, "john@example.com");
            p_assert_eq!(ctx.greeter_user_id(), alice.user_id());
            p_assert_eq!(*ctx.greeter_human_handle(), alice.human_handle);
            ctx
        }
        _ => unreachable!(),
    };

    // Step 1: wait peer

    let greeter_private_key = Arc::new(PrivateKey::generate());
    let greeter_nonce: Bytes = Bytes::from_static(b"123");
    let claimer_public_key = Arc::new(Mutex::new(None)); // Set in invite_1 hook
    let claimer_hashed_nonce = Arc::new(Mutex::new(None)); // Set in invite_2a hook
    let claimer_nonce = Arc::new(Mutex::new(None)); // Set in invite_2a hook

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) `invite_1_claimer_wait_peer`
        {
            let claimer_public_key = claimer_public_key.clone();
            let greeter_private_key = greeter_private_key.clone();
            move |req: protocol::invited_cmds::latest::invite_1_claimer_wait_peer::Req| {
                let mut guard = claimer_public_key.lock().unwrap();
                p_assert_matches!(*guard, None); // Should be set only once !
                *guard = Some(req.claimer_public_key);

                protocol::invited_cmds::latest::invite_1_claimer_wait_peer::Rep::Ok {
                    greeter_public_key: greeter_private_key.public_key(),
                }
            }
        },
        // 2) `invite_2a_claimer_send_hashed_nonce`
        {
            let claimer_hashed_nonce = claimer_hashed_nonce.clone();
            let greeter_nonce = greeter_nonce.clone();
            move |req: protocol::invited_cmds::latest::invite_2a_claimer_send_hashed_nonce::Req| {
                let mut guard = claimer_hashed_nonce.lock().unwrap();
                p_assert_matches!(*guard, None); // Should be set only once !
                *guard = Some(req.claimer_hashed_nonce);

                protocol::invited_cmds::latest::invite_2a_claimer_send_hashed_nonce::Rep::Ok {
                    greeter_nonce,
                }
            }
        },
        // 3) `invite_2b_claimer_send_nonce`
        {
            let claimer_nonce = claimer_nonce.clone();
            move |req: protocol::invited_cmds::latest::invite_2b_claimer_send_nonce::Req| {
                let mut guard = claimer_nonce.lock().unwrap();
                p_assert_matches!(*guard, None); // Should be set only once !
                *guard = Some(req.claimer_nonce);

                protocol::invited_cmds::latest::invite_2b_claimer_send_nonce::Rep::Ok
            }
        },
    );

    let ctx = ctx.do_wait_peer().await.unwrap();

    let claimer_public_key = {
        let guard = claimer_public_key.lock().unwrap();
        (*guard)
            .clone()
            .expect("set during invite_1_claimer_wait_peer")
    };
    let claimer_hashed_nonce = {
        let guard = claimer_hashed_nonce.lock().unwrap();
        (*guard)
            .clone()
            .expect("set during invite_2a_claimer_send_hashed_nonce")
    };
    let claimer_nonce = {
        let guard = claimer_nonce.lock().unwrap();
        (*guard)
            .clone()
            .expect("set during invite_2b_claimer_send_nonce")
    };

    p_assert_eq!(HashDigest::from_data(&claimer_nonce), claimer_hashed_nonce);

    let shared_secret_key =
        Arc::new(greeter_private_key.generate_shared_secret_key(&claimer_public_key));
    let (expected_claimer_sas, expected_greeter_sas) =
        SASCode::generate_sas_codes(&claimer_nonce, &greeter_nonce, &shared_secret_key);

    // Step 2: signify trust

    let sas_choices = ctx.generate_greeter_sas_choices(3);
    p_assert_eq!(sas_choices.len(), 3);
    p_assert_eq!(ctx.greeter_sas(), &expected_greeter_sas);

    test_register_send_hook(
        &env.discriminant_dir,
        |_req: protocol::invited_cmds::latest::invite_3a_claimer_signify_trust::Req| {
            protocol::invited_cmds::latest::invite_3a_claimer_signify_trust::Rep::Ok
        },
    );

    let ctx = ctx.do_signify_trust().await.unwrap();

    // Step 3: wait peer trust

    p_assert_eq!(ctx.claimer_sas(), &expected_claimer_sas);

    test_register_send_hook(
        &env.discriminant_dir,
        |_req: protocol::invited_cmds::latest::invite_3b_claimer_wait_peer_trust::Req| {
            protocol::invited_cmds::latest::invite_3b_claimer_wait_peer_trust::Rep::Ok
        },
    );

    let ctx = ctx.do_wait_peer_trust().await.unwrap();

    // Step 4: claim user

    let requested_device_label: DeviceLabel = "Requested My dev1".parse().unwrap();
    let requested_human_handle: HumanHandle = "Requested John Doe <requested.john@example.com>"
        .parse()
        .unwrap();
    let device_id: DeviceID = "john@dev1".parse().unwrap();
    let device_label: DeviceLabel = "My dev1".parse().unwrap();
    let human_handle: HumanHandle = "John Doe <john@example.com>".parse().unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) `invite_4_claimer_communicate`
        {
            let shared_secret_key = shared_secret_key.clone();
            let requested_device_label = requested_device_label.clone();
            let requested_human_handle = requested_human_handle.clone();
            move |req: protocol::invited_cmds::latest::invite_4_claimer_communicate::Req| {
                let in_data =
                    InviteUserData::decrypt_and_load(&req.payload, &shared_secret_key).unwrap();

                p_assert_eq!(in_data.requested_device_label, requested_device_label);
                p_assert_eq!(in_data.requested_human_handle, requested_human_handle);

                protocol::invited_cmds::latest::invite_4_claimer_communicate::Rep::Ok {
                    payload: Bytes::from_static(b""),
                    last: false,
                }
            }
        },
        // 2) `invite_4_claimer_communicate` confirmation
        {
            let shared_secret_key = shared_secret_key.clone();
            let root_verify_key = env.organization_addr().root_verify_key().to_owned();
            let device_id = device_id.clone();
            let device_label = device_label.clone();
            let human_handle = human_handle.clone();
            move |req: protocol::invited_cmds::latest::invite_4_claimer_communicate::Req| {
                assert!(req.payload.is_empty());

                let out_payload = InviteUserConfirmation {
                    device_id,
                    device_label,
                    human_handle,
                    profile: UserProfile::Standard,
                    root_verify_key,
                }
                .dump_and_encrypt(&shared_secret_key);

                protocol::invited_cmds::latest::invite_4_claimer_communicate::Rep::Ok {
                    payload: out_payload.into(),
                    last: true,
                }
            }
        }
    );

    let ctx = ctx
        .do_claim_user(requested_device_label, requested_human_handle)
        .await
        .unwrap();

    p_assert_eq!(
        ctx.new_local_device.organization_addr,
        *env.organization_addr()
    );
    p_assert_eq!(ctx.new_local_device.device_id, device_id);
    p_assert_eq!(ctx.new_local_device.device_label, device_label);
    p_assert_eq!(ctx.new_local_device.human_handle, human_handle);
    p_assert_eq!(ctx.new_local_device.initial_profile, UserProfile::Standard);

    // Step 5: finalize

    let default_file_key = ctx.get_default_key_file();
    let expected_default_file_key = env
        .discriminant_dir
        .join("devices")
        .join(format!("{}.keys", ctx.new_local_device.slughash()));
    p_assert_eq!(default_file_key, expected_default_file_key);

    let new_local_device = ctx.new_local_device.clone();
    let key_file = tmp_path.join("device.keys");
    let password: Password = "P@ssw0rd.".to_string().into();
    let access = DeviceAccessStrategy::Password { key_file, password };
    let available_device = ctx.save_local_device(&access).await.unwrap();
    p_assert_eq!(
        available_device,
        AvailableDevice {
            key_file_path: tmp_path.join("device.keys"),
            organization_id: new_local_device.organization_id().to_owned(),
            device_id: new_local_device.device_id.clone(),
            device_label,
            human_handle,
            slug: new_local_device.slug(),
            ty: DeviceFileType::Password,
        }
    );

    // Check device can be loaded
    let reloaded_new_local_device =
        libparsec_platform_device_loader::load_device(&env.discriminant_dir, &access)
            .await
            .unwrap();
    p_assert_eq!(reloaded_new_local_device, new_local_device);
}

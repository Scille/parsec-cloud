// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::{Arc, Mutex};

use libparsec_client_connection::{
    protocol, test_register_send_hook, test_register_sequence_of_send_hooks,
};
use libparsec_platform_device_loader::{AvailableDeviceType, DeviceSaveStrategy};
use libparsec_protocol::invited_cmds::latest::invite_claimer_step::{ClaimerStep, GreeterStep};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    claimer_retrieve_info, AnyClaimRetrievedInfoCtx, ClientConfig, MountpointMountStrategy,
    ProxyConfig, WorkspaceStorageCacheSize,
};

#[parsec_test(testbed = "minimal")]
async fn claimer(tmp_path: TmpPath, env: &TestbedEnv) {
    let addr = ParsecInvitationAddr::new(
        env.server_addr.clone(),
        env.organization_id.clone(),
        libparsec_types::InvitationType::User,
        AccessToken::default(),
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

    let alice = env.local_device("alice@dev1");

    // Part 0: retrieve info

    test_register_send_hook(&env.discriminant_dir, {
        let alice = alice.clone();
        move |_req: protocol::invited_cmds::latest::invite_info::Req| {
            protocol::invited_cmds::latest::invite_info::Rep::Ok(
                protocol::invited_cmds::latest::invite_info::InvitationType::User {
                    claimer_email: "john@example.com".parse().unwrap(),
                    created_by: protocol::invited_cmds::latest::invite_info::InvitationCreatedBy::User {
                        user_id: alice.user_id.to_owned(),
                        human_handle: alice.human_handle.to_owned(),
                    },
                    administrators: vec![
                        protocol::invited_cmds::latest::invite_info::UserGreetingAdministrator {
                            user_id: alice.user_id.to_owned(),
                            human_handle: alice.human_handle.to_owned(),
                            online_status: protocol::invited_cmds::latest::invite_info::UserOnlineStatus::Online,
                            last_greeting_attempt_joined_on: None,
                        },
                    ],
                },
            )
        }
    });

    let ctx = claimer_retrieve_info(config, addr, None).await.unwrap();

    p_assert_matches!(&ctx, AnyClaimRetrievedInfoCtx::User(_));
    let ctx = match ctx {
        AnyClaimRetrievedInfoCtx::User(list_administrators_ctx) => {
            p_assert_eq!(
                list_administrators_ctx.claimer_email().to_string(),
                "john@example.com"
            );
            let ctxs = list_administrators_ctx.list_initial_ctxs();
            assert_eq!(ctxs.len(), 1);
            let ctx = ctxs.into_iter().next().unwrap();
            p_assert_eq!(ctx.greeter_user_id(), alice.user_id);
            p_assert_eq!(*ctx.greeter_human_handle(), alice.human_handle);
            ctx
        }
        _ => unreachable!(),
    };

    let greeter_private_key = Arc::new(PrivateKey::generate());
    let greeter_nonce: Bytes = Bytes::from_static(b"123");
    let greeter_id = Arc::new(Mutex::new(None)); // set in start_attempt
    let claimer_public_key = Arc::new(Mutex::new(None)); // Set in step 0 wait peer
    let claimer_hashed_nonce = Arc::new(Mutex::new(None)); // Set in step 1 send hashed nonce
    let claimer_nonce = Arc::new(Mutex::new(None)); // Set in step 3 get nonce
    let greeting_attempt =
        GreetingAttemptID::from_hex("211575b8-74f9-11ef-8fec-838123f8cb25").unwrap();
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // `invite_claimer_start_greeting_attempt`
        {
            let greeter_id = greeter_id.clone();
            move |req: protocol::invited_cmds::latest::invite_claimer_start_greeting_attempt::Req| {
                let mut guard = greeter_id.lock().unwrap();
                p_assert_matches!(*guard, None); // Should be set only once !
                *guard = Some(req.greeter);

                protocol::invited_cmds::latest::invite_claimer_start_greeting_attempt::Rep::Ok { greeting_attempt }
            }
        },
        // 0) `invite_claimer_step` wait peer
        {
            let claimer_public_key = claimer_public_key.clone();
            let greeter_private_key = greeter_private_key.clone();

            move |req: protocol::invited_cmds::latest::invite_claimer_step::Req| {
                let mut guard = claimer_public_key.lock().unwrap();
                p_assert_matches!(*guard, None); // Should be set only once !

                *guard = Some(match req.claimer_step {
                    ClaimerStep::Number0WaitPeer { public_key } => public_key,
                    e => panic!("Expected step 0 wait peer, found step {e:?}"),
                });

                protocol::invited_cmds::latest::invite_claimer_step::Rep::Ok {
                    greeter_step: GreeterStep::Number0WaitPeer {
                        public_key: greeter_private_key.public_key(),
                    },
                }
            }
        },
        // 1) `invite_claimer_step` send hashed nonce
        {
            let claimer_hashed_nonce = claimer_hashed_nonce.clone();
            move |req: protocol::invited_cmds::latest::invite_claimer_step::Req| {
                let mut guard = claimer_hashed_nonce.lock().unwrap();
                p_assert_matches!(*guard, None); // Should be set only once !
                *guard = Some(match req.claimer_step {
                    ClaimerStep::Number1SendHashedNonce { hashed_nonce } => hashed_nonce,
                    e => panic!("Expected step 1 send hashed nonce, found step {e:?}"),
                });

                protocol::invited_cmds::latest::invite_claimer_step::Rep::Ok {
                    greeter_step: GreeterStep::Number1GetHashedNonce {},
                }
            }
        },
        // 2) `invite_claimer_step` get nonce
        {
            let greeter_nonce = greeter_nonce.clone();
            move |req: protocol::invited_cmds::latest::invite_claimer_step::Req| {
                match req.claimer_step {
                    ClaimerStep::Number2GetNonce => {}
                    e => panic!("Expected step 2 get nonce, found step {e:?}"),
                };

                protocol::invited_cmds::latest::invite_claimer_step::Rep::Ok {
                    greeter_step: GreeterStep::Number2SendNonce { greeter_nonce },
                }
            }
        },
        // 3) `invite_claimer_step` send nonce
        {
            let claimer_nonce = claimer_nonce.clone();
            move |req: protocol::invited_cmds::latest::invite_claimer_step::Req| {
                let mut guard = claimer_nonce.lock().unwrap();
                p_assert_matches!(*guard, None); // Should be set only once !
                *guard = Some(match req.claimer_step {
                    ClaimerStep::Number3SendNonce { claimer_nonce } => claimer_nonce,
                    e => panic!("Expected step 3 send nonce, found step {e:?}"),
                });

                protocol::invited_cmds::latest::invite_claimer_step::Rep::Ok {
                    greeter_step: GreeterStep::Number3GetNonce {},
                }
            }
        },
    );

    let ctx = ctx.do_wait_peer().await.unwrap();

    let claimer_public_key = {
        let guard = claimer_public_key.lock().unwrap();
        (*guard)
            .clone()
            .expect("set during invite_claimer_start_greeting_attempt")
    };
    let claimer_hashed_nonce = {
        let guard = claimer_hashed_nonce.lock().unwrap();
        (*guard).clone().expect("set during step 1")
    };
    let claimer_nonce = {
        let guard = claimer_nonce.lock().unwrap();
        (*guard).clone().expect("set during step 3")
    };

    p_assert_eq!(HashDigest::from_data(&claimer_nonce), claimer_hashed_nonce);

    let shared_secret_key = Arc::new(
        greeter_private_key
            .generate_shared_secret_key(&claimer_public_key, SharedSecretKeyRole::Claimer)
            .unwrap(),
    );
    let (expected_claimer_sas, expected_greeter_sas) =
        SASCode::generate_sas_codes(&claimer_nonce, &greeter_nonce, &shared_secret_key);

    // Part 2: signify trust

    let sas_choices = ctx.generate_greeter_sas_choices(3);
    p_assert_eq!(sas_choices.len(), 3);
    p_assert_eq!(ctx.greeter_sas(), &expected_greeter_sas);

    test_register_send_hook(
        &env.discriminant_dir,
        // 4) `invite_claimer_step` signify trust
        |req: protocol::invited_cmds::latest::invite_claimer_step::Req| {
            match req.claimer_step {
                ClaimerStep::Number4SignifyTrust => {}
                e => panic!("Expected step 4 signify trust, found step {e:?}"),
            };

            protocol::invited_cmds::latest::invite_claimer_step::Rep::Ok {
                greeter_step: GreeterStep::Number4WaitPeerTrust {},
            }
        },
    );

    let ctx = ctx.do_signify_trust().await.unwrap();

    // Part 3: wait peer trust

    p_assert_eq!(ctx.claimer_sas(), &expected_claimer_sas);

    test_register_send_hook(
        &env.discriminant_dir,
        // 5) `invite_claimer_step` wait peer trust
        |req: protocol::invited_cmds::latest::invite_claimer_step::Req| {
            match req.claimer_step {
                ClaimerStep::Number5WaitPeerTrust => {}
                e => panic!("Expected step 5 wait peer trust, found step {e:?}"),
            };

            protocol::invited_cmds::latest::invite_claimer_step::Rep::Ok {
                greeter_step: GreeterStep::Number5SignifyTrust {},
            }
        },
    );

    let ctx = ctx.do_wait_peer_trust().await.unwrap();

    // Part 4: claim user

    let requested_device_label: DeviceLabel = "Requested My dev1".parse().unwrap();
    let requested_human_handle: HumanHandle = "Requested John Doe <requested.john@example.com>"
        .parse()
        .unwrap();
    let device_id = DeviceID::default();
    let device_label: DeviceLabel = "My dev1".parse().unwrap();
    let human_handle: HumanHandle = "John Doe <john@example.com>".parse().unwrap();
    let user_id = UserID::default();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 6) `invite_claimer_step` send payload
        {
            let shared_secret_key = shared_secret_key.clone();
            let requested_device_label = requested_device_label.clone();
            let requested_human_handle = requested_human_handle.clone();
            move |req: protocol::invited_cmds::latest::invite_claimer_step::Req| {
                let payload = match req.claimer_step {
                    ClaimerStep::Number6SendPayload { claimer_payload } => claimer_payload,
                    e => panic!("Expected step 6 send payload, found step {e:?}"),
                };
                let in_data =
                    InviteUserData::decrypt_and_load(&payload, &shared_secret_key).unwrap();

                p_assert_eq!(in_data.requested_device_label, requested_device_label);
                p_assert_eq!(in_data.requested_human_handle, requested_human_handle);

                protocol::invited_cmds::latest::invite_claimer_step::Rep::Ok {
                    greeter_step: GreeterStep::Number6GetPayload {},
                }
            }
        },
        // 7) `invite_claimer_step` get payload
        {
            let shared_secret_key = shared_secret_key.clone();
            let root_verify_key = env.organization_addr().root_verify_key().to_owned();
            let device_label = device_label.clone();
            let human_handle = human_handle.clone();
            move |req: protocol::invited_cmds::latest::invite_claimer_step::Req| {
                match req.claimer_step {
                    ClaimerStep::Number7GetPayload => {}
                    e => panic!("Expected step 7 get payload, found step {e:?}"),
                };

                let greeter_payload = InviteUserConfirmation {
                    device_id,
                    device_label,
                    human_handle,
                    profile: UserProfile::Standard,
                    root_verify_key,
                    user_id,
                }
                .dump_and_encrypt(&shared_secret_key)
                .into();

                protocol::invited_cmds::latest::invite_claimer_step::Rep::Ok {
                    greeter_step: GreeterStep::Number7SendPayload { greeter_payload },
                }
            }
        },
        // 8) `invite_claimer_step` ack
        {
            move |req: protocol::invited_cmds::latest::invite_claimer_step::Req| {
                match req.claimer_step {
                    ClaimerStep::Number8Acknowledge => {}
                    e => panic!("Expected step 8 acknowledge, found step {e:?}"),
                };

                protocol::invited_cmds::latest::invite_claimer_step::Rep::Ok {
                    greeter_step: GreeterStep::Number8WaitPeerAcknowledgment {},
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

    // Part 5: finalize

    let default_file_key = ctx.get_default_key_file();
    let expected_default_file_key = env
        .discriminant_dir
        .join("devices")
        .join(format!("{}.keys", ctx.new_local_device.device_id.hex()));
    p_assert_eq!(default_file_key, expected_default_file_key);

    let new_local_device = ctx.new_local_device.clone();
    let key_file = tmp_path.join("device.keys");
    let password: Password = "P@ssw0rd.".to_string().into();
    let strategy = DeviceSaveStrategy::Password { password };
    let available_device = ctx.save_local_device(&strategy, &key_file).await.unwrap();

    p_assert_eq!(available_device.key_file_path, tmp_path.join("device.keys"));
    p_assert_eq!(
        available_device.organization_id,
        new_local_device.organization_id().to_owned()
    );
    p_assert_eq!(available_device.device_id, new_local_device.device_id,);
    p_assert_eq!(available_device.device_label, device_label);
    p_assert_eq!(available_device.human_handle, human_handle);
    p_assert_eq!(available_device.ty, AvailableDeviceType::Password);
    p_assert_eq!(
        available_device.server_addr,
        "parsec3://noserver.example.com/".parse().unwrap()
    );
    p_assert_eq!(available_device.user_id, user_id);
    // created_on and protected_on date times not checked

    // Check device can be loaded
    let reloaded_new_local_device = libparsec_platform_device_loader::load_device(
        &env.discriminant_dir,
        &strategy.into_access(key_file),
    )
    .await
    .unwrap();
    p_assert_eq!(reloaded_new_local_device, new_local_device);
}

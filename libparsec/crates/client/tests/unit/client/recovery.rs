// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::DeviceInfo;
use crate::{Client, EventBus};
use libparsec_client_connection::test_register_sequence_of_send_hooks;
use libparsec_platform_device_loader::get_default_key_file;
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;

fn assert_device(info: &DeviceInfo, expected_device: impl TryInto<DeviceID>, env: &TestbedEnv) {
    let (certif, _) = env.get_device_certificate(expected_device);

    let DeviceInfo {
        id,
        device_label,
        created_on,
        created_by,
    } = info;
    p_assert_eq!(id, &certif.device_id);
    let expected_device_label = match &certif.device_label {
        MaybeRedacted::Real(device_label) => device_label,
        MaybeRedacted::Redacted(_) => unreachable!(),
    };
    p_assert_eq!(device_label, expected_device_label);
    p_assert_eq!(created_on, &certif.timestamp);
    let expected_created_by = match &certif.author {
        CertificateSignerOwned::User(author) => Some(author),
        CertificateSignerOwned::Root => None,
    };
    p_assert_eq!(created_by.as_ref(), expected_created_by);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let config = client.config.clone();
    // Mock requests to server
    let new_common_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();

    // 1. create recovery device
    // test_register_sequence_of_send_hooks!(
    //     &env.discriminant_dir,
    //     {
    //         let new_common_certificates = new_common_certificates.clone();
    //         move |req: authenticated_cmds::latest::device_create::Req| {
    //             new_common_certificates
    //                 .lock()
    //                 .unwrap()
    //                 .push(req.device_certificate);
    //             authenticated_cmds::latest::device_create::Rep::Ok
    //         }
    //     },
    //     {
    //         let new_common_certificates = new_common_certificates.clone();
    //         move |_req: authenticated_cmds::latest::certificate_get::Req| {
    //             authenticated_cmds::latest::certificate_get::Rep::Ok {
    //                 common_certificates: new_common_certificates.lock().unwrap().clone(),
    //                 realm_certificates: HashMap::new(),
    //                 sequester_certificates: vec![],
    //                 shamir_recovery_certificates: vec![],
    //             }
    //         }
    //     },
    // );

    let recovery_device_label = dbg!(DeviceLabel::try_from("recovery").unwrap());
    let (passphrase, data) = client
        .export_recovery_device(recovery_device_label.clone())
        .await
        .unwrap();
    client.refresh_workspaces_list().await.unwrap();

    let workspaces = client.list_workspaces().await;
    p_assert_eq!(dbg!(&workspaces).len(), 1, "{:?}", workspaces);

    // assert_eq!(new_common_certificates.lock().unwrap().len(), 1);

    let alice_devices = client
        .list_user_devices("alice".parse().unwrap())
        .await
        .unwrap();
    p_assert_eq!(alice_devices.len(), 3);
    assert_device(&alice_devices[0], "alice@dev1", env);
    assert_device(&alice_devices[1], "alice@dev2", env);
    // assert_device(&alice_devices[2], dbg!(alice_devices[2].id), env);

    assert_eq!(dbg!(&alice_devices[2]).device_label, recovery_device_label);

    // // we lose access
    client.stop().await;
    // 2. import recovery device and create new device
    // let new_common_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();

    // test_register_sequence_of_send_hooks!(
    //     &env.discriminant_dir,
    //     // download old certificates
    //     // {
    //     //     let new_common_certificates = new_common_certificates.clone();
    //     //     move |_req: authenticated_cmds::latest::certificate_get::Req| {
    //     //         authenticated_cmds::latest::certificate_get::Rep::Ok {
    //     //             common_certificates: new_common_certificates.lock().unwrap().clone(),
    //     //             realm_certificates: HashMap::new(),
    //     //             sequester_certificates: vec![],
    //     //             shamir_recovery_certificates: vec![],
    //     //         }
    //     //     }
    //     // },
    //     // create new device
    //     {
    //         let new_common_certificates = new_common_certificates.clone();
    //         move |req: authenticated_cmds::latest::device_create::Req| {
    //            // new_common_certificates.lock().unwrap().clear();
    //             new_common_certificates
    //                 .lock()
    //                 .unwrap()
    //                 .push(req.device_certificate);
    //             authenticated_cmds::latest::device_create::Rep::Ok
    //         }
    //     },
    //     {
    //         let new_common_certificates = new_common_certificates.clone();
    //         move |_req: authenticated_cmds::latest::certificate_get::Req| {
    //             authenticated_cmds::latest::certificate_get::Rep::Ok {
    //                 common_certificates: new_common_certificates.lock().unwrap().clone(),
    //                 realm_certificates: HashMap::new(),
    //                 sequester_certificates: vec![],
    //                 shamir_recovery_certificates: vec![],
    //             }
    //         }
    //     },
    // );
    let new_device_label = dbg!(DeviceLabel::try_from("new_device").unwrap());

    let (recovery_device, new_device) =
        libparsec_platform_device_loader::inner_import_recovery_device(
            data,
            passphrase,
            new_device_label.clone(),
        )
        .await
        .unwrap();

    let client = client_factory(&env.discriminant_dir, recovery_device.into()).await;

    //client.poll_server_for_new_certificates().await.unwrap();

    assert_eq!(client.user_id(), "alice".parse().unwrap());

    // let alice_devices = client.list_user_devices(client.user_id()).await.unwrap();
    // p_assert_eq!(alice_devices.len(), 3);
    // assert_device(&alice_devices[0], "alice@dev1", env);
    // assert_device(&alice_devices[1], "alice@dev2", env);
    // assert_eq!(dbg!(&alice_devices[2]).device_label, recovery_device_label);

    let save_strategy = DeviceSaveStrategy::Keyring;
    let access = {
        let key_file = get_default_key_file(&client.config.config_dir, &new_device.device_id);
        save_strategy.into_access(key_file)
    };
    let saved_device = client
        .create_device_from_recovery(new_device.clone(), access)
        .await
        .unwrap();

    client.stop().await;
    let client = client_factory(&env.discriminant_dir, new_device.into()).await;
    // check from avavble device

    // test_register_sequence_of_send_hooks!(
    //     &env.discriminant_dir,

    //     {
    //         let new_common_certificates = new_common_certificates.clone();
    //         move |_req: authenticated_cmds::latest::certificate_get::Req| {
    //             authenticated_cmds::latest::certificate_get::Rep::Ok {
    //                 common_certificates: new_common_certificates.lock().unwrap().clone(),
    //                 realm_certificates: HashMap::new(),
    //                 sequester_certificates: vec![],
    //                 shamir_recovery_certificates: vec![],
    //             }
    //         }
    //     },
    // );
    client.poll_server_for_new_certificates().await.unwrap();

    let alice_devices = client.list_user_devices(client.user_id()).await.unwrap();
    p_assert_eq!(alice_devices.len(), 4);
    assert_device(&alice_devices[0], "alice@dev1", env);
    assert_device(&alice_devices[1], "alice@dev2", env);
    assert_eq!(dbg!(&alice_devices[2]).device_label, recovery_device_label);
    assert_eq!(dbg!(&alice_devices[3]).device_label, new_device_label);

    client.refresh_workspaces_list().await.unwrap();
    client.ensure_workspaces_bootstrapped().await.unwrap();

    let workspaces = client.list_workspaces().await;
    p_assert_eq!(dbg!(&workspaces).len(), 1, "{:?}", workspaces);
    // assert_eq!(new_common_certificates.lock().unwrap().len(), 2);
}

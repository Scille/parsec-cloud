// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::test_register_sequence_of_send_hooks;
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    // Mock requests to server
    let new_shamir_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();
    let new_device_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Create setup and device
        {
            let new_device_certificates = new_device_certificates.clone();
            move |req: authenticated_cmds::latest::device_create::Req| {
                let device_certificate = dbg!(req.device_certificate);
                let mut certs = new_device_certificates.lock().unwrap();
                certs.push(device_certificate);
                dbg!(authenticated_cmds::latest::device_create::Rep::Ok)
            }
        },
        {
            let new_shamir_certificates = new_shamir_certificates.clone();
            move |req: authenticated_cmds::latest::shamir_recovery_setup::Req| {
                let setup = req.setup.unwrap();
                let mut certs = new_shamir_certificates.lock().unwrap();
                certs.push(setup.brief);
                certs.extend(setup.shares);
                authenticated_cmds::latest::shamir_recovery_setup::Rep::Ok
            }
        },
        // 2) Fetch new certificates
        {
            let new_shamir_certificates = new_shamir_certificates.clone();
            let new_device_certificates = new_device_certificates.clone();
            move |_req: authenticated_cmds::latest::certificate_get::Req| {
                authenticated_cmds::latest::certificate_get::Rep::Ok {
                    common_certificates: new_device_certificates.lock().unwrap().clone(),
                    realm_certificates: HashMap::new(),
                    sequester_certificates: vec![],
                    shamir_recovery_certificates: new_shamir_certificates.lock().unwrap().clone(),
                }
            }
        },
    );

    let bob_user_id: UserID = "bob".parse().unwrap();
    let share_recipients = HashMap::from([(bob_user_id, 2)]);
    client
        .shamir_setup_create(share_recipients, 1)
        .await
        .unwrap();
}

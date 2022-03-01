// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use hex_literal::hex;
use rstest::rstest;

use parsec_api_crypto::{HashDigest, PublicKey};
use parsec_api_protocol::*;
use parsec_api_types::HumanHandle;

#[rstest]
#[case::user(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   type: "USER"
        //   claimer_email: "alice@dev1"
        //   cmd: "invite_new"
        //   send_email: true
        &hex!(
            "84ad636c61696d65725f656d61696caa616c6963654064657631a3636d64aa696e76697465"
            "5f6e6577aa73656e645f656d61696cc3a474797065a455534552"
        )[..],
        InviteNewReqSchema::User {
            cmd: "invite_new".to_owned(),
            claimer_email: "alice@dev1".to_owned(),
            send_email: true,
        }
    )
)]
#[case::device(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   type: "DEVICE"
        //   cmd: "invite_new"
        //   send_email: true
        &hex!(
            "83a3636d64aa696e766974655f6e6577aa73656e645f656d61696cc3a474797065a6444556"
            "494345"
        )[..],
        InviteNewReqSchema::Device {
            cmd: "invite_new".to_owned(),
            send_email: true,
        }
    )
)]
fn serde_invite_new_req(#[case] data_expected: (&[u8], InviteNewReqSchema)) {
    let (data, expected) = data_expected;

    let schema = InviteNewReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteNewReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_new_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   email_sent: "SUCCESS"
    //   status: "ok"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let data = hex!(
        "83aa656d61696c5f73656e74a753554343455353a6737461747573a26f6ba5746f6b656ed8"
        "02d864b93ded264aae9ae583fd3d40c45a"
    );

    let expected = InviteNewRepSchema::Ok {
        token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
        email_sent: InvitationEmailSentStatus::Success,
    };

    let schema = InviteNewRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteNewRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_delete_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_delete"
    //   reason: "FINISHED"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let data = hex!(
        "83a3636d64ad696e766974655f64656c657465a6726561736f6ea846494e4953484544a574"
        "6f6b656ed802d864b93ded264aae9ae583fd3d40c45a"
    );

    let expected = InviteDeleteReqSchema {
        cmd: "invite_delete".to_owned(),
        token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
        reason: InvitationDeletedReason::Finished,
    };

    let schema = InviteDeleteReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteDeleteReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_delete_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = InviteDeleteRepSchema::Ok;

    let schema = InviteDeleteRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteDeleteRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_list_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_list"
    let data = hex!("81a3636d64ab696e766974655f6c697374");

    let expected = InviteListReqSchema {
        cmd: "invite_list".to_owned(),
    };

    let schema = InviteListReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteListReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_list_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   invitations: [
    //     {
    //       type: "USER"
    //       claimer_email: "alice@dev1"
    //       created_on: ext(1, 946774800.0)
    //       status: "IDLE"
    //       token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    //     }
    //     {
    //       type: "DEVICE"
    //       created_on: ext(1, 946774800.0)
    //       status: "IDLE"
    //       token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    //     }
    //   ]
    //   status: "ok"
    let data = hex!(
        "82ab696e7669746174696f6e739285ad636c61696d65725f656d61696caa616c6963654064"
        "657631aa637265617465645f6f6ed70141cc375188000000a6737461747573a449444c45a5"
        "746f6b656ed802d864b93ded264aae9ae583fd3d40c45aa474797065a45553455284aa6372"
        "65617465645f6f6ed70141cc375188000000a6737461747573a449444c45a5746f6b656ed8"
        "02d864b93ded264aae9ae583fd3d40c45aa474797065a6444556494345a6737461747573a2"
        "6f6b"
    );

    let expected = InviteListRepSchema::Ok {
        invitations: vec![
            InviteListItemSchema::User {
                token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                claimer_email: "alice@dev1".to_owned(),
                status: InvitationStatus::Idle,
            },
            InviteListItemSchema::Device {
                token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                status: InvitationStatus::Idle,
            },
        ],
    };

    let schema = InviteListRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteListRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_info_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_info"
    let data = hex!("81a3636d64ab696e766974655f696e666f");

    let expected = InviteInfoReqSchema {
        cmd: "invite_info".to_owned(),
    };

    let schema = InviteInfoReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteInfoReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::user(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   type: "USER"
        //   claimer_email: "alice@dev1"
        //   greeter_human_handle: ["bob@dev1", "bob"]
        //   greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a"
        //   status: "ok"
        &hex!(
            "85ad636c61696d65725f656d61696caa616c6963654064657631b4677265657465725f6875"
            "6d616e5f68616e646c6592a8626f624064657631a3626f62af677265657465725f75736572"
            "5f6964d9203130396236386261356364663432386561303031376663366263633034643461"
            "a6737461747573a26f6ba474797065a455534552"
        )[..],
        InviteInfoRepSchema::Ok(InviteInfoUserOrDeviceRep::User {
            claimer_email: "alice@dev1".to_owned(),
            greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
            greeter_human_handle: HumanHandle::new("bob@dev1", "bob").unwrap(),
        })
    )
)]
#[case::device(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   type: "DEVICE"
        //   greeter_human_handle: ["bob@dev1", "bob"]
        //   greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a"
        //   status: "ok"
        &hex!(
            "84b4677265657465725f68756d616e5f68616e646c6592a8626f624064657631a3626f62af"
            "677265657465725f757365725f6964d9203130396236386261356364663432386561303031"
            "376663366263633034643461a6737461747573a26f6ba474797065a6444556494345"
        )[..],
        InviteInfoRepSchema::Ok(InviteInfoUserOrDeviceRep::Device {
                greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
                greeter_human_handle: HumanHandle::new("bob@dev1", "bob").unwrap(),
            }
        )
    )
)]
fn serde_invite_info_rep(#[case] data_expected: (&[u8], InviteInfoRepSchema)) {
    let (data, expected) = data_expected;

    let schema = InviteInfoRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteInfoRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_1_claimer_wait_peer_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   claimer_public_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //   cmd: "invite_1_claimer_wait_peer"
    let data = hex!(
        "82b2636c61696d65725f7075626c69635f6b6579c4206507907d33bae6b5980b32fa03f3eb"
        "ac56141b126e44f352ea46c5f22cd5ac57a3636d64ba696e766974655f315f636c61696d65"
        "725f776169745f70656572"
    );

    let expected = Invite1ClaimerWaitPeerReqSchema {
        cmd: "invite_1_claimer_wait_peer".to_owned(),
        claimer_public_key: PublicKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
    };

    let schema = Invite1ClaimerWaitPeerReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite1ClaimerWaitPeerReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_1_claimer_wait_peer_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   greeter_public_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //   status: "ok"
    let data = hex!(
        "82b2677265657465725f7075626c69635f6b6579c4206507907d33bae6b5980b32fa03f3eb"
        "ac56141b126e44f352ea46c5f22cd5ac57a6737461747573a26f6b"
    );

    let expected = Invite1ClaimerWaitPeerRepSchema::Ok {
        greeter_public_key: PublicKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
    };

    let schema = Invite1ClaimerWaitPeerRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite1ClaimerWaitPeerRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_1_greeter_wait_peer_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_1_greeter_wait_peer"
    //   greeter_public_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let data = hex!(
        "83a3636d64ba696e766974655f315f677265657465725f776169745f70656572b267726565"
        "7465725f7075626c69635f6b6579c4206507907d33bae6b5980b32fa03f3ebac56141b126e"
        "44f352ea46c5f22cd5ac57a5746f6b656ed802d864b93ded264aae9ae583fd3d40c45a"
    );

    let expected = Invite1GreeterWaitPeerReqSchema {
        cmd: "invite_1_greeter_wait_peer".to_owned(),
        token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
        greeter_public_key: PublicKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
    };

    let schema = Invite1GreeterWaitPeerReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite1GreeterWaitPeerReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_1_greeter_wait_peer_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   claimer_public_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //   status: "ok"
    let data = hex!(
        "82b2636c61696d65725f7075626c69635f6b6579c4206507907d33bae6b5980b32fa03f3eb"
        "ac56141b126e44f352ea46c5f22cd5ac57a6737461747573a26f6b"
    );

    let expected = Invite1GreeterWaitPeerRepSchema::Ok {
        claimer_public_key: PublicKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
    };

    let schema = Invite1GreeterWaitPeerRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite1GreeterWaitPeerRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_2a_claimer_send_hashed_nonce_hash_nonce_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   claimer_hashed_nonce: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
    //   cmd: "invite_2a_claimer_send_hashed_nonce_hash_nonce"
    let data = hex!(
        "82b4636c61696d65725f6861736865645f6e6f6e6365c420e37ce3b00a1f15b3de62029972"
        "345420b76313a885c6ccc6e3b5547857b3ecc6a3636d64d92e696e766974655f32615f636c"
        "61696d65725f73656e645f6861736865645f6e6f6e63655f686173685f6e6f6e6365"
    );

    let expected = Invite2aClaimerSendHashedNonceHashNonceReqSchema {
        cmd: "invite_2a_claimer_send_hashed_nonce_hash_nonce".to_owned(),
        claimer_hashed_nonce: HashDigest::from(hex!(
            "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
        )),
    };

    let schema = Invite2aClaimerSendHashedNonceHashNonceReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite2aClaimerSendHashedNonceHashNonceReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_2a_claimer_send_hashed_nonce_hash_nonce_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   greeter_nonce: hex!("666f6f626172")
    //   status: "ok"
    let data = hex!("82ad677265657465725f6e6f6e6365c406666f6f626172a6737461747573a26f6b");

    let expected = Invite2aClaimerSendHashedNonceHashNonceRepSchema::Ok {
        greeter_nonce: b"foobar".to_vec(),
    };

    let schema = Invite2aClaimerSendHashedNonceHashNonceRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite2aClaimerSendHashedNonceHashNonceRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_2a_greeter_get_hashed_nonce_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   claimer_hashed_nonce: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
    //   status: "ok"
    let data = hex!(
        "82b4636c61696d65725f6861736865645f6e6f6e6365c420e37ce3b00a1f15b3de62029972"
        "345420b76313a885c6ccc6e3b5547857b3ecc6a6737461747573a26f6b"
    );

    let expected = Invite2aGreeterGetHashedNonceRepSchema::Ok {
        claimer_hashed_nonce: HashDigest::from(hex!(
            "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
        )),
    };

    let schema = Invite2aGreeterGetHashedNonceRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite2aGreeterGetHashedNonceRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_2b_greeter_send_nonce_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   claimer_nonce: hex!("666f6f626172")
    //   status: "ok"
    let data = hex!("82ad636c61696d65725f6e6f6e6365c406666f6f626172a6737461747573a26f6b");

    let expected = Invite2bGreeterSendNonceRepSchema::Ok {
        claimer_nonce: b"foobar".to_vec(),
    };

    let schema = Invite2bGreeterSendNonceRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite2bGreeterSendNonceRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_2b_claimer_send_nonce_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   claimer_nonce: hex!("666f6f626172")
    //   cmd: "invite_2b_claimer_send_nonce"
    let data = hex!(
        "82ad636c61696d65725f6e6f6e6365c406666f6f626172a3636d64bc696e766974655f3262"
        "5f636c61696d65725f73656e645f6e6f6e6365"
    );

    let expected = Invite2bClaimerSendNonceReqSchema {
        cmd: "invite_2b_claimer_send_nonce".to_owned(),
        claimer_nonce: b"foobar".to_vec(),
    };

    let schema = Invite2bClaimerSendNonceReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite2bClaimerSendNonceReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_2b_claimer_send_nonce_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = Invite2bClaimerSendNonceRepSchema::Ok;

    let schema = Invite2bClaimerSendNonceRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite2bClaimerSendNonceRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_3a_greeter_wait_peer_trust_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_3a_greeter_wait_peer_trust"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let data = hex!(
        "82a3636d64d921696e766974655f33615f677265657465725f776169745f706565725f7472"
        "757374a5746f6b656ed802d864b93ded264aae9ae583fd3d40c45a"
    );

    let expected = Invite3aGreeterWaitPeerTrustReqSchema {
        cmd: "invite_3a_greeter_wait_peer_trust".to_owned(),
        token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
    };

    let schema = Invite3aGreeterWaitPeerTrustReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3aGreeterWaitPeerTrustReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_3a_greeter_wait_peer_trust_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = Invite3aGreeterWaitPeerTrustRepSchema::Ok;

    let schema = Invite3aGreeterWaitPeerTrustRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3aGreeterWaitPeerTrustRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_3b_claimer_wait_peer_trust_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_3b_claimer_wait_peer_trust"
    let data = hex!(
        "81a3636d64d921696e766974655f33625f636c61696d65725f776169745f706565725f7472"
        "757374"
    );

    let expected = Invite3bClaimerWaitPeerTrustReqSchema {
        cmd: "invite_3b_claimer_wait_peer_trust".to_owned(),
    };

    let schema = Invite3bClaimerWaitPeerTrustReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3bClaimerWaitPeerTrustReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_3b_claimer_wait_peer_trust_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = Invite3bClaimerWaitPeerTrustRepSchema::Ok;

    let schema = Invite3bClaimerWaitPeerTrustRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3bClaimerWaitPeerTrustRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_3b_greeter_signify_trust_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_3b_greeter_signify_trust"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let data = hex!(
        "82a3636d64bf696e766974655f33625f677265657465725f7369676e6966795f7472757374"
        "a5746f6b656ed802d864b93ded264aae9ae583fd3d40c45a"
    );

    let expected = Invite3bGreeterSignifyTrustReqSchema {
        cmd: "invite_3b_greeter_signify_trust".to_owned(),
        token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
    };

    let schema = Invite3bGreeterSignifyTrustReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3bGreeterSignifyTrustReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_3b_greeter_signify_trust_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = Invite3bGreeterSignifyTrustRepSchema::Ok;

    let schema = Invite3bGreeterSignifyTrustRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3bGreeterSignifyTrustRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_3a_claimer_signify_trust_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_3a_claimer_signify_trust"
    let data = hex!("81a3636d64bf696e766974655f33615f636c61696d65725f7369676e6966795f7472757374");

    let expected = Invite3aClaimerSignifyTrustReqSchema {
        cmd: "invite_3a_claimer_signify_trust".to_owned(),
    };

    let schema = Invite3aClaimerSignifyTrustReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3aClaimerSignifyTrustReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_3a_claimer_signify_trust_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = Invite3aClaimerSignifyTrustRepSchema::Ok;

    let schema = Invite3aClaimerSignifyTrustRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3aClaimerSignifyTrustRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_4_greeter_communicate_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_4_greeter_communicate"
    //   payload: hex!("666f6f626172")
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let data = hex!(
        "83a3636d64bc696e766974655f345f677265657465725f636f6d6d756e6963617465a77061"
        "796c6f6164c406666f6f626172a5746f6b656ed802d864b93ded264aae9ae583fd3d40c45a"
    );

    let expected = Invite4GreeterCommunicateReqSchema {
        cmd: "invite_4_greeter_communicate".to_owned(),
        token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
        payload: b"foobar".to_vec(),
    };

    let schema = Invite4GreeterCommunicateReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite4GreeterCommunicateReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_4_greeter_communicate_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   payload: hex!("666f6f626172")
    //   status: "ok"
    let data = hex!("82a77061796c6f6164c406666f6f626172a6737461747573a26f6b");

    let expected = Invite4GreeterCommunicateRepSchema::Ok {
        payload: b"foobar".to_vec(),
    };

    let schema = Invite4GreeterCommunicateRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite4GreeterCommunicateRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_4_claimer_communicate_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_4_claimer_communicate"
    //   payload: hex!("666f6f626172")
    let data = hex!(
        "82a3636d64bc696e766974655f345f636c61696d65725f636f6d6d756e6963617465a77061"
        "796c6f6164c406666f6f626172"
    );

    let expected = Invite4ClaimerCommunicateReqSchema {
        cmd: "invite_4_claimer_communicate".to_owned(),
        payload: b"foobar".to_vec(),
    };

    let schema = Invite4ClaimerCommunicateReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite4ClaimerCommunicateReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_4_claimer_communicate_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   payload: hex!("666f6f626172")
    //   status: "ok"
    let data = hex!("82a77061796c6f6164c406666f6f626172a6737461747573a26f6b");

    let expected = Invite4ClaimerCommunicateRepSchema::Ok {
        payload: b"foobar".to_vec(),
    };

    let schema = Invite4ClaimerCommunicateRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite4ClaimerCommunicateRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

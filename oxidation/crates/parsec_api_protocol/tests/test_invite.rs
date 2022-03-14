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
        InviteNewReq::User {
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
        InviteNewReq::Device {
            cmd: "invite_new".to_owned(),
            send_email: true,
        }
    )
)]
fn serde_invite_new_req(#[case] data_expected: (&[u8], InviteNewReq)) {
    let (data, expected) = data_expected;

    let schema = InviteNewReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteNewReq::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   email_sent: "SUCCESS"
        //   status: "ok"
        //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
        &hex!(
            "83aa656d61696c5f73656e74a753554343455353a6737461747573a26f6ba5746f6b656ed8"
            "02d864b93ded264aae9ae583fd3d40c45a"
        )[..],
        InviteNewRep::Ok {
            token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
            email_sent: InvitationEmailSentStatus::Success,
        }
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_allowed"
        &hex!(
            "81a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        InviteNewRep::NotAllowed
    )
)]
#[case::already_member(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "already_member"
        &hex!(
            "81a6737461747573ae616c72656164795f6d656d626572"
        )[..],
        InviteNewRep::AlreadyMember
    )
)]
#[case::not_available(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_available"
        &hex!(
            "81a6737461747573ad6e6f745f617661696c61626c65"
        )[..],
        InviteNewRep::NotAvailable
    )
)]
fn serde_invite_new_rep(#[case] data_expected: (&[u8], InviteNewRep)) {
    let (data, expected) = data_expected;

    let schema = InviteNewRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteNewRep::load(&data2).unwrap();

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

    let expected = InviteDeleteReq {
        cmd: "invite_delete".to_owned(),
        token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
        reason: InvitationDeletedReason::Finished,
    };

    let schema = InviteDeleteReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteDeleteReq::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        InviteDeleteRep::Ok
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        InviteDeleteRep::NotFound
    )
)]
#[case::already_deleted(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "already_deleted"
        &hex!(
            "81a6737461747573af616c72656164795f64656c65746564"
        )[..],
        InviteDeleteRep::AlreadyDeleted
    )
)]
fn serde_invite_delete_rep(#[case] data_expected: (&[u8], InviteDeleteRep)) {
    let (data, expected) = data_expected;

    let schema = InviteDeleteRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteDeleteRep::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_list_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_list"
    let data = hex!("81a3636d64ab696e766974655f6c697374");

    let expected = InviteListReq {
        cmd: "invite_list".to_owned(),
    };

    let schema = InviteListReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteListReq::load(&data2).unwrap();

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

    let expected = InviteListRep::Ok {
        invitations: vec![
            InviteListItem::User {
                token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                claimer_email: "alice@dev1".to_owned(),
                status: InvitationStatus::Idle,
            },
            InviteListItem::Device {
                token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                status: InvitationStatus::Idle,
            },
        ],
    };

    let schema = InviteListRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteListRep::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_info_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_info"
    let data = hex!("81a3636d64ab696e766974655f696e666f");

    let expected = InviteInfoReq {
        cmd: "invite_info".to_owned(),
    };

    let schema = InviteInfoReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteInfoReq::load(&data2).unwrap();

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
        InviteInfoRep::Ok(InviteInfoUserOrDeviceRep::User {
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
        InviteInfoRep::Ok(InviteInfoUserOrDeviceRep::Device {
                greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
                greeter_human_handle: HumanHandle::new("bob@dev1", "bob").unwrap(),
            }
        )
    )
)]
fn serde_invite_info_rep(#[case] data_expected: (&[u8], InviteInfoRep)) {
    let (data, expected) = data_expected;

    let schema = InviteInfoRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = InviteInfoRep::load(&data2).unwrap();

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

    let expected = Invite1ClaimerWaitPeerReq {
        cmd: "invite_1_claimer_wait_peer".to_owned(),
        claimer_public_key: PublicKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
    };

    let schema = Invite1ClaimerWaitPeerReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite1ClaimerWaitPeerReq::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   greeter_public_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        //   status: "ok"
        &hex!(
            "82b2677265657465725f7075626c69635f6b6579c4206507907d33bae6b5980b32fa03f3eb"
            "ac56141b126e44f352ea46c5f22cd5ac57a6737461747573a26f6b"
        )[..],
        Invite1ClaimerWaitPeerRep::Ok {
            greeter_public_key: PublicKey::from(hex!(
                "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
            )),
        }
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        Invite1ClaimerWaitPeerRep::NotFound
    )
)]
#[case::invalid_state(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "invalid_state"
        &hex!(
            "81a6737461747573ad696e76616c69645f7374617465"
        )[..],
        Invite1ClaimerWaitPeerRep::InvalidState
    )
)]
fn serde_invite_1_claimer_wait_peer_rep(#[case] data_expected: (&[u8], Invite1ClaimerWaitPeerRep)) {
    let (data, expected) = data_expected;

    let schema = Invite1ClaimerWaitPeerRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite1ClaimerWaitPeerRep::load(&data2).unwrap();

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

    let expected = Invite1GreeterWaitPeerReq {
        cmd: "invite_1_greeter_wait_peer".to_owned(),
        token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
        greeter_public_key: PublicKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
    };

    let schema = Invite1GreeterWaitPeerReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite1GreeterWaitPeerReq::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   claimer_public_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        //   status: "ok"
        &hex!(
            "82b2636c61696d65725f7075626c69635f6b6579c4206507907d33bae6b5980b32fa03f3eb"
            "ac56141b126e44f352ea46c5f22cd5ac57a6737461747573a26f6b"
        )[..],
        Invite1GreeterWaitPeerRep::Ok {
            claimer_public_key: PublicKey::from(hex!(
                "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
            )),
        }
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        Invite1GreeterWaitPeerRep::NotFound
    )
)]
#[case::already_deleted(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "already_deleted"
        &hex!(
            "81a6737461747573af616c72656164795f64656c65746564"
        )[..],
        Invite1GreeterWaitPeerRep::AlreadyDeleted
    )
)]
#[case::invalid_state(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "invalid_state"
        &hex!(
            "81a6737461747573ad696e76616c69645f7374617465"
        )[..],
        Invite1GreeterWaitPeerRep::InvalidState
    )
)]
fn serde_invite_1_greeter_wait_peer_rep(#[case] data_expected: (&[u8], Invite1GreeterWaitPeerRep)) {
    let (data, expected) = data_expected;

    let schema = Invite1GreeterWaitPeerRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite1GreeterWaitPeerRep::load(&data2).unwrap();

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

    let expected = Invite2aClaimerSendHashedNonceHashNonceReq {
        cmd: "invite_2a_claimer_send_hashed_nonce_hash_nonce".to_owned(),
        claimer_hashed_nonce: HashDigest::from(hex!(
            "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
        )),
    };

    let schema = Invite2aClaimerSendHashedNonceHashNonceReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite2aClaimerSendHashedNonceHashNonceReq::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   greeter_nonce: hex!("666f6f626172")
        //   status: "ok"
        &hex!(
            "82ad677265657465725f6e6f6e6365c406666f6f626172a6737461747573a26f6b"
        )[..],
        Invite2aClaimerSendHashedNonceHashNonceRep::Ok {
            greeter_nonce: b"foobar".to_vec(),
        }
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        Invite2aClaimerSendHashedNonceHashNonceRep::NotFound
    )
)]
// Generated from Python implementation (Parsec v2.6.0+dev)
// Content:
//   status: "already_deleted"
#[case::already_deleted(
    (
        &hex!(
            "81a6737461747573af616c72656164795f64656c65746564"
        )[..],
        Invite2aClaimerSendHashedNonceHashNonceRep::AlreadyDeleted
    )
)]
// Generated from Python implementation (Parsec v2.6.0+dev)
// Content:
//   status: "invalid_state"
#[case::invalid_state(
    (
        &hex!(
            "81a6737461747573ad696e76616c69645f7374617465"
        )[..],
        Invite2aClaimerSendHashedNonceHashNonceRep::InvalidState
    )
)]
fn serde_invite_2a_claimer_send_hashed_nonce_hash_nonce_rep(
    #[case] data_expected: (&[u8], Invite2aClaimerSendHashedNonceHashNonceRep),
) {
    let (data, expected) = data_expected;

    let schema = Invite2aClaimerSendHashedNonceHashNonceRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite2aClaimerSendHashedNonceHashNonceRep::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   claimer_hashed_nonce: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
        //   status: "ok"
        &hex!(
            "82b4636c61696d65725f6861736865645f6e6f6e6365c420e37ce3b00a1f15b3de62029972"
            "345420b76313a885c6ccc6e3b5547857b3ecc6a6737461747573a26f6b"
        )[..],
        Invite2aGreeterGetHashedNonceRep::Ok {
            claimer_hashed_nonce: HashDigest::from(hex!(
                "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
            )),
        }
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        Invite2aGreeterGetHashedNonceRep::NotFound
    )
)]
#[case::already_deleted(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "already_deleted"
        &hex!(
            "81a6737461747573af616c72656164795f64656c65746564"
        )[..],
        Invite2aGreeterGetHashedNonceRep::AlreadyDeleted
    )
)]
#[case::invalid_state(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "invalid_state"
        &hex!(
            "81a6737461747573ad696e76616c69645f7374617465"
        )[..],
        Invite2aGreeterGetHashedNonceRep::InvalidState
    )
)]
fn serde_invite_2a_greeter_get_hashed_nonce_rep(
    #[case] data_expected: (&[u8], Invite2aGreeterGetHashedNonceRep),
) {
    let (data, expected) = data_expected;

    let schema = Invite2aGreeterGetHashedNonceRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite2aGreeterGetHashedNonceRep::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   claimer_nonce: hex!("666f6f626172")
        //   status: "ok"
        &hex!(
            "82ad636c61696d65725f6e6f6e6365c406666f6f626172a6737461747573a26f6b"
        )[..],
        Invite2bGreeterSendNonceRep::Ok {
            claimer_nonce: b"foobar".to_vec(),
        }
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        Invite2bGreeterSendNonceRep::NotFound
    )
)]
#[case::already_deleted(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "already_deleted"
        &hex!(
            "81a6737461747573af616c72656164795f64656c65746564"
        )[..],
        Invite2bGreeterSendNonceRep::AlreadyDeleted
    )
)]
#[case::invalid_state(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "invalid_state"
        &hex!(
            "81a6737461747573ad696e76616c69645f7374617465"
        )[..],
        Invite2bGreeterSendNonceRep::InvalidState
    )
)]
fn serde_invite_2b_greeter_send_nonce_rep(
    #[case] data_expected: (&[u8], Invite2bGreeterSendNonceRep),
) {
    let (data, expected) = data_expected;

    let schema = Invite2bGreeterSendNonceRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite2bGreeterSendNonceRep::load(&data2).unwrap();

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

    let expected = Invite2bClaimerSendNonceReq {
        cmd: "invite_2b_claimer_send_nonce".to_owned(),
        claimer_nonce: b"foobar".to_vec(),
    };

    let schema = Invite2bClaimerSendNonceReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite2bClaimerSendNonceReq::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        Invite2bClaimerSendNonceRep::Ok
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        Invite2bClaimerSendNonceRep::NotFound
    )
)]
#[case::invalid_state(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "invalid_state"
        &hex!(
            "81a6737461747573ad696e76616c69645f7374617465"
        )[..],
        Invite2bClaimerSendNonceRep::InvalidState
    )
)]
fn serde_invite_2b_claimer_send_nonce_rep(
    #[case] data_expected: (&[u8], Invite2bClaimerSendNonceRep),
) {
    let (data, expected) = data_expected;

    let schema = Invite2bClaimerSendNonceRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite2bClaimerSendNonceRep::load(&data2).unwrap();

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

    let expected = Invite3aGreeterWaitPeerTrustReq {
        cmd: "invite_3a_greeter_wait_peer_trust".to_owned(),
        token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
    };

    let schema = Invite3aGreeterWaitPeerTrustReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3aGreeterWaitPeerTrustReq::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        Invite3aGreeterWaitPeerTrustRep::Ok
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        Invite3aGreeterWaitPeerTrustRep::NotFound
    )
)]
#[case::already_deleted(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "already_deleted"
        &hex!(
            "81a6737461747573af616c72656164795f64656c65746564"
        )[..],
        Invite3aGreeterWaitPeerTrustRep::AlreadyDeleted
    )
)]
#[case::invalid_state(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "invalid_state"
        &hex!(
            "81a6737461747573ad696e76616c69645f7374617465"
        )[..],
        Invite3aGreeterWaitPeerTrustRep::InvalidState
    )
)]
fn serde_invite_3a_greeter_wait_peer_trust_rep(
    #[case] data_expected: (&[u8], Invite3aGreeterWaitPeerTrustRep),
) {
    let (data, expected) = data_expected;

    let schema = Invite3aGreeterWaitPeerTrustRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3aGreeterWaitPeerTrustRep::load(&data2).unwrap();

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

    let expected = Invite3bClaimerWaitPeerTrustReq {
        cmd: "invite_3b_claimer_wait_peer_trust".to_owned(),
    };

    let schema = Invite3bClaimerWaitPeerTrustReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3bClaimerWaitPeerTrustReq::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        Invite3bClaimerWaitPeerTrustRep::Ok
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        Invite3bClaimerWaitPeerTrustRep::NotFound
    )
)]
#[case::invalid_state(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "invalid_state"
        &hex!(
            "81a6737461747573ad696e76616c69645f7374617465"
        )[..],
        Invite3bClaimerWaitPeerTrustRep::InvalidState
    )
)]
fn serde_invite_3b_claimer_wait_peer_trust_rep(
    #[case] data_expected: (&[u8], Invite3bClaimerWaitPeerTrustRep),
) {
    let (data, expected) = data_expected;

    let schema = Invite3bClaimerWaitPeerTrustRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3bClaimerWaitPeerTrustRep::load(&data2).unwrap();

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

    let expected = Invite3bGreeterSignifyTrustReq {
        cmd: "invite_3b_greeter_signify_trust".to_owned(),
        token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
    };

    let schema = Invite3bGreeterSignifyTrustReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3bGreeterSignifyTrustReq::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        Invite3bGreeterSignifyTrustRep::Ok
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        Invite3bGreeterSignifyTrustRep::NotFound
    )
)]
#[case::already_deleted(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "already_deleted"
        &hex!(
            "81a6737461747573af616c72656164795f64656c65746564"
        )[..],
        Invite3bGreeterSignifyTrustRep::AlreadyDeleted
    )
)]
#[case::invalid_state(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "invalid_state"
        &hex!(
            "81a6737461747573ad696e76616c69645f7374617465"
        )[..],
        Invite3bGreeterSignifyTrustRep::InvalidState
    )
)]
fn serde_invite_3b_greeter_signify_trust_rep(
    #[case] data_expected: (&[u8], Invite3bGreeterSignifyTrustRep),
) {
    let (data, expected) = data_expected;

    let schema = Invite3bGreeterSignifyTrustRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3bGreeterSignifyTrustRep::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_invite_3a_claimer_signify_trust_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_3a_claimer_signify_trust"
    let data = hex!("81a3636d64bf696e766974655f33615f636c61696d65725f7369676e6966795f7472757374");

    let expected = Invite3aClaimerSignifyTrustReq {
        cmd: "invite_3a_claimer_signify_trust".to_owned(),
    };

    let schema = Invite3aClaimerSignifyTrustReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3aClaimerSignifyTrustReq::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        Invite3aClaimerSignifyTrustRep::Ok
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        Invite3aClaimerSignifyTrustRep::NotFound
    )
)]
#[case::invalid_state(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "invalid_state"
        &hex!(
            "81a6737461747573ad696e76616c69645f7374617465"
        )[..],
        Invite3aClaimerSignifyTrustRep::InvalidState
    )
)]
fn serde_invite_3a_claimer_signify_trust_rep(
    #[case] data_expected: (&[u8], Invite3aClaimerSignifyTrustRep),
) {
    let (data, expected) = data_expected;

    let schema = Invite3aClaimerSignifyTrustRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite3aClaimerSignifyTrustRep::load(&data2).unwrap();

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

    let expected = Invite4GreeterCommunicateReq {
        cmd: "invite_4_greeter_communicate".to_owned(),
        token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
        payload: b"foobar".to_vec(),
    };

    let schema = Invite4GreeterCommunicateReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite4GreeterCommunicateReq::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   payload: hex!("666f6f626172")
        //   status: "ok"
        &hex!(
            "82a77061796c6f6164c406666f6f626172a6737461747573a26f6b"
        )[..],
        Invite4GreeterCommunicateRep::Ok {
            payload: b"foobar".to_vec(),
        }
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        Invite4GreeterCommunicateRep::NotFound
    )
)]
#[case::already_deleted(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "already_deleted"
        &hex!(
            "81a6737461747573af616c72656164795f64656c65746564"
        )[..],
        Invite4GreeterCommunicateRep::AlreadyDeleted
    )
)]
#[case::invalid_state(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "invalid_state"
        &hex!(
            "81a6737461747573ad696e76616c69645f7374617465"
        )[..],
        Invite4GreeterCommunicateRep::InvalidState
    )
)]
fn serde_invite_4_greeter_communicate_rep(
    #[case] data_expected: (&[u8], Invite4GreeterCommunicateRep),
) {
    let (data, expected) = data_expected;

    let schema = Invite4GreeterCommunicateRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite4GreeterCommunicateRep::load(&data2).unwrap();

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

    let expected = Invite4ClaimerCommunicateReq {
        cmd: "invite_4_claimer_communicate".to_owned(),
        payload: b"foobar".to_vec(),
    };

    let schema = Invite4ClaimerCommunicateReq::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite4ClaimerCommunicateReq::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   payload: hex!("666f6f626172")
        //   status: "ok"
        &hex!(
            "82a77061796c6f6164c406666f6f626172a6737461747573a26f6b"
        )[..],
        Invite4ClaimerCommunicateRep::Ok {
            payload: b"foobar".to_vec(),
        }
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        Invite4ClaimerCommunicateRep::NotFound
    )
)]
#[case::invalid_state(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "invalid_state"
        &hex!(
            "81a6737461747573ad696e76616c69645f7374617465"
        )[..],
        Invite4ClaimerCommunicateRep::InvalidState
    )
)]
fn serde_invite_4_claimer_communicate_rep(
    #[case] data_expected: (&[u8], Invite4ClaimerCommunicateRep),
) {
    let (data, expected) = data_expected;

    let schema = Invite4ClaimerCommunicateRep::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = Invite4ClaimerCommunicateRep::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_protocol::{
    authenticated_cmds::v3 as authenticated_cmds, invited_cmds::v3 as invited_cmds,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test]
fn serde_invite_info_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_info"
    let raw = hex!("81a3636d64ab696e766974655f696e666f");

    let req = invited_cmds::invite_info::Req;

    let expected = invited_cmds::AnyCmdReq::InviteInfo(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let invited_cmds::AnyCmdReq::InviteInfo(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::user(
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
    ),
    invited_cmds::invite_info::Rep::Ok(
        invited_cmds::invite_info::UserOrDevice::User {
            claimer_email: "alice@dev1".to_owned(),
            greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
            greeter_human_handle: Some(HumanHandle::new("bob@dev1", "bob").unwrap()),
        }
    )
)]
#[case::device(
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
    ),
    invited_cmds::invite_info::Rep::Ok(
        invited_cmds::invite_info::UserOrDevice::Device {
            greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
            greeter_human_handle: Some(HumanHandle::new("bob@dev1", "bob").unwrap()),
        }
    )
)]
#[case::user_no_human_handle(
    // Generated from Rust implementation (Parsec v2.13.0-rc1+dev)
    // Content:
    //   type: "USER"
    //   claimer_email: "alice@dev1"
    //   greeter_human_handle: None
    //   greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a"
    //   status: "ok"
    //
    &hex!(
        "85a6737461747573a26f6ba474797065a455534552ad636c61696d65725f656d61696caa61"
        "6c6963654064657631af677265657465725f757365725f6964d92031303962363862613563"
        "64663432386561303031376663366263633034643461b4677265657465725f68756d616e5f"
        "68616e646c65c0"
    ),
    invited_cmds::invite_info::Rep::Ok(
        invited_cmds::invite_info::UserOrDevice::User {
            greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
            claimer_email: "alice@dev1".to_string(),
            greeter_human_handle: None,
        }
    )
)]
#[case::device_no_human_handle(
    // Generated from Rust implementation (Parsec v2.13.0-rc1+dev)
    // Content:
    //   type: "DEVICE"
    //   greeter_human_handle: None
    //   greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a"
    //   status: "ok"
    //
    &hex!(
        "84a6737461747573a26f6ba474797065a6444556494345af677265657465725f757365725f"
        "6964d9203130396236386261356364663432386561303031376663366263633034643461b4"
        "677265657465725f68756d616e5f68616e646c65c0"
    ),
    invited_cmds::invite_info::Rep::Ok(
        invited_cmds::invite_info::UserOrDevice::Device {
            greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
            greeter_human_handle: None,
        }
    )
)]
fn serde_invite_info_rep(#[case] raw: &[u8], #[case] expected: invited_cmds::invite_info::Rep) {
    let data = invited_cmds::invite_info::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_info::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_2a_claimer_send_hashed_nonce_req() {
    // Generated from Python implementation (Parsec v2.10.0+dev)
    // Content:
    //   claimer_hashed_nonce: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
    //   cmd: "invite_2a_claimer_send_hashed_nonce"
    let raw = hex!(
        "82b4636c61696d65725f6861736865645f6e6f6e6365c420e37ce3b00a1f15b3de62029972"
        "345420b76313a885c6ccc6e3b5547857b3ecc6a3636d64d923696e766974655f32615f636c"
        "61696d65725f73656e645f6861736865645f6e6f6e6365"
    );

    let req = invited_cmds::invite_2a_claimer_send_hashed_nonce::Req {
        claimer_hashed_nonce: HashDigest::from(hex!(
            "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
        )),
    };

    let expected = invited_cmds::AnyCmdReq::Invite2aClaimerSendHashedNonce(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let invited_cmds::AnyCmdReq::Invite2aClaimerSendHashedNonce(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   greeter_nonce: hex!("666f6f626172")
    //   status: "ok"
    &hex!(
        "82ad677265657465725f6e6f6e6365c406666f6f626172a6737461747573a26f6b"
    )[..],
    invited_cmds::invite_2a_claimer_send_hashed_nonce::Rep::Ok {
        greeter_nonce: b"foobar".as_ref().into(),
    }
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    invited_cmds::invite_2a_claimer_send_hashed_nonce::Rep::NotFound
)]
#[case::already_deleted(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "already_deleted"
    &hex!(
        "81a6737461747573af616c72656164795f64656c65746564"
    )[..],
    invited_cmds::invite_2a_claimer_send_hashed_nonce::Rep::AlreadyDeleted
)]
#[case::invalid_state(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    &hex!(
        "81a6737461747573ad696e76616c69645f7374617465"
    )[..],
    invited_cmds::invite_2a_claimer_send_hashed_nonce::Rep::InvalidState
)]
fn serde_invite_2a_claimer_send_hashed_nonce_rep(
    #[case] raw: &[u8],
    #[case] expected: invited_cmds::invite_2a_claimer_send_hashed_nonce::Rep,
) {
    let data = invited_cmds::invite_2a_claimer_send_hashed_nonce::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_2a_claimer_send_hashed_nonce::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_2b_greeter_send_nonce_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_2b_greeter_send_nonce"
    //   greeter_nonce: hex!("666f6f626172")
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let raw = hex!(
        "83a3636d64bc696e766974655f32625f677265657465725f73656e645f6e6f6e6365ad6772"
        "65657465725f6e6f6e6365c406666f6f626172a5746f6b656ed802d864b93ded264aae9ae5"
        "83fd3d40c45a"
    );

    let req = authenticated_cmds::invite_2b_greeter_send_nonce::Req {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
        greeter_nonce: b"foobar".as_ref().into(),
    };

    let expected = authenticated_cmds::AnyCmdReq::Invite2bGreeterSendNonce(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let authenticated_cmds::AnyCmdReq::Invite2bGreeterSendNonce(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   claimer_nonce: hex!("666f6f626172")
    //   status: "ok"
    &hex!(
        "82ad636c61696d65725f6e6f6e6365c406666f6f626172a6737461747573a26f6b"
    )[..],
    authenticated_cmds::invite_2b_greeter_send_nonce::Rep::Ok {
        claimer_nonce: b"foobar".as_ref().into(),
    }
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::invite_2b_greeter_send_nonce::Rep::NotFound
)]
#[case::already_deleted(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "already_deleted"
    &hex!(
        "81a6737461747573af616c72656164795f64656c65746564"
    )[..],
    authenticated_cmds::invite_2b_greeter_send_nonce::Rep::AlreadyDeleted
)]
#[case::invalid_state(
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "invalid_state"
        &hex!(
            "81a6737461747573ad696e76616c69645f7374617465"
        )[..],
        authenticated_cmds::invite_2b_greeter_send_nonce::Rep::InvalidState
)]
fn serde_invite_2b_greeter_send_nonce_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::invite_2b_greeter_send_nonce::Rep,
) {
    let data = authenticated_cmds::invite_2b_greeter_send_nonce::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_2b_greeter_send_nonce::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_3a_greeter_wait_peer_trust_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_3a_greeter_wait_peer_trust"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let raw = hex!(
        "82a3636d64d921696e766974655f33615f677265657465725f776169745f706565725f7472"
        "757374a5746f6b656ed802d864b93ded264aae9ae583fd3d40c45a"
    );

    let req = authenticated_cmds::invite_3a_greeter_wait_peer_trust::Req {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::Invite3aGreeterWaitPeerTrust(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let authenticated_cmds::AnyCmdReq::Invite3aGreeterWaitPeerTrust(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    &hex!(
        "81a6737461747573a26f6b"
    )[..],
    authenticated_cmds::invite_3a_greeter_wait_peer_trust::Rep::Ok
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::invite_3a_greeter_wait_peer_trust::Rep::NotFound
)]
#[case::already_deleted(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "already_deleted"
    &hex!(
        "81a6737461747573af616c72656164795f64656c65746564"
    )[..],
    authenticated_cmds::invite_3a_greeter_wait_peer_trust::Rep::AlreadyDeleted
)]
#[case::invalid_state(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    &hex!(
        "81a6737461747573ad696e76616c69645f7374617465"
    )[..],
    authenticated_cmds::invite_3a_greeter_wait_peer_trust::Rep::InvalidState
)]
fn serde_invite_3a_greeter_wait_peer_trust_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::invite_3a_greeter_wait_peer_trust::Rep,
) {
    let data = authenticated_cmds::invite_3a_greeter_wait_peer_trust::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_3a_greeter_wait_peer_trust::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_3b_claimer_wait_peer_trust_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_3b_claimer_wait_peer_trust"
    let raw = hex!(
        "81a3636d64d921696e766974655f33625f636c61696d65725f776169745f706565725f7472"
        "757374"
    );

    let req = invited_cmds::invite_3b_claimer_wait_peer_trust::Req;

    let expected = invited_cmds::AnyCmdReq::Invite3bClaimerWaitPeerTrust(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let invited_cmds::AnyCmdReq::Invite3bClaimerWaitPeerTrust(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    &hex!(
        "81a6737461747573a26f6b"
    )[..],
    invited_cmds::invite_3b_claimer_wait_peer_trust::Rep::Ok
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    invited_cmds::invite_3b_claimer_wait_peer_trust::Rep::NotFound
)]
#[case::invalid_state(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    &hex!(
        "81a6737461747573ad696e76616c69645f7374617465"
    )[..],
    invited_cmds::invite_3b_claimer_wait_peer_trust::Rep::InvalidState
)]
fn serde_invite_3b_claimer_wait_peer_trust_rep(
    #[case] raw: &[u8],
    #[case] expected: invited_cmds::invite_3b_claimer_wait_peer_trust::Rep,
) {
    let data = invited_cmds::invite_3b_claimer_wait_peer_trust::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_3b_claimer_wait_peer_trust::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_3b_greeter_signify_trust_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_3b_greeter_signify_trust"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let raw = hex!(
        "82a3636d64bf696e766974655f33625f677265657465725f7369676e6966795f7472757374"
        "a5746f6b656ed802d864b93ded264aae9ae583fd3d40c45a"
    );

    let req = authenticated_cmds::invite_3b_greeter_signify_trust::Req {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::Invite3bGreeterSignifyTrust(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let authenticated_cmds::AnyCmdReq::Invite3bGreeterSignifyTrust(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    &hex!(
        "81a6737461747573a26f6b"
    )[..],
    authenticated_cmds::invite_3b_greeter_signify_trust::Rep::Ok
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::invite_3b_greeter_signify_trust::Rep::NotFound
)]
#[case::already_deleted(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "already_deleted"
    &hex!(
        "81a6737461747573af616c72656164795f64656c65746564"
    )[..],
    authenticated_cmds::invite_3b_greeter_signify_trust::Rep::AlreadyDeleted
)]
#[case::invalid_state(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    &hex!(
        "81a6737461747573ad696e76616c69645f7374617465"
    )[..],
    authenticated_cmds::invite_3b_greeter_signify_trust::Rep::InvalidState
)]
fn serde_invite_3b_greeter_signify_trust_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::invite_3b_greeter_signify_trust::Rep,
) {
    let data = authenticated_cmds::invite_3b_greeter_signify_trust::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_3b_greeter_signify_trust::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_4_claimer_communicate_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_4_claimer_communicate"
    //   payload: hex!("666f6f626172")
    let raw = hex!(
        "82a3636d64bc696e766974655f345f636c61696d65725f636f6d6d756e6963617465a77061"
        "796c6f6164c406666f6f626172"
    );

    let req = invited_cmds::invite_4_claimer_communicate::Req {
        payload: b"foobar".as_ref().into(),
    };

    let expected = invited_cmds::AnyCmdReq::Invite4ClaimerCommunicate(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let invited_cmds::AnyCmdReq::Invite4ClaimerCommunicate(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   payload: hex!("666f6f626172")
    //   status: "ok"
    &hex!(
        "82a77061796c6f6164c406666f6f626172a6737461747573a26f6b"
    )[..],
    invited_cmds::invite_4_claimer_communicate::Rep::Ok {
        payload: b"foobar".as_ref().into(),
    }
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    invited_cmds::invite_4_claimer_communicate::Rep::NotFound
)]
#[case::invalid_state(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    &hex!(
        "81a6737461747573ad696e76616c69645f7374617465"
    )[..],
    invited_cmds::invite_4_claimer_communicate::Rep::InvalidState
)]
fn serde_invite_4_claimer_communicate_rep(
    #[case] raw: &[u8],
    #[case] expected: invited_cmds::invite_4_claimer_communicate::Rep,
) {
    let data = invited_cmds::invite_4_claimer_communicate::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_4_claimer_communicate::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

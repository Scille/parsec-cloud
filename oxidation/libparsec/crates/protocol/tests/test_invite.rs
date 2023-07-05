// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use std::num::NonZeroU64;

use libparsec_protocol::{
    authenticated_cmds::v2 as authenticated_cmds,
    invited_cmds::v2::{self as invited_cmds, invite_info::ShamirRecoveryRecipient},
};
use libparsec_tests_fixtures::parsec_test;
use libparsec_types::prelude::*;

#[parsec_test]
#[case::user(
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   type: "USER"
    //   claimer_email: "alice@example.com"
    //   cmd: "invite_new"
    //   send_email: true
    //
    &hex!(
        "84a3636d64aa696e766974655f6e6577a474797065a455534552ad636c61696d65725f656d"
        "61696cb1616c696365406578616d706c652e636f6daa73656e645f656d61696cc3"
    )[..],
    authenticated_cmds::AnyCmdReq::InviteNew(authenticated_cmds::invite_new::Req(
        authenticated_cmds::invite_new::UserOrDeviceOrShamirRecovery::User {
            claimer_email: "alice@example.com".to_owned(),
            send_email: true,
        }
    ))
)]
#[case::device(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   type: "DEVICE"
    //   cmd: "invite_new"
    //   send_email: true
    &hex!(
        "83a3636d64aa696e766974655f6e6577aa73656e645f656d61696cc3a474797065a6444556"
        "494345"
    )[..],
    authenticated_cmds::AnyCmdReq::InviteNew(authenticated_cmds::invite_new::Req(
        authenticated_cmds::invite_new::UserOrDeviceOrShamirRecovery::Device {
            send_email: true,
        }
    ))
)]
#[case::shamir_recovery(
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   type: "SHAMIR_RECOVERY"
    //   claimer_user_id: "alice"
    //   cmd: "invite_new"
    //   send_email: true
    //
    &hex!(
        "84a3636d64aa696e766974655f6e6577a474797065af5348414d49525f5245434f56455259"
        "af636c61696d65725f757365725f6964a5616c696365aa73656e645f656d61696cc3"
    )[..],
    authenticated_cmds::AnyCmdReq::InviteNew(authenticated_cmds::invite_new::Req(
        authenticated_cmds::invite_new::UserOrDeviceOrShamirRecovery::ShamirRecovery {
            send_email: true,
            claimer_user_id: "alice".parse().unwrap(),
        }
    ))
)]
fn serde_invite_new_req(#[case] raw: &[u8], #[case] expected: authenticated_cmds::AnyCmdReq) {
    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok_without_email_sent(
    // Generated from Rust implementation (Parsec v2.12.1+dev)
    // Content:
    //   status: "ok"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    //
    &hex!(
        "82a6737461747573a26f6ba5746f6b656ed802d864b93ded264aae9ae583fd3d40c45a"
    )[..],
    authenticated_cmds::invite_new::Rep::Ok {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
        email_sent: Maybe::Absent,
    }
)]
#[case::ok_with_email_sent(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   email_sent: "SUCCESS"
    //   status: "ok"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    &hex!(
        "83aa656d61696c5f73656e74a753554343455353a6737461747573a26f6ba5746f6b656ed8"
        "02d864b93ded264aae9ae583fd3d40c45a"
    )[..],
    authenticated_cmds::invite_new::Rep::Ok {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
        email_sent: Maybe::Present(authenticated_cmds::invite_new::InvitationEmailSentStatus::Success),
    }
)]
#[case::not_allowed(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    &hex!(
        "81a6737461747573ab6e6f745f616c6c6f776564"
    )[..],
    authenticated_cmds::invite_new::Rep::NotAllowed
)]
#[case::already_member(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "already_member"
    &hex!(
        "81a6737461747573ae616c72656164795f6d656d626572"
    )[..],
    authenticated_cmds::invite_new::Rep::AlreadyMember
)]
#[case::not_available(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_available"
    &hex!(
        "81a6737461747573ad6e6f745f617661696c61626c65"
    )[..],
    authenticated_cmds::invite_new::Rep::NotAvailable
)]
fn serde_invite_new_rep(#[case] raw: &[u8], #[case] expected: authenticated_cmds::invite_new::Rep) {
    let data = authenticated_cmds::invite_new::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_new::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_delete_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_delete"
    //   reason: "FINISHED"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let raw = hex!(
        "83a3636d64ad696e766974655f64656c657465a6726561736f6ea846494e4953484544a574"
        "6f6b656ed802d864b93ded264aae9ae583fd3d40c45a"
    );

    let req = authenticated_cmds::invite_delete::Req {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
        reason: authenticated_cmds::invite_delete::InvitationDeletedReason::Finished,
    };

    let expected = authenticated_cmds::AnyCmdReq::InviteDelete(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

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
    authenticated_cmds::invite_delete::Rep::Ok
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::invite_delete::Rep::NotFound
)]
#[case::already_deleted(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "already_deleted"
    &hex!(
        "81a6737461747573af616c72656164795f64656c65746564"
    )[..],
    authenticated_cmds::invite_delete::Rep::AlreadyDeleted
)]
fn serde_invite_delete_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::invite_delete::Rep,
) {
    let data = authenticated_cmds::invite_delete::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_delete::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_list_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_list"
    let raw = hex!("81a3636d64ab696e766974655f6c697374");

    let req = authenticated_cmds::invite_list::Req;

    let expected = authenticated_cmds::AnyCmdReq::InviteList(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_list_rep() {
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   invitations: [
    //     {
    //       type: "USER"
    //       claimer_email: "alice@example.com"
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
    //     {
    //       type: "SHAMIR_RECOVERY"
    //       claimer_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a"
    //       created_on: ext(1, 946774800.0)
    //       status: "IDLE"
    //       token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    //     }
    //   ]
    //   status: "ok"
    //
    let raw = hex!(
        "82a6737461747573a26f6bab696e7669746174696f6e739385a474797065a455534552ad63"
        "6c61696d65725f656d61696cb1616c696365406578616d706c652e636f6daa637265617465"
        "645f6f6ed70141cc375188000000a6737461747573a449444c45a5746f6b656ed802d864b9"
        "3ded264aae9ae583fd3d40c45a84a474797065a6444556494345aa637265617465645f6f6e"
        "d70141cc375188000000a6737461747573a449444c45a5746f6b656ed802d864b93ded264a"
        "ae9ae583fd3d40c45a85a474797065af5348414d49525f5245434f56455259af636c61696d"
        "65725f757365725f6964d92031303962363862613563646634323865613030313766633662"
        "63633034643461aa637265617465645f6f6ed70141cc375188000000a6737461747573a449"
        "444c45a5746f6b656ed802d864b93ded264aae9ae583fd3d40c45a"
    );

    let expected = authenticated_cmds::invite_list::Rep::Ok {
        invitations: vec![
            authenticated_cmds::invite_list::InviteListItem::User {
                token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                claimer_email: "alice@example.com".to_owned(),
                status: InvitationStatus::Idle,
            },
            authenticated_cmds::invite_list::InviteListItem::Device {
                token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                status: InvitationStatus::Idle,
            },
            authenticated_cmds::invite_list::InviteListItem::ShamirRecovery {
                token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                claimer_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
                status: InvitationStatus::Idle,
            },
        ],
    };

    let data = authenticated_cmds::invite_list::Rep::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_list::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

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
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::user(
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   type: "USER"
    //   claimer_email: "alice@example.com"
    //   greeter_human_handle: ["bob@example.com", "bob"]
    //   greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a"
    //   status: "ok"
    //
    &hex!(
        "85a6737461747573a26f6ba474797065a455534552ad636c61696d65725f656d61696cb161"
        "6c696365406578616d706c652e636f6db4677265657465725f68756d616e5f68616e646c65"
        "92af626f62406578616d706c652e636f6da3626f62af677265657465725f757365725f6964"
        "d9203130396236386261356364663432386561303031376663366263633034643461"
    ),
    invited_cmds::invite_info::Rep::Ok(
        invited_cmds::invite_info::UserOrDeviceOrShamirRecovery::User {
            claimer_email: "alice@example.com".to_owned(),
            greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
            greeter_human_handle: Some(HumanHandle::new("bob@example.com", "bob").unwrap()),
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
        invited_cmds::invite_info::UserOrDeviceOrShamirRecovery::Device {
            greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
            greeter_human_handle: Some(HumanHandle::new("bob@dev1", "bob").unwrap()),
        }
    )
)]
#[case::user_no_human_handle(
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   type: "USER"
    //   claimer_email: "alice@example.com"
    //   greeter_human_handle: None
    //   greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a"
    //   status: "ok"
    //
    &hex!(
        "85a6737461747573a26f6ba474797065a455534552ad636c61696d65725f656d61696cb161"
        "6c696365406578616d706c652e636f6db4677265657465725f68756d616e5f68616e646c65"
        "c0af677265657465725f757365725f6964d920313039623638626135636466343238656130"
        "3031376663366263633034643461"
    ),
    invited_cmds::invite_info::Rep::Ok(
        invited_cmds::invite_info::UserOrDeviceOrShamirRecovery::User {
            greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
            claimer_email: "alice@example.com".to_string(),
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
        invited_cmds::invite_info::UserOrDeviceOrShamirRecovery::Device {
            greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
            greeter_human_handle: None,
        }
    )
)]
#[case::shamir_recovery_without_recipients(
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   type: "SHAMIR_RECOVERY"
    //   recipients: []
    //   status: "ok"
    //   threshold: 1
    //
    &hex!(
        "84a6737461747573a26f6ba474797065af5348414d49525f5245434f56455259aa72656369"
        "7069656e747390a97468726573686f6c6401"
    )[..],
    invited_cmds::invite_info::Rep::Ok(
        invited_cmds::invite_info::UserOrDeviceOrShamirRecovery::ShamirRecovery {
            threshold: NonZeroU64::new(1).unwrap(),
            recipients: vec![],
        }
    )
)]
#[case::shamir_recovery_with_recipients(
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   type: "SHAMIR_RECOVERY"
    //   recipients: [
    //     {
    //       human_handle: ["alice@example.com", "Alicey McAliceFace"]
    //       shares: 1
    //       user_id: "alice"
    //     }
    //   ]
    //   status: "ok"
    //   threshold: 1
    //
    &hex!(
        "84a6737461747573a26f6ba474797065af5348414d49525f5245434f56455259aa72656369"
        "7069656e74739183ac68756d616e5f68616e646c6592b1616c696365406578616d706c652e"
        "636f6db2416c69636579204d63416c69636546616365a673686172657301a7757365725f69"
        "64a5616c696365a97468726573686f6c6401"
    )[..],
    invited_cmds::invite_info::Rep::Ok(
        invited_cmds::invite_info::UserOrDeviceOrShamirRecovery::ShamirRecovery {
            threshold: NonZeroU64::new(1).unwrap(),
            recipients: vec![ShamirRecoveryRecipient {
                user_id: "alice".parse().unwrap(),
                human_handle: Some("AliceyMcAliceFace <alice@example.com>".parse().unwrap()),
                shares: NonZeroU64::new(1).unwrap(),
            }],
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
fn serde_invite_1_claimer_wait_peer_req_legacy() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   claimer_public_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //   cmd: "invite_1_claimer_wait_peer"
    let raw = hex!(
        "82b2636c61696d65725f7075626c69635f6b6579c4206507907d33bae6b5980b32fa03f3eb"
        "ac56141b126e44f352ea46c5f22cd5ac57a3636d64ba696e766974655f315f636c61696d65"
        "725f776169745f70656572"
    );

    let req = invited_cmds::invite_1_claimer_wait_peer::Req {
        greeter_user_id: Maybe::Absent,
        claimer_public_key: PublicKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
    };

    let expected = invited_cmds::AnyCmdReq::Invite1ClaimerWaitPeer(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_1_claimer_wait_peer_req() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   claimer_public_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //   cmd: "invite_1_claimer_wait_peer"
    //   greeter_user_id: "alice"
    let raw = hex!(
      "83a3636d64ba696e766974655f315f636c61696d65725f776169745f70656572b2636c6169"
      "6d65725f7075626c69635f6b6579c4206507907d33bae6b5980b32fa03f3ebac56141b126e"
      "44f352ea46c5f22cd5ac57af677265657465725f757365725f6964a5616c696365"
    );

    let req = invited_cmds::invite_1_claimer_wait_peer::Req {
        claimer_public_key: PublicKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
        greeter_user_id: Maybe::Present("alice".parse().unwrap()),
    };

    let expected = invited_cmds::AnyCmdReq::Invite1ClaimerWaitPeer(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   greeter_public_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //   status: "ok"
    &hex!(
        "82b2677265657465725f7075626c69635f6b6579c4206507907d33bae6b5980b32fa03f3eb"
        "ac56141b126e44f352ea46c5f22cd5ac57a6737461747573a26f6b"
    )[..],
    invited_cmds::invite_1_claimer_wait_peer::Rep::Ok {
        greeter_public_key: PublicKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
    }
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    invited_cmds::invite_1_claimer_wait_peer::Rep::NotFound
)]
#[case::invalid_state(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    &hex!(
        "81a6737461747573ad696e76616c69645f7374617465"
    )[..],
    invited_cmds::invite_1_claimer_wait_peer::Rep::InvalidState
)]
fn serde_invite_1_claimer_wait_peer_rep(
    #[case] raw: &[u8],
    #[case] expected: invited_cmds::invite_1_claimer_wait_peer::Rep,
) {
    let data = invited_cmds::invite_1_claimer_wait_peer::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_1_claimer_wait_peer::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_1_greeter_wait_peer_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_1_greeter_wait_peer"
    //   greeter_public_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let raw = hex!(
        "83a3636d64ba696e766974655f315f677265657465725f776169745f70656572b267726565"
        "7465725f7075626c69635f6b6579c4206507907d33bae6b5980b32fa03f3ebac56141b126e"
        "44f352ea46c5f22cd5ac57a5746f6b656ed802d864b93ded264aae9ae583fd3d40c45a"
    );

    let req = authenticated_cmds::invite_1_greeter_wait_peer::Req {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
        greeter_public_key: PublicKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
    };

    let expected = authenticated_cmds::AnyCmdReq::Invite1GreeterWaitPeer(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   claimer_public_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //   status: "ok"
    &hex!(
        "82b2636c61696d65725f7075626c69635f6b6579c4206507907d33bae6b5980b32fa03f3eb"
        "ac56141b126e44f352ea46c5f22cd5ac57a6737461747573a26f6b"
    )[..],
    authenticated_cmds::invite_1_greeter_wait_peer::Rep::Ok {
        claimer_public_key: PublicKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
    }
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::invite_1_greeter_wait_peer::Rep::NotFound
)]
#[case::already_deleted(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "already_deleted"
    &hex!(
        "81a6737461747573af616c72656164795f64656c65746564"
    )[..],
    authenticated_cmds::invite_1_greeter_wait_peer::Rep::AlreadyDeleted
)]
#[case::invalid_state(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    &hex!(
        "81a6737461747573ad696e76616c69645f7374617465"
    )[..],
    authenticated_cmds::invite_1_greeter_wait_peer::Rep::InvalidState
)]
fn serde_invite_1_greeter_wait_peer_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::invite_1_greeter_wait_peer::Rep,
) {
    let data = authenticated_cmds::invite_1_greeter_wait_peer::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_1_greeter_wait_peer::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_2a_claimer_send_hashed_nonce_req_legacy() {
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
        greeter_user_id: Maybe::Absent,
        claimer_hashed_nonce: HashDigest::from(hex!(
            "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
        )),
    };

    let expected = invited_cmds::AnyCmdReq::Invite2aClaimerSendHashedNonce(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_2a_claimer_send_hashed_nonce_req() {
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   claimer_hashed_nonce: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
    //   cmd: "invite_2a_claimer_send_hashed_nonce"
    //   greeter_user_id: "alice"
    //
    let raw = hex!(
        "83a3636d64d923696e766974655f32615f636c61696d65725f73656e645f6861736865645f"
        "6e6f6e6365b4636c61696d65725f6861736865645f6e6f6e6365c420e37ce3b00a1f15b3de"
        "62029972345420b76313a885c6ccc6e3b5547857b3ecc6af677265657465725f757365725f"
        "6964a5616c696365"
    );

    let req = invited_cmds::invite_2a_claimer_send_hashed_nonce::Req {
        greeter_user_id: Maybe::Present("alice".parse().unwrap()),
        claimer_hashed_nonce: HashDigest::from(hex!(
            "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
        )),
    };

    let expected = invited_cmds::AnyCmdReq::Invite2aClaimerSendHashedNonce(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

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
        greeter_nonce: b"foobar".to_vec(),
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
fn serde_invite_2a_greeter_get_hashed_nonce_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_2a_greeter_get_hashed_nonce"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let raw = hex!(
        "82a3636d64d922696e766974655f32615f677265657465725f6765745f6861736865645f6e"
        "6f6e6365a5746f6b656ed802d864b93ded264aae9ae583fd3d40c45a"
    );

    let req = authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Req {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::Invite2aGreeterGetHashedNonce(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   claimer_hashed_nonce: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
    //   status: "ok"
    &hex!(
        "82b4636c61696d65725f6861736865645f6e6f6e6365c420e37ce3b00a1f15b3de62029972"
        "345420b76313a885c6ccc6e3b5547857b3ecc6a6737461747573a26f6b"
    )[..],
    authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Rep::Ok {
        claimer_hashed_nonce: HashDigest::from(hex!(
            "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
        )),
    }
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Rep::NotFound
)]
#[case::already_deleted(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "already_deleted"
    &hex!(
        "81a6737461747573af616c72656164795f64656c65746564"
    )[..],
    authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Rep::AlreadyDeleted
)]
#[case::invalid_state(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    &hex!(
        "81a6737461747573ad696e76616c69645f7374617465"
    )[..],
    authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Rep::InvalidState
)]
fn serde_invite_2a_greeter_get_hashed_nonce_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Rep,
) {
    let data = authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Rep::load(&raw2).unwrap();

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
        greeter_nonce: b"foobar".to_vec(),
    };

    let expected = authenticated_cmds::AnyCmdReq::Invite2bGreeterSendNonce(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

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
        claimer_nonce: b"foobar".to_vec(),
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
fn serde_invite_2b_claimer_send_nonce_req_legacy() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   claimer_nonce: hex!("666f6f626172")
    //   cmd: "invite_2b_claimer_send_nonce"
    let raw = hex!(
        "82ad636c61696d65725f6e6f6e6365c406666f6f626172a3636d64bc696e766974655f3262"
        "5f636c61696d65725f73656e645f6e6f6e6365"
    );

    let req = invited_cmds::invite_2b_claimer_send_nonce::Req {
        greeter_user_id: Maybe::Absent,
        claimer_nonce: b"foobar".to_vec(),
    };

    let expected = invited_cmds::AnyCmdReq::Invite2bClaimerSendNonce(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_2b_claimer_send_nonce_req() {
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   claimer_nonce: hex!("666f6f626172")
    //   cmd: "invite_2b_claimer_send_nonce"
    //   greeter_user_id: "alice"
    //
    let raw = hex!(
        "83a3636d64bc696e766974655f32625f636c61696d65725f73656e645f6e6f6e6365ad636c"
        "61696d65725f6e6f6e6365c406666f6f626172af677265657465725f757365725f6964a561"
        "6c696365"
    );

    let req = invited_cmds::invite_2b_claimer_send_nonce::Req {
        greeter_user_id: Maybe::Present("alice".parse().unwrap()),
        claimer_nonce: b"foobar".to_vec(),
    };

    let expected = invited_cmds::AnyCmdReq::Invite2bClaimerSendNonce(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

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
    invited_cmds::invite_2b_claimer_send_nonce::Rep::Ok
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    invited_cmds::invite_2b_claimer_send_nonce::Rep::NotFound
)]
#[case::invalid_state(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    &hex!(
        "81a6737461747573ad696e76616c69645f7374617465"
    )[..],
    invited_cmds::invite_2b_claimer_send_nonce::Rep::InvalidState
)]
fn serde_invite_2b_claimer_send_nonce_rep(
    #[case] raw: &[u8],
    #[case] expected: invited_cmds::invite_2b_claimer_send_nonce::Rep,
) {
    let data = invited_cmds::invite_2b_claimer_send_nonce::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_2b_claimer_send_nonce::Rep::load(&raw2).unwrap();

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
    let raw2 = data.dump().unwrap();

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
fn serde_invite_3b_claimer_wait_peer_trust_req_legacy() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_3b_claimer_wait_peer_trust"
    let raw = hex!(
        "81a3636d64d921696e766974655f33625f636c61696d65725f776169745f706565725f7472"
        "757374"
    );

    let req = invited_cmds::invite_3b_claimer_wait_peer_trust::Req {
        greeter_user_id: Maybe::Absent,
    };

    let expected = invited_cmds::AnyCmdReq::Invite3bClaimerWaitPeerTrust(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_3b_claimer_wait_peer_trust_req() {
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   cmd: "invite_3b_claimer_wait_peer_trust"
    //   greeter_user_id: "alice"
    //
    let raw = hex!(
        "82a3636d64d921696e766974655f33625f636c61696d65725f776169745f706565725f7472"
        "757374af677265657465725f757365725f6964a5616c696365"
    );

    let req = invited_cmds::invite_3b_claimer_wait_peer_trust::Req {
        greeter_user_id: Maybe::Present("alice".parse().unwrap()),
    };

    let expected = invited_cmds::AnyCmdReq::Invite3bClaimerWaitPeerTrust(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

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
    let raw2 = data.dump().unwrap();

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
fn serde_invite_3a_claimer_signify_trust_req_legacy() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_3a_claimer_signify_trust"
    let raw = hex!("81a3636d64bf696e766974655f33615f636c61696d65725f7369676e6966795f7472757374");

    let req = invited_cmds::invite_3a_claimer_signify_trust::Req {
        greeter_user_id: Maybe::Absent,
    };

    let expected = invited_cmds::AnyCmdReq::Invite3aClaimerSignifyTrust(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_3a_claimer_signify_trust_req() {
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   cmd: "invite_3a_claimer_signify_trust"
    //   greeter_user_id: "alice"
    //
    let raw = hex!(
        "82a3636d64bf696e766974655f33615f636c61696d65725f7369676e6966795f7472757374"
        "af677265657465725f757365725f6964a5616c696365"
    );

    let req = invited_cmds::invite_3a_claimer_signify_trust::Req {
        greeter_user_id: Maybe::Present("alice".parse().unwrap()),
    };

    let expected = invited_cmds::AnyCmdReq::Invite3aClaimerSignifyTrust(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

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
    invited_cmds::invite_3a_claimer_signify_trust::Rep::Ok
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    invited_cmds::invite_3a_claimer_signify_trust::Rep::NotFound
)]
#[case::invalid_state(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    &hex!(
        "81a6737461747573ad696e76616c69645f7374617465"
    )[..],
    invited_cmds::invite_3a_claimer_signify_trust::Rep::InvalidState
)]
fn serde_invite_3a_claimer_signify_trust_rep(
    #[case] raw: &[u8],
    #[case] expected: invited_cmds::invite_3a_claimer_signify_trust::Rep,
) {
    let data = invited_cmds::invite_3a_claimer_signify_trust::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_3a_claimer_signify_trust::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_4_greeter_communicate_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_4_greeter_communicate"
    //   payload: hex!("666f6f626172")
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let raw = hex!(
        "83a3636d64bc696e766974655f345f677265657465725f636f6d6d756e6963617465a77061"
        "796c6f6164c406666f6f626172a5746f6b656ed802d864b93ded264aae9ae583fd3d40c45a"
    );

    let req = authenticated_cmds::invite_4_greeter_communicate::Req {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
        payload: b"foobar".to_vec(),
    };

    let expected = authenticated_cmds::AnyCmdReq::Invite4GreeterCommunicate(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

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
    authenticated_cmds::invite_4_greeter_communicate::Rep::Ok {
        payload: b"foobar".to_vec(),
    }
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::invite_4_greeter_communicate::Rep::NotFound
)]
#[case::already_deleted(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "already_deleted"
    &hex!(
        "81a6737461747573af616c72656164795f64656c65746564"
    )[..],
    authenticated_cmds::invite_4_greeter_communicate::Rep::AlreadyDeleted
)]
#[case::invalid_state(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    &hex!(
        "81a6737461747573ad696e76616c69645f7374617465"
    )[..],
    authenticated_cmds::invite_4_greeter_communicate::Rep::InvalidState
)]
fn serde_invite_4_greeter_communicate_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::invite_4_greeter_communicate::Rep,
) {
    let data = authenticated_cmds::invite_4_greeter_communicate::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();
    let data2 = authenticated_cmds::invite_4_greeter_communicate::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_4_claimer_communicate_req_legacy() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_4_claimer_communicate"
    //   payload: hex!("666f6f626172")
    let raw = hex!(
        "82a3636d64bc696e766974655f345f636c61696d65725f636f6d6d756e6963617465a77061"
        "796c6f6164c406666f6f626172"
    );

    let req = invited_cmds::invite_4_claimer_communicate::Req {
        greeter_user_id: Maybe::Absent,
        payload: b"foobar".to_vec(),
    };

    let expected = invited_cmds::AnyCmdReq::Invite4ClaimerCommunicate(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_4_claimer_communicate_req() {
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   cmd: "invite_4_claimer_communicate"
    //   greeter_user_id: "alice"
    //   payload: hex!("666f6f626172")
    //
    let raw = hex!(
        "83a3636d64bc696e766974655f345f636c61696d65725f636f6d6d756e6963617465af6772"
        "65657465725f757365725f6964a5616c696365a77061796c6f6164c406666f6f626172"
    );

    let req = invited_cmds::invite_4_claimer_communicate::Req {
        greeter_user_id: Maybe::Present("alice".parse().unwrap()),
        payload: b"foobar".to_vec(),
    };

    let expected = invited_cmds::AnyCmdReq::Invite4ClaimerCommunicate(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

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
        payload: b"foobar".to_vec(),
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

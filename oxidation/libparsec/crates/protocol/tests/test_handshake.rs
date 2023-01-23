// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use core::panic;
use hex_literal::hex;
use serde_json::{json, Value};
use std::str::FromStr;

use rstest::rstest;

use libparsec_protocol::*;
use libparsec_types::{DateTime, InvitationToken, InvitationType, OrganizationID};

use libparsec_tests_fixtures::{alice, bob, timestamp, Device};

#[cfg(feature = "test")]
#[rstest]
fn test_good_authenticated_handshake_server(alice: &Device, timestamp: DateTime) {
    let challenge = hex!(
        "bbf9777bdc479d6c3d6a77b93aa26370f88ee51f81f3983a074ab32fbfcd33ef6a1b6e3f2e9718e2"
        "27d0b59e8a749078"
    );

    let sh = ServerHandshakeStalled::default()
        .build_challenge_req_with_challenge(challenge, timestamp)
        .unwrap();

    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   type: "AUTHENTICATED"
    //   answer: hex!(
    //     "aea26853c5d2b206eb3fc532d035164f1b16dea5268b32a3d068fd998853e85a67285b0f81c2045b"
    //     "e19b7be1c3baad937c5426d68e030ddb46434f2e4d0a560d82a6616e73776572c430bbf9777bdc47"
    //     "9d6c3d6a77b93aa26370f88ee51f81f3983a074ab32fbfcd33ef6a1b6e3f2e9718e227d0b59e8a74"
    //     "9078a474797065ad7369676e65645f616e73776572"
    //   )
    //   client_api_version: [2, 5]
    //   device_id: "alice@dev1"
    //   handshake: "answer"
    //   organization_id: "CoolOrg"
    //   rvk: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    let answer_req = hex!(
        "87a6616e73776572c48daea26853c5d2b206eb3fc532d035164f1b16dea5268b32a3d068fd"
        "998853e85a67285b0f81c2045be19b7be1c3baad937c5426d68e030ddb46434f2e4d0a560d"
        "82a6616e73776572c430bbf9777bdc479d6c3d6a77b93aa26370f88ee51f81f3983a074ab3"
        "2fbfcd33ef6a1b6e3f2e9718e227d0b59e8a749078a474797065ad7369676e65645f616e73"
        "776572b2636c69656e745f6170695f76657273696f6e920205a96465766963655f6964aa61"
        "6c6963654064657631a968616e647368616b65a6616e73776572af6f7267616e697a617469"
        "6f6e5f6964a7436f6f6c4f7267a372766bc420be2976732cec8ca94eedcf0aafd413cd1593"
        "63e0fadc9e68572c77a1e17d9bbda474797065ad41555448454e54494341544544"
    );

    let sh = sh.process_answer_req(&answer_req).unwrap();

    assert_eq!(
        sh.data,
        Answer::Authenticated {
            answer: hex!(
                "aea26853c5d2b206eb3fc532d035164f1b16dea5268b32a3d068fd998853e85a67285b0f81c2045b"
                "e19b7be1c3baad937c5426d68e030ddb46434f2e4d0a560d82a6616e73776572c430bbf9777bdc47"
                "9d6c3d6a77b93aa26370f88ee51f81f3983a074ab32fbfcd33ef6a1b6e3f2e9718e227d0b59e8a74"
                "9078a474797065ad7369676e65645f616e73776572"
            )
            .to_vec(),
            client_api_version: API_V2_VERSION,
            organization_id: alice.organization_id().to_owned(),
            device_id: alice.device_id.to_owned(),
            rvk: Box::new(alice.root_verify_key().to_owned()),
        }
    );

    let sh = sh.build_result_req(Some(alice.verify_key())).unwrap();

    assert_eq!(sh.client_api_version, API_V2_VERSION);
}

#[rstest]
fn test_good_authenticated_handshake_client(alice: &Device) {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   backend_timestamp: ext(1, 1647884434.47775)
    //   ballpark_client_early_offset: 50.0
    //   ballpark_client_late_offset: 70.0
    //   challenge: hex!(
    //     "58f7ec2bb24b81a57feee1bad250726a2f7588a3cdd0617a206687adf8fb1274f34b0aebf9fd27a5"
    //     "d29f56dce902ddcd"
    //   )
    //   handshake: "challenge"
    //   supported_api_versions: [[2, 5], [1, 3]
    let challenge_req = hex!(
        "86b16261636b656e645f74696d657374616d70d70141d88e2e249e9375bc62616c6c706172"
        "6b5f636c69656e745f6561726c795f6f6666736574cb4049000000000000bb62616c6c7061"
        "726b5f636c69656e745f6c6174655f6f6666736574cb4051800000000000a96368616c6c65"
        "6e6765c43058f7ec2bb24b81a57feee1bad250726a2f7588a3cdd0617a206687adf8fb1274"
        "f34b0aebf9fd27a5d29f56dce902ddcda968616e647368616b65a96368616c6c656e6765b6"
        "737570706f727465645f6170695f76657273696f6e7392920205920103"
    );

    let ch = AuthenticatedClientHandshakeStalled::new(
        alice.organization_id().clone(),
        alice.device_id.clone(),
        alice.signing_key.clone(),
        alice.root_verify_key().clone(),
        "2022-03-21T17:40:34.477750Z".parse().unwrap(),
    );

    ch.process_challenge_req(&challenge_req).unwrap();
}

#[rstest]
fn test_good_authenticated_handshake(alice: &Device, timestamp: DateTime) {
    let t1 = timestamp;
    let t2 = t1.add_us(1);
    let sh = ServerHandshakeStalled::default()
        .build_challenge_req(t1)
        .unwrap();

    let ch = AuthenticatedClientHandshakeStalled::new(
        alice.organization_id().clone(),
        alice.device_id.clone(),
        alice.signing_key.clone(),
        alice.root_verify_key().clone(),
        t2,
    )
    .process_challenge_req(&sh.raw)
    .unwrap();

    let sh = sh.process_answer_req(&ch.raw).unwrap();

    if let Answer::Authenticated {
        client_api_version,
        organization_id,
        device_id,
        rvk,
        ..
    } = &sh.data
    {
        assert_eq!(*client_api_version, API_V2_VERSION);
        assert_eq!(organization_id, alice.organization_id());
        assert_eq!(*device_id, alice.device_id);
        assert_eq!(**rvk, *alice.root_verify_key());
    } else {
        panic!("unexpected value `sh.data`")
    }

    let sh = sh.build_result_req(Some(alice.verify_key())).unwrap();

    assert_eq!(sh.client_api_version, API_V2_VERSION);
}

#[cfg(feature = "test")]
#[rstest]
#[case::user((
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   type: "INVITED"
    //   client_api_version: [2, 5]
    //   handshake: "answer"
    //   invitation_type: "USER"
    //   organization_id: "Org"
    //   token: ext(2, hex!("9931e631856a44709c825d7f7d339197")
    &hex!(
        "86b2636c69656e745f6170695f76657273696f6e920205a968616e647368616b65a6616e73"
        "776572af696e7669746174696f6e5f74797065a455534552af6f7267616e697a6174696f6e"
        "5f6964a34f7267a5746f6b656ed8029931e631856a44709c825d7f7d339197a474797065a7"
        "494e5649544544"
    )[..],
    InvitationType::User,
    "9931e631856a44709c825d7f7d339197".parse().unwrap(),
    hex!(
        "496749c018fb2d3a6cf52befaab622d9f0c377e6f5be7a5fe9c40dab2845d3bd135d3e589212f3c9"
        "f718bf26ca792616"
    )
))]
#[case::device((
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   type: "INVITED"
    //   client_api_version: [2, 5]
    //   handshake: "answer"
    //   invitation_type: "DEVICE"
    //   organization_id: "Org"
    //   token: ext(2, hex!("4e12636f08c840c4bb09404e1e696b09"))
    &hex!(
        "86b2636c69656e745f6170695f76657273696f6e920205a968616e647368616b65a6616e73"
        "776572af696e7669746174696f6e5f74797065a6444556494345af6f7267616e697a617469"
        "6f6e5f6964a34f7267a5746f6b656ed8024e12636f08c840c4bb09404e1e696b09a4747970"
        "65a7494e5649544544"
    )[..],
    InvitationType::Device,
    "4e12636f08c840c4bb09404e1e696b09".parse().unwrap(),
    hex!(
        "0d5db6bdf55ea3e54311a2a4608dcaa5af942de37ac201c3c0eb32117dd4941feadde89aac57fa3f"
        "5ba7736fbfd11d75"
    )
))]
fn test_good_invited_handshake_server(
    timestamp: DateTime,
    #[case] input: (
        &[u8],
        InvitationType,
        InvitationToken,
        [u8; HANDSHAKE_CHALLENGE_SIZE],
    ),
) {
    let _organization_id = OrganizationID::from_str("Org").unwrap();
    let (answer_req, _invitation_type, _token, challenge) = input;

    let sh = ServerHandshakeStalled::default()
        .build_challenge_req_with_challenge(challenge, timestamp)
        .unwrap();

    let sh = sh.process_answer_req(&answer_req).unwrap();

    if let Answer::Invited {
        client_api_version,
        organization_id,
        invitation_type,
        token,
        ..
    } = &sh.data
    {
        assert_eq!(*client_api_version, API_V2_VERSION);
        assert_eq!(*organization_id, _organization_id);
        assert_eq!(*invitation_type, _invitation_type);
        assert_eq!(*token, _token);
    } else {
        assert!(false)
    }

    let sh = sh.build_result_req(None).unwrap();

    assert_eq!(sh.client_api_version, API_V2_VERSION);
}

#[rstest]
#[case::user((
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   backend_timestamp: ext(1, 1647885016.472496)
    //   ballpark_client_early_offset: 50.0
    //   ballpark_client_late_offset: 70.0
    //   challenge: hex!(
    //     "496749c018fb2d3a6cf52befaab622d9f0c377e6f5be7a5fe9c40dab2845d3bd135d3e589212f3c9"
    //     "f718bf26ca792616"
    //   )
    //   handshake: "challenge"
    //   supported_api_versions: [[2, 5], [1, 3]]
    &hex!(
        "86b16261636b656e645f74696d657374616d70d70141d88e2eb61e3d60bc62616c6c706172"
        "6b5f636c69656e745f6561726c795f6f6666736574cb4049000000000000bb62616c6c7061"
        "726b5f636c69656e745f6c6174655f6f6666736574cb4051800000000000a96368616c6c65"
        "6e6765c430496749c018fb2d3a6cf52befaab622d9f0c377e6f5be7a5fe9c40dab2845d3bd"
        "135d3e589212f3c9f718bf26ca792616a968616e647368616b65a96368616c6c656e6765b6"
        "737570706f727465645f6170695f76657273696f6e7392920205920103"
    )[..],
    InvitationType::User,
    InvitationToken::from_hex("9931e631856a44709c825d7f7d339197").unwrap(),
))]
#[case::device((
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   backend_timestamp: ext(1, 1647885016.474222)
    //   ballpark_client_early_offset: 50.0
    //   ballpark_client_late_offset: 70.0
    //   challenge: hex!(
    //     "0d5db6bdf55ea3e54311a2a4608dcaa5af942de37ac201c3c0eb32117dd4941feadde89aac57fa3f"
    //     "5ba7736fbfd11d75"
    //   )
    //   handshake: "challenge"
    //   supported_api_versions: [[2, 5], [1, 3]]
    &hex!(
        "86b16261636b656e645f74696d657374616d70d70141d88e2eb61e59a7bc62616c6c706172"
        "6b5f636c69656e745f6561726c795f6f6666736574cb4049000000000000bb62616c6c7061"
        "726b5f636c69656e745f6c6174655f6f6666736574cb4051800000000000a96368616c6c65"
        "6e6765c4300d5db6bdf55ea3e54311a2a4608dcaa5af942de37ac201c3c0eb32117dd4941f"
        "eadde89aac57fa3f5ba7736fbfd11d75a968616e647368616b65a96368616c6c656e6765b6"
        "737570706f727465645f6170695f76657273696f6e7392920205920103"
    )[..],
    InvitationType::Device,
    InvitationToken::from_hex("4e12636f08c840c4bb09404e1e696b09").unwrap(),
))]
fn test_good_invited_handshake_client(#[case] input: (&[u8], InvitationType, InvitationToken)) {
    let organization_id = OrganizationID::from_str("Org").unwrap();
    let (challenge_req, invitation_type, token) = input;

    let ch = InvitedClientHandshakeStalled::new(
        organization_id,
        invitation_type,
        token,
        "2022-03-21T17:50:16.474222Z".parse().unwrap(),
    );

    ch.process_challenge_req(challenge_req).unwrap();
}

#[rstest]
#[case::user(InvitationType::User)]
#[case::device(InvitationType::Device)]
fn test_good_invited_handshake(timestamp: DateTime, #[case] _invitation_type: InvitationType) {
    let t1 = timestamp;
    let t2 = t1.add_us(1);
    let _organization_id = OrganizationID::default();
    let _token = InvitationToken::default();

    let sh = ServerHandshakeStalled::default()
        .build_challenge_req(t1)
        .unwrap();

    let ch =
        InvitedClientHandshakeStalled::new(_organization_id.clone(), _invitation_type, _token, t2)
            .process_challenge_req(&sh.raw)
            .unwrap();

    let sh = sh.process_answer_req(&ch.raw).unwrap();

    if let Answer::Invited {
        client_api_version,
        organization_id,
        invitation_type,
        token,
        ..
    } = &sh.data
    {
        assert_eq!(*client_api_version, API_V2_VERSION);
        assert_eq!(*organization_id, _organization_id);
        assert_eq!(*invitation_type, _invitation_type);
        assert_eq!(*token, _token);
    } else {
        panic!("unexpected value for `sh.data`")
    }

    let sh = sh.build_result_req(None).unwrap();

    assert_eq!(sh.client_api_version, API_V2_VERSION);
}

// 1) Server build challenge (nothing more to test...)

// 2) Client process challenge

#[rstest]
#[case(json!({}))]
#[case(json!({
    "handshake": "foo",
    "challenge": b"58f7ec2bb24b81a57feee1bad250726a2f7588a3cdd0617a206687adf8fb1274f34b0aebf9fd27a5d29f56dce902ddcd".to_vec(),
    "supported_api_versions": [API_V2_VERSION],
}))]
#[case(json!({
    "handshake": "challenge",
    "challenge": b"58f7ec2bb24b81a57feee1bad250726a2f7588a3cdd0617a206687adf8fb1274f34b0aebf9fd27a5d29f56dce902ddcd".to_vec()
}))]
#[case(json!({
    "challenge": b"58f7ec2bb24b81a57feee1bad250726a2f7588a3cdd0617a206687adf8fb1274f34b0aebf9fd27a5d29f56dce902ddcd".to_vec()
}))]
#[case(json!({
    "challenge": b"58f7ec2bb24b81a57feee1bad250726a2f7588a3cdd0617a206687adf8fb1274f34b0aebf9fd27a5d29f56dce902ddcd".to_vec(),
    "supported_api_versions": [API_V2_VERSION]
}))]
#[case(json!({
    "handshake": "challenge",
}))]
#[case(json!({
    "handshake": "challenge",
    "supported_api_versions": [API_V2_VERSION]
}))]
#[case(json!({
    "handshake": "challenge",
    "challenge": 42,
    "supported_api_versions": [API_V2_VERSION]
}))]
#[case(json!({
    "handshake": "challenge",
    "challenge": b"58f7ec2bb24b81a57feee1bad250726a2f7588a3cdd0617a206687adf8fb1274f34b0aebf9fd27a5d29f56dce902ddcd".to_vec()
}))]
#[case(json!({
    "handshake": "challenge",
    "challenge": b"58f7ec2bb24b81a57feee1bad250726a2f7588a3cdd0617a206687adf8fb1274f34b0aebf9fd27a5d29f56dce902ddcd".to_vec(),
    "supported_api_versions": "invalid"
}))]
fn test_process_challenge_req_bad_format(alice: &Device, timestamp: DateTime, #[case] req: Value) {
    let req = rmp_serde::to_vec_named(&req).unwrap();

    let err = AuthenticatedClientHandshakeStalled::new(
        alice.organization_id().clone(),
        alice.device_id.clone(),
        alice.signing_key.clone(),
        alice.root_verify_key().clone(),
        timestamp,
    )
    .process_challenge_req(&req)
    .unwrap_err();

    assert!(matches!(err, HandshakeError::InvalidMessage(_)));
}

// 2-b) Client check API version

#[rstest]
#[case((ApiVersion { version: 2, revision: 22}, ApiVersion { version: 1, revision: 0 }, false))]
#[case((ApiVersion { version: 2, revision: 22}, ApiVersion { version: 1, revision: 111 }, false))]
#[case((ApiVersion { version: 2, revision: 22}, ApiVersion { version: 2, revision: 0 }, true))]
#[case((ApiVersion { version: 2, revision: 22}, ApiVersion { version: 2, revision: 22 }, true))]
#[case((ApiVersion { version: 2, revision: 22}, ApiVersion { version: 2, revision: 222 }, true))]
#[case((ApiVersion { version: 2, revision: 22}, ApiVersion { version: 3, revision: 0 }, false))]
#[case((ApiVersion { version: 2, revision: 22}, ApiVersion { version: 3, revision: 33 }, false))]
#[case((ApiVersion { version: 2, revision: 22}, ApiVersion { version: 3, revision: 333 }, false))]
fn test_process_challenge_req_good_api_version(
    alice: &Device,
    timestamp: DateTime,
    #[case] input: (ApiVersion, ApiVersion, bool),
) {
    let (client_version, backend_version, valid) = input;
    let t1 = timestamp;
    let t2 = t1.add_us(1);

    let req = Handshake::Challenge {
        challenge: hex!("58f7ec2bb24b81a57feee1bad250726a2f7588a3cdd0617a206687adf8fb1274f34b0aebf9fd27a5d29f56dce902ddcd"),
        supported_api_versions: vec![backend_version],
        backend_timestamp: Some(t1),
        ballpark_client_early_offset: Some(BALLPARK_CLIENT_EARLY_OFFSET),
        ballpark_client_late_offset: Some(BALLPARK_CLIENT_LATE_OFFSET),
    }
    .dump()
    .unwrap();

    let mut ch = AuthenticatedClientHandshakeStalled::new(
        alice.organization_id().clone(),
        alice.device_id.clone(),
        alice.signing_key.clone(),
        alice.root_verify_key().clone(),
        t2,
    );

    ch.supported_api_versions = vec![client_version];

    if !valid {
        // Invalid versioning
        let err = ch.process_challenge_req(&req).unwrap_err();
        match err {
            HandshakeError::APIVersion {
                client_versions,
                backend_versions,
            } => {
                assert_eq!(client_versions, vec![client_version]);
                assert_eq!(backend_versions, vec![backend_version]);
            }
            _ => panic!("unexpected value err `{err}`"),
        }
    } else {
        // Valid versioning
        let ch = ch.process_challenge_req(&req).unwrap();
        assert_eq!(ch.supported_api_versions, vec![backend_version]);
        assert_eq!(ch.backend_api_version, backend_version);
        assert_eq!(ch.client_api_version, client_version);
    }
}

#[rstest]
#[case((
    vec![ApiVersion { version: 2, revision: 22 }, ApiVersion { version: 3, revision: 33 }],
    vec![ApiVersion { version: 0, revision: 000 }, ApiVersion { version: 1, revision: 111 }],
    None,
    None,
))]
#[case((
    vec![ApiVersion { version: 2, revision: 22 }, ApiVersion { version: 3, revision: 33 }],
    vec![ApiVersion { version: 1, revision: 111 }, ApiVersion { version: 2, revision: 222 }],
    Some(ApiVersion { version: 2, revision: 22 }),
    Some(ApiVersion { version: 2, revision: 222 }),
))]
#[case((
    vec![ApiVersion { version: 2, revision: 22 }, ApiVersion { version: 3, revision: 33 }],
    vec![ApiVersion { version: 2, revision: 222 }, ApiVersion { version: 3, revision: 333 }],
    Some(ApiVersion { version: 3, revision: 33 }),
    Some(ApiVersion { version: 3, revision: 333 }),
))]
#[case((
    vec![ApiVersion { version: 2, revision: 22 }, ApiVersion { version: 3, revision: 33 }],
    vec![ApiVersion { version: 3, revision: 333 }, ApiVersion { version: 4, revision: 444 }],
    Some(ApiVersion { version: 3, revision: 33 }),
    Some(ApiVersion { version: 3, revision: 333 }),
))]
#[case((
    vec![ApiVersion { version: 2, revision: 22 }, ApiVersion { version: 3, revision: 33 }],
    vec![ApiVersion { version: 4, revision: 444 }, ApiVersion { version: 5, revision: 555 }],
    None,
    None,
))]
#[case((
    vec![ApiVersion { version: 2, revision: 22 }, ApiVersion { version: 4, revision: 44 }],
    vec![ApiVersion { version: 1, revision: 111 }, ApiVersion { version: 2, revision: 222 }],
    Some(ApiVersion { version: 2, revision: 22 }),
    Some(ApiVersion { version: 2, revision: 222 }),
))]
#[case((
    vec![ApiVersion { version: 2, revision: 22 }, ApiVersion { version: 4, revision: 44 }],
    vec![ApiVersion { version: 1, revision: 111 }, ApiVersion { version: 3, revision: 333 }],
    None,
    None,
))]
#[case((
    vec![ApiVersion { version: 2, revision: 22 }, ApiVersion { version: 4, revision: 44 }],
    vec![ApiVersion { version: 2, revision: 222 }, ApiVersion { version: 3, revision: 333 }],
    Some(ApiVersion { version: 2, revision: 22 }),
    Some(ApiVersion { version: 2, revision: 222 }),
))]
#[case((
    vec![ApiVersion { version: 2, revision: 22 }, ApiVersion { version: 4, revision: 44 }],
    vec![ApiVersion { version: 2, revision: 222 }, ApiVersion { version: 4, revision: 444 }],
    Some(ApiVersion { version: 4, revision: 44 }),
    Some(ApiVersion { version: 4, revision: 444 }),
))]
#[case((
    vec![ApiVersion { version: 2, revision: 22 }, ApiVersion { version: 4, revision: 44 }],
    vec![ApiVersion { version: 3, revision: 333 }, ApiVersion { version: 4, revision: 444 }],
    Some(ApiVersion { version: 4, revision: 44 }),
    Some(ApiVersion { version: 4, revision: 444 }),
))]
#[case((
    vec![ApiVersion { version: 2, revision: 22 }, ApiVersion { version: 4, revision: 44 }],
    vec![ApiVersion { version: 3, revision: 333 }, ApiVersion { version: 5, revision: 555 }],
    None,
    None,
))]
#[case((
    vec![ApiVersion { version: 2, revision: 22 }, ApiVersion { version: 4, revision: 44 }],
    vec![ApiVersion { version: 4, revision: 444 }, ApiVersion { version: 5, revision: 555 }],
    Some(ApiVersion { version: 4, revision: 44 }),
    Some(ApiVersion { version: 4, revision: 444 }),
))]
#[case((
    vec![ApiVersion { version: 2, revision: 22 }, ApiVersion { version: 4, revision: 44 }],
    vec![ApiVersion { version: 4, revision: 444 }, ApiVersion { version: 6, revision: 666 }],
    Some(ApiVersion { version: 4, revision: 44 }),
    Some(ApiVersion { version: 4, revision: 444 }),
))]
#[case((
    vec![ApiVersion { version: 2, revision: 22 }, ApiVersion { version: 4, revision: 44 }],
    vec![ApiVersion { version: 5, revision: 555 }, ApiVersion { version: 6, revision: 666 }],
    None,
    None,
))]
fn test_process_challenge_req_good_multiple_api_version(
    alice: &Device,
    timestamp: DateTime,
    #[case] input: (
        Vec<ApiVersion>,
        Vec<ApiVersion>,
        Option<ApiVersion>,
        Option<ApiVersion>,
    ),
) {
    let (_client_versions, _backend_versions, expected_client_version, expected_backend_version) =
        input;
    let t1 = timestamp;
    let t2 = t1.add_us(1);

    let req = Handshake::Challenge {
        challenge: hex!("58f7ec2bb24b81a57feee1bad250726a2f7588a3cdd0617a206687adf8fb1274f34b0aebf9fd27a5d29f56dce902ddcd"),
        supported_api_versions: _backend_versions.clone(),
        backend_timestamp: Some(t1),
        ballpark_client_early_offset: Some(BALLPARK_CLIENT_EARLY_OFFSET),
        ballpark_client_late_offset: Some(BALLPARK_CLIENT_LATE_OFFSET),
    }
    .dump()
    .unwrap();

    let mut ch = AuthenticatedClientHandshakeStalled::new(
        alice.organization_id().clone(),
        alice.device_id.clone(),
        alice.signing_key.clone(),
        alice.root_verify_key().clone(),
        t2,
    );

    ch.supported_api_versions = _client_versions.clone();

    if expected_client_version.is_none() {
        // Invalid versioning
        let err = ch.process_challenge_req(&req).unwrap_err();
        match err {
            HandshakeError::APIVersion {
                client_versions,
                backend_versions,
            } => {
                assert_eq!(client_versions, _client_versions);
                assert_eq!(backend_versions, _backend_versions);
            }
            _ => panic!("unexpected value err `{err}`"),
        }
    } else {
        // Valid versioning
        let ch = ch.process_challenge_req(&req).unwrap();
        assert_eq!(ch.supported_api_versions, _backend_versions);
        assert_eq!(Some(ch.backend_api_version), expected_backend_version);
        assert_eq!(Some(ch.client_api_version), expected_client_version);
    }
}

// 3) Server process answer

#[rstest]
#[case(json!({}))]
#[case(json!({
    "handshake": "answer",
    "type": "dummy",  // Invalid type
}))]
// Authenticated answer
#[case(json!({
    "handshake": "answer",
    "type": "AUTHENTICATED",
    "organization_id": "<good>",
    "device_id": "<good>",
    // Missing rvk
    "answer": b"good answer",
}))]
#[case(json!({
    "handshake": "answer",
    "type": "AUTHENTICATED",
    "organization_id": "<good>",
    // Missing device_id
    "rvk": "<good>",
    "answer": b"good answer",
}))]
#[case(json!({
    "handshake": "answer",
    "type": "AUTHENTICATED",
    "organization_id": "<good>",
    "device_id": "<good>",
    "rvk": "<good>",
    // Missing answer
}))]
#[case(json!({
    "handshake": "answer",
    "type": "AUTHENTICATED",
    "organization_id": "<good>",
    "device_id": "<good>",
    "rvk": "<good>",
    "answer": 42,  // Bad type
}))]
#[case(json!({
    "handshake": "answer",
    "type": "AUTHENTICATED",
    "organization_id": "<good>",
    "device_id": "dummy",  // Invalid DeviceID
    "rvk": "<good>",
    "answer": b"good answer",
}))]
#[case(json!({
    "handshake": "answer",
    "type": "AUTHENTICATED",
    "organization_id": "<good>",
    "device_id": "<good>",
    "rvk": b"dummy",  // Invalid VerifyKey
    "answer": b"good answer",
}))]
// Invited answer
#[case(json!({
    "handshake": "answer",
    "type": "INVITED",
    "invitation_type": "USER",
    "organization_id": "d@mmy",  // Invalid OrganizationID
    "token": "<good>",
}))]
#[case(json!({
    "handshake": "answer",
    "type": "INVITED",
    "invitation_type": "dummy",  // Invalid invitation_type
    "organization_id": "<good>",
    "token": "<good>",
}))]
#[case(json!({
    "handshake": "answer",
    "type": "INVITED",
    "invitation_type": "USER",
    "organization_id": "<good>",
    "token": "abc123",  // Invalid token type
}))]
fn test_process_answer_req_bad_format(alice: &Device, timestamp: DateTime, #[case] mut req: Value) {
    for (key, good_value) in [
        ("organization_id", json!(alice.organization_id())),
        ("device_id", json!(alice.device_id)),
        ("rvk", json!(alice.root_verify_key())),
        ("token", json!(&InvitationToken::default())),
    ] {
        if let Some("<good>") = req.get(key).and_then(|v| v.as_str()) {
            req[key] = good_value
        }
    }
    req["client_api_version"] = json!(API_V2_VERSION);
    let req = &rmp_serde::to_vec_named(&req).unwrap();
    let err = ServerHandshakeStalled::default()
        .build_challenge_req(timestamp)
        .unwrap()
        .process_answer_req(req)
        .unwrap_err();

    assert!(matches!(err, HandshakeError::InvalidMessage(_)));
}

// 4) Server build result

#[rstest]
fn test_build_result_req_bad_key(alice: &Device, bob: &Device, timestamp: DateTime) {
    let sh = ServerHandshakeStalled::default()
        .build_challenge_req(timestamp)
        .unwrap();

    let answer = Handshake::Answer(Answer::Authenticated {
        client_api_version: API_V2_VERSION,
        organization_id: alice.organization_id().clone(),
        device_id: alice.device_id.clone(),
        rvk: Box::new(alice.root_verify_key().clone()),
        answer: alice.signing_key.sign(
            &SignedAnswer {
                answer: sh.challenge,
            }
            .dump()
            .unwrap(),
        ),
    });

    let err = sh
        .process_answer_req(&answer.dump().unwrap())
        .unwrap()
        .build_result_req(Some(bob.verify_key()))
        .unwrap_err();

    assert!(matches!(err, HandshakeError::FailedChallenge));
}

#[rstest]
fn test_build_result_req_bad_challenge(alice: &Device, timestamp: DateTime) {
    let sh = ServerHandshakeStalled::default()
        .build_challenge_req(timestamp)
        .unwrap();

    let mut challenge = sh.challenge;
    challenge[0] = 0;
    challenge[1] = 0;

    let answer = Handshake::Answer(Answer::Authenticated {
        client_api_version: API_V2_VERSION,
        organization_id: alice.organization_id().clone(),
        device_id: alice.device_id.clone(),
        rvk: Box::new(alice.root_verify_key().clone()),
        answer: alice
            .signing_key
            .sign(&SignedAnswer { answer: challenge }.dump().unwrap()),
    });

    let err = sh
        .process_answer_req(&answer.dump().unwrap())
        .unwrap()
        .build_result_req(Some(alice.verify_key()))
        .unwrap_err();

    assert!(matches!(err, HandshakeError::FailedChallenge));
}

#[rstest]
#[case(
    Box::new(|sh: ServerHandshakeAnswer| {
        (sh.build_bad_protocol_result_req(None).unwrap(), HandshakeResult::BadProtocol)
    })
)]
#[case(
    Box::new(|sh: ServerHandshakeAnswer| {
        (sh.build_bad_identity_result_req(None).unwrap(), HandshakeResult::BadIdentity)
    })
)]
#[case(
    Box::new(|sh: ServerHandshakeAnswer| {
        (sh.build_organization_expired_result_req(None).unwrap(), HandshakeResult::OrganizationExpired)
    })
)]
#[case(
    Box::new(|sh: ServerHandshakeAnswer| {
        (sh.build_rvk_mismatch_result_req(None).unwrap(), HandshakeResult::RvkMismatch)
    })
)]
#[case(
    Box::new(|sh: ServerHandshakeAnswer| {
        (sh.build_revoked_device_result_req(None).unwrap(), HandshakeResult::RevokedDevice)
    })
)]
#[case(
    Box::new(|sh: ServerHandshakeAnswer| {
        (sh.build_bad_administration_token_result_req(None).unwrap(), HandshakeResult::BadAdminToken)
    })
)]
fn test_build_bad_outcomes(
    alice: &Device,
    timestamp: DateTime,
    #[case] generate_method_and_expected: Box<
        dyn FnOnce(ServerHandshakeAnswer) -> (ServerHandshakeResult, HandshakeResult),
    >,
) {
    let sh = ServerHandshakeStalled::default()
        .build_challenge_req(timestamp)
        .unwrap();

    let answer = Handshake::Answer(Answer::Authenticated {
        client_api_version: API_V2_VERSION,
        organization_id: alice.organization_id().clone(),
        device_id: alice.device_id.clone(),
        rvk: Box::new(alice.root_verify_key().clone()),
        answer: alice.signing_key.sign(
            &SignedAnswer {
                answer: sh.challenge,
            }
            .dump()
            .unwrap(),
        ),
    });
    let sh = sh.process_answer_req(&answer.dump().unwrap()).unwrap();
    let (sh, expected) = generate_method_and_expected(sh);
    if let Handshake::Result { result, .. } = Handshake::load(&sh.raw).unwrap() {
        assert_eq!(result, expected);
    } else {
        panic!("unexpected value for `Handshake::load`");
    }
}

// 5) Client process result

#[rstest]
#[case(json!({}))]
#[case(json!({"handshake": "foo", "result": "ok"}))]
#[case(json!({"result": "ok"}))]
#[case(json!({"handshake": "result", "result": "error"}))]
fn test_process_result_req_bad_format(timestamp: DateTime, #[case] req: Value) {
    let req = &rmp_serde::to_vec_named(&req).unwrap();

    let err = InvitedClientHandshakeStalled::new(
        OrganizationID::default(),
        InvitationType::User,
        InvitationToken::default(),
        timestamp,
    )
    .process_result_req(req)
    .unwrap_err();

    assert!(matches!(err, HandshakeError::InvalidMessage(_)));
}

#[rstest]
#[case(("bad_identity", HandshakeError::BadIdentity))]
#[case(("organization_expired", HandshakeError::OrganizationExpired))]
#[case(("rvk_mismatch", HandshakeError::RVKMismatch))]
#[case(("revoked_device", HandshakeError::RevokedDevice))]
#[case(("bad_admin_token", HandshakeError::BadAdministrationToken))]
#[case(("dummy", HandshakeError::InvalidMessage("Deserialization failed")))]
fn test_process_result_req_bad_outcome(
    timestamp: DateTime,
    #[case] result_expected: (&str, HandshakeError),
) {
    let (result, expected) = result_expected;

    let req = &rmp_serde::to_vec_named(&json!({
        "handshake": "result", "result": result
    }))
    .unwrap();

    let err = InvitedClientHandshakeStalled::new(
        OrganizationID::default(),
        InvitationType::User,
        InvitationToken::default(),
        timestamp,
    )
    .process_result_req(req)
    .unwrap_err();

    assert_eq!(err, expected)
}

// Generated from Python implementation (Parsec v2.15.0+dev)
// Content:
//   version: 2
//   revision: 15
#[rstest]
fn serde_api_version() {
    let expected = ApiVersion {
        version: 2,
        revision: 15,
    };
    let bytes = hex!("92020f");

    let loaded_version = ApiVersion::load(&bytes).unwrap();
    assert_eq!(loaded_version, expected);

    // Roundtrip test ...
    let bytes2 = loaded_version.dump().unwrap();
    let loaded_version2 = ApiVersion::load(&bytes2).unwrap();
    assert_eq!(loaded_version2, expected);
}

// TODO: test with revoked device
// TODO: test with user with all devices revoked

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

// wrap this in each test function
// macro that contains a closure
use super::authenticated_cmds;

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::prelude::*;

// Request

pub fn req() {
    for (raw, req) in [
        (
            // Generated from Parsec 3.7.1-a.0+dev
            // Content:
            //   cmd: 'async_enrollment_accept'
            //   enrollment_id: ext(2, 0x1d3353157d7d4e95ad2fdea7b3bd19c5)
            //   submitter_user_certificate: 0x3c7375626d69747465725f757365725f63657274696669636174653e
            //   submitter_device_certificate: 0x3c7375626d69747465725f6465766963655f63657274696669636174653e
            //   submitter_redacted_user_certificate: 0x3c7375626d69747465725f72656461637465645f757365725f63657274696669636174653e
            //   submitter_redacted_device_certificate: 0x3c7375626d69747465725f72656461637465645f6465766963655f63657274696669636174653e
            //   accept_payload:
            //     0x0028b52ffd0058a50600340c87a474797065bf6173796e635f656e726f6c6c6d656e745f616363
            //     6570745f7061796c6f6164a7757365725f6964d802a11cec001000a9646576696365de10ac6c6162
            //     656caf4d792064657631206d616368696e65ac68756d616e5f68616e646c6592b1616c6963654065
            //     78616d706c652e636f6db2416c69636579204d6346616365a770726f66696c65a541444d494eaf72
            //     6f6f745f7665726966795f6b6579c420be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68
            //     572c77a1e17d9bbd0500cf22e067c68292eda7d8b60128
            //   accept_payload_signature: {
            //     type: 'PKI',
            //     accepter_der_x509_certificate: 0x3c616363657074657220783530392063657274696669636174653e,
            //     algorithm: 'RSASSA-PSS-SHA256',
            //     intermediate_der_x509_certificates: [
            //       0x3c696e7465726d656469617465203120783530392063657274696669636174653e,
            //       0x3c696e7465726d656469617465203220783530392063657274696669636174653e,
            //     ],
            //     signature: 0x3c7369676e61747572653e,
            //   }
            hex!(
                "88a3636d64b76173796e635f656e726f6c6c6d656e745f616363657074ad656e726f6c"
                "6c6d656e745f6964d8021d3353157d7d4e95ad2fdea7b3bd19c5ba7375626d69747465"
                "725f757365725f6365727469666963617465c41c3c7375626d69747465725f75736572"
                "5f63657274696669636174653ebc7375626d69747465725f6465766963655f63657274"
                "69666963617465c41e3c7375626d69747465725f6465766963655f6365727469666963"
                "6174653ed9237375626d69747465725f72656461637465645f757365725f6365727469"
                "666963617465c4253c7375626d69747465725f72656461637465645f757365725f6365"
                "7274696669636174653ed9257375626d69747465725f72656461637465645f64657669"
                "63655f6365727469666963617465c4273c7375626d69747465725f7265646163746564"
                "5f6465766963655f63657274696669636174653eae6163636570745f7061796c6f6164"
                "c4de0028b52ffd0058a50600340c87a474797065bf6173796e635f656e726f6c6c6d65"
                "6e745f6163636570745f7061796c6f6164a7757365725f6964d802a11cec001000a964"
                "6576696365de10ac6c6162656caf4d792064657631206d616368696e65ac68756d616e"
                "5f68616e646c6592b1616c696365406578616d706c652e636f6db2416c69636579204d"
                "6346616365a770726f66696c65a541444d494eaf726f6f745f7665726966795f6b6579"
                "c420be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd05"
                "00cf22e067c68292eda7d8b60128b86163636570745f7061796c6f61645f7369676e61"
                "7475726585a474797065a3504b49bd61636365707465725f6465725f783530395f6365"
                "727469666963617465c41b3c6163636570746572207835303920636572746966696361"
                "74653ea9616c676f726974686db15253415353412d5053532d534841323536d922696e"
                "7465726d6564696174655f6465725f783530395f63657274696669636174657392c421"
                "3c696e7465726d656469617465203120783530392063657274696669636174653ec421"
                "3c696e7465726d656469617465203220783530392063657274696669636174653ea973"
                "69676e6174757265c40b3c7369676e61747572653e"
            ).as_ref(),

            authenticated_cmds::async_enrollment_accept::Req {
                enrollment_id: AsyncEnrollmentID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
                submitter_user_certificate: b"<submitter_user_certificate>".as_ref().into(),
                submitter_device_certificate: b"<submitter_device_certificate>".as_ref().into(),
                submitter_redacted_user_certificate: b"<submitter_redacted_user_certificate>".as_ref().into(),
                submitter_redacted_device_certificate: b"<submitter_redacted_device_certificate>".as_ref().into(),
                accept_payload: hex!(
                    "0028b52ffd0058a50600340c87a474797065bf6173796e635f656e726f6c6c6d656e74"
                    "5f6163636570745f7061796c6f6164a7757365725f6964d802a11cec001000a9646576"
                    "696365de10ac6c6162656caf4d792064657631206d616368696e65ac68756d616e5f68"
                    "616e646c6592b1616c696365406578616d706c652e636f6db2416c69636579204d6346"
                    "616365a770726f66696c65a541444d494eaf726f6f745f7665726966795f6b6579c420"
                    "be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd0500cf"
                    "22e067c68292eda7d8b60128"
                ).as_ref().into(),
                accept_payload_signature: authenticated_cmds::async_enrollment_accept::AcceptPayloadSignature::PKI {
                    signature: hex!("3c7369676e61747572653e").as_ref().into(),
                    algorithm: PkiSignatureAlgorithm::RsassaPssSha256,
                    accepter_der_x509_certificate: b"<accepter x509 certificate>".as_ref().into(),
                    intermediate_der_x509_certificates: [
                        b"<intermediate 1 x509 certificate>".as_ref().into(),
                        b"<intermediate 2 x509 certificate>".as_ref().into()
                    ].to_vec(),
                },
            }
        ),
        (
            // Generated from Parsec 3.7.1-a.0+dev
            // Content:
            //   cmd: 'async_enrollment_accept'
            //   enrollment_id: ext(2, 0x1d3353157d7d4e95ad2fdea7b3bd19c5)
            //   submitter_user_certificate: 0x3c7375626d69747465725f757365725f63657274696669636174653e
            //   submitter_device_certificate: 0x3c7375626d69747465725f6465766963655f63657274696669636174653e
            //   submitter_redacted_user_certificate: 0x3c7375626d69747465725f72656461637465645f757365725f63657274696669636174653e
            //   submitter_redacted_device_certificate: 0x3c7375626d69747465725f72656461637465645f6465766963655f63657274696669636174653e
            //   accept_payload:
            //     0x0028b52ffd0058a50600340c87a474797065bf6173796e635f656e726f6c6c6d656e745f616363
            //     6570745f7061796c6f6164a7757365725f6964d802a11cec001000a9646576696365de10ac6c6162
            //     656caf4d792064657631206d616368696e65ac68756d616e5f68616e646c6592b1616c6963654065
            //     78616d706c652e636f6db2416c69636579204d6346616365a770726f66696c65a541444d494eaf72
            //     6f6f745f7665726966795f6b6579c420be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68
            //     572c77a1e17d9bbd0500cf22e067c68292eda7d8b60128
            //   accept_payload_signature: {
            //     type: 'OPEN_BAO',
            //     accepter_openbao_entity_id: 'e7d07e2a-b05d-ec59-dcf2-4403efdbf1b7',
            //     signature: 'vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==',
            //   }
            hex!(
                "88a3636d64b76173796e635f656e726f6c6c6d656e745f616363657074ad656e726f6c"
                "6c6d656e745f6964d8021d3353157d7d4e95ad2fdea7b3bd19c5ba7375626d69747465"
                "725f757365725f6365727469666963617465c41c3c7375626d69747465725f75736572"
                "5f63657274696669636174653ebc7375626d69747465725f6465766963655f63657274"
                "69666963617465c41e3c7375626d69747465725f6465766963655f6365727469666963"
                "6174653ed9237375626d69747465725f72656461637465645f757365725f6365727469"
                "666963617465c4253c7375626d69747465725f72656461637465645f757365725f6365"
                "7274696669636174653ed9257375626d69747465725f72656461637465645f64657669"
                "63655f6365727469666963617465c4273c7375626d69747465725f7265646163746564"
                "5f6465766963655f63657274696669636174653eae6163636570745f7061796c6f6164"
                "c4de0028b52ffd0058a50600340c87a474797065bf6173796e635f656e726f6c6c6d65"
                "6e745f6163636570745f7061796c6f6164a7757365725f6964d802a11cec001000a964"
                "6576696365de10ac6c6162656caf4d792064657631206d616368696e65ac68756d616e"
                "5f68616e646c6592b1616c696365406578616d706c652e636f6db2416c69636579204d"
                "6346616365a770726f66696c65a541444d494eaf726f6f745f7665726966795f6b6579"
                "c420be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd05"
                "00cf22e067c68292eda7d8b60128b86163636570745f7061796c6f61645f7369676e61"
                "7475726583a474797065a84f50454e5f42414fba61636365707465725f6f70656e6261"
                "6f5f656e746974795f6964d92465376430376532612d623035642d656335392d646366"
                "322d343430336566646266316237a97369676e6174757265d9617661756c743a76313a"
                "43346a525a782b796d4c6f753236744e3851324b44793436644134375737782f4d4836"
                "6e75455a5671647a2b483052766f614662515541486365424b68422b516f7732715841"
                "586952464146574b47505a55393343513d3d"
            ).as_ref(),

            authenticated_cmds::async_enrollment_accept::Req {
                enrollment_id: AsyncEnrollmentID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
                submitter_user_certificate: b"<submitter_user_certificate>".as_ref().into(),
                submitter_device_certificate: b"<submitter_device_certificate>".as_ref().into(),
                submitter_redacted_user_certificate: b"<submitter_redacted_user_certificate>".as_ref().into(),
                submitter_redacted_device_certificate: b"<submitter_redacted_device_certificate>".as_ref().into(),
                accept_payload: hex!(
                    "0028b52ffd0058a50600340c87a474797065bf6173796e635f656e726f6c6c6d656e74"
                    "5f6163636570745f7061796c6f6164a7757365725f6964d802a11cec001000a9646576"
                    "696365de10ac6c6162656caf4d792064657631206d616368696e65ac68756d616e5f68"
                    "616e646c6592b1616c696365406578616d706c652e636f6db2416c69636579204d6346"
                    "616365a770726f66696c65a541444d494eaf726f6f745f7665726966795f6b6579c420"
                    "be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd0500cf"
                    "22e067c68292eda7d8b60128"
                ).as_ref().into(),
                accept_payload_signature: authenticated_cmds::async_enrollment_accept::AcceptPayloadSignature::OpenBao {
                    signature: "vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==".to_owned(),
                    accepter_openbao_entity_id: "e7d07e2a-b05d-ec59-dcf2-4403efdbf1b7".to_owned(),
                },
            }
        ),
    ] {
        println!("***expected: {:?}", req.dump().unwrap());

        let expected = authenticated_cmds::AnyCmdReq::AsyncEnrollmentAccept(req.clone());

        let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = req.dump().unwrap();

        let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'ok'
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();
    let expected = authenticated_cmds::async_enrollment_accept::Rep::Ok;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'author_not_allowed'
    let raw: &[u8] = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564").as_ref();
    let expected = authenticated_cmds::async_enrollment_accept::Rep::AuthorNotAllowed;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_invalid_certificate() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'invalid_certificate'
    let raw: &[u8] = hex!("81a6737461747573b3696e76616c69645f6365727469666963617465").as_ref();
    let expected = authenticated_cmds::async_enrollment_accept::Rep::InvalidCertificate;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_invalid_accept_payload() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'invalid_accept_payload'
    let raw: &[u8] = hex!(
        "81a6737461747573b6696e76616c69645f6163636570745f7061796c6f6164"
    ).as_ref();
    let expected = authenticated_cmds::async_enrollment_accept::Rep::InvalidAcceptPayload;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_submit_and_accept_identity_systems_mismatch() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'submit_and_accept_identity_systems_mismatch'
    let raw: &[u8] = hex!(
        "81a6737461747573d92b7375626d69745f616e645f6163636570745f6964656e746974"
        "795f73797374656d735f6d69736d61746368"
    ).as_ref();
    let expected = authenticated_cmds::async_enrollment_accept::Rep::SubmitAndAcceptIdentitySystemsMismatch;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_invalid_accept_payload_signature() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'invalid_accept_payload_signature'
    let raw: &[u8] = hex!(
        "81a6737461747573d920696e76616c69645f6163636570745f7061796c6f61645f7369"
        "676e6174757265"
    ).as_ref();
    let expected = authenticated_cmds::async_enrollment_accept::Rep::InvalidAcceptPayloadSignature;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_invalid_der_x509_certificate() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'invalid_der_x509_certificate'
    let raw: &[u8] = hex!(
        "81a6737461747573bc696e76616c69645f6465725f783530395f636572746966696361"
        "7465"
    ).as_ref();
    let expected = authenticated_cmds::async_enrollment_accept::Rep::InvalidDerX509Certificate;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_invalid_x509_trustchain() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'invalid_x509_trustchain'
    let raw: &[u8] = hex!(
        "81a6737461747573b7696e76616c69645f783530395f7472757374636861696e"
    ).as_ref();
    let expected = authenticated_cmds::async_enrollment_accept::Rep::InvalidX509Trustchain;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_enrollment_not_found() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'enrollment_not_found'
    let raw: &[u8] = hex!("81a6737461747573b4656e726f6c6c6d656e745f6e6f745f666f756e64").as_ref();
    let expected = authenticated_cmds::async_enrollment_accept::Rep::EnrollmentNotFound;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_enrollment_no_longer_available() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'enrollment_no_longer_available'
    let raw: &[u8] = hex!(
        "81a6737461747573be656e726f6c6c6d656e745f6e6f5f6c6f6e6765725f617661696c"
        "61626c65"
    )
    .as_ref();
    let expected = authenticated_cmds::async_enrollment_accept::Rep::EnrollmentNoLongerAvailable;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_active_users_limit_reached() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'active_users_limit_reached'
    let raw: &[u8] =
        hex!("81a6737461747573ba6163746976655f75736572735f6c696d69745f72656163686564").as_ref();
    let expected = authenticated_cmds::async_enrollment_accept::Rep::ActiveUsersLimitReached;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_user_already_exists() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'user_already_exists'
    let raw: &[u8] = hex!("81a6737461747573b3757365725f616c72656164795f657869737473").as_ref();
    let expected = authenticated_cmds::async_enrollment_accept::Rep::UserAlreadyExists;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_human_handle_already_taken() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'human_handle_already_taken'
    let raw: &[u8] =
        hex!("81a6737461747573ba68756d616e5f68616e646c655f616c72656164795f74616b656e").as_ref();
    let expected = authenticated_cmds::async_enrollment_accept::Rep::HumanHandleAlreadyTaken;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_timestamp_out_of_ballpark() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'timestamp_out_of_ballpark'
    //   ballpark_client_early_offset: 300.0
    //   ballpark_client_late_offset: 320.0
    //   client_timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    //   server_timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    let raw: &[u8] = hex!(
        "85a6737461747573b974696d657374616d705f6f75745f6f665f62616c6c7061726bbc"
        "62616c6c7061726b5f636c69656e745f6561726c795f6f6666736574cb4072c0000000"
        "0000bb62616c6c7061726b5f636c69656e745f6c6174655f6f6666736574cb40740000"
        "00000000b0636c69656e745f74696d657374616d70d70100035d162fa2e400b0736572"
        "7665725f74696d657374616d70d70100035d162fa2e400"
    )
    .as_ref();
    let expected = authenticated_cmds::async_enrollment_accept::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 300.0,
        ballpark_client_late_offset: 320.0,
        server_timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-01-02T01:00:00Z".parse().unwrap(),
    };
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_require_greater_timestamp() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'require_greater_timestamp'
    //   strictly_greater_than: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    let raw: &[u8] = hex!(
        "82a6737461747573b9726571756972655f677265617465725f74696d657374616d70b5"
        "7374726963746c795f677265617465725f7468616ed70100035d162fa2e400"
    )
    .as_ref();
    let expected = authenticated_cmds::async_enrollment_accept::Rep::RequireGreaterTimestamp {
        strictly_greater_than: "2000-1-2T01:00:00Z".parse().unwrap(),
    };
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::async_enrollment_accept::Rep) {
    let data = authenticated_cmds::async_enrollment_accept::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::async_enrollment_accept::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

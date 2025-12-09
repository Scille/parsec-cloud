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
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   cmd: 'async_enrollment_list'
    let raw: &[u8] = hex!("81a3636d64b56173796e635f656e726f6c6c6d656e745f6c697374").as_ref();

    let req = authenticated_cmds::async_enrollment_list::Req;
    println!("***expected: {:?}", req.dump().unwrap());

    let expected = authenticated_cmds::AnyCmdReq::AsyncEnrollmentList(req.clone());

    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = req.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'ok'
    //   enrollments: [
    //     {
    //       enrollment_id: ext(2, 0x7c021dd1e2cd4c328f00dd6e64b89279),
    //       submit_payload:
    //         0x0028b52ffd0058e50600e40c85a474797065bf6173796e635f656e726f6c6c6d656e745f737562
    //         6d69745f7061796c6f6164aa7665726966795f6b6579c420845415cd821748005054dba8d456ac18
    //         ad3e71acdf980e19d6a925191362c9f9aa7075626c6963e1b20b860a78ef778d0c776121c7027cd9
    //         0ce04b4d2f1a291a48d911f145724db67265717565737465645f6465766963655f6c6162656caf4d
    //         792064657631206d616368696e6568756d616e5f68616e646c6592b1616c696365406578616d706c
    //         652e636f6db2416c69636579204d634661636503006111e550ca8406077506,
    //       submit_payload_signature: {
    //         type: 'PKI',
    //         algorithm: 'RSASSA-PSS-SHA256',
    //         intermediate_der_x509_certificates: [
    //           0x3c696e7465726d656469617465203120783530392063657274696669636174653e,
    //           0x3c696e7465726d656469617465203220783530392063657274696669636174653e,
    //         ],
    //         signature: 0x3c7369676e61747572653e,
    //         submitter_der_x509_certificate: 0x3c7375626d697474657220783530392063657274696669636174653e,
    //       },
    //       submitted_on: ext(1, 946688461000000) i.e. 2000-01-01T02:01:01Z,
    //     },
    //     {
    //       enrollment_id: ext(2, 0xea778fb3bca142d2bb116ea5f1b51667),
    //       submit_payload:
    //         0x0028b52ffd0058e50600e40c85a474797065bf6173796e635f656e726f6c6c6d656e745f737562
    //         6d69745f7061796c6f6164aa7665726966795f6b6579c420845415cd821748005054dba8d456ac18
    //         ad3e71acdf980e19d6a925191362c9f9aa7075626c6963e1b20b860a78ef778d0c776121c7027cd9
    //         0ce04b4d2f1a291a48d911f145724db67265717565737465645f6465766963655f6c6162656caf4d
    //         792064657631206d616368696e6568756d616e5f68616e646c6592b1616c696365406578616d706c
    //         652e636f6db2416c69636579204d634661636503006111e550ca8406077506,
    //       submit_payload_signature: {
    //         type: 'OPEN_BAO',
    //         signature: 'vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==',
    //         submitter_openbao_entity_id: 'e7d07e2a-b05d-ec59-dcf2-4403efdbf1b7',
    //       },
    //       submitted_on: ext(1, 949456922000000) i.e. 2000-02-02T03:02:02Z,
    //     },
    //   ]
    let raw: &[u8] = hex!(
        "82a6737461747573a26f6bab656e726f6c6c6d656e74739284ad656e726f6c6c6d656e"
        "745f6964d8027c021dd1e2cd4c328f00dd6e64b89279ae7375626d69745f7061796c6f"
        "6164c4e60028b52ffd0058e50600e40c85a474797065bf6173796e635f656e726f6c6c"
        "6d656e745f7375626d69745f7061796c6f6164aa7665726966795f6b6579c420845415"
        "cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9aa7075626c69"
        "63e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724db672"
        "65717565737465645f6465766963655f6c6162656caf4d792064657631206d61636869"
        "6e6568756d616e5f68616e646c6592b1616c696365406578616d706c652e636f6db241"
        "6c69636579204d634661636503006111e550ca8406077506b87375626d69745f706179"
        "6c6f61645f7369676e617475726585a474797065a3504b49a9616c676f726974686db1"
        "5253415353412d5053532d534841323536d922696e7465726d6564696174655f646572"
        "5f783530395f63657274696669636174657392c4213c696e7465726d65646961746520"
        "3120783530392063657274696669636174653ec4213c696e7465726d65646961746520"
        "3220783530392063657274696669636174653ea97369676e6174757265c40b3c736967"
        "6e61747572653ebe7375626d69747465725f6465725f783530395f6365727469666963"
        "617465c41c3c7375626d697474657220783530392063657274696669636174653eac73"
        "75626d69747465645f6f6ed70100035d02156e4d4084ad656e726f6c6c6d656e745f69"
        "64d802ea778fb3bca142d2bb116ea5f1b51667ae7375626d69745f7061796c6f6164c4"
        "e60028b52ffd0058e50600e40c85a474797065bf6173796e635f656e726f6c6c6d656e"
        "745f7375626d69745f7061796c6f6164aa7665726966795f6b6579c420845415cd8217"
        "48005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9aa7075626c6963e1b2"
        "0b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724db672657175"
        "65737465645f6465766963655f6c6162656caf4d792064657631206d616368696e6568"
        "756d616e5f68616e646c6592b1616c696365406578616d706c652e636f6db2416c6963"
        "6579204d634661636503006111e550ca8406077506b87375626d69745f7061796c6f61"
        "645f7369676e617475726583a474797065a84f50454e5f42414fa97369676e61747572"
        "65d9617661756c743a76313a43346a525a782b796d4c6f753236744e3851324b447934"
        "36644134375737782f4d48366e75455a5671647a2b483052766f614662515541486365"
        "424b68422b516f7732715841586952464146574b47505a55393343513d3dd921737562"
        "6d69747465725f6f70656e62616f5f656e746974795f616c6961735f6964d924653764"
        "30376532612d623035642d656335392d646366322d343430336566646266316237ac73"
        "75626d69747465645f6f6ed70100035f86aa90ba80"
    ).as_ref();

    let expected = authenticated_cmds::async_enrollment_list::Rep::Ok {
        enrollments: vec![
            authenticated_cmds::async_enrollment_list::Enrollment {
                enrollment_id: AsyncEnrollmentID::from_hex("7c021dd1e2cd4c328f00dd6e64b89279").unwrap(),
                submitted_on: "2000-01-01T01:01:01Z".parse().unwrap(),
                submit_payload: hex!(
                    "0028b52ffd0058e50600e40c85a474797065bf6173796e635f656e726f6c6c6d656e74"
                    "5f7375626d69745f7061796c6f6164aa7665726966795f6b6579c420845415cd821748"
                    "005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9aa7075626c6963e1b20b"
                    "860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724db67265717565"
                    "737465645f6465766963655f6c6162656caf4d792064657631206d616368696e656875"
                    "6d616e5f68616e646c6592b1616c696365406578616d706c652e636f6db2416c696365"
                    "79204d634661636503006111e550ca8406077506"
                ).as_ref().into(),
                submit_payload_signature: authenticated_cmds::async_enrollment_list::SubmitPayloadSignature::PKI {
                    signature: hex!("3c7369676e61747572653e").as_ref().into(),
                    algorithm: PkiSignatureAlgorithm::RsassaPssSha256,
                    submitter_der_x509_certificate: b"<submitter x509 certificate>".as_ref().into(),
                    intermediate_der_x509_certificates: [
                        b"<intermediate 1 x509 certificate>".as_ref().into(),
                        b"<intermediate 2 x509 certificate>".as_ref().into()
                    ].to_vec(),
                },
            },
            authenticated_cmds::async_enrollment_list::Enrollment {
                enrollment_id: AsyncEnrollmentID::from_hex("ea778fb3bca142d2bb116ea5f1b51667").unwrap(),
                submitted_on: "2000-02-02T02:02:02Z".parse().unwrap(),
                submit_payload: hex!(
                    "0028b52ffd0058e50600e40c85a474797065bf6173796e635f656e726f6c6c6d656e74"
                    "5f7375626d69745f7061796c6f6164aa7665726966795f6b6579c420845415cd821748"
                    "005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9aa7075626c6963e1b20b"
                    "860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724db67265717565"
                    "737465645f6465766963655f6c6162656caf4d792064657631206d616368696e656875"
                    "6d616e5f68616e646c6592b1616c696365406578616d706c652e636f6db2416c696365"
                    "79204d634661636503006111e550ca8406077506"
                ).as_ref().into(),
                submit_payload_signature: authenticated_cmds::async_enrollment_list::SubmitPayloadSignature::OpenBao {
                    signature: "vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==".to_owned(),
                    submitter_openbao_entity_id: "e7d07e2a-b05d-ec59-dcf2-4403efdbf1b7".to_owned(),
                },
            }
        ]
    };
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'author_not_allowed'
    let raw: &[u8] = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564").as_ref();
    let expected = authenticated_cmds::async_enrollment_list::Rep::AuthorNotAllowed;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::async_enrollment_list::Rep) {
    let data = authenticated_cmds::async_enrollment_list::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::async_enrollment_list::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

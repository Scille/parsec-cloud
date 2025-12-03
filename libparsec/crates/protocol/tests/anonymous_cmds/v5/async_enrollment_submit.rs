// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

// wrap this in each test function
// macro that contains a closure
use super::anonymous_cmds;

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::prelude::*;

// Request

pub fn req() {
    for (raw, req) in [
        (
            // Generated from Parsec 3.7.1-a.0+dev
            // Content:
            //   cmd: 'async_enrollment_submit'
            //   enrollment_id: ext(2, 0x7c021dd1e2cd4c328f00dd6e64b89279)
            //   force: True
            //   payload:
            //     0x0028b52ffd0058e50600e40c85a474797065bf6173796e635f656e726f6c6c6d656e745f737562
            //     6d69745f7061796c6f6164aa7665726966795f6b6579c420845415cd821748005054dba8d456ac18
            //     ad3e71acdf980e19d6a925191362c9f9aa7075626c6963e1b20b860a78ef778d0c776121c7027cd9
            //     0ce04b4d2f1a291a48d911f145724db67265717565737465645f6465766963655f6c6162656caf4d
            //     792064657631206d616368696e6568756d616e5f68616e646c6592b1616c696365406578616d706c
            //     652e636f6db2416c69636579204d634661636503006111e550ca8406077506
            //   payload_signature: {
            //     type: 'PKI',
            //     algorithm: 'RSASSA-PSS-SHA256',
            //     intermediate_der_x509_certificates: [
            //       0x3c696e7465726d656469617465203120783530392063657274696669636174653e,
            //       0x3c696e7465726d656469617465203220783530392063657274696669636174653e,
            //     ],
            //     signature: 0x3c7369676e61747572653e,
            //     submitter_der_x509_certificate: 0x3c7375626d697474657220783530392063657274696669636174653e,
            //   }
            hex!(
                "85a3636d64b76173796e635f656e726f6c6c6d656e745f7375626d6974ad656e726f6c"
                "6c6d656e745f6964d8027c021dd1e2cd4c328f00dd6e64b89279a5666f726365c3a770"
                "61796c6f6164c4e60028b52ffd0058e50600e40c85a474797065bf6173796e635f656e"
                "726f6c6c6d656e745f7375626d69745f7061796c6f6164aa7665726966795f6b6579c4"
                "20845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9aa70"
                "75626c6963e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145"
                "724db67265717565737465645f6465766963655f6c6162656caf4d792064657631206d"
                "616368696e6568756d616e5f68616e646c6592b1616c696365406578616d706c652e63"
                "6f6db2416c69636579204d634661636503006111e550ca8406077506b17061796c6f61"
                "645f7369676e617475726585a474797065a3504b49a9616c676f726974686db1525341"
                "5353412d5053532d534841323536d922696e7465726d6564696174655f6465725f7835"
                "30395f63657274696669636174657392c4213c696e7465726d65646961746520312078"
                "3530392063657274696669636174653ec4213c696e7465726d65646961746520322078"
                "3530392063657274696669636174653ea97369676e6174757265c40b3c7369676e6174"
                "7572653ebe7375626d69747465725f6465725f783530395f6365727469666963617465"
                "c41c3c7375626d697474657220783530392063657274696669636174653e"
            ).as_ref(),

            anonymous_cmds::async_enrollment_submit::Req {
                enrollment_id: AsyncEnrollmentID::from_hex("7c021dd1e2cd4c328f00dd6e64b89279").unwrap(),
                force: true,
                payload: hex!(
                    "0028b52ffd0058e50600e40c85a474797065bf6173796e635f656e726f6c6c6d656e74"
                    "5f7375626d69745f7061796c6f6164aa7665726966795f6b6579c420845415cd821748"
                    "005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9aa7075626c6963e1b20b"
                    "860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724db67265717565"
                    "737465645f6465766963655f6c6162656caf4d792064657631206d616368696e656875"
                    "6d616e5f68616e646c6592b1616c696365406578616d706c652e636f6db2416c696365"
                    "79204d634661636503006111e550ca8406077506"
                ).as_ref().into(),
                payload_signature: anonymous_cmds::async_enrollment_submit::PayloadSignature::PKI {
                    signature: hex!("3c7369676e61747572653e").as_ref().into(),
                    algorithm: PkiSignatureAlgorithm::RsassaPssSha256,
                    submitter_der_x509_certificate: b"<submitter x509 certificate>".as_ref().into(),
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
            //   cmd: 'async_enrollment_submit'
            //   enrollment_id: ext(2, 0xea778fb3bca142d2bb116ea5f1b51667)
            //   force: True
            //   payload:
            //     0x0028b52ffd0058e50600e40c85a474797065bf6173796e635f656e726f6c6c6d656e745f737562
            //     6d69745f7061796c6f6164aa7665726966795f6b6579c420845415cd821748005054dba8d456ac18
            //     ad3e71acdf980e19d6a925191362c9f9aa7075626c6963e1b20b860a78ef778d0c776121c7027cd9
            //     0ce04b4d2f1a291a48d911f145724db67265717565737465645f6465766963655f6c6162656caf4d
            //     792064657631206d616368696e6568756d616e5f68616e646c6592b1616c696365406578616d706c
            //     652e636f6db2416c69636579204d634661636503006111e550ca8406077506
            //   payload_signature: {
            //     type: 'OPEN_BAO',
            //     signature: 'vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==',
            //     submitter_openbao_entity_alias_id: 'e7d07e2a-b05d-ec59-dcf2-4403efdbf1b7',
            //   }
            hex!(
                "85a3636d64b76173796e635f656e726f6c6c6d656e745f7375626d6974ad656e726f6c"
                "6c6d656e745f6964d802ea778fb3bca142d2bb116ea5f1b51667a5666f726365c3a770"
                "61796c6f6164c4e60028b52ffd0058e50600e40c85a474797065bf6173796e635f656e"
                "726f6c6c6d656e745f7375626d69745f7061796c6f6164aa7665726966795f6b6579c4"
                "20845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9aa70"
                "75626c6963e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145"
                "724db67265717565737465645f6465766963655f6c6162656caf4d792064657631206d"
                "616368696e6568756d616e5f68616e646c6592b1616c696365406578616d706c652e63"
                "6f6db2416c69636579204d634661636503006111e550ca8406077506b17061796c6f61"
                "645f7369676e617475726583a474797065a84f50454e5f42414fa97369676e61747572"
                "65d9617661756c743a76313a43346a525a782b796d4c6f753236744e3851324b447934"
                "36644134375737782f4d48366e75455a5671647a2b483052766f614662515541486365"
                "424b68422b516f7732715841586952464146574b47505a55393343513d3dd921737562"
                "6d69747465725f6f70656e62616f5f656e746974795f616c6961735f6964d924653764"
                "30376532612d623035642d656335392d646366322d343430336566646266316237"
            ).as_ref(),

            anonymous_cmds::async_enrollment_submit::Req {
                enrollment_id: AsyncEnrollmentID::from_hex("ea778fb3bca142d2bb116ea5f1b51667").unwrap(),
                force: true,
                payload: hex!(
                    "0028b52ffd0058e50600e40c85a474797065bf6173796e635f656e726f6c6c6d656e74"
                    "5f7375626d69745f7061796c6f6164aa7665726966795f6b6579c420845415cd821748"
                    "005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9aa7075626c6963e1b20b"
                    "860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724db67265717565"
                    "737465645f6465766963655f6c6162656caf4d792064657631206d616368696e656875"
                    "6d616e5f68616e646c6592b1616c696365406578616d706c652e636f6db2416c696365"
                    "79204d634661636503006111e550ca8406077506"
                ).as_ref().into(),
                payload_signature: anonymous_cmds::async_enrollment_submit::PayloadSignature::OpenBao {
                    signature: "vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==".to_owned(),
                    submitter_openbao_entity_alias_id: "e7d07e2a-b05d-ec59-dcf2-4403efdbf1b7".to_owned(),
                },
            }
        ),
    ] {
        println!("***expected: {:?}", req.dump().unwrap());

        let expected = anonymous_cmds::AnyCmdReq::AsyncEnrollmentSubmit(req.clone());

        let data = anonymous_cmds::AnyCmdReq::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = req.dump().unwrap();

        let data2 = anonymous_cmds::AnyCmdReq::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'ok'
    //   submitted_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    let raw: &[u8] =
        hex!("82a6737461747573a26f6bac7375626d69747465645f6f6ed70100035d013b37e000").as_ref();
    let expected = anonymous_cmds::async_enrollment_submit::Rep::Ok {
        submitted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
    };
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_email_already_submitted() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'email_already_submitted'
    //   submitted_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    let raw: &[u8] = hex!(
        "82a6737461747573b7656d61696c5f616c72656164795f7375626d6974746564ac7375"
        "626d69747465645f6f6ed70100035d013b37e000"
    ).as_ref();
    let expected = anonymous_cmds::async_enrollment_submit::Rep::EmailAlreadySubmitted {
        submitted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
    };
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_email_already_enrolled() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'email_already_enrolled'
    let raw: &[u8] = hex!(
        "81a6737461747573b6656d61696c5f616c72656164795f656e726f6c6c6564"
    ).as_ref();
    let expected = anonymous_cmds::async_enrollment_submit::Rep::EmailAlreadyEnrolled;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_id_already_used() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'id_already_used'
    let raw: &[u8] = hex!("81a6737461747573af69645f616c72656164795f75736564").as_ref();
    let expected = anonymous_cmds::async_enrollment_submit::Rep::IdAlreadyUsed;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_invalid_payload() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'invalid_payload'
    let raw: &[u8] = hex!("81a6737461747573af696e76616c69645f7061796c6f6164").as_ref();
    let expected = anonymous_cmds::async_enrollment_submit::Rep::InvalidPayload;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_invalid_payload_signature() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'invalid_payload_signature'
    let raw: &[u8] =
        hex!("81a6737461747573b9696e76616c69645f7061796c6f61645f7369676e6174757265").as_ref();
    let expected = anonymous_cmds::async_enrollment_submit::Rep::InvalidPayloadSignature;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

fn rep_helper(raw: &[u8], expected: anonymous_cmds::async_enrollment_submit::Rep) {
    let data = anonymous_cmds::async_enrollment_submit::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::async_enrollment_submit::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

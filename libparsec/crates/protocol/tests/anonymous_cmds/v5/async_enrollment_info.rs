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
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   cmd: 'async_enrollment_info'
    //   enrollment_id: ext(2, 0x7c021dd1e2cd4c328f00dd6e64b89279)
    let raw: &[u8] = hex!(
        "82a3636d64b56173796e635f656e726f6c6c6d656e745f696e666fad656e726f6c6c6d"
        "656e745f6964d8027c021dd1e2cd4c328f00dd6e64b89279"
    )
    .as_ref();

    let req = anonymous_cmds::async_enrollment_info::Req {
        enrollment_id: AsyncEnrollmentID::from_hex("7c021dd1e2cd4c328f00dd6e64b89279").unwrap(),
    };
    println!("***expected: {:?}", req.dump().unwrap());

    let expected = anonymous_cmds::AnyCmdReq::AsyncEnrollmentInfo(req.clone());

    let data = anonymous_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = req.dump().unwrap();

    let data2 = anonymous_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    for (raw, expected) in [
        (
            // Generated from Parsec 3.7.1-a.0+dev
            // Content:
            //   status: 'ok'
            //   enrollment_status: 'SUBMITTED'
            //   submitted_on: ext(1, 946688461000000) i.e. 2000-01-01T02:01:01Z
            hex!(
                "83a6737461747573a26f6bb1656e726f6c6c6d656e745f737461747573a95355424d49"
                "54544544ac7375626d69747465645f6f6ed70100035d02156e4d40"
            ).as_ref(),

            anonymous_cmds::async_enrollment_info::Rep::Ok (
                anonymous_cmds::async_enrollment_info::InfoStatus::Submitted {
                    submitted_on: "2000-01-01T01:01:01Z".parse().unwrap(),
                }
            )
        ),
        (
            // Generated from Parsec 3.7.1-a.0+dev
            // Content:
            //   status: 'ok'
            //   enrollment_status: 'REJECTED'
            //   rejected_on: ext(1, 978314522000000) i.e. 2001-01-01T03:02:02Z
            //   submitted_on: ext(1, 946692122000000) i.e. 2000-01-01T03:02:02Z
            hex!(
                "84a6737461747573a26f6bb1656e726f6c6c6d656e745f737461747573a852454a4543"
                "544544ab72656a65637465645f6f6ed701000379c5998ffa80ac7375626d6974746564"
                "5f6f6ed70100035d02efa4ba80"
            ).as_ref(),

            anonymous_cmds::async_enrollment_info::Rep::Ok (
                anonymous_cmds::async_enrollment_info::InfoStatus::Rejected {
                    submitted_on: "2000-01-01T02:02:02Z".parse().unwrap(),
                    rejected_on: "2001-01-01T02:02:02Z".parse().unwrap(),
                }
            )
        ),
        (
            // Generated from Parsec 3.7.1-a.0+dev
            // Content:
            //   status: 'ok'
            //   enrollment_status: 'CANCELLED'
            //   cancelled_on: ext(1, 978318183000000) i.e. 2001-01-01T04:03:03Z
            //   submitted_on: ext(1, 946695783000000) i.e. 2000-01-01T04:03:03Z
            hex!(
                "84a6737461747573a26f6bb1656e726f6c6c6d656e745f737461747573a943414e4345"
                "4c4c4544ac63616e63656c6c65645f6f6ed701000379c673c667c0ac7375626d697474"
                "65645f6f6ed70100035d03c9db27c0"
            ).as_ref(),

            anonymous_cmds::async_enrollment_info::Rep::Ok (
                anonymous_cmds::async_enrollment_info::InfoStatus::Cancelled {
                    submitted_on: "2000-01-01T03:03:03Z".parse().unwrap(),
                    cancelled_on: "2001-01-01T03:03:03Z".parse().unwrap(),
                }
            )
        ),
        (
            // Generated from Parsec 3.7.1-a.0+dev
            // Content:
            //   status: 'ok'
            //   enrollment_status: 'ACCEPTED'
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
            //   accepted_on: ext(1, 978321844000000) i.e. 2001-01-01T05:04:04Z
            //   submitted_on: ext(1, 946699444000000) i.e. 2000-01-01T05:04:04Z
            hex!(
                "86a6737461747573a26f6bb1656e726f6c6c6d656e745f737461747573a84143434550"
                "544544ae6163636570745f7061796c6f6164c4de0028b52ffd0058a50600340c87a474"
                "797065bf6173796e635f656e726f6c6c6d656e745f6163636570745f7061796c6f6164"
                "a7757365725f6964d802a11cec001000a9646576696365de10ac6c6162656caf4d7920"
                "64657631206d616368696e65ac68756d616e5f68616e646c6592b1616c696365406578"
                "616d706c652e636f6db2416c69636579204d6346616365a770726f66696c65a541444d"
                "494eaf726f6f745f7665726966795f6b6579c420be2976732cec8ca94eedcf0aafd413"
                "cd159363e0fadc9e68572c77a1e17d9bbd0500cf22e067c68292eda7d8b60128b86163"
                "636570745f7061796c6f61645f7369676e617475726585a474797065a3504b49bd6163"
                "6365707465725f6465725f783530395f6365727469666963617465c41b3c6163636570"
                "74657220783530392063657274696669636174653ea9616c676f726974686db1525341"
                "5353412d5053532d534841323536d922696e7465726d6564696174655f6465725f7835"
                "30395f63657274696669636174657392c4213c696e7465726d65646961746520312078"
                "3530392063657274696669636174653ec4213c696e7465726d65646961746520322078"
                "3530392063657274696669636174653ea97369676e6174757265c40b3c7369676e6174"
                "7572653eab61636365707465645f6f6ed701000379c74dfcd500ac7375626d69747465"
                "645f6f6ed70100035d04a4119500"
            ).as_ref(),

            anonymous_cmds::async_enrollment_info::Rep::Ok (
                anonymous_cmds::async_enrollment_info::InfoStatus::Accepted {
                    submitted_on: "2000-01-01T04:04:04Z".parse().unwrap(),
                    accepted_on: "2001-01-01T04:04:04Z".parse().unwrap(),
                    accept_payload: hex!(
                        "0028b52ffd0058a50600340c87a474797065bf6173796e635f656e726f6c6c6d656e74"
                        "5f6163636570745f7061796c6f6164a7757365725f6964d802a11cec001000a9646576"
                        "696365de10ac6c6162656caf4d792064657631206d616368696e65ac68756d616e5f68"
                        "616e646c6592b1616c696365406578616d706c652e636f6db2416c69636579204d6346"
                        "616365a770726f66696c65a541444d494eaf726f6f745f7665726966795f6b6579c420"
                        "be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd0500cf"
                        "22e067c68292eda7d8b60128"
                    ).as_ref().into(),
                    accept_payload_signature: anonymous_cmds::async_enrollment_info::AcceptPayloadSignature::PKI {
                        signature: hex!("3c7369676e61747572653e").as_ref().into(),
                        algorithm: PkiSignatureAlgorithm::RsassaPssSha256,
                        accepter_der_x509_certificate: b"<accepter x509 certificate>".as_ref().into(),
                        intermediate_der_x509_certificates: [
                            b"<intermediate 1 x509 certificate>".as_ref().into(),
                            b"<intermediate 2 x509 certificate>".as_ref().into()
                        ].to_vec(),
                    },
                }
            )
        ),
        (
            // Generated from Parsec 3.7.1-a.0+dev
            // Content:
            //   status: 'ok'
            //   enrollment_status: 'ACCEPTED'
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
            //   accepted_on: ext(1, 978325505000000) i.e. 2001-01-01T06:05:05Z
            //   submitted_on: ext(1, 946703105000000) i.e. 2000-01-01T06:05:05Z
            hex!(
                "86a6737461747573a26f6bb1656e726f6c6c6d656e745f737461747573a84143434550"
                "544544ae6163636570745f7061796c6f6164c4de0028b52ffd0058a50600340c87a474"
                "797065bf6173796e635f656e726f6c6c6d656e745f6163636570745f7061796c6f6164"
                "a7757365725f6964d802a11cec001000a9646576696365de10ac6c6162656caf4d7920"
                "64657631206d616368696e65ac68756d616e5f68616e646c6592b1616c696365406578"
                "616d706c652e636f6db2416c69636579204d6346616365a770726f66696c65a541444d"
                "494eaf726f6f745f7665726966795f6b6579c420be2976732cec8ca94eedcf0aafd413"
                "cd159363e0fadc9e68572c77a1e17d9bbd0500cf22e067c68292eda7d8b60128b86163"
                "636570745f7061796c6f61645f7369676e617475726583a474797065a84f50454e5f42"
                "414fba61636365707465725f6f70656e62616f5f656e746974795f6964d92465376430"
                "376532612d623035642d656335392d646366322d343430336566646266316237a97369"
                "676e6174757265d9617661756c743a76313a43346a525a782b796d4c6f753236744e38"
                "51324b44793436644134375737782f4d48366e75455a5671647a2b483052766f614662"
                "515541486365424b68422b516f7732715841586952464146574b47505a55393343513d"
                "3dab61636365707465645f6f6ed701000379c828334240ac7375626d69747465645f6f"
                "6ed70100035d057e480240"
            ).as_ref(),

            anonymous_cmds::async_enrollment_info::Rep::Ok (
                anonymous_cmds::async_enrollment_info::InfoStatus::Accepted {
                    submitted_on: "2000-01-01T05:05:05Z".parse().unwrap(),
                    accepted_on: "2001-01-01T05:05:05Z".parse().unwrap(),
                    accept_payload: hex!(
                        "0028b52ffd0058a50600340c87a474797065bf6173796e635f656e726f6c6c6d656e74"
                        "5f6163636570745f7061796c6f6164a7757365725f6964d802a11cec001000a9646576"
                        "696365de10ac6c6162656caf4d792064657631206d616368696e65ac68756d616e5f68"
                        "616e646c6592b1616c696365406578616d706c652e636f6db2416c69636579204d6346"
                        "616365a770726f66696c65a541444d494eaf726f6f745f7665726966795f6b6579c420"
                        "be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd0500cf"
                        "22e067c68292eda7d8b60128"
                    ).as_ref().into(),
                    accept_payload_signature: anonymous_cmds::async_enrollment_info::AcceptPayloadSignature::OpenBao {
                        signature: "vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==".to_owned(),
                        accepter_openbao_entity_id: "e7d07e2a-b05d-ec59-dcf2-4403efdbf1b7".to_owned(),
                    },
                }
            )
        ),
    ] {
        println!("***expected: {:?}", expected.dump().unwrap());

        rep_helper(raw, expected);
    }
}

pub fn rep_enrollment_not_found() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'enrollment_not_found'
    let raw: &[u8] = hex!("81a6737461747573b4656e726f6c6c6d656e745f6e6f745f666f756e64").as_ref();
    let expected = anonymous_cmds::async_enrollment_info::Rep::EnrollmentNotFound;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

fn rep_helper(raw: &[u8], expected: anonymous_cmds::async_enrollment_info::Rep) {
    let data = anonymous_cmds::async_enrollment_info::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::async_enrollment_info::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

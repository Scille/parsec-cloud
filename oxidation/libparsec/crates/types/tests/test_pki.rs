// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use rstest::rstest;
use std::str::FromStr;

use libparsec_crypto::{PublicKey, VerifyKey};
use libparsec_types::{
    DeviceID, DeviceLabel, PkiEnrollmentAnswerPayload, PkiEnrollmentSubmitPayload, UserProfile,
};

#[rstest]
#[case::without(
    // Generated from Python implementation (Parsec v2.13.0+dev)
    // Content:
    //   type: "pki_enrollment_answer_payload"
    //   device_id: "alice@dev1"
    //   device_label: None
    //   human_handle: None
    //   profile: "ADMIN"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    //
    &hex!(
        "789c6b5b5e50949f969993bad4d1c5d7d36f4d4a6a5966726a7c4e62526ace81f545f9f925"
        "f165a945996995f1d9a9954714f6699615ebbce959e9f7f63cd7fa2bc2674527273ff87567"
        "5e46b84ef9c287b5b3f7ae84eacf4c59959803643800f9866b324a7313f3e23312f3527252"
        "0f2c29a92c48dd5b909d199f9a57949f93939b9a57129f98575c9e5a145f905899939f9802"
        "00271d4432"
    )[..],
    PkiEnrollmentAnswerPayload {
        device_id: DeviceID::from_str("alice@dev1").unwrap(),
        device_label: None,
        human_handle: None,
        profile: UserProfile::Admin,
        root_verify_key: libparsec_crypto::VerifyKey::from(hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd"))
    }
)]
#[case::full(
    // Generated from Python implementation (Parsec v2.13.0+dev)
    // Content:
    //   type: "pki_enrollment_answer_payload"
    //   device_id: "alice@dev1"
    //   device_label: "My dev1 machine"
    //   human_handle: ["alice@example.com", "Alicey McAliceFace"]
    //   profile: "ADMIN"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    //
    &hex!(
        "789c6b5b99925a96999c1a9f99b22a3107c87000f20dd717e5e797c497a51665a655c667a7"
        "561e51d8a75956acf3a667a5dfdbf35cebaf089f159d9cfce0d79d7919e13ae50b1fd6cede"
        "bb066a4c4e62526ace7adf4a0590310ab989c9199979a9cb0b8af2d3327352973abaf87afa"
        "2d29a92c48dd5b909d199f9a57949f93939b9a57129f98575c9e5a145f905899939f98b226"
        "a33437312f3e23312f252775d24688c3522b12730b7252f592f373373982442a157c93c10c"
        "b7c4e45400b9cd57ac"
    )[..],
    PkiEnrollmentAnswerPayload {
        device_id: DeviceID::from_str("alice@dev1").unwrap(),
        device_label: Some(DeviceLabel::from_str("My dev1 machine").unwrap()),
        human_handle: Some(("alice@example.com", "Alicey McAliceFace").try_into().unwrap()),
        profile: UserProfile::Admin,
        root_verify_key: VerifyKey::from(hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd"))
    }
)]
fn serde_pki_enrollment_answer_payload(
    #[case] raw: &[u8],
    #[case] expected: PkiEnrollmentAnswerPayload,
) {
    let data = PkiEnrollmentAnswerPayload::load(raw).unwrap();
    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw = data.dump();
    let data = PkiEnrollmentAnswerPayload::load(&raw).unwrap();

    assert_eq!(data, expected);
}

#[rstest]
#[case(
    // Generated from Python implementation (Parsec v2.13.0+dev)
    // Content:
    //   type: "pki_enrollment_submit_payload"
    //   public_key: hex!("e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d")
    //   requested_device_label: "My dev1 machine"
    //   verify_key: hex!("845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9")
    //
    &hex!(
        "789c6bd956945a589a5a5c929a129f925a96999c1a9f9398949ab3deb75201c83754c84d4c"
        "cec8cc4b5d55509a9493991c9f9d5a7944e1e126ee36ae8af7e5bd3ce5898ac7996a6ef23c"
        "f0f6d597d294f2b829f8d1b5c87755596a51665a2544714b88e8d926710f868090db2bae84"
        "ad91586b57b8e6fe0c3ec96b2b552585934efe5c52525990bab7203b333e35af283f272737"
        "35af24beb8342937b324be20b132273f3105007e8d4634"
    )[..],
    PkiEnrollmentSubmitPayload {
        public_key: PublicKey::from(hex!("e1b20b860a78ef778d0c776121c7027cd90ce04b4d2f1a291a48d911f145724d")),
        requested_device_label: DeviceLabel::from_str("My dev1 machine").unwrap(),
        verify_key: VerifyKey::from(hex!("845415cd821748005054dba8d456ac18ad3e71acdf980e19d6a925191362c9f9"))
    }
)]
fn serde_pki_enrollment_submit_payload(
    #[case] raw: &[u8],
    #[case] expected: PkiEnrollmentSubmitPayload,
) {
    let data = PkiEnrollmentSubmitPayload::load(raw).unwrap();
    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw = data.dump();
    let data = PkiEnrollmentSubmitPayload::load(&raw).unwrap();

    assert_eq!(data, expected);
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use crate::fixtures::{alice, Device};
use crate::prelude::*;

#[rstest]
fn password_protected_device_file(alice: &Device) {
    // Generated from Parsec 3.0.0-b.6+dev
    // Content:
    //   type: "password"
    //   ciphertext: <encrypted alice local device>
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   device_label: "My dev1 machine"
    //   organization_id: "CoolOrg"
    //   device_id: "alice@dev1"
    //   slug: "f78292422e#CoolOrg#alice@dev1"
    //   algorithm:
    //     type: "ARGON2ID"
    //     salt: hex!("2ae6167f0f7472b8565c390df3af4a8b")
    //     opslimit: 1
    //     memlimit_kb: 8
    //     parallelism: 1
    let filedata = hex!(
        "88aa63697068657274657874c502119743c0a4c62a016e8c1afd000197ecdc6a589ccb"
        "c5a97e323eb2a0ad4304f2e2a04dc56fcdda1bf857ca6ef5c8a63c1485b65d333da166"
        "f59395ef12381016d5d7edc934d112b6404d113a0549d87b86f673c970dc740e580150"
        "c6fafc155c217a8e2ab463f10d32c3a4ea20e500cd5ec88f4e0f20f8772cf70b0ffb3d"
        "0a6fbd35d29d4676a012f08c9cf8f8c3e36b0af29fa9a7371d785ee06f6b5966567e17"
        "cdb4c2a9789511b6f469b394668a56ca60b2ab3d4843007075d8cde834054db43751dc"
        "39e2dd3936750a3244f53778645e7daeb3b1030e7edae8bea4a770ce2cfb465a5b5996"
        "2fdc574859ba3d5989b349501d5d0224f1388bb9a5f68e578319502c4b3b9d8f7f0c06"
        "5018bc2b3368e3bb96d13cc53e22848bee0b86b4f7daa497bdb308cd3d39daf09f198a"
        "ceb93f385d4984c8ed36e225d0ef40c60431ec1791e8082f0039eaaa7d8641af4e2dbc"
        "441a36c71f13d4e214440fcbc25374d370d8d5033f4a45c4c9cb5fe7483d6eeba308b1"
        "efd9a4f335ef80b3e8353462088622afaf25916d495945065e1db440615986e0a1b4c0"
        "f8c29f8819c548603a76215e8301508504aa5dbd136233304c75ed6327a4706e9a5074"
        "e76d9c693a58d0e411eb54a67a910b15e36894c83e8099f4b80027053faf46fde70a84"
        "69650d6f91560a7dfe1b876d23f22c2668aaefb817cdc47be0f1f8d2a5500014920f51"
        "adb2f5a1d9a3e543012c3ed270ff12cd481df5a96465766963655f6964aa616c696365"
        "4064657631ac6465766963655f6c6162656caf4d792064657631206d616368696e65ac"
        "68756d616e5f68616e646c6592b1616c696365406578616d706c652e636f6db2416c69"
        "636579204d63416c69636546616365af6f7267616e697a6174696f6e5f6964a7436f6f"
        "6c4f7267a4736c7567bd6637383239323432326523436f6f6c4f726723616c69636540"
        "64657631a474797065a870617373776f7264a9616c676f726974686d85a474797065a8"
        "4152474f4e324944a473616c74c4102ae6167f0f7472b8565c390df3af4a8ba86f7073"
        "6c696d697401ab6d656d6c696d69745f6b6208ab706172616c6c656c69736d01"
    );
    let _password = "P@ssw0rd.";

    let expected = DeviceFilePassword {
        ciphertext: hex!(
            "9743c0a4c62a016e8c1afd000197ecdc6a589ccbc5a97e323eb2a0ad4304f2e2a04dc5"
            "6fcdda1bf857ca6ef5c8a63c1485b65d333da166f59395ef12381016d5d7edc934d112"
            "b6404d113a0549d87b86f673c970dc740e580150c6fafc155c217a8e2ab463f10d32c3"
            "a4ea20e500cd5ec88f4e0f20f8772cf70b0ffb3d0a6fbd35d29d4676a012f08c9cf8f8"
            "c3e36b0af29fa9a7371d785ee06f6b5966567e17cdb4c2a9789511b6f469b394668a56"
            "ca60b2ab3d4843007075d8cde834054db43751dc39e2dd3936750a3244f53778645e7d"
            "aeb3b1030e7edae8bea4a770ce2cfb465a5b59962fdc574859ba3d5989b349501d5d02"
            "24f1388bb9a5f68e578319502c4b3b9d8f7f0c065018bc2b3368e3bb96d13cc53e2284"
            "8bee0b86b4f7daa497bdb308cd3d39daf09f198aceb93f385d4984c8ed36e225d0ef40"
            "c60431ec1791e8082f0039eaaa7d8641af4e2dbc441a36c71f13d4e214440fcbc25374"
            "d370d8d5033f4a45c4c9cb5fe7483d6eeba308b1efd9a4f335ef80b3e8353462088622"
            "afaf25916d495945065e1db440615986e0a1b4c0f8c29f8819c548603a76215e830150"
            "8504aa5dbd136233304c75ed6327a4706e9a5074e76d9c693a58d0e411eb54a67a910b"
            "15e36894c83e8099f4b80027053faf46fde70a8469650d6f91560a7dfe1b876d23f22c"
            "2668aaefb817cdc47be0f1f8d2a5500014920f51adb2f5a1d9a3e543012c3ed270ff12"
            "cd481df5"
        )
        .as_ref()
        .into(),
        human_handle: alice.human_handle.clone(),
        device_label: alice.device_label.clone(),
        user_id: alice.user_id,
        device_id: alice.device_id,
        organization_id: alice.organization_id().to_owned(),
        slug: alice.local_device().slug(),
        algorithm: DeviceFilePasswordAlgorithm::Argon2id {
            salt: hex!("2ae6167f0f7472b8565c390df3af4a8b").as_ref().into(),
            opslimit: 1,
            memlimit_kb: 8,
            parallelism: 1,
        },
    };

    let file_device = rmp_serde::from_slice::<DeviceFilePassword>(&filedata).unwrap();
    p_assert_eq!(file_device, expected);

    // TODO: Test ciphertext decryption
}

#[rstest]
fn keyring_device_file(alice: &Device) {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   type: "keyring"
    //   ciphertext: <encrypted alice local device>
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   device_label: "My dev1 machine"
    //   organization_id: "CoolOrg"
    //   device_id: "alice@dev1"
    //   slug: "f78292422e#CoolOrg#alice@dev1"
    //   keyring_user: "keyring_user"
    let filedata = hex!(
        "99a76b657972696e67c5021141d162cceacbc6845a582ee1497e37469ce46e6e09b33e"
        "1cd5d68d0e054188e38aa85e2a02d0a344cd986e712274f8824f9464fa5a27f7f2291f"
        "42cf579b6be44e0733f89a07f45150dfd8096634f685a44db07693e085e3af6f5525c1"
        "4216f3860adaf612c4a8235e005bc1f9dc31aa24f49383b5afb9f7a2e788bbadb67589"
        "4a5f316085449b223df957b799140fe9cdfd3fea56fbaa452b250cad39ce4cc6c7ccdf"
        "e8248e15c7575c4b259f7d3aa93c106b5d99443724b6b2be4e9f55dd3ba5cd29da633a"
        "a20a643599221b8bfe99c6029f7f2160cd18c89a05d71713d51b57b98f56036a9cab70"
        "4376cd6a754985345fc6309de5c6e852489406b946e213a71154668ccf9adb089527d2"
        "8f84ba7b8425d0d13aa697e8ecaa5b6192b7400b1fc589bbb72a7b5dbf1b9942a3837d"
        "1bb1e86401f329172ed57e140a67ec80ba3a74fc804436588ec8057540529c121887ba"
        "4e86236d53b1b8ed7f8d8d4798c769e5ddbdf785fbe9872cf5dc201016705f67424dfd"
        "3a02ddc5be737a64325c80a3b678b3908e1036de6880c897fb87236decde2110eeff74"
        "53f07b755bb634020d2a06fb3599513cbd0c56f25f2cd7fc10ecf11719ec719d7e36d6"
        "d79d9f72ec213d9be2113c9e90d9449e1e2b143c608662c9456b7fb34ff1dcbcf8f54e"
        "f860d6681f6a8c6ef65e7831a381999dfedc98e7b33afae58d4edb101475c29fd9804a"
        "7bd9adfc99bc2697e16c460e5a4f2dc792b1616c696365406578616d706c652e636f6d"
        "b2416c69636579204d63416c69636546616365af4d792064657631206d616368696e65"
        "aa616c6963654064657631a7436f6f6c4f7267bd6637383239323432326523436f6f6c"
        "4f726723616c6963654064657631a6706172736563ac6b657972696e675f75736572"
    );

    let expected = DeviceFileKeyring {
        ciphertext: hex!(
            "41d162cceacbc6845a582ee1497e37469ce46e6e09b33e1cd5d68d0e054188e38aa85e"
            "2a02d0a344cd986e712274f8824f9464fa5a27f7f2291f42cf579b6be44e0733f89a07"
            "f45150dfd8096634f685a44db07693e085e3af6f5525c14216f3860adaf612c4a8235e"
            "005bc1f9dc31aa24f49383b5afb9f7a2e788bbadb675894a5f316085449b223df957b7"
            "99140fe9cdfd3fea56fbaa452b250cad39ce4cc6c7ccdfe8248e15c7575c4b259f7d3a"
            "a93c106b5d99443724b6b2be4e9f55dd3ba5cd29da633aa20a643599221b8bfe99c602"
            "9f7f2160cd18c89a05d71713d51b57b98f56036a9cab704376cd6a754985345fc6309d"
            "e5c6e852489406b946e213a71154668ccf9adb089527d28f84ba7b8425d0d13aa697e8"
            "ecaa5b6192b7400b1fc589bbb72a7b5dbf1b9942a3837d1bb1e86401f329172ed57e14"
            "0a67ec80ba3a74fc804436588ec8057540529c121887ba4e86236d53b1b8ed7f8d8d47"
            "98c769e5ddbdf785fbe9872cf5dc201016705f67424dfd3a02ddc5be737a64325c80a3"
            "b678b3908e1036de6880c897fb87236decde2110eeff7453f07b755bb634020d2a06fb"
            "3599513cbd0c56f25f2cd7fc10ecf11719ec719d7e36d6d79d9f72ec213d9be2113c9e"
            "90d9449e1e2b143c608662c9456b7fb34ff1dcbcf8f54ef860d6681f6a8c6ef65e7831"
            "a381999dfedc98e7b33afae58d4edb101475c29fd9804a7bd9adfc99bc2697e16c460e"
            "5a4f2dc7"
        )
        .as_ref()
        .into(),
        human_handle: alice.human_handle.clone(),
        device_label: alice.device_label.clone(),
        user_id: alice.user_id,
        device_id: alice.device_id,
        organization_id: alice.organization_id().to_owned(),
        slug: alice.local_device().slug(),
        keyring_service: "parsec".into(),
        keyring_user: "keyring_user".into(),
    };

    let file_device = rmp_serde::from_slice::<DeviceFileKeyring>(&filedata).unwrap();
    p_assert_eq!(file_device, expected);

    // TODO: Test ciphertext decryption
}

#[rstest]
fn recovery_device_file(alice: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "recovery"
    //   ciphertext: <encrypted alice local device>
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   device_label: "My dev1 machine"
    //   organization_id: "CoolOrg"
    //   device_id: "alice@dev1"
    //   slug: "f78292422e#CoolOrg#alice@dev1"
    let filedata = hex!(
        "87aa63697068657274657874c5021141d162cceacbc6845a582ee1497e37469ce46e6e"
        "09b33e1cd5d68d0e054188e38aa85e2a02d0a344cd986e712274f8824f9464fa5a27f7"
        "f2291f42cf579b6be44e0733f89a07f45150dfd8096634f685a44db07693e085e3af6f"
        "5525c14216f3860adaf612c4a8235e005bc1f9dc31aa24f49383b5afb9f7a2e788bbad"
        "b675894a5f316085449b223df957b799140fe9cdfd3fea56fbaa452b250cad39ce4cc6"
        "c7ccdfe8248e15c7575c4b259f7d3aa93c106b5d99443724b6b2be4e9f55dd3ba5cd29"
        "da633aa20a643599221b8bfe99c6029f7f2160cd18c89a05d71713d51b57b98f56036a"
        "9cab704376cd6a754985345fc6309de5c6e852489406b946e213a71154668ccf9adb08"
        "9527d28f84ba7b8425d0d13aa697e8ecaa5b6192b7400b1fc589bbb72a7b5dbf1b9942"
        "a3837d1bb1e86401f329172ed57e140a67ec80ba3a74fc804436588ec8057540529c12"
        "1887ba4e86236d53b1b8ed7f8d8d4798c769e5ddbdf785fbe9872cf5dc201016705f67"
        "424dfd3a02ddc5be737a64325c80a3b678b3908e1036de6880c897fb87236decde2110"
        "eeff7453f07b755bb634020d2a06fb3599513cbd0c56f25f2cd7fc10ecf11719ec719d"
        "7e36d6d79d9f72ec213d9be2113c9e90d9449e1e2b143c608662c9456b7fb34ff1dcbc"
        "f8f54ef860d6681f6a8c6ef65e7831a381999dfedc98e7b33afae58d4edb101475c29f"
        "d9804a7bd9adfc99bc2697e16c460e5a4f2dc7a96465766963655f6964aa616c696365"
        "4064657631ac6465766963655f6c6162656caf4d792064657631206d616368696e65ac"
        "68756d616e5f68616e646c6592b1616c696365406578616d706c652e636f6db2416c69"
        "636579204d63416c69636546616365af6f7267616e697a6174696f6e5f6964a7436f6f"
        "6c4f7267a4736c7567bd6637383239323432326523436f6f6c4f726723616c69636540"
        "64657631a474797065a87265636f76657279"
    );
    let _recovery_password = "F4D4-ZGIQ-3DYH-WPFF-QPIM-DWXJ-VFKA-Z7FT-K444-EU2Q-7DAI-QPGW-NNWQ";

    let expected = DeviceFileRecovery {
        ciphertext: hex!(
            "41d162cceacbc6845a582ee1497e37469ce46e6e09b33e1cd5d68d0e054188e38aa85e"
            "2a02d0a344cd986e712274f8824f9464fa5a27f7f2291f42cf579b6be44e0733f89a07"
            "f45150dfd8096634f685a44db07693e085e3af6f5525c14216f3860adaf612c4a8235e"
            "005bc1f9dc31aa24f49383b5afb9f7a2e788bbadb675894a5f316085449b223df957b7"
            "99140fe9cdfd3fea56fbaa452b250cad39ce4cc6c7ccdfe8248e15c7575c4b259f7d3a"
            "a93c106b5d99443724b6b2be4e9f55dd3ba5cd29da633aa20a643599221b8bfe99c602"
            "9f7f2160cd18c89a05d71713d51b57b98f56036a9cab704376cd6a754985345fc6309d"
            "e5c6e852489406b946e213a71154668ccf9adb089527d28f84ba7b8425d0d13aa697e8"
            "ecaa5b6192b7400b1fc589bbb72a7b5dbf1b9942a3837d1bb1e86401f329172ed57e14"
            "0a67ec80ba3a74fc804436588ec8057540529c121887ba4e86236d53b1b8ed7f8d8d47"
            "98c769e5ddbdf785fbe9872cf5dc201016705f67424dfd3a02ddc5be737a64325c80a3"
            "b678b3908e1036de6880c897fb87236decde2110eeff7453f07b755bb634020d2a06fb"
            "3599513cbd0c56f25f2cd7fc10ecf11719ec719d7e36d6d79d9f72ec213d9be2113c9e"
            "90d9449e1e2b143c608662c9456b7fb34ff1dcbcf8f54ef860d6681f6a8c6ef65e7831"
            "a381999dfedc98e7b33afae58d4edb101475c29fd9804a7bd9adfc99bc2697e16c460e"
            "5a4f2dc7"
        )
        .as_ref()
        .into(),
        human_handle: alice.human_handle.clone(),
        device_label: alice.device_label.clone(),
        user_id: alice.user_id,
        device_id: alice.device_id,
        organization_id: alice.organization_id().to_owned(),
        slug: alice.local_device().slug(),
    };

    let file_device = rmp_serde::from_slice::<DeviceFileRecovery>(&filedata).unwrap();
    p_assert_eq!(file_device, expected);

    // TODO: Test ciphertext decryption
}

#[rstest]
fn smartcard_device_file(alice: &Device) {
    // Generated from Python implementation (Parsec v2.15.0+dev)
    // Content:
    //   type: "smartcard"
    //   certificate_id: "foo"
    //   certificate_sha1: hex!("666f6f")
    //   ciphertext: hex!(
    //     "a73aff77a2aa692b4393e094bfd2c2ccad4b0a8d010960caf27165b787fb412ed2aeeb99f492a870"
    //     "63e368cebe38dc1f20c65273cb3254480cc9e4b519a53241205b531b41edfa749419b83aeb0fb46c"
    //     "c2e21ae25782a1ab8fd3a32ca3b8fca4a0ffc8a301b62aca6612d87e7f34a89b6e747ec82d38734d"
    //     "7943e17009d091d699871aaf964b8426292d0a405ea3e868dca65028dae317a0311af3a958f86541"
    //     "edaef33d49e05056ccb038cc9f7dae40a336dd207eea4341229ba7efa39aa0df28d0d33d91fba49d"
    //     "d63f3814c162fff9083674acd6cc8b621b869c801d0a527474e7da6cd51053803529542d39c9e679"
    //     "4353be278c39ec06cb20560a01e80db86a20df80808f2115ff28afdc2cf9da5099218d4c873dfcbf"
    //     "78e88e4e63ddfcf883de5527b4b234ca63c286a7aa12de2fc6337dd1709f6f5922e3d9f1029ce2b6"
    //     "6d2fb856edb1c701f32c33fa4ca5d0789f52ce2091c48270324f5f631000f6ded1f0c5e1ae94831f"
    //     "488faeff93d8e0c2e26411b499dea920a14733fbea42dd95a8ec13726f33c45f0c19f6e6b9b37add"
    //     "e46ce49465ebad63bfd8106e2d1fb7bc2ff3fea5c86d713226e098aca0ea48fe4180e801eac583e9"
    //     "6fd9fae329358f54f57d46c22f845e3d083f6d6deaf09d821eaaadbbd945ac6f8131b70427794db0"
    //     "dedba44fcb6224654859605a2bbeb979e7d73f233724ab4846b38c94ce603de796f866d0d90fb0ba"
    //     "dd037a135cca4d018e"
    //   )
    //   device_id: "alice@dev1"
    //   device_label: "My dev1 machine"
    //   encrypted_key: hex!("666f6f")
    //   human_handle: ["alice@example.com", "Alicey McAliceFace"]
    //   organization_id: "CoolOrg"
    //   slug: "f78292422e#CoolOrg#alice@dev1"
    //
    let raw = hex!(
        "8aae63657274696669636174655f6964a3666f6fb063657274696669636174655f7368"
        "6131c403666f6faa63697068657274657874c50211a73aff77a2aa692b4393e094bfd2"
        "c2ccad4b0a8d010960caf27165b787fb412ed2aeeb99f492a87063e368cebe38dc1f20"
        "c65273cb3254480cc9e4b519a53241205b531b41edfa749419b83aeb0fb46cc2e21ae2"
        "5782a1ab8fd3a32ca3b8fca4a0ffc8a301b62aca6612d87e7f34a89b6e747ec82d3873"
        "4d7943e17009d091d699871aaf964b8426292d0a405ea3e868dca65028dae317a0311a"
        "f3a958f86541edaef33d49e05056ccb038cc9f7dae40a336dd207eea4341229ba7efa3"
        "9aa0df28d0d33d91fba49dd63f3814c162fff9083674acd6cc8b621b869c801d0a5274"
        "74e7da6cd51053803529542d39c9e6794353be278c39ec06cb20560a01e80db86a20df"
        "80808f2115ff28afdc2cf9da5099218d4c873dfcbf78e88e4e63ddfcf883de5527b4b2"
        "34ca63c286a7aa12de2fc6337dd1709f6f5922e3d9f1029ce2b66d2fb856edb1c701f3"
        "2c33fa4ca5d0789f52ce2091c48270324f5f631000f6ded1f0c5e1ae94831f488faeff"
        "93d8e0c2e26411b499dea920a14733fbea42dd95a8ec13726f33c45f0c19f6e6b9b37a"
        "dde46ce49465ebad63bfd8106e2d1fb7bc2ff3fea5c86d713226e098aca0ea48fe4180"
        "e801eac583e96fd9fae329358f54f57d46c22f845e3d083f6d6deaf09d821eaaadbbd9"
        "45ac6f8131b70427794db0dedba44fcb6224654859605a2bbeb979e7d73f233724ab48"
        "46b38c94ce603de796f866d0d90fb0badd037a135cca4d018ea96465766963655f6964"
        "aa616c6963654064657631ac6465766963655f6c6162656caf4d792064657631206d61"
        "6368696e65ad656e637279707465645f6b6579c403666f6fac68756d616e5f68616e64"
        "6c6592b1616c696365406578616d706c652e636f6db2416c69636579204d63416c6963"
        "6546616365af6f7267616e697a6174696f6e5f6964a7436f6f6c4f7267a4736c7567bd"
        "6637383239323432326523436f6f6c4f726723616c6963654064657631a474797065a9"
        "736d61727463617264"
    );
    let expected = DeviceFile::Smartcard(DeviceFileSmartcard {
        encrypted_key: b"foo".as_ref().into(),
        certificate_id: "foo".into(),
        certificate_sha1: Some("foo".into()),
        ciphertext: hex!(
            "a73aff77a2aa692b4393e094bfd2c2ccad4b0a8d010960caf27165b787fb412ed2aeeb"
            "99f492a87063e368cebe38dc1f20c65273cb3254480cc9e4b519a53241205b531b41ed"
            "fa749419b83aeb0fb46cc2e21ae25782a1ab8fd3a32ca3b8fca4a0ffc8a301b62aca66"
            "12d87e7f34a89b6e747ec82d38734d7943e17009d091d699871aaf964b8426292d0a40"
            "5ea3e868dca65028dae317a0311af3a958f86541edaef33d49e05056ccb038cc9f7dae"
            "40a336dd207eea4341229ba7efa39aa0df28d0d33d91fba49dd63f3814c162fff90836"
            "74acd6cc8b621b869c801d0a527474e7da6cd51053803529542d39c9e6794353be278c"
            "39ec06cb20560a01e80db86a20df80808f2115ff28afdc2cf9da5099218d4c873dfcbf"
            "78e88e4e63ddfcf883de5527b4b234ca63c286a7aa12de2fc6337dd1709f6f5922e3d9"
            "f1029ce2b66d2fb856edb1c701f32c33fa4ca5d0789f52ce2091c48270324f5f631000"
            "f6ded1f0c5e1ae94831f488faeff93d8e0c2e26411b499dea920a14733fbea42dd95a8"
            "ec13726f33c45f0c19f6e6b9b37adde46ce49465ebad63bfd8106e2d1fb7bc2ff3fea5"
            "c86d713226e098aca0ea48fe4180e801eac583e96fd9fae329358f54f57d46c22f845e"
            "3d083f6d6deaf09d821eaaadbbd945ac6f8131b70427794db0dedba44fcb6224654859"
            "605a2bbeb979e7d73f233724ab4846b38c94ce603de796f866d0d90fb0badd037a135c"
            "ca4d018e"
        )
        .as_ref()
        .into(),
        human_handle: alice.human_handle.clone(),
        device_label: alice.device_label.clone(),
        user_id: alice.user_id,
        device_id: alice.device_id,
        organization_id: alice.organization_id().clone(),
        slug: alice.local_device().slug(),
    });

    let device = DeviceFile::load(&raw).unwrap();

    p_assert_eq!(device, expected);

    // Also test roundtrip

    let raw2 = device.dump();
    let device2 = DeviceFile::load(&raw2).unwrap();

    p_assert_eq!(device2, expected);
}

#[test]
fn available_device() {
    let org: OrganizationID = "CoolOrg".parse().unwrap();

    let available = AvailableDevice {
        key_file_path: "/foo/bar".into(),
        organization_id: org.clone(),
        user_id: "9c50250fa3b644e29f77eeefa53dc37d".parse().unwrap(),
        device_id: "9fd3863a3eb240cfaec64904efe5bed3".parse().unwrap(),
        slug:
            "672d515cbb#CoolOrg#9c50250fa3b644e29f77eeefa53dc37d@9fd3863a3eb240cfaec64904efe5bed3"
                .to_owned(),
        human_handle: HumanHandle::new("john@example.com", "John Doe").unwrap(),
        device_label: "MyPc".parse().unwrap(),
        ty: DeviceFileType::Password,
    };

    p_assert_eq!(
        available.slughash(),
        "57f426e7a3cd5dc4a5d19fb8a83addb9112a65d12a13e2f72dd1fdfb9a8a4971"
    );
}

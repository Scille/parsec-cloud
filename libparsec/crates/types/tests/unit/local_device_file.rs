// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use crate::fixtures::{alice, Device};
use crate::prelude::*;

#[rstest]
fn password_protected_device_file(alice: &Device) {
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: "password"
    //   ciphertext: <encrypted alice local device>
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   device_label: "My dev1 machine"
    //   organization_id: "CoolOrg"
    //   device_id: ext(2, de10a11cec0010000000000000000000)
    //   algorithm:
    //     type: "ARGON2ID"
    //     salt: hex!("2ae6167f0f7472b8565c390df3af4a8b")
    //     opslimit: 1
    //     memlimit_kb: 8
    //     parallelism: 1
    let raw = hex!(
        "9ba870617373776f7264d70100047c0f0d84c000d70100047cc41a172000b668747470"
        "733a2f2f7061727365632e696e76616c6964a7436f6f6c4f7267d802a11cec00100000"
        "000000000000000000d802de10a11cec001000000000000000000092b1616c69636540"
        "6578616d706c652e636f6db2416c69636579204d63416c69636546616365af4d792064"
        "657631206d616368696e6595a84152474f4e324944080101c4102ae6167f0f7472b856"
        "5c390df3af4a8bc502119743c0a4c62a016e8c1afd000197ecdc6a589ccbc5a97e323e"
        "b2a0ad4304f2e2a04dc56fcdda1bf857ca6ef5c8a63c1485b65d333da166f59395ef12"
        "381016d5d7edc934d112b6404d113a0549d87b86f673c970dc740e580150c6fafc155c"
        "217a8e2ab463f10d32c3a4ea20e500cd5ec88f4e0f20f8772cf70b0ffb3d0a6fbd35d2"
        "9d4676a012f08c9cf8f8c3e36b0af29fa9a7371d785ee06f6b5966567e17cdb4c2a978"
        "9511b6f469b394668a56ca60b2ab3d4843007075d8cde834054db43751dc39e2dd3936"
        "750a3244f53778645e7daeb3b1030e7edae8bea4a770ce2cfb465a5b59962fdc574859"
        "ba3d5989b349501d5d0224f1388bb9a5f68e578319502c4b3b9d8f7f0c065018bc2b33"
        "68e3bb96d13cc53e22848bee0b86b4f7daa497bdb308cd3d39daf09f198aceb93f385d"
        "4984c8ed36e225d0ef40c60431ec1791e8082f0039eaaa7d8641af4e2dbc441a36c71f"
        "13d4e214440fcbc25374d370d8d5033f4a45c4c9cb5fe7483d6eeba308b1efd9a4f335"
        "ef80b3e8353462088622afaf25916d495945065e1db440615986e0a1b4c0f8c29f8819"
        "c548603a76215e8301508504aa5dbd136233304c75ed6327a4706e9a5074e76d9c693a"
        "58d0e411eb54a67a910b15e36894c83e8099f4b80027053faf46fde70a8469650d6f91"
        "560a7dfe1b876d23f22c2668aaefb817cdc47be0f1f8d2a5500014920f51adb2f5a1d9"
        "a3e543012c3ed270ff12cd481df5"
    );
    let _password = "P@ssw0rd.";

    let expected = DeviceFile::Password(DeviceFilePassword {
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
        created_on: "2010-01-01T00:00:00Z".parse().unwrap(),
        protected_on: "2010-01-10T00:00:00Z".parse().unwrap(),
        server_url: "https://parsec.invalid".to_string(),
        organization_id: alice.organization_id().to_owned(),
        user_id: alice.user_id,
        device_id: alice.device_id,
        human_handle: alice.human_handle.clone(),
        device_label: alice.device_label.clone(),
        algorithm: PasswordAlgorithm::Argon2id {
            salt: hex!("2ae6167f0f7472b8565c390df3af4a8b"),
            opslimit: 1,
            memlimit_kb: 8,
            parallelism: 1,
        },
    });
    println!("***expected: {:?}", expected.dump());

    let device = DeviceFile::load(&raw).unwrap();
    p_assert_eq!(device, expected);

    // Also test roundtrip

    let raw2 = device.dump();
    let device2 = DeviceFile::load(&raw2).unwrap();

    p_assert_eq!(device2, expected);
}

#[rstest]
fn keyring_device_file(alice: &Device) {
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: "keyring"
    //   ciphertext: <encrypted alice local device>
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   device_label: "My dev1 machine"
    //   organization_id: "CoolOrg"
    //   device_id: ext(2, de10a11cec0010000000000000000000)
    //   keyring_user: "keyring_user"
    let raw = hex!(
        "9ca76b657972696e67d70100047c0f0d84c000d70100047cc41a172000b66874747073"
        "3a2f2f7061727365632e696e76616c6964a7436f6f6c4f7267d802a11cec0010000000"
        "0000000000000000d802de10a11cec001000000000000000000092b1616c6963654065"
        "78616d706c652e636f6db2416c69636579204d63416c69636546616365af4d79206465"
        "7631206d616368696e65a6706172736563ac6b657972696e675f75736572c5021141d1"
        "62cceacbc6845a582ee1497e37469ce46e6e09b33e1cd5d68d0e054188e38aa85e2a02"
        "d0a344cd986e712274f8824f9464fa5a27f7f2291f42cf579b6be44e0733f89a07f451"
        "50dfd8096634f685a44db07693e085e3af6f5525c14216f3860adaf612c4a8235e005b"
        "c1f9dc31aa24f49383b5afb9f7a2e788bbadb675894a5f316085449b223df957b79914"
        "0fe9cdfd3fea56fbaa452b250cad39ce4cc6c7ccdfe8248e15c7575c4b259f7d3aa93c"
        "106b5d99443724b6b2be4e9f55dd3ba5cd29da633aa20a643599221b8bfe99c6029f7f"
        "2160cd18c89a05d71713d51b57b98f56036a9cab704376cd6a754985345fc6309de5c6"
        "e852489406b946e213a71154668ccf9adb089527d28f84ba7b8425d0d13aa697e8ecaa"
        "5b6192b7400b1fc589bbb72a7b5dbf1b9942a3837d1bb1e86401f329172ed57e140a67"
        "ec80ba3a74fc804436588ec8057540529c121887ba4e86236d53b1b8ed7f8d8d4798c7"
        "69e5ddbdf785fbe9872cf5dc201016705f67424dfd3a02ddc5be737a64325c80a3b678"
        "b3908e1036de6880c897fb87236decde2110eeff7453f07b755bb634020d2a06fb3599"
        "513cbd0c56f25f2cd7fc10ecf11719ec719d7e36d6d79d9f72ec213d9be2113c9e90d9"
        "449e1e2b143c608662c9456b7fb34ff1dcbcf8f54ef860d6681f6a8c6ef65e7831a381"
        "999dfedc98e7b33afae58d4edb101475c29fd9804a7bd9adfc99bc2697e16c460e5a4f"
        "2dc7"
    );

    let expected = DeviceFile::Keyring(DeviceFileKeyring {
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
        created_on: "2010-01-01T00:00:00Z".parse().unwrap(),
        protected_on: "2010-01-10T00:00:00Z".parse().unwrap(),
        server_url: "https://parsec.invalid".to_string(),
        organization_id: alice.organization_id().to_owned(),
        user_id: alice.user_id,
        device_id: alice.device_id,
        human_handle: alice.human_handle.clone(),
        device_label: alice.device_label.clone(),
        keyring_service: "parsec".into(),
        keyring_user: "keyring_user".into(),
    });
    println!("***expected: {:?}", expected.dump());

    let device = DeviceFile::load(&raw).unwrap();
    p_assert_eq!(device, expected);

    // Also test roundtrip

    let raw2 = device.dump();
    let device2 = DeviceFile::load(&raw2).unwrap();

    p_assert_eq!(device2, expected);
}

#[rstest]
fn recovery_device_file(alice: &Device) {
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   type: "recovery"
    //   ciphertext: <encrypted alice local device>
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   device_label: "My dev1 machine"
    //   organization_id: "CoolOrg"
    //   device_id: ext(2, de10a11cec0010000000000000000000)
    let raw = hex!(
        "9aa87265636f76657279d70100047c0f0d84c000d70100047cc41a172000b668747470"
        "733a2f2f7061727365632e696e76616c6964a7436f6f6c4f7267d802a11cec00100000"
        "000000000000000000d802de10a11cec001000000000000000000092b1616c69636540"
        "6578616d706c652e636f6db2416c69636579204d63416c69636546616365af4d792064"
        "657631206d616368696e65c5021141d162cceacbc6845a582ee1497e37469ce46e6e09"
        "b33e1cd5d68d0e054188e38aa85e2a02d0a344cd986e712274f8824f9464fa5a27f7f2"
        "291f42cf579b6be44e0733f89a07f45150dfd8096634f685a44db07693e085e3af6f55"
        "25c14216f3860adaf612c4a8235e005bc1f9dc31aa24f49383b5afb9f7a2e788bbadb6"
        "75894a5f316085449b223df957b799140fe9cdfd3fea56fbaa452b250cad39ce4cc6c7"
        "ccdfe8248e15c7575c4b259f7d3aa93c106b5d99443724b6b2be4e9f55dd3ba5cd29da"
        "633aa20a643599221b8bfe99c6029f7f2160cd18c89a05d71713d51b57b98f56036a9c"
        "ab704376cd6a754985345fc6309de5c6e852489406b946e213a71154668ccf9adb0895"
        "27d28f84ba7b8425d0d13aa697e8ecaa5b6192b7400b1fc589bbb72a7b5dbf1b9942a3"
        "837d1bb1e86401f329172ed57e140a67ec80ba3a74fc804436588ec8057540529c1218"
        "87ba4e86236d53b1b8ed7f8d8d4798c769e5ddbdf785fbe9872cf5dc201016705f6742"
        "4dfd3a02ddc5be737a64325c80a3b678b3908e1036de6880c897fb87236decde2110ee"
        "ff7453f07b755bb634020d2a06fb3599513cbd0c56f25f2cd7fc10ecf11719ec719d7e"
        "36d6d79d9f72ec213d9be2113c9e90d9449e1e2b143c608662c9456b7fb34ff1dcbcf8"
        "f54ef860d6681f6a8c6ef65e7831a381999dfedc98e7b33afae58d4edb101475c29fd9"
        "804a7bd9adfc99bc2697e16c460e5a4f2dc7"
    );
    let _recovery_password = "F4D4-ZGIQ-3DYH-WPFF-QPIM-DWXJ-VFKA-Z7FT-K444-EU2Q-7DAI-QPGW-NNWQ";

    let expected = DeviceFile::Recovery(DeviceFileRecovery {
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
        created_on: "2010-01-01T00:00:00Z".parse().unwrap(),
        protected_on: "2010-01-10T00:00:00Z".parse().unwrap(),
        server_url: "https://parsec.invalid".to_string(),
        organization_id: alice.organization_id().to_owned(),
        user_id: alice.user_id,
        device_id: alice.device_id,
        human_handle: alice.human_handle.clone(),
        device_label: alice.device_label.clone(),
    });
    println!("***expected: {:?}", expected.dump());

    let device = DeviceFile::load(&raw).unwrap();
    p_assert_eq!(device, expected);

    // Also test roundtrip

    let raw2 = device.dump();
    let device2 = DeviceFile::load(&raw2).unwrap();

    p_assert_eq!(device2, expected);
}

#[rstest]

fn smartcard_device_file(alice: &Device) {
    // Generated from Parsec 3.5.1-a.0+dev
    // Content:
    //   type: 'smartcard'
    //   created_on: ext(1, 1262304000000000) i.e. 2010-01-01T01:00:00Z
    //   protected_on: ext(1, 1263081600000000) i.e. 2010-01-10T01:00:00Z
    //   server_url: 'https://parsec.invalid'
    //   organization_id: 'CoolOrg'
    //   user_id: ext(2, 0xa11cec00100000000000000000000000)
    //   device_id: ext(2, 0xde10a11cec0010000000000000000000)
    //   human_handle: [ 'alice@example.com', 'Alicey McAliceFace', ]
    //   device_label: 'My dev1 machine'
    //   certificate_ref: {
    //     uris: [ { windowscng: 0x666f6f, }, ],
    //     hash: 'sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=',
    //   }
    //   algorithm_for_encrypted_key: 'RSAES-OAEP-SHA256'
    //   encrypted_key: 0x666f6f
    //   ciphertext:
    //     0xa73aff77a2aa692b4393e094bfd2c2ccad4b0a8d010960caf27165b787fb412ed2aeeb99f492a8
    //     7063e368cebe38dc1f20c65273cb3254480cc9e4b519a53241205b531b41edfa749419b83aeb0fb4
    //     6cc2e21ae25782a1ab8fd3a32ca3b8fca4a0ffc8a301b62aca6612d87e7f34a89b6e747ec82d3873
    //     4d7943e17009d091d699871aaf964b8426292d0a405ea3e868dca65028dae317a0311af3a958f865
    //     41edaef33d49e05056ccb038cc9f7dae40a336dd207eea4341229ba7efa39aa0df28d0d33d91fba4
    //     9dd63f3814c162fff9083674acd6cc8b621b869c801d0a527474e7da6cd51053803529542d39c9e6
    //     794353be278c39ec06cb20560a01e80db86a20df80808f2115ff28afdc2cf9da5099218d4c873dfc
    //     bf78e88e4e63ddfcf883de5527b4b234ca63c286a7aa12de2fc6337dd1709f6f5922e3d9f1029ce2
    //     b66d2fb856edb1c701f32c33fa4ca5d0789f52ce2091c48270324f5f631000f6ded1f0c5e1ae9483
    //     1f488faeff93d8e0c2e26411b499dea920a14733fbea42dd95a8ec13726f33c45f0c19f6e6b9b37a
    //     dde46ce49465ebad63bfd8106e2d1fb7bc2ff3fea5c86d713226e098aca0ea48fe4180e801eac583
    //     e96fd9fae329358f54f57d46c22f845e3d083f6d6deaf09d821eaaadbbd945ac6f8131b70427794d
    //     b0dedba44fcb6224654859605a2bbeb979e7d73f233724ab4846b38c94ce603de796f866d0d90fb0
    //     badd037a135cca4d018e
    let raw: &[u8] = hex!(
        "8da474797065a9736d61727463617264aa637265617465645f6f6ed70100047c0f0d84"
        "c000ac70726f7465637465645f6f6ed70100047cc41a172000aa7365727665725f7572"
        "6cb668747470733a2f2f7061727365632e696e76616c6964af6f7267616e697a617469"
        "6f6e5f6964a7436f6f6c4f7267a7757365725f6964d802a11cec001000000000000000"
        "00000000a96465766963655f6964d802de10a11cec0010000000000000000000ac6875"
        "6d616e5f68616e646c6592b1616c696365406578616d706c652e636f6db2416c696365"
        "79204d63416c69636546616365ac6465766963655f6c6162656caf4d79206465763120"
        "6d616368696e65af63657274696669636174655f72656682a4757269739181aa77696e"
        "646f7773636e67c403666f6fa468617368d9337368613235362d414141414141414141"
        "414141414141414141414141414141414141414141414141414141414141414141413d"
        "bb616c676f726974686d5f666f725f656e637279707465645f6b6579b152534145532d"
        "4f4145502d534841323536ad656e637279707465645f6b6579c403666f6faa63697068"
        "657274657874c50211a73aff77a2aa692b4393e094bfd2c2ccad4b0a8d010960caf271"
        "65b787fb412ed2aeeb99f492a87063e368cebe38dc1f20c65273cb3254480cc9e4b519"
        "a53241205b531b41edfa749419b83aeb0fb46cc2e21ae25782a1ab8fd3a32ca3b8fca4"
        "a0ffc8a301b62aca6612d87e7f34a89b6e747ec82d38734d7943e17009d091d699871a"
        "af964b8426292d0a405ea3e868dca65028dae317a0311af3a958f86541edaef33d49e0"
        "5056ccb038cc9f7dae40a336dd207eea4341229ba7efa39aa0df28d0d33d91fba49dd6"
        "3f3814c162fff9083674acd6cc8b621b869c801d0a527474e7da6cd51053803529542d"
        "39c9e6794353be278c39ec06cb20560a01e80db86a20df80808f2115ff28afdc2cf9da"
        "5099218d4c873dfcbf78e88e4e63ddfcf883de5527b4b234ca63c286a7aa12de2fc633"
        "7dd1709f6f5922e3d9f1029ce2b66d2fb856edb1c701f32c33fa4ca5d0789f52ce2091"
        "c48270324f5f631000f6ded1f0c5e1ae94831f488faeff93d8e0c2e26411b499dea920"
        "a14733fbea42dd95a8ec13726f33c45f0c19f6e6b9b37adde46ce49465ebad63bfd810"
        "6e2d1fb7bc2ff3fea5c86d713226e098aca0ea48fe4180e801eac583e96fd9fae32935"
        "8f54f57d46c22f845e3d083f6d6deaf09d821eaaadbbd945ac6f8131b70427794db0de"
        "dba44fcb6224654859605a2bbeb979e7d73f233724ab4846b38c94ce603de796f866d0"
        "d90fb0badd037a135cca4d018e"
    )
    .as_ref();
    let expected = DeviceFile::Smartcard(DeviceFileSmartcard {
        encrypted_key: b"foo".as_ref().into(),
        certificate_ref: X509CertificateReference::from(X509CertificateHash::fake_sha256())
            .add_or_replace_uri(X509WindowsCngURI::from(Bytes::from_static(b"foo"))),
        algorithm_for_encrypted_key: PKIEncryptionAlgorithm::RsaesOaepSha256,
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
        created_on: "2010-01-01T00:00:00Z".parse().unwrap(),
        protected_on: "2010-01-10T00:00:00Z".parse().unwrap(),
        server_url: "https://parsec.invalid".to_string(),
        organization_id: alice.organization_id().to_owned(),
        user_id: alice.user_id,
        device_id: alice.device_id,
        human_handle: alice.human_handle.clone(),
        device_label: alice.device_label.clone(),
    });
    println!("***expected: {:?}", expected.dump());

    let device = DeviceFile::load(raw).unwrap();
    p_assert_eq!(device, expected);

    // Also test roundtrip

    let raw2 = device.dump();
    let device2 = DeviceFile::load(&raw2).unwrap();

    p_assert_eq!(device2, expected);
}

#[rstest]
fn account_vault_protected_device_file(alice: &Device) {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   type: 'account_vault'
    //   created_on: ext(1, 1262304000000000) i.e. 2010-01-01T01:00:00Z
    //   protected_on: ext(1, 1263081600000000) i.e. 2010-01-10T01:00:00Z
    //   server_url: 'https://parsec.invalid'
    //   organization_id: 'CoolOrg'
    //   user_id: ext(2, 0xa11cec00100000000000000000000000)
    //   device_id: ext(2, 0xde10a11cec0010000000000000000000)
    //   human_handle: [ 'alice@example.com', 'Alicey McAliceFace', ]
    //   device_label: 'My dev1 machine'
    //   ciphertext_key_id: ext(2, 0x84cbb64f7fbf46cd98196343210df9b0)
    //   ciphertext:
    //     0x9743c0a4c62a016e8c1afd000197ecdc6a589ccbc5a97e323eb2a0ad4304f2e2a04dc56fcdda1b
    //     f857ca6ef5c8a63c1485b65d333da166f59395ef12381016d5d7edc934d112b6404d113a0549d87b
    //     86f673c970dc740e580150c6fafc155c217a8e2ab463f10d32c3a4ea20e500cd5ec88f4e0f20f877
    //     2cf70b0ffb3d0a6fbd35d29d4676a012f08c9cf8f8c3e36b0af29fa9a7371d785ee06f6b5966567e
    //     17cdb4c2a9789511b6f469b394668a56ca60b2ab3d4843007075d8cde834054db43751dc39e2dd39
    //     36750a3244f53778645e7daeb3b1030e7edae8bea4a770ce2cfb465a5b59962fdc574859ba3d5989
    //     b349501d5d0224f1388bb9a5f68e578319502c4b3b9d8f7f0c065018bc2b3368e3bb96d13cc53e22
    //     848bee0b86b4f7daa497bdb308cd3d39daf09f198aceb93f385d4984c8ed36e225d0ef40c60431ec
    //     1791e8082f0039eaaa7d8641af4e2dbc441a36c71f13d4e214440fcbc25374d370d8d5033f4a45c4
    //     c9cb5fe7483d6eeba308b1efd9a4f335ef80b3e8353462088622afaf25916d495945065e1db44061
    //     5986e0a1b4c0f8c29f8819c548603a76215e8301508504aa5dbd136233304c75ed6327a4706e9a50
    //     74e76d9c693a58d0e411eb54a67a910b15e36894c83e8099f4b80027053faf46fde70a8469650d6f
    //     91560a7dfe1b876d23f22c2668aaefb817cdc47be0f1f8d2a5500014920f51adb2f5a1d9a3e54301
    //     2c3ed270ff12cd481df5
    let raw: &[u8] = hex!(
        "8ba474797065ad6163636f756e745f7661756c74aa637265617465645f6f6ed7010004"
        "7c0f0d84c000ac70726f7465637465645f6f6ed70100047cc41a172000aa7365727665"
        "725f75726cb668747470733a2f2f7061727365632e696e76616c6964af6f7267616e69"
        "7a6174696f6e5f6964a7436f6f6c4f7267a7757365725f6964d802a11cec0010000000"
        "0000000000000000a96465766963655f6964d802de10a11cec00100000000000000000"
        "00ac68756d616e5f68616e646c6592b1616c696365406578616d706c652e636f6db241"
        "6c69636579204d63416c69636546616365ac6465766963655f6c6162656caf4d792064"
        "657631206d616368696e65b1636970686572746578745f6b65795f6964d80284cbb64f"
        "7fbf46cd98196343210df9b0aa63697068657274657874c502119743c0a4c62a016e8c"
        "1afd000197ecdc6a589ccbc5a97e323eb2a0ad4304f2e2a04dc56fcdda1bf857ca6ef5"
        "c8a63c1485b65d333da166f59395ef12381016d5d7edc934d112b6404d113a0549d87b"
        "86f673c970dc740e580150c6fafc155c217a8e2ab463f10d32c3a4ea20e500cd5ec88f"
        "4e0f20f8772cf70b0ffb3d0a6fbd35d29d4676a012f08c9cf8f8c3e36b0af29fa9a737"
        "1d785ee06f6b5966567e17cdb4c2a9789511b6f469b394668a56ca60b2ab3d48430070"
        "75d8cde834054db43751dc39e2dd3936750a3244f53778645e7daeb3b1030e7edae8be"
        "a4a770ce2cfb465a5b59962fdc574859ba3d5989b349501d5d0224f1388bb9a5f68e57"
        "8319502c4b3b9d8f7f0c065018bc2b3368e3bb96d13cc53e22848bee0b86b4f7daa497"
        "bdb308cd3d39daf09f198aceb93f385d4984c8ed36e225d0ef40c60431ec1791e8082f"
        "0039eaaa7d8641af4e2dbc441a36c71f13d4e214440fcbc25374d370d8d5033f4a45c4"
        "c9cb5fe7483d6eeba308b1efd9a4f335ef80b3e8353462088622afaf25916d49594506"
        "5e1db440615986e0a1b4c0f8c29f8819c548603a76215e8301508504aa5dbd13623330"
        "4c75ed6327a4706e9a5074e76d9c693a58d0e411eb54a67a910b15e36894c83e8099f4"
        "b80027053faf46fde70a8469650d6f91560a7dfe1b876d23f22c2668aaefb817cdc47b"
        "e0f1f8d2a5500014920f51adb2f5a1d9a3e543012c3ed270ff12cd481df5"
    )
    .as_ref();

    let expected = DeviceFile::AccountVault(DeviceFileAccountVault {
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
        ciphertext_key_id: AccountVaultItemOpaqueKeyID::from_hex(
            "84cbb64f7fbf46cd98196343210df9b0",
        )
        .unwrap(),
        created_on: "2010-01-01T00:00:00Z".parse().unwrap(),
        protected_on: "2010-01-10T00:00:00Z".parse().unwrap(),
        server_url: "https://parsec.invalid".to_string(),
        organization_id: alice.organization_id().to_owned(),
        user_id: alice.user_id,
        device_id: alice.device_id,
        human_handle: alice.human_handle.clone(),
        device_label: alice.device_label.clone(),
    });
    println!("***expected: {:?}", expected.dump());

    let device = DeviceFile::load(raw).unwrap();
    p_assert_eq!(device, expected);

    // Also test roundtrip

    let raw2 = device.dump();
    let device2 = DeviceFile::load(&raw2).unwrap();

    p_assert_eq!(device2, expected);
}

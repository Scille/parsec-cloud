// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use libparsec_types::{HumanHandle, OrganizationID};
use rstest::rstest;

use libparsec_client_types::{
    AvailableDevice, DeviceFilePassword, DeviceFileRecovery, DeviceFileType,
};
use tests_fixtures::{alice, Device};

#[rstest]
fn password_protected_device_file(alice: &Device) {
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "password"
    //   ciphertext: <encrypted alice local device>
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   device_label: "My dev1 machine"
    //   organization_id: "CoolOrg"
    //   device_id: "alice@dev1"
    //   slug: "f78292422e#CoolOrg#alice@dev1"
    //   salt: hex!("2ae6167f0f7472b8565c390df3af4a8b")
    let filedata = hex!(
        "88aa63697068657274657874c502119743c0a4c62a016e8c1afd000197ecdc6a589ccbc5a97e32"
        "3eb2a0ad4304f2e2a04dc56fcdda1bf857ca6ef5c8a63c1485b65d333da166f59395ef12381016"
        "d5d7edc934d112b6404d113a0549d87b86f673c970dc740e580150c6fafc155c217a8e2ab463f1"
        "0d32c3a4ea20e500cd5ec88f4e0f20f8772cf70b0ffb3d0a6fbd35d29d4676a012f08c9cf8f8c3"
        "e36b0af29fa9a7371d785ee06f6b5966567e17cdb4c2a9789511b6f469b394668a56ca60b2ab3d"
        "4843007075d8cde834054db43751dc39e2dd3936750a3244f53778645e7daeb3b1030e7edae8be"
        "a4a770ce2cfb465a5b59962fdc574859ba3d5989b349501d5d0224f1388bb9a5f68e578319502c"
        "4b3b9d8f7f0c065018bc2b3368e3bb96d13cc53e22848bee0b86b4f7daa497bdb308cd3d39daf0"
        "9f198aceb93f385d4984c8ed36e225d0ef40c60431ec1791e8082f0039eaaa7d8641af4e2dbc44"
        "1a36c71f13d4e214440fcbc25374d370d8d5033f4a45c4c9cb5fe7483d6eeba308b1efd9a4f335"
        "ef80b3e8353462088622afaf25916d495945065e1db440615986e0a1b4c0f8c29f8819c548603a"
        "76215e8301508504aa5dbd136233304c75ed6327a4706e9a5074e76d9c693a58d0e411eb54a67a"
        "910b15e36894c83e8099f4b80027053faf46fde70a8469650d6f91560a7dfe1b876d23f22c2668"
        "aaefb817cdc47be0f1f8d2a5500014920f51adb2f5a1d9a3e543012c3ed270ff12cd481df5a964"
        "65766963655f6964aa616c6963654064657631ac6465766963655f6c6162656caf4d7920646576"
        "31206d616368696e65ac68756d616e5f68616e646c6592b1616c696365406578616d706c652e63"
        "6f6db2416c69636579204d63416c69636546616365af6f7267616e697a6174696f6e5f6964a743"
        "6f6f6c4f7267a473616c74c4102ae6167f0f7472b8565c390df3af4a8ba4736c7567bd66373832"
        "39323432326523436f6f6c4f726723616c6963654064657631a474797065a870617373776f7264"
    );
    let _password = "P@ssw0rd.";

    let expected = DeviceFilePassword {
        ciphertext: hex!(
            "9743c0a4c62a016e8c1afd000197ecdc6a589ccbc5a97e323eb2a0ad4304f2e2a04dc56fcd"
            "da1bf857ca6ef5c8a63c1485b65d333da166f59395ef12381016d5d7edc934d112b6404d11"
            "3a0549d87b86f673c970dc740e580150c6fafc155c217a8e2ab463f10d32c3a4ea20e500cd"
            "5ec88f4e0f20f8772cf70b0ffb3d0a6fbd35d29d4676a012f08c9cf8f8c3e36b0af29fa9a7"
            "371d785ee06f6b5966567e17cdb4c2a9789511b6f469b394668a56ca60b2ab3d4843007075"
            "d8cde834054db43751dc39e2dd3936750a3244f53778645e7daeb3b1030e7edae8bea4a770"
            "ce2cfb465a5b59962fdc574859ba3d5989b349501d5d0224f1388bb9a5f68e578319502c4b"
            "3b9d8f7f0c065018bc2b3368e3bb96d13cc53e22848bee0b86b4f7daa497bdb308cd3d39da"
            "f09f198aceb93f385d4984c8ed36e225d0ef40c60431ec1791e8082f0039eaaa7d8641af4e"
            "2dbc441a36c71f13d4e214440fcbc25374d370d8d5033f4a45c4c9cb5fe7483d6eeba308b1"
            "efd9a4f335ef80b3e8353462088622afaf25916d495945065e1db440615986e0a1b4c0f8c2"
            "9f8819c548603a76215e8301508504aa5dbd136233304c75ed6327a4706e9a5074e76d9c69"
            "3a58d0e411eb54a67a910b15e36894c83e8099f4b80027053faf46fde70a8469650d6f9156"
            "0a7dfe1b876d23f22c2668aaefb817cdc47be0f1f8d2a5500014920f51adb2f5a1d9a3e543"
            "012c3ed270ff12cd481df5"
        )
        .to_vec(),
        human_handle: alice.human_handle.to_owned(),
        device_label: alice.device_label.to_owned(),
        device_id: alice.device_id.to_owned(),
        organization_id: alice.organization_id().to_owned(),
        slug: alice.local_device().slug(),
        salt: hex!("2ae6167f0f7472b8565c390df3af4a8b").to_vec(),
    };

    let file_device = rmp_serde::from_slice::<DeviceFilePassword>(&filedata).unwrap();
    assert_eq!(file_device, expected);

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
        "87aa63697068657274657874c5021141d162cceacbc6845a582ee1497e37469ce46e6e09b33e1c"
        "d5d68d0e054188e38aa85e2a02d0a344cd986e712274f8824f9464fa5a27f7f2291f42cf579b6b"
        "e44e0733f89a07f45150dfd8096634f685a44db07693e085e3af6f5525c14216f3860adaf612c4"
        "a8235e005bc1f9dc31aa24f49383b5afb9f7a2e788bbadb675894a5f316085449b223df957b799"
        "140fe9cdfd3fea56fbaa452b250cad39ce4cc6c7ccdfe8248e15c7575c4b259f7d3aa93c106b5d"
        "99443724b6b2be4e9f55dd3ba5cd29da633aa20a643599221b8bfe99c6029f7f2160cd18c89a05"
        "d71713d51b57b98f56036a9cab704376cd6a754985345fc6309de5c6e852489406b946e213a711"
        "54668ccf9adb089527d28f84ba7b8425d0d13aa697e8ecaa5b6192b7400b1fc589bbb72a7b5dbf"
        "1b9942a3837d1bb1e86401f329172ed57e140a67ec80ba3a74fc804436588ec8057540529c1218"
        "87ba4e86236d53b1b8ed7f8d8d4798c769e5ddbdf785fbe9872cf5dc201016705f67424dfd3a02"
        "ddc5be737a64325c80a3b678b3908e1036de6880c897fb87236decde2110eeff7453f07b755bb6"
        "34020d2a06fb3599513cbd0c56f25f2cd7fc10ecf11719ec719d7e36d6d79d9f72ec213d9be211"
        "3c9e90d9449e1e2b143c608662c9456b7fb34ff1dcbcf8f54ef860d6681f6a8c6ef65e7831a381"
        "999dfedc98e7b33afae58d4edb101475c29fd9804a7bd9adfc99bc2697e16c460e5a4f2dc7a964"
        "65766963655f6964aa616c6963654064657631ac6465766963655f6c6162656caf4d7920646576"
        "31206d616368696e65ac68756d616e5f68616e646c6592b1616c696365406578616d706c652e63"
        "6f6db2416c69636579204d63416c69636546616365af6f7267616e697a6174696f6e5f6964a743"
        "6f6f6c4f7267a4736c7567bd6637383239323432326523436f6f6c4f726723616c696365406465"
        "7631a474797065a87265636f76657279"
    );
    let _recovery_password = "F4D4-ZGIQ-3DYH-WPFF-QPIM-DWXJ-VFKA-Z7FT-K444-EU2Q-7DAI-QPGW-NNWQ";

    let expected = DeviceFileRecovery {
        ciphertext: hex!(
            "41d162cceacbc6845a582ee1497e37469ce46e6e09b33e1cd5d68d0e054188e38aa85e2a02"
            "d0a344cd986e712274f8824f9464fa5a27f7f2291f42cf579b6be44e0733f89a07f45150df"
            "d8096634f685a44db07693e085e3af6f5525c14216f3860adaf612c4a8235e005bc1f9dc31"
            "aa24f49383b5afb9f7a2e788bbadb675894a5f316085449b223df957b799140fe9cdfd3fea"
            "56fbaa452b250cad39ce4cc6c7ccdfe8248e15c7575c4b259f7d3aa93c106b5d99443724b6"
            "b2be4e9f55dd3ba5cd29da633aa20a643599221b8bfe99c6029f7f2160cd18c89a05d71713"
            "d51b57b98f56036a9cab704376cd6a754985345fc6309de5c6e852489406b946e213a71154"
            "668ccf9adb089527d28f84ba7b8425d0d13aa697e8ecaa5b6192b7400b1fc589bbb72a7b5d"
            "bf1b9942a3837d1bb1e86401f329172ed57e140a67ec80ba3a74fc804436588ec805754052"
            "9c121887ba4e86236d53b1b8ed7f8d8d4798c769e5ddbdf785fbe9872cf5dc201016705f67"
            "424dfd3a02ddc5be737a64325c80a3b678b3908e1036de6880c897fb87236decde2110eeff"
            "7453f07b755bb634020d2a06fb3599513cbd0c56f25f2cd7fc10ecf11719ec719d7e36d6d7"
            "9d9f72ec213d9be2113c9e90d9449e1e2b143c608662c9456b7fb34ff1dcbcf8f54ef860d6"
            "681f6a8c6ef65e7831a381999dfedc98e7b33afae58d4edb101475c29fd9804a7bd9adfc99"
            "bc2697e16c460e5a4f2dc7"
        )
        .to_vec(),
        human_handle: alice.human_handle.to_owned(),
        device_label: alice.device_label.to_owned(),
        device_id: alice.device_id.to_owned(),
        organization_id: alice.organization_id().to_owned(),
        slug: alice.local_device().slug(),
    };

    let file_device = rmp_serde::from_slice::<DeviceFileRecovery>(&filedata).unwrap();
    assert_eq!(file_device, expected);

    // TODO: Test ciphertext decryption
}

#[rstest]
fn test_available_device() {
    let org: OrganizationID = "CoolOrg".parse().unwrap();

    let available = AvailableDevice {
        key_file_path: "/foo/bar".into(),
        organization_id: org.clone(),
        device_id: "9c50250fa3b644e29f77eeefa53dc37d@9fd3863a3eb240cfaec64904efe5bed3"
            .parse()
            .unwrap(),
        slug:
            "672d515cbb#CoolOrg#9c50250fa3b644e29f77eeefa53dc37d@9fd3863a3eb240cfaec64904efe5bed3"
                .to_owned(),
        human_handle: Some(HumanHandle::new("john@example.com", "John Doe").unwrap()),
        device_label: Some("MyPc".parse().unwrap()),
        ty: DeviceFileType::Password,
    };

    assert_eq!(
        available.slughash(),
        "57f426e7a3cd5dc4a5d19fb8a83addb9112a65d12a13e2f72dd1fdfb9a8a4971"
    );
    assert_eq!(available.user_display(), "John Doe <john@example.com>");
    assert_eq!(available.short_user_display(), "John Doe");
    assert_eq!(available.device_display(), "MyPc");

    let available_legacy = AvailableDevice {
        key_file_path: "/foo/bar".into(),
        organization_id: org,
        device_id: "john@mypc".parse().unwrap(),
        slug: "672d515cbb#CoolOrg#john@mypc".to_owned(),
        human_handle: None,
        device_label: None,
        ty: DeviceFileType::Password,
    };

    assert_eq!(
        available_legacy.slughash(),
        "35e1d5ceb858ea3fd89973e084105d2bd928de10047cdd6d0037259030954ca1"
    );
    assert_eq!(available_legacy.user_display(), "john");
    assert_eq!(available_legacy.short_user_display(), "john");
    assert_eq!(available_legacy.device_display(), "mypc");
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use rstest::rstest;

use libparsec_crypto::*;
use libparsec_types::*;

use libparsec_tests_fixtures::{alice, Device};

#[rstest]
#[case::admin(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   organization_addr: parsec://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&rvk=XYUXM4ZM5SGKSTXNZ4FK7VATZUKZGY7A7LOJ42CXFR32DYL5TO6Qssss"
    //   device_id: "alice@dev1"
    //   device_label: "My dev1 machine"
    //   human_handle: ("alice@example.com", "Alicey McAliceFace")
    //   signing_key: <alice.signing_key as bytes>
    //   private_key: <alice.private_key as bytes>
    //   is_admin: True
    //   profile: "ADMIN"
    //   user_manifest_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_manifest_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!("beca94a37a491bccbdb90244de62d292ab587d3adc2806f78f014efeb57d456c2e495d1da88d"
    "8ed7dcf301a95d87874944a9f1fcba54206f6816b7aafb6884cc835054c8442c8c2ab11cb6482b382d"
    "616fe3e7a67820a0000465d14c1d98403aaa1973071dc5ed89d204ca661956202054d187451ca9d0d1"
    "ee3b86c6a043b9a25b25f9b7acf6957bdbd77aef5e91ca2079ceb4e2baa7192eff02bd612b4cb7e219"
    "b1aebf760ad3e7f49d5da6ac61c070353bfa3b5fc8db96dbb2bc7002eda76d81db67c7f621471cc126"
    "782200ec5fcd9808b247c77496579c7dcd27139aaee30da14aa62cf781c035c905f629a63c770d85a3"
    "85b9e5efd93b024f06d2c641b233283220f13d197ede76b9747ab738a1bba7e8317e477d1d1e7346e2"
    "25bd965f4bac75ff325575d3978ba6b868f197cfd90f2dd7fd5be3ce50b5650c7c664f833cb1aa33cf"
    "52ca74ff1e7b280a65cca00b6be55afc433d2b86b3ee183ea5fc1a4c30683cacfb2df3682549a5cd5c"
    "0696cda4e8ab03282b96ee2b515ae5b23c4242f73bbaabfef8abc75b79f3176efcf4af002374a5416e"
    "b2226975cb256b5a5f067b8a720a21e5b63428d6f0f67d1374af5d4defd2d87e49019da25e11803bdc"
    "ad854f90c189231dc27c621ba19813cd73278e324d3629cf8db380d444f32ae94f6ff4c1def9455ee2"
    "02a43ae5cec12cc2267987e2c4f9740b31dad8e6760aefd651cd5cd7a73db10ae2c024210b062dcd"),
    true,
    true,
    UserProfile::Admin,
)]
#[case::standard(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   organization_addr: parsec://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&rvk=XYUXM4ZM5SGKSTXNZ4FK7VATZUKZGY7A7LOJ42CXFR32DYL5TO6Qssss"
    //   device_id: "alice@dev1"
    //   device_label: "My dev1 machine"
    //   human_handle: ("alice@example.com", "Alicey McAliceFace")
    //   signing_key: <alice.signing_key as bytes>
    //   private_key: <alice.private_key as bytes>
    //   is_admin: False
    //   profile: "STANDARD"
    //   user_manifest_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_manifest_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!("5af07c0039eae0ca900d27696c40448f73e3758e1f370809748590ac0bb7693e0745f9e38e7a"
    "96e865ebb4dd39092e9c11c4d6a2ba9449ae717e8c841e4fa82a7ebfee578af63319dda187440bdbfe"
    "e813174f993b63c29e5717d479527c5a1438d074c60e1009bd8f957865a4500f59a070d786fa85a460"
    "88b17a33d6a2df9117da46e949ad077fec560af2edc3252721d37d435da98b93f7f966b7de85c6ae1a"
    "32b4372274ad8bcc4183bd557404741980e3fcd3c0cf3fab909b69a04fd4c8fccf08bddf5a77519678"
    "82f1e6ca827d99d475428e373d9103e74b7badaea8c8f01827b07f8e22c64e76bcb4e994df4c46653d"
    "a258f5e69965ea20e65bd068c61e947a60caa7adc27a0b959e9fd8b4be017243856f30eccdb93ccc2d"
    "b7b9cf7b1953f07ce78a38d072047238ddb027bfc8a4dea3e41c3bf2c7aa66144e15f694beb2583230"
    "5a0b060051cf36f0bf7da2b5661583c92ba522b949e0cb840ae526c30f2e8f7c8af3324387b9919b7f"
    "aa5b2b49b8ec26b6d940c71d4d587c8ad311421ae491a9141e6a6cfc535c96734f993ca4f5cbee2b80"
    "48055688dd0cc6681bd57930db40e7993ccf1c6dbd795dd20ccbb9b52caf3ac845f629cabfbccc0761"
    "e722b25663a5c933f43f5eecfa9a90b31cc26389e41493ec1a753a35b70f0a358c0b411f6d46a800d6"
    "2d424bf412d3528bd281ef85b660e4f4e3051a248555a1085cde9f196ad9a92b1698a5e46732f8bf68"
    "1763"),
    true,
    true,
    UserProfile::Standard,
)]
#[case::outsider(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   organization_addr: parsec://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&rvk=XYUXM4ZM5SGKSTXNZ4FK7VATZUKZGY7A7LOJ42CXFR32DYL5TO6Qssss"
    //   device_id: "alice@dev1"
    //   device_label: "My dev1 machine"
    //   human_handle: ("alice@example.com", "Alicey McAliceFace")
    //   signing_key: <alice.signing_key as bytes>
    //   private_key: <alice.private_key as bytes>
    //   is_admin: False
    //   profile: "OUTSIDER"
    //   user_manifest_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_manifest_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!("37587595b3acfb304055ce8c8bde9acaddc5d6f5019da7db2a959ba09a885642481094a6a5fa"
    "4b0812ddc4efec88b84d5ddf97955dc19465d42dfc46d760a9be9258b518d398dc994840d80750a449"
    "af56212b7b883bded7d31631c97f1f2c55625280b13a30b7faeaf34a6b820727ccb2e04d2ea97fa6be"
    "7b8fdabe9829101358b9432a3928f77ef076b9ce8ca1d279031211708dbc4257b6fdd26ccf21c44a03"
    "9cf48d05b8b12eb2d89512e1563ba3cfde7c01b8bd90b7e3d9bb67d4e2819a97a24a1c0b8445d5ae70"
    "702ab3302b731bac8f827e6688ea7a9ded96394b79b3d776bb0ef8ea86b2a8d55354fd49d4069db4ae"
    "a309b0d76bfc1f8cf765d1761ca29a87a70cb67e1c727424c4276995f58f4e92cd3d0e100a635c1010"
    "9ae47f158d0627e5c5ceb9da6cc113f930c79fb1160e2e4badb47aacbb3e7e435a8048950a1c4c2376"
    "0abfdb4a6b7a9dfdbf61389808b9b4cd1129bd20609715913a595bf5f269be49f3af5a0a84ca3260f7"
    "b7e9751347ebbba0e2f3feeabc1635eac32ef918faf8405d61c565f03e47acca598a3e1021f45a0aa3"
    "0eeccf4844269c1474b8d638856f622d3ef36812d93e9e11a451467c9ff7f1926481cb13b2702b61a3"
    "e1e9e1ad512294fa36a5bacf618b27a87e340b409ca2aed69b49a7d201747539427a418ed349995658"
    "9fb52f153480dce5bcad316520c52b6c5f519f91e92ad031af63dda97107e77bfa0c8922252b3583cc"
    "606d"),
    true,
    true,
    UserProfile::Outsider,
)]
#[case::no_device_label_no_human_handle(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   organization_addr: parsec://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&rvk=XYUXM4ZM5SGKSTXNZ4FK7VATZUKZGY7A7LOJ42CXFR32DYL5TO6Qssss"
    //   device_id: "alice@dev1"
    //   device_label: None
    //   human_handle: None
    //   signing_key: <alice.signing_key as bytes>
    //   private_key: <alice.private_key as bytes>
    //   is_admin: True
    //   profile: "ADMIN"
    //   user_manifest_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_manifest_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!("37ad98807525c58ec3e9820fcd2e186ab19d7f4e0c5a7753a5c9e603f06410a78130b6fab9ce"
    "0bddd1bcb01a09b2c023ff664f0cc19a3db53c8754b4d424a8562c52e0657a50f2b07d1d607481db94"
    "483d67a354353f5dbbbd3250f16ebf712c4b93650945eb64aae53790404174e3fa79d03cbfa997a4af"
    "4e0937319b20452a1308dfca16d539ae1391338c053c2872074edc79415645fadefe0bb60f826382ad"
    "c230a16f64a9f7dd63aec318d06f7ba2c3ba06ba2810adb6d7b7f53c3cccfa592c06e2417e89a24ba9"
    "b4ab78d651bc0dc36aa6dc701f7b417f9350953388a284a58fe5dc59ebc17985bbe38f6eec6626859d"
    "dd1bea59f79eebf55adcc56b76584704bce45a559b5d27abef0b64790c4b05cc166200e613f979c8e5"
    "f66c61efcfe4944688ee1de89d8cdc9e153a616107b1e65368c1e34aead0befc51e9dd17b3da36fb67"
    "0f380562f254a0a5e3e07df4c1a1ddea26d867ff809b85b2a51609e9d61c43723b9916b61ec443a979"
    "f3947a47f618a988234f7608a56cc64510af40c620b0d778a643410e343c7ec938d1078563c50cb4ff"
    "e55813114cdcdacb8027c9291a5feb8cd0e968b3744fbaecc11b6224da8a0e2971896d8cc84e132825"
    "1519459218185c90511dc53630d4bd391698191f81f045e9f87cacc12b"),
    false,
    false,
    UserProfile::Admin,
)]
#[case::legacy_format_admin(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   organization_addr: parsec://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&rvk=XYUXM4ZM5SGKSTXNZ4FK7VATZUKZGY7A7LOJ42CXFR32DYL5TO6Qssss"
    //   device_id: "alice@dev1"
    //   signing_key: <alice.signing_key as bytes>
    //   private_key: <alice.private_key as bytes>
    //   is_admin: True
    //   user_manifest_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_manifest_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!("7e60a6301d747dbeff43e497edca2600f6e2bc0b3f1001cf58fbf3eba68da610867f528896d3"
    "471d7e27384e8234a5b38fe5d9a10fc9d78ec83382d18353d2404112251e9117c19dc7e080bc6e2691"
    "dbd914695670df4b1d58eb07474c017656fafbff02fb8e2e1e5c7a2215900423a66406b2c4158e7672"
    "c99d25206ef5f3477932eb7023afd65c10a2e18589710a24bdf5ef225ed63c087aacd5ae86cd2963bb"
    "9e5eb2616a13347d53b00a5700dd3095bdb6d24f13aba3134870848ecf0b0cc3bed6121cb8d524dd84"
    "b80f1960ace6c1707e7b94a4ee3ff7d2eea3e2a50c03466cfbaa1086fbc8b813e055ffc85be6b59c82"
    "fba72f8baa96fb7a3ac90c9fc145a62c6e23b3910a0969ce3c4284a2a50f692c45dda8d25151019fbf"
    "8b6c98436cfeef0b4c039696f331bf68e4741e2fcd34d235782635b3335bb5a52dc222be2b38ccbd50"
    "b011ee6e4e32f010a28c79b0d48aae3c0cecf1644a98a992bbd8cdd5644d999bde9b823a8bf56c4ec8"
    "ea8eae7ab22cecd558b5e14ef83f751e4c9d978999b502abe74ac153301b61e75704e8bde4912bc65a"
    "1c1ddde74521e38615675201410dfa88d6ea9bdeab4c63586d80d427"),
    false,
    false,
    UserProfile::Admin,
)]
#[case::legacy_format_non_admin(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   organization_addr: parsec://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&rvk=XYUXM4ZM5SGKSTXNZ4FK7VATZUKZGY7A7LOJ42CXFR32DYL5TO6Qssss"
    //   device_id: "alice@dev1"
    //   signing_key: <alice.signing_key as bytes>
    //   private_key: <alice.private_key as bytes>
    //   is_admin: False
    //   user_manifest_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_manifest_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!("13050baf7d164c56396eb424b6ed684e1a5e64026b57965f5eabe6ca3a1421022dd10284073c"
    "6a4117b08ee82a18bcf994dcb15e43437e0c0d7325144282d7306dfcb0ab4495047dc4849b194ca1ef"
    "33c47686f0018c93de9b924bcf670de222f3ef5934729730df55981fb9b1437976507000aea0447474"
    "0c4af73e79530fdf04b0701b6e0a54e4385732e993dc1e7a58e90526493b63c874309ae7eac3c5916e"
    "40d72a4eb27542579e7de66717bb425207be26f3ca8ccd0045215a7c2ca3c501987145fb98b755fcc7"
    "5e24efd1a3b2b3cd9eb11658c1809112c00781bffe848b59a4bacc96bdb9d36179f0000550774d4762"
    "dc7bcae8a7efb7bd32c679389281cd4ae9af35bc89c40a4f25ad45351efd34c41cdf110f04b90cda63"
    "c0e19a072e4f2bc4294122373237660b717ddbef3fad8bf7c4e09735803ce0923a542ac90b9691c833"
    "6df74001a20ca6d31b3b4aae83e9f30c90cb3d3b3a68efedc1007e20105a940ac5a352f8f65838b486"
    "be5cfa1cca50f17101b669ef703d2a91168a6060795b78fe1449b9c696b75df49b0f8b21f3854ba7fd"
    "defbcc5c9e5d073afe52ccdb141c2cf19df59879fc81e31c3225ac46"),
    false,
    false,
    UserProfile::Standard,
)]
fn serde_local_device_data(
    alice: &Device,
    #[case] data: &[u8],
    #[case] with_device_label: bool,
    #[case] with_human_handle: bool,
    #[case] user_profile: UserProfile,
) {
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let expected = {
        let mut expected = alice.local_device();
        if !with_device_label {
            expected.device_label = None;
        }
        if !with_human_handle {
            expected.human_handle = None;
        }
        expected.profile = user_profile;
        expected
    };

    let manifest = LocalDevice::decrypt_and_load(data, &key).unwrap();

    assert_eq!(manifest, expected);

    // Also test serialization round trip
    let data2 = manifest.dump_and_encrypt(&key);

    // Note we cannot just compare with `data` due to signature and keys order
    let manifest2 = LocalDevice::decrypt_and_load(&data2, &key).unwrap();
    assert_eq!(manifest2, expected);
}

#[rstest]
fn slug(alice: &Device) {
    let local_device = alice.local_device();

    assert_eq!(local_device.slug(), "f78292422e#CoolOrg#alice@dev1");
    assert_eq!(
        local_device.slughash(),
        "a9172f17141ae27ab662d312869169b36fc9d036e88605a9682c98f372c3041b"
    );
}

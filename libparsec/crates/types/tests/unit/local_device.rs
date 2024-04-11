// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use crate::fixtures::{alice, Device};
use crate::prelude::*;

#[rstest]
#[case::admin(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   organization_addr: "parsec://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&rvk=XYUXM4ZM5SGKSTXNZ4FK7VATZUKZGY7A7LOJ42CXFR32DYL5TO6Qssss"
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
    //   organization_addr: "parsec://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&rvk=XYUXM4ZM5SGKSTXNZ4FK7VATZUKZGY7A7LOJ42CXFR32DYL5TO6Qssss"
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
    //   organization_addr: "parsec://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&rvk=XYUXM4ZM5SGKSTXNZ4FK7VATZUKZGY7A7LOJ42CXFR32DYL5TO6Qssss"
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
    //   organization_addr: "parsec://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&rvk=XYUXM4ZM5SGKSTXNZ4FK7VATZUKZGY7A7LOJ42CXFR32DYL5TO6Qssss"
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
    //   organization_addr: "parsec://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&rvk=XYUXM4ZM5SGKSTXNZ4FK7VATZUKZGY7A7LOJ42CXFR32DYL5TO6Qssss"
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
    //   organization_addr: "parsec://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&rvk=XYUXM4ZM5SGKSTXNZ4FK7VATZUKZGY7A7LOJ42CXFR32DYL5TO6Qssss"
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
fn serde_local_device_data_legacy_v2(
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
            expected.device_label = DeviceLabel::new_redacted(alice.device_id.device_name());
        }
        if !with_human_handle {
            expected.human_handle = HumanHandle::new_redacted(alice.user_id());
        }
        expected.initial_profile = user_profile;
        expected
    };

    let manifest = LocalDevice::decrypt_and_load(data, &key).unwrap();

    p_assert_eq!(manifest, expected);

    // Also test serialization round trip
    let data2 = manifest.dump_and_encrypt(&key);

    // Note we cannot just compare with `data` due to signature and keys order
    let manifest2 = LocalDevice::decrypt_and_load(&data2, &key).unwrap();
    p_assert_eq!(manifest2, expected);
}

#[rstest]
#[case::admin(
    // Generated from v3.0.0-b.6+dev
    // Content:
    //   organization_addr: "parsec3://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&p=xCC-KXZzLOyMqU7tzwqv1BPNFZNj4PrcnmhXLHeh4X2bvQ"  // cspell:disable-line
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
    &hex!(
        "af1493aa6edc33b42dbfcbe9fd570ec85c77c218b06988e1e292d4f33c5b836be6aac2"
        "454f3be0c03e2238a17d4ea5b0d2f88ffc47b881721b5206ea8f2b6381cf7f87180640"
        "76037eae8e79f78764970416b96f4edc8312301ce4f2c2f72c92f36a3d4b956e00708c"
        "e64f4ec5ae8f3550a6e8c7dcb185dcea2dc594390f70ad4c51e57cf8101f564414ab82"
        "b795ab960221eacbc60dcb618827e2e038d13fa55ec6b55414f8e5350e3fc1200c9a3f"
        "e27900ac55ea0f77f94054610f77f1cedf5aefe0e41b5a1205c64c546bfd000e4645c3"
        "719dabac15801a9d103c5dc9b51087fc3b537260e115cefe12996948498bc7d70651c8"
        "6e160086e31c286746db2ad55e14659e77ab26d1ebea89ee382666e80fff300d7f82db"
        "c328285a0b368d0dc0fbb537910309c22ab634dc5a29ef3d24eff5fd20532d73a0dc8a"
        "e20d245a4f6b87701ab8c7305a3844a8d83c1a2c4f2f12c810a986438fa523a489ee8e"
        "d7151262e1f27473477d6dc38c7f74f7297da0dc59888a4c7598a025c7bbc3736429bd"
        "246e08a7a474735a3fc44161335796e679bf08d81c5a5034889989e6ecccc6628fa6d0"
        "dc48807ac9d26a32c05710811dbf61239f7a52978f01f54ddee6665bef2c8998ce9bd4"
        "0c3a69cdbed47da46b76d00573304c5524f49a623e0a4de03ac9842983e789d3767b3c"
        "1059ce8a846715faf46f9919fc9d251db5aa19a8b54b576a88c1b49f"
    ),
    true,
    true,
    UserProfile::Admin,
)]
#[case::standard(
    // Generated from v3.0.0-b.6+dev
    // Content:
    //   organization_addr: "parsec3://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&p=xCC-KXZzLOyMqU7tzwqv1BPNFZNj4PrcnmhXLHeh4X2bvQ"  // cspell:disable-line
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
    &hex!(
        "a48947d94daec37be9c4b75005e8c16a31371873f44e29c3c7b099407014051b8773b7"
        "130f0cea883e6b33ed4bb99b675f73994f6a11fd239dfa048fac85b63c3d9190643387"
        "0aacf0ef05d58aa576950614cee9a162b62f1686290e38f94db1f6488ef568f03e356c"
        "1de02fd37b7818cdffc3cc2b689079af64869dedcf903b7081f8c318595d57d71659b0"
        "a7f669504e058fd77a15f94938c8cb06fac1443cd4bbc3413aa5de41c6b901bb422380"
        "42586d1d1aec51270499709db4777f44dc8e01bd01f3c4d0f4483f541f5dfaed856c44"
        "81f7c0be236e9ddbc2c432c504b0e7985e6b7f519773c3b175434d611082166b025050"
        "efde42f565065b68c334c070e43aa1b58702a5f130fba1447318949e910cf380dd2e39"
        "04bc1026d0e23da2c4e3270f6d615f745c8d65fbbc1ec2f77564f198a56e7992288d61"
        "21b42e6a36393ddcc62fa575581dc1d0201b78678b8f63339cafd46089a02f3fecbc66"
        "63dd94e5c7dde170deae823997371b3414f3dd413f13084265460916e9e87333bf0a55"
        "02d9466ecf1d62441aa75d40b4f7ca22d4d4a7c191a0a89ab93f8880aebed038cd0013"
        "25b838a723e6f004e8c79d47960efe55fabcc0e11777436b92df06d622b3049101331a"
        "cccd9d46ba97019bc6ded6af48650bae6f1112d8ae78a94d3e435a36ce1900b963e647"
        "c3c9b767393dbe7bf53d449dff0d4f8f18af4ee8d68e9277a945d3639548c5"
    ),
    true,
    true,
    UserProfile::Standard,
)]
#[case::outsider(
    // Generated from v3.0.0-b.6+dev
    // Content:
    //   organization_addr: "parsec3://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&p=xCC-KXZzLOyMqU7tzwqv1BPNFZNj4PrcnmhXLHeh4X2bvQ"  // cspell:disable-line
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
    &hex!(
        "3234f5ea940ca16240766319aa1a1a6da35e83c9c6a235c6c976d7fcbfabd6aa490d50"
        "543f3b491390d430747b0ea32883ca6fd3d873f07959f32390d4aa725b17db5ae2a356"
        "ced6ab35fc64b0d69c0b1dc819a224ebcb80d9807f707b9942db2372e2ee2c054d66e4"
        "649ce764058b4713f616fbb29a9b60a86b7319205b8b5332c2d815afe058fcce3dc0d4"
        "3e5d19a9fd0f1b008cbfee5ec2751ee308f83b3658aa0946d4e9bbd78b8738b0387079"
        "38926709e47df3880c8526b74dab706e34ec799061ddb8e8fe34e2802e40680b78dfa4"
        "c6597557f95ed0c6d27484bdb204418c941319df51fe563abe4109be022be199a4975b"
        "9103203afabec2d52e0f202c90db37b428f7f3fb2bbd27d75e59bd9cb06d4d754b32af"
        "c7ae5ffd3712587eac076f5161675627f44e23e4800fccf548b012bafb4a0b3a2859dc"
        "f28afa11eca11e799efbb44785d50ad0592ce89628fddc870e8a973b3dde0e75fb0181"
        "ab76d2566a5ffb039bf7a8ac98fd20128304b815d9edb927f8433c1a030a0c4af6b2ba"
        "c82201aed7d5daec63ee61d95b9722bb688405d3b7fbe781304b488c741b763d3c0b86"
        "c6bcc636ec87bc48dc401eb05ddbc9f06c279ecc1a941782ab1adc545f6e552f56af42"
        "a3f2d4fc7452f81f30c8123a6fc77f731a35a0ec7d59eeca214b100132fce6dc1024a6"
        "c5f8b0653599600b1ffb7fdc3a3fe3b25c1c161d13e9a869c9c5ec559e3173"
    ),
    true,
    true,
    UserProfile::Outsider,
)]
#[case::no_device_label_no_human_handle(
    // Generated from v3.0.0-b.6+dev
    // Content:
    //   organization_addr: "parsec3://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&p=xCC-KXZzLOyMqU7tzwqv1BPNFZNj4PrcnmhXLHeh4X2bvQ"  // cspell:disable-line
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
    &hex!(
        "96814dc462e24fd25e78175b1cfc8827bc8a118a0ca8735bdb69a98f92ee416bf613c8"
        "f5cb6778fca9ef08aad0cda6d2c38b685caccba35830f81fd546e5699f3d9fd471a889"
        "8577809bd5f59848f6088d34fe4a29e78c9c71a61483845c68916702ce41f7b392787b"
        "5410e6506d4af3f36585caf862075448c0114cd6621b5ca5b188d285082dc3a31db953"
        "2a7d221e8614684bfc0e20d835bf29a2a981bb494029a781ccec834322830f3c2eba59"
        "d01d167612434cc54b2893313c8e88dc0e14f918e60980508e1573acb6e296e4235cf6"
        "de38617c5f1693732ea25184a7b29e708ce5f7a6619dddb3363b4eff29ee2c5393160a"
        "2bfad7d64631db54d28957bb9874d1382a15edfcc7e6438c88262bba521662cd9ae1bd"
        "26bd075cdfcf5078d6e65fb0b72c7d25e485791f8071db66f060c0d00a9bb133527163"
        "08da35dccd2effdf3d8327f0022cdd8fc49ca519e7bbd29c6ec54e8f29032e55b7f2c6"
        "dbe3ab5e4bb87cd0da1c37a0578877b3a76e1d352a4834bb95483eee4c4a0634f7be52"
        "c2ee77855bdb5fe68d89a93bc474322dea8180bb895d6e81f4c49e13bbfb897370742a"
        "a8cea1e3d29506f3463e5ada952eac6408d398e129688e4366b317863f98c6f31eb18d"
        "7f"
    ),
    false,
    false,
    UserProfile::Admin,
)]
#[case::legacy_format_admin(
    // Generated from v3.0.0-b.6+dev
    // Content:
    //   organization_addr: "parsec3://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&p=xCC-KXZzLOyMqU7tzwqv1BPNFZNj4PrcnmhXLHeh4X2bvQ"  // cspell:disable-line
    //   device_id: "alice@dev1"
    //   signing_key: <alice.signing_key as bytes>
    //   private_key: <alice.private_key as bytes>
    //   is_admin: True
    //   user_manifest_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_manifest_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!(
        "d89792d05fca215c12b939df6be221541e423a49c070db3e014abb4e7f76504a4f8404"
        "eea110964cc1f2df97ea22e0a6e2a5ccbe41114b25edbc90f80b3dd906bdc290d3bcc8"
        "5b6b19e8a0b3ffe1b3ca7d6f388444c5e07081fa357b2c8bb13f00bdfe36bd9faf3af0"
        "d0f14912475bc7366b468d67d296fc9be546cd3b961b37808cd3d83c5f7e8e27a16bd2"
        "69e9c16e800c777bbd2abc68a88d65cc04a260b09f3f3c5a923df956899028b01018fe"
        "9fd54190992102163c8f4f772f22a4aeb586bffb887234ab5f9c63d85ecdae57a50193"
        "a005036bcddfeee47dfd5edbb03fd6ccf5bdb244848db9d0179aed37384a9889175560"
        "d722c951a698a4e61b6adf83bd4aa2f359745bf5397d771246be348f61aad859a19c5b"
        "1fef2c76582651f4d20bb53a19f63126480770bfbddc077deaeddcc63c67d3c11fe7a4"
        "88544857431b941cf8929c9268618aeb7a3e0f6ca306c5a341a07f8070533084adf4cf"
        "3a96f3443c347e48b82544c47bf975ed3faa6d31be810af4dc1e2c6f4d2b703b2e6c38"
        "c128ffedde264ddd5349d359342db61f44104a7386391ea9c62051aa675349a7050483"
        "a2e1bd50c23faab132745e6b040643feac909cd9227db39108f154c5182a46f2ec7abd"
        "8a"
    ),
    false,
    false,
    UserProfile::Admin,
)]
#[case::legacy_format_non_admin(
    // Generated from v3.0.0-b.6+dev
    // Content:
    //   organization_addr: "parsec3://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&p=xCC-KXZzLOyMqU7tzwqv1BPNFZNj4PrcnmhXLHeh4X2bvQ"  // cspell:disable-line
    //   device_id: "alice@dev1"
    //   signing_key: <alice.signing_key as bytes>
    //   private_key: <alice.private_key as bytes>
    //   is_admin: False
    //   user_manifest_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_manifest_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!(
        "5011226d6835f50f59b3091096121e8436436e33a8381368f8a39c13da44feeef8a86a"
        "0246b4d8ba9dbb646bd0581d50418d0bfd0b43b49fbf7acd0d783f6a9309c714847c64"
        "451b4b0db9d86f9dbd2afc05796314d7928de69058bcf04470e98f967ba735c8b46229"
        "28d42bd2f1467fc39626952cc2e8875048ef102a7a51d268f8c731be0d98aedbd8814a"
        "45527dd8b88b7197c8f27084d6a3504f74b9758e6f4798e714a3db307c59417bae55eb"
        "9b560e49c244becb577041abf09f0bc12a3819fb853c3e32f9497ade3b8c5aa8174804"
        "6f60dc06f4f720123573a7dfd061ef03bc6b40760f8557505372fd01eaef12e5332d03"
        "f30cb2057091ee3a97dc6a3f087d95032d4c8d3746d8ebcfc867424a350616908815e5"
        "7f252bf018e9a10251b191f44f2bd02dde8dcfa9fe9001a122feccf5b4c471c0599ded"
        "f7713654bf308c49daf27cccc807967d928319d370c7a5bebb28f334021c5fbab4b0af"
        "a3524cf03ebe707bc1bc66f0eb2c794bf0a020f97aaa7da3dfb60295db3237ad309c8a"
        "7a2ad30047957ce085ce2ebeebb5a9176ca5c6d2e4a12cb26b585e5fac6dc40c05ed96"
        "82f06f9a7ad50932612484310e39e8471aa652a85995d3d6fad1b20a271b2147717e8a"
        "a628adec"
    ),
    false,
    false,
    UserProfile::Standard,
)]
fn serde_local_device_data_x(
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
            expected.device_label = DeviceLabel::new_redacted(alice.device_id.device_name());
        }
        if !with_human_handle {
            expected.human_handle = HumanHandle::new_redacted(alice.user_id());
        }
        expected.initial_profile = user_profile;
        expected
    };

    let manifest = LocalDevice::decrypt_and_load(data, &key).unwrap();

    p_assert_eq!(manifest, expected);

    // Also test serialization round trip
    let data2 = manifest.dump_and_encrypt(&key);

    // Note we cannot just compare with `data` due to signature and keys order
    let manifest2 = LocalDevice::decrypt_and_load(&data2, &key).unwrap();
    p_assert_eq!(manifest2, expected);
}

#[rstest]
fn slug(alice: &Device) {
    let local_device = alice.local_device();

    p_assert_eq!(local_device.slug(), "f78292422e#CoolOrg#alice@dev1");
    p_assert_eq!(
        local_device.slughash(),
        "a9172f17141ae27ab662d312869169b36fc9d036e88605a9682c98f372c3041b"
    );
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;

use crate::fixtures::{alice, Device};
use crate::prelude::*;

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
    //   profile: "ADMIN"
    //   user_realm_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_realm_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!(
        "ba0ae0c480a5874d0d1a66b2cb8060148b950858978a935eecc11b24da4770807723c0"
        "620a273f5f69a297d5ad28a051b9f36026495123e62e72a7bd9d89faa5f99eec71671f"
        "722b81f13fd9971f49b1575a77f01891f6a25154662e28391f258321fac5c65a97e804"
        "4c81ffb14ab998b5172249988140614cbfc2a8cb17967859e94601224aa68db250c8ac"
        "6b0b7a02c0b3f9152adaa378eb022e009305484ce7cb0c9004f147257a8a39665591e7"
        "cb17cd71c21fa5cda1185d66031466e25ca7f4d65fb8ea26a3b50e120491c5eae97237"
        "3131741539130218fc3439964b933825061829182a725787d5e8520f7f34c61b27971e"
        "ecfa0eba3c1653d2a382dffd1ed5b3697185730aaaefc2154db802954fea4de91a2a56"
        "79a9093357498b53ceedaa5643b35d97387502b42fba4a6fd826138afeedbe007943b4"
        "34bf14ba58e71cf98f30766c65b8b4205188c171e31a29908df296066f2dd7ff54a527"
        "a7e9ad6556f54b80599097800e83d19e4bcb1ffede37359ce3e9da6ae914c4f7fb5334"
        "25a0663b080b81bcda9d9187db528d74d7e5ed3f9db5a4e784e59eb490b4a342caf3ba"
        "3de074d7ccafafb5949c78828044dcb2bb787684a1a795e1607cc358891f16392bf531"
        "3eec401d5d49fa03e21d3c73f0e3aaf7aae2319bbf7276294183427cf41292a1cdbcd4"
        "0044a297afef5b9a77f0d8b280c2a74b0152453b"
    ),
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
    //   profile: "STANDARD"
    //   user_realm_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_realm_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!(
        "bf6d22104f1ff8f892335611d15941409141e4696b185d011931d0433cb7cb836693e2"
        "6f69559487236161871d27068eb797cf82c94daaeb0531de1d68fa898e1bb24112cf7e"
        "d78f4bd420aafdccca306123c34a071794a594c8b9e0c1369a29cc0134ff8ce3be65a0"
        "ae94a39b657cf437d07e61d808f9760f122b52a8c13a918eadb09141d47fd116d54b7a"
        "77ad73982358a012deba358ea980481f8f1a29dc80281c498cbceea2b39052d8d3854b"
        "f4d5581c061867d8c15fed0d5998dbb71582b28e8a407165870557a10a030152f1c783"
        "702f63e44262e9cbe02f196fb88ac06ee390b03b5f69d363ef58e61b1b1d6fa1799e99"
        "1603494ef6423fe1425669044b084f101c8df097ceaff982d2ddc745ea02f7152839c5"
        "1fd8ecfab13ec8923a58d8303487e2f0f7cf28c792917dbf6ddb8a06cd841e6b526a88"
        "e504d5711e2c90fec6f6a2237510679976e050d4dd8429e797c835163e916ed719fabf"
        "e996f1b47e2ed4ed1f5bb5ad863297e19c1fb23aae22d68f550addedb9b707ea1f4a5d"
        "cf894ee2368859e84e4c62e2079212b924a1cf3a65fcd15e179ebfe9ac99efdbf37681"
        "6fe8af264c2c034d3245af68452cb152fac42442067557edb29f6908479bc4bf5f911e"
        "54c54873c1ed8355dc12e0d137d2784e88f6c0c13cbf165c50b6fec37a1ee72988824d"
        "6b39c0af1dd55f7f52fdff3e072e26f57b47c9f292c254"
    ),
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
    //   profile: "OUTSIDER"
    //   user_realm_id: ext(2, hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"))
    //   user_realm_key: hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a")
    //   local_symkey: hex!("125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4")
    &hex!(
        "b88f78194ced8d5c9aa4047054e6d49306b753d93465ee3b0c72390c958a34b3e9db4f"
        "6355888d7a30b9b493e18ffba20a762d0c5cb0e5b3ed8b9037dbedb80ddf671e4640e4"
        "b928fe257748346771d5eceb61b7d1feb15f281db27f7b40df693930d1d9c4db6183f0"
        "7c0c714573995a7f6d2f77afe395fcf2b4ae7ba2742c402ce8ccb9a02318061c7f4946"
        "0a5662525eebf9f0eab4b4b7523b8a56e843f442454d2926f0d2c414cabe1c60119dfd"
        "9fb150d37717ff65d70b7e1c65ce73d8484729d9922f38b3135c457b66c75c5c81237c"
        "92123279d1e34fb954eed1eb9baedcf6bab2de5f4c3d99952035c36da87258ce27ceb9"
        "8a69bfbbb836dcbc7c6e1391c2679f63aace2e39f955c100a9abaf56f0010b266dc7bb"
        "2e1a2e26510fd42cad8ddbb5f45c1d532224180df37407a6ce2044096548302379130e"
        "9eb995c796dfc49da128154eaf2ee19b7271b496c65dfb0d28547e500c057b09c2c60f"
        "84ae17d753369f91941785ba9e61d7f347084c9f2e6fddfc1cf2b5d0a7c822d2fd7464"
        "b6c36aae97f18d535f31c7a81df7744172f6ca320f3ed2c2464ffe4463d81e9a207afc"
        "48799171f0ded9cebf642991a8ff6a8fa56a5877d31aa4606b3ab35180baae507e1dc6"
        "e7c122c4e53c3aa3db3d65d1e96cbff37f762721521f9f18858bc0046d57040aa9285f"
        "16dbd76c70a867646dcca08749d7f75064a43920cb9020"
    ),
    UserProfile::Outsider,
)]
fn serde_local_device_data_x(#[case] data: &[u8], #[case] initial_profile: UserProfile) {
    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let expected = LocalDevice {
        initial_profile,
        // cspell:disable-next-line
        organization_addr: "parsec3://alice_dev1.example.com:9999/CoolOrg?no_ssl=true&p=xCC-KXZzLOyMqU7tzwqv1BPNFZNj4PrcnmhXLHeh4X2bvQ".parse().unwrap(),
        user_id: "alice".parse().unwrap(),
        device_id: "alice@dev1".parse().unwrap(),
        device_label: "My dev1 machine".parse().unwrap(),
        human_handle: HumanHandle::new("alice@example.com", "Alicey McAliceFace").unwrap(),
        signing_key: SigningKey::from(hex!(
            "d544f66ece9c85d5b80275db9124b5f04bb038081622bed139c1e789c5217400"
        )),
        private_key: PrivateKey::from(hex!(
            "74e860967fd90d063ebd64fb1ba6824c4c010099dd37508b7f2875a5db2ef8c9"
        )),
        user_realm_id: VlobID::from_hex("a4031e8bcdd84df8ae12bd3d05e6e20f").unwrap(),
        user_realm_key: SecretKey::from(hex!(
            "26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a"
        )),
        local_symkey: SecretKey::from(hex!(
            "125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4"
        )),
        time_provider: TimeProvider::default(),
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

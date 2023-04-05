// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use rstest::rstest;

use libparsec_crypto::SecretKey;

use crate::{
    fixtures::{bob, Device},
    InviteDeviceConfirmation, InviteDeviceData, InviteUserConfirmation, InviteUserData,
    UserProfile,
};

#[rstest]
#[case::normal(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_user_data"
    //   requested_device_label: "My dev1 machine"
    //   requested_human_handle: ("bob@example.com", "Boby McBobFace")
    //   public_key: <bob.public_key as bytes>
    //   verify_key: <bob.verify_key as bytes>
    &hex!("2802b8a233ad8ec183bb7060b81432fab2f7786ce1a9f774798f3c793bed75fc9ac654056ba0"
    "c42786dd9f95b4eeaeb9ba51fe3e799d93f26f8de11f47bfec8067d844d9d3cc1a4665ad94c0189060"
    "0c1a29b2c058ede68f50c872a3762fb843a5516283d70565bb5f089eac347da2524af11775efa56189"
    "eba5bdbc3bffca52a8e948a3502419f0694afa481f7f1b8beb149b09182eb258d4ce95dedb6bed0c90"
    "0dacd1cd3fec67637721351cd449e7093477e05e3510fa01cefce2d31af8df499e54d9e15e136b4546"
    "57b3bf104fb7bc3e0d1bd170a1c9cba023bd19693e4ae3c115bfbf01d8ce6fbb414e5db5e752bf05c2"
    "0a"),
    true,
    true,
)]
#[case::no_device_label(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_user_data"
    //   requested_device_label: None
    //   requested_human_handle: ("bob@example.com", "Boby McBobFace")
    //   public_key: <bob.public_key as bytes>
    //   verify_key: <bob.verify_key as bytes>
    &hex!("d46540539bf4b402de092350dc0a5c68aafba95138237941ade1a6eb4a3dd2f708d37938d62f"
    "441240c3a7886303b28f8e8c6d813d685b64b631a701b9134d118e48b84c535ff27bed64cfe7e899ac"
    "875501e8f636ae32975851d2bd878008477f3767c18c51ee54cc781cdbf038c62f6ca72ff028743dd9"
    "22ccd8516e1d2970a73fb0ed034a47e4f1ae83d1caa416854680e2cadf21e4eaddc7c0e4a0d9b077a1"
    "9b16dbb7cd21dac5db04b0f32c0a38b96ea37fb0c6b0f9e47fb73f8783793620f9b6835b417a4e7ae4"
    "54c9b770489fbee2ae4b7a40e5ae28f6c1e66f8652a111c97d275441"),
    false,
    true,
)]
#[case::no_human_handle(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_user_data"
    //   requested_device_label: "My dev1 machine"
    //   requested_human_handle: None
    //   public_key: <bob.public_key as bytes>
    //   verify_key: <bob.verify_key as bytes>
    &hex!("dec9cd353582772c1f9cde21eb587a812f08e9bab16708f6440f168e405225aa442394fdcd78"
    "14087a36c0260db468355752b571169752701aefecb745f3436e46f093f12f7d96f048e3330c2338e5"
    "0b553d6e38c6b798e4323daddaffd2d3112872c2528bcab66e2354fb5ce2379fd79063ff9ae885a681"
    "392738c38b13fc06cad8bb6fb231a58d82404fb8448b6d0aaec66ea1e77ccd85f9825362567aab5a76"
    "933f5894caf556ff0090e1891959133a2b4b4915e76bf3366e711678487dcc0ab2c01eaa86aa01ffcf"
    "e839a4df38d132ee3ecee70e"),
    true,
    false,
)]
#[case::no_device_label_no_human_handle(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_user_data"
    //   requested_device_label: None
    //   requested_human_handle: None
    //   public_key: <bob.public_key as bytes>
    //   verify_key: <bob.verify_key as bytes>
    &hex!("0c121c9f7bb1125024c7f9b70f1bbcfd26861fdac043cbc28b00660562f72d46b7dd8f565dc1"
    "85706304a61827495214bdcbfe94c569ab296363f399089a9f20f331fd47a395e591d03e1609e4ca85"
    "06f37f39afa58cc004f6d23e4671bb2df16cb92e7d87787e392f57b392e0ffe487c88758cbc975f408"
    "285df15eb06da7a2bb66b7380c3d89b7ea1b070d0404eea1137330a2f38323c2571294bf9d6b2a2e41"
    "599e45d2ae9d807b0dda2679d4b1a17262015ef76cac6fbbb7cb1f57cb9605ffdee7b79ea7510d"),
    false,
    false,
)]
fn serde_invite_user_data(
    bob: &Device,
    #[case] encrypted: &[u8],
    #[case] with_device_label: bool,
    #[case] with_human_handle: bool,
) {
    let expected = InviteUserData {
        requested_device_label: if with_device_label {
            bob.device_label.to_owned()
        } else {
            None
        },
        requested_human_handle: if with_human_handle {
            bob.human_handle.to_owned()
        } else {
            None
        },
        public_key: bob.public_key(),
        verify_key: bob.verify_key(),
    };

    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let data = InviteUserData::decrypt_and_load(encrypted, &key).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let encrypted2 = data.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to signature and keys order
    let data2 = InviteUserData::decrypt_and_load(&encrypted2, &key).unwrap();
    assert_eq!(data2, expected);
}

#[rstest]
#[case::normal(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_user_confirmation"
    //   device_id: "bob@dev1"
    //   device_label: "My dev1 machine"
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   profil: "STANDARD"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    &hex!("d49ccfc53ca9eeda99cf7218c7b2cd60996885f7b16059f444530a6da146be4cf7a95c638e86"
    "ac42ac446bb03da4dd77aba5b7df397d2584cd07741168094bb0f4927aa4fff893a3580ad289c60e3a"
    "ad326eb7534339fb000553b1f37b52b99d9636f78344ef2f7b5afd621a40f9bca39c4db8fbedfc0936"
    "28eace16c81714710273c772a0ee4c236a7056c688ad49d69d5a6661d98c6dbc1677d8c4a6dff65c01"
    "d5d1a91580e5a45bd638dcf23beffc690cc70382690e9ae76be763ef7c8ef077be06a6d2672aa24537"
    "a59545918ad5dd952394d0f9371271ef8b6d71af2dd087725be430f96b"),
    true,
    true,
)]
#[case::no_device_label(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_user_confirmation"
    //   device_id: "bob@dev1"
    //   device_label: None
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   profil: "STANDARD"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    &hex!("7a4828b511994f08df30927abe6b2db976b08073532eaa7e61787a3e606547b7a18ee51785dd"
    "4c2fdb769c3d79cb4b918d00d15bc305e876958a58f6afc5dc9d206db09ac15fe2bb18dbcc6608b556"
    "68682d9e78100f3a8c1244d8e480e7b37908852c800e25bc1d9f07663b28946e0846403cb379bb9d4a"
    "7fb81dbe6698dc0d483ebb6976ef23c91f7ed6cf42d9bf6eca0bc5d697385d2362e7c5b76596e51583"
    "1b7f136919d41bdc93f34becefa64394a5c35a2a9e54a1a1e0e88b749fb88331a3db7a173685d4945c"
    "a97f7352c1dab993a3700ca2130359ce"),
    false,
    true,
)]
#[case::no_human_handle(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_user_confirmation"
    //   device_id: "bob@dev1"
    //   device_label: "My dev1 machine"
    //   human_handle: None
    //   profil: "STANDARD"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    &hex!("4318022198226d023cc52199b741aa9cfe17df702494e5479f19fdc69bcd6c05461c8000699d"
    "d6987ca95973ceb2efcd99937e632ad492c92953f8969ea5cd0738a727b673b89a28c6bbe7407e5b13"
    "e579969702cc34967a75e3c13ba73ba4b3dd151ae0c36bcfb0e1b05976af597dc18a0b0ee909329b8a"
    "a3db753d174dc9ce1cd2b1477b8e7103c07bb5a82b9b57d07911eb0c7cb3d683ff730a53de699eb3bc"
    "52ed2b48df6074d95737991152bd194160009c0cc43de8007380fd95b389c68c399e0c968f00d8bfab"
    "92"),
    true,
    false,
)]
#[case::no_device_label_no_human_handle(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_user_confirmation"
    //   device_id: "bob@dev1"
    //   device_label: None
    //   human_handle: None
    //   profil: "STANDARD"
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    &hex!("c72484a668c77b542795bdcfee644a4f9d97b36dbeac1be5df92ead9ae3f1eef8d2a8e7c13b5"
    "45b7a4db47c19740907e92dbf70688dda9721fd7702dd089e73ba50e9a11aabd50a7565068df879862"
    "31cc8a586e0e85e9cae691895da116a45d2661929d5fce03bf94072724fbd722fe1c368355db8dacdb"
    "fa1853558a793b11c079fdd8dc0e3007114c1c359a043969e3f6f9764533cfef7da67b908dd1640514"
    "27e17d5559638811c1354a72cfb63cdd0f4bc1e3f1eada310410d55c55"),
    false,
    false,
)]
fn serde_invite_user_confirmation(
    bob: &Device,
    #[case] encrypted: &[u8],
    #[case] with_device_label: bool,
    #[case] with_human_handle: bool,
) {
    let expected = InviteUserConfirmation {
        device_id: bob.device_id.to_owned(),
        device_label: if with_device_label {
            bob.device_label.to_owned()
        } else {
            None
        },
        human_handle: if with_human_handle {
            bob.human_handle.to_owned()
        } else {
            None
        },
        profile: UserProfile::Standard,
        root_verify_key: bob.root_verify_key().to_owned(),
    };

    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let data = InviteUserConfirmation::decrypt_and_load(encrypted, &key).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let encrypted2 = data.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to signature and keys order
    let data2 = InviteUserConfirmation::decrypt_and_load(&encrypted2, &key).unwrap();
    assert_eq!(data2, expected);
}

#[rstest]
#[case::normal(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_device_data"
    //   requested_device_label: "My dev1 machine"
    //   verify_key: <bob.verify_key as bytes>
    &hex!("6ffae1baf9f4eec1ef7b29ec88dbe4e006672a30a48871d04a0f5d63965653a8d69a0fbf3fa7"
    "c6ed1351863080232acc245748e5c970c0fda93b192c6b81e1d8c5126e6960f1e0470deded2f42eaca"
    "16a793198818b85c03914ef3fd780785278feed85a7243e8274a0a98d2fbed573d69b6df31a8d06ac5"
    "7ca2529ad112cb0d214ab9c1886073b3d81a7e5da402fcea57696dbb25c860"),
    true,
)]
#[case::no_device_label(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_device_data"
    //   requested_device_label: None
    //   verify_key: <bob.verify_key as bytes>
    &hex!("b315263f2e0db947066e6d49b211e5f0e5f06e261e86eefb86e1bb5a3aa0e46fe6ccb8dda435"
    "87183d4fbb913cd4b2462d6d280a75d9ed438ec9747e3c39c4071462fe14dfebe9ba6a2c6d7f858d04"
    "1ca168dbd88fb7e2a439c43b92bf7b0d5384d5de96f28a28295e6fcbd499bc25bdd825c17c536c2fec"
    "ca19fee158a8beffc33e898bc9bd60da3b"),
    false,
)]
fn serde_invite_device_data(
    bob: &Device,
    #[case] encrypted: &[u8],
    #[case] with_device_label: bool,
) {
    let expected = InviteDeviceData {
        requested_device_label: if with_device_label {
            bob.device_label.to_owned()
        } else {
            None
        },
        verify_key: bob.verify_key(),
    };

    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let data = InviteDeviceData::decrypt_and_load(encrypted, &key).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let encrypted2 = data.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to signature and keys order
    let data2 = InviteDeviceData::decrypt_and_load(&encrypted2, &key).unwrap();
    assert_eq!(data2, expected);
}

#[rstest]
#[case::normal(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_device_confirmation"
    //   device_id: "bob@dev1"
    //   device_label: "My dev1 machine"
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   profil: "STANDARD"
    //   private_key: <bob.private_key as bytes>
    //   user_manifest_id: ext(2, hex!("71568d41afcb4e2380b3d164ace4fb85"))
    //   user_manifest_key: <bob.user_manifest_key as bytes>
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    &hex!("b703ea4cfb8aed8c7ba7a434316f0acd43389159bbf66c3a2bedd34d943ed97ed22302c76d2b"
    "91790f19ca7a87c4dd5526ea493cf388ff5a492c7343409175f17542f29ccf251bf8bb4b503a78c361"
    "4535225ffcf974069bd036576f31f2ec1cb498e8716f08a56339eb1c784183bf34874963e0f1656d6a"
    "f6859ffcb58e57ab3e73ce6285de981d3b636f7b4dd37070f3f68947ff463d1c9668f30f4e30adc898"
    "741c57871d97975be07da4639abad4249e461fed36635bd9240b1bbca9317d239164426eed3d03f861"
    "20ecfef81d5c03e318988311db64628e01560546bec8a48633b6024cd417d49dfff774de638d1a14fb"
    "133d54a69d6c21238417dea12b4f8f8db2fc8d99b20e62c49a6b7efc22b9e4cd2d00dacd66e0cf57c1"
    "2ca375a2e2853794f294af7e7858832b66ec128713ee3c2310fd25d3b1e2c14622d103c9ce636025b4"
    "a61ed7ee1d5ac9817b14955b99f5b9b93c65d300"),
    true,
    true,
)]
#[case::no_device_label(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_device_confirmation"
    //   device_id: "bob@dev1"
    //   device_label: None
    //   human_handle: ("bob@example.com", "Boby McBobFace")
    //   profil: "STANDARD"
    //   private_key: <bob.private_key as bytes>
    //   user_manifest_id: ext(2, hex!("71568d41afcb4e2380b3d164ace4fb85"))
    //   user_manifest_key: <bob.user_manifest_key as bytes>
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    &hex!("042e6d46ab119cf0bc7bdb528e1d49b3c5f515ff69c30531f5163b88f2b025d911cbfde692d7"
    "c00c5bd6262b848cc9ed68247145a55dc0927967b21db6b039e9190f07341610d828a2e2454be2ba6a"
    "51525fb74b960abbef9ed31f7fd97b973bddde76728b9769d56fce58cacde7a9d83f21980348361588"
    "09a4b76d2203a2cbf6741b1c52948a480cdf75f10bc63088bc6623bdab1bbcec5caec4f5b0e4d584c0"
    "5612e7226c66cc71136916376385349541060ffbbe1eea8164721e7af5dced0d62f93a0ad625d6fa65"
    "e45b9b12d1529159ab9b32917c3b8934aab961aeb0c6bdf3b6465f79e0e038fe287d7119cc33e44c35"
    "9c79e2fab6787e9118be9fb401653141d7f7f37dff49c3eca91a6394c5e684a411f787051ee44e19c9"
    "6bce3018ead799acbed874550558ff738993cb24c8f0d3e2a29c12a3eb08089cb5c1dcfd37240a7d2f"
    "0b84d941cd51"),
    false,
    true,
)]
#[case::no_human_handle(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_device_confirmation"
    //   device_id: "bob@dev1"
    //   device_label: "My dev1 machine"
    //   human_handle: None
    //   profil: "STANDARD"
    //   private_key: <bob.private_key as bytes>
    //   user_manifest_id: ext(2, hex!("71568d41afcb4e2380b3d164ace4fb85"))
    //   user_manifest_key: <bob.user_manifest_key as bytes>
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    &hex!("3c2b77f02a13cb153241d12b5cccaac0ca545e601be43f090cdaa1352243f5be92a98f923d32"
    "883400a5bbf7fa7bfcec2813f1a97ce57e29ff5c49560c8f543197fa952ed326bb4918221ed2e4aa0b"
    "6da866c6df63616e55ace92ee1938ee1fdbc3d5b5348e0e37abe9755d7d9d544f01a29e5f91927dd52"
    "9ef73221f7960b425993c70c672fb69d09099b943e64b5a596b2f97dcb8463f449859d0ab2299ed372"
    "c5fd5df8fcc1b53b084f43d6e0b61ba6606898580d50da33f54c639ebbf2d6709c010b2e69a673a3e0"
    "b301231039db209ea5f09f68491e775509d992872ff96258b7a8255d40dbd4bf6634c221db5610ae10"
    "99d5024ec20fe336f1e08f3b474482325440d874f20c23f626f4a86c8677198d63af0aa3e2b16038ed"
    "22986f38b49396506cfe166fc5df05a5fe87aec848ed741d73a29cd916a4c2dc"),
    true,
    false,
)]
#[case::no_device_label_no_human_handle(
    // Generated from Python implementation (Parsec v2.6.0)
    // Content:
    //   type: "invite_device_confirmation"
    //   device_id: "bob@dev1"
    //   device_label: None
    //   human_handle: None
    //   profil: "STANDARD"
    //   private_key: <bob.private_key as bytes>
    //   user_manifest_id: ext(2, hex!("71568d41afcb4e2380b3d164ace4fb85"))
    //   user_manifest_key: <bob.user_manifest_key as bytes>
    //   root_verify_key: hex!("be2976732cec8ca94eedcf0aafd413cd159363e0fadc9e68572c77a1e17d9bbd")
    &hex!("b051637b1862ecb3056bc428a61ca717869fd3c4962a69a53cb2e386b63c408e1eb3abb00f5b"
    "53200a6a2e523ee3fa2451b28dab41ede97e4f9e802fb760bbbf08ee81b4b26748b9d0d5d42b80ceec"
    "5da58ef5e729091a40618043a3e82d9b6110df07bff00c98a5eaa43570c6cf3fc4f23b11c70544ec49"
    "ac42b7b3b8808291b4d68cb99b24996fd59de9ae49666f4d50a56b5c42fade951b2be24faefafec48b"
    "1f704440ced47fc4c8c839a266e68cdf87e43a224a1dc05fcee335b434f62c43d9b5b39a060258e302"
    "a242936b2200548a435729d414b990dcdebe8ce08df875ba598d1e749f46f9d6d327182bf42cf577ba"
    "c65f56c609cd29448113ee5c2fc963e38fbdb45071fe1c1f1fc8d9254e36416129fffb7c7e65df31c8"
    "113a04761abbfbd1336b9a845ae4c688a4a013"),
    false,
    false,
)]
fn serde_invite_device_confirmation(
    bob: &Device,
    #[case] encrypted: &[u8],
    #[case] with_device_label: bool,
    #[case] with_human_handle: bool,
) {
    let expected = InviteDeviceConfirmation {
        device_id: bob.device_id.to_owned(),
        device_label: if with_device_label {
            bob.device_label.to_owned()
        } else {
            None
        },
        human_handle: if with_human_handle {
            bob.human_handle.to_owned()
        } else {
            None
        },
        profile: UserProfile::Standard,
        private_key: bob.private_key.to_owned(),
        user_manifest_id: bob.user_manifest_id.to_owned(),
        user_manifest_key: bob.user_manifest_key.to_owned(),
        root_verify_key: bob.root_verify_key().to_owned(),
    };

    let key = SecretKey::from(hex!(
        "b1b52e16c1b46ab133c8bf576e82d26c887f1e9deae1af80043a258c36fcabf3"
    ));

    let data = InviteDeviceConfirmation::decrypt_and_load(encrypted, &key).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let encrypted2 = data.dump_and_encrypt(&key);
    // Note we cannot just compare with `data` due to signature and keys order
    let data2 = InviteDeviceConfirmation::decrypt_and_load(&encrypted2, &key).unwrap();
    assert_eq!(data2, expected);
}

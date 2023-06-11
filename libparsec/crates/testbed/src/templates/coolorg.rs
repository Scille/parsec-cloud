// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use hex_literal::hex;

use libparsec_types::prelude::*;

use crate::TestbedTemplate;

pub(crate) fn generate() -> Arc<TestbedTemplate> {
    TestbedTemplate::from_builder("coolorg")
        .bootstrap_organization(
            hex!("b62e7d2a9ed95187975294a1afb1ba345a79e4beb873389366d6c836d20ec5bc"),
            None,
            "alice@dev1",
            Some("Alicey McAliceFace <alice@example.com>"),
            hex!("74e860967fd90d063ebd64fb1ba6824c4c010099dd37508b7f2875a5db2ef8c9"),
            Some("My dev1 machine"),
            hex!("d544f66ece9c85d5b80275db9124b5f04bb038081622bed139c1e789c5217400"),
            hex!("a4031e8bcdd84df8ae12bd3d05e6e20f"),
            hex!("26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a"),
            hex!("323614fc6bd2d300f42d6731059f542f89534cdca11b2cb13d5a9a5a6b19b6ac"),
            "P@ssw0rd.",
        )
        .new_user(
            "alice@dev1",
            "bob@dev1",
            Some("Boby McBobFace <bob@example.com>"),
            hex!("16767ec446f2611f971c36f19c2dc11614d853475ac395d6c1d70ba46d07dd49"),
            Some("My dev1 machine"),
            hex!("85f47472a2c0f30f01b769617db248f3ec8d96a490602a9262f95e9e43432b30"),
            UserProfile::Standard,
            hex!("71568d41afcb4e2380b3d164ace4fb85"),
            hex!("65de53d2c6cd965aa53a1ba5cc7e54b331419e6103466121996fa99a97197a48"),
            hex!("93f25b18491016f20b10dcf4eb7986716d914653d6ab4e778701c13435e6bdf0"),
            "P@ssw0rd.",
        )
        .new_device(
            "alice@dev1",
            "alice@dev2",
            Some("My dev2 machine"),
            hex!("571d726cc5586b4bfd5b20d8af2365cf8bb8c881b4925794e6e38cdcc5ec82ef"),
            hex!("fd64fba009dc635af6662f45753b8c9772ff6e214e203e9d61ce029550e250f7"),
            "P@ssw0rd.",
        )
        .new_device(
            "bob@dev1",
            "bob@dev2",
            Some("My dev2 machine"),
            hex!("0d00fbdeef1cd8b12b6bf40ce88452e9190ed03f2130394930524e3edde192f0"),
            hex!("e4f37fc4a62c7d775c703d23a95b91820dde82ce923a694a1c131f66bc952474"),
            "P@ssw0rd.",
        )
        .new_user(
            "alice@dev1",
            "mallory@dev1",
            None::<HumanHandle>,
            hex!("ef97499baaf2f5ee729b8fc7b6a89ec00d149e3cfb86c52a5db5cb3db5c0d521"),
            None::<DeviceLabel>,
            hex!("f73862c4d73c9a6b6d5123fdfadcbd547cc8e9674c2a6f5a32f67e228d56eb9d"),
            UserProfile::Standard,
            hex!("bfd597b825ac4a0aaeb8a31b08f4f377"),
            hex!("64d129049f977bc12d285b9520469603766161ef793c19a486bd0ee26eb56142"),
            hex!("dd718a6d8eecc42e2cb7a158960d38ff153b43668286e0bae994134db8723b3f"),
            "P@ssw0rd.",
        )
        .finalize()
}

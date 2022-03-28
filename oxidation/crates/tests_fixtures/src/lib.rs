// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use hex_literal::hex;
use rstest::fixture;

use parsec_api_crypto::*;
use parsec_api_types::*;
use parsec_client_types::*;

pub struct Device {
    pub organization_addr: BackendOrganizationAddr,
    pub device_id: DeviceID,
    pub device_label: Option<DeviceLabel>,
    pub human_handle: Option<HumanHandle>,
    pub signing_key: SigningKey,
    pub private_key: PrivateKey,
    pub profile: UserProfile,
    pub user_manifest_id: EntryID,
    pub user_manifest_key: SecretKey,
    pub local_symkey: SecretKey,
}

impl Device {
    pub fn user_id(&self) -> &UserID {
        &self.device_id.user_id
    }

    pub fn organization_id(&self) -> &OrganizationID {
        self.organization_addr.organization_id()
    }

    pub fn verify_key(&self) -> VerifyKey {
        self.signing_key.verify_key()
    }

    pub fn public_key(&self) -> PublicKey {
        self.private_key.public_key()
    }

    pub fn root_verify_key(&self) -> &VerifyKey {
        self.organization_addr.root_verify_key()
    }

    pub fn local_device(&self) -> LocalDevice {
        LocalDevice {
            organization_addr: self.organization_addr.to_owned(),
            device_id: self.device_id.to_owned(),
            device_label: self.device_label.to_owned(),
            human_handle: self.human_handle.to_owned(),
            signing_key: self.signing_key.to_owned(),
            private_key: self.private_key.to_owned(),
            profile: self.profile.to_owned(),
            user_manifest_id: self.user_manifest_id.to_owned(),
            user_manifest_key: self.user_manifest_key.to_owned(),
            local_symkey: self.local_symkey.to_owned(),
        }
    }
}

pub struct Organization {
    pub addr: BackendOrganizationAddr,
    pub signing_key: SigningKey,
}

impl Organization {
    /// Convert `parsec://example.com:9999/...` -> `parsec://alice_dev1.example.com:9999/...`
    /// (useful to filter connections so that some devices end up offline)
    pub fn addr_with_prefixed_host(&self, prefix: &str) -> BackendOrganizationAddr {
        let mut url = self.addr.to_url();
        let custom_host = format!("{}.{}", prefix, url.host().unwrap());
        url.set_host(Some(&custom_host)).unwrap();
        url.as_ref().parse().unwrap()
    }
}

#[fixture]
#[once]
pub fn coolorg() -> Organization {
    Organization {
        addr: "parsec://example.com:9999/CoolOrg?no_ssl=true&rvk=XYUXM4ZM5SGKSTXNZ4FK7VATZUKZGY7A7LOJ42CXFR32DYL5TO6Qssss".parse().unwrap(),
        signing_key: SigningKey::from(hex!("b62e7d2a9ed95187975294a1afb1ba345a79e4beb873389366d6c836d20ec5bc")),
    }
}

#[fixture]
#[once]
pub fn alice(coolorg: &Organization) -> Device {
    Device {
        organization_addr: coolorg.addr_with_prefixed_host("alice_dev1"),
        device_id: "alice@dev1".parse().unwrap(),
        device_label: Some("My dev1 machine".parse().unwrap()),
        human_handle: Some(HumanHandle::new("alice@example.com", "Alicey McAliceFace").unwrap()),
        signing_key: SigningKey::from(hex!(
            "d544f66ece9c85d5b80275db9124b5f04bb038081622bed139c1e789c5217400"
        )),
        private_key: PrivateKey::from(hex!(
            "74e860967fd90d063ebd64fb1ba6824c4c010099dd37508b7f2875a5db2ef8c9"
        )),
        profile: UserProfile::Admin,
        user_manifest_id: "a4031e8bcdd84df8ae12bd3d05e6e20f".parse().unwrap(),
        user_manifest_key: SecretKey::from(hex!(
            "26bf35a98c1e54e90215e154af92a1af2d1142cdd0dba25b990426b0b30b0f9a"
        )),
        local_symkey: SecretKey::from(hex!(
            "125a78618995e2e0f9a19bc8617083c809c03deb5457d5b82df5bcaec9966cd4"
        )),
    }
}

#[fixture]
#[once]
pub fn bob(coolorg: &Organization) -> Device {
    Device {
        organization_addr: coolorg.addr_with_prefixed_host("bob_dev1"),
        device_id: "bob@dev1".parse().unwrap(),
        device_label: Some("My dev1 machine".parse().unwrap()),
        human_handle: Some(HumanHandle::new("bob@example.com", "Boby McBobFace").unwrap()),
        signing_key: SigningKey::from(hex!(
            "85f47472a2c0f30f01b769617db248f3ec8d96a490602a9262f95e9e43432b30"
        )),
        private_key: PrivateKey::from(hex!(
            "16767ec446f2611f971c36f19c2dc11614d853475ac395d6c1d70ba46d07dd49"
        )),
        profile: UserProfile::Standard,
        user_manifest_id: "71568d41afcb4e2380b3d164ace4fb85".parse().unwrap(),
        user_manifest_key: SecretKey::from(hex!(
            "65de53d2c6cd965aa53a1ba5cc7e54b331419e6103466121996fa99a97197a48"
        )),
        local_symkey: SecretKey::from(hex!(
            "93f25b18491016f20b10dcf4eb7986716d914653d6ab4e778701c13435e6bdf0"
        )),
    }
}

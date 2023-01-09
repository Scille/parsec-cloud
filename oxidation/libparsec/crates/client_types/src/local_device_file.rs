// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::{
    collections::HashSet,
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
};

use libparsec_crypto::SecretKey;
use libparsec_types::{DeviceID, DeviceLabel, HumanHandle, OrganizationID};

use crate::{LocalDevice, LocalDeviceError, LocalDeviceResult, StrPath};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceFilePassword {
    #[serde_as(as = "Bytes")]
    pub salt: Vec<u8>,

    #[serde_as(as = "Bytes")]
    pub ciphertext: Vec<u8>,

    pub human_handle: Option<HumanHandle>,
    pub device_label: Option<DeviceLabel>,

    pub device_id: DeviceID,
    pub organization_id: OrganizationID,
    // Handle legacy device with option
    pub slug: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceFileRecovery {
    #[serde_as(as = "Bytes")]
    pub ciphertext: Vec<u8>,

    pub human_handle: Option<HumanHandle>,
    pub device_label: Option<DeviceLabel>,

    pub device_id: DeviceID,
    pub organization_id: OrganizationID,
    pub slug: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceFileSmartcard {
    #[serde_as(as = "Bytes")]
    pub encrypted_key: Vec<u8>,

    pub certificate_id: String,

    #[serde_as(as = "Option<Bytes>")]
    pub certificate_sha1: Option<Vec<u8>>,

    #[serde_as(as = "Bytes")]
    pub ciphertext: Vec<u8>,

    pub human_handle: Option<HumanHandle>,
    pub device_label: Option<DeviceLabel>,

    pub device_id: DeviceID,
    pub organization_id: OrganizationID,
    pub slug: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum DeviceFile {
    Password(DeviceFilePassword),
    Recovery(DeviceFileRecovery),
    Smartcard(DeviceFileSmartcard),
}

impl DeviceFile {
    pub fn save(&self, key_file_path: &Path) -> LocalDeviceResult<()> {
        if let Some(parent) = key_file_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|_| LocalDeviceError::Access(key_file_path.to_path_buf()))?;
        }

        let data = rmp_serde::to_vec_named(self)
            .map_err(|_| LocalDeviceError::Serialization(key_file_path.to_path_buf()))?;

        std::fs::write(key_file_path, data)
            .map_err(|_| LocalDeviceError::Access(key_file_path.to_path_buf()))
    }

    fn load(key_file_path: &Path) -> LocalDeviceResult<Self> {
        let data = std::fs::read(key_file_path)
            .map_err(|_| LocalDeviceError::Access(key_file_path.to_path_buf()))?;

        rmp_serde::from_slice::<DeviceFile>(&data)
            .map_err(|_| LocalDeviceError::Deserialization(key_file_path.to_path_buf()))
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(Serialize))]
pub struct LegacyDeviceFilePassword {
    #[serde_as(as = "Bytes")]
    pub salt: Vec<u8>,
    #[serde_as(as = "Bytes")]
    pub ciphertext: Vec<u8>,
    pub human_handle: Option<HumanHandle>,
    pub device_label: Option<DeviceLabel>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum LegacyDeviceFile {
    Password(LegacyDeviceFilePassword),
}

impl LegacyDeviceFile {
    pub fn decode(serialized: &[u8]) -> Result<Self, &'static str> {
        rmp_serde::from_slice(serialized).map_err(|_| "Invalid serialization")
    }

    #[cfg(test)]
    pub fn dump(&self) -> Vec<u8> {
        rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceFileType {
    Password,
    Recovery,
    Smartcard,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct AvailableDevice {
    pub key_file_path: StrPath,
    pub organization_id: OrganizationID,
    pub device_id: DeviceID,
    pub human_handle: Option<HumanHandle>,
    pub device_label: Option<DeviceLabel>,
    pub slug: String,
    #[serde(rename = "type")]
    pub ty: DeviceFileType,
}

impl AvailableDevice {
    pub fn user_display(&self) -> &str {
        self.human_handle
            .as_ref()
            .map(|x| x.as_ref())
            .unwrap_or_else(|| self.device_id.user_id().as_ref())
    }

    pub fn short_user_display(&self) -> &str {
        self.human_handle
            .as_ref()
            .map(|hh| hh.label())
            .unwrap_or_else(|| self.device_id.user_id().as_ref())
    }

    pub fn device_display(&self) -> &str {
        self.device_label
            .as_ref()
            .map(|x| x.as_ref())
            .unwrap_or_else(|| self.device_id.device_name().as_ref())
    }

    /// For the legacy device files, the slug is contained in the device filename
    fn load(key_file_path: PathBuf) -> LocalDeviceResult<Self> {
        let (ty, organization_id, device_id, human_handle, device_label, slug) =
            match DeviceFile::load(&key_file_path)? {
                DeviceFile::Password(device) => (
                    DeviceFileType::Password,
                    device.organization_id,
                    device.device_id,
                    device.human_handle,
                    device.device_label,
                    // Handle legacy device
                    match device.slug {
                        Some(slug) => slug,
                        None => {
                            let slug = key_file_path
                                .file_stem()
                                .expect("Unreachable because deserialization succeed")
                                .to_str()
                                .expect("It may be unreachable")
                                .to_string();

                            if LocalDevice::load_slug(&slug).is_err() {
                                return Err(LocalDeviceError::InvalidSlug);
                            }

                            slug
                        }
                    },
                ),
                DeviceFile::Recovery(device) => (
                    DeviceFileType::Recovery,
                    device.organization_id,
                    device.device_id,
                    device.human_handle,
                    device.device_label,
                    device.slug,
                ),
                DeviceFile::Smartcard(device) => (
                    DeviceFileType::Smartcard,
                    device.organization_id,
                    device.device_id,
                    device.human_handle,
                    device.device_label,
                    device.slug,
                ),
            };

        Ok(Self {
            key_file_path: key_file_path.into(),
            organization_id,
            device_id,
            human_handle,
            device_label,
            slug,
            ty,
        })
    }
}

/// Return the default keyfile path for a given device.
///
/// Note that the filename does not carry any intrinsic meaning.
/// Here, we simply use the slughash to avoid name collision.
pub fn get_default_key_file(config_dir: &Path, device: &LocalDevice) -> PathBuf {
    let devices_dir = config_dir.join("devices");
    let _ = std::fs::create_dir_all(&devices_dir);
    devices_dir.join(device.slughash() + ".keys")
}

fn read_key_file_paths(path: PathBuf) -> LocalDeviceResult<Vec<PathBuf>> {
    let mut key_file_paths = vec![];

    for path in std::fs::read_dir(&path)
        .map_err(|_| LocalDeviceError::Access(path))?
        .filter_map(|path| path.ok())
        .map(|entry| entry.path())
    {
        if path.extension() == Some(OsStr::new("keys")) {
            key_file_paths.push(path)
        } else if path.is_dir() {
            key_file_paths.append(&mut read_key_file_paths(path)?)
        }
    }

    Ok(key_file_paths)
}

pub fn list_available_devices(config_dir: &Path) -> LocalDeviceResult<Vec<AvailableDevice>> {
    let mut list = vec![];
    // Set of seen slugs
    let mut seen = HashSet::new();

    let key_file_paths = config_dir.join("devices");

    // Consider `.keys` files in devices directory
    let mut key_file_paths = read_key_file_paths(key_file_paths)?;

    // Sort paths so the discovery order is deterministic
    // In the case of duplicate files, that means only the first discovered device is considered
    key_file_paths.sort();

    for key_file_path in key_file_paths {
        let device = match AvailableDevice::load(key_file_path) {
            // Load the device file
            Ok(device) => device,
            // Ignore invalid files
            Err(_) => continue,
        };

        // Ignore duplicate files
        if seen.contains(&device.slug) {
            continue;
        }

        seen.insert(device.slug.clone());

        list.push(device);
    }

    Ok(list)
}

pub async fn save_recovery_device(
    key_file: &Path,
    device: LocalDevice,
    force: bool,
) -> LocalDeviceResult<String> {
    if File::open(key_file).is_ok() && !force {
        return Err(LocalDeviceError::AlreadyExists(key_file.to_path_buf()));
    }

    let (passphrase, key) = SecretKey::generate_recovery_passphrase();

    let ciphertext = key.encrypt(&device.dump());

    let key_file_content = DeviceFile::Recovery(DeviceFileRecovery {
        ciphertext,
        organization_id: device.organization_id().clone(),
        slug: device.slug(),
        human_handle: device.human_handle,
        device_label: device.device_label,
        device_id: device.device_id,
    });

    key_file_content.save(key_file)?;

    Ok(passphrase)
}

/// TODO: need test (backend_cmds required)
pub async fn load_recovery_device(
    key_file: &Path,
    passphrase: &str,
) -> LocalDeviceResult<LocalDevice> {
    let ciphertext =
        std::fs::read(key_file).map_err(|_| LocalDeviceError::Access(key_file.to_path_buf()))?;
    let data = rmp_serde::from_slice::<DeviceFile>(&ciphertext)
        .map_err(|_| LocalDeviceError::Deserialization(key_file.to_path_buf()))?;

    let device = match data {
        DeviceFile::Recovery(device) => device,
        _ => return Err(LocalDeviceError::Validation(DeviceFileType::Recovery)),
    };

    let key =
        SecretKey::from_recovery_passphrase(passphrase).map_err(LocalDeviceError::CryptoError)?;
    let plaintext = key
        .decrypt(&device.ciphertext)
        .map_err(LocalDeviceError::CryptoError)?;
    LocalDevice::load(&plaintext)
        .map_err(|_| LocalDeviceError::Deserialization(key_file.to_path_buf()))
}

#[cfg(test)]
mod test {
    use crate::{LegacyDeviceFile, LegacyDeviceFilePassword};
    use hex_literal::hex;
    use libparsec_types::{DeviceLabel, HumanHandle};
    use rstest::rstest;
    use std::str::FromStr;

    #[rstest]
    #[case::with_device_label_and_human_handle(
    // Generated from Python implementation (Parsec v2.15.0)
    // Content:
    //   type: "password"
    //   ciphertext: hex!(
    //     "92d7b106ad7efbbb5603a41094681bc7c65fff23066e0e0eff8e1db5e626b626f22acd334e5ae3f9"
    //     "92bb822717b6ed29fd706250583a9007b59b87d41e072b92d78775c936a2b418cce1561b46feb8f9"
    //     "9c2216ee8491cdf5b45f54f54cc982fee61c79c95c1b0894d8a266f2f19e5bd133352a3bf59ee160"
    //     "4114a676272a1da1c79ada1461bcb4922d04c88eaff4022ca77e1e7cf77abd20e64e094f86438431"
    //     "8956d44595631df703e5c93ecb4616eddd9e09f52ce551d3b842e4a3bf68731fa4d0a58a32f384eb"
    //     "5f7273573af3b269b1033b644458eef8b6a82695edbb3daa28f7306efa3d74a0a1b217717af9d958"
    //     "0a818c6c81d0dd33e04116b36ff62b8431070c3c7febd4405329dff739f48225d3261a5b2feac583"
    //     "d69993577f3275a43279a59f8b783497a80043409399f93f3a7a3c6ded8ed43078f9842cb709c7db"
    //     "221e94e620eb61943fbb5ed773ec9dd42879f82d69b01c1834eda4791d81f60254d11a9a95a388c9"
    //     "df32630b7e5504ef361d55741c7ced33faf5249deba7b60417ed217411166954feb70da66d7bf144"
    //     "2eae01e4f71648eefd46fb037e6a57482fdea63941a9c9159807c3b04db6d9e195942c96c7b9ae96"
    //     "d1af2b23919e8841be9c85d5305b96149f58e020abb3aff2641370ecb2aeca9084fce184f78ff56a"
    //     "889c9ad80e8b59e441d86c82242e70261c31cde2ba82535a5667cac313c1fb8cb0107e2b1a1f6224"
    //     "559dc9d052be275865"
    //   )
    //   device_label: "My dev1 machine"
    //   human_handle: ["alice@example.com", "Alicey McAliceFace"]
    //   salt: hex!("6f48555e77fd45429cfe26d1dcdd3a8e")
    &hex!(
        "85aa63697068657274657874c5021192d7b106ad7efbbb5603a41094681bc7c65fff23066e"
        "0e0eff8e1db5e626b626f22acd334e5ae3f992bb822717b6ed29fd706250583a9007b59b87"
        "d41e072b92d78775c936a2b418cce1561b46feb8f99c2216ee8491cdf5b45f54f54cc982fe"
        "e61c79c95c1b0894d8a266f2f19e5bd133352a3bf59ee1604114a676272a1da1c79ada1461"
        "bcb4922d04c88eaff4022ca77e1e7cf77abd20e64e094f864384318956d44595631df703e5"
        "c93ecb4616eddd9e09f52ce551d3b842e4a3bf68731fa4d0a58a32f384eb5f7273573af3b2"
        "69b1033b644458eef8b6a82695edbb3daa28f7306efa3d74a0a1b217717af9d9580a818c6c"
        "81d0dd33e04116b36ff62b8431070c3c7febd4405329dff739f48225d3261a5b2feac583d6"
        "9993577f3275a43279a59f8b783497a80043409399f93f3a7a3c6ded8ed43078f9842cb709"
        "c7db221e94e620eb61943fbb5ed773ec9dd42879f82d69b01c1834eda4791d81f60254d11a"
        "9a95a388c9df32630b7e5504ef361d55741c7ced33faf5249deba7b60417ed217411166954"
        "feb70da66d7bf1442eae01e4f71648eefd46fb037e6a57482fdea63941a9c9159807c3b04d"
        "b6d9e195942c96c7b9ae96d1af2b23919e8841be9c85d5305b96149f58e020abb3aff26413"
        "70ecb2aeca9084fce184f78ff56a889c9ad80e8b59e441d86c82242e70261c31cde2ba8253"
        "5a5667cac313c1fb8cb0107e2b1a1f6224559dc9d052be275865ac6465766963655f6c6162"
        "656caf4d792064657631206d616368696e65ac68756d616e5f68616e646c6592b1616c6963"
        "65406578616d706c652e636f6db2416c69636579204d63416c69636546616365a473616c74"
        "c4106f48555e77fd45429cfe26d1dcdd3a8ea474797065a870617373776f7264"
    )[..],
    LegacyDeviceFile::Password(LegacyDeviceFilePassword {
        salt: hex!("6f48555e77fd45429cfe26d1dcdd3a8e").to_vec(),
        ciphertext: hex!(
            "92d7b106ad7efbbb5603a41094681bc7c65fff23066e0e0eff8e1db5e626b626f22acd334e5ae3f9"
            "92bb822717b6ed29fd706250583a9007b59b87d41e072b92d78775c936a2b418cce1561b46feb8f9"
            "9c2216ee8491cdf5b45f54f54cc982fee61c79c95c1b0894d8a266f2f19e5bd133352a3bf59ee160"
            "4114a676272a1da1c79ada1461bcb4922d04c88eaff4022ca77e1e7cf77abd20e64e094f86438431"
            "8956d44595631df703e5c93ecb4616eddd9e09f52ce551d3b842e4a3bf68731fa4d0a58a32f384eb"
            "5f7273573af3b269b1033b644458eef8b6a82695edbb3daa28f7306efa3d74a0a1b217717af9d958"
            "0a818c6c81d0dd33e04116b36ff62b8431070c3c7febd4405329dff739f48225d3261a5b2feac583"
            "d69993577f3275a43279a59f8b783497a80043409399f93f3a7a3c6ded8ed43078f9842cb709c7db"
            "221e94e620eb61943fbb5ed773ec9dd42879f82d69b01c1834eda4791d81f60254d11a9a95a388c9"
            "df32630b7e5504ef361d55741c7ced33faf5249deba7b60417ed217411166954feb70da66d7bf144"
            "2eae01e4f71648eefd46fb037e6a57482fdea63941a9c9159807c3b04db6d9e195942c96c7b9ae96"
            "d1af2b23919e8841be9c85d5305b96149f58e020abb3aff2641370ecb2aeca9084fce184f78ff56a"
            "889c9ad80e8b59e441d86c82242e70261c31cde2ba82535a5667cac313c1fb8cb0107e2b1a1f6224"
            "559dc9d052be275865"
      ).to_vec(),
        human_handle: Some(HumanHandle::new("alice@example.com",  "Alicey McAliceFace").unwrap()),
        device_label: Some(DeviceLabel::from_str("My dev1 machine").unwrap())
    })
)]
    #[case::without_device_label_and_human_handle(
    // Generated from Python implementation (Parsec v2.15.0)
    // Content:
    //   type: "password"
    //   ciphertext: hex!(
    //     "92d7b106ad7efbbb5603a41094681bc7c65fff23066e0e0eff8e1db5e626b626f22acd334e5ae3f9"
    //     "92bb822717b6ed29fd706250583a9007b59b87d41e072b92d78775c936a2b418cce1561b46feb8f9"
    //     "9c2216ee8491cdf5b45f54f54cc982fee61c79c95c1b0894d8a266f2f19e5bd133352a3bf59ee160"
    //     "4114a676272a1da1c79ada1461bcb4922d04c88eaff4022ca77e1e7cf77abd20e64e094f86438431"
    //     "8956d44595631df703e5c93ecb4616eddd9e09f52ce551d3b842e4a3bf68731fa4d0a58a32f384eb"
    //     "5f7273573af3b269b1033b644458eef8b6a82695edbb3daa28f7306efa3d74a0a1b217717af9d958"
    //     "0a818c6c81d0dd33e04116b36ff62b8431070c3c7febd4405329dff739f48225d3261a5b2feac583"
    //     "d69993577f3275a43279a59f8b783497a80043409399f93f3a7a3c6ded8ed43078f9842cb709c7db"
    //     "221e94e620eb61943fbb5ed773ec9dd42879f82d69b01c1834eda4791d81f60254d11a9a95a388c9"
    //     "df32630b7e5504ef361d55741c7ced33faf5249deba7b60417ed217411166954feb70da66d7bf144"
    //     "2eae01e4f71648eefd46fb037e6a57482fdea63941a9c9159807c3b04db6d9e195942c96c7b9ae96"
    //     "d1af2b23919e8841be9c85d5305b96149f58e020abb3aff2641370ecb2aeca9084fce184f78ff56a"
    //     "889c9ad80e8b59e441d86c82242e70261c31cde2ba82535a5667cac313c1fb8cb0107e2b1a1f6224"
    //     "559dc9d052be275865"
    //   )
    //   salt: hex!("6f48555e77fd45429cfe26d1dcdd3a8e")
    &hex!(
        "83aa63697068657274657874c5021192d7b106ad7efbbb5603a41094681bc7c65fff23066e"
        "0e0eff8e1db5e626b626f22acd334e5ae3f992bb822717b6ed29fd706250583a9007b59b87"
        "d41e072b92d78775c936a2b418cce1561b46feb8f99c2216ee8491cdf5b45f54f54cc982fe"
        "e61c79c95c1b0894d8a266f2f19e5bd133352a3bf59ee1604114a676272a1da1c79ada1461"
        "bcb4922d04c88eaff4022ca77e1e7cf77abd20e64e094f864384318956d44595631df703e5"
        "c93ecb4616eddd9e09f52ce551d3b842e4a3bf68731fa4d0a58a32f384eb5f7273573af3b2"
        "69b1033b644458eef8b6a82695edbb3daa28f7306efa3d74a0a1b217717af9d9580a818c6c"
        "81d0dd33e04116b36ff62b8431070c3c7febd4405329dff739f48225d3261a5b2feac583d6"
        "9993577f3275a43279a59f8b783497a80043409399f93f3a7a3c6ded8ed43078f9842cb709"
        "c7db221e94e620eb61943fbb5ed773ec9dd42879f82d69b01c1834eda4791d81f60254d11a"
        "9a95a388c9df32630b7e5504ef361d55741c7ced33faf5249deba7b60417ed217411166954"
        "feb70da66d7bf1442eae01e4f71648eefd46fb037e6a57482fdea63941a9c9159807c3b04d"
        "b6d9e195942c96c7b9ae96d1af2b23919e8841be9c85d5305b96149f58e020abb3aff26413"
        "70ecb2aeca9084fce184f78ff56a889c9ad80e8b59e441d86c82242e70261c31cde2ba8253"
        "5a5667cac313c1fb8cb0107e2b1a1f6224559dc9d052be275865a473616c74c4106f48555e"
        "77fd45429cfe26d1dcdd3a8ea474797065a870617373776f7264"
    )[..],
    LegacyDeviceFile::Password(LegacyDeviceFilePassword {
        salt: hex!("6f48555e77fd45429cfe26d1dcdd3a8e").to_vec(),
        ciphertext: hex!(
            "92d7b106ad7efbbb5603a41094681bc7c65fff23066e0e0eff8e1db5e626b626f22acd334e5ae3f9"
            "92bb822717b6ed29fd706250583a9007b59b87d41e072b92d78775c936a2b418cce1561b46feb8f9"
            "9c2216ee8491cdf5b45f54f54cc982fee61c79c95c1b0894d8a266f2f19e5bd133352a3bf59ee160"
            "4114a676272a1da1c79ada1461bcb4922d04c88eaff4022ca77e1e7cf77abd20e64e094f86438431"
            "8956d44595631df703e5c93ecb4616eddd9e09f52ce551d3b842e4a3bf68731fa4d0a58a32f384eb"
            "5f7273573af3b269b1033b644458eef8b6a82695edbb3daa28f7306efa3d74a0a1b217717af9d958"
            "0a818c6c81d0dd33e04116b36ff62b8431070c3c7febd4405329dff739f48225d3261a5b2feac583"
            "d69993577f3275a43279a59f8b783497a80043409399f93f3a7a3c6ded8ed43078f9842cb709c7db"
            "221e94e620eb61943fbb5ed773ec9dd42879f82d69b01c1834eda4791d81f60254d11a9a95a388c9"
            "df32630b7e5504ef361d55741c7ced33faf5249deba7b60417ed217411166954feb70da66d7bf144"
            "2eae01e4f71648eefd46fb037e6a57482fdea63941a9c9159807c3b04db6d9e195942c96c7b9ae96"
            "d1af2b23919e8841be9c85d5305b96149f58e020abb3aff2641370ecb2aeca9084fce184f78ff56a"
            "889c9ad80e8b59e441d86c82242e70261c31cde2ba82535a5667cac313c1fb8cb0107e2b1a1f6224"
            "559dc9d052be275865"
        ).to_vec(),
        human_handle: None,
        device_label: None,
    })
)]
    fn serde_legacy_device_file(#[case] raw: &[u8], #[case] expected: LegacyDeviceFile) {
        let decoded = LegacyDeviceFile::decode(raw).unwrap();
        assert_eq!(decoded, expected);

        // Roundtrip
        let roundtrip_raw = decoded.dump();
        assert_eq!(LegacyDeviceFile::decode(&roundtrip_raw).unwrap(), expected);
    }
}

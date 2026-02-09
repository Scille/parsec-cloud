// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use hex_literal::hex;
use paste::paste;
use std::{
    collections::{HashMap, HashSet},
    num::{NonZeroU64, NonZeroU8},
    sync::Arc,
};

use libparsec_types::prelude::*;

use super::events::*;
use super::TestbedTemplate;

// The size must be greater than `512` bits, because it's not enough to
// encrypt the secret key size of `32` bytes long in our algorithm.
const SEQUESTER_AUTHORITY_SIGNING_KEY_DER_1024: &[u8] = &hex!(
    "30820276020100300d06092a864886f70d0101010500048202603082025c0201000281"
    "8100b2dc00a3c3b5c689b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf62"
    "4400256cc660d5084782738a3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79b"
    "d09da19162dcc083662269b7b62cb38582f8a30219047b087c11b60184b0493e0c1c8b"
    "1d10f9d7e6a2eb5aff66f7ee18303195f3bcc72ab57207ebfd0203010001028180417d"
    "044ef20dd09001a409cac5e4e0f82d84cb64f8cd6e30d1212e9df703647fde7eff7eb4"
    "813e5b4218cccef93e0b947ac1adbb626da9622a6f89afd55c8ac8bb0a0f1832b2520f"
    "c1d92ac320180b9657a8b1598e6701119d8f77f5285ac5703f4c0a93e1e5ebe6ec179b"
    "ccff62495e7734d5899d86d2dcbdeb648923d29b11024100eb2b5d5ad5bc28cae16545"
    "05c16a2b0d82cfa237f8f10f70e5e7a3333028aade5de4f9e2b3e8a9f924a0538f7119"
    "e088f3c11f1258319c59da03d637a324c663024100c2b3c49145dfd7930ac7f5681afa"
    "43b7b18f697163469c7ab9a6ca8e12168eab1a33cc4e73b53f4509c48052a31ff289e7"
    "357a54f88e28ee543040009976621f024026b608b3ff22ee04177e38126e782f8615d6"
    "5ff99ebcefb1c1e69372c5a6ac19d692ee9f66c611d4b536bf0a89af9cca6e7587cbd9"
    "40b160090740a7ffeef9c902410093566a77ecc29965e290b2bb173f2fa380b0a00078"
    "39e50c52154fcef70d2ee5782c9e7cf7bebea445e1f7a1916409ac25d5283fc8dffb45"
    "6f5c1bf2d82ee7cd024018b8c579f32bbd74cc1f10c786e1e0e192526c9e4134c65bfc"
    "76799b82900adf467a5c7fb3164eb775650abaae08500bc6691e60c8575b5a5abf4f21"
    "56911c4e"
);

// The size must be greater than `512` bits, because it's not enough to
// encrypt the secret key size of `32` bytes long in our algorithm.
const SEQUESTER_AUTHORITY_VERIFY_KEY_DER_1024: &[u8] = &hex!(
    "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5"
    "c689b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d508"
    "4782738a3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083"
    "662269b7b62cb38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5a"
    "ff66f7ee18303195f3bcc72ab57207ebfd0203010001"
);

type SequesterServiceIdentityType = ([u8; 16], &'static str, &'static [u8], &'static [u8]);
const SEQUESTER_SERVICE_IDENTITIES: &[SequesterServiceIdentityType] = &[
    // (SequesterID, SequesterLabel, PrivateDer1024, PublicDer1024)
    (
        hex!("00000000000000000000000000000001"),
        "Sequester Service 1",
        &hex!(
            "30820278020100300d06092a864886f70d0101010500048202623082025e0201000281"
            "8100d89038b4ff4684f7f55baaf28976196c6f8521812cac53e00e12e0c094aa0c1a3e"
            "1ae52ee62d19001e626b085dec126a2c9dbbf4a98ffc67e73c9afe0025c54e37139938"
            "dfdf996a3cf9f983f06d09c3464bc49351f5dc4e1c3c55fcb0e094e52c34bc0c0e40ae"
            "c9758fe0c2e8352ef7782e51084c585b17574dc6a84c9a3777020301000102818100d5"
            "d652a82608b3f3434aa899a432011899659397c14cc7e54d0046fce1cea6f4a2ae5bee"
            "db495c8e497254cd86303c5eaa7f75c0384dfca57c26d3c44ad100e84fe9e61e93f1f9"
            "78263a242c2db9865629659ee9ff2ebeae5f6c724ac5d0d6e09f15a1ecce286f5714f5"
            "93fdf5d448849fa51ea26e82d01b40298d1c549677c1024100ff3041efb19845969b01"
            "2ed22111501525adf797a1e6e50b1fa7498ea2f73e34964abfc6ebf9813b842c884533"
            "6fd1081e38180bccfc6a6b9a143f65c1ceb61d024100d940852c67973fa897bd91abf2"
            "976dca2b23e4470c87de8e590bd9199e0594e7604f31047a148d19f7cb4e28ea06895a"
            "31b2e1f4d17f1324a2155d96876ddfa3024100a3883b58ed88555ff42947e5e4c70178"
            "ebd2964e17b4ada6f93bed097929d43542f2d7ff140daa61878c3a2f8e8ce379be53d8"
            "2507d1e228de6e874a206ee58902410081cb2c451536a58fa1e85dc96ecbcd8a053012"
            "47e8529c424b2ceaed851d2c92f75518a1e615b51f188ddc0a5196ca249aa096a25f2f"
            "1e4eba8f2d34ab80972502404b7d9beb0f9233d534697dc9fb30a2fe34196302bc995f"
            "fefaab7d835917563897a3dd016c3b051b2bcce41679b53417b2ee991512aaffc0b1f5"
            "09b1ec0163c4"
        ),
        &hex!(
            "30819f300d06092a864886f70d010101050003818d0030818902818100d89038b4ff46"
            "84f7f55baaf28976196c6f8521812cac53e00e12e0c094aa0c1a3e1ae52ee62d19001e"
            "626b085dec126a2c9dbbf4a98ffc67e73c9afe0025c54e37139938dfdf996a3cf9f983"
            "f06d09c3464bc49351f5dc4e1c3c55fcb0e094e52c34bc0c0e40aec9758fe0c2e8352e"
            "f7782e51084c585b17574dc6a84c9a37770203010001"
        ),
    ),
    (
        hex!("00000000000000000000000000000002"),
        "Sequester Service 2",
        &hex!(
            "30820276020100300d06092a864886f70d0101010500048202603082025c0201000281"
            "8100bddb4967273214d11d92b44ff4825f34dee8c1733cb2462156c4690d6b6350dc17"
            "ba9c18120e7fe272a2febd92d238a856ff0a3f43aed200803488cf524e8cdbe025036f"
            "8f5b66059f79f9e774ce4e28962889cbdaa065029620fd744420874cc18241c83e6ec9"
            "7a061ba5c4a1726514a84e040a2da0664f468a297094981b55020301000102818100b0"
            "2be4900889eb51243cf67e5ee7ff0a6371ddd85dd11b4c62643d0b0bc40bcb3f6594e4"
            "ad14b14c628da70e284853f4b94d4da7e8d936daea1c557af3e819c16e62a2f2e80f08"
            "ad11f73a19e19dd6aa05efd8023900600a4a1752713b4f307fc393698e28daa0d897af"
            "cebc48bc5ce586558c362d3eac23fd0e8d951568d901024100f53f32f0a713ebec9469"
            "5928b8f84969775e6db76e460aea53a7c55240a32e899c9c93ed655a74a2ee93ddb528"
            "8eaf0c51758b3f1b97e8719fcaf419e5a81815024100c62e5a72759e1edede21519cae"
            "e9e418e7a4d9bb48ccd521841da85bf292d671f0abfc52f8de00d3e83df27373fc9cd0"
            "2c664df2e666a9e0ff46fba273d3864102400e916b803a1c87d62cd6b9321f12b8a1a8"
            "3296c7dfa80f7bedee385d73737e0f349b647a249c23f38dc280d80309bec2379088ed"
            "fdc09f512891d0c03d11282502402debfbc55dffbfc380d153a9ecd601a6cbb6654582"
            "7f043bf9f32d59f1019973598360ba91345018ca4de06a644bfac23a279294605315b6"
            "2f30f18a1930c081024041dd779db04b255fc5f999a22aa21bee3299c7d05bf1523904"
            "24fea269dde53426162702ffb636531161440122d2773e5e5a702a105a0a5c90d7f8ab"
            "9db7b141"
        ),
        &hex!(
            "30819f300d06092a864886f70d010101050003818d0030818902818100bddb49672732"
            "14d11d92b44ff4825f34dee8c1733cb2462156c4690d6b6350dc17ba9c18120e7fe272"
            "a2febd92d238a856ff0a3f43aed200803488cf524e8cdbe025036f8f5b66059f79f9e7"
            "74ce4e28962889cbdaa065029620fd744420874cc18241c83e6ec97a061ba5c4a17265"
            "14a84e040a2da0664f468a297094981b550203010001"
        ),
    ),
    (
        hex!("00000000000000000000000000000003"),
        "Sequester Service 3",
        &hex!(
            "30820276020100300d06092a864886f70d0101010500048202603082025c0201000281"
            "8100c97a27cd1622de2260f2a2eb2ca0decfe884eaf87b95b5ee0a553c8031939daf20"
            "68ab00739b8e8963e9bb3762494f1c064044517780ca97c3804931b3dcb6356663ffd1"
            "3ce2b4fb7aa514f72796bf50933eddaafa2fe9e2a78cd4fe607504950d52f36e60fb00"
            "3d27196d50352a7f43f3f6488109e0ad2e5a9e9fafe1f90d4d02030100010281800ae3"
            "953f1a512c1c438d198d084e717c5f1ebfec4a119f518c316b21aa8c45db6f2ef8feff"
            "408b0595e6cdfd824c60002dbe4f72efb8803a8f906164544a3b76b2d267201b7abfc6"
            "fcd08708a84f1f09a54e59a04be518f7859fd58e6e21490bffbda25c1bfbab1492df49"
            "ebed6f3ba39ee7e736e5d5fb0013d2b6361b835c01024100e750518abc9d7f7520dcc2"
            "4273e1984752827fc89c479fb0d330c2f79b41c7d821df25ed9d7abbf9ceafd3c936e0"
            "f9ccc8c365f3aaa3b1b3ab60a72c06885381024100defaad5f07f122518bfa0f4268d8"
            "9d5e395d2f4ac4701b06b5e7e91970a88716c60a45eebe06304c2a998886e5322bc838"
            "f72d3a2de1ffcfc40019b3de39afcd02405a08cc44691810b5617e2beabbba32908850"
            "1d36d3859965b53e4495260c5ba207c518b93d53b97909772cc324263b74f72bff31f1"
            "d85761acb2293f9ca7518102402b2c1bff47595fccac2e795fe14ef78133d81ffcf8f5"
            "bfb5d7e8941051e8bf67206702cd4bcb84f46a5719c10c855f46c008d39fed1c51dc57"
            "55b1a44ac59e8d0241008d017bc1821bc513d6062d1334da8cc70b23e9e9bdf7d7d95a"
            "95c74af7a8454ac19db6d1419a9e392ddd81d16ab896fe370610685136e4efa9fab318"
            "60d79e59"
        ),
        &hex!(
            "30819f300d06092a864886f70d010101050003818d0030818902818100c97a27cd1622"
            "de2260f2a2eb2ca0decfe884eaf87b95b5ee0a553c8031939daf2068ab00739b8e8963"
            "e9bb3762494f1c064044517780ca97c3804931b3dcb6356663ffd13ce2b4fb7aa514f7"
            "2796bf50933eddaafa2fe9e2a78cd4fe607504950d52f36e60fb003d27196d50352a7f"
            "43f3f6488109e0ad2e5a9e9fafe1f90d4d0203010001"
        ),
    ),
    // "3 sequester services ought to be enough for anybody" 1981 William Henry Gates III
];

#[derive(Clone)]
pub struct TestbedTemplateBuilderCounters {
    current_timestamp: DateTime,
    current_invitation_token: u128,
    current_entry_id: u128,
    current_user_id: u128,
    current_device_id: u128,
    current_256bits_key: [u8; 32],
    current_sequester_service_identity: std::slice::Iter<'static, SequesterServiceIdentityType>,
}

impl Default for TestbedTemplateBuilderCounters {
    fn default() -> Self {
        Self {
            current_timestamp: "2000-01-01T00:00:00Z".parse().unwrap(),
            current_invitation_token: 0xE000_0000_0000_0000_0000_0000_0000_0000,
            current_entry_id: 0xF000_0000_0000_0000_0000_0000_0000_0000,
            current_user_id: 0xA000_0000_0000_0000_0000_0000_0000_0000,
            current_device_id: 0xB000_0000_0000_0000_0000_0000_0000_0000,
            // All our keys start with 0xAA, then grow up
            current_256bits_key: hex!(
                "AA00000000000000000000000000000000000000000000000000000000000000"
            ),
            current_sequester_service_identity: SEQUESTER_SERVICE_IDENTITIES.iter(),
        }
    }
}

impl TestbedTemplateBuilderCounters {
    pub fn next_timestamp(&mut self) -> DateTime {
        self.current_timestamp += Duration::try_days(1).expect("Invalid duration");
        self.current_timestamp
    }
    pub fn set_current_timestamp(&mut self, timestamp: DateTime) {
        assert!(timestamp > self.current_timestamp);
        self.current_timestamp = timestamp;
    }
    pub fn current_timestamp(&self) -> DateTime {
        self.current_timestamp
    }
    pub fn next_user_id(&mut self) -> UserID {
        self.current_user_id += 1;
        self.current_user_id.to_be_bytes().into()
    }
    pub fn next_device_id(&mut self) -> DeviceID {
        self.current_device_id += 1;
        self.current_device_id.to_be_bytes().into()
    }
    pub fn next_entry_id(&mut self) -> VlobID {
        self.current_entry_id += 1;
        self.current_entry_id.to_be_bytes().into()
    }
    pub fn next_invitation_token(&mut self) -> AccessToken {
        self.current_invitation_token += 1;
        self.current_invitation_token.to_be_bytes().into()
    }
    fn next_256bits_key(&mut self) -> &[u8; 32] {
        // 256 keys should be far enough for our needs
        self.current_256bits_key[31] = self.current_256bits_key[31]
            .checked_add(1)
            .expect("No more items, your template is too big :(");
        &self.current_256bits_key
    }
    pub fn next_key_derivation(&mut self) -> KeyDerivation {
        (*self.next_256bits_key()).into()
    }
    pub fn next_secret_key(&mut self) -> SecretKey {
        (*self.next_256bits_key()).into()
    }
    pub fn next_private_key(&mut self) -> PrivateKey {
        (*self.next_256bits_key()).into()
    }
    pub fn next_signing_key(&mut self) -> SigningKey {
        (*self.next_256bits_key()).into()
    }
    pub fn next_sequester_service_identity(
        &mut self,
    ) -> (
        SequesterServiceID,
        String,
        SequesterPrivateKeyDer,
        SequesterPublicKeyDer,
    ) {
        let (id, label, privkey, pubkey) = self
            .current_sequester_service_identity
            .next()
            .expect("No more items, your template is too big :(");
        (
            (*id).into(),
            (*label).to_owned(),
            (*privkey).try_into().unwrap(),
            (*pubkey).try_into().unwrap(),
        )
    }
}

pub struct TestbedTemplateBuilder {
    pub(super) id: &'static str,
    pub(super) events: Vec<TestbedEvent>,
    custom_events_offset: usize,
    // Stuff is useful to transmit arbitrary things (e.g. IDs) from the template to the test
    pub stuff: Vec<(&'static str, &'static (dyn std::any::Any + Send + Sync))>,
    pub counters: TestbedTemplateBuilderCounters,
    pub(super) check_consistency: bool,
}

#[track_caller]
pub(super) fn get_stuff<T>(
    stuff: &[(&'static str, &'static (dyn std::any::Any + Send + Sync))],
    key: &'static str,
) -> &'static T {
    let caller = std::panic::Location::caller();
    let (_, value_any) = stuff.iter().find(|(k, _)| *k == key).unwrap_or_else(|| {
        let available_stuff: Vec<_> = stuff.iter().map(|(k, _)| k).collect();
        panic!(
            "Key `{key}` is not among the stuff (available keys: {available_stuff:?}) (caller: {caller})"
        );
    });
    value_any.downcast_ref::<T>().unwrap_or_else(|| {
        panic!("Key `{key}` is among the stuff, but you got its type wrong :'( (caller: {caller})");
    })
}

impl TestbedTemplateBuilder {
    pub(crate) fn new(id: &'static str) -> Self {
        Self {
            id,
            events: vec![],
            custom_events_offset: 0,
            stuff: vec![],
            counters: TestbedTemplateBuilderCounters::default(),
            check_consistency: true,
        }
    }

    pub(crate) fn new_from_template(id: &'static str, template: &TestbedTemplate) -> Self {
        Self {
            id,
            events: template.events.clone(),
            custom_events_offset: template.events.len(),
            stuff: template.stuff.clone(),
            counters: template.build_counters.clone(),
            check_consistency: true,
        }
    }

    pub fn events(&self) -> &[TestbedEvent] {
        &self.events
    }

    pub fn with_check_consistency_disabled<T>(&mut self, cb: impl FnOnce(&mut Self) -> T) -> T {
        self.check_consistency = false;
        let ret = cb(self);
        self.check_consistency = true;
        ret
    }

    pub fn store_stuff(
        &mut self,
        key: &'static str,
        obj: &(impl std::any::Any + Clone + Send + Sync),
    ) {
        let boxed = Box::new(obj.to_owned());
        assert!(
            self.stuff.iter().all(|(k, _)| *k != key),
            "Key `{key}` is already part of stuff"
        );
        // It's no big deal to leak the data here: the template is kept until the end
        // of the program anyway (and the amount of leak is insignificant).
        // On the other hand it allows to provide the stuff as `&'static Foo` which
        // is convenient.
        self.stuff.push((key, Box::leak(boxed)));
    }

    pub fn get_stuff<T>(&self, key: &'static str) -> &'static T {
        get_stuff(&self.stuff, key)
    }

    /// Remove events you're not happy with (use it in conjunction with `TestbedEnv::customize`)
    ///
    /// You are only able to remove client-side events, as server-side ones depend
    /// on each other (e.g. removing NewUser that creates a device used in ShareRealm).
    pub fn filter_client_storage_events(&mut self, filter: fn(&TestbedEvent) -> bool) {
        // Filtering change the number of events present, so it clashes with
        // `custom_events_offset` field that we use to know which events are custom
        // and hence should be sent to the server.
        // So here we must update `custom_events_offset` along with `events`, which
        // is much easier to do if no custom events have been added yet.
        assert!(
            self.events.len() == self.custom_events_offset,
            "Filtering must be done before custom events are added !"
        );

        let events = std::mem::take(&mut self.events);
        let only_client_side_filter = |e: &TestbedEvent| match e {
            // Only allow client-side events to be filtered
            e @ (TestbedEvent::CertificatesStorageFetchCertificates(_)
            | TestbedEvent::UserStorageFetchUserVlob(_)
            | TestbedEvent::UserStorageFetchRealmCheckpoint(_)
            | TestbedEvent::UserStorageLocalUpdate(_)
            | TestbedEvent::WorkspaceDataStorageFetchFileVlob(_)
            | TestbedEvent::WorkspaceDataStorageFetchFolderVlob(_)
            | TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
            | TestbedEvent::WorkspaceDataStorageLocalFolderManifestCreateOrUpdate(_)
            | TestbedEvent::WorkspaceDataStorageLocalFileManifestCreateOrUpdate(_)
            | TestbedEvent::WorkspaceDataStorageFetchRealmCheckpoint(_)) => filter(e),
            // Server side events are kept no matter what
            _ => true,
        };
        self.events = events.into_iter().filter(only_client_side_filter).collect();
        self.custom_events_offset = self.events.len();
    }

    pub fn finalize(self) -> Arc<TestbedTemplate> {
        Arc::new(TestbedTemplate {
            id: self.id,
            events: self.events,
            custom_events_offset: self.custom_events_offset,
            stuff: self.stuff,
            build_counters: self.counters,
        })
    }

    pub fn current_timestamp(&self) -> DateTime {
        self.counters.current_timestamp
    }

    /// Workspace manifest corresponds to the workspace's root folder manifest
    pub fn create_or_update_workspace_manifest_vlob(
        &mut self,
        device: impl TryInto<DeviceID>,
        realm: impl TryInto<VlobID>,
    ) -> TestbedEventCreateOrUpdateFolderManifestVlobBuilder<'_> {
        let realm: VlobID = realm
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid value for param realm"));
        self.create_or_update_folder_manifest_vlob(device, realm, realm, realm)
    }

    /// Workspace manifest corresponds to the workspace's root folder manifest
    pub fn workspace_data_storage_fetch_workspace_vlob(
        &mut self,
        device: impl TryInto<DeviceID>,
        realm: impl TryInto<VlobID>,
        prevent_sync_pattern: impl TryInto<PreventSyncPattern>,
    ) -> TestbedEventWorkspaceDataStorageFetchFolderVlobBuilder<'_> {
        let realm: VlobID = realm
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid value for param realm"));
        self.workspace_data_storage_fetch_folder_vlob(device, realm, realm, prevent_sync_pattern)
    }

    /// Workspace manifest corresponds to the workspace's root folder manifest
    pub fn workspace_data_storage_local_workspace_manifest_update(
        &mut self,
        device: impl TryInto<DeviceID>,
        realm: impl TryInto<VlobID>,
    ) -> TestbedEventWorkspaceDataStorageLocalFolderManifestCreateOrUpdateBuilder<'_> {
        let realm: VlobID = realm
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid value for param realm"));
        self.workspace_data_storage_local_folder_manifest_create_or_update(
            device, realm, realm, realm,
        )
    }
}

macro_rules! impl_event_builder {
    ($name:ident $(, [$($param:ident: $param_type:ty),+ $(,)?])?) => {
        paste!{
            pub struct [< TestbedEvent $name Builder >]<'a> {
                builder: &'a mut TestbedTemplateBuilder,
            }
            impl<'a> [< TestbedEvent $name Builder >]<'a> {
                /// This is the heavy artillery, try using the `with_...` methods instead.
                pub fn customize(self, cb: impl FnOnce(&mut [< TestbedEvent $name >])) -> Self {
                    let event = match self.builder.events.iter_mut().last() {
                        Some(TestbedEvent::$name(x)) => x,
                        _ => panic!("Missing or unexpected type for last event !")
                    };
                    cb(event);
                    self
                }

                /// Use this to retrieve params auto-generated (typically IDs, keys & timestamps)
                pub fn get_event(&self) -> &[< TestbedEvent $name >] {
                    match self.builder.events.iter().last() {
                        Some(TestbedEvent::$name(x)) => x,
                        _ => panic!("Missing or unexpected type for last event !")
                    }
                }

                pub fn map<T>(&self, cb: impl FnOnce(&[< TestbedEvent $name >]) -> T ) -> T {
                    cb(self.get_event())
                }
            }
            impl TestbedTemplateBuilder {
                /// See [`TestbedEvent`] for more information about the possible events.
                ///
                /// You can also disable consistency check with the method
                /// [`with_check_consistency_disabled`].
                pub fn [< $name:snake >](&mut self $( $(, $param: impl TryInto<$param_type>)* )? ) -> [< TestbedEvent $name Builder >]<'_> {
                    $( $( let $param: $param_type = $param.try_into().unwrap_or_else(|_| panic!(concat!("Invalid value for param ", stringify!($param)))); )* )?
                    let event = [< TestbedEvent $name >]::from_builder(self $( $(, $param)* )? );
                    let event = TestbedEvent::$name(event);
                    self.events.push(event);
                    [< TestbedEvent $name Builder >] { builder: self }
                }
            }
        }
    };
}

macro_rules! impl_customize_field_meth {
    ($field_name:ident, $field_type:ty) => {
        paste! {
            pub fn [< with_ $field_name >](self, $field_name: $field_type) -> Self {
                self.customize(|event| {event.$field_name = $field_name})
            }
        }
    };
}

/*
 * TestbedEventBootstrapOrganizationBuilder
 */

impl_event_builder!(BootstrapOrganization, [first_user: UserID]);

impl TestbedEventBootstrapOrganizationBuilder<'_> {
    pub fn and_set_sequestered_organization(self) -> Self {
        self.customize(|event| {
            event.sequester_authority = Some(TestbedEventBootstrapOrganizationSequesterAuthority {
                signing_key: SEQUESTER_AUTHORITY_SIGNING_KEY_DER_1024.try_into().unwrap(),
                verify_key: SEQUESTER_AUTHORITY_VERIFY_KEY_DER_1024.try_into().unwrap(),
            });
        })
    }
    impl_customize_field_meth!(first_user_id, UserID);
    impl_customize_field_meth!(first_user_human_handle, HumanHandle);
    impl_customize_field_meth!(first_user_first_device_id, DeviceID);
    impl_customize_field_meth!(first_user_first_device_label, DeviceLabel);
    impl_customize_field_meth!(first_user_local_password, String);
}

/*
 * TestbedEventNewSequesterServiceBuilder
 */

impl_event_builder!(NewSequesterService);

impl TestbedEventNewSequesterServiceBuilder<'_> {
    impl_customize_field_meth!(id, SequesterServiceID);
    impl_customize_field_meth!(label, String);
}

/*
 * TestbedEventRevokeSequesterServiceBuilder
 */

impl_event_builder!(RevokeSequesterService, [service_id: SequesterServiceID]);

/*
 * TestbedEventNewUserBuilder
 */

impl_event_builder!(NewUser, [user: UserID]);

impl TestbedEventNewUserBuilder<'_> {
    impl_customize_field_meth!(author, DeviceID);
    impl_customize_field_meth!(human_handle, HumanHandle);
    impl_customize_field_meth!(first_device_id, DeviceID);
    impl_customize_field_meth!(first_device_label, DeviceLabel);
    impl_customize_field_meth!(initial_profile, UserProfile);
    impl_customize_field_meth!(user_realm_id, VlobID);
    impl_customize_field_meth!(local_password, String);
}

/*
 * TestbedEventNewDeviceBuilder
 */

impl_event_builder!(NewDevice, [user: UserID]);

impl TestbedEventNewDeviceBuilder<'_> {
    impl_customize_field_meth!(author, DeviceID);
    impl_customize_field_meth!(device_id, DeviceID);
    impl_customize_field_meth!(device_label, DeviceLabel);
    impl_customize_field_meth!(local_password, String);
}

/*
 * TestbedEventUpdateUserProfileBuilder
 */

impl_event_builder!(UpdateUserProfile, [user: UserID, profile: UserProfile]);

impl TestbedEventUpdateUserProfileBuilder<'_> {
    impl_customize_field_meth!(author, DeviceID);
}

/*
 * TestbedEventRevokeUserBuilder
 */

impl_event_builder!(RevokeUser, [user: UserID]);

impl TestbedEventRevokeUserBuilder<'_> {
    impl_customize_field_meth!(author, DeviceID);
}

/*
 * TestbedEventNewDeviceInvitation
 */

impl_event_builder!(NewDeviceInvitation, [created_by: DeviceID]);

impl TestbedEventNewDeviceInvitationBuilder<'_> {
    impl_customize_field_meth!(created_by, DeviceID);
    impl_customize_field_meth!(created_on, DateTime);
    impl_customize_field_meth!(token, AccessToken);
}

/*
 * TestbedEventNewUserInvitation
 */

impl_event_builder!(NewUserInvitation, [claimer_email: EmailAddress]);

impl TestbedEventNewUserInvitationBuilder<'_> {
    impl_customize_field_meth!(claimer_email, EmailAddress);
    impl_customize_field_meth!(created_by, DeviceID);
    impl_customize_field_meth!(created_on, DateTime);
    impl_customize_field_meth!(token, AccessToken);
}

/*
 * TestbedEventNewShamirRecoveryInvitation
 */

impl_event_builder!(NewShamirRecoveryInvitation, [claimer: UserID]);

impl TestbedEventNewShamirRecoveryInvitationBuilder<'_> {
    impl_customize_field_meth!(claimer, UserID);
    impl_customize_field_meth!(created_by, DeviceID);
    impl_customize_field_meth!(created_on, DateTime);
    impl_customize_field_meth!(token, AccessToken);
}

/*
 * TestbedEventNewRealmBuilder
 */

impl_event_builder!(NewRealm, [first_owner: UserID]);

impl TestbedTemplateBuilder {
    pub fn new_user_realm(
        &mut self,
        user: impl TryInto<UserID>,
    ) -> TestbedEventNewRealmBuilder<'_> {
        let user: UserID = user
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid value for param user"));
        let realm_id =
            // TODO: there is still a check consistency there
            match super::utils::assert_user_exists_and_not_revoked(&self.events, user) {
                TestbedEvent::BootstrapOrganization(x) => x.first_user_user_realm_id,
                TestbedEvent::NewUser(x) => x.user_realm_id,
                _ => unreachable!(),
            };
        let mut event = TestbedEventNewRealm::from_builder(self, user);
        event.realm_id = realm_id;
        self.events.push(TestbedEvent::NewRealm(event));
        TestbedEventNewRealmBuilder { builder: self }
    }
}

impl<'a> TestbedEventNewRealmBuilder<'a> {
    impl_customize_field_meth!(author, DeviceID);

    pub fn then_do_initial_key_rotation(self) -> TestbedEventRotateKeyRealmBuilder<'a> {
        let realm = self.get_event().realm_id;

        let event = TestbedEventRotateKeyRealm::from_builder(self.builder, realm);
        self.builder
            .events
            .push(TestbedEvent::RotateKeyRealm(event));

        TestbedEventRotateKeyRealmBuilder {
            builder: self.builder,
        }
    }

    /// Workspace realm must have done initial key rotation and naming before
    /// being able to be shared.
    pub fn then_do_initial_key_rotation_and_naming(
        self,
        name: impl TryInto<EntryName>,
    ) -> TestbedEventRenameRealmBuilder<'a> {
        let realm = self.get_event().realm_id;

        let event = TestbedEventRotateKeyRealm::from_builder(self.builder, realm);
        self.builder
            .events
            .push(TestbedEvent::RotateKeyRealm(event));

        let event = TestbedEventRenameRealm::from_builder(self.builder, realm, name);
        self.builder.events.push(TestbedEvent::RenameRealm(event));

        TestbedEventRenameRealmBuilder {
            builder: self.builder,
        }
    }

    // pub fn then_share_with(
    //     self,
    //     user: impl TryInto<UserID>,
    //     role: Option<RealmRole>,
    // ) -> TestbedEventShareRealmBuilder<'a> {
    //     let realm = self.get_event().realm_id;

    //     // Must do key rotation before any sharing
    //     let event = TestbedEventRotateKeyRealm::from_builder(self.builder, realm);
    //     self.builder
    //         .events
    //         .push(TestbedEvent::RotateKeyRealm(event));

    //     // Actual sharing
    //     let event = TestbedEventShareRealm::from_builder(self.builder, realm, user, role);
    //     self.builder.events.push(TestbedEvent::ShareRealm(event));

    //     TestbedEventShareRealmBuilder {
    //         builder: self.builder,
    //     }
    // }

    /// Unlike workspace realm, user realm never do key rotation and naming.
    pub fn then_create_initial_user_manifest_vlob(
        self,
    ) -> TestbedEventCreateOrUpdateUserManifestVlobBuilder<'a> {
        let expected = self.get_event().realm_id;
        let user_id =
            super::utils::user_id_from_device_id(&self.builder.events, self.get_event().author);
        let event = TestbedEventCreateOrUpdateUserManifestVlob::from_builder(self.builder, user_id);
        assert_eq!(event.manifest.id, expected);

        self.builder
            .events
            .push(TestbedEvent::CreateOrUpdateUserManifestVlob(event));
        TestbedEventCreateOrUpdateUserManifestVlobBuilder {
            builder: self.builder,
        }
    }

    // pub fn then_create_initial_workspace_manifest_vlob(
    //     self,
    // ) -> TestbedEventCreateOrUpdateWorkspaceManifestVlobBuilder<'a> {
    //     let device = self.get_event().author;
    //     let realm = self.get_event().realm_id;
    //     let event = TestbedEventCreateOrUpdateWorkspaceManifestVlob::from_builder(
    //         self.builder,
    //         device,
    //         realm,
    //     );
    //     assert_eq!(event.manifest.id, realm);

    //     self.builder
    //         .events
    //         .push(TestbedEvent::CreateOrUpdateWorkspaceManifestVlob(event));
    //     TestbedEventCreateOrUpdateWorkspaceManifestVlobBuilder {
    //         builder: self.builder,
    //     }
    // }
}

/*
 * TestbedEventShareRealmBuilder
 */

impl_event_builder!(
    ShareRealm,
    [realm: VlobID, user: UserID, role: Option<RealmRole>]
);

impl<'a> TestbedEventShareRealmBuilder<'a> {
    impl_customize_field_meth!(author, DeviceID);
    impl_customize_field_meth!(key_index, Option<IndexInt>);
    impl_customize_field_meth!(custom_keys_bundle_access, Option<Bytes>);

    pub fn then_also_share_with(
        self,
        user: impl TryInto<UserID>,
        role: Option<RealmRole>,
    ) -> TestbedEventShareRealmBuilder<'a> {
        let realm = self.get_event().realm;
        let event = TestbedEventShareRealm::from_builder(self.builder, realm, user, role);
        self.builder.events.push(TestbedEvent::ShareRealm(event));
        TestbedEventShareRealmBuilder {
            builder: self.builder,
        }
    }
}

/*
 * TestbedEventRenameRealmBuilder
 */

impl_event_builder!(
    RenameRealm,
    [
        realm: VlobID,
        name: EntryName,
    ]
);

impl TestbedEventRenameRealmBuilder<'_> {
    impl_customize_field_meth!(author, DeviceID);
    impl_customize_field_meth!(key_index, IndexInt);
    impl_customize_field_meth!(key, SecretKey);
}

/*
 * TestbedEventRotateKeyRealmBuilder
 */

impl_event_builder!(
    RotateKeyRealm,
    [
        realm: VlobID,
    ]
);

impl TestbedEventRotateKeyRealmBuilder<'_> {
    impl_customize_field_meth!(author, DeviceID);
    impl_customize_field_meth!(key_index, IndexInt);
    impl_customize_field_meth!(hash_algorithm, HashAlgorithm);
    impl_customize_field_meth!(encryption_algorithm, SecretKeyAlgorithm);
    impl_customize_field_meth!(custom_key_canary, Option<Vec<u8>>);
}

/*
 * TestbedEventArchiveRealmBuilder
 */

impl_event_builder!(
    ArchiveRealm,
    [
        realm: VlobID,
        configuration: RealmArchivingConfiguration,
    ]
);

impl TestbedEventArchiveRealmBuilder<'_> {
    impl_customize_field_meth!(author, DeviceID);
}

/*
 * TestbedEventNewShamirRecoveryBuilder
 */

impl_event_builder!(
    NewShamirRecovery,
    [
        user: UserID,
        threshold: NonZeroU8,
        per_recipient_shares: Vec<(UserID, NonZeroU8)>,
        recovery_device: DeviceID,
    ]
);

impl TestbedEventNewShamirRecoveryBuilder<'_> {
    impl_customize_field_meth!(author, DeviceID);
}

/*
 * TestbedEventDeleteShamirRecoveryBuilder
 */

impl_event_builder!(
    DeleteShamirRecovery,
    [
        user: UserID,
    ]
);

impl TestbedEventDeleteShamirRecoveryBuilder<'_> {
    impl_customize_field_meth!(author, DeviceID);
    impl_customize_field_meth!(setup_to_delete_timestamp, DateTime);
    impl_customize_field_meth!(share_recipients, HashSet<UserID>);
}

/*
 * TestbedEventCreateOrUpdateUserManifestVlobBuilder
 */

impl_event_builder!(CreateOrUpdateUserManifestVlob, [user: UserID]);

/*
 * TestbedEventCreateOrUpdateFileManifestVlobBuilder
 */

impl_event_builder!(
    CreateOrUpdateFileManifestVlob,
    [device: DeviceID, realm: VlobID, vlob: Option<VlobID>, parent: Option<VlobID>]
);

impl TestbedEventCreateOrUpdateFileManifestVlobBuilder<'_> {
    impl_customize_field_meth!(key_index, IndexInt);
    impl_customize_field_meth!(key, SecretKey);
}

/*
 * TestbedEventCreateOrUpdateFolderManifestVlobBuilder
 */

impl_event_builder!(
    CreateOrUpdateFolderManifestVlob,
    [device: DeviceID, realm: VlobID, vlob: Option<VlobID>, parent: Option<VlobID>]
);

impl TestbedEventCreateOrUpdateFolderManifestVlobBuilder<'_> {
    pub fn customize_children(
        self,
        children: impl Iterator<Item = (impl TryInto<EntryName>, Option<VlobID>)>,
    ) -> Self {
        self.customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            for (entry_name, change) in children {
                let entry_name = entry_name
                    .try_into()
                    .unwrap_or_else(|_| panic!("Not a valid EntryName"));
                match change {
                    None => {
                        manifest.children.remove(&entry_name);
                    }
                    Some(id) => {
                        manifest.children.insert(entry_name, id);
                    }
                }
            }
        })
    }
}

impl TestbedEventCreateOrUpdateFolderManifestVlobBuilder<'_> {
    impl_customize_field_meth!(key_index, IndexInt);
    impl_customize_field_meth!(key, SecretKey);
}

/*
 * TestbedEventCreateBlockBuilder
 */

impl_event_builder!(
    CreateBlock,
    [device: DeviceID, realm: VlobID, cleartext: Bytes]
);

impl TestbedEventCreateBlockBuilder<'_> {
    pub fn as_block_access(self, offset: SizeInt) -> BlockAccess {
        let event = self.get_event();
        let size = NonZeroU64::new(event.cleartext.len() as u64).expect("block is not empty");
        BlockAccess {
            id: event.block_id,
            offset,
            size,
            digest: HashDigest::from_data(&event.cleartext),
        }
    }
}

/*
 * TestbedEventCreateOpaqueBlockBuilder
 */

impl_event_builder!(
    CreateOpaqueBlock,
    [
        device: DeviceID,
        realm: VlobID,
        block_id: BlockID,
        key_index: IndexInt,
        encrypted_block: Bytes
    ]
);

/*
 * TestbedEventFreezeUser
 */

impl_event_builder!(
    FreezeUser,
    [
        user: UserID,
    ]
);

/*
 * TestbedEventUpdateOrganization
 */

impl_event_builder!(
    UpdateOrganization,
    [
        is_expired: Option<bool>,
        active_users_limit: Option<ActiveUsersLimit>,
        user_profile_outsider_allowed: Option<bool>,
        minimum_archiving_period: Option<u64>,
        tos: Option<HashMap<String, String>>,
    ]
);

/*
 * TestbedEventCertificatesStorageFetchCertificatesBuilder
 */

impl_event_builder!(CertificatesStorageFetchCertificates, [device: DeviceID]);

/*
 * TestbedEventUserStorageFetchUserVlobBuilder
 */

impl_event_builder!(UserStorageFetchUserVlob, [device: DeviceID]);

/*
 * TestbedEventUserStorageFetchRealmCheckpointBuilder
 */

impl_event_builder!(UserStorageFetchRealmCheckpoint, [device: DeviceID]);

/*
 * TestbedEventUserStorageLocalCreateOrUpdateBuilder
 */

impl_event_builder!(UserStorageLocalUpdate, [device: DeviceID]);

impl TestbedEventUserStorageLocalUpdateBuilder<'_> {
    /// If the `workspace_id` is already present, replace its name by a placeholder
    /// (i.e. `role` field is left as-is).
    ///
    /// If the `workspace_id` is not present, add it as a newly created workspace
    /// (i.e. `role` & `name` are both placeholder).
    pub fn add_or_update_placeholder(self, workspace_id: VlobID, name: EntryName) -> Self {
        self.customize(|e| {
            let manifest = Arc::make_mut(&mut e.local_manifest);
            let found = manifest
                .local_workspaces
                .iter_mut()
                .find(|e| e.id == workspace_id);
            match found {
                Some(entry) => {
                    entry.name = name;
                    entry.name_origin = CertificateBasedInfoOrigin::Placeholder;
                }
                None => {
                    manifest
                        .local_workspaces
                        .push(LocalUserManifestWorkspaceEntry {
                            id: workspace_id,
                            name,
                            name_origin: CertificateBasedInfoOrigin::Placeholder,
                            role: RealmRole::Owner,
                            role_origin: CertificateBasedInfoOrigin::Placeholder,
                        });
                }
            }
        })
    }

    pub fn update_local_workspaces_with_fetched_certificates(self) -> Self {
        let device_id = self.get_event().device;
        let user_id = super::utils::user_id_from_device_id(&self.builder.events, device_id);

        let user_realm_id = self
            .builder
            .events
            .iter()
            .find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(e) if e.first_user_id == user_id => {
                    Some(e.first_user_user_realm_id)
                }
                TestbedEvent::NewUser(e) if e.user_id == user_id => Some(e.user_realm_id),
                _ => None,
            })
            .expect("user must exist");

        // First retrieve the fetched certificates if any
        let fetched_up_to = self
            .builder
            .events
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, e)| match e {
                TestbedEvent::CertificatesStorageFetchCertificates(event)
                    if event.device == device_id =>
                {
                    Some(index)
                }
                _ => None,
            });
        let fetched_certifs = match fetched_up_to {
            None => return self,
            Some(index) => self.builder.events.iter().take(index),
        };

        let mut local_workspaces: Vec<LocalUserManifestWorkspaceEntry> = vec![];
        let placeholder_name: EntryName = "<placeholder>".parse().unwrap();
        let mut rename_events = vec![];
        for event in fetched_certifs {
            match event {
                TestbedEvent::NewRealm(event) => {
                    // Ignore user realm as it is not a proper workspace
                    if event.realm_id == user_realm_id {
                        continue;
                    }

                    // If we are not it creator, then this realm has nothing to do with us
                    let author_user_id =
                        super::utils::user_id_from_device_id(&self.builder.events, event.author);
                    if author_user_id != user_id {
                        continue;
                    }

                    // Sanity check: new realm event must be the first about any given realm !
                    assert!(!local_workspaces.iter().any(|w| w.id == event.realm_id));
                    local_workspaces.push(LocalUserManifestWorkspaceEntry {
                        id: event.realm_id,
                        name: placeholder_name.clone(),
                        name_origin: CertificateBasedInfoOrigin::Placeholder,
                        role: RealmRole::Owner,
                        role_origin: CertificateBasedInfoOrigin::Certificate {
                            timestamp: event.timestamp,
                        },
                    });
                }
                TestbedEvent::ShareRealm(event) if event.user == user_id => match event.role {
                    None => {
                        let found = local_workspaces
                            .iter()
                            .position(|entry| entry.id == event.realm);
                        found.map(|index| local_workspaces.swap_remove(index));
                    }
                    Some(role) => {
                        let found = local_workspaces.iter_mut().find(|w| w.id == event.realm);
                        match found {
                            None => {
                                local_workspaces.push(LocalUserManifestWorkspaceEntry {
                                    id: event.realm,
                                    name: placeholder_name.clone(),
                                    name_origin: CertificateBasedInfoOrigin::Placeholder,
                                    role,
                                    role_origin: CertificateBasedInfoOrigin::Certificate {
                                        timestamp: event.timestamp,
                                    },
                                });
                            }
                            Some(entry) => {
                                entry.role = role;
                                entry.role_origin = CertificateBasedInfoOrigin::Certificate {
                                    timestamp: event.timestamp,
                                };
                            }
                        }
                    }
                },
                TestbedEvent::RenameRealm(event) => {
                    rename_events.push(event);
                }
                _ => (),
            }
        }
        // Cannot set the workspace name in the previous loop as this would hide
        // the rename events that occurred before the workspace was shared with us!
        for event in rename_events {
            let found: Option<&mut LocalUserManifestWorkspaceEntry> =
                local_workspaces.iter_mut().find(|w| w.id == event.realm);
            if let Some(entry) = found {
                entry.name = event.name.clone();
                entry.name_origin = CertificateBasedInfoOrigin::Certificate {
                    timestamp: event.timestamp,
                };
            }
        }

        self.customize(|e| {
            let manifest = Arc::make_mut(&mut e.local_manifest);
            manifest.local_workspaces = local_workspaces;
        })
    }
}

/*
 * TestbedEventWorkspaceCacheStorageFetchBlockBuilder
 */

impl_event_builder!(
    WorkspaceCacheStorageFetchBlock,
    [device: DeviceID, realm: VlobID, block: BlockID]
);

/*
 * TestbedEventWorkspaceDataStorageFetchFileVlobBuilder
 */

impl_event_builder!(
    WorkspaceDataStorageFetchFileVlob,
    [device: DeviceID, realm: VlobID, vlob: VlobID]
);

/*
 * TestbedEventWorkspaceDataStorageFetchFolderVlobBuilder
 */

impl_event_builder!(
    WorkspaceDataStorageFetchFolderVlob,
    [
        device: DeviceID,
        realm: VlobID,
        vlob: VlobID,
        prevent_sync_pattern: PreventSyncPattern
    ]
);

/*
 * TestbedEventWorkspaceDataStorageFetchRealmCheckpointBuilder
 */

impl_event_builder!(
    WorkspaceDataStorageFetchRealmCheckpoint,
    [device: DeviceID, realm: VlobID]
);

/*
 * TestbedEventWorkspaceDataStorageChunkCreateBuilder
 */

impl_event_builder!(
    WorkspaceDataStorageChunkCreate,
    [device: DeviceID, realm: VlobID, chunk: Bytes]
);

/*
 * TestbedEventWorkspaceDataStorageLocalFolderManifestCreateOrUpdateBuilder
 */

impl_event_builder!(
    WorkspaceDataStorageLocalFolderManifestCreateOrUpdate,
    [device: DeviceID, realm: VlobID, vlob: Option<VlobID>, parent: Option<VlobID>]
);

impl TestbedEventWorkspaceDataStorageLocalFolderManifestCreateOrUpdateBuilder<'_> {
    pub fn customize_children(
        self,
        children: impl Iterator<Item = (impl TryInto<EntryName>, Option<VlobID>)>,
    ) -> Self {
        self.customize(|e| {
            let manifest = Arc::make_mut(&mut e.local_manifest);
            for (entry_name, change) in children {
                let entry_name = entry_name
                    .try_into()
                    .unwrap_or_else(|_| panic!("Not a valid EntryName"));
                match change {
                    None => {
                        manifest.children.remove(&entry_name);
                    }
                    Some(id) => {
                        manifest.children.insert(entry_name, id);
                    }
                }
            }
        })
    }
}

/*
 * TestbedEventWorkspaceDataStorageLocalFileManifestCreateOrUpdateBuilder
 */

impl_event_builder!(
    WorkspaceDataStorageLocalFileManifestCreateOrUpdate,
    [device: DeviceID, realm: VlobID, vlob: Option<VlobID>, parent: Option<VlobID>]
);

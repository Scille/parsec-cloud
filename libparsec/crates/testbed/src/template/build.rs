// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use hex_literal::hex;
use paste::paste;
use std::collections::HashMap;
use std::num::NonZeroU64;
use std::sync::Arc;

use libparsec_types::prelude::*;

use super::events::*;
use super::TestbedTemplate;

// The size must be greater than `512` bits, because it's not enough to
// encrypt the secret key size of `32` bytes long in our algorithm.
const SEQUESTER_AUTHORITY_SIGNING_KEY_DER_1024: &[u8] = &hex!(
    "30820276020100300d06092a864886f70d0101010500048202603082025c0201"
    "0002818100b2dc00a3c3b5c689b069f3f40c494d2a5be313b1034fbf1dfe0eee"
    "e0f36cfbcf624400256cc660d5084782738a3045d75b584c1943bc04c7123d68"
    "ac0cef253b4ee8d79bd09da19162dcc083662269b7b62cb38582f8a30219047b"
    "087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195f3bcc72a"
    "b57207ebfd0203010001028180417d044ef20dd09001a409cac5e4e0f82d84cb"
    "64f8cd6e30d1212e9df703647fde7eff7eb4813e5b4218cccef93e0b947ac1ad"
    "bb626da9622a6f89afd55c8ac8bb0a0f1832b2520fc1d92ac320180b9657a8b1"
    "598e6701119d8f77f5285ac5703f4c0a93e1e5ebe6ec179bccff62495e7734d5"
    "899d86d2dcbdeb648923d29b11024100eb2b5d5ad5bc28cae1654505c16a2b0d"
    "82cfa237f8f10f70e5e7a3333028aade5de4f9e2b3e8a9f924a0538f7119e088"
    "f3c11f1258319c59da03d637a324c663024100c2b3c49145dfd7930ac7f5681a"
    "fa43b7b18f697163469c7ab9a6ca8e12168eab1a33cc4e73b53f4509c48052a3"
    "1ff289e7357a54f88e28ee543040009976621f024026b608b3ff22ee04177e38"
    "126e782f8615d65ff99ebcefb1c1e69372c5a6ac19d692ee9f66c611d4b536bf"
    "0a89af9cca6e7587cbd940b160090740a7ffeef9c902410093566a77ecc29965"
    "e290b2bb173f2fa380b0a0007839e50c52154fcef70d2ee5782c9e7cf7bebea4"
    "45e1f7a1916409ac25d5283fc8dffb456f5c1bf2d82ee7cd024018b8c579f32b"
    "bd74cc1f10c786e1e0e192526c9e4134c65bfc76799b82900adf467a5c7fb316"
    "4eb775650abaae08500bc6691e60c8575b5a5abf4f2156911c4e"
);

// The size must be greater than `512` bits, because it's not enough to
// encrypt the secret key size of `32` bytes long in our algorithm.
const SEQUESTER_AUTHORITY_VERIFY_KEY_DER_1024: &[u8] = &hex!(
    "30819f300d06092a864886f70d010101050003818d0030818902818100b2dc00a3c3b5c689"
    "b069f3f40c494d2a5be313b1034fbf1dfe0eeee0f36cfbcf624400256cc660d5084782738a"
    "3045d75b584c1943bc04c7123d68ac0cef253b4ee8d79bd09da19162dcc083662269b7b62c"
    "b38582f8a30219047b087c11b60184b0493e0c1c8b1d10f9d7e6a2eb5aff66f7ee18303195"
    "f3bcc72ab57207ebfd0203010001"
);

type SequesterServiceIdentityType = ([u8; 16], &'static str, &'static [u8], &'static [u8]);
const SEQUESTER_SERVICE_IDENTITIES: &[SequesterServiceIdentityType] = &[
    // (SequesterID, SequesterLabel, PrivateDer1024, PublicDer1024)
    (
        hex!("00000000000000000000000000000001"),
        "Sequester Service 1",
        &hex!(
            "30820278020100300d06092a864886f70d0101010500048202623082025e0201"
            "0002818100d89038b4ff4684f7f55baaf28976196c6f8521812cac53e00e12e0"
            "c094aa0c1a3e1ae52ee62d19001e626b085dec126a2c9dbbf4a98ffc67e73c9a"
            "fe0025c54e37139938dfdf996a3cf9f983f06d09c3464bc49351f5dc4e1c3c55"
            "fcb0e094e52c34bc0c0e40aec9758fe0c2e8352ef7782e51084c585b17574dc6"
            "a84c9a3777020301000102818100d5d652a82608b3f3434aa899a43201189965"
            "9397c14cc7e54d0046fce1cea6f4a2ae5beedb495c8e497254cd86303c5eaa7f"
            "75c0384dfca57c26d3c44ad100e84fe9e61e93f1f978263a242c2db986562965"
            "9ee9ff2ebeae5f6c724ac5d0d6e09f15a1ecce286f5714f593fdf5d448849fa5"
            "1ea26e82d01b40298d1c549677c1024100ff3041efb19845969b012ed2211150"
            "1525adf797a1e6e50b1fa7498ea2f73e34964abfc6ebf9813b842c8845336fd1"
            "081e38180bccfc6a6b9a143f65c1ceb61d024100d940852c67973fa897bd91ab"
            "f2976dca2b23e4470c87de8e590bd9199e0594e7604f31047a148d19f7cb4e28"
            "ea06895a31b2e1f4d17f1324a2155d96876ddfa3024100a3883b58ed88555ff4"
            "2947e5e4c70178ebd2964e17b4ada6f93bed097929d43542f2d7ff140daa6187"
            "8c3a2f8e8ce379be53d82507d1e228de6e874a206ee58902410081cb2c451536"
            "a58fa1e85dc96ecbcd8a05301247e8529c424b2ceaed851d2c92f75518a1e615"
            "b51f188ddc0a5196ca249aa096a25f2f1e4eba8f2d34ab80972502404b7d9beb"
            "0f9233d534697dc9fb30a2fe34196302bc995ffefaab7d835917563897a3dd01"
            "6c3b051b2bcce41679b53417b2ee991512aaffc0b1f509b1ec0163c4"
        ),
        &hex!(
            "30819f300d06092a864886f70d010101050003818d0030818902818100d89038"
            "b4ff4684f7f55baaf28976196c6f8521812cac53e00e12e0c094aa0c1a3e1ae5"
            "2ee62d19001e626b085dec126a2c9dbbf4a98ffc67e73c9afe0025c54e371399"
            "38dfdf996a3cf9f983f06d09c3464bc49351f5dc4e1c3c55fcb0e094e52c34bc"
            "0c0e40aec9758fe0c2e8352ef7782e51084c585b17574dc6a84c9a3777020301"
            "0001"
        ),
    ),
    (
        hex!("00000000000000000000000000000002"),
        "Sequester Service 2",
        &hex!(
            "30820276020100300d06092a864886f70d0101010500048202603082025c0201"
            "0002818100bddb4967273214d11d92b44ff4825f34dee8c1733cb2462156c469"
            "0d6b6350dc17ba9c18120e7fe272a2febd92d238a856ff0a3f43aed200803488"
            "cf524e8cdbe025036f8f5b66059f79f9e774ce4e28962889cbdaa065029620fd"
            "744420874cc18241c83e6ec97a061ba5c4a1726514a84e040a2da0664f468a29"
            "7094981b55020301000102818100b02be4900889eb51243cf67e5ee7ff0a6371"
            "ddd85dd11b4c62643d0b0bc40bcb3f6594e4ad14b14c628da70e284853f4b94d"
            "4da7e8d936daea1c557af3e819c16e62a2f2e80f08ad11f73a19e19dd6aa05ef"
            "d8023900600a4a1752713b4f307fc393698e28daa0d897afcebc48bc5ce58655"
            "8c362d3eac23fd0e8d951568d901024100f53f32f0a713ebec94695928b8f849"
            "69775e6db76e460aea53a7c55240a32e899c9c93ed655a74a2ee93ddb5288eaf"
            "0c51758b3f1b97e8719fcaf419e5a81815024100c62e5a72759e1edede21519c"
            "aee9e418e7a4d9bb48ccd521841da85bf292d671f0abfc52f8de00d3e83df273"
            "73fc9cd02c664df2e666a9e0ff46fba273d3864102400e916b803a1c87d62cd6"
            "b9321f12b8a1a83296c7dfa80f7bedee385d73737e0f349b647a249c23f38dc2"
            "80d80309bec2379088edfdc09f512891d0c03d11282502402debfbc55dffbfc3"
            "80d153a9ecd601a6cbb66545827f043bf9f32d59f1019973598360ba91345018"
            "ca4de06a644bfac23a279294605315b62f30f18a1930c081024041dd779db04b"
            "255fc5f999a22aa21bee3299c7d05bf152390424fea269dde53426162702ffb6"
            "36531161440122d2773e5e5a702a105a0a5c90d7f8ab9db7b141"
        ),
        &hex!(
            "30819f300d06092a864886f70d010101050003818d0030818902818100bddb49"
            "67273214d11d92b44ff4825f34dee8c1733cb2462156c4690d6b6350dc17ba9c"
            "18120e7fe272a2febd92d238a856ff0a3f43aed200803488cf524e8cdbe02503"
            "6f8f5b66059f79f9e774ce4e28962889cbdaa065029620fd744420874cc18241"
            "c83e6ec97a061ba5c4a1726514a84e040a2da0664f468a297094981b55020301"
            "0001"
        ),
    ),
    (
        hex!("00000000000000000000000000000003"),
        "Sequester Service 3",
        &hex!(
            "30820276020100300d06092a864886f70d0101010500048202603082025c0201"
            "0002818100c97a27cd1622de2260f2a2eb2ca0decfe884eaf87b95b5ee0a553c"
            "8031939daf2068ab00739b8e8963e9bb3762494f1c064044517780ca97c38049"
            "31b3dcb6356663ffd13ce2b4fb7aa514f72796bf50933eddaafa2fe9e2a78cd4"
            "fe607504950d52f36e60fb003d27196d50352a7f43f3f6488109e0ad2e5a9e9f"
            "afe1f90d4d02030100010281800ae3953f1a512c1c438d198d084e717c5f1ebf"
            "ec4a119f518c316b21aa8c45db6f2ef8feff408b0595e6cdfd824c60002dbe4f"
            "72efb8803a8f906164544a3b76b2d267201b7abfc6fcd08708a84f1f09a54e59"
            "a04be518f7859fd58e6e21490bffbda25c1bfbab1492df49ebed6f3ba39ee7e7"
            "36e5d5fb0013d2b6361b835c01024100e750518abc9d7f7520dcc24273e19847"
            "52827fc89c479fb0d330c2f79b41c7d821df25ed9d7abbf9ceafd3c936e0f9cc"
            "c8c365f3aaa3b1b3ab60a72c06885381024100defaad5f07f122518bfa0f4268"
            "d89d5e395d2f4ac4701b06b5e7e91970a88716c60a45eebe06304c2a998886e5"
            "322bc838f72d3a2de1ffcfc40019b3de39afcd02405a08cc44691810b5617e2b"
            "eabbba329088501d36d3859965b53e4495260c5ba207c518b93d53b97909772c"
            "c324263b74f72bff31f1d85761acb2293f9ca7518102402b2c1bff47595fccac"
            "2e795fe14ef78133d81ffcf8f5bfb5d7e8941051e8bf67206702cd4bcb84f46a"
            "5719c10c855f46c008d39fed1c51dc5755b1a44ac59e8d0241008d017bc1821b"
            "c513d6062d1334da8cc70b23e9e9bdf7d7d95a95c74af7a8454ac19db6d1419a"
            "9e392ddd81d16ab896fe370610685136e4efa9fab31860d79e59"
        ),
        &hex!(
            "30819f300d06092a864886f70d010101050003818d0030818902818100c97a27"
            "cd1622de2260f2a2eb2ca0decfe884eaf87b95b5ee0a553c8031939daf2068ab"
            "00739b8e8963e9bb3762494f1c064044517780ca97c3804931b3dcb6356663ff"
            "d13ce2b4fb7aa514f72796bf50933eddaafa2fe9e2a78cd4fe607504950d52f3"
            "6e60fb003d27196d50352a7f43f3f6488109e0ad2e5a9e9fafe1f90d4d020301"
            "0001"
        ),
    ),
    // "3 sequester services ought to be enough for anybody" 1981 William Henry Gates III
];

#[derive(Clone)]
pub(super) struct TestbedTemplateBuilderCounters {
    current_timestamp: DateTime,
    current_invitation_token: u128,
    current_entry_id: u128,
    current_256bits_key: [u8; 32],
    current_sequester_service_identity: std::slice::Iter<'static, SequesterServiceIdentityType>,
}

impl Default for TestbedTemplateBuilderCounters {
    fn default() -> Self {
        Self {
            current_timestamp: "2000-01-01T00:00:00Z".parse().unwrap(),
            current_invitation_token: 0xE000_0000_0000_0000_0000_0000_0000_0000,
            current_entry_id: 0xF000_0000_0000_0000_0000_0000_0000_0000,
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
        self.current_timestamp += Duration::days(1);
        self.current_timestamp
    }
    pub fn current_timestamp(&self) -> DateTime {
        self.current_timestamp
    }
    pub fn next_entry_id(&mut self) -> VlobID {
        self.current_entry_id += 1;
        self.current_entry_id.to_be_bytes().into()
    }
    pub fn next_invitation_token(&mut self) -> InvitationToken {
        self.current_invitation_token += 1;
        self.current_invitation_token.to_be_bytes().into()
    }
    fn next_256bits_key(&mut self) -> &[u8; 32] {
        // 256 keys should be far enough for our needs
        self.current_256bits_key[31]
            .checked_add(1)
            .expect("No more items, your template is too big :(");
        &self.current_256bits_key
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
    pub(super) allow_server_side_events: bool,
    pub(super) events: Vec<TestbedEvent>,
    // Stuff is useful store provide arbitrary things (e.g. IDs) from the template to the test
    pub stuff: Vec<(&'static str, &'static (dyn std::any::Any + Send + Sync))>,
    pub(super) counters: TestbedTemplateBuilderCounters,
    pub(super) check_consistency: bool,
}

pub(super) fn get_stuff<T>(
    stuff: &[(&'static str, &'static (dyn std::any::Any + Send + Sync))],
    key: &'static str,
) -> &'static T {
    let (_, value_any) = stuff.iter().find(|(k, _)| *k == key).unwrap_or_else(|| {
        let available_stuff: Vec<_> = stuff.iter().map(|(k, _)| k).collect();
        panic!(
            "Key `{}` is not among the stuff (available keys: {:?})",
            key, available_stuff
        );
    });
    value_any.downcast_ref::<T>().unwrap_or_else(|| {
        panic!(
            "Key `{}` is among the stuff, but you got it type wrong :'(",
            key
        );
    })
}

impl TestbedTemplateBuilder {
    pub(crate) fn new(id: &'static str) -> Self {
        Self {
            id,
            allow_server_side_events: true,
            events: vec![],
            stuff: vec![],
            counters: TestbedTemplateBuilderCounters::default(),
            check_consistency: true,
        }
    }

    pub(crate) fn new_from_template(
        id: &'static str,
        template: &TestbedTemplate,
        allow_server_side_events: bool,
    ) -> Self {
        Self {
            id,
            allow_server_side_events,
            events: template.events.clone(),
            stuff: template.stuff.clone(),
            counters: template.build_counters.clone(),
            check_consistency: true,
        }
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
        // It's no big deal to leak the data here: the template is kept until the end
        // of the program anyway (and the amount of leak is negligeable).
        // On the other hand it allows to provide the stuff as `&'static Foo` which
        // is convenient.
        self.stuff.push((key, Box::leak(boxed)));
    }

    pub fn get_stuff<T>(&self, key: &'static str) -> &'static T {
        get_stuff(&self.stuff, key)
    }

    /// Remove events you're not happy with (use it in conjuction with `TestbedEnv::customize`)
    ///
    /// You are only able to remove client-side events, as server-side ones depend
    /// on each other (e.g. removing NewUser that creates a device used in ShareRealm).
    pub fn filter_client_storage_events(&mut self, filter: fn(&TestbedEvent) -> bool) {
        let events = std::mem::take(&mut self.events);
        let only_client_side_filter = |e: &TestbedEvent| match e {
            // Only allow client-side events to be filtered
            e @ (TestbedEvent::CertificatesStorageFetchCertificates(_)
            | TestbedEvent::UserStorageFetchUserVlob(_)
            | TestbedEvent::UserStorageFetchRealmCheckpoint(_)
            | TestbedEvent::UserStorageLocalUpdate(_)
            | TestbedEvent::WorkspaceDataStorageFetchWorkspaceVlob(_)
            | TestbedEvent::WorkspaceDataStorageFetchFileVlob(_)
            | TestbedEvent::WorkspaceDataStorageFetchFolderVlob(_)
            | TestbedEvent::WorkspaceCacheStorageFetchBlock(_)
            | TestbedEvent::WorkspaceDataStorageLocalWorkspaceManifestUpdate(_)
            | TestbedEvent::WorkspaceDataStorageLocalFolderManifestCreateOrUpdate(_)
            | TestbedEvent::WorkspaceDataStorageLocalFileManifestCreateOrUpdate(_)
            | TestbedEvent::WorkspaceDataStorageFetchRealmCheckpoint(_)) => filter(e),
            // Server side events are kept no matter what
            _ => true,
        };
        self.events = events.into_iter().filter(only_client_side_filter).collect();
    }

    pub fn finalize(self) -> Arc<TestbedTemplate> {
        Arc::new(TestbedTemplate {
            id: self.id,
            events: self.events,
            stuff: self.stuff,
            build_counters: self.counters,
        })
    }

    pub fn current_timestamp(&self) -> DateTime {
        self.counters.current_timestamp
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
                    if !self.allow_server_side_events && !event.is_client_side() {
                        panic!("Testbed connects to an actual server, hence server-side events cannot be added: {:?}", event);
                    }
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

impl<'a> TestbedEventBootstrapOrganizationBuilder<'a> {
    pub fn and_set_sequestered_organization(self) -> Self {
        self.customize(|event| {
            event.sequester_authority = Some(TestbedEventBootstrapOrganizationSequesterAuthority {
                signing_key: SEQUESTER_AUTHORITY_SIGNING_KEY_DER_1024.try_into().unwrap(),
                verify_key: SEQUESTER_AUTHORITY_VERIFY_KEY_DER_1024.try_into().unwrap(),
            });
        })
    }
    impl_customize_field_meth!(first_user_device_id, DeviceID);
    impl_customize_field_meth!(first_user_human_handle, HumanHandle);
    impl_customize_field_meth!(first_user_first_device_label, DeviceLabel);
    impl_customize_field_meth!(first_user_local_password, &'static str);
}

/*
 * TestbedEventNewSequesterServiceBuilder
 */

impl_event_builder!(NewSequesterService);

impl<'a> TestbedEventNewSequesterServiceBuilder<'a> {
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

impl<'a> TestbedEventNewUserBuilder<'a> {
    impl_customize_field_meth!(author, DeviceID);
    impl_customize_field_meth!(device_id, DeviceID);
    impl_customize_field_meth!(human_handle, HumanHandle);
    impl_customize_field_meth!(first_device_label, DeviceLabel);
    impl_customize_field_meth!(initial_profile, UserProfile);
    impl_customize_field_meth!(user_realm_id, VlobID);
    impl_customize_field_meth!(local_password, &'static str);
}

/*
 * TestbedEventNewDeviceBuilder
 */

impl_event_builder!(NewDevice, [user: UserID]);

impl<'a> TestbedEventNewDeviceBuilder<'a> {
    impl_customize_field_meth!(author, DeviceID);
    impl_customize_field_meth!(device_label, DeviceLabel);
    impl_customize_field_meth!(local_password, &'static str);
    pub fn with_device_name(self, device_name: DeviceName) -> Self {
        self.customize(|event| {
            event.device_id = DeviceID::new(event.device_id.user_id().to_owned(), device_name);
        })
    }
}

/*
 * TestbedEventUpdateUserProfileBuilder
 */

impl_event_builder!(UpdateUserProfile, [user: UserID, profile: UserProfile]);

impl<'a> TestbedEventUpdateUserProfileBuilder<'a> {
    impl_customize_field_meth!(author, DeviceID);
}

/*
 * TestbedEventRevokeUserBuilder
 */

impl_event_builder!(RevokeUser, [user: UserID]);

impl<'a> TestbedEventRevokeUserBuilder<'a> {
    impl_customize_field_meth!(author, DeviceID);
}

/*
 * TestbedEventNewDeviceInvitation
 */

impl_event_builder!(NewDeviceInvitation, [greeter_user_id: UserID]);

impl<'a> TestbedEventNewDeviceInvitationBuilder<'a> {
    impl_customize_field_meth!(greeter_user_id, UserID);
    impl_customize_field_meth!(created_on, DateTime);
    impl_customize_field_meth!(token, InvitationToken);
}

/*
 * TestbedEventNewUserInvitation
 */

impl_event_builder!(NewUserInvitation, [claimer_email: String]);

impl<'a> TestbedEventNewUserInvitationBuilder<'a> {
    impl_customize_field_meth!(claimer_email, String);
    impl_customize_field_meth!(greeter_user_id, UserID);
    impl_customize_field_meth!(created_on, DateTime);
    impl_customize_field_meth!(token, InvitationToken);
}

/*
 * TestbedEventNewRealmBuilder
 */

impl_event_builder!(NewRealm, [first_owner: UserID]);

impl TestbedTemplateBuilder {
    pub fn new_user_realm(&mut self, user: impl TryInto<UserID>) -> TestbedEventNewRealmBuilder {
        let user: UserID = user
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid value for param user"));
        let realm_id =
            // TODO: there is still a check consistency there
            match super::utils::assert_user_exists_and_not_revoked(&self.events, &user) {
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
        let user = self.get_event().author.user_id().to_owned();
        let expected = self.get_event().realm_id;
        let event = TestbedEventCreateOrUpdateUserManifestVlob::from_builder(self.builder, user);
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
    //     let device = self.get_event().author.clone();
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

impl<'a> TestbedEventRenameRealmBuilder<'a> {
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

impl<'a> TestbedEventRotateKeyRealmBuilder<'a> {
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

impl<'a> TestbedEventArchiveRealmBuilder<'a> {
    impl_customize_field_meth!(author, DeviceID);
}

/*
 * TestbedEventNewShamirRecoveryBuilder
 */

impl_event_builder!(
    NewShamirRecovery,
    [
        user: UserID,
        threshold: NonZeroU64,
        per_recipient_shares: HashMap<UserID, NonZeroU64>,
    ]
);

impl<'a> TestbedEventNewShamirRecoveryBuilder<'a> {
    impl_customize_field_meth!(author, DeviceID);
}

/*
 * TestbedEventCreateOrUpdateUserManifestVlobBuilder
 */

impl_event_builder!(CreateOrUpdateUserManifestVlob, [user: UserID]);

/*
 * TestbedEventCreateOrUpdateWorkspaceManifestVlobBuilder
 */

impl_event_builder!(
    CreateOrUpdateWorkspaceManifestVlob,
    [device: DeviceID, realm: VlobID]
);

impl<'a> TestbedEventCreateOrUpdateWorkspaceManifestVlobBuilder<'a> {
    impl_customize_field_meth!(key_index, IndexInt);
    impl_customize_field_meth!(key, SecretKey);
}

impl<'a> TestbedEventCreateOrUpdateWorkspaceManifestVlobBuilder<'a> {
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

/*
 * TestbedEventCreateOrUpdateFileManifestVlobBuilder
 */

impl_event_builder!(
    CreateOrUpdateFileManifestVlob,
    [device: DeviceID, realm: VlobID, vlob: Option<VlobID>]
);

impl<'a> TestbedEventCreateOrUpdateFileManifestVlobBuilder<'a> {
    impl_customize_field_meth!(key_index, IndexInt);
    impl_customize_field_meth!(key, SecretKey);
}

/*
 * TestbedEventCreateOrUpdateFolderManifestVlobBuilder
 */

impl_event_builder!(
    CreateOrUpdateFolderManifestVlob,
    [device: DeviceID, realm: VlobID, vlob: Option<VlobID>]
);

impl<'a> TestbedEventCreateOrUpdateFolderManifestVlobBuilder<'a> {
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

impl<'a> TestbedEventCreateOrUpdateFolderManifestVlobBuilder<'a> {
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

impl<'a> TestbedEventCreateBlockBuilder<'a> {
    pub fn as_block_access(self, offset: SizeInt) -> BlockAccess {
        let event = self.get_event();
        let size =
            std::num::NonZeroU64::new(event.cleartext.len() as u64).expect("block is not empty");
        BlockAccess {
            id: event.block_id,
            key: event.block_key.clone(),
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
        encrypted_block: Bytes
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

impl<'a> TestbedEventUserStorageLocalUpdateBuilder<'a> {
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
        let device = &self.get_event().device;

        let user_realm_id = self
            .builder
            .events
            .iter()
            .find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(e)
                    if e.first_user_device_id.user_id() == device.user_id() =>
                {
                    Some(e.first_user_user_realm_id)
                }
                TestbedEvent::NewUser(e) if e.device_id.user_id() == device.user_id() => {
                    Some(e.user_realm_id)
                }
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
                    if event.device == *device =>
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
                TestbedEvent::NewRealm(event) if event.author.user_id() == device.user_id() => {
                    // Ignore user realm as it is not a proper workspace
                    if event.realm_id == user_realm_id {
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
                TestbedEvent::ShareRealm(event) if event.user == *device.user_id() => {
                    match event.role {
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
                    }
                }
                TestbedEvent::RenameRealm(event) => {
                    rename_events.push(event);
                }
                _ => (),
            }
        }
        // Cannot set the workspace name in the previous loop as this would hide
        // the rename events that occured before the workspace was shared with us !
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
 * TestbedEventWorkspaceDataStorageFetchWorkspaceVlobBuilder
 */

impl_event_builder!(
    WorkspaceDataStorageFetchWorkspaceVlob,
    [
        device: DeviceID,
        realm: VlobID,
        prevent_sync_pattern: Option<Regex>,
    ]
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
        prevent_sync_pattern: Option<Regex>
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
 * TestbedEventWorkspaceDataStorageLocalWorkspaceManifestCreateOrUpdateBuilder
 */

impl_event_builder!(
    WorkspaceDataStorageLocalWorkspaceManifestUpdate,
    [device: DeviceID, realm: VlobID]
);

impl<'a> TestbedEventWorkspaceDataStorageLocalWorkspaceManifestUpdateBuilder<'a> {
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
 * TestbedEventWorkspaceDataStorageLocalFolderManifestCreateOrUpdateBuilder
 */

impl_event_builder!(
    WorkspaceDataStorageLocalFolderManifestCreateOrUpdate,
    [device: DeviceID, realm: VlobID, vlob: Option<VlobID>]
);

impl<'a> TestbedEventWorkspaceDataStorageLocalFolderManifestCreateOrUpdateBuilder<'a> {
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
    [device: DeviceID, realm: VlobID, vlob: Option<VlobID>]
);

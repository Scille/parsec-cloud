// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use hex_literal::hex;
use paste::paste;
use std::sync::Arc;

use libparsec_types::prelude::*;

use super::events::*;
use super::TestbedTemplate;

const SEQUESTER_AUTHORITY_SIGNING_KEY_DER_512: &[u8] = &hex!(
    "30820156020100300d06092a864886f70d0101010500048201403082013c0201"
    "00024100ee40d70583355ac2eeaa1b9c7a491981bf8d3c88b4547e38788d852c"
    "ef9110bf5218d9e31dcd4536c22c898bd099021d21b8f7414fb534266107b63d"
    "dc79fe930203010001024100bef4eb21f8f2d786eb611df641b0bb27da0e6a59"
    "2b27996ecb78aa27c4ef611a5ac8a310655af7762b5c442fefc478c9f3f41ddd"
    "14490f24913bdb4d1c79d741022100fc5e1c4f07c3f799fb4f1c29cd5f2e8d1e"
    "e18d42c1fc58d51897209e33661873022100f1aeb907f9b1b099e4786fdf50b3"
    "6ad0b4a49299d693fff46ebd709a739a9961022100de8b1eeb922d7d6a8bf277"
    "2365b74995a865bdebe0a466b183bdc145c08d8781022100d1392007ae7acc56"
    "bd7a21e72276e02ea363a1623a67cf7a2cf024ee20cc22a102203db44464ae98"
    "70dd31999226f420ba39152f679ccde179e38dfd33c16a66a465"
);

const SEQUESTER_AUTHORITY_VERIFY_KEY_DER_512: &[u8] = &hex!(
    "305c300d06092a864886f70d0101010500034b003048024100ee40d70583355a"
    "c2eeaa1b9c7a491981bf8d3c88b4547e38788d852cef9110bf5218d9e31dcd45"
    "36c22c898bd099021d21b8f7414fb534266107b63ddc79fe930203010001"
);

type SequesterServiceIdentityType = ([u8; 16], &'static str, &'static [u8], &'static [u8]);
const SEQUESTER_SERVICE_IDENTITIES: &[SequesterServiceIdentityType] = &[
    // (SequesterID, SequesterLabel, PrivateDer512, PublicDer512)
    (
        hex!("00000000000000000000000000000001"),
        "Sequester Service 1",
        &hex!(
            "30820138020100024060dba3946918889a4fc99495b05a938e4174679845fafc26"
            "f86cccf247acfb7c1988b121b082fa9772ed08e71144c2ddd6fe1ab276a1059de7"
            "658c92e86798a3020301000102402b5d0a2e5ea3023d7bed36dd9177ebc9eb9a0e"
            "eb48a1669497f2360b5e759e9d515a0541e9e54747685f991383d3af819bcf4cdf"
            "6e46ca30f97bf9edfb5f5e79022100b64ccba58e6e88446c25fc56643ea2dee5fa"
            "ca624ef34be5af38b02c825439e70221008803fd76351ff648aa0a1f7665a49c8f"
            "5d0e296270f774cfc9d91fa2993a2be502206d4f55193b7399aff56d3d868beb66"
            "fbe2c8e38bd92d57c82cae002d6024a2e7022007d05ade509cfe741c38aaa80d9c"
            "d055b81dd928cb9e9bba58393e050452803502207661e9120f99fe31115d024d37"
            "3b969adf76d4d87bd016120ab10982e3623d6c"
        ),
        &hex!(
            "305b300d06092a864886f70d0101010500034a003047024060dba3946918889a4f"
            "c99495b05a938e4174679845fafc26f86cccf247acfb7c1988b121b082fa9772ed"
            "08e71144c2ddd6fe1ab276a1059de7658c92e86798a30203010001"
        ),
    ),
    (
        hex!("00000000000000000000000000000002"),
        "Sequester Service 2",
        &hex!(
            "3082013b020100024100a8b17a7bdf6236c1956b86703462a38f5ac76fead607ef"
            "0c0718f8884c6d7cb5133038bc12f34f8860dd83840dc7b44ac22bd2950758191a"
            "a8a82d628f36a26102030100010240294c299c21b7bd87eeb88014fffe15e48668"
            "c4b14cec6c4d197fb778469d9725a66bc3f8a60c3af02963826132d1311ea1500a"
            "45e9e3ab48c6d3607b53efd041022100eee29adf045e88dc5d953449ab65e68063"
            "9fab4d079fe33c5f9bc9a067237455022100b4c77bb5f6d30d931db6070801f80c"
            "36b4594abc8a3185a44c8e69449b6b61dd022100d66d4bcd01f4056fa1050c1150"
            "abc052e099f33ecaa84765eb0040e51d8f629d022009a6d8e107338f76fa501ca1"
            "fdd0eb7e0434fc3b82b950244c7e11fababd89f50221008ef4ab5e5a3fefde8a02"
            "16cbad59304dc5e5ce989b9d4e70da80439984fc0496"
        ),
        &hex!(
            "305c300d06092a864886f70d0101010500034b003048024100a8b17a7bdf6236c1"
            "956b86703462a38f5ac76fead607ef0c0718f8884c6d7cb5133038bc12f34f8860"
            "dd83840dc7b44ac22bd2950758191aa8a82d628f36a2610203010001"
        ),
    ),
    (
        hex!("00000000000000000000000000000003"),
        "Sequester Service 3",
        &hex!(
            "3082013a020100024100bc200e0bf21948934768426347ec51a26e6a9cb52d406b"
            "2d5564032ebbb35b891543e246d4488c1a364e6088379d79688001324750bd0caa"
            "32b4a4cdfa9151ad02030100010240477b4e9b0f64b804ee4f195aac0b898154bf"
            "41c83de78a51f16e4d1f46c701f24f186542e5580596c4a555bdaa607486a38973"
            "92a1eec3b0937068d941a34e69022100f0e6d0e7a315af439b8e8b87e2b9fe6321"
            "c2833eaa620b0eb6af587e3cac437b022100c7ea759caec5fc1510af123870b4b2"
            "273b15d6c6cc9ffdafe3b7dd2a11dbc2f70220185df0818da072e2eec8235af257"
            "07e13517fcb888f973cee031aa4cd28c46230220514b3c4c5c9dd1e657047a2a4d"
            "5a8d5f7a9d64db2e7698d89f8732fe5a88bcfd022100c68a97656a4454ea5f049b"
            "7e8ebf027b4515c7c0e93a9e8cea86a4ee9435fc43"
        ),
        &hex!(
            "305c300d06092a864886f70d0101010500034b003048024100bc200e0bf2194893"
            "4768426347ec51a26e6a9cb52d406b2d5564032ebbb35b891543e246d4488c1a36"
            "4e6088379d79688001324750bd0caa32b4a4cdfa9151ad0203010001"
        ),
    ),
    // "3 sequester services ought to be enough for anybody" 1981 William Henry Gates III
];

#[derive(Clone)]
pub(super) struct TestbedTemplateBuilderCounters {
    current_timestamp: DateTime,
    current_certificate_index: IndexInt,
    current_entry_id: u128,
    current_256bits_key: [u8; 32],
    current_sequester_service_identity: std::slice::Iter<'static, SequesterServiceIdentityType>,
}

impl Default for TestbedTemplateBuilderCounters {
    fn default() -> Self {
        Self {
            current_timestamp: "2000-01-01T00:00:00Z".parse().unwrap(),
            current_certificate_index: 0,
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
    pub fn current_certificate_index(&self) -> IndexInt {
        self.current_certificate_index
    }
    pub fn next_certificate_index(&mut self) -> IndexInt {
        self.current_certificate_index += 1;
        self.current_certificate_index
    }
    pub fn next_entry_id(&mut self) -> VlobID {
        self.current_entry_id += 1;
        self.current_entry_id.to_be_bytes().into()
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
}

impl TestbedTemplateBuilder {
    pub(crate) fn new(id: &'static str) -> Self {
        Self {
            id,
            allow_server_side_events: true,
            events: vec![],
            stuff: vec![],
            counters: TestbedTemplateBuilderCounters::default(),
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
            stuff: vec![],
            counters: template.build_counters.clone(),
        }
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
            | TestbedEvent::WorkspaceDataStorageLocalFolderManifestUpdate(_)
            | TestbedEvent::WorkspaceDataStorageLocalFileManifestUpdate(_)
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

    pub fn current_certificate_index(&self) -> IndexInt {
        self.counters.current_certificate_index
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
        let next_index = self.builder.counters.next_certificate_index();
        self.customize(|event| {
            event.sequester_authority = Some(TestbedEventBootstrapOrganizationSequesterAuthority {
                certificate_index: event.first_user_certificate_index,
                signing_key: SEQUESTER_AUTHORITY_SIGNING_KEY_DER_512.try_into().unwrap(),
                verify_key: SEQUESTER_AUTHORITY_VERIFY_KEY_DER_512.try_into().unwrap(),
            });
            event.first_user_certificate_index = event.first_user_first_device_certificate_index;
            event.first_user_first_device_certificate_index = next_index;
        })
    }
    impl_customize_field_meth!(first_user_device_id, DeviceID);
    impl_customize_field_meth!(first_user_human_handle, Option<HumanHandle>);
    impl_customize_field_meth!(first_user_first_device_label, Option<DeviceLabel>);
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
 * TestbedEventNewUserBuilder
 */

impl_event_builder!(NewUser, [user: UserID]);

impl<'a> TestbedEventNewUserBuilder<'a> {
    impl_customize_field_meth!(author, DeviceID);
    impl_customize_field_meth!(device_id, DeviceID);
    impl_customize_field_meth!(human_handle, Option<HumanHandle>);
    impl_customize_field_meth!(first_device_label, Option<DeviceLabel>);
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
    impl_customize_field_meth!(device_label, Option<DeviceLabel>);
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
        let (realm_id, realm_key) =
            match super::utils::assert_user_exists_and_not_revoked(&self.events, &user) {
                TestbedEvent::BootstrapOrganization(x) => (
                    x.first_user_user_realm_id,
                    x.first_user_user_realm_key.clone(),
                ),
                TestbedEvent::NewUser(x) => (x.user_realm_id, x.user_realm_key.clone()),
                _ => unreachable!(),
            };
        let mut event = TestbedEventNewRealm::from_builder(self, user);
        event.realm_id = realm_id;
        event.realm_key = realm_key;
        self.events.push(TestbedEvent::NewRealm(event));
        TestbedEventNewRealmBuilder { builder: self }
    }
}

impl<'a> TestbedEventNewRealmBuilder<'a> {
    impl_customize_field_meth!(author, DeviceID);

    pub fn then_share_with(
        self,
        user: UserID,
        role: Option<RealmRole>,
    ) -> TestbedEventShareRealmBuilder<'a> {
        let realm = self.get_event().realm_id;
        let event = TestbedEventShareRealm::from_builder(self.builder, realm, user, role);
        self.builder.events.push(TestbedEvent::ShareRealm(event));
        TestbedEventShareRealmBuilder {
            builder: self.builder,
        }
    }

    pub fn then_add_workspace_entry_to_user_manifest_vlob(
        self,
    ) -> TestbedEventCreateOrUpdateUserManifestVlobBuilder<'a> {
        let (user, wksp_id, wksp_key, wksp_timestamp) = {
            let realm_event = self.get_event();
            (
                realm_event.author.user_id().to_owned(),
                realm_event.realm_id,
                realm_event.realm_key.clone(),
                realm_event.timestamp,
            )
        };

        let mut update_user_manifest_event =
            TestbedEventCreateOrUpdateUserManifestVlob::from_builder(self.builder, user);

        let x = Arc::make_mut(&mut update_user_manifest_event.manifest);
        x.workspaces.push(WorkspaceEntry::new(
            wksp_id,
            "wksp".parse().unwrap(),
            wksp_key,
            1,
            wksp_timestamp,
        ));

        self.builder
            .events
            .push(TestbedEvent::CreateOrUpdateUserManifestVlob(
                update_user_manifest_event,
            ));

        TestbedEventCreateOrUpdateUserManifestVlobBuilder {
            builder: self.builder,
        }
    }

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

    pub fn then_create_initial_workspace_manifest_vlob(
        self,
    ) -> TestbedEventCreateOrUpdateWorkspaceManifestVlobBuilder<'a> {
        let device = self.get_event().author.clone();
        let realm = self.get_event().realm_id;
        let event = TestbedEventCreateOrUpdateWorkspaceManifestVlob::from_builder(
            self.builder,
            device,
            realm,
        );
        assert_eq!(event.manifest.id, realm);

        self.builder
            .events
            .push(TestbedEvent::CreateOrUpdateWorkspaceManifestVlob(event));
        TestbedEventCreateOrUpdateWorkspaceManifestVlobBuilder {
            builder: self.builder,
        }
    }
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
    impl_customize_field_meth!(realm_entry_name, EntryName);

    pub fn then_also_share_with(
        self,
        user: UserID,
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
 * TestbedEventStartRealmReencryptionBuilder
 */

impl_event_builder!(StartRealmReencryption, [realm: VlobID]);

impl<'a> TestbedEventStartRealmReencryptionBuilder<'a> {
    impl_customize_field_meth!(author, DeviceID);
    impl_customize_field_meth!(entry_name, EntryName);
}

/*
 * TestbedEventFinishRealmReencryptionBuilder
 */

impl_event_builder!(FinishRealmReencryption, [realm: VlobID]);

impl<'a> TestbedEventFinishRealmReencryptionBuilder<'a> {
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

/*
 * TestbedEventCreateOrUpdateFileManifestVlobBuilder
 */

impl_event_builder!(
    CreateOrUpdateFileManifestVlob,
    [device: DeviceID, realm: VlobID, vlob: Option<VlobID>]
);

/*
 * TestbedEventCreateOrUpdateFolderManifestVlobBuilder
 */

impl_event_builder!(
    CreateOrUpdateFolderManifestVlob,
    [device: DeviceID, realm: VlobID, vlob: Option<VlobID>]
);

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
 * TestbedEventCertificatesStorageFetchCertificates
 */

impl_event_builder!(CertificatesStorageFetchCertificates, [device: DeviceID]);

/*
 * TestbedEventUserStorageFetchUserVlob
 */

impl_event_builder!(UserStorageFetchUserVlob, [device: DeviceID]);

/*
 * TestbedEventUserStorageFetchRealmCheckpoint
 */

impl_event_builder!(UserStorageFetchRealmCheckpoint, [device: DeviceID]);

/*
 * TestbedEventUserStorageLocalUpdate
 */

impl_event_builder!(UserStorageLocalUpdate, [device: DeviceID]);

/*
 * TestbedEventWorkspaceCacheStorageFetchBlock
 */

impl_event_builder!(
    WorkspaceCacheStorageFetchBlock,
    [device: DeviceID, realm: VlobID, block: BlockID]
);

/*
 * TestbedEventWorkspaceDataStorageFetchWorkspaceVlob
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
 * TestbedEventWorkspaceDataStorageFetchFileVlob
 */

impl_event_builder!(
    WorkspaceDataStorageFetchFileVlob,
    [device: DeviceID, realm: VlobID, vlob: VlobID]
);

/*
 * TestbedEventWorkspaceDataStorageFetchFolderVlob
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
 * TestbedEventWorkspaceDataStorageFetchRealmCheckpoint
 */

impl_event_builder!(
    WorkspaceDataStorageFetchRealmCheckpoint,
    [device: DeviceID, realm: VlobID]
);

/*
 * TestbedEventWorkspaceDataStorageLocalWorkspaceManifestUpdate
 */

impl_event_builder!(
    WorkspaceDataStorageLocalWorkspaceManifestUpdate,
    [device: DeviceID, realm: VlobID]
);

/*
 * TestbedEventWorkspaceDataStorageLocalFolderManifestUpdate
 */

impl_event_builder!(
    WorkspaceDataStorageLocalFolderManifestUpdate,
    [device: DeviceID, realm: VlobID, vlob: VlobID]
);

/*
 * TestbedEventWorkspaceDataStorageLocalFileManifestUpdate
 */

impl_event_builder!(
    WorkspaceDataStorageLocalFileManifestUpdate,
    [device: DeviceID, realm: VlobID, vlob: VlobID]
);

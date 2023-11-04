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
            "30820156020100300d06092a864886f70d0101010500048201403082013c0201"
            "00024100c765e251febf8b9fdbc9c395575cc5d81b7779b0f5e085a386e14c74"
            "cb62c5590926d311b3ef92ef1cfa0beb1458d528b2c48da0a105ff2ffc130e8f"
            "c9f13ea10203010001024100b59580ef2ed0fcb40d91c63d201bea480d6b5634"
            "e3151e5e771c8165f339f580ad8089b5780ff51a32eccb2e640b6d3aec595c80"
            "3001c5d4d6eeba72e6121001022100eef3e72031602579229b3fe5e793c73de3"
            "17699cda06ce1abeefcb0dab4d50e1022100d59f93576d58ebe5013640ca2e89"
            "aa9b871b667e6aede7f8c797c669ef0ee5c1022100baa47c65a2eb8c999fb450"
            "26cdc99a18f6e674b1f582d73e00e1e062dee8d741022100b1cc1904011cab69"
            "74b2c50601e9ceb797a1b492af48c773861bcaa64cb3e78102203e4b4471d422"
            "5b34ec9b288123c947b820f692876b6bda2529bfb8f3b6670b31"
        ),
        &hex!(
            "305c300d06092a864886f70d0101010500034b003048024100c765e251febf8b"
            "9fdbc9c395575cc5d81b7779b0f5e085a386e14c74cb62c5590926d311b3ef92"
            "ef1cfa0beb1458d528b2c48da0a105ff2ffc130e8fc9f13ea10203010001"
        ),
    ),
    (
        hex!("00000000000000000000000000000002"),
        "Sequester Service 2",
        &hex!(
            "30820153020100300d06092a864886f70d01010105000482013d308201390201"
            "00024100a04eb6eac913471065c894467da3b8e1cf178d14073e33f673a80557"
            "c15491c647f3a4dec3c90c07052debd0b0dc2d3157b94f16f4038604414ce57d"
            "4faf7e610203010001024079ea0320619540994edf112cdec61187e18826a991"
            "ac93b61d552de3044bbda41abf4be24e0d182722966dd65057ae206254b0b919"
            "52171eaac6336aaa17b855022100cfd2ddb27ad0496c1b0137ecd849cb883834"
            "b68498d8f546cb26e6dec85f47eb022100c5780933ddd9f780c84c2fad4eb66f"
            "a6d4787955672c8e57b8b89ae9d70aebe302203126e2203043cad9b2995b2cbd"
            "0ad70464a6b2f431623ec07b93bbcebcfd2bf902205ce711654e7dcabcd2ff4c"
            "fafb6a477334c90d4489cae3990fb60263fba10ec3022012aa4508d2a7880305"
            "c99ed3e07ba73ae2463f0ed344d0dce260f267ef22cdc3"
        ),
        &hex!(
            "305c300d06092a864886f70d0101010500034b003048024100a04eb6eac91347"
            "1065c894467da3b8e1cf178d14073e33f673a80557c15491c647f3a4dec3c90c"
            "07052debd0b0dc2d3157b94f16f4038604414ce57d4faf7e610203010001"
        ),
    ),
    (
        hex!("00000000000000000000000000000003"),
        "Sequester Service 3",
        &hex!(
            "30820154020100300d06092a864886f70d01010105000482013e3082013a0201"
            "00024100b8bbc066ec5b401eaf2e9e4231f116737788c2e9fb95070f235e59a2"
            "c8578b119bace9d795a123692040e8977e6d7ef03e785f172a2355dfc73c13a7"
            "ebe772f90203010001024048e9ba10259c029bd9d3119dd8ee6a491ad5fcf0b3"
            "5ee7d15ec95fa1b3677238cd61cb187e40e34f27ba8c6474bd8f4297e88d1dda"
            "f9df53ed2c027e96403431022100f0f48d29a12b7ce948f3f83bbf72393d2843"
            "9e29848c91b6bf0953bfd1792df5022100c4448b914f683e9cc9e9c31728f74b"
            "335dfc693811d7b4b6a154ed303bc66a7502202e6538451ac66fdfcf6f428de6"
            "980cd36ccc5048f83fc4d2e647da7b65cf2a7d0220387fcaf5afb8409dfdfa0f"
            "313197cecc20b832c9f348f73ac3c2a0ed0ba818a9022100ae43b2f61dd4a9f7"
            "cf932121e39fa36c56e0cc1454db54349179025ae066c2ca"
        ),
        &hex!(
            "305c300d06092a864886f70d0101010500034b003048024100b8bbc066ec5b40"
            "1eaf2e9e4231f116737788c2e9fb95070f235e59a2c8578b119bace9d795a123"
            "692040e8977e6d7ef03e785f172a2355dfc73c13a7ebe772f90203010001"
        ),
    ),
    // "3 sequester services ought to be enough for anybody" 1981 William Henry Gates III
];

#[derive(Clone)]
pub(super) struct TestbedTemplateBuilderCounters {
    current_timestamp: DateTime,
    current_certificate_index: IndexInt,
    current_invitation_token: u128,
    current_entry_id: u128,
    current_256bits_key: [u8; 32],
    current_sequester_service_identity: std::slice::Iter<'static, SequesterServiceIdentityType>,
}

impl Default for TestbedTemplateBuilderCounters {
    fn default() -> Self {
        Self {
            current_timestamp: "2000-01-01T00:00:00Z".parse().unwrap(),
            current_certificate_index: 0,
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
    pub fn new_user_realm(
        &mut self,
        user: impl TryInto<UserID>,
    ) -> TestbedEventNewRealmBuilder<'_> {
        let user: UserID = user
            .try_into()
            .unwrap_or_else(|_| panic!("Invalid value for param user"));
        let (realm_id, realm_key) =
            // TODO: there is still a check consistency there
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
        user: impl TryInto<UserID>,
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

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod build;
mod crc_hash;
mod events;
mod utils;

use std::collections::HashMap;

pub use build::*;
pub use events::*;

use libparsec_types::prelude::*;

use crc_hash::CrcHash;

#[derive(Clone)]
pub struct TestbedTemplate {
    pub id: &'static str,
    pub events: Vec<TestbedEvent>,
    // If the template has been customized, we need a way to retrieve only the custom
    // events (so that we can send them to the server).
    custom_events_offset: usize,
    // Stuff is useful store provide arbitrary things (e.g. IDs) from the template to the test
    pub stuff: Vec<(&'static str, &'static (dyn std::any::Any + Send + Sync))>,
    // As it name suggest, `build_counters` is only used by the `TestbedTemplateBuilder`
    // and is it never used by `TestbedTemplate`.
    // It is only kept here so that we can re-create a template builder from this
    // template, which is what `TestbedEnv::customize` uses to add new events.
    build_counters: TestbedTemplateBuilderCounters,
}

impl std::fmt::Debug for TestbedTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TestbedTemplate")
            .field("id", &self.id)
            .field("events", &self.events)
            .finish()
    }
}

impl TestbedTemplate {
    pub fn custom_events(&self) -> &[TestbedEvent] {
        &self.events[self.custom_events_offset..]
    }

    #[track_caller]
    pub fn get_stuff<T>(&self, key: &'static str) -> &'static T {
        build::get_stuff(&self.stuff, key)
    }

    pub fn from_builder(id: &'static str) -> TestbedTemplateBuilder {
        TestbedTemplateBuilder::new(id)
    }

    pub fn compute_crc(&self) -> u32 {
        let mut hasher = crc32fast::Hasher::new();
        for event in self.events.iter() {
            event.crc_hash(&mut hasher);
        }
        // Note `build_counters` is only used to add new events, hence there
        // is no need to hash it
        hasher.finalize()
    }

    pub fn root_signing_key(&self) -> &SigningKey {
        match self
            .events
            .first()
            .expect("Organization is not bootstrapped")
        {
            TestbedEvent::BootstrapOrganization(x) => &x.root_signing_key,
            _ => unreachable!(),
        }
    }

    pub fn sequester_authority_signing_key(&self) -> &SequesterSigningKeyDer {
        match self
            .events
            .first()
            .expect("Organization is not bootstrapped")
        {
            TestbedEvent::BootstrapOrganization(x) => x
                .sequester_authority
                .as_ref()
                .map(|sequester_authority| &sequester_authority.signing_key)
                .expect("Not a sequestered organization"),
            _ => unreachable!(),
        }
    }

    pub fn sequester_services_public_key(
        &self,
    ) -> Option<impl Iterator<Item = (&SequesterServiceID, &SequesterPublicKeyDer)>> {
        match self
            .events
            .first()
            .expect("Organization is not bootstrapped")
        {
            TestbedEvent::BootstrapOrganization(x) => {
                x.sequester_authority.as_ref()?;
            }
            _ => unreachable!(),
        }

        Some(self.events.iter().filter_map(|e| match e {
            TestbedEvent::NewSequesterService(x) => Some((&x.id, &x.encryption_public_key)),
            _ => None,
        }))
    }

    pub fn device_creation_event(&self, device_id: DeviceID) -> &TestbedEvent {
        self.events
            .iter()
            .find(|e| {
                matches!(e,
                    TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                        first_user_first_device_id: candidate,
                        ..
                    })
                    | TestbedEvent::NewUser(TestbedEventNewUser {
                        first_device_id: candidate,
                        ..
                    })
                    | TestbedEvent::NewDevice(TestbedEventNewDevice {
                        device_id: candidate,
                        ..
                    }) if *candidate == device_id,
                )
            })
            .expect("Device doesn't exist")
    }

    pub fn user_creation_event(&self, user_id: UserID) -> &TestbedEvent {
        self.events
            .iter()
            .find(|e| {
                matches!(e,
                    TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                        first_user_id: candidate,
                        ..
                    })
                    | TestbedEvent::NewUser(TestbedEventNewUser {
                        user_id: candidate, ..
                    })
                    | TestbedEvent::NewDevice(TestbedEventNewDevice {
                        user_id: candidate, ..
                    }) if *candidate == user_id,
                )
            })
            .expect("User doesn't exist")
    }

    pub fn device_user_id(&self, device_id: DeviceID) -> UserID {
        self.events
            .iter()
            .find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                    first_user_first_device_id: candidate,
                    first_user_id: user_id,
                    ..
                })
                | TestbedEvent::NewUser(TestbedEventNewUser {
                    first_device_id: candidate,
                    user_id,
                    ..
                })
                | TestbedEvent::NewDevice(TestbedEventNewDevice {
                    device_id: candidate,
                    user_id,
                    ..
                }) if *candidate == device_id => Some(*user_id),
                _ => None,
            })
            .expect("Device doesn't exist")
    }

    pub fn device_signing_key(&self, device_id: DeviceID) -> &SigningKey {
        self.events
            .iter()
            .find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                    first_user_first_device_id: candidate,
                    first_user_first_device_signing_key: signing_key,
                    ..
                })
                | TestbedEvent::NewUser(TestbedEventNewUser {
                    first_device_id: candidate,
                    first_device_signing_key: signing_key,
                    ..
                })
                | TestbedEvent::NewDevice(TestbedEventNewDevice {
                    device_id: candidate,
                    signing_key,
                    ..
                }) if *candidate == device_id => Some(signing_key),
                _ => None,
            })
            .expect("Device doesn't exist")
    }

    pub fn device_local_symkey(&self, device_id: DeviceID) -> &SecretKey {
        self.events
            .iter()
            .find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                    first_user_first_device_id: candidate,
                    first_user_local_symkey: local_symkey,
                    ..
                })
                | TestbedEvent::NewUser(TestbedEventNewUser {
                    first_device_id: candidate,
                    local_symkey,
                    ..
                })
                | TestbedEvent::NewDevice(TestbedEventNewDevice {
                    device_id: candidate,
                    local_symkey,
                    ..
                }) if *candidate == device_id => Some(local_symkey),
                _ => None,
            })
            .expect("Device doesn't exist")
    }

    pub fn user_realm_key(&self, user_id: UserID) -> &SecretKey {
        self.events
            .iter()
            .find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                    first_user_id: candidate,
                    first_user_user_realm_key: user_realm_key,
                    ..
                })
                | TestbedEvent::NewUser(TestbedEventNewUser {
                    user_id: candidate,
                    user_realm_key,
                    ..
                }) if *candidate == user_id => Some(user_realm_key),
                _ => None,
            })
            .expect("User doesn't exist")
    }

    pub fn user_private_key(&self, user_id: UserID) -> &PrivateKey {
        self.events
            .iter()
            .find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                    first_user_id: candidate,
                    first_user_private_key: private_key,
                    ..
                })
                | TestbedEvent::NewUser(TestbedEventNewUser {
                    user_id: candidate,
                    private_key,
                    ..
                }) if *candidate == user_id => Some(private_key),
                _ => None,
            })
            .expect("User doesn't exist")
    }

    pub fn user_profile_at(&self, user_id: UserID, up_to: Option<DateTime>) -> UserProfile {
        // Default to the last timestamp used to create event, i.e. all events are included
        let up_to = up_to.unwrap_or(self.build_counters.current_timestamp());

        let mut current_profile = None;
        self.events.iter().find_map(|e| {
            let maybe_profile_update = match e {
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                    first_user_id: candidate,
                    timestamp,
                    ..
                }) if *candidate == user_id => Some((UserProfile::Admin, *timestamp)),
                TestbedEvent::NewUser(TestbedEventNewUser {
                    user_id: candidate,
                    initial_profile,
                    timestamp,
                    ..
                }) if *candidate == user_id => Some((*initial_profile, *timestamp)),
                TestbedEvent::UpdateUserProfile(TestbedEventUpdateUserProfile {
                    user: candidate,
                    profile,
                    timestamp,
                    ..
                }) if *candidate == user_id => Some((*profile, *timestamp)),
                _ => None,
            };
            maybe_profile_update.map(|(profile, certificate_timestamp)| {
                if certificate_timestamp > up_to {
                    // Stop iteration
                    Some(())
                } else {
                    current_profile = Some(profile);
                    None
                }
            })
        });
        current_profile.expect("User doesn't exist")
    }

    pub fn realm_users_roles_at(
        &self,
        realm_id: VlobID,
        up_to: Option<DateTime>,
    ) -> HashMap<UserID, RealmRole> {
        // Default to the last timestamp used to create event, i.e. all events are included
        let up_to = up_to.unwrap_or(self.build_counters.current_timestamp());

        let mut roles = HashMap::new();
        for e in &self.events {
            match e {
                TestbedEvent::NewRealm(e) if e.realm_id == realm_id && e.timestamp <= up_to => {
                    let user_id = self.device_user_id(e.author);
                    roles.insert(user_id, RealmRole::Owner);
                }
                TestbedEvent::ShareRealm(e) if e.realm == realm_id && e.timestamp <= up_to => {
                    match e.role {
                        Some(role) => {
                            roles.insert(e.user, role);
                        }
                        None => {
                            roles.remove(&e.user);
                        }
                    }
                }
                _ => (),
            }
        }
        roles
    }

    pub fn user_devices_ids(&self, user_id: UserID) -> Vec<DeviceID> {
        self.events
            .iter()
            .filter_map(|e| match e {
                TestbedEvent::BootstrapOrganization(e) if e.first_user_id == user_id => {
                    Some(e.first_user_first_device_id)
                }
                TestbedEvent::NewUser(e) if e.user_id == user_id => Some(e.first_device_id),
                TestbedEvent::NewDevice(e) if e.user_id == user_id => Some(e.device_id),
                _ => None,
            })
            .collect()
    }

    pub fn certificates(&self) -> impl Iterator<Item = TestbedTemplateEventCertificate> + '_ {
        self.events
            .iter()
            .flat_map(|event| event.certificates(self))
    }

    pub fn certificates_rev(&self) -> impl Iterator<Item = TestbedTemplateEventCertificate> + '_ {
        self.events
            .iter()
            .rev()
            .flat_map(|event| event.certificates(self))
    }
}

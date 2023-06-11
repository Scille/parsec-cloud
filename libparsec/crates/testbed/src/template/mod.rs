// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod build;
mod crc_hash;
mod events;

pub use build::*;
pub use events::*;

use std::sync::Arc;

use libparsec_types::prelude::*;

use self::crc_hash::CrcHash;

#[derive(Debug)]
pub struct TestbedUserData {
    pub user_id: UserID,
    pub human_handle: Option<HumanHandle>,
    pub private_key: PrivateKey,
    pub profile: UserProfile,
    pub user_manifest_id: EntryID,
    pub user_manifest_key: SecretKey,
}

#[derive(Debug)]
pub struct TestbedDeviceData {
    pub device_id: DeviceID,
    pub device_label: Option<DeviceLabel>,
    pub signing_key: SigningKey,
    pub local_symkey: SecretKey,
}

pub struct TestbedTemplate {
    pub id: &'static str,

    // A template can be based on another one, this is useful to customize the
    // template from within a test
    base: Option<Arc<TestbedTemplate>>,

    events: Vec<TestbedEvent>,

    last_timestamp: DateTime,
    last_certificate_index: IndexInt,
}

impl std::fmt::Debug for TestbedTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TestbedTemplate")
            .field("id", &self.id)
            .field("events", &self.events)
            .finish()
    }
}

impl TestbedTemplate {
    pub fn from_builder(
        id: &'static str,
    ) -> TestbedTemplateBuilder<
        Arc<TestbedTemplate>,
        impl FnOnce(Arc<TestbedTemplate>) -> Arc<TestbedTemplate>,
    > {
        TestbedTemplateBuilder::new(id, |t| t)
    }

    fn new(id: &'static str) -> Self {
        let last_timestamp = "2000-01-01T00:00:00Z".parse().unwrap();
        Self {
            id,
            base: None,
            events: vec![],
            last_timestamp,
            last_certificate_index: 0,
        }
    }

    pub fn compute_crc(&self) -> u32 {
        let mut hasher = crc32fast::Hasher::new();
        self.events.crc_hash(&mut hasher);
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

    pub fn device_signing_key<'a>(&'a self, device_id: &DeviceID) -> &'a SigningKey {
        self.events()
            .find_map(|e| match e {
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                    first_user_device_id: candidate,
                    first_user_first_device_signing_key: signing_key,
                    ..
                })
                | TestbedEvent::NewUser(TestbedEventNewUser {
                    device_id: candidate,
                    first_device_signing_key: signing_key,
                    ..
                })
                | TestbedEvent::NewDevice(TestbedEventNewDevice {
                    device_id: candidate,
                    signing_key,
                    ..
                }) if candidate == device_id => Some(signing_key),
                _ => None,
            })
            .expect("Device doesn't exist")
    }

    pub fn user_profile_at(
        &self,
        user_id: &UserID,
        up_to_certificate_index: IndexInt,
    ) -> UserProfile {
        let mut current_profile = None;
        self.events().find_map(|e| {
            let maybe_profile_update = match e {
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                    first_user_device_id,
                    first_user_certificate_index,
                    ..
                }) if first_user_device_id.user_id() == user_id => {
                    Some((UserProfile::Admin, *first_user_certificate_index))
                }
                TestbedEvent::NewUser(TestbedEventNewUser {
                    device_id,
                    initial_profile,
                    user_certificate_index,
                    ..
                }) if device_id.user_id() == user_id => {
                    Some((*initial_profile, *user_certificate_index))
                }
                TestbedEvent::UpdateUserProfile(TestbedEventUpdateUserProfile {
                    user,
                    profile,
                    certificate_index,
                    ..
                }) if user == user_id => Some((*profile, *certificate_index)),
                _ => None,
            };
            maybe_profile_update.map(|(profile, certificate_index)| {
                if certificate_index > up_to_certificate_index {
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

    pub fn certificates(&self) -> impl Iterator<Item = TestbedTemplateEventCertificate> + '_ {
        self.events().flat_map(|event| event.certificates(self))
    }

    pub fn events(
        &self,
    ) -> impl Iterator<Item = &TestbedEvent> + DoubleEndedIterator + ExactSizeIterator {
        TestbedTemplateAllEventsIterator {
            base_iter: self.base.as_ref().map(|base| {
                // Arbitrary level of inheritance make the iterator more complicated
                // to implement, and is not needed here.
                assert!(
                    matches!(base.base, None),
                    "Only one level of inheritance allowed !"
                );
                base.events.iter()
            }),
            iter: self.events.iter(),
        }
    }
}

struct TestbedTemplateAllEventsIterator<'a, A, B>
where
    A: Iterator<Item = &'a TestbedEvent> + DoubleEndedIterator + ExactSizeIterator + 'a,
    B: Iterator<Item = &'a TestbedEvent> + DoubleEndedIterator + ExactSizeIterator + 'a,
{
    base_iter: Option<A>,
    iter: B,
}

impl<'a, A, B> Iterator for TestbedTemplateAllEventsIterator<'a, A, B>
where
    A: Iterator<Item = &'a TestbedEvent> + DoubleEndedIterator + ExactSizeIterator + 'a,
    B: Iterator<Item = &'a TestbedEvent> + DoubleEndedIterator + ExactSizeIterator + 'a,
{
    type Item = &'a TestbedEvent;

    fn next(&mut self) -> Option<Self::Item> {
        let base_next = match &mut self.base_iter {
            Some(base_iter) => base_iter.next(),
            None => None,
        };
        base_next.or_else(|| self.iter.next())
    }
}

impl<'a, A, B> DoubleEndedIterator for TestbedTemplateAllEventsIterator<'a, A, B>
where
    A: Iterator<Item = &'a TestbedEvent> + DoubleEndedIterator + ExactSizeIterator + 'a,
    B: Iterator<Item = &'a TestbedEvent> + DoubleEndedIterator + ExactSizeIterator + 'a,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let back = self.iter.next_back();
        back.or_else(|| match &mut self.base_iter {
            Some(base_iter) => base_iter.next_back(),
            None => None,
        })
    }
}

impl<'a, A, B> ExactSizeIterator for TestbedTemplateAllEventsIterator<'a, A, B>
where
    A: Iterator<Item = &'a TestbedEvent> + DoubleEndedIterator + ExactSizeIterator + 'a,
    B: Iterator<Item = &'a TestbedEvent> + DoubleEndedIterator + ExactSizeIterator + 'a,
{
    fn len(&self) -> usize {
        self.iter.len()
            + self
                .base_iter
                .as_ref()
                .map_or(0, |base_iter| base_iter.len())
    }
}

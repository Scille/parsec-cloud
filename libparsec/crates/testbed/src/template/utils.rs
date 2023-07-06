// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::{TestbedEvent, TestbedEventBootstrapOrganization, TestbedEventNewUser};

pub(super) fn non_revoked_users(events: &[TestbedEvent]) -> impl Iterator<Item = &DeviceID> {
    events.iter().filter_map(|e| match e {
        TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
            first_user_device_id: device_id,
            ..
        })
        | TestbedEvent::NewUser(TestbedEventNewUser { device_id, .. }) => {
            let user_id = device_id.user_id();
            let is_revoked = events
                .iter()
                .any(|e| matches!(e, TestbedEvent::RevokeUser(x) if x.user == *user_id));
            if is_revoked {
                None
            } else {
                Some(device_id)
            }
        }
        _ => None,
    })
}

pub(super) fn non_revoked_admins(events: &[TestbedEvent]) -> impl Iterator<Item = &DeviceID> {
    events.iter().filter_map(|e| match e {
        TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
            first_user_device_id: device_id,
            ..
        })
        | TestbedEvent::NewUser(TestbedEventNewUser { device_id, .. }) => {
            let user_id = device_id.user_id();
            // Is revoked ?
            if events
                .iter()
                .any(|e| matches!(e, TestbedEvent::RevokeUser(x) if x.user == *user_id))
            {
                return None;
            }
            // Is admin ?
            let profile = events
                .iter()
                .rev()
                .find_map(|e| match e {
                    TestbedEvent::UpdateUserProfile(x) if x.user == *user_id => Some(x.profile),
                    TestbedEvent::BootstrapOrganization(x)
                        if x.first_user_device_id.user_id() == user_id =>
                    {
                        Some(UserProfile::Admin)
                    }
                    TestbedEvent::NewUser(x) if x.device_id.user_id() == user_id => {
                        Some(x.initial_profile)
                    }
                    _ => None,
                })
                .expect("The user must at least have a creation event");
            if profile == UserProfile::Admin {
                Some(device_id)
            } else {
                None
            }
        }
        _ => None,
    })
}

pub(super) fn non_revoked_realm_owners<'a, 'b: 'a>(
    events: &'a [TestbedEvent],
    realm: &'b RealmID,
) -> impl Iterator<Item = &'a DeviceID> {
    non_revoked_users(events).filter(|device_id| {
        let user_id = device_id.user_id();
        let is_owner = events.iter().rev().find_map(|e| match e {
            TestbedEvent::NewRealm(x) if x.realm_id == *realm && x.author.user_id() == user_id => {
                Some(true)
            }
            TestbedEvent::ShareRealm(x) if x.realm == *realm && x.user == *user_id => {
                Some(matches!(x.role, Some(RealmRole::Owner)))
            }
            _ => None,
        });
        match is_owner {
            Some(true) => true,
            // User is either not part of the realm, or not owner
            _ => false,
        }
    })
}

pub(super) fn non_revoked_realm_members<'a, 'b: 'a>(
    events: &'a [TestbedEvent],
    realm: &'b RealmID,
) -> impl Iterator<Item = &'a DeviceID> {
    non_revoked_users(events).filter(|device_id| {
        let user_id = device_id.user_id();
        let is_owner = events.iter().rev().find_map(|e| match e {
            TestbedEvent::NewRealm(x) if x.realm_id == *realm && x.author.user_id() == user_id => {
                Some(true)
            }
            TestbedEvent::ShareRealm(x) if x.realm == *realm && x.user == *user_id => {
                Some(x.role.is_some())
            }
            _ => None,
        });
        match is_owner {
            Some(true) => true,
            // User is either not part of the realm, or not owner
            _ => false,
        }
    })
}

pub(super) fn assert_organization_bootstrapped(
    events: &[TestbedEvent],
) -> &TestbedEventBootstrapOrganization {
    match events.first().expect("Bootstrap organization first !") {
        TestbedEvent::BootstrapOrganization(x) => x,
        _ => unreachable!(),
    }
}

pub(super) fn assert_user_exists_and_not_revoked<'a>(
    events: &'a [TestbedEvent],
    user: &UserID,
) -> &'a TestbedEvent {
    let mut creation_event = None;
    for event in events.iter() {
        match event {
            e @ TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                first_user_device_id: candidate,
                ..
            })
            | e @ TestbedEvent::NewUser(TestbedEventNewUser {
                device_id: candidate,
                ..
            }) if candidate.user_id() == user => {
                creation_event = Some(e);
            }
            TestbedEvent::RevokeUser(x) if x.user == *user => panic!("User already revoked !"),
            _ => (),
        }
    }
    creation_event.unwrap_or_else(|| panic!("User {} doesn't exist", user))
}

pub(super) fn assert_device_exists<'a>(
    events: &'a [TestbedEvent],
    device: &DeviceID,
) -> &'a TestbedEvent {
    let mut creation_event = None;
    for event in events.iter() {
        match event {
            e @ TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                first_user_device_id: candidate,
                ..
            })
            | e @ TestbedEvent::NewUser(TestbedEventNewUser {
                device_id: candidate,
                ..
            }) if candidate == device => {
                creation_event = Some(e);
            }
            _ => (),
        }
    }
    creation_event.unwrap_or_else(|| panic!("Device {} doesn't exist", device))
}

pub(super) fn assert_realm_exists_and_under_reencryption(
    events: &[TestbedEvent],
    realm: &RealmID,
) -> IndexInt {
    let outcome = events
        .iter()
        .rev()
        .find_map(|e| match e {
            TestbedEvent::StartRealmReencryption(x) if x.realm == *realm => {
                Some(Some(x.encryption_revision))
            }
            TestbedEvent::FinishRealmReencryption(x) if x.realm == *realm => Some(None),
            TestbedEvent::NewRealm(x) if x.realm_id == *realm => Some(None),
            _ => None,
        })
        .unwrap_or_else(|| panic!("Realm {} doesn't exist", realm));
    match outcome {
        None => panic!("Realm {} not currently under reencryption", realm),
        Some(encryption_revision) => encryption_revision,
    }
}

pub(super) fn assert_realm_exists_and_not_under_reencryption(
    events: &[TestbedEvent],
    realm: &RealmID,
) -> IndexInt {
    let outcome = events
        .iter()
        .rev()
        .find_map(|e| match e {
            TestbedEvent::NewRealm(x) if x.realm_id == *realm => Some(Some(1)),
            TestbedEvent::StartRealmReencryption(x) if x.realm == *realm => {
                Some(Some(x.encryption_revision))
            }
            TestbedEvent::FinishRealmReencryption(x) if x.realm == *realm => Some(None),
            _ => None,
        })
        .unwrap_or_else(|| panic!("Realm {} doesn't exist", realm));
    match outcome {
        None => panic!("Realm {} currently under reencryption", realm),
        Some(reencryption_revision) => reencryption_revision,
    }
}

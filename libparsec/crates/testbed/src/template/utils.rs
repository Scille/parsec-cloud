// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use crate::TestbedEventNewDevice;

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

pub(super) fn non_revoked_realm_owners(
    events: &[TestbedEvent],
    realm: VlobID,
) -> impl Iterator<Item = &DeviceID> {
    non_revoked_users(events).filter(move |device_id| {
        let user_id = device_id.user_id();
        let is_owner = events.iter().rev().find_map(|e| match e {
            TestbedEvent::NewRealm(x) if x.realm_id == realm && x.author.user_id() == user_id => {
                Some(true)
            }
            TestbedEvent::ShareRealm(x) if x.realm == realm && x.user == *user_id => {
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

#[allow(unused)]
pub(super) fn non_revoked_realm_members(
    events: &[TestbedEvent],
    realm: VlobID,
) -> impl Iterator<Item = (&DeviceID, RealmRole)> {
    non_revoked_users(events).filter_map(move |device_id| {
        let user_id = device_id.user_id();
        events.iter().rev().find_map(|e| match e {
            TestbedEvent::NewRealm(x) if x.realm_id == realm && x.author.user_id() == user_id => {
                Some((device_id, RealmRole::Owner))
            }
            TestbedEvent::ShareRealm(x) if x.realm == realm && x.user == *user_id => {
                x.role.map(|role| (device_id, role))
            }
            _ => None,
        })
    })
}

#[allow(unused)]
pub(super) fn realm_keys(
    events: &[TestbedEvent],
    realm: VlobID,
) -> impl Iterator<Item = (IndexInt, &SecretKey)> {
    events.iter().filter_map(move |e| match e {
        TestbedEvent::RotateKeyRealm(x) if x.realm == realm => Some((
            x.key_index,
            x.keys.last().expect("Certificate must contains keys"),
        )),
        _ => None,
    })
}

pub(super) fn assert_realm_member_has_read_access(
    events: &[TestbedEvent],
    realm: VlobID,
    user: &UserID,
) {
    let has_read_access = events
        .iter()
        .rev()
        .find_map(move |e| match e {
            TestbedEvent::NewRealm(x) if x.realm_id == realm => {
                // Last chance if the user is the creator of the realm
                Some(x.author.user_id() == user)
            }
            TestbedEvent::ShareRealm(x) if x.realm == realm && x.user == *user => {
                Some(x.role.map(|r| r.can_read()).unwrap_or(false))
            }
            _ => None,
        })
        .unwrap_or(false);

    if !has_read_access {
        panic!("User {} has no read access to realm {}", user, realm);
    }
}

pub(super) fn assert_realm_member_has_write_access(
    events: &[TestbedEvent],
    realm: VlobID,
    user: &UserID,
) {
    let has_write_access = events
        .iter()
        .rev()
        .find_map(move |e| match e {
            TestbedEvent::NewRealm(x) if x.realm_id == realm => {
                // Last chance if the user is the creator of the realm
                Some(x.author.user_id() == user)
            }
            TestbedEvent::ShareRealm(x) if x.realm == realm && x.user == *user => {
                Some(x.role.map(|r| r.can_write()).unwrap_or(false))
            }
            _ => None,
        })
        .unwrap_or(false);

    if !has_write_access {
        panic!("User {} has no write access to realm {}", user, realm);
    }
}

pub(super) fn assert_organization_bootstrapped(
    events: &[TestbedEvent],
) -> &TestbedEventBootstrapOrganization {
    match events.first().expect("Bootstrap organization first !") {
        TestbedEvent::BootstrapOrganization(x) => x,
        _ => unreachable!(),
    }
}

pub(super) fn assert_device_exists_and_not_revoked<'a>(
    events: &'a [TestbedEvent],
    device: &DeviceID,
) -> &'a TestbedEvent {
    for event in events.iter().rev() {
        match event {
            e @ TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                first_user_device_id: candidate,
                ..
            })
            | e @ TestbedEvent::NewUser(TestbedEventNewUser {
                device_id: candidate,
                ..
            }) if candidate == device => return e,
            e @ TestbedEvent::NewDevice(TestbedEventNewDevice {
                device_id: candidate,
                ..
            }) if candidate == device => return e,
            TestbedEvent::RevokeUser(x) if x.user == *device.user_id() => {
                panic!("User {} already revoked !", device.user_id())
            }
            _ => (),
        }
    }
    panic!("Device {} doesn't exist", device);
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

pub(super) fn assert_realm_exists(events: &[TestbedEvent], realm: VlobID) {
    events
        .iter()
        .rev()
        .find(|e| matches!(e, TestbedEvent::NewRealm(x) if x.realm_id == realm))
        .unwrap_or_else(|| panic!("Realm {} doesn't exist", realm));
}

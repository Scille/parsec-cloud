// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use crate::TestbedEventNewDevice;

use super::{
    TestbedEvent, TestbedEventBootstrapOrganization, TestbedEventNewSequesterService,
    TestbedEventNewUser, TestbedEventUpdateUserProfile,
};

pub(super) fn user_id_from_device_id(events: &[TestbedEvent], device: DeviceID) -> UserID {
    events
        .iter()
        .find_map(|e| match e {
            TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                first_user_id: user_id,
                first_user_first_device_id: device_id,
                ..
            })
            | TestbedEvent::NewUser(TestbedEventNewUser {
                user_id,
                first_device_id: device_id,
                ..
            })
            | TestbedEvent::NewDevice(TestbedEventNewDevice {
                user_id, device_id, ..
            }) if *device_id == device => Some(*user_id),
            _ => None,
        })
        .expect("Uknown device ID !")
}

/// Return the first device of each non-revoked user.
pub(super) fn non_revoked_users(
    events: &[TestbedEvent],
) -> impl Iterator<Item = (UserID, DeviceID)> + '_ {
    events.iter().filter_map(|e| match e {
        TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
            first_user_id: user_id,
            first_user_first_device_id: device_id,
            ..
        })
        | TestbedEvent::NewUser(TestbedEventNewUser {
            user_id,
            first_device_id: device_id,
            ..
        }) => {
            let is_revoked = events
                .iter()
                .any(|e| matches!(e, TestbedEvent::RevokeUser(x) if x.user == *user_id));
            if is_revoked {
                None
            } else {
                Some((*user_id, *device_id))
            }
        }
        _ => None,
    })
}

/// Return all devices of each non-revoked user.
pub(super) fn non_revoked_users_each_devices(
    events: &[TestbedEvent],
) -> impl Iterator<Item = (UserID, DeviceID)> + '_ {
    events.iter().filter_map(|e| match e {
        TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
            first_user_id: user_id,
            first_user_first_device_id: device_id,
            ..
        })
        | TestbedEvent::NewUser(TestbedEventNewUser {
            user_id,
            first_device_id: device_id,
            ..
        })
        | TestbedEvent::NewDevice(TestbedEventNewDevice {
            user_id, device_id, ..
        }) => {
            let is_revoked = events
                .iter()
                .any(|e| matches!(e, TestbedEvent::RevokeUser(x) if x.user == *user_id));
            if is_revoked {
                None
            } else {
                Some((*user_id, *device_id))
            }
        }
        _ => None,
    })
}

/// Return the first device of each non-revoked user with ADMIN profile.
pub(super) fn non_revoked_admins(
    events: &[TestbedEvent],
) -> impl Iterator<Item = (UserID, DeviceID)> + '_ {
    events.iter().filter_map(|e| match e {
        TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
            first_user_id: user_id,
            first_user_first_device_id: device_id,
            ..
        })
        | TestbedEvent::NewUser(TestbedEventNewUser {
            user_id,
            first_device_id: device_id,
            ..
        }) => {
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
                    TestbedEvent::BootstrapOrganization(x) if x.first_user_id == *user_id => {
                        Some(UserProfile::Admin)
                    }
                    TestbedEvent::NewUser(x) if x.user_id == *user_id => Some(x.initial_profile),
                    _ => None,
                })
                .expect("The user must at least have a creation event");
            if profile == UserProfile::Admin {
                Some((*user_id, *device_id))
            } else {
                None
            }
        }
        _ => None,
    })
}

/// Return the first device of each non-revoked user having OWNER profile on the given realm.
pub(super) fn non_revoked_realm_owners(
    events: &[TestbedEvent],
    realm: VlobID,
) -> impl Iterator<Item = (UserID, DeviceID)> + '_ {
    events.iter().filter_map(move |e| match e {
        TestbedEvent::NewRealm(x) if x.realm_id == realm => {
            non_revoked_users_each_devices(events).find(|(_, d)| *d == x.author)
        }
        TestbedEvent::ShareRealm(x)
            if x.realm == realm && matches!(x.role, Some(RealmRole::Owner)) =>
        {
            non_revoked_users(events).find(|(u, _)| *u == x.user)
        }
        _ => None,
    })
}

/// Return the first device of each non-revoked user having access to the given realm.
pub(super) fn non_revoked_realm_members(
    events: &[TestbedEvent],
    realm: VlobID,
) -> impl Iterator<Item = (UserID, DeviceID, RealmRole)> + '_ {
    events.iter().filter_map(move |e| match e {
        TestbedEvent::NewRealm(x) if x.realm_id == realm => non_revoked_users_each_devices(events)
            .find(|(_, d)| *d == x.author)
            .map(|(user_id, device_id)| (user_id, device_id, RealmRole::Owner)),
        TestbedEvent::ShareRealm(x) if x.realm == realm => match x.role {
            Some(role) => non_revoked_users(events)
                .find(|(u, _)| *u == x.user)
                .map(|(user_id, device_id)| (user_id, device_id, role)),
            None => None,
        },
        _ => None,
    })
}

pub(super) fn non_revoked_sequester_services(
    events: &[TestbedEvent],
) -> Option<impl Iterator<Item = &TestbedEventNewSequesterService> + '_> {
    assert_organization_bootstrapped(events)
        .sequester_authority
        .as_ref()?;

    Some(events.iter().enumerate().filter_map(move |(i, e)| match e {
        TestbedEvent::NewSequesterService(service) => {
            let revoked = events.iter().skip(i).any(
                |e| matches!(e, TestbedEvent::RevokeSequesterService(x) if x.id == service.id),
            );
            if revoked {
                None
            } else {
                Some(service)
            }
        }
        _ => None,
    }))
}

pub(super) fn realm_keys(
    events: &[TestbedEvent],
    realm: VlobID,
) -> impl DoubleEndedIterator<Item = (IndexInt, &KeyDerivation)> {
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
    user: UserID,
) {
    let has_read_access = events
        .iter()
        .rev()
        .find_map(move |e| match e {
            TestbedEvent::ShareRealm(x) if x.realm == realm && x.user == user => {
                Some(x.role.map(|r| r.can_read()).unwrap_or(false))
            }
            TestbedEvent::NewRealm(x) if x.realm_id == realm => {
                // Last chance if the user is the creator of the realm
                let author_user_id = user_id_from_device_id(events, x.author);
                Some(author_user_id == user)
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
    user: UserID,
) {
    let has_write_access = events
        .iter()
        .rev()
        .find_map(move |e| match e {
            TestbedEvent::ShareRealm(x) if x.realm == realm && x.user == user => {
                Some(x.role.map(|r| r.can_write()).unwrap_or(false))
            }
            TestbedEvent::NewRealm(x) if x.realm_id == realm => {
                // Last chance if the user is the creator of the realm
                let author_user_id = user_id_from_device_id(events, x.author);
                Some(author_user_id == user)
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

pub(super) fn assert_device_exists_and_not_revoked(
    events: &'_ [TestbedEvent],
    device: DeviceID,
) -> &'_ TestbedEvent {
    let user_id = user_id_from_device_id(events, device);

    for event in events.iter().rev() {
        match event {
            e @ TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                first_user_first_device_id: candidate,
                ..
            })
            | e @ TestbedEvent::NewUser(TestbedEventNewUser {
                first_device_id: candidate,
                ..
            }) if *candidate == device => return e,
            e @ TestbedEvent::NewDevice(TestbedEventNewDevice {
                device_id: candidate,
                ..
            }) if *candidate == device => return e,
            TestbedEvent::RevokeUser(x) if x.user == user_id => {
                panic!("User {} already revoked !", user_id)
            }
            _ => (),
        }
    }
    panic!("Device {} doesn't exist", device);
}

pub(super) fn assert_user_exists_and_not_revoked(
    events: &'_ [TestbedEvent],
    user: UserID,
) -> &'_ TestbedEvent {
    let mut creation_event = None;
    for event in events.iter() {
        match event {
            e @ TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                first_user_id: candidate,
                ..
            })
            | e @ TestbedEvent::NewUser(TestbedEventNewUser {
                user_id: candidate, ..
            }) if *candidate == user => {
                creation_event = Some(e);
            }
            TestbedEvent::RevokeUser(x) if x.user == user => panic!("User already revoked !"),
            _ => (),
        }
    }
    creation_event.unwrap_or_else(|| panic!("User {} doesn't exist", user))
}

pub(super) fn assert_device_exists(
    events: &'_ [TestbedEvent],
    device: DeviceID,
) -> &'_ TestbedEvent {
    events
        .iter()
        .rev()
        .find(|e| {
            matches!(e,
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                    first_user_first_device_id: candidate,
                    ..
                })
                | TestbedEvent::NewUser(TestbedEventNewUser {
                    first_device_id: candidate, ..
                })
                | TestbedEvent::NewDevice(TestbedEventNewDevice { device_id: candidate, .. })
                if *candidate == device
            )
        })
        .unwrap_or_else(|| panic!("Device {} doesn't exist", device))
}

pub(super) fn assert_user_exists(events: &'_ [TestbedEvent], user: UserID) -> &'_ TestbedEvent {
    events
        .iter()
        .rev()
        .find(|e| {
            matches!(e,
                TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                    first_user_id: candidate,
                    ..
                })
                | TestbedEvent::NewUser(TestbedEventNewUser {
                    user_id: candidate, ..
                }) if *candidate == user
            )
        })
        .unwrap_or_else(|| panic!("User {} doesn't exist", user))
}

pub(super) fn assert_realm_exists(events: &[TestbedEvent], realm: VlobID) {
    events
        .iter()
        .rev()
        .find(|e| matches!(e, TestbedEvent::NewRealm(x) if x.realm_id == realm))
        .unwrap_or_else(|| panic!("Realm {} doesn't exist", realm));
}

pub(super) fn assert_user_has_non_deleted_shamir_recovery(
    events: &'_ [TestbedEvent],
    user: UserID,
) -> &'_ TestbedEvent {
    events
        .iter()
        .rev()
        .find(|e| match e {
            TestbedEvent::NewShamirRecovery(x) if x.user_id == user => true,
            TestbedEvent::DeleteShamirRecovery(x) if x.setup_to_delete_user_id == user => {
                panic!("User {}'s Shamir recovery is deleted !", user)
            }
            _ => false,
        })
        .unwrap_or_else(|| panic!("User {} doesn't have any Shamir recovery", user))
}

pub(super) fn get_user_current_profile(events: &'_ [TestbedEvent], user: UserID) -> UserProfile {
    for event in events.iter().rev() {
        match event {
            TestbedEvent::BootstrapOrganization(TestbedEventBootstrapOrganization {
                first_user_id: candidate,
                ..
            }) if *candidate == user => {
                return UserProfile::Admin;
            }
            TestbedEvent::NewUser(TestbedEventNewUser {
                user_id: candidate,
                initial_profile,
                ..
            }) if *candidate == user => {
                return *initial_profile;
            }
            TestbedEvent::UpdateUserProfile(TestbedEventUpdateUserProfile {
                user: candidate,
                profile,
                ..
            }) if *candidate == user => {
                return *profile;
            }
            _ => (),
        }
    }
    panic!("User {} doesn't exist", user);
}

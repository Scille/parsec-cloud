// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{test_register_send_hook, test_send_hook_realm_get_keys_bundle};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::{CertifListWorkspaceTimelineError, WorkspaceTimelineEntry};

use super::utils::certificates_ops_factory;

/// Create a realm with all kinds of timeline entries:
/// creation (+ initial naming), sharing, role change, unsharing, renaming,
/// archiving, deletion planned and restoring.
fn build_realm_with_all_entries(
    builder: &mut TestbedTemplateBuilder,
) -> (VlobID, Vec<WorkspaceTimelineEntry>) {
    let wkp_name1: EntryName = "wkp1".parse().unwrap();
    let wkp_name2: EntryName = "wkp2".parse().unwrap();
    let deletion_date = DateTime::from_ymd_hms_us(2100, 1, 1, 0, 0, 0, 0).unwrap();

    let mut expected = vec![];

    // Creation
    let realm_id = builder.new_realm("alice").map(|event| {
        expected.push(WorkspaceTimelineEntry::Created {
            timestamp: event.timestamp,
            author: event.author,
            initial_name: Some(wkp_name1.clone()),
        });
        event.realm_id
    });
    // Initial key rotation and naming are part of the creation, hence no entry
    builder.rotate_key_realm(realm_id);
    builder.rename_realm(realm_id, wkp_name1.clone());

    let bob_id = builder.new_user("bob").map(|event| event.user_id);

    // Sharing, role change and unsharing
    for (previous_role, new_role) in [
        (None, Some(RealmRole::Contributor)),
        (Some(RealmRole::Contributor), Some(RealmRole::Manager)),
        (Some(RealmRole::Manager), None),
    ] {
        builder
            .share_realm(realm_id, bob_id, new_role)
            .map(|event| {
                expected.push(WorkspaceTimelineEntry::UserRoleChanged {
                    timestamp: event.timestamp,
                    author: event.author,
                    user_id: bob_id,
                    previous_role,
                    new_role,
                })
            });
    }

    // Renaming
    builder
        .rename_realm(realm_id, wkp_name2.clone())
        .map(|event| {
            expected.push(WorkspaceTimelineEntry::Renamed {
                timestamp: event.timestamp,
                author: event.author,
                previous_name: Some(wkp_name1.clone()),
                new_name: Some(wkp_name2.clone()),
            })
        });

    // Archiving
    builder
        .archive_realm(realm_id, RealmArchivingConfiguration::Archived)
        .map(|event| {
            expected.push(WorkspaceTimelineEntry::Archived {
                timestamp: event.timestamp,
                author: event.author,
            })
        });
    builder
        .archive_realm(
            realm_id,
            RealmArchivingConfiguration::DeletionPlanned { deletion_date },
        )
        .map(|event| {
            expected.push(WorkspaceTimelineEntry::DeletionPlanned {
                timestamp: event.timestamp,
                author: event.author,
                deletion_date,
            })
        });
    builder
        .archive_realm(realm_id, RealmArchivingConfiguration::Available)
        .map(|event| {
            expected.push(WorkspaceTimelineEntry::Restored {
                timestamp: event.timestamp,
                author: event.author,
            })
        });

    builder.certificates_storage_fetch_certificates("alice@dev1");

    (realm_id, expected)
}

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let (realm_id, expected) = env.customize(build_realm_with_all_entries).await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    // Names are encrypted, hence the keys bundle must be fetched to decrypt them
    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, realm_id),
    );

    let res = ops.list_workspace_timeline(realm_id).await.unwrap();

    p_assert_eq!(res, expected);
}

#[parsec_test(testbed = "minimal")]
async fn names_cannot_be_decrypted(env: &TestbedEnv) {
    let (realm_id, expected) = env.customize(build_realm_with_all_entries).await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    // No send hook registered: we are offline, so the keys bundle cannot be fetched.
    // This is not an error, the entries are still listed but without the names.
    let expected = expected
        .into_iter()
        .map(|entry| match entry {
            WorkspaceTimelineEntry::Created {
                timestamp, author, ..
            } => WorkspaceTimelineEntry::Created {
                timestamp,
                author,
                initial_name: None,
            },
            WorkspaceTimelineEntry::Renamed {
                timestamp, author, ..
            } => WorkspaceTimelineEntry::Renamed {
                timestamp,
                author,
                previous_name: None,
                new_name: None,
            },
            entry => entry,
        })
        .collect::<Vec<_>>();

    let res = ops.list_workspace_timeline(realm_id).await.unwrap();

    p_assert_eq!(res, expected);
}

#[parsec_test(testbed = "minimal")]
async fn never_named(env: &TestbedEnv) {
    let (realm_id, expected) = env
        .customize(|builder| {
            let (realm_id, expected) = builder.new_realm("alice").map(|event| {
                (
                    event.realm_id,
                    vec![WorkspaceTimelineEntry::Created {
                        timestamp: event.timestamp,
                        author: event.author,
                        initial_name: None,
                    }],
                )
            });
            builder.rotate_key_realm(realm_id);

            builder.certificates_storage_fetch_certificates("alice@dev1");

            (realm_id, expected)
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops.list_workspace_timeline(realm_id).await.unwrap();

    p_assert_eq!(res, expected);
}

#[parsec_test(testbed = "minimal")]
async fn unknown_realm(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops
        .list_workspace_timeline(VlobID::default())
        .await
        .unwrap();

    assert!(res.is_empty());
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops
        .list_workspace_timeline(VlobID::default())
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifListWorkspaceTimelineError::Stopped);
}

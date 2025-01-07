// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, sync::Arc};

use libparsec_types::prelude::*;

use crate::TestbedTemplate;

/// WorkspaceHistory contains:
/// - 3 users: `alice` (admin), `bob` (regular) and `mallory` (regular)
/// - 3 devices: `alice@dev1`, `bob@dev1` and `mallory@dev1` (every device has its local
///   storage up-to-date regarding the certificates)
/// - On 2001-01-01, Alice creates and bootstraps workspace `wksp1`
/// - On 2001-01-02, Alice uploads workspace manifest in v1 (empty)
/// - On 2001-01-03T01:00:00, Alice uploads file manifest `bar.txt` in v1 (containing "Hello v1")
/// - On 2001-01-03T02:00:00, Alice uploads folder manifest `foo` in v1 (empty)
/// - On 2001-01-03T03:00:00, Alice uploads workspace manifest in v2 (containing `foo` and `bar.txt`)
/// - On 2001-01-04, Alice gives Bob access to the workspace (as OWNER)
/// - On 2001-01-05, Alice gives Mallory access to the workspace (as CONTRIBUTOR)
/// - On 2001-01-06, Bob removes access to the workspace from Alice
/// - On 2001-01-07, Mallory uploads file manifest `bar.txt` in v2 (containing "Hello v2 world", stored on 2 blocks)
/// - On 2001-01-08T01:00:00, Mallory uploads folder manifest `foo` in v2 (containing `spam` and `egg.txt`)
/// - On 2001-01-08T02:00:00, Mallory uploads file manifest `foo/egg.txt` in v1 (empty file)
/// - On 2001-01-08T03:00:00, Mallory uploads folder manifest `foo/spam` in v1 (empty)
/// - On 2001-01-09, Bob uploads workspace manifest in v3 with renames `foo`->`foo2` and `bar.txt`->`bar2.txt`
/// - On 2001-01-10, Bob uploads file manifest `bar.txt` (now named `bar2.txt`) in v3 with content "Hello v3"
/// - On 2001-01-11, Bob uploads file manifest `foo/egg.txt` in v2 with content "v2"
/// - On 2001-01-12, Bob uploads folder manifest `foo` (now named `foo2`) in v3 with rename `egg.txt`->`egg2.txt` and removed `spam`
/// - On 2001-01-30, Bob gives back access to the workspace to Alice (as READER)
///
/// In the end we have the following per-entry history:
///   - `/`
///     - 2001-01-02: v1 from Alice, content: []
///     - 2001-01-03T03:00:00: v2 from Alice, content: ["foo", "bar.txt"]
///     - 2001-01-09: v3 from Bob, content: ["foo2", "bar2.txt"]
/// - `bar.txt`:
///   - 2001-01-03T01:00:00: v1 from Alice, content: "Hello v1"
///   - 2001-01-07: v2 from Alice, content: "Hello v2 world"
///   - 2001-01-10: v3 from Bob, content "Hello v3"
/// - `foo`:
///   - 2001-01-03T02:00:00: v1 from Alice, content: []
///   - 2001-01-08T01:00:00: v2 from Mallory, content ["spam", "egg.txt"],
///     with children not existing themselves yet !
///   - 2001-01-12: v3 from Bob, content ["egg2.txt"]
/// - `foo/egg.txt`:
///   - 2001-01-08T02:00:00: v1 from Mallory, content ""
///   - 2001-01-11: v2 from Bob, content "v2"
/// - `foo/spam`:
///   - 2001-01-08T03:00:00: v1 from Mallory, content: []
pub(crate) fn generate() -> Arc<TestbedTemplate> {
    // If you change something here:
    // - Update this function's docstring
    // - Update `server/tests/common/client.py::WorkspaceHistoryOrgRpcClients`s docstring

    let mut builder = TestbedTemplate::from_builder("workspace_history");

    // `Hello vX` is 8 bytes long, so we can store is on a single block
    let block_size: Blocksize = 8.try_into().unwrap();

    // 1) Create user & devices

    builder.bootstrap_organization("alice"); // alice@dev1
    builder
        .new_user("bob")
        .with_initial_profile(UserProfile::Standard); // bob@dev1
    builder
        .new_user("mallory")
        .with_initial_profile(UserProfile::Outsider); // mallory@dev1

    // 2) Create the workspace and its history

    let wksp1_bar_txt_id = builder.counters.next_entry_id();
    let wksp1_foo_id = builder.counters.next_entry_id();
    let wksp1_foo_egg_txt_id = builder.counters.next_entry_id();
    let wksp1_foo_spam_id = builder.counters.next_entry_id();
    builder.store_stuff("wksp1_bar_txt_id", &wksp1_bar_txt_id);
    builder.store_stuff("wksp1_foo_id", &wksp1_foo_id);
    builder.store_stuff("wksp1_foo_egg_txt_id", &wksp1_foo_egg_txt_id);
    builder.store_stuff("wksp1_foo_spam_id", &wksp1_foo_spam_id);

    // On 2001-01-01, Alice creates and bootstraps workspace `wksp1`
    let wksp1_created_timestamp = "2001-01-01T00:00:00Z".parse::<DateTime>().unwrap();
    let wksp1_bootstrapped_timestamp = "2001-01-01T00:00:02Z".parse::<DateTime>().unwrap();

    let wksp1_id = builder
        .new_realm("alice")
        .customize(|e| e.timestamp = wksp1_created_timestamp)
        .map(|e| e.realm_id);
    builder
        .rotate_key_realm(wksp1_id)
        .customize(|e| e.timestamp = "2001-01-01T00:00:01Z".parse().unwrap());
    let wksp1_name = builder
        .rename_realm(wksp1_id, "wksp1")
        .customize(|e| e.timestamp = wksp1_bootstrapped_timestamp)
        .map(|e| e.name.clone());

    builder.store_stuff("wksp1_id", &wksp1_id);
    builder.store_stuff("wksp1_name", &wksp1_name);
    builder.store_stuff("wksp1_created_timestamp", &wksp1_created_timestamp);
    builder.store_stuff(
        "wksp1_bootstrapped_timestamp",
        &wksp1_bootstrapped_timestamp,
    );

    // On 2001-01-02, Alice uploads workspace manifest in v1 (empty)

    let wksp1_v1_timestamp = "2001-01-02T00:00:00Z".parse::<DateTime>().unwrap();

    builder
        .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id)
        .customize(|e| Arc::make_mut(&mut e.manifest).timestamp = wksp1_v1_timestamp);

    builder.store_stuff("wksp1_v1_timestamp", &wksp1_v1_timestamp);

    // On 2001-01-03T01:00:00, Alice uploads file manifest `bar.txt` in v1 (containing "Hello v1")

    let wksp1_bar_txt_v1_timestamp = "2001-01-03T01:00:00Z".parse::<DateTime>().unwrap();
    let bar_txt_v1_content = b"Hello v1";

    let bar_txt_v1_block_access = builder
        .create_block("alice@dev1", wksp1_id, bar_txt_v1_content.as_ref())
        .as_block_access(0);
    builder.store_stuff("wksp1_bar_txt_v1_block_access", &bar_txt_v1_block_access);

    builder
        .create_or_update_file_manifest_vlob(
            "alice@dev1",
            wksp1_id,
            Some(wksp1_bar_txt_id),
            wksp1_id,
        )
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.blocksize = block_size;
            manifest.timestamp = wksp1_bar_txt_v1_timestamp;
            manifest.size = bar_txt_v1_block_access.size.get();
            manifest.blocks = vec![bar_txt_v1_block_access];
        });

    builder.store_stuff("wksp1_bar_txt_v1_timestamp", &wksp1_bar_txt_v1_timestamp);

    // On 2001-01-03T02:00:00, Alice uploads folder manifest `foo` in v1 (empty)

    let wksp1_foo_v1_timestamp = "2001-01-03T02:00:00Z".parse::<DateTime>().unwrap();

    builder
        .create_or_update_folder_manifest_vlob("alice@dev1", wksp1_id, Some(wksp1_foo_id), wksp1_id)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.timestamp = wksp1_foo_v1_timestamp;
        });

    builder.store_stuff("wksp1_foo_v1_timestamp", &wksp1_foo_v1_timestamp);

    // On 2001-01-03T03:00:00, Alice uploads workspace manifest in v2 (containing `foo` and `bar.txt`)

    let wksp1_v2_timestamp = "2001-01-03T03:00:00Z".parse::<DateTime>().unwrap();

    builder
        .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.timestamp = wksp1_v2_timestamp;
            manifest.children = HashMap::from_iter([
                ("foo".parse().unwrap(), wksp1_foo_id),
                ("bar.txt".parse().unwrap(), wksp1_bar_txt_id),
            ]);
        });

    builder.store_stuff("wksp1_v2_timestamp", &wksp1_v2_timestamp);

    // On 2001-01-04, Alice gives Bob access to the workspace (as OWNER)

    builder
        .share_realm(wksp1_id, "bob", Some(RealmRole::Owner))
        .customize(|e| e.timestamp = "2001-01-04T00:00:00Z".parse().unwrap());

    // On 2001-01-05, Alice gives Mallory access to the workspace (as CONTRIBUTOR)

    builder
        .share_realm(wksp1_id, "mallory", Some(RealmRole::Contributor))
        .customize(|e| e.timestamp = "2001-01-05T00:00:00Z".parse().unwrap());

    // On 2001-01-06, Bob removes access to the workspace from Alice

    builder
        .share_realm(wksp1_id, "alice", None)
        .customize(|e| e.timestamp = "2001-01-06T00:00:00Z".parse().unwrap());

    // On 2001-01-07, Mallory uploads file manifest `bar.txt` in v2 (containing "Hello v2 world", stored on 2 blocks)

    let wksp1_bar_txt_v2_timestamp = "2001-01-07T00:00:00Z".parse::<DateTime>().unwrap();
    let bar_txt_v2_content = b"Hello v2 world";

    let bar_txt_v2_block1_access = builder
        .create_block(
            "mallory@dev1",
            wksp1_id,
            bar_txt_v2_content.as_ref()[..*block_size as _].as_ref(),
        )
        .as_block_access(0);
    let bar_txt_v2_block2_access = builder
        .create_block(
            "mallory@dev1",
            wksp1_id,
            bar_txt_v2_content.as_ref()[*block_size as _..].as_ref(),
        )
        .as_block_access(*block_size);
    assert_eq!(bar_txt_v2_block2_access.offset, 8);
    assert_eq!(bar_txt_v2_block2_access.size.get(), 6);
    assert_eq!(bar_txt_v2_block1_access.offset, 0);
    assert_eq!(bar_txt_v2_block1_access.size.get(), 8);
    builder.store_stuff("wksp1_bar_txt_v2_block1_access", &bar_txt_v2_block1_access);
    builder.store_stuff("wksp1_bar_txt_v2_block2_access", &bar_txt_v2_block2_access);

    builder
        .create_or_update_file_manifest_vlob(
            "mallory@dev1",
            wksp1_id,
            Some(wksp1_bar_txt_id),
            wksp1_id,
        )
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.blocksize = block_size;
            manifest.timestamp = wksp1_bar_txt_v2_timestamp;
            manifest.size = bar_txt_v2_content.len() as _;
            manifest.blocks = vec![bar_txt_v2_block1_access, bar_txt_v2_block2_access];
        });

    builder.store_stuff("wksp1_bar_txt_v2_timestamp", &wksp1_bar_txt_v2_timestamp);

    // On 2001-01-08T01:00:00, Mallory uploads folder manifest `foo` in v2 (containing `spam` and `egg.txt`)

    let wksp1_foo_v2_timestamp = "2001-01-08T01:00:00Z".parse::<DateTime>().unwrap();

    builder
        .create_or_update_folder_manifest_vlob("mallory@dev1", wksp1_id, Some(wksp1_foo_id), None)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.timestamp = wksp1_foo_v2_timestamp;
            manifest.children = HashMap::from_iter([
                ("spam".parse().unwrap(), wksp1_foo_spam_id),
                ("egg.txt".parse().unwrap(), wksp1_foo_egg_txt_id),
            ]);
        });

    builder.store_stuff("wksp1_foo_v2_timestamp", &wksp1_foo_v2_timestamp);

    // On 2001-01-08T02:00:00, Mallory uploads file manifest `foo/egg.txt` in v1 (empty file)

    let wksp1_foo_egg_txt_v1_timestamp = "2001-01-08T02:00:00Z".parse::<DateTime>().unwrap();

    builder
        .create_or_update_file_manifest_vlob(
            "mallory@dev1",
            wksp1_id,
            Some(wksp1_foo_egg_txt_id),
            wksp1_foo_id,
        )
        .customize(|e| Arc::make_mut(&mut e.manifest).timestamp = wksp1_foo_egg_txt_v1_timestamp);

    builder.store_stuff(
        "wksp1_foo_egg_txt_v1_timestamp",
        &wksp1_foo_egg_txt_v1_timestamp,
    );

    // On 2001-01-08T03:00:00, Mallory uploads folder manifest `foo/spam` in v1 (empty)

    let wksp1_foo_spam_v1_timestamp = "2001-01-08T03:00:00Z".parse::<DateTime>().unwrap();

    builder
        .create_or_update_folder_manifest_vlob(
            "mallory@dev1",
            wksp1_id,
            Some(wksp1_foo_spam_id),
            wksp1_foo_id,
        )
        .customize(|e| Arc::make_mut(&mut e.manifest).timestamp = wksp1_foo_spam_v1_timestamp);

    builder.store_stuff("wksp1_foo_spam_v1_timestamp", &wksp1_foo_spam_v1_timestamp);
    // Alias for this timestamp to simply know when the parent manifest children become valid
    builder.store_stuff(
        "wksp1_foo_v2_children_available_timestamp",
        &wksp1_foo_spam_v1_timestamp,
    );

    // On 2001-01-09, Bob uploads workspace manifest in v3 with renames `foo`->`foo2` and `bar.txt`->`bar2.txt`

    let wksp1_v3_timestamp = "2001-01-09T00:00:00Z".parse::<DateTime>().unwrap();

    builder
        .create_or_update_workspace_manifest_vlob("bob@dev1", wksp1_id)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.timestamp = wksp1_v3_timestamp;
            manifest.children = HashMap::from_iter([
                ("foo2".parse().unwrap(), wksp1_foo_id),
                ("bar2.txt".parse().unwrap(), wksp1_bar_txt_id),
            ]);
        });

    builder.store_stuff("wksp1_v3_timestamp", &wksp1_v3_timestamp);

    // On 2001-01-10, Bob uploads file manifest `bar.txt` (now named `bar2.txt`) in v3 with content "Hello v3"

    let wksp1_bar_txt_v3_timestamp = "2001-01-10T00:00:00Z".parse::<DateTime>().unwrap();
    let bar_txt_v3_content = b"Hello v3";

    let bar_txt_v3_block_access = builder
        .create_block("bob@dev1", wksp1_id, bar_txt_v3_content.as_ref())
        .as_block_access(0);
    builder.store_stuff("wksp1_bar_txt_v3_block_access", &bar_txt_v3_block_access);

    builder
        .create_or_update_file_manifest_vlob("bob@dev1", wksp1_id, Some(wksp1_bar_txt_id), wksp1_id)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.blocksize = block_size;
            manifest.timestamp = wksp1_bar_txt_v3_timestamp;
            manifest.size = bar_txt_v3_content.len() as _;
            manifest.blocks = vec![bar_txt_v3_block_access];
        });

    builder.store_stuff("wksp1_bar_txt_v3_timestamp", &wksp1_bar_txt_v3_timestamp);

    // On 2001-01-11, Bob uploads file manifest `foo/egg.txt` in v2 with content "v2"

    let wksp1_foo_egg_txt_v2_timestamp = "2001-01-11T00:00:00Z".parse::<DateTime>().unwrap();
    let foo_egg_txt_v2_content = b"v2";

    let foo_egg_txt_v2_block_access = builder
        .create_block("bob@dev1", wksp1_id, foo_egg_txt_v2_content.as_ref())
        .as_block_access(0);
    builder.store_stuff(
        "wksp1_foo_egg_txt_v2_block_access",
        &foo_egg_txt_v2_block_access,
    );

    builder
        .create_or_update_file_manifest_vlob("bob@dev1", wksp1_id, Some(wksp1_bar_txt_id), wksp1_id)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.blocksize = block_size;
            manifest.timestamp = wksp1_foo_egg_txt_v2_timestamp;
            manifest.size = foo_egg_txt_v2_content.len() as _;
            manifest.blocks = vec![foo_egg_txt_v2_block_access];
        });

    builder.store_stuff(
        "wksp1_foo_egg_txt_v2_timestamp",
        &wksp1_foo_egg_txt_v2_timestamp,
    );

    // On 2001-01-12, Bob uploads folder manifest `foo` (now named `foo2`) in v3 with rename `egg.txt`->`egg2.txt` and removed `spam`

    let wksp1_foo_v3_timestamp = "2001-01-12T00:00:00Z".parse::<DateTime>().unwrap();

    builder
        .create_or_update_folder_manifest_vlob("bob@dev1", wksp1_id, Some(wksp1_foo_id), None)
        .customize(|e| {
            let manifest = Arc::make_mut(&mut e.manifest);
            manifest.timestamp = wksp1_foo_v3_timestamp;
            manifest.children =
                HashMap::from_iter([("egg2.txt".parse().unwrap(), wksp1_foo_egg_txt_id)]);
        });

    builder.store_stuff("wksp1_foo_v3_timestamp", &wksp1_foo_v3_timestamp);

    // On 2001-01-30, Bob gives back access to the workspace to Alice (as READER)

    let wksp1_alice_gets_back_access_timestamp =
        "2001-01-30T00:00:00Z".parse::<DateTime>().unwrap();

    builder
        .share_realm(wksp1_id, "alice", Some(RealmRole::Reader))
        .customize(|e| {
            e.timestamp = wksp1_alice_gets_back_access_timestamp;
        });

    builder.store_stuff(
        "wksp1_alice_gets_back_access_timestamp",
        &wksp1_alice_gets_back_access_timestamp,
    );

    // 3) Initialize client storages for alice@dev1, bob@dev1 and mallory@dev1

    builder
        .counters
        .set_current_timestamp("2001-02-01T00:00:00Z".parse().unwrap());

    macro_rules! fetch_and_update_local_storage {
        ($device: literal) => {
            builder.certificates_storage_fetch_certificates($device);
            builder
                .user_storage_local_update($device)
                .update_local_workspaces_with_fetched_certificates();
            builder.user_storage_fetch_realm_checkpoint($device);
            builder.workspace_data_storage_fetch_realm_checkpoint($device, wksp1_id);
        };
    }
    fetch_and_update_local_storage!("alice@dev1");
    fetch_and_update_local_storage!("bob@dev1");
    fetch_and_update_local_storage!("mallory@dev1");

    builder.finalize()
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_vlob_read_batch,
};
use libparsec_platform_async::channel::{self, RecvError};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{DataAccessStrategy, StartWorkspaceHistoryOpsError};
use crate::workspace_history::{WorkspaceHistoryEntryStat, WorkspaceHistorySearchMatch};

async fn drain_sorted(rx: &channel::Receiver<WorkspaceHistorySearchMatch>) -> Vec<String> {
    let mut paths = Vec::new();
    while let Ok(m) = rx.recv_async().await {
        paths.push(m.path.to_string());
    }
    paths.sort();
    paths
}

async fn drain_matches(
    rx: &channel::Receiver<WorkspaceHistorySearchMatch>,
) -> Vec<WorkspaceHistorySearchMatch> {
    let mut results = Vec::new();
    while let Ok(m) = rx.recv_async().await {
        results.push(m);
    }
    results
}

// The `workspace_history` testbed workspace layout at
// `wksp1_foo_v2_children_available_timestamp`:
//
//   /bar.txt          (file)
//   /foo/             (folder)
//   /foo/egg.txt      (file)
//   /foo/spam/        (folder, empty)

/// Register the server hooks required for a full BFS traversal from the
/// workspace root. The BFS visits two levels:
///   1. children of `/`      → bar.txt and foo manifests
///   2. children of `/foo`   → egg.txt and spam manifests
///   3. children of `/foo/spam` → none (empty)
///
/// Note: the `/` manifest is always in cache after `start_workspace_history_ops_at`;
/// the `/foo` manifest is cached after level 1, so it is not re-fetched at level 2.
macro_rules! register_bfs_from_root_hooks {
    ($env:expr, $timestamp:expr, $wksp1_id:expr, $bar_txt_id:expr, $foo_id:expr, $egg_txt_id:expr, $spam_id:expr) => {
        test_register_sequence_of_send_hooks!(
            &$env.discriminant_dir,
            // Level 1: children of `/` (order is non-deterministic due to HashMap)
            test_send_hook_vlob_read_batch!($env, at: $timestamp, $wksp1_id, allowed: [$bar_txt_id, $foo_id]),
            test_send_hook_vlob_read_batch!($env, at: $timestamp, $wksp1_id, allowed: [$bar_txt_id, $foo_id]),
            // Level 2: children of `/foo` (foo manifest is already cached from level 1)
            test_send_hook_vlob_read_batch!($env, at: $timestamp, $wksp1_id, allowed: [$egg_txt_id, $spam_id]),
            test_send_hook_vlob_read_batch!($env, at: $timestamp, $wksp1_id, allowed: [$egg_txt_id, $spam_id]),
            // Level 3: `/foo/spam` is empty — no hooks needed
        )
    };
}

/// Register the server hooks required for a BFS traversal starting from `/foo`.
/// Here the `/foo` manifest is not yet cached, so `resolve_path` fetches it first.
macro_rules! register_bfs_from_foo_hooks {
    ($env:expr, $timestamp:expr, $wksp1_id:expr, $foo_id:expr, $egg_txt_id:expr, $spam_id:expr) => {
        test_register_sequence_of_send_hooks!(
            &$env.discriminant_dir,
            // resolve_path("/foo"): workspace root in cache → fetch `/foo` manifest
            test_send_hook_vlob_read_batch!($env, at: $timestamp, $wksp1_id, $foo_id),
            // Children of `/foo` (order is non-deterministic)
            test_send_hook_vlob_read_batch!($env, at: $timestamp, $wksp1_id, allowed: [$egg_txt_id, $spam_id]),
            test_send_hook_vlob_read_batch!($env, at: $timestamp, $wksp1_id, allowed: [$egg_txt_id, $spam_id]),
            // `/foo/spam` is empty — no hooks needed
        )
    };
}

#[parsec_test(testbed = "workspace_history")]
async fn empty_query_returns_all(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");

    let ops = match strategy
        .start_workspace_history_ops_at(env, timestamp)
        .await
    {
        Ok(ops) => ops,
        Err(StartWorkspaceHistoryOpsError::RealmExportNotSupportedOnWeb) => return,
    };

    if matches!(strategy, DataAccessStrategy::Server) {
        register_bfs_from_root_hooks!(
            env,
            timestamp,
            wksp1_id,
            wksp1_bar_txt_id,
            wksp1_foo_id,
            wksp1_foo_egg_txt_id,
            wksp1_foo_spam_id
        );
    }

    let search = Arc::clone(&ops).search("/".parse().unwrap(), "");
    let paths = drain_sorted(&search.results).await;

    p_assert_eq!(paths, ["/bar.txt", "/foo", "/foo/egg.txt", "/foo/spam"]);
}

#[parsec_test(testbed = "workspace_history")]
async fn match_by_name(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");

    let ops = match strategy
        .start_workspace_history_ops_at(env, timestamp)
        .await
    {
        Ok(ops) => ops,
        Err(StartWorkspaceHistoryOpsError::RealmExportNotSupportedOnWeb) => return,
    };

    if matches!(strategy, DataAccessStrategy::Server) {
        register_bfs_from_root_hooks!(
            env,
            timestamp,
            wksp1_id,
            wksp1_bar_txt_id,
            wksp1_foo_id,
            wksp1_foo_egg_txt_id,
            wksp1_foo_spam_id
        );
    }

    // "bar" only matches "/bar.txt"
    let search = Arc::clone(&ops).search("/".parse().unwrap(), "bar");
    p_assert_eq!(drain_sorted(&search.results).await, ["/bar.txt"]);
}

#[parsec_test(testbed = "workspace_history")]
async fn no_match_returns_empty(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");

    let ops = match strategy
        .start_workspace_history_ops_at(env, timestamp)
        .await
    {
        Ok(ops) => ops,
        Err(StartWorkspaceHistoryOpsError::RealmExportNotSupportedOnWeb) => return,
    };

    if matches!(strategy, DataAccessStrategy::Server) {
        register_bfs_from_root_hooks!(
            env,
            timestamp,
            wksp1_id,
            wksp1_bar_txt_id,
            wksp1_foo_id,
            wksp1_foo_egg_txt_id,
            wksp1_foo_spam_id
        );
    }

    let search = Arc::clone(&ops).search("/".parse().unwrap(), "zzzz");
    let paths = drain_sorted(&search.results).await;

    assert!(paths.is_empty(), "expected no results, got {paths:?}");
}

#[parsec_test(testbed = "workspace_history")]
async fn bfs_order_shallow_before_deep(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");

    let ops = match strategy
        .start_workspace_history_ops_at(env, timestamp)
        .await
    {
        Ok(ops) => ops,
        Err(StartWorkspaceHistoryOpsError::RealmExportNotSupportedOnWeb) => return,
    };

    if matches!(strategy, DataAccessStrategy::Server) {
        register_bfs_from_root_hooks!(
            env,
            timestamp,
            wksp1_id,
            wksp1_bar_txt_id,
            wksp1_foo_id,
            wksp1_foo_egg_txt_id,
            wksp1_foo_spam_id
        );
    }

    let search = Arc::clone(&ops).search("/".parse().unwrap(), "");
    let results = drain_matches(&search.results).await;

    p_assert_eq!(results.len(), 4);

    // The first two results must be root-level entries (depth 1: bar.txt, foo)
    let first_two_depths: Vec<usize> = results[..2].iter().map(|m| m.path.parts().len()).collect();
    p_assert_eq!(first_two_depths, [1, 1]);

    // The last two results must be entries nested inside /foo (depth 2)
    let last_two_depths: Vec<usize> = results[2..].iter().map(|m| m.path.parts().len()).collect();
    p_assert_eq!(last_two_depths, [2, 2]);
}

#[parsec_test(testbed = "workspace_history")]
async fn stat_type_matches_entry_kind(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");

    let ops = match strategy
        .start_workspace_history_ops_at(env, timestamp)
        .await
    {
        Ok(ops) => ops,
        Err(StartWorkspaceHistoryOpsError::RealmExportNotSupportedOnWeb) => return,
    };

    if matches!(strategy, DataAccessStrategy::Server) {
        register_bfs_from_root_hooks!(
            env,
            timestamp,
            wksp1_id,
            wksp1_bar_txt_id,
            wksp1_foo_id,
            wksp1_foo_egg_txt_id,
            wksp1_foo_spam_id
        );
    }

    let search = Arc::clone(&ops).search("/".parse().unwrap(), "");
    let results = drain_matches(&search.results).await;

    let find = |path_str: &str| {
        results
            .iter()
            .find(|m| m.path.to_string() == path_str)
            .unwrap_or_else(|| panic!("no result for {path_str}"))
    };

    p_assert_matches!(
        find("/bar.txt").stat,
        WorkspaceHistoryEntryStat::File { .. }
    );
    p_assert_matches!(find("/foo").stat, WorkspaceHistoryEntryStat::Folder { .. });
    p_assert_matches!(
        find("/foo/egg.txt").stat,
        WorkspaceHistoryEntryStat::File { .. }
    );
    p_assert_matches!(
        find("/foo/spam").stat,
        WorkspaceHistoryEntryStat::Folder { .. }
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn subdirectory_scope(
    #[values(DataAccessStrategy::Server, DataAccessStrategy::RealmExport)]
    strategy: DataAccessStrategy,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");

    let ops = match strategy
        .start_workspace_history_ops_at(env, timestamp)
        .await
    {
        Ok(ops) => ops,
        Err(StartWorkspaceHistoryOpsError::RealmExportNotSupportedOnWeb) => return,
    };

    // Empty query from /foo: should see only /foo's descendants, not /bar.txt or /foo itself.
    // Only the first search needs hooks for the server strategy; subsequent searches
    // on the same ops instance find the manifests already in cache.
    if matches!(strategy, DataAccessStrategy::Server) {
        register_bfs_from_foo_hooks!(
            env,
            timestamp,
            wksp1_id,
            wksp1_foo_id,
            wksp1_foo_egg_txt_id,
            wksp1_foo_spam_id
        );
    }
    let search = Arc::clone(&ops).search("/foo".parse().unwrap(), "");
    p_assert_eq!(
        drain_sorted(&search.results).await,
        ["/foo/egg.txt", "/foo/spam"]
    );

    // A name-based query scoped to /foo (manifests are now cached).
    let search = Arc::clone(&ops).search("/foo".parse().unwrap(), "egg");
    p_assert_eq!(drain_sorted(&search.results).await, ["/foo/egg.txt"]);

    // A query that would match /bar.txt from root returns nothing from /foo.
    let search = Arc::clone(&ops).search("/foo".parse().unwrap(), "bar");
    assert!(
        drain_sorted(&search.results).await.is_empty(),
        "bar.txt must not appear when searching under /foo"
    );
}

#[parsec_test(testbed = "workspace_history")]
async fn abort_on_drop(env: &TestbedEnv) {
    let timestamp: DateTime = *env
        .template
        .get_stuff("wksp1_foo_v2_children_available_timestamp");

    let ops = match DataAccessStrategy::RealmExport
        .start_workspace_history_ops_at(env, timestamp)
        .await
    {
        Ok(ops) => ops,
        Err(StartWorkspaceHistoryOpsError::RealmExportNotSupportedOnWeb) => return,
    };

    let search = Arc::clone(&ops).search("/".parse().unwrap(), "");

    let rx = search.results.clone();
    drop(search); // aborts the task

    // Since it is a rendez-vous channel, `rx` doesn't contain anything and is
    // closed once the task has been aborted. Use recv_async so the executor can
    // actually run and cancel the task (synchronous recv would deadlock in a
    // single-threaded executor).
    p_assert_matches!(rx.recv_async().await, Err(RecvError::Disconnected));
}

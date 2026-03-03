// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_platform_async::channel::{self, RecvError};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::workspace::{EntryStat, WorkspaceSearchMatch};

async fn drain_sorted(rx: &channel::Receiver<WorkspaceSearchMatch>) -> Vec<String> {
    let mut paths = Vec::new();
    while let Ok(m) = rx.recv_async().await {
        paths.push(m.path.to_string());
    }
    paths.sort();
    paths
}

async fn drain_matches(rx: &channel::Receiver<WorkspaceSearchMatch>) -> Vec<WorkspaceSearchMatch> {
    let mut results = Vec::new();
    while let Ok(m) = rx.recv_async().await {
        results.push(m);
    }
    results
}

// The `minimal_client_ready` testbed workspace layout:
//
//   /bar.txt          (file)
//   /foo/             (folder)
//   /foo/egg.txt      (file)
//   /foo/spam/        (folder)

#[parsec_test(testbed = "minimal_client_ready")]
async fn empty_query_returns_all(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops =
        Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await);

    let root: FsPath = "/".parse().unwrap();
    let search = Arc::clone(&ops).search(root.clone(), "");
    let paths = drain_sorted(&search.results).await;

    p_assert_eq!(paths, ["/bar.txt", "/foo", "/foo/egg.txt", "/foo/spam"]);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn match_by_name(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops =
        Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await);

    // "bar" is a subsequence of "/bar.txt" but does not appear in any other path
    let root: FsPath = "/".parse().unwrap();
    let search = Arc::clone(&ops).search(root.clone(), "bar");
    p_assert_eq!(drain_sorted(&search.results).await, ["/bar.txt"]);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn no_match_returns_empty(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops =
        Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await);

    let root: FsPath = "/".parse().unwrap();
    let search = Arc::clone(&ops).search(root.clone(), "zzzz");
    let paths = drain_sorted(&search.results).await;

    assert!(paths.is_empty(), "expected no results, got {paths:?}");
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn bfs_order_shallow_before_deep(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops =
        Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await);

    let root: FsPath = "/".parse().unwrap();
    let search = Arc::clone(&ops).search(root.clone(), "");
    let results = drain_matches(&search.results).await;

    p_assert_eq!(results.len(), 4);

    // The first two results must be root-level entries (depth 1: bar.txt, foo)
    let first_two_depths: Vec<usize> = results[..2].iter().map(|m| m.path.parts().len()).collect();
    p_assert_eq!(first_two_depths, [1, 1]);

    // The last two results must be entries nested inside /foo (depth 2)
    let last_two_depths: Vec<usize> = results[2..].iter().map(|m| m.path.parts().len()).collect();
    p_assert_eq!(last_two_depths, [2, 2]);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stat_type_matches_entry_kind(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops =
        Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await);

    let root: FsPath = "/".parse().unwrap();
    let search = Arc::clone(&ops).search(root.clone(), "");
    let results = drain_matches(&search.results).await;

    let find = |path_str: &str| {
        results
            .iter()
            .find(|m| m.path.to_string() == path_str)
            .unwrap_or_else(|| panic!("no result for {path_str}"))
    };

    p_assert_matches!(find("/bar.txt").stat, EntryStat::File { .. });
    p_assert_matches!(find("/foo").stat, EntryStat::Folder { .. });
    p_assert_matches!(find("/foo/egg.txt").stat, EntryStat::File { .. });
    p_assert_matches!(find("/foo/spam").stat, EntryStat::Folder { .. });
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn subdirectory_scope(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops =
        Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await);

    let foo: FsPath = "/foo".parse().unwrap();

    // Empty query from /foo: should see only /foo's descendants, not /bar.txt
    // or /foo itself.
    let search = Arc::clone(&ops).search(foo.clone(), "");
    p_assert_eq!(
        drain_sorted(&search.results).await,
        ["/foo/egg.txt", "/foo/spam"]
    );

    // A name-based query scoped to /foo.
    let search = Arc::clone(&ops).search(foo.clone(), "egg");
    p_assert_eq!(drain_sorted(&search.results).await, ["/foo/egg.txt"]);

    // A query that would match /bar.txt from root returns nothing from /foo.
    let search = Arc::clone(&ops).search(foo.clone(), "bar");
    assert!(
        drain_sorted(&search.results).await.is_empty(),
        "bar.txt must not appear when searching under /foo"
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn abort_on_drop(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let alice = env.local_device("alice@dev1");
    let ops =
        Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await);

    let root: FsPath = "/".parse().unwrap();
    let search = Arc::clone(&ops).search(root.clone(), "");

    let rx = search.results.clone();
    drop(search); // aborts the task

    // Since it is a rendez-vous channel, `rx` doesn't contain anything and is
    // closed once the task has been aborted. Use recv_async so the executor can
    // actually run and cancel the task (synchronous recv would deadlock in a
    // single-threaded executor).
    p_assert_matches!(rx.recv_async().await, Err(RecvError::Disconnected));
}

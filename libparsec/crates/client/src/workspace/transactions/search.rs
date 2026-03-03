// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::VecDeque, sync::Arc};

use libparsec_platform_async::channel;
use libparsec_types::prelude::*;

use crate::{
    search::FuzzyMatch,
    workspace::{transactions::EntryStat, WorkspaceOps, WorkspaceStatFolderChildrenError},
};

/// A single entry whose path fuzzy-matched the search query.
#[derive(Debug, Clone)]
pub struct WorkspaceSearchMatch {
    pub path: FsPath,
    pub stat: EntryStat,
    /// Relevance score — higher means a better match.
    pub score: u32,
    /// Indices into the *characters* of `path.to_string()` that were matched,
    /// in ascending order. Useful for rendering highlights in a UI.
    pub match_positions: Vec<u32>,
}

/// An in-progress fuzzy search, note the underlaying task is aborted once this
/// struct is dropped.
#[derive(Debug)]
pub struct WorkspaceSearch {
    abort_handle: libparsec_platform_async::AbortHandle,
    pub results: channel::Receiver<WorkspaceSearchMatch>,
}

impl Drop for WorkspaceSearch {
    fn drop(&mut self) {
        self.abort_handle.abort();
    }
}

/// The traversal visits directories breadth-first, so shallower (typically
/// more relevant) entries appear in the channel first.  Only entries that are
/// strict descendants of `root` are visited; `root` itself is not emitted.
///
/// Note dropping the returned [`WorkspaceSearch`] aborts the underlying search task.
pub fn search(ops: Arc<WorkspaceOps>, root: FsPath, query: &str) -> WorkspaceSearch {
    // Use a rendez-vous channel (i.e. with a 0 capacity) to only search if the
    // caller is actually interested in the result (i.e. fetches it).
    let (tx, rx) = channel::bounded(0);

    let fuzzy_match = FuzzyMatch::new(query);

    let join_handle = libparsec_platform_async::spawn(search_task(ops, root, fuzzy_match, tx));

    WorkspaceSearch {
        abort_handle: join_handle.abort_handle(),
        results: rx,
    }
}

async fn search_task(
    ops: Arc<WorkspaceOps>,
    root: FsPath,
    fuzzy_match: FuzzyMatch,
    tx: channel::Sender<WorkspaceSearchMatch>,
) {
    // Note the traversal visits directories breadth-first, so shallower (typically
    // more relevant) entries appear in the channel first.

    let mut to_search_subdirs: VecDeque<FsPath> = VecDeque::new();
    to_search_subdirs.push_back(root);

    while let Some(dir_path) = to_search_subdirs.pop_front() {
        let children = match ops.stat_folder_children(&dir_path).await {
            Ok(c) => c,
            // The workspace was stopped, no point continuing.
            Err(WorkspaceStatFolderChildrenError::Stopped) => return,
            // Anything else (permission denied, bad manifest, …), skip this
            // directory and keep going so the rest of the tree is still visited.
            Err(_) => continue,
        };

        for (name, stat) in children {
            let child_path = dir_path.join(name);

            if let Some((score, match_positions)) = fuzzy_match.matches(&child_path) {
                let m = WorkspaceSearchMatch {
                    path: child_path.clone(),
                    stat: stat.clone(),
                    score,
                    match_positions,
                };
                if tx.send_async(m).await.is_err() {
                    // A send error means the receiver was dropped
                    return;
                }
            }

            if matches!(stat, EntryStat::Folder { .. }) {
                to_search_subdirs.push_back(child_path);
            }
        }
    }
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{pyclass, pymethods, PyObject, Python};
use std::collections::HashSet;

use crate::{
    data::BlockAccess,
    ids::{BlockID, EntryID},
};

#[pyclass]
pub(crate) struct FSBlockEventBus(libparsec::core_fs::FSBlockEventBus);

#[pymethods]
impl FSBlockEventBus {
    #[new]
    fn new() -> Self {
        Self(libparsec::core_fs::FSBlockEventBus::default())
    }

    fn connect_downloaded(&mut self, workspace_id: EntryID, handle_downloaded: PyObject) {
        let handle_downloaded = Box::new(move |block_access| {
            Python::with_gil(|py| {
                let _ = handle_downloaded.call1(py, (BlockAccess(block_access),));
            })
        });
        self.0.connect_downloaded(workspace_id.0, handle_downloaded);
    }

    fn connect_purged(&mut self, workspace_id: EntryID, handle_purged: PyObject) {
        let handle_purged = Box::new(move |block_ids: HashSet<_>| {
            Python::with_gil(|py| {
                let _ = handle_purged.call1(
                    py,
                    (block_ids.into_iter().map(BlockID).collect::<HashSet<_>>(),),
                );
            })
        });
        self.0.connect_purged(workspace_id.0, handle_purged)
    }

    fn disconnect_downloaded(&mut self, workspace_id: &EntryID) {
        self.0.disconnect_downloaded(&workspace_id.0);
    }

    fn disconnect_purged(&mut self, workspace_id: &EntryID) {
        self.0.disconnect_purged(&workspace_id.0);
    }

    fn send_downloaded(&self, workspace_id: &EntryID, block_access: BlockAccess) {
        self.0.send_downloaded(&workspace_id.0, block_access.0)
    }

    fn send_purged(&self, workspace_id: &EntryID, block_ids: HashSet<BlockID>) {
        self.0.send_purged(
            &workspace_id.0,
            block_ids.into_iter().map(|x| x.0).collect(),
        )
    }
}

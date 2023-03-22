// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::{HashMap, HashSet};

use libparsec_types::{BlockAccess, BlockID, EntryID};

type DownloadedCallBack = Box<dyn Fn(BlockAccess) + Send + Sync>;
type PurgedCallBack = Box<dyn Fn(HashSet<BlockID>) + Send + Sync>;

#[derive(Default)]
pub struct FSBlockEventBus {
    handle_downloaded: HashMap<EntryID, DownloadedCallBack>,
    handle_purged: HashMap<EntryID, PurgedCallBack>,
}

impl FSBlockEventBus {
    pub fn connect_downloaded(&mut self, workspace_id: EntryID, cb: DownloadedCallBack) {
        self.handle_downloaded.insert(workspace_id, cb);
    }

    pub fn connect_purged(&mut self, workspace_id: EntryID, cb: PurgedCallBack) {
        self.handle_purged.insert(workspace_id, cb);
    }

    pub fn disconnect_downloaded(&mut self, workspace_id: &EntryID) {
        self.handle_downloaded.remove(workspace_id);
    }

    pub fn disconnect_purged(&mut self, workspace_id: &EntryID) {
        self.handle_purged.remove(workspace_id);
    }

    pub fn send_downloaded(&self, workspace_id: &EntryID, block_access: BlockAccess) {
        if let Some(cb) = self.handle_downloaded.get(workspace_id) {
            cb(block_access)
        }
    }

    pub fn send_purged(&self, workspace_id: &EntryID, block_ids: HashSet<BlockID>) {
        if let Some(cb) = self.handle_purged.get(workspace_id) {
            cb(block_ids)
        }
    }
}

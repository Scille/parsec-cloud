# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Callable

from parsec._parsec import DateTime
from parsec.core.fs.workspacefs.sync_transactions import SyncTransactions
from parsec.core.fs.workspacefs.workspacefs import WorkspaceFS
from parsec.core.types import WorkspaceEntry, WorkspaceRole


class WorkspaceFSTimestamped(WorkspaceFS):
    def __init__(self, workspacefs: WorkspaceFS, timestamp: DateTime):
        self.workspace_id = workspacefs.workspace_id
        self.get_workspace_entry: Callable[[], WorkspaceEntry]
        if isinstance(workspacefs, WorkspaceFSTimestamped):
            self.get_workspace_entry = workspacefs.get_workspace_entry
        else:
            self.get_workspace_entry = self.timestamp_get_entry(workspacefs.get_workspace_entry)
        self.device = workspacefs.device
        self.local_storage = workspacefs.local_storage.to_timestamped(timestamp)
        self.backend_cmds = workspacefs.backend_cmds
        self.event_bus = workspacefs.event_bus
        self.preferred_language = workspacefs.preferred_language

        # Archiving attributes
        self._archiving_configuration = workspacefs._archiving_configuration
        self._archiving_configured_on = workspacefs._archiving_configured_on
        self._archiving_configured_by = workspacefs._archiving_configured_by
        self._archiving_configuration_timestamp = workspacefs._archiving_configuration_timestamp

        self.timestamp = timestamp

        self.remote_loader = workspacefs.remote_loader.to_timestamped(timestamp)
        self.transactions = SyncTransactions(
            self.workspace_id,
            self.get_workspace_entry,
            self.get_archiving_configuration,
            self.device,
            self.local_storage,
            self.remote_loader,
            self.event_bus,
            self.preferred_language,
        )

    def timestamp_get_entry(
        self, get_original_workspace_entry: Callable[[], WorkspaceEntry]
    ) -> Callable[[], WorkspaceEntry]:
        def get_timestamped_workspace_entry() -> WorkspaceEntry:
            return get_original_workspace_entry().evolve(role=WorkspaceRole.READER)

        return get_timestamped_workspace_entry

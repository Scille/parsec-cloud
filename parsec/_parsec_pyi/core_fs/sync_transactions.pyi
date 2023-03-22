# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec_pyi.ids import EntryID
from parsec._parsec_pyi.manifest import AnyRemoteManifest, BlockAccess

class ChangesAfterSync:
    added_blocks: set[BlockAccess]
    removed_blocks: set[BlockAccess]
    added_entries: set[EntryID]
    removed_entries: set[EntryID]

    @classmethod
    def from_manifests(
        cls,
        old_manifest: AnyRemoteManifest,
        new_manifest: AnyRemoteManifest,
    ) -> ChangesAfterSync: ...

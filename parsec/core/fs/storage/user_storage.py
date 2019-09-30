# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from pathlib import Path
from async_generator import asynccontextmanager

from parsec.core.fs.exceptions import FSLocalMissError
from parsec.core.fs.storage.manifest_storage import ManifestStorage
from parsec.core.types import EntryID, LocalDevice, LocalUserManifest


class UserStorage(ManifestStorage):
    """Storage for the user manifest.

    Provides a synchronous interface to the user manifest as it is used very often.
    """

    @property
    def user_manifest_id(self):
        return self.realm_id

    @classmethod
    @asynccontextmanager
    async def run(cls, device: LocalDevice, path: Path, user_manifest_id: EntryID):
        path /= "user_data.sqlite"
        async with super().run(device, path, user_manifest_id) as self:

            # Load the user manifest
            try:
                await self.get_manifest(self.user_manifest_id)
            except FSLocalMissError:
                pass
            else:
                assert self.user_manifest_id in self._cache

            yield self

    def get_user_manifest(self):
        try:
            return self._cache[self.user_manifest_id]
        except KeyError:
            # In the unlikely event the user manifest is not present in
            # local (e.g. device just created or during tests), we fall
            # back on an empty manifest which is a good aproximation of
            # the very first version of the manifest (field `created` is
            # invalid, but it will be corrected by the merge during sync).
            return LocalUserManifest.new_placeholder(id=self.device.user_manifest_id)

    async def set_user_manifest(self, user_manifest):
        await self.set_manifest(self.user_manifest_id, user_manifest)

    async def run_vacuum(self) -> None:
        pass

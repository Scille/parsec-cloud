from huepy import que

from parsec.core.fs.base import FSBase
from parsec.core.fs.utils import normalize_path, is_placeholder_access, is_folder_manifest


class FSSyncMixin(FSBase):
    async def sync(self, path: str, recursive=True):
        print(que("start syncing %s" % path))
        parent_path, entry_name = normalize_path(path)
        if path == "/":
            access, manifest = await self._local_tree.retrieve_entry(path)
            await self._sync(access, manifest, recursive=recursive)
        else:
            access, manifest = await self._local_tree.retrieve_entry(path)
            if is_placeholder_access(access):
                # We must sync parents before ourself. The trick is we want to sync
                # the minimal path to the entry we were originally asked to sync
                to_sync_recursive_map = recursive
                curr_ancestor_access = access
                curr_ancestor_path = path
                while is_placeholder_access(curr_ancestor_access):
                    curr_ancestor_path, name = curr_ancestor_path.rsplit("/", 1)
                    to_sync_recursive_map = {name: to_sync_recursive_map}
                    # No risk of missing entry here given we retrieved the whole path earlier
                    curr_ancestor_access, _ = self._local_tree.retrieve_entry_sync(
                        curr_ancestor_path
                    )
                    if not curr_ancestor_access:
                        curr_ancestor_path = "/"
                        break
                await self.sync(curr_ancestor_path, recursive=to_sync_recursive_map)
            else:
                await self._sync(access, manifest, recursive=recursive)
        print(que("done syncing %s" % path))

    async def _sync(self, access, manifest, recursive=False):
        if is_folder_manifest(manifest):
            return await self._sync_folder(access, manifest, recursive=recursive)
        else:
            return await self._sync_file(access, manifest)

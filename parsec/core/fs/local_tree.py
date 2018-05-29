import pendulum

from parsec.core.fs.utils import (
    FSInvalidPath,
    new_user_manifest,
    new_placeholder_access,
    is_placeholder_access,
    is_folder_manifest,
    copy_manifest,
    convert_to_local_manifest,
)


# Custom exception only used internally
class NotInLocalTreeError(Exception):

    def __init__(self, access):
        self.access = access


class LocalTree:

    def __init__(self, device, manifests_manager):
        self.device = device
        self.manifests_manager = manifests_manager
        self._root_manifest_cache = None
        self._manifests_cache = {}
        self._resolved_placeholder_accesses = {}

        # Fetch from local storage the data tree
        manifest = self.manifests_manager.fetch_user_manifest_from_local()
        if manifest:
            self._root_manifest_cache = manifest
        else:
            self._root_manifest_cache = new_user_manifest(self.device)
        self._recursive_load_local_manifests(self._root_manifest_cache)

    def dump(self):

        def recursive_resolve(manifest):
            dump = copy_manifest(manifest)
            if is_folder_manifest(dump):
                for name, access in manifest["children"].items():
                    try:
                        child_manifest = self._manifests_cache[access["id"]]
                    except KeyError:
                        dump["children"][name] = {"access": access}
                    else:
                        child_dump = recursive_resolve(child_manifest)
                        child_dump["access"] = access
                        dump["children"][name] = child_dump
            return dump

        return recursive_resolve(self._root_manifest_cache)

    def _recursive_load_local_manifests(self, folder_manifest):
        for access in folder_manifest["children"].values():
            manifest = self.manifests_manager.fetch_from_local(access["id"], access["key"])
            if not manifest:
                # This entry is not in local storage, just skip it
                continue
            # Sanity check: make sure each manifest is not present multiple
            # times in our tree to avoid loops and cache corruptions
            if access["id"] in self._manifests_cache:
                raise RuntimeError("Access %r is present in multiple manifests" % access)
            self._manifests_cache[access["id"]] = manifest
            if "children" in manifest:
                self._recursive_load_local_manifests(manifest)

    def retrieve_entry_by_access(self, access):
        if not access:
            return copy_manifest(self._root_manifest_cache)
        try:
            return copy_manifest(self._manifests_cache[access["id"]])
        except KeyError:
            if is_placeholder_access(access):
                resolved_access_id = self._resolved_placeholder_accesses.get(access["id"])
                if resolved_access_id:
                    return copy_manifest(self._manifests_cache[resolved_access_id])
            raise

    async def retrieve_entries(self, *paths):
        # First try the fast way
        try:
            return [self.retrieve_entry_sync(path) for path in paths]
        except NotInLocalTreeError:
            # Slow way: first make sure data in in local cache, then
            # do the atomic retrieval
            for path in paths:
                await self.retrieve_entry(path)
            # TODO: in very weird case this can fail (if a path entry is
            # removed from cache during another retrieve_entry)
            return [self.retrieve_entry_sync(path) for path in paths]

    def retrieve_entry_sync(self, path):
        hops = [x for x in path.split("/") if x]
        access = None
        current = self._root_manifest_cache
        for hop in hops:
            try:
                access = current["children"][hop]
            except KeyError:
                # Either current is not a folder or it has no valid child
                raise FSInvalidPath("Path `%s` doesn't exists" % path)
            try:
                current = self._manifests_cache[access["id"]]
            except KeyError:
                if access["type"] == "placeholder":
                    raise RuntimeError("Access %r points to nothing !" % access)
                else:
                    # Local cache miss, nothing we can do here
                    raise NotInLocalTreeError(access)

        return access, copy_manifest(current)

    async def retrieve_entry(self, path):
        while True:
            try:
                return self.retrieve_entry_sync(path)
            except NotInLocalTreeError as exc:
                # The path is not fully available in local, hence we must
                # fetch from the backend the missing part and retry
                access = exc.access
                manifest = await self.manifests_manager.fetch_from_backend(
                    access["id"], access["rts"], access["key"]
                )
                manifest = convert_to_local_manifest(manifest)
                # It maybe possible another coroutine did the resolution while
                # we were in the await
                if access["id"] not in self._manifests_cache:
                    # No concurrent write, it's up to us to update the
                    # local cache with this new entry
                    self._manifests_cache[access["id"]] = manifest
                    self.manifests_manager.flush_on_local(access["id"], access["key"], manifest)

    def overwrite_entry(self, access, manifest):
        if not access:
            self.manifests_manager.flush_user_manifest_on_local(manifest)
            self._root_manifest_cache = manifest
        else:
            self.manifests_manager.flush_on_local(access["id"], access["key"], manifest)
            self._manifests_cache[access["id"]] = manifest

    def update_entry(self, access, manifest):
        manifest["updated"] = pendulum.now()
        manifest["need_sync"] = True
        self.overwrite_entry(access, manifest)

    def resolve_placeholder_access(self, placeholder_access, resolved_access):
        manifest = self._manifests_cache.pop(placeholder_access["id"])
        self.manifests_manager.flush_on_local(
            resolved_access["id"], resolved_access["key"], manifest
        )
        self._manifests_cache[resolved_access["id"]] = manifest
        self._resolved_placeholder_accesses[placeholder_access["id"]] = resolved_access["id"]
        self.manifests_manager.remove_from_local(placeholder_access["id"])

    def move_modifications(self, old_access, new_access, manifest=None):
        try:
            if not manifest:
                manifest = self._manifests_cache.pop(old_access["id"])
            else:
                del self._manifests_cache[old_access["id"]]
        except KeyError:
            # Nothing to move
            return
        self._manifests_cache[new_access["id"]] = manifest
        self.manifests_manager.flush_on_local(new_access["id"], new_access["key"], manifest)
        self.manifests_manager.remove_from_local(old_access["id"])

    def insert_new_entry(self, manifest):
        access = new_placeholder_access()
        manifest["need_sync"] = True

        self._manifests_cache[access["id"]] = manifest
        self.manifests_manager.flush_on_local(access["id"], access["key"], manifest)

        return access

    def delete_entry(self, access):
        try:
            return self._manifests_cache.pop(access["id"])
        except KeyError:
            # Entry simply not available locally
            pass

        self.manifests_manager.remove_from_local(access["id"])

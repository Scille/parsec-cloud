import attr
import pendulum

from parsec.core.fs2.utils import (
    new_user_manifest,
    new_placeholder_access,
    is_placeholder_access,
    is_folder_manifest,
    copy_manifest,
)
from parsec.core.fs2.exceptions import InvalidPath


# Custom exception only used internally
@attr.s
class NotInLocalTreeError(Exception):
    access = attr.ib()


class LocalTree:

    def __init__(self, device, manifests_manager):
        self.device = device
        self.manifests_manager = manifests_manager
        self._root_manifest_cache = None
        self._manifests_cache = {}
        self._resolved_placeholder_accesses = {}

        # Fetch from local storage the data tree
        manifest = self.manifests_manager.fetch_user_manifest_from_local2()
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
            manifest = self.manifests_manager.fetch_from_local2(
                access["id"], access["key"]
            )
            if not manifest:
                # This entry is not in local storage, just skip it
                continue
            # Sanity check: make sure each manifest is not present multiple
            # times in our tree to avoid loops and cache corruptions
            if access["id"] in self._manifests_cache:
                raise RuntimeError(
                    "Access %r is present in multiple manifests" % access
                )
            self._manifests_cache[access["id"]] = manifest
            if "children" in manifest:
                self._recursive_load_local_manifests(manifest)

    def retrieve_entry_by_access(self, access):
        if not access:
            return self._root_manifest_cache
        try:
            return self._manifests_cache[access["id"]]
        except KeyError:
            if (
                is_placeholder_access(access)
                and access["id"] in self._resolved_placeholder_accesses
            ):
                resolved_access = self._resolved_placeholder_accesses.get(access["id"])
                if resolved_access:
                    return self.retrieve_entry_by_access(resolved_access)
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
                raise InvalidPath("Path `%s` doesn't exists" % path)
            try:
                current = self._manifests_cache[access["id"]]
            except KeyError:
                if access["type"] == "placeholder":
                    raise RuntimeError("Access %r points to nothing !" % access)
                else:
                    # Local cache miss, nothing we can do here
                    raise NotInLocalTreeError(access)

        return access, current

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
                    self.manifests_manager.flush_on_local2(
                        access["id"], access["key"], manifest
                    )

    def overwrite_entry(self, access, manifest):
        if not access:
            self._root_manifest_cache = manifest
            self.manifests_manager.flush_user_manifest_on_local2(manifest)
        else:
            self._manifests_cache[access["id"]] = manifest
            self.manifests_manager.flush_on_local2(
                access["id"], access["key"], manifest
            )

    def update_entry(self, access, manifest):
        manifest["updated"] = pendulum.now()
        manifest["need_sync"] = True
        self.overwrite_entry(access, manifest)

    def resolve_placeholder_access(self, placeholder_access, resolved_access):
        manifest = self._manifests_cache.pop(placeholder_access["id"])
        self._manifests_cache[resolved_access["id"]] = manifest
        self._resolved_placeholder_accesses[placeholder_access["id"]] = resolved_access[
            "id"
        ]

    def move_modified_entry_to_placeholder(
        self, parent_access, parent_manifest, access, diverged_manifest, diverged_name
    ):

        # First create a new placeholder containing the diverged data
        new_access = self.insert_new_entry(diverged_manifest)

        # Now udpate the parent manifest to link to the new placeholder
        parent_manifest["children"][diverged_name] = new_access
        self.update_entry(parent_access, parent_manifest)

        # Finally we can rollback the original diverged entry by removing
        # it local data. Note this must be done last to avoid losing data
        # if something goes wrong.
        self.manifests_manager.remove_from_local2(access["id"])

        return new_access

    def insert_new_entry(self, manifest):
        access = new_placeholder_access()
        manifest["need_sync"] = True

        self._manifests_cache[access["id"]] = manifest
        self.manifests_manager.flush_on_local2(access["id"], access["key"], manifest)

        return access

    def delete_entry(self, access):
        try:
            del self._manifests_cache[access["id"]]
        except KeyError:
            # Entry simply not available locally
            pass

        # TODO
        # self.manifests_manager.remove_from_local2(access['id'])

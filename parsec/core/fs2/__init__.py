import attr
import pendulum
from uuid import uuid4
from copy import deepcopy

from parsec.utils import generate_sym_key
from parsec.core.fs2.opened_file import OpenedFile, fast_forward_file_manifest
from parsec.core.fs2.merge_folders import (
    merge_remote_folder_manifests,
    merge_local_folder_manifests,
)
from parsec.core.backend_storage import BackendConcurrencyError


class InvalidPath(Exception):
    pass


@attr.s
class NotLoaded(Exception):
    access = attr.ib()


def _normalize_path(path):
    hops = path.split("/")
    for hop in hops:
        # TODO: enable this later
        # if hop == '.' or hop == '..':
        #     raise InvalidPath('Path %r is invalid' % path)
        pass
    return "/".join(hops[:-1]), hop


def new_folder_manifest(author):
    now = pendulum.now()
    return {
        "type": "local_folder_manifest",
        "user_id": author.user_id,
        "device_name": author.device_name,
        "base_version": 0,
        "need_sync": True,
        "created": now,
        "updated": now,
        "children": {},
    }


def new_placeholder_access():
    return {"type": "placeholder", "id": uuid4().hex, "key": generate_sym_key()}


def new_access():
    return {
        "type": "vlob",
        "id": uuid4().hex,
        "key": generate_sym_key(),
        "rts": uuid4().hex,
        "wts": uuid4().hex,
    }


def new_user_manifest(author):
    now = pendulum.now()
    return {
        "type": "local_user_manifest",
        "user_id": author.user_id,
        "device_name": author.device_name,
        "base_version": 0,
        "need_sync": True,
        "created": now,
        "updated": now,
        "children": {},
        "last_processed_message": 0,
    }


def new_file_manifest(author):
    now = pendulum.now()
    return {
        "type": "local_file_manifest",
        "user_id": author.user_id,
        "device_name": author.device_name,
        "base_version": 0,
        "need_sync": True,
        "created": now,
        "updated": now,
        "size": 0,
        "blocks": [],
        "dirty_blocks": [],
    }


def mark_updated(manifest):
    manifest["updated"] = pendulum.now()
    manifest["need_sync"] = True


@attr.s
class FS2:
    device = attr.ib()
    manifests_manager = attr.ib()
    blocks_manager = attr.ib()
    _root_manifest_cache = attr.ib(default=None)
    _manifests_cache = attr.ib(default=attr.Factory(dict))
    _opened_files = attr.ib(default=attr.Factory(dict))

    def _dump_local_fs(self):

        def recursive_resolve(manifest):
            dump = deepcopy(manifest)
            if "children" in dump:
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

    def _debug_pocav(self, name="DUMP FS"):
        from pprint import pprint

        print("~~~ %s ~~~" % name)
        pprint(self._dump_local_fs())
        pprint(self._opened_files)
        pprint(self._manifests_cache)
        print("~~~ END %s ~~~" % name)

    async def init(self, nursery):
        # Fetch from local storage the data tree
        manifest = self.manifests_manager.fetch_user_manifest_from_local2()
        if manifest:
            self._root_manifest_cache = manifest
        else:
            self._root_manifest_cache = new_user_manifest(self.device)
        self._recursive_load_local_manifests(self._root_manifest_cache)

    def _recursive_load_local_manifests(self, folder_manifest):
        for access in folder_manifest["children"].values():
            manifest = self.manifests_manager.fetch_from_local2(access["id"], access["key"])
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

    async def teardown(self):
        # TODO
        # for file in self._opened_files.values():
        #     self._flush_opened_file(file)
        return

    async def _retrieve_entries(self, *paths):
        # First try the fast way
        try:
            return [self._retrieve_entry_sync(path) for path in paths]
        except NotLoaded:
            # Slow way: first make sure data in in local cache, then
            # do the atomic retrieval
            for path in paths:
                await self._retrieve_entry(path)
            return [self._retrieve_entry_sync(path) for path in paths]

    def _retrieve_entry_sync(self, path):
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
                    raise NotLoaded(access)

        return access, current

    async def _retrieve_entry(self, path):
        while True:
            try:
                return self._retrieve_entry_sync(path)
            except NotLoaded as exc:
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
                    self.manifests_manager.flush_on_local2(access["id"], access["key"], manifest)

    def _flush_entry(self, access, manifest):
        if not access:
            self.manifests_manager.flush_user_manifest_on_local2(manifest)
        else:
            self.manifests_manager.flush_on_local2(access["id"], access["key"], manifest)

    async def _insert_new(self, path, manifest):
        if path == "/":
            raise InvalidPath("Path `/` already exists")

        parent_path, file_name = _normalize_path(path)
        parent_access, parent_manifest = await self._retrieve_entry(parent_path)
        if "children" not in parent_manifest:
            raise InvalidPath("Path `%s` is not a folder" % parent_path)
        if file_name in parent_manifest["children"]:
            raise InvalidPath("Path `%s` already exists" % path)

        access = new_placeholder_access()
        parent_manifest["children"][file_name] = access
        self._manifests_cache[access["id"]] = manifest
        self.manifests_manager.flush_on_local2(access["id"], access["key"], manifest)
        mark_updated(parent_manifest)
        self._flush_entry(parent_access, parent_manifest)

    async def file_create(self, path: str):
        manifest = new_file_manifest(self.device)
        await self._insert_new(path, manifest)

    async def folder_create(self, path: str):
        manifest = new_folder_manifest(self.device)
        await self._insert_new(path, manifest)

    def _open_file(self, access, manifest):
        try:
            return self._opened_files[access["id"]]
        except KeyError:
            fd = OpenedFile(access, manifest)
            self._opened_files[access["id"]] = fd
            return fd

    async def file_read(self, path: str, size: int = -1, offset: int = 0):
        if path == "/":
            raise InvalidPath("Path `/` is not a file")
        _normalize_path(path)
        access, manifest = await self._retrieve_entry(path)
        if "children" in manifest:
            raise InvalidPath("Path `%s` is not a file" % path)

        return b""
        # TODO: finish this...

        # fd = self._open_file(access, manifest)

        # # Start of atomic operations

        # size, in_ram, in_local, in_remote = entry.get_read_map(size, offset)

        # buffer = bytearray(size)

        # for start, end, content in in_ram:
        #     buffer[start, end] = content

        # for start, end, id in in_local:
        #     content = self.store.get_block(id)
        #     buffer[start, end] = content

        # # End of atomic operations

        # for start, end, content in in_remote:
        #     await self.blocks_manager.fetch_from_backend()
        #     content = await self.store.get_remote_block(id)
        #     buffer[start, end] = content

        # return content

    async def file_write(self, path, buffer: bytes, offset: int = -1):
        if path == "/":
            raise InvalidPath("Path `/` is not a file")
        _normalize_path(path)
        access, manifest = await self._retrieve_entry(path)
        if "children" in manifest:
            raise InvalidPath("Path `%s` is not a file" % path)

        fd = self._open_file(access, manifest)
        fd.write(buffer, offset)
        # Provide some gc mechanism to auto flush if needed here ?
        mark_updated(manifest)

    async def file_truncate(self, path: str, length: int):
        if path == "/":
            raise InvalidPath("Path `/` is not a file")
        _normalize_path(path)
        access, manifest = await self._retrieve_entry(path)
        if "children" in manifest:
            raise InvalidPath("Path `%s` is not a file" % path)

        fd = self._open_file(access, manifest)
        fd.truncate(length)
        mark_updated(manifest)

    async def file_flush(self, path: str):
        if path == "/":
            # TODO: done for compatibility
            # raise InvalidPath("Path `/` is not a file")
            return
        _normalize_path(path)
        access, manifest = await self._retrieve_entry(path)
        if "children" in manifest:
            # TODO: done for compatibility
            # raise InvalidPath("Path `%s` doesn't point to a file")
            return

        if access["id"] not in self._opened_files:
            return
        # TODO: finish this...

        # fd = self._opened_files[access['id']]
        # flush_map = fd.get_flush_map()

        # self.manifests_manager.flush_on_local2(access['id'], access['key'], manifest)

    async def move(self, src: str, dst: str):
        if src == "/":
            raise InvalidPath("Cannot move `/` root folder")
        if dst == "/":
            raise InvalidPath("Path `/` already exists")

        src_parent_path, src_file_name = _normalize_path(src)
        dst_parent_path, dst_file_name = _normalize_path(dst)
        (src_parent_access, src_parent_manifest), (
            dst_parent_access,
            dst_parent_manifest,
        ) = await self._retrieve_entries(src_parent_path, dst_parent_path)

        if "children" not in src_parent_manifest:
            raise InvalidPath("Path `%s` is not a folder" % src_parent_path)
        if "children" not in dst_parent_manifest:
            raise InvalidPath("Path `%s` is not a folder" % dst_parent_path)

        if src_file_name not in src_parent_manifest["children"]:
            raise InvalidPath("Path `%s` doesn't exists" % src)

        if dst_file_name in dst_parent_manifest["children"]:
            raise InvalidPath("Path `%s` already exists" % dst)

        # TODO: useful ?
        if src == dst:
            return {"status": "invalid_path", "reason": "Cannot move `%s` to itself" % src}

        if dst.startswith(src + "/"):
            raise InvalidPath("Cannot move `%s` to a subdirectory of itself" % src)

        try:
            target_access = src_parent_manifest["children"].pop(src_file_name)
        except KeyError:
            raise InvalidPath("Path `%s` doesn't exists" % src)
        dst_parent_manifest["children"][dst_file_name] = target_access
        mark_updated(dst_parent_manifest)
        mark_updated(src_parent_manifest)

        self._flush_entry(src_parent_access, src_parent_manifest)
        self._flush_entry(dst_parent_access, dst_parent_manifest)

    # async def copy(self):
    #     pass

    async def delete(self, path: str):
        if path == "/":
            raise InvalidPath("Cannot delete `/` root folder")
        parent_path, file_name = _normalize_path(path)
        parent_access, parent_manifest = await self._retrieve_entry(parent_path)

        try:
            file_access = parent_manifest["children"].pop(file_name)
        except KeyError:
            raise InvalidPath("Path `%s` doesn't exists" % path)
        mark_updated(parent_manifest)

        try:
            del self._manifests_cache[file_access["id"]]
        except KeyError:
            # Entry simply not available locally
            pass

        # TODO
        # self.manifests_manager.delete_on_local2(access['id'])
        self._flush_entry(parent_access, parent_manifest)

    async def stat(self, path: str):
        if path == "/":
            manifest = self._root_manifest_cache
            is_placeholder = False
        else:
            _normalize_path(path)
            access, manifest = await self._retrieve_entry(path)
            is_placeholder = access["type"] == "placeholder"
            need_flush = access["id"] in self._opened_files

        if "children" in manifest:
            return {
                "type": "folder",
                "created": manifest["created"],
                "updated": manifest["updated"],
                "base_version": manifest["base_version"],
                "is_placeholder": is_placeholder,
                "need_sync": manifest["need_sync"],
                "need_flush": need_flush,
                "children": list(sorted(manifest["children"].keys())),
            }
        else:
            # Ignore the modification that are currently in the opened file
            # system given they are not flush on the disk yet.
            return {
                "type": "file",
                "created": manifest["created"],
                "updated": manifest["updated"],
                "base_version": manifest["base_version"],
                "is_placeholder": is_placeholder,
                "need_sync": manifest["need_sync"],
                "need_flush": need_flush,
                "size": manifest["size"],
            }

    async def sync(self, path: str):
        _normalize_path(path)
        access, manifest = await self._retrieve_entry(path)
        await self._sync(access, manifest)

    async def _sync(self, access, manifest):
        if "children" in manifest:
            return await self._sync_folder(access, manifest)
        else:
            return await self._sync_file(access, manifest)

    async def _sync_folder(self, access, manifest):
        # Make a snapshot of ourself to avoid concurrency
        # Serialize as local folder manifest
        base_local_manifest = deepcopy(manifest)
        snapshot_manifest = deepcopy(manifest)

        # Recursive sync of children first
        synced_children = {}
        for child_name, child_access in snapshot_manifest["children"].items():
            try:
                child_manifest = self._manifests_cache[child_access["id"]]
            except KeyError:
                # This entry is not present in local (or has been deleted since
                # we started the sync), no need to go further
                synced_children[child_name] = child_access
                continue
            try:
                child_access = await self._sync(child_access, child_manifest)
            except BackendConcurrencyError:
                # Placeholder are not yet in backend, cannot have concurrence on them !
                assert child_access["type"] != "placeholder"
                # File already modified, must rename it to avoid losing data!
                if child_access:
                    child_target_manifest = await self.manifests_manager.fetch_from_backend(
                        child_access["id"], child_access["rts"], child_access["key"]
                    )
                else:
                    child_target_manifest = (
                        await self.manifests_manager.fetch_user_manifest_from_backend()
                    )
                child_target_manifest = convert_to_local_manifest(child_target_manifest)
                child_diverged_manifest = self._manifests_cache.pop(child_access["id"])
                self._manifests_cache[child_access["id"]] = child_target_manifest

                # Move the diverged entry to avoid losing it
                diverged_access = new_placeholder_access()
                duplicated_name = child_name
                while True:
                    duplicated_name += ".conflict"
                    if (
                        duplicated_name not in snapshot_manifest["children"]
                        and duplicated_name not in synced_children
                    ):
                        synced_children[duplicated_name] = diverged_access
                        break
                self._manifests_cache[diverged_access["id"]] = child_diverged_manifest
                mark_updated(snapshot_manifest)

                # TODO: protect aganst BackendconcurrencyError ?
                await self._sync(diverged_access, child_diverged_manifest)

            # Required if access is a placeholder
            # TODO: find a better way to do it...
            synced_children[child_name] = child_access

        snapshot_manifest["children"] = synced_children

        # Sync ourself or check if a new version is available in the backend
        if not snapshot_manifest["need_sync"]:
            # This folder hasn't been modified locally,
            # just download last version from the backend if any.
            if not access:
                target_remote_manifest = (
                    await self.manifests_manager.fetch_user_manifest_from_backend()
                )
            else:
                # Placeholder means we need synchro !
                assert access["type"] != "placeholder"
                target_remote_manifest = await self.manifests_manager.fetch_from_backend(
                    access["id"], access["rts"], access["key"]
                )
            if (
                not target_remote_manifest
                or target_remote_manifest["version"] == manifest["base_version"]
            ):
                return access
        else:

            to_sync_manifest = convert_to_remote_manifest(snapshot_manifest)
            # Upload the file manifest as new vlob version
            while True:
                try:
                    if not access:
                        await self.manifests_manager.sync_user_manifest_with_backend(
                            to_sync_manifest
                        )
                    elif access["type"] == "placeholder":
                        id, rts, wts = await self.manifests_manager.sync_new_entry_with_backend(
                            access["key"], to_sync_manifest
                        )
                        access.update(id=id, rts=rts, wts=wts, type="vlob")
                    else:
                        await self.manifests_manager.sync_with_backend(
                            access["id"], access["wts"], access["key"], to_sync_manifest
                        )
                    target_remote_manifest = to_sync_manifest
                    break

                except BackendConcurrencyError:
                    # Do a 3-ways merge to fix the concurrency error, first we must
                    # fetch the base version and the new one present in the backend
                    # TODO: should be available locally
                    if not access:
                        base = await self.manifests_manager.fetch_user_manifest_from_backend(
                            version=to_sync_manifest["version"] - 1
                        )
                        target = await self.manifests_manager.fetch_user_manifest_from_backend()
                    else:
                        base = await self.manifests_manager.fetch_from_backend(
                            access["id"],
                            access["rts"],
                            access["key"],
                            version=to_sync_manifest["version"] - 1,
                        )
                        target = await self.manifests_manager.fetch_from_backend(
                            access["id"], access["rts"], access["key"]
                        )
                    # 3-ways merge between base, modified and target versions
                    to_sync_manifest, _ = merge_remote_folder_manifests(
                        base, to_sync_manifest, target
                    )
                    to_sync_manifest["version"] = target["version"] + 1

        # Finally merge with the version present in cache which may have
        # been modified in the meantime
        target_local_manifest = convert_to_local_manifest(target_remote_manifest)
        manifest, _ = merge_local_folder_manifests(
            base_local_manifest, manifest, target_local_manifest
        )
        if not access:
            self._root_manifest_cache = manifest
        else:
            # Note the manifest is still kept in the cache with it
            # previous placeholder id
            self._manifests_cache[access["id"]] = manifest
        self._flush_entry(access, manifest)

        return access

    async def _sync_file(self, access, manifest):
        if not manifest["need_sync"]:
            # Placeholder means we need synchro !
            assert access["type"] != "placeholder"
            # This file hasn't been modified locally,
            # just download last version from the backend if any.
            target_manifest = await self.manifests_manager.fetch_from_backend(
                access["id"], access["rts"], access["key"]
            )
            if target_manifest["version"] != manifest["base_version"]:
                if manifest["need_sync"]:
                    raise BackendConcurrencyError(
                        "File with access %r has locally diverged" % access
                    )
                else:
                    target_manifest = convert_to_local_manifest(target_manifest)
                    self._manifests_cache[access["id"]] = target_manifest
                    self._flush_entry(access, target_manifest)

        else:
            # The file has been locally modified, time to sync it with the backend !
            # TODO
            # fd = self._open_file(access, manifest)
            # ...
            snapshot_manifest = convert_to_remote_manifest(manifest)
            if access["type"] == "placeholder":
                id, rts, wts = await self.manifests_manager.sync_new_entry_with_backend(
                    access["key"], snapshot_manifest
                )
                access.update(id=id, rts=rts, wts=wts, type="vlob")
            else:
                await self.manifests_manager.sync_with_backend(
                    access["id"], access["wts"], access["key"], snapshot_manifest
                )
            # Finally fast forward the manifest with it new base version
            # TODO
            manifest = fast_forward_file_manifest(snapshot_manifest, manifest)
            # Note the manifest is still kept in the cache with it
            # previous placeholder id
            self._manifests_cache[access["id"]] = manifest
            self._flush_entry(access, manifest)

        return access


def convert_to_remote_manifest(local_manifest):
    manifest = deepcopy(local_manifest)
    manifest["version"] = manifest.pop("base_version") + 1
    local_type = manifest["type"]
    if local_type == "local_file_manifest":
        manifest["type"] = "file_manifest"
    elif local_type == "local_folder_manifest":
        manifest["type"] = "folder_manifest"
    elif local_type == "local_user_manifest":
        manifest["type"] = "user_manifest"
    else:
        raise RuntimeError("Unknown type in local manifest %r" % local_manifest)
    if "children" in local_manifest:
        for access in manifest["children"].values():
            del access["type"]
    return manifest


def convert_to_local_manifest(manifest):
    local_manifest = deepcopy(manifest)
    local_manifest["need_sync"] = False
    local_manifest["base_version"] = local_manifest.pop("version")
    remote_type = manifest["type"]
    if remote_type == "file_manifest":
        local_manifest["type"] = "local_file_manifest"
    elif remote_type == "folder_manifest":
        local_manifest["type"] = "local_folder_manifest"
    elif remote_type == "user_manifest":
        local_manifest["type"] = "local_user_manifest"
    else:
        raise RuntimeError("Unknown type in remote manifest %r" % manifest)
    if "children" in local_manifest:
        for access in local_manifest["children"].values():
            access["type"] = "vlob"
    return local_manifest

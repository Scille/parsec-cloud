from parsec.core.fs.base import FSBase
from parsec.core.fs.utils import (
    FSInvalidPath,
    normalize_path,
    is_placeholder_access,
    is_file_manifest,
    is_folder_manifest,
    new_file_manifest,
    new_folder_manifest,
    new_dirty_block_access,
)


class FSOpsMixin(FSBase):
    async def _insert_new(self, path, manifest):
        if path == "/":
            raise FSInvalidPath("Path `/` already exists")

        parent_path, file_name = normalize_path(path)
        parent_access, parent_manifest = await self._local_tree.retrieve_entry(parent_path)
        if not is_folder_manifest(parent_manifest):
            raise FSInvalidPath("Path `%s` is not a folder" % parent_path)
        if file_name in parent_manifest["children"]:
            raise FSInvalidPath("Path `%s` already exists" % path)

        access = self._local_tree.insert_new_entry(manifest)
        parent_manifest["children"][file_name] = access
        self._local_tree.update_entry(parent_access, parent_manifest)

    async def file_create(self, path: str):
        manifest = new_file_manifest(self._device)
        await self._insert_new(path, manifest)

    async def folder_create(self, path: str):
        manifest = new_folder_manifest(self._device)
        await self._insert_new(path, manifest)

    async def _build_data_from_contiguous_space(self, cs):
        data = bytearray(cs.size)
        for bs in cs.get_in_ram_buffers():
            data[bs.start - cs.start : bs.end - cs.start] = bs.get_data()

        for bs in cs.get_dirty_blocks():
            buff = self._blocks_manager.fetch_from_local(
                bs.buffer.data["id"], bs.buffer.data["key"]
            )
            assert buff
            data[bs.start - cs.start : bs.end - cs.start] = buff[
                bs.buffer_slice_start : bs.buffer_slice_end
            ]

        for bs in cs.get_blocks():
            buff = await self._blocks_manager.fetch_from_backend(
                bs.buffer.data["id"], bs.buffer.data["key"]
            )
            assert buff
            data[bs.start - cs.start : bs.end - cs.start] = buff[
                bs.buffer_slice_start : bs.buffer_slice_end
            ]

        return data

    async def file_read(self, path: str, size: int = -1, offset: int = 0):
        if path == "/":
            raise FSInvalidPath("Path `/` is not a file")
        normalize_path(path)
        access, manifest = await self._local_tree.retrieve_entry(path)
        if not is_file_manifest(manifest):
            raise FSInvalidPath("Path `%s` is not a file" % path)

        if size == 0:
            return b""

        fd = self._opened_files.open_file(access, manifest)
        cs = fd.get_read_map(manifest, size, offset)
        return await self._build_data_from_contiguous_space(cs)

    async def file_write(self, path, buffer: bytes, offset: int = -1):
        if path == "/":
            raise FSInvalidPath("Path `/` is not a file")
        normalize_path(path)
        access, manifest = await self._local_tree.retrieve_entry(path)
        if not is_file_manifest(manifest):
            raise FSInvalidPath("Path `%s` is not a file" % path)

        if not buffer:
            return

        fd = self._opened_files.open_file(access, manifest)
        fd.write(buffer, offset)

    async def file_truncate(self, path: str, length: int):
        if path == "/":
            raise FSInvalidPath("Path `/` is not a file")
        normalize_path(path)
        access, manifest = await self._local_tree.retrieve_entry(path)
        if not is_file_manifest(manifest):
            raise FSInvalidPath("Path `%s` is not a file" % path)

        fd = self._opened_files.open_file(access, manifest)
        fd.truncate(length)

    async def file_flush(self, path: str):
        if path == "/":
            # TODO: done for compatibility
            # raise FSInvalidPath("Path `/` is not a file")
            return
        normalize_path(path)

        while True:
            access, manifest = await self._local_tree.retrieve_entry(path)
            if not is_file_manifest(manifest):
                # TODO: done for compatibility
                # raise FSInvalidPath("Path `%s` doesn't point to a file")
                return

            fd = self._opened_files.open_file(access, manifest)
            if not fd.need_flush(manifest):
                return

            if fd.is_syncing():
                # This file is in the middle of a sync, wait for it to
                # end and retry everything to avoid concurrency issues
                await fd.wait_not_syncing()
                continue

            new_size, new_dirty_blocks = fd.get_flush_map()
            for ndb in new_dirty_blocks:
                ndba = new_dirty_block_access(ndb.start, ndb.size)
                self._blocks_manager.flush_on_local(ndba["id"], ndba["key"], ndb.data)
                manifest["dirty_blocks"].append(ndba)
            manifest["size"] = new_size
            self._local_tree.update_entry(access, manifest)

            self._opened_files.close_file(access)
            break

    async def move(self, src: str, dst: str):
        if src == "/":
            raise FSInvalidPath("Cannot move `/` root folder")
        if dst == "/":
            raise FSInvalidPath("Path `/` already exists")

        src_parent_path, src_file_name = normalize_path(src)
        dst_parent_path, dst_file_name = normalize_path(dst)
        if src_parent_path == dst_parent_path:
            parent_access, parent_manifest = await self._local_tree.retrieve_entry(src_parent_path)

            if not is_folder_manifest(parent_manifest):
                raise FSInvalidPath("Path `%s` is not a folder" % src_parent_path)

            if dst_file_name in parent_manifest["children"]:
                raise FSInvalidPath("Path `%s` already exists" % dst)

            try:
                target_access = parent_manifest["children"].pop(src_file_name)
            except KeyError:
                raise FSInvalidPath("Path `%s` doesn't exists" % src)
            parent_manifest["children"][dst_file_name] = target_access

            self._local_tree.update_entry(parent_access, parent_manifest)

        else:
            (src_parent_access, src_parent_manifest), (
                dst_parent_access,
                dst_parent_manifest,
            ) = await self._local_tree.retrieve_entries(src_parent_path, dst_parent_path)

            if not is_folder_manifest(src_parent_manifest):
                raise FSInvalidPath("Path `%s` is not a folder" % src_parent_path)
            if not is_folder_manifest(dst_parent_manifest):
                raise FSInvalidPath("Path `%s` is not a folder" % dst_parent_path)

            if dst_file_name in dst_parent_manifest["children"]:
                raise FSInvalidPath("Path `%s` already exists" % dst)

            if dst.startswith(src + "/"):
                raise FSInvalidPath("Cannot move `%s` to a subdirectory of itself" % src)

            try:
                target_access = src_parent_manifest["children"].pop(src_file_name)
            except KeyError:
                raise FSInvalidPath("Path `%s` doesn't exists" % src)
            dst_parent_manifest["children"][dst_file_name] = target_access

            self._local_tree.update_entry(src_parent_access, src_parent_manifest)
            self._local_tree.update_entry(dst_parent_access, dst_parent_manifest)

    # async def copy(self):
    #     pass

    async def delete(self, path: str):
        if path == "/":
            raise FSInvalidPath("Cannot delete `/` root folder")
        parent_path, entry_name = normalize_path(path)
        parent_access, parent_manifest = await self._local_tree.retrieve_entry(parent_path)

        try:
            entry_access = parent_manifest["children"].pop(entry_name)
        except KeyError:
            raise FSInvalidPath("Path `%s` doesn't exists" % path)

        self._recursive_clean_local_modifications(entry_access)
        self._local_tree.update_entry(parent_access, parent_manifest)

    def _recursive_clean_local_modifications(self, entry_access):
        manifest = self._local_tree.delete_entry(entry_access)
        if manifest:
            if is_folder_manifest(manifest):
                for child_access in manifest["children"].values():
                    self._recursive_clean_local_modifications(child_access)
            else:
                try:
                    self._opened_files.close_file(entry_access)
                except KeyError:
                    pass

    async def stat(self, path: str):
        if path == "/":
            _, manifest = await self._local_tree.retrieve_entry(path)
            is_placeholder = False
        else:
            normalize_path(path)
            access, manifest = await self._local_tree.retrieve_entry(path)
            is_placeholder = is_placeholder_access(access)

        if is_file_manifest(manifest):
            fd = self._opened_files.open_file(access, manifest)
            return {
                "type": "file",
                "created": manifest["created"],
                "updated": manifest["updated"],
                "base_version": manifest["base_version"],
                "is_placeholder": is_placeholder,
                "need_sync": fd.need_sync(manifest),
                "need_flush": fd.need_flush(manifest),
                "size": fd.size,
            }

        else:
            return {
                "type": "folder",
                "created": manifest["created"],
                "updated": manifest["updated"],
                "base_version": manifest["base_version"],
                "is_placeholder": is_placeholder,
                "need_sync": manifest["need_sync"],
                "need_flush": False,
                "children": list(sorted(manifest["children"].keys())),
            }

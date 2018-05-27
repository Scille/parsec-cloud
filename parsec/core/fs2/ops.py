from parsec.core.fs2.base import FSBase
from parsec.core.fs2.utils import (
    normalize_path,
    is_placeholder_access,
    is_file_manifest,
    is_folder_manifest,
    new_file_manifest,
    new_folder_manifest,
    new_dirty_block_access,
)
from parsec.core.fs2.opened_file import OpenedFile, fast_forward_file_manifest, OpenedFilesManager
from parsec.core.fs2.exceptions import InvalidPath


class FSOpsMixin(FSBase):

    async def _insert_new(self, path, manifest):
        if path == "/":
            raise InvalidPath("Path `/` already exists")

        parent_path, file_name = normalize_path(path)
        parent_access, parent_manifest = await self._local_tree.retrieve_entry(parent_path)
        if not is_folder_manifest(parent_manifest):
            raise InvalidPath("Path `%s` is not a folder" % parent_path)
        if file_name in parent_manifest["children"]:
            raise InvalidPath("Path `%s` already exists" % path)

        access = self._local_tree.insert_new_entry(manifest)
        parent_manifest["children"][file_name] = access
        self._local_tree.update_entry(parent_access, parent_manifest)

    async def file_create(self, path: str):
        manifest = new_file_manifest(self._device)
        await self._insert_new(path, manifest)

    async def folder_create(self, path: str):
        manifest = new_folder_manifest(self._device)
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
        normalize_path(path)
        access, manifest = await self._local_tree.retrieve_entry(path)
        if not is_file_manifest(manifest):
            raise InvalidPath("Path `%s` is not a file" % path)

        if size == 0:
            return b''

        fd = self._opened_files.open_file(access, manifest)
        size, opened_file_rm, dirty_blocks_rm, blocks_rm = fd.get_read_map(size, offset)

        data = bytearray(size)
        for bs in opened_file_rm:
            data[bs.start: bs.end] = bs.buffer.data[bs.buffer_slice_start: bs.buffer_slice_end]

        for bs in dirty_blocks_rm:
            buff = self._blocks_manager.fetch_from_local(bs.data)
            data[bs.start: bs.end] = buff[bs.buffer_slice_start: bs.buffer_slice_end]

        # for bs in blocks_rm:
        #     data[bs.start: bs.end] = bs.data[bs.buffer_slice_start: bs.buffer_slice_end]

        return data

    async def file_write(self, path, buffer: bytes, offset: int = -1):
        if path == "/":
            raise InvalidPath("Path `/` is not a file")
        normalize_path(path)
        access, manifest = await self._local_tree.retrieve_entry(path)
        if not is_file_manifest(manifest):
            raise InvalidPath("Path `%s` is not a file" % path)

        if not buffer:
            return

        fd = self._opened_files.open_file(access, manifest)
        fd.write(buffer, offset)

    async def file_truncate(self, path: str, length: int):
        if path == "/":
            raise InvalidPath("Path `/` is not a file")
        normalize_path(path)
        access, manifest = await self._local_tree.retrieve_entry(path)
        if not is_file_manifest(manifest):
            raise InvalidPath("Path `%s` is not a file" % path)

        fd = self._opened_files.open_file(access, manifest)
        fd.truncate(length)

    async def file_flush(self, path: str):
        if path == "/":
            # TODO: done for compatibility
            # raise InvalidPath("Path `/` is not a file")
            return
        normalize_path(path)
        access, manifest = await self._local_tree.retrieve_entry(path)
        if not is_file_manifest(manifest):
            # TODO: done for compatibility
            # raise InvalidPath("Path `%s` doesn't point to a file")
            return

        fd = self._opened_files.open_file(access, manifest)

        new_size, new_dirty_blocks = fd.get_flush_map()
        for ndb in new_dirty_blocks:
            ndba = new_dirty_block_access(ndb.start, ndb.size)
            self._blocks_manager.flush_on_local(ndba['id'], ndba['key'], ndb.data)
            manifest['dirty_blocks'].append(ndba)
        manifest['size'] = new_size
        self._local_tree.update_entry(access, manifest)

        self._opened_files.close_file(access)

    async def move(self, src: str, dst: str):
        if src == "/":
            raise InvalidPath("Cannot move `/` root folder")
        if dst == "/":
            raise InvalidPath("Path `/` already exists")

        src_parent_path, src_file_name = normalize_path(src)
        dst_parent_path, dst_file_name = normalize_path(dst)
        if src_parent_path == dst_parent_path:
            parent_access, parent_manifest = await self._local_tree.retrieve_entry(src_parent_path)

            if not is_folder_manifest(parent_manifest):
                raise InvalidPath("Path `%s` is not a folder" % src_parent_path)

            if dst_file_name in parent_manifest["children"]:
                raise InvalidPath("Path `%s` already exists" % dst)

            try:
                target_access = parent_manifest["children"].pop(src_file_name)
            except KeyError:
                raise InvalidPath("Path `%s` doesn't exists" % src)
            parent_manifest["children"][dst_file_name] = target_access

            self._local_tree.update_entry(parent_access, parent_manifest)

        else:
            (src_parent_access, src_parent_manifest), (
                dst_parent_access,
                dst_parent_manifest,
            ) = await self._local_tree.retrieve_entries(src_parent_path, dst_parent_path)

            if not is_folder_manifest(src_parent_manifest):
                raise InvalidPath("Path `%s` is not a folder" % src_parent_path)
            if not is_folder_manifest(dst_parent_manifest):
                raise InvalidPath("Path `%s` is not a folder" % dst_parent_path)

            if dst_file_name in dst_parent_manifest["children"]:
                raise InvalidPath("Path `%s` already exists" % dst)

            if dst.startswith(src + "/"):
                raise InvalidPath("Cannot move `%s` to a subdirectory of itself" % src)

            try:
                target_access = src_parent_manifest["children"].pop(src_file_name)
            except KeyError:
                raise InvalidPath("Path `%s` doesn't exists" % src)
            dst_parent_manifest["children"][dst_file_name] = target_access

            self._local_tree.update_entry(src_parent_access, src_parent_manifest)
            self._local_tree.update_entry(dst_parent_access, dst_parent_manifest)

    # async def copy(self):
    #     pass

    async def delete(self, path: str):
        if path == "/":
            raise InvalidPath("Cannot delete `/` root folder")
        parent_path, file_name = normalize_path(path)
        parent_access, parent_manifest = await self._local_tree.retrieve_entry(parent_path)

        try:
            file_access = parent_manifest["children"].pop(file_name)
        except KeyError:
            raise InvalidPath("Path `%s` doesn't exists" % path)

        self._local_tree.delete_entry(file_access)
        self._local_tree.update_entry(parent_access, parent_manifest)

    async def stat(self, path: str):
        if path == "/":
            _, manifest = await self._local_tree.retrieve_entry(path)
            is_placeholder = False
        else:
            normalize_path(path)
            access, manifest = await self._local_tree.retrieve_entry(path)
            is_placeholder = is_placeholder_access(access)
            if is_file_manifest(manifest):
                need_flush = self._opened_files.is_opened(access)
            else:
                need_flush = False

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

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from uuid import UUID
from typing import List, Tuple

from parsec.event_bus import EventBus
from parsec.core.types import (
    FsPath,
    Access,
    WorkspaceEntry,
    LocalDevice,
    LocalManifest,
    local_manifest_serializer,
    ManifestAccess,
    LocalFileManifest,
    LocalFolderManifest,
    LocalWorkspaceManifest,
    LocalUserManifest,
)
from parsec.core.local_db import LocalDB, LocalDBMissingEntry
from parsec.core.fs.utils import (
    is_file_manifest,
    is_folder_manifest,
    is_folderish_manifest,
    is_workspace_manifest,
    is_user_manifest,
)


class FSEntryNotFound(Exception):
    pass


class FSManifestLocalMiss(Exception):
    def __init__(self, access):
        super().__init__(access)
        self.access = access


class FSMultiManifestLocalMiss(Exception):
    def __init__(self, accesses):
        super().__init__(accesses)
        self.accesses = accesses


class LocalFolderFS:
    def __init__(self, device: LocalDevice, local_db: LocalDB, event_bus: EventBus):
        self.local_author = device.device_id
        self.root_access = device.user_manifest_access
        self._local_db = local_db
        self.event_bus = event_bus
        self._manifests_cache = {}
        self._hot_files = {}

    def _register_hot_file(self, access_id, hot_file):
        self._hot_files[access_id] = hot_file

    def _delete_hot_file(self, access_id):
        self._hot_files.pop(access_id, None)

    def get_local_vlob_groups(self) -> List[UUID]:
        # vlob_group_id is either the id of the user manifest or of a workpace manifest
        vlob_groups = [self.root_access.id]
        try:
            root_manifest = self._get_manifest_read_only(self.root_access)
            # Currently workspace can only direct children of the user manifest
            for child_access in root_manifest.children.values():
                try:
                    child_manifest = self._get_manifest_read_only(child_access)
                except FSManifestLocalMiss:
                    continue
                if is_workspace_manifest(child_manifest):
                    vlob_groups.append(child_access.id)
        except FSManifestLocalMiss:
            raise AssertionError("root manifest should always be available in local !")
        return vlob_groups

    def dump(self) -> dict:
        def _recursive_dump(access: Access):
            dump_data = {"access": attr.asdict(access)}
            try:
                manifest = self.get_manifest(access)
            except FSManifestLocalMiss:
                return dump_data
            dump_data.update(attr.asdict(manifest))
            if is_user_manifest(manifest):
                dump_data["children"] = {}

            if is_folderish_manifest(manifest):
                for child_name, child_access in manifest.children.items():
                    dump_data["children"][child_name] = _recursive_dump(child_access)

            return dump_data

        return _recursive_dump(self.root_access)

    def _get_raw_manifest_from_local_db(self, access: Access) -> LocalManifest:
        # Try local manifest first, although it should not matter
        try:
            return self._local_db.get_dirty_manifest(access)
        # Then remote manifest
        except LocalDBMissingEntry:
            return self._local_db.get_clean_manifest(access)

    def _get_manifest_read_only(self, access: Access) -> LocalManifest:
        try:
            return self._manifests_cache[access.id]
        except KeyError:
            pass
        try:
            raw = self._get_raw_manifest_from_local_db(access)
        except LocalDBMissingEntry as exc:
            # Last chance: if we are looking for the user manifest, we can
            # fake to know it version 0, which is useful during boostrap step
            if access == self.root_access:
                manifest = LocalUserManifest(self.local_author)
            else:
                raise FSManifestLocalMiss(access) from exc
        else:
            manifest = local_manifest_serializer.loads(raw)
        self._manifests_cache[access.id] = manifest
        # TODO: shouldn't be processed in multiple places like this...
        if is_workspace_manifest(manifest):
            path, *_ = self.get_entry_path(access.id)
            self.event_bus.send("fs.workspace.loaded", path=str(path), id=access.id)
        return manifest

    def get_user_manifest(self) -> LocalUserManifest:
        """
        Same as `get_manifest`, unlike this cannot fail given user manifest is
        always available.
        """
        try:
            return self._get_manifest_read_only(self.root_access)

        except FSManifestLocalMiss as exc:
            raise RuntimeError("Should never occurs !!!") from exc

    def get_manifest(self, access: Access) -> LocalManifest:
        return self._get_manifest_read_only(access)

    def set_clean_manifest(self, access: Access, manifest: LocalManifest, force=False):
        # Always keep the user manifest locally
        if access == self.root_access:
            return self.set_dirty_manifest(access, manifest)
        # Remove the corresponding local manifest if it exists
        if force:
            try:
                self._local_db.clear_dirty_manifest(access)
            except LocalDBMissingEntry:
                pass
        # Serialize and set remote manifest
        raw = local_manifest_serializer.dumps(manifest)
        self._local_db.set_clean_manifest(access, raw)
        self._manifests_cache[access.id] = manifest

    def set_dirty_manifest(self, access: Access, manifest: LocalManifest):
        try:
            self._local_db.clear_clean_manifest(access)
        except LocalDBMissingEntry:
            pass
        raw = local_manifest_serializer.dumps(manifest)
        self._local_db.set_dirty_manifest(access, raw)
        self._manifests_cache[access.id] = manifest

    def mark_outdated_manifest(self, access: Access):
        try:
            self._local_db.clear_clean_manifest(access)
        except LocalDBMissingEntry:
            try:
                self._local_db.clear_dirty_manifest(access)
            except LocalDBMissingEntry:
                pass
        self._manifests_cache.pop(access.id, None)

    def get_vlob_group(self, path: FsPath) -> UUID:
        # The vlob group is used to notify other clients that we modified an entry.
        # We try to use the id of workspace containing the modification as
        # vlob group. This is not possible when directly modifying the user
        # manifest in which case we use the user manifest id as vlob group.
        try:
            _, workspace_name, *_ = path.parts
        except ValueError:
            return self.root_access.id

        access, manifest = self._retrieve_entry_read_only(FsPath(f"/{workspace_name}"))
        assert is_workspace_manifest(manifest)
        return access.id

    def get_entry(self, path: FsPath) -> Tuple[Access, LocalManifest]:
        return self._retrieve_entry(path)

    def get_entry_path(self, entry_id: UUID) -> Tuple[FsPath, Access, LocalManifest]:
        """
        Raises:
            FSEntryNotFound: If the entry is not present in local
        """
        if entry_id == self.root_access.id:
            return FsPath("/"), self.root_access, self.get_manifest(self.root_access)

        # Brute force style
        def _recursive_search(access, path):
            try:
                manifest = self._get_manifest_read_only(access)
            except FSManifestLocalMiss:
                return
            if access.id == entry_id:
                return path, access, manifest

            if is_folderish_manifest(manifest):
                for child_name, child_access in manifest.children.items():
                    found = _recursive_search(child_access, path / child_name)
                    if found:
                        return found

        found = _recursive_search(self.root_access, FsPath("/"))
        if not found:
            raise FSEntryNotFound(entry_id)
        return found

    def _retrieve_entry(self, path: FsPath, collector=None) -> Tuple[Access, LocalManifest]:
        access, read_only_manifest = self._retrieve_entry_read_only(path, collector)
        return access, read_only_manifest

    def _retrieve_entry_read_only(
        self, path: FsPath, collector=None
    ) -> Tuple[Access, LocalManifest]:
        curr_access = self.root_access
        curr_manifest = self._get_manifest_read_only(curr_access)
        if collector:
            collector(curr_access, curr_manifest)

        try:
            _, *hops, dest = list(path.walk_to_path())
        except ValueError:
            return curr_access, curr_manifest

        for hop in hops:
            try:
                curr_access = curr_manifest.children[hop.name]
            except KeyError:
                raise FileNotFoundError(2, "No such file or directory", str(hop))

            curr_manifest = self._get_manifest_read_only(curr_access)

            if not is_folderish_manifest(curr_manifest):
                raise NotADirectoryError(20, "Not a directory", str(hop))
            if collector:
                collector(curr_access, curr_manifest)

        try:
            curr_access = curr_manifest.children[dest.name]
        except KeyError:
            raise FileNotFoundError(2, "No such file or directory", str(dest))

        curr_manifest = self._get_manifest_read_only(curr_access)

        if collector:
            collector(curr_access, curr_manifest)

        return curr_access, curr_manifest

    def get_sync_strategy(self, path: FsPath, recursive: dict) -> Tuple[FsPath, dict]:
        # Consider root is never a placeholder
        for curr_path in path.walk_to_path():
            _, curr_manifest = self._retrieve_entry_read_only(curr_path)
            if curr_manifest.is_placeholder:
                sync_path = curr_path.parent
                break
        else:
            return path, recursive

        sync_recursive = recursive
        for curr_path in path.walk_from_path():
            if curr_path == sync_path:
                break
            sync_recursive = {curr_path.name: sync_recursive}

        return sync_path, sync_recursive

    def get_access(self, path: FsPath, mode="rw") -> Access:
        if not path.is_root() and "w" in mode:
            self._ensure_workspace_write_right(path.parts[1])
        access, _ = self._retrieve_entry_read_only(path)
        return access

    def stat(self, path: FsPath) -> dict:
        access, manifest = self._retrieve_entry_read_only(path)
        if is_file_manifest(manifest):
            hf = self._hot_files.get(access.id)
            if hf:
                size = hf.size
                need_sync = hf.pending_writes or manifest.need_sync
            else:
                size = manifest.size
                need_sync = manifest.need_sync

            return {
                "id": access.id,
                "type": "file",
                "is_folder": False,
                "created": manifest.created,
                "updated": manifest.updated,
                "base_version": manifest.base_version,
                "is_placeholder": manifest.is_placeholder,
                "need_sync": need_sync,
                "size": size,
            }

        elif is_workspace_manifest(manifest):
            entry = self._retrieve_workspace_entry(path.parts[1])
            return {
                "id": access.id,
                "type": "workspace",
                "admin_right": entry.admin_right,
                "read_right": entry.read_right,
                "write_right": entry.write_right,
                "is_folder": True,
                "created": manifest.created,
                "updated": manifest.updated,
                "base_version": manifest.base_version,
                "is_placeholder": manifest.is_placeholder,
                "need_sync": manifest.need_sync,
                "children": list(sorted(manifest.children.keys())),
                "creator": manifest.creator,
                "participants": list(manifest.participants),
            }

        elif is_user_manifest(manifest):
            # Only list workspace we still have access to
            workspaces = [
                we.name
                for we in manifest.workspaces
                if we.admin_right or we.read_right or we.write_right
            ]
            return {
                "type": "root",
                "id": access.id,
                "is_folder": True,
                "created": manifest.created,
                "updated": manifest.updated,
                "base_version": manifest.base_version,
                "is_placeholder": manifest.is_placeholder,
                "need_sync": manifest.need_sync,
                # TODO: rename this to workspaces ?
                "children": list(sorted(workspaces)),
            }

        else:  # folder manifest
            return {
                "type": "folder",
                "id": access.id,
                "is_folder": True,
                "created": manifest.created,
                "updated": manifest.updated,
                "base_version": manifest.base_version,
                "is_placeholder": manifest.is_placeholder,
                "need_sync": manifest.need_sync,
                "children": list(sorted(manifest.children.keys())),
            }

    def _retrieve_workspace_entry(self, workspace):
        user_manifest = self.get_user_manifest()
        return next((we for we in user_manifest.workspaces if we.name == workspace), None)

    def _ensure_workspace_write_right(self, workspace):
        workspace_entry = self._retrieve_workspace_entry(workspace)
        if workspace_entry and not workspace_entry.write_right:
            raise PermissionError(13, "No write right for workspace", f"/{workspace}")

    def touch(self, path: FsPath) -> UUID:
        if path.is_root():
            raise FileExistsError(17, "File exists", str(path))

        if path.parent.is_root():
            raise PermissionError(
                13, "Permission denied (only workpace allowed at root level)", str(path)
            )

        self._ensure_workspace_write_right(path.parts[1])

        access, manifest = self._retrieve_entry(path.parent)
        if not is_folderish_manifest(manifest):
            raise NotADirectoryError(20, "Not a directory", str(path.parent))
        if path.name in manifest.children:
            raise FileExistsError(17, "File exists", str(path))

        child_access = ManifestAccess()
        child_manifest = LocalFileManifest(self.local_author)
        manifest = manifest.evolve_children_and_mark_updated({path.name: child_access})
        self.set_dirty_manifest(access, manifest)
        self.set_dirty_manifest(child_access, child_manifest)
        self.event_bus.send("fs.entry.updated", id=access.id)
        self.event_bus.send("fs.entry.updated", id=child_access.id)

        return child_access.id

    def mkdir(self, path: FsPath) -> None:
        if path.is_root():
            raise FileExistsError(17, "File exists", str(path))

        if path.parent.is_root():
            raise PermissionError(
                13, "Permission denied (only workpace allowed at root level)", str(path)
            )

        self._ensure_workspace_write_right(path.parts[1])

        access, manifest = self._retrieve_entry(path.parent)
        if not is_folderish_manifest(manifest):
            raise NotADirectoryError(20, "Not a directory", str(path.parent))
        if path.name in manifest.children:
            raise FileExistsError(17, "File exists", str(path))

        child_access = ManifestAccess()
        child_manifest = LocalFolderManifest(self.local_author)
        manifest = manifest.evolve_children_and_mark_updated({path.name: child_access})

        self.set_dirty_manifest(access, manifest)
        self.set_dirty_manifest(child_access, child_manifest)
        self.event_bus.send("fs.entry.updated", id=access.id)
        self.event_bus.send("fs.entry.updated", id=child_access.id)

        return child_access.id

    def workspace_create(self, path: FsPath) -> UUID:
        if not path.parent.is_root():
            raise PermissionError(
                13, "Permission denied (workspace only allowed at root level)", str(path)
            )

        root_manifest = self.get_user_manifest()
        if path.name in root_manifest.children:
            raise FileExistsError(17, "File exists", str(path))

        workspace_entry = WorkspaceEntry(path.name)
        workspace_manifest = LocalWorkspaceManifest(self.local_author)
        workspace_access = workspace_entry.access
        root_manifest = root_manifest.evolve_workspaces_and_mark_updated(workspace_entry)

        self.set_dirty_manifest(self.root_access, root_manifest)
        self.set_dirty_manifest(workspace_access, workspace_manifest)
        self.event_bus.send("fs.entry.updated", id=self.root_access.id)
        self.event_bus.send("fs.entry.updated", id=workspace_access.id)

        self.event_bus.send("fs.workspace.loaded", path=str(path), id=workspace_access.id)

        return workspace_access.id

    def workspace_rename(self, src: FsPath, dst: FsPath) -> None:
        """
        Workspace is the only manifest that is allowed to be moved without
        changing it access.
        The reason behind this is changing a workspace's access means creating
        a completely new workspace... And on the other hand the name of a
        workspace is not globally unique so a given user should be able to
        change it.
        """
        src_access, src_manifest = self._retrieve_entry_read_only(src)
        if not is_workspace_manifest(src_manifest):
            raise PermissionError(13, "Permission denied (not a workspace)", str(src), str(dst))
        if not dst.is_workspace():
            raise PermissionError(
                13, "Permission denied (workspace must be direct root child)", str(src), str(dst)
            )

        # No point in raising a FileExistsError in this case
        if src == dst:
            return

        root_manifest = self.get_user_manifest()
        if dst.name in root_manifest.children:
            raise FileExistsError(17, "File exists", str(dst))

        # Just move the workspace's access from one place to another
        workspace_entry = self._retrieve_workspace_entry(src.parts[1])
        workspace_entry = workspace_entry.evolve(name=dst.name)
        root_manifest = root_manifest.evolve_workspaces_and_mark_updated(workspace_entry)
        self.set_dirty_manifest(self.root_access, root_manifest)

        self.event_bus.send("fs.entry.updated", id=self.root_access.id)

    def _delete(self, path: FsPath, expect=None) -> None:
        if path.is_root() or path.is_workspace():
            raise PermissionError(13, "Permission denied", str(path))

        self._ensure_workspace_write_right(path.parts[1])

        parent_access, parent_manifest = self._retrieve_entry(path.parent)
        if not is_folderish_manifest(parent_manifest):
            raise NotADirectoryError(20, "Not a directory", str(path.parent))

        try:
            item_access = parent_manifest.children[path.name]
        except KeyError:
            raise FileNotFoundError(2, "No such file or directory", str(path))

        item_manifest = self.get_manifest(item_access)
        if is_folderish_manifest(item_manifest):
            if expect == "file":
                raise IsADirectoryError(21, "Is a directory", str(path))
            if item_manifest.children:
                raise OSError(39, "Directory not empty", str(path))
        elif expect == "folder":
            raise NotADirectoryError(20, "Not a directory", str(path))

        parent_manifest = parent_manifest.evolve_children_and_mark_updated({path.name: None})
        self.set_dirty_manifest(parent_access, parent_manifest)
        # TODO: If a file is opened while getting removed, we cannot
        # drop the file manifest given a subsequent read/write would
        # need it.
        # The `LocalFolderFS._hot_files` is kind of a hack around this
        # (but we endup leaking old files manifests), we should replace
        # this by a clean ref counting strategy.
        if item_access.id not in self._hot_files:
            self.mark_outdated_manifest(item_access)
        self.event_bus.send("fs.entry.updated", id=parent_access.id)

    def delete(self, path: FsPath) -> None:
        return self._delete(path)

    def unlink(self, path: FsPath) -> None:
        return self._delete(path, expect="file")

    def rmdir(self, path: FsPath) -> None:
        return self._delete(path, expect="folder")

    def move(self, src: FsPath, dst: FsPath, overwrite: bool = True) -> None:
        # The idea here is to consider a manifest never move around the fs
        # (i.e. a given access always points to the same path). This simplify
        # sync notifications handling and avoid ending up with two path
        # (possibly from different workspaces !) pointing to the same manifest
        # which would be weird for user experience.
        # Long story short, when moving a manifest, we must recursively
        # copy the manifests and create new accesses for them.

        parent_src = src.parent
        parent_dst = dst.parent

        # No matter what, cannot move or overwrite root
        if src.is_root():
            # Raise FileNotFoundError if parent_dst doesn't exists
            _, parent_dst_manifest = self._retrieve_entry(parent_dst)
            if not is_folderish_manifest(parent_dst_manifest):
                raise NotADirectoryError(20, "Not a directory", str(parent_dst))
            else:
                raise PermissionError(13, "Permission denied", str(src), str(dst))
        elif dst.is_root():
            # Raise FileNotFoundError if parent_src doesn't exists
            _, parent_src_manifest = self._retrieve_entry(src.parent)
            if not is_folderish_manifest(parent_src_manifest):
                raise NotADirectoryError(20, "Not a directory", str(src.parent))
            else:
                raise PermissionError(13, "Permission denied", str(src), str(dst))

        self._ensure_workspace_write_right(src.parts[1])
        self._ensure_workspace_write_right(dst.parts[1])

        if src == dst:
            # Raise FileNotFoundError if doesn't exist
            src_access, src_ro_manifest = self._retrieve_entry_read_only(src)
            if is_workspace_manifest(src_ro_manifest):
                raise PermissionError(
                    13,
                    "Permission denied (cannot move/copy workpace, must rename it)",
                    str(src),
                    str(dst),
                )
            return

        if parent_src == parent_dst:
            parent_access, parent_manifest = self._retrieve_entry(parent_src)
            if not is_folderish_manifest(parent_manifest):
                raise NotADirectoryError(20, "Not a directory", str(parent_src))

            try:
                dst.relative_to(src)
            except ValueError:
                pass
            else:
                raise OSError(22, "Invalid argument", str(src), None, str(dst))

            try:
                src_access = parent_manifest.children[src.name]
            except KeyError:
                raise FileNotFoundError(2, "No such file or directory", str(src))

            existing_dst_access = parent_manifest.children.get(dst.name)
            src_manifest = self.get_manifest(src_access)

            if is_workspace_manifest(src_manifest):
                raise PermissionError(
                    13,
                    "Permission denied (cannot move/copy workpace, must rename it)",
                    str(src),
                    str(dst),
                )

            if existing_dst_access:
                if not overwrite:
                    raise FileExistsError(17, "File exists", str(dst))

                existing_dst_manifest = self.get_manifest(existing_dst_access)
                if is_folderish_manifest(src_manifest):
                    if is_file_manifest(existing_dst_manifest):
                        raise NotADirectoryError(20, "Not a directory", str(dst))
                    elif existing_dst_manifest.children:
                        raise OSError(39, "Directory not empty", str(dst))
                else:
                    if is_folderish_manifest(existing_dst_manifest):
                        raise IsADirectoryError(21, "Is a directory", str(dst))

            moved_access = self._recursive_manifest_copy(src_access, src_manifest)

            parent_manifest = parent_manifest.evolve_children_and_mark_updated(
                {dst.name: moved_access, src.name: None}
            )
            self.set_dirty_manifest(parent_access, parent_manifest)
            self.event_bus.send("fs.entry.updated", id=parent_access.id)

        elif parent_dst.is_root():
            raise PermissionError(
                13, "Permission denied (only workspaces can be moved to root)", str(src), str(dst)
            )

        else:
            parent_src_access, parent_src_manifest = self._retrieve_entry(parent_src)
            if not is_folderish_manifest(parent_src_manifest):
                raise NotADirectoryError(20, "Not a directory", str(parent_src))

            parent_dst_access, parent_dst_manifest = self._retrieve_entry(parent_dst)
            if not is_folderish_manifest(parent_dst_manifest):
                raise NotADirectoryError(20, "Not a directory", str(parent_dst))

            try:
                dst.relative_to(src)
            except ValueError:
                pass
            else:
                raise OSError(22, "Invalid argument", str(src), None, str(dst))

            try:
                src_access = parent_src_manifest.children[src.name]
            except KeyError:
                raise FileNotFoundError(2, "No such file or directory", str(src))

            existing_dst_access = parent_dst_manifest.children.get(dst.name)
            src_manifest = self.get_manifest(src_access)

            if is_workspace_manifest(src_manifest):
                raise PermissionError(
                    13,
                    "Permission denied (cannot move/copy workpace, must rename it)",
                    str(src),
                    str(dst),
                )

            if existing_dst_access:
                if not overwrite:
                    raise FileExistsError(17, "File exists", str(dst))

                existing_entry_manifest = self.get_manifest(existing_dst_access)
                if is_folderish_manifest(src_manifest):
                    if is_file_manifest(existing_entry_manifest):
                        raise NotADirectoryError(20, "Not a directory", str(dst))
                    elif existing_entry_manifest.children:
                        raise OSError(39, "Directory not empty", str(dst))
                else:
                    if is_folderish_manifest(existing_entry_manifest):
                        raise IsADirectoryError(21, "Is a directory", str(dst))

            moved_access = self._recursive_manifest_copy(src_access, src_manifest)

            # Update destination
            parent_dst_manifest = parent_dst_manifest.evolve_children_and_mark_updated(
                {dst.name: moved_access}
            )
            self.set_dirty_manifest(parent_dst_access, parent_dst_manifest)
            self.event_bus.send("fs.entry.updated", id=parent_dst_access.id)

            # Update source
            parent_src_manifest = parent_src_manifest.evolve_children_and_mark_updated(
                {src.name: None}
            )
            self.set_dirty_manifest(parent_src_access, parent_src_manifest)
            self.event_bus.send("fs.entry.updated", id=parent_src_access.id)

    def _recursive_manifest_copy(self, access, manifest):
        # Copying a workspace is a bad idea given new and old ones will share
        # the blocks, making it really hard to do history data cleanup.
        assert not is_workspace_manifest(manifest)

        # First, make sure all the manifest have to copy are locally present
        # (otherwise we will have to stop while part of the manifest are
        # already copied, creating garbage and making us doing more retries
        # that strictly needed)

        def _recursive_create_copy_map(access, manifest):
            copy_map = {"access": access, "manifest": manifest}
            manifests_miss = []

            if is_folderish_manifest(manifest):
                copy_map["children"] = {}

                for child_name, child_access in manifest.children.items():
                    try:
                        child_manifest = self._get_manifest_read_only(child_access)
                    except FSManifestLocalMiss as exc:
                        manifests_miss.append(exc.access)
                    else:
                        try:
                            copy_map["children"][child_name] = _recursive_create_copy_map(
                                child_access, child_manifest
                            )
                        except FSMultiManifestLocalMiss as exc:
                            manifests_miss += exc.accesses

            if manifests_miss:
                raise FSMultiManifestLocalMiss(manifests_miss)
            return copy_map

        copy_map = _recursive_create_copy_map(access, manifest)

        # Now we can walk the copy map and copy each manifest and create new
        # corresponding accesses

        def _recursive_process_copy_map(copy_map):
            manifest = copy_map["manifest"]

            cpy_access = ManifestAccess()
            if is_file_manifest(manifest):
                cpy_manifest = LocalFileManifest(
                    author=self.local_author,
                    size=manifest.size,
                    blocks=manifest.blocks,
                    dirty_blocks=manifest.dirty_blocks,
                )

            else:
                assert is_folder_manifest(manifest)

                cpy_children = {}
                for child_name in manifest.children.keys():
                    child_copy_map = copy_map["children"][child_name]
                    new_child_access = _recursive_process_copy_map(child_copy_map)
                    cpy_children[child_name] = new_child_access

                cpy_manifest = LocalFolderManifest(author=self.local_author, children=cpy_children)

            self.set_dirty_manifest(cpy_access, cpy_manifest)
            return cpy_access

        return _recursive_process_copy_map(copy_map)

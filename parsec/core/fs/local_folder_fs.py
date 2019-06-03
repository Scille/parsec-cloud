# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from uuid import UUID
from typing import Tuple

from parsec.event_bus import EventBus
from parsec.core.types import (
    FsPath,
    EntryID,
    BlockAccess,
    ManifestAccess,
    WorkspaceEntry,
    WorkspaceRole,
    LocalDevice,
    LocalManifest,
    LocalFileManifest,
    LocalFolderManifest,
    LocalWorkspaceManifest,
    LocalUserManifest,
)
from parsec.core.local_storage import LocalStorage, LocalStorageMissingError
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
    def __init__(self, entry_id):
        super().__init__(entry_id)
        self.entry_id = entry_id


class FSMultiManifestLocalMiss(Exception):
    def __init__(self, entries_ids):
        super().__init__(entries_ids)
        self.entries_ids = entries_ids


class LocalFolderFS:
    def __init__(self, device: LocalDevice, local_storage: LocalStorage, event_bus: EventBus):
        self.local_author = device.device_id
        self.root_access = ManifestAccess(
            device.user_manifest_id, device.user_manifest_id, device.user_manifest_key, 1
        )
        self.local_storage = local_storage
        self.event_bus = event_bus

    # Manifest methods

    def get_manifest(self, access: ManifestAccess) -> LocalManifest:
        try:
            return self.local_storage.get_manifest(access.id)
        except LocalStorageMissingError as exc:
            # Last chance: if we are looking for the user manifest, we can
            # fake to know it version 0, which is useful during boostrap step
            if access.id == self.root_access.id:
                manifest = LocalUserManifest(self.local_author)
                self.set_dirty_manifest(access, manifest)
                return manifest
            raise FSManifestLocalMiss(access) from exc
        # TODO: move this block to the right place
        # if is_workspace_manifest(manifest):
        #     path, *_ = self.get_entry_path(access.id)
        #     self.event_bus.send("fs.workspace.loaded", path=str(path), id=access.id)

    def get_user_manifest(self) -> LocalUserManifest:
        """
        Same as `get_manifest`, unlike this cannot fail given user manifest is
        always available.
        """
        try:
            return self.get_manifest(self.root_access)

        except FSManifestLocalMiss as exc:
            raise RuntimeError("Should never occurs !!!") from exc

    def set_clean_manifest(self, access: ManifestAccess, manifest: LocalManifest, force=False):
        # Always keep the user manifest locally
        if access == self.root_access:
            return self.set_dirty_manifest(access, manifest)
        if manifest.need_sync:
            raise ValueError("The given manifest is not a clean manifest.")
        self.local_storage.set_clean_manifest(access.id, manifest, force)

    def set_dirty_manifest(self, access: ManifestAccess, manifest: LocalManifest):
        if access != self.root_access and not manifest.need_sync:
            raise ValueError("The given manifest is not a dirty manifest.")
        self.local_storage.set_dirty_manifest(access.id, manifest)

    def clear_manifest(self, access: ManifestAccess):
        self.local_storage.clear_manifest(access.id)

    def dump(self) -> dict:
        def _recursive_dump(access: ManifestAccess):
            dump_data = {"access": attr.asdict(access)}
            try:
                manifest = self.get_manifest(access)
            except FSManifestLocalMiss:
                return dump_data
            manifest_data = attr.asdict(manifest)
            # TODO: due to legacy reason, workspaces is provided by `manifest.children`
            manifest_data.pop("workspaces", None)
            dump_data.update(manifest_data)
            if is_user_manifest(manifest):
                dump_data["children"] = {}

            if is_folderish_manifest(manifest):
                for child_name, child_entry_id in manifest.children.items():
                    child_access = ManifestAccess(
                        child_entry_id, access.realm_id, access.key, access.encryption_revision
                    )
                    dump_data["children"][child_name] = _recursive_dump(child_access)

            return dump_data

        return _recursive_dump(self.root_access)

    def get_entry(self, path: FsPath) -> Tuple[ManifestAccess, LocalManifest]:
        return self._retrieve_entry(path)

    def get_entry_path(self, entry_id: UUID) -> Tuple[FsPath, ManifestAccess, LocalManifest]:
        """
        Raises:
            FSEntryNotFound: If the entry is not present in local
        """
        user_manifest = self.get_user_manifest()
        if entry_id == self.root_access.id:
            return FsPath("/"), self.root_access, user_manifest

        # Brute force style
        def _recursive_search_on_workspace(access, path):
            try:
                manifest = self.get_manifest(access)
            except FSManifestLocalMiss:
                return
            if access.id == entry_id:
                return path, access, manifest

            if is_folderish_manifest(manifest):
                for child_name, child_entry_id in manifest.children.items():
                    child_access = ManifestAccess(
                        child_entry_id, access.realm_id, access.key, access.encryption_revision
                    )
                    found = _recursive_search_on_workspace(child_access, path / child_name)
                    if found:
                        return found

        for workspace_entry in user_manifest.workspaces:
            workspace_path = FsPath(f"/{workspace_entry.name}")
            workspace_access = ManifestAccess(
                workspace_entry.id,
                workspace_entry.id,
                workspace_entry.key,
                workspace_entry.encryption_revision,
            )
            found = _recursive_search_on_workspace(workspace_access, workspace_path)
            if found:
                return found

        else:
            raise FSEntryNotFound(entry_id)

    def _retrieve_entry(self, path: FsPath, collector=None) -> Tuple[ManifestAccess, LocalManifest]:
        access, read_only_manifest = self._retrieve_entry_read_only(path, collector)
        return access, read_only_manifest

    def _retrieve_entry_read_only(
        self, path: FsPath, collector=None
    ) -> Tuple[ManifestAccess, LocalManifest]:
        curr_access = self.root_access
        curr_manifest = self.get_manifest(curr_access)
        if collector:
            collector(curr_access, curr_manifest)

        try:
            _, *hops, dest = list(path.walk_to_path())
        except ValueError:
            return curr_access, curr_manifest

        def _retrieve_hop(hop):
            nonlocal curr_access

            if hasattr(curr_manifest, "workspaces"):
                # curr_manifest is a User manifest
                workspace_entry = next(
                    (w for w in curr_manifest.workspaces if w.name == hop.name), None
                )
                if not workspace_entry:
                    raise FileNotFoundError(2, "No such file or directory", str(hop))
                curr_access = ManifestAccess(
                    workspace_entry.id,
                    workspace_entry.id,
                    workspace_entry.key,
                    workspace_entry.encryption_revision,
                )

            else:
                try:
                    curr_access = ManifestAccess(
                        curr_manifest.children[hop.name],
                        curr_access.realm_id,
                        curr_access.key,
                        curr_access.encryption_revision,
                    )
                except KeyError:
                    raise FileNotFoundError(2, "No such file or directory", str(hop))

            return curr_access, self.get_manifest(curr_access)

        for hop in hops:
            curr_access, curr_manifest = _retrieve_hop(hop)

            if not is_folderish_manifest(curr_manifest):
                raise NotADirectoryError(20, "Not a directory", str(hop))
            if collector:
                collector(curr_access, curr_manifest)

        curr_access, curr_manifest = _retrieve_hop(dest)
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

    def get_access(self, path: FsPath, mode="rw") -> ManifestAccess:
        if not path.is_root() and "w" in mode:
            self._ensure_workspace_write_right(path.parts[1])
        access, _ = self._retrieve_entry_read_only(path)
        return access

    def get_file_access(self, path: FsPath, mode="rw") -> ManifestAccess:
        access = self.get_access(path, mode)
        manifest = self.get_manifest(access)
        if not is_file_manifest(manifest):
            raise IsADirectoryError(21, "Is a directory")
        return access

    def stat(self, path: FsPath) -> dict:
        access, manifest = self._retrieve_entry_read_only(path)
        if is_file_manifest(manifest):
            return {
                "id": access.id,
                "type": "file",
                "is_folder": False,
                "created": manifest.created,
                "updated": manifest.updated,
                "base_version": manifest.base_version,
                "is_placeholder": manifest.is_placeholder,
                "need_sync": manifest.need_sync,
                "size": manifest.size,
            }

        elif is_workspace_manifest(manifest):
            entry = self._retrieve_workspace_entry(path.parts[1])
            return {
                "id": access.id,
                "type": "workspace",
                # TODO: reeeeally hacky legacy compatibility...
                "admin_right": entry.role in (WorkspaceRole.MANAGER, WorkspaceRole.OWNER),
                "read_right": entry.role is not None,
                "write_right": entry.role != WorkspaceRole.READER,
                "is_folder": True,
                "created": manifest.created,
                "updated": manifest.updated,
                "base_version": manifest.base_version,
                "is_placeholder": manifest.is_placeholder,
                "need_sync": manifest.need_sync,
                "children": list(sorted(manifest.children.keys())),
            }

        elif is_user_manifest(manifest):
            # Only list workspace we still have access to
            workspaces = [we.name for we in manifest.workspaces if not we.is_revoked()]
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
        if workspace_entry and workspace_entry.role in (None, WorkspaceRole.READER):
            raise PermissionError(13, "No write right for workspace", f"/{workspace}")

    def touch(self, path: FsPath) -> UUID:
        if path.is_root():
            raise FileExistsError(17, "File exists", str(path))

        if path.parent.is_root():
            raise PermissionError(
                13, "Permission denied (only workspace allowed at root level)", str(path)
            )

        self._ensure_workspace_write_right(path.parts[1])

        access, manifest = self._retrieve_entry(path.parent)
        if not is_folderish_manifest(manifest):
            raise NotADirectoryError(20, "Not a directory", str(path.parent))
        if path.name in manifest.children:
            raise FileExistsError(17, "File exists", str(path))

        child_access = ManifestAccess(
            EntryID(), access.realm_id, access.key, access.encryption_revision
        )
        child_manifest = LocalFileManifest(self.local_author)
        manifest = manifest.evolve_children_and_mark_updated({path.name: child_access.id})
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
                13, "Permission denied (only workspace allowed at root level)", str(path)
            )

        self._ensure_workspace_write_right(path.parts[1])

        access, manifest = self._retrieve_entry(path.parent)
        if not is_folderish_manifest(manifest):
            raise NotADirectoryError(20, "Not a directory", str(path.parent))
        if path.name in manifest.children:
            raise FileExistsError(17, "File exists", str(path))

        child_access = ManifestAccess(
            EntryID(), access.realm_id, access.key, access.encryption_revision
        )
        child_manifest = LocalFolderManifest(self.local_author)
        manifest = manifest.evolve_children_and_mark_updated({path.name: child_access.id})

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
        workspace_access = ManifestAccess(
            workspace_entry.id,
            workspace_entry.id,
            workspace_entry.key,
            WorkspaceEntry.encryption_revision,
        )
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
            item_entry_id = parent_manifest.children[path.name]
            item_access = ManifestAccess(
                item_entry_id,
                parent_access.realm_id,
                parent_access.key,
                parent_access.encryption_revision,
            )
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
        if is_file_manifest(item_manifest):
            self.local_storage.remove_file_reference(item_access, item_manifest)
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
                    "Permission denied (cannot move/copy workspace, must rename it)",
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
                src_entry_id = parent_manifest.children[src.name]
                src_access = ManifestAccess(
                    src_entry_id,
                    parent_access.realm_id,
                    parent_access.key,
                    parent_access.encryption_revision,
                )
            except KeyError:
                raise FileNotFoundError(2, "No such file or directory", str(src))

            existing_dst_id = parent_manifest.children.get(dst.name)
            src_manifest = self.get_manifest(src_access)

            if is_workspace_manifest(src_manifest):
                raise PermissionError(
                    13,
                    "Permission denied (cannot move/copy workspace, must rename it)",
                    str(src),
                    str(dst),
                )

            if existing_dst_id:
                existing_dst_access = ManifestAccess(
                    existing_dst_id,
                    parent_access.realm_id,
                    parent_access.key,
                    parent_access.encryption_revision,
                )
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
                {dst.name: moved_access.id, src.name: None}
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
                src_entry_id = parent_src_manifest.children[src.name]
                src_access = ManifestAccess(
                    src_entry_id,
                    parent_src_access.realm_id,
                    parent_src_access.key,
                    parent_src_access.encryption_revision,
                )
            except KeyError:
                raise FileNotFoundError(2, "No such file or directory", str(src))

            existing_dst_entry_id = parent_dst_manifest.children.get(dst.name)
            src_manifest = self.get_manifest(src_access)

            if is_workspace_manifest(src_manifest):
                raise PermissionError(
                    13,
                    "Permission denied (cannot move/copy workspace, must rename it)",
                    str(src),
                    str(dst),
                )

            if existing_dst_entry_id:
                existing_dst_access = ManifestAccess(
                    existing_dst_entry_id,
                    parent_dst_access.realm_id,
                    parent_dst_access.key,
                    parent_dst_access.encryption_revision,
                )
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
                {dst.name: moved_access.id}
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

                for child_name, child_entry_id in manifest.children.items():
                    child_access = ManifestAccess(
                        child_entry_id, access.realm_id, access.key, access.encryption_revision
                    )
                    try:
                        child_manifest = self.get_manifest(child_access)
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
            access = copy_map["access"]
            manifest = copy_map["manifest"]

            cpy_access = ManifestAccess(
                EntryID(), access.realm_id, access.key, access.encryption_revision
            )
            if is_file_manifest(manifest):

                # Temporary hack: copy the dirty blocks
                # Recursive move will be remove in the local_folder_fs refactoring
                new_dirty_blocks = []
                for access in manifest.dirty_blocks:
                    content = self.local_storage.get_block(access)
                    new_access = BlockAccess.from_block(content, access.offset)
                    self.local_storage.set_dirty_block(new_access, content)
                    new_dirty_blocks.append(new_access)

                cpy_manifest = LocalFileManifest(
                    author=self.local_author,
                    size=manifest.size,
                    blocks=manifest.blocks,
                    dirty_blocks=new_dirty_blocks,
                )

            else:
                assert is_folder_manifest(manifest)

                cpy_children = {}
                for child_name in manifest.children.keys():
                    child_copy_map = copy_map["children"][child_name]
                    new_child_access = _recursive_process_copy_map(child_copy_map)
                    cpy_children[child_name] = new_child_access.id

                cpy_manifest = LocalFolderManifest(author=self.local_author, children=cpy_children)

            self.set_dirty_manifest(cpy_access, cpy_manifest)

            if is_file_manifest(manifest):
                self.local_storage.remove_file_reference(access, manifest)
            else:
                self.clear_manifest(access)

            return cpy_access

        return _recursive_process_copy_map(copy_map)

from uuid import UUID
from typing import List, Tuple
import pendulum

from parsec.event_bus import EventBus
from parsec.core.types import LocalDevice
from parsec.core.local_db import LocalDB, LocalDBMissingEntry
from parsec.core.schemas import dumps_manifest, loads_manifest
from parsec.core.fs.utils import (
    is_file_manifest,
    is_folder_manifest,
    is_workspace_manifest,
    new_access,
    new_local_user_manifest,
    new_local_workspace_manifest,
    new_local_folder_manifest,
    new_local_file_manifest,
    copy_manifest,
)
from parsec.core.fs.types import Path, Access, LocalManifest, LocalUserManifest


def mark_manifest_modified(manifest: LocalManifest):
    manifest["updated"] = pendulum.now()
    manifest["need_sync"] = True


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
        self.local_author = device.id
        self.root_access = device.user_manifest_access
        self._local_db = local_db
        self.event_bus = event_bus
        self._manifests_cache = {}

    def get_local_beacons(self) -> List[UUID]:
        # beacon_id is either the id of the user manifest or of a workpace manifest
        beacons = [self.root_access["id"]]
        try:
            root_manifest = self._get_manifest_read_only(self.root_access)
            # Currently workspace can only direct children of the user manifest
            for child_access in root_manifest["children"].values():
                try:
                    child_manifest = self._get_manifest_read_only(child_access)
                except FSManifestLocalMiss as exc:
                    continue
                if is_workspace_manifest(child_manifest):
                    beacons.append(child_access["id"])
        except FSManifestLocalMiss as exc:
            raise AssertionError("root manifest should always be available in local !")
        return beacons

    def dump(self) -> dict:
        def _recursive_dump(access: Access):
            dump_data = {"access": access}
            try:
                manifest = self.get_manifest(access)
            except FSManifestLocalMiss:
                return dump_data
            dump_data.update(manifest)
            if is_folder_manifest(manifest):
                for child_name, child_access in manifest["children"].items():
                    dump_data["children"][child_name] = _recursive_dump(child_access)

            return dump_data

        return _recursive_dump(self.root_access)

    def _get_manifest_read_only(self, access: Access) -> LocalManifest:
        try:
            return self._manifests_cache[access["id"]]
        except KeyError:
            pass
        try:
            raw = self._local_db.get(access)
        except LocalDBMissingEntry as exc:
            # Last chance: if we are looking for the user manifest, we can
            # fake to know it version 0, which is useful during boostrap step
            if access == self.root_access:
                manifest = new_local_user_manifest(self.local_author)
            else:
                raise FSManifestLocalMiss(access) from exc
        else:
            manifest = loads_manifest(raw)
        self._manifests_cache[access["id"]] = manifest
        # TODO: shouldn't be processed in multiple places like this...
        if is_workspace_manifest(manifest):
            path, *_ = self.get_entry_path(access["id"])
            self.event_bus.send("fs.workspace.loaded", path=str(path), id=access["id"])
        return manifest

    def get_user_manifest(self) -> LocalUserManifest:
        """
        Same as `get_manifest`, unlike this cannot fail given user manifest is
        always available.
        """
        try:
            return copy_manifest(self._get_manifest_read_only(self.root_access))
        except FSManifestLocalMiss as exc:
            raise ValueError("Should never occurs !!!") from exc

    def get_manifest(self, access: Access) -> LocalManifest:
        return copy_manifest(self._get_manifest_read_only(access))

    def set_manifest(self, access: Access, manifest: LocalManifest):
        raw = dumps_manifest(manifest)
        self._local_db.set(access, raw, False)
        self._manifests_cache[access["id"]] = copy_manifest(manifest)

    def update_manifest(self, access: Access, manifest: LocalManifest):
        mark_manifest_modified(manifest)
        self.set_manifest(access, manifest)
        self._manifests_cache[access["id"]] = copy_manifest(manifest)

    def mark_outdated_manifest(self, access: Access):
        self._local_db.clear(access)
        self._manifests_cache.pop(access["id"], None)

    def get_beacon(self, path: Path) -> UUID:
        # The beacon is used to notify other clients that we modified an entry.
        # We try to use the id of workspace containing the modification as
        # beacon. This is not possible when directly modifying the user
        # manifest in which case we use the user manifest id as beacon.
        try:
            _, workspace_name, *_ = path.parts
        except ValueError:
            return self.root_access["id"]

        access, manifest = self._retrieve_entry_read_only(Path(f"/{workspace_name}"))
        assert is_workspace_manifest(manifest)
        return access["id"]

    def get_entry(self, path: Path) -> Tuple[Access, LocalManifest]:
        return self._retrieve_entry(path)

    def get_entry_path(self, entry_id: UUID) -> Tuple[Path, Access, LocalManifest]:
        """
        Raises:
            FSEntryNotFound: If the entry is not present in local
        """
        if entry_id == self.root_access["id"]:
            return Path("/"), self.root_access, self.get_manifest(self.root_access)

        # Brute force style
        def _recursive_search(access, path):
            try:
                manifest = self._get_manifest_read_only(access)
            except FSManifestLocalMiss:
                return
            if access["id"] == entry_id:
                return path, access, copy_manifest(manifest)

            if is_folder_manifest(manifest):
                for child_name, child_access in manifest["children"].items():
                    found = _recursive_search(child_access, path / child_name)
                    if found:
                        return found

        found = _recursive_search(self.root_access, Path("/"))
        if not found:
            raise FSEntryNotFound(entry_id)
        return found

    def _retrieve_entry(self, path: Path, collector=None) -> Tuple[Access, LocalManifest]:
        access, read_only_manifest = self._retrieve_entry_read_only(path, collector)
        return access, copy_manifest(read_only_manifest)

    def _retrieve_entry_read_only(self, path: Path, collector=None) -> Tuple[Access, LocalManifest]:
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
                curr_access = curr_manifest["children"][hop.name]
            except KeyError:
                raise FileNotFoundError(2, "No such file or directory", str(hop))

            curr_manifest = self._get_manifest_read_only(curr_access)

            if not is_folder_manifest(curr_manifest):
                raise NotADirectoryError(20, "Not a directory", str(hop))
            if collector:
                collector(curr_access, curr_manifest)

        try:
            curr_access = curr_manifest["children"][dest.name]
        except KeyError:
            raise FileNotFoundError(2, "No such file or directory", str(dest))

        curr_manifest = self._get_manifest_read_only(curr_access)

        if collector:
            collector(curr_access, curr_manifest)

        return curr_access, curr_manifest

    def get_sync_strategy(self, path: Path, recursive: dict) -> Tuple[Path, dict]:
        # Consider root is never a placeholder
        for curr_path in path.walk_to_path():
            _, curr_manifest = self._retrieve_entry_read_only(curr_path)
            if curr_manifest["is_placeholder"]:
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

    def get_access(self, path: Path) -> Access:
        access, _ = self._retrieve_entry_read_only(path)
        return access

    def stat(self, path: Path) -> dict:
        access, manifest = self._retrieve_entry_read_only(path)
        if is_file_manifest(manifest):
            return {
                "type": "file",
                "is_folder": False,
                "created": manifest["created"],
                "updated": manifest["updated"],
                "base_version": manifest["base_version"],
                "is_placeholder": manifest["is_placeholder"],
                "need_sync": manifest["need_sync"],
                "size": manifest["size"],
            }

        elif is_workspace_manifest(manifest):
            return {
                "type": "workspace",
                "is_folder": True,
                "created": manifest["created"],
                "updated": manifest["updated"],
                "base_version": manifest["base_version"],
                "is_placeholder": manifest["is_placeholder"],
                "need_sync": manifest["need_sync"],
                "children": list(sorted(manifest["children"].keys())),
                "creator": manifest["creator"],
                "participants": list(manifest["participants"]),
            }
        else:
            return {
                "type": "root" if path.is_root() else "folder",
                "is_folder": True,
                "created": manifest["created"],
                "updated": manifest["updated"],
                "base_version": manifest["base_version"],
                "is_placeholder": manifest["is_placeholder"],
                "need_sync": manifest["need_sync"],
                "children": list(sorted(manifest["children"].keys())),
            }

    def touch(self, path: Path) -> None:
        if path.is_root():
            raise FileExistsError(17, "File exists", str(path))

        if path.parent.is_root():
            raise PermissionError(
                13, "Permission denied (only workpace allowed at root level)", str(path)
            )

        access, manifest = self._retrieve_entry(path.parent)
        if not is_folder_manifest(manifest):
            raise NotADirectoryError(20, "Not a directory", str(path.parent))
        if path.name in manifest["children"]:
            raise FileExistsError(17, "File exists", str(path))

        child_access = new_access()
        child_manifest = new_local_file_manifest(self.local_author)
        manifest["children"][path.name] = child_access
        mark_manifest_modified(manifest)
        self.set_manifest(access, manifest)
        self.set_manifest(child_access, child_manifest)
        self.event_bus.send("fs.entry.updated", id=access["id"])
        self.event_bus.send("fs.entry.updated", id=child_access["id"])

    def mkdir(self, path: Path) -> None:
        if path.is_root():
            raise FileExistsError(17, "File exists", str(path))

        if path.parent.is_root():
            raise PermissionError(
                13, "Permission denied (only workpace allowed at root level)", str(path)
            )

        access, manifest = self._retrieve_entry(path.parent)
        if not is_folder_manifest(manifest):
            raise NotADirectoryError(20, "Not a directory", str(path.parent))
        if path.name in manifest["children"]:
            raise FileExistsError(17, "File exists", str(path))

        child_access = new_access()
        child_manifest = new_local_folder_manifest(self.local_author)

        manifest["children"][path.name] = child_access
        mark_manifest_modified(manifest)

        self.set_manifest(access, manifest)
        self.set_manifest(child_access, child_manifest)
        self.event_bus.send("fs.entry.updated", id=access["id"])
        self.event_bus.send("fs.entry.updated", id=child_access["id"])

    def workspace_create(self, path: Path) -> None:
        if not path.parent.is_root():
            raise PermissionError(
                13, "Permission denied (workspace only allowed at root level)", str(path)
            )

        root_manifest = self.get_user_manifest()
        if path.name in root_manifest["children"]:
            raise FileExistsError(17, "File exists", str(path))

        child_access = new_access()
        child_manifest = new_local_workspace_manifest(self.local_author)

        root_manifest["children"][path.name] = child_access
        mark_manifest_modified(root_manifest)

        self.set_manifest(self.root_access, root_manifest)
        self.set_manifest(child_access, child_manifest)
        self.event_bus.send("fs.entry.updated", id=self.root_access["id"])
        self.event_bus.send("fs.entry.updated", id=child_access["id"])

        self.event_bus.send("fs.workspace.loaded", path=str(path), id=child_access["id"])

    def workspace_rename(self, src: Path, dst: Path) -> None:
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
        if not dst.parent.is_root():
            raise PermissionError(
                13, "Permission denied (workspace must be direct root child)", str(src), str(dst)
            )

        root_manifest = self.get_user_manifest()
        if dst.name in root_manifest["children"]:
            raise FileExistsError(17, "File exists", str(dst))

        # Just move the workspace's access from one place to another
        root_manifest["children"][dst.name] = root_manifest["children"].pop(src.name)
        mark_manifest_modified(root_manifest)
        self.set_manifest(self.root_access, root_manifest)

        self.event_bus.send("fs.entry.updated", id=self.root_access["id"])

    def _delete(self, path: Path, expect=None) -> None:
        if path.is_root():
            raise PermissionError(13, "Permission denied", str(path))
        parent_access, parent_manifest = self._retrieve_entry(path.parent)
        if not is_folder_manifest(parent_manifest):
            raise NotADirectoryError(20, "Not a directory", str(path.parent))

        try:
            item_access = parent_manifest["children"].pop(path.name)
        except KeyError:
            raise FileNotFoundError(2, "No such file or directory", str(path))

        item_manifest = self.get_manifest(item_access)
        if is_folder_manifest(item_manifest):
            if expect == "file":
                raise IsADirectoryError(21, "Is a directory", str(path))
            if item_manifest["children"]:
                raise OSError(39, "Directory not empty", str(path))
        elif expect == "folder":
            raise NotADirectoryError(20, "Not a directory", str(path))

        mark_manifest_modified(parent_manifest)
        self.set_manifest(parent_access, parent_manifest)
        self.event_bus.send("fs.entry.updated", id=parent_access["id"])

    def delete(self, path: Path) -> None:
        return self._delete(path)

    def unlink(self, path: Path) -> None:
        return self._delete(path, expect="file")

    def rmdir(self, path: Path) -> None:
        return self._delete(path, expect="folder")

    def move(self, src: Path, dst: Path) -> None:
        return self._copy(src, dst, True)

    def copy(self, src: Path, dst: Path) -> None:
        return self._copy(src, dst, False)

    def _copy(self, src: Path, dst: Path, delete_src: bool) -> None:
        # The idea here is to consider a manifest never move around the fs
        # (i.e. a given access always points to the same path). This simplify
        # sync notifications handling and avoid ending up with two path
        # (possibly from different workspaces !) pointing to the same manifest
        # which would be weird for user experience.
        # Long story short, when moving/copying manifest, we must recursively
        # copy the manifests and create new accesses for them.

        parent_src = src.parent
        parent_dst = dst.parent

        # No matter what, cannot move or overwrite root
        if src.is_root():
            # Raise FileNotFoundError if parent_dst doesn't exists
            _, parent_dst_manifest = self._retrieve_entry(parent_dst)
            if not is_folder_manifest(parent_dst_manifest):
                raise NotADirectoryError(20, "Not a directory", str(parent_dst))
            else:
                raise PermissionError(13, "Permission denied", str(src), str(dst))
        elif dst.is_root():
            # Raise FileNotFoundError if parent_src doesn't exists
            _, parent_src_manifest = self._retrieve_entry(src.parent)
            if not is_folder_manifest(parent_src_manifest):
                raise NotADirectoryError(20, "Not a directory", str(src.parent))
            else:
                raise PermissionError(13, "Permission denied", str(src), str(dst))

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
            if not is_folder_manifest(parent_manifest):
                raise NotADirectoryError(20, "Not a directory", str(parent_src))

            try:
                dst.relative_to(src)
            except ValueError:
                pass
            else:
                raise OSError(22, "Invalid argument", str(src), None, str(dst))

            try:
                src_access = parent_manifest["children"][src.name]
            except KeyError:
                raise FileNotFoundError(2, "No such file or directory", str(src))

            existing_dst_access = parent_manifest["children"].get(dst.name)
            src_manifest = self.get_manifest(src_access)

            if is_workspace_manifest(src_manifest):
                raise PermissionError(
                    13,
                    "Permission denied (cannot move/copy workpace, must rename it)",
                    str(src),
                    str(dst),
                )

            if existing_dst_access:
                existing_dst_manifest = self.get_manifest(existing_dst_access)
                if is_folder_manifest(src_manifest):
                    if is_file_manifest(existing_dst_manifest):
                        raise NotADirectoryError(20, "Not a directory", str(dst))
                    elif existing_dst_manifest["children"]:
                        raise OSError(39, "Directory not empty", str(dst))
                else:
                    if is_folder_manifest(existing_dst_manifest):
                        raise IsADirectoryError(21, "Is a directory", str(dst))

            moved_access = self._recursive_manifest_copy(src_access, src_manifest)

            parent_manifest["children"][dst.name] = moved_access
            if delete_src:
                del parent_manifest["children"][src.name]

            mark_manifest_modified(parent_manifest)

            self.set_manifest(parent_access, parent_manifest)
            self.event_bus.send("fs.entry.updated", id=parent_access["id"])

        else:
            parent_src_access, parent_src_manifest = self._retrieve_entry(parent_src)
            if not is_folder_manifest(parent_src_manifest):
                raise NotADirectoryError(20, "Not a directory", str(parent_src))

            parent_dst_access, parent_dst_manifest = self._retrieve_entry(parent_dst)
            if not is_folder_manifest(parent_dst_manifest):
                raise NotADirectoryError(20, "Not a directory", str(parent_dst))

            try:
                dst.relative_to(src)
            except ValueError:
                pass
            else:
                raise OSError(22, "Invalid argument", str(src), None, str(dst))

            try:
                src_access = parent_src_manifest["children"][src.name]
            except KeyError:
                raise FileNotFoundError(2, "No such file or directory", str(src))

            existing_dst_access = parent_dst_manifest["children"].get(dst.name)
            src_manifest = self.get_manifest(src_access)

            if is_workspace_manifest(src_manifest):
                raise PermissionError(
                    13,
                    "Permission denied (cannot move/copy workpace, must rename it)",
                    str(src),
                    str(dst),
                )

            if existing_dst_access:
                existing_entry_manifest = self.get_manifest(existing_dst_access)
                if is_folder_manifest(src_manifest):
                    if is_file_manifest(existing_entry_manifest):
                        raise NotADirectoryError(20, "Not a directory", str(dst))
                    elif existing_entry_manifest["children"]:
                        raise OSError(39, "Directory not empty", str(dst))
                else:
                    if is_folder_manifest(existing_entry_manifest):
                        raise IsADirectoryError(21, "Is a directory", str(dst))

            moved_access = self._recursive_manifest_copy(src_access, src_manifest)

            parent_dst_manifest["children"][dst.name] = moved_access
            mark_manifest_modified(parent_dst_manifest)

            self.set_manifest(parent_dst_access, parent_dst_manifest)
            self.event_bus.send("fs.entry.updated", id=parent_dst_access["id"])

            if delete_src:
                del parent_src_manifest["children"][src.name]
                mark_manifest_modified(parent_src_manifest)

                self.set_manifest(parent_src_access, parent_src_manifest)
                self.event_bus.send("fs.entry.updated", id=parent_src_access["id"])

    def _recursive_manifest_copy(self, access, manifest):

        # First, make sure all the manifest have to copy are locally present
        # (otherwise we will have to stop while part of the manifest are
        # already copied, creating garbage and making us doing more retries
        # that strictly needed)

        def _recursive_create_copy_map(access, manifest):
            copy_map = {"access": access, "manifest": manifest}
            manifests_miss = []

            if is_folder_manifest(manifest) or is_workspace_manifest(manifest):
                copy_map["children"] = {}

                for child_name, child_access in manifest["children"].items():
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

            cpy_access = new_access()
            if is_file_manifest(manifest):
                cpy_manifest = new_local_file_manifest(self.local_author)
                cpy_manifest["size"] = manifest["size"]
                cpy_manifest["blocks"] = copy_manifest(manifest["blocks"])
                cpy_manifest["dirty_blocks"] = copy_manifest(manifest["dirty_blocks"])

            elif is_folder_manifest(manifest):
                cpy_manifest = new_local_folder_manifest(self.local_author)
                for child_name in manifest["children"].keys():
                    child_copy_map = copy_map["children"][child_name]
                    new_child_access = _recursive_process_copy_map(child_copy_map)
                    cpy_manifest["children"][child_name] = new_child_access

            else:
                assert is_workspace_manifest(manifest)
                cpy_manifest = new_local_workspace_manifest(self.local_author)

            self.set_manifest(cpy_access, cpy_manifest)
            return cpy_access

        return _recursive_process_copy_map(copy_map)

import pendulum

from parsec.core.local_db import LocalDBMissingEntry
from parsec.core.schemas import dumps_manifest, loads_manifest
from parsec.core.fs.data import (
    is_file_manifest,
    is_folder_manifest,
    new_access,
    new_local_folder_manifest,
    new_local_file_manifest,
)


def mark_manifest_modified(manifest):
    manifest["updated"] = pendulum.now()
    manifest["need_sync"] = True


def normalize_path(path):
    normalized = "/" + "/".join([x for x in path.split("/") if x])
    return normalized


class FSManifestLocalMiss(Exception):
    def __init__(self, access):
        super().__init__(access)
        self.access = access


class LocalFolderFS:
    def __init__(self, device, signal_ns):
        self.local_author = device.id
        self.root_access = device.user_manifest_access
        self._local_db = device.local_db
        self.signal_ns = signal_ns

    def get_local_beacons(self):
        # Only user manifest and workspace manifests have a beacon id
        beacons = []
        try:
            root_manifest = self.get_manifest(self.root_access)
            beacons.append(root_manifest["beacon_id"])
            # Currently workspace can only direct children of the user manifest
            for child_access in root_manifest["children"].values():
                try:
                    child_manifest = self.get_manifest(child_access)
                except FSManifestLocalMiss as exc:
                    continue
                if "beacon_id" in child_manifest:
                    beacons.append(child_manifest["beacon_id"])
        except FSManifestLocalMiss as exc:
            pass
        return beacons

    def dump(self):
        def _recursive_dump(access):
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

    def get_manifest(self, access):
        try:
            raw = self._local_db.get(access)
        except LocalDBMissingEntry as exc:
            raise FSManifestLocalMiss(access) from exc
        return loads_manifest(raw)

    def set_manifest(self, access, manifest):
        raw = dumps_manifest(manifest)
        self._local_db.set(access, raw)

    def mark_outdated_manifest(self, access):
        self._local_db.clear(access)

    def get_beacons(self, path):
        path = normalize_path(path)
        beacons = []

        def _beacons_collector(access, manifest):
            beacon_id = manifest.get("beacon_id")
            if beacon_id:
                beacons.append(beacon_id)

        self._retrieve_entry(path, collector=_beacons_collector)
        return beacons

    def get_entry(self, path):
        return self._retrieve_entry(path)

    def get_entry_path(self, entry_id):
        if entry_id == self.root_access["id"]:
            return "/", self.root_access, self.get_manifest(self.root_access)

        # Brute force style
        def _recursive_search(access, path):
            manifest = self.get_manifest(access)
            if access["id"] == entry_id:
                return path, access, manifest

            if is_folder_manifest(manifest):
                for child_name, child_access in manifest["children"].items():
                    found = _recursive_search(child_access, "%s/%s" % (path, child_name))
                    if found:
                        return found

        found = _recursive_search(self.root_access, "")
        if not found:
            raise FSManifestLocalMiss(entry_id)
        return found

    def _retrieve_entry(self, path, collector=None):
        assert "//" not in path
        assert path.startswith("/")

        def _retrieve_entry_recursive(curr_access, curr_path, hops):
            curr_manifest = self.get_manifest(curr_access)

            if not hops:
                if collector:
                    collector(curr_access, curr_manifest)
                return curr_access, curr_manifest

            if not is_folder_manifest(curr_manifest):
                raise NotADirectoryError(20, "Not a directory", curr_path)

            if collector:
                collector(curr_access, curr_manifest)

            hop, *hops = hops
            try:
                curr_path = f"{curr_path}/{hop}"
                return _retrieve_entry_recursive(curr_manifest["children"][hop], curr_path, hops)

            except KeyError:
                raise FileNotFoundError(2, "No such file or directory", curr_path)

        hops = [hop for hop in path.split("/") if hop]
        return _retrieve_entry_recursive(self.root_access, "", hops)

    def get_sync_strategy(self, path, recursive):
        path = normalize_path(path)
        # Consider root is never a placeholder
        curr_path = "/"
        curr_parent_path = ""
        for hop in path.split("/")[1:]:
            curr_parent_path = curr_path
            curr_path += hop if curr_path.endswith("/") else ("/" + hop)
            _, curr_manifest = self._retrieve_entry(curr_path)
            if curr_manifest["is_placeholder"]:
                sync_path = curr_parent_path
                break

        else:
            return path, recursive

        sync_recursive = recursive
        for hop in reversed(path[len(curr_parent_path) :].split("/")):
            sync_recursive = {hop: sync_recursive}

        return sync_path, sync_recursive

    def get_access(self, path):
        path = normalize_path(path)
        access, _ = self._retrieve_entry(path)
        return access

    def stat(self, path):
        path = normalize_path(path)
        access, manifest = self._retrieve_entry(path)
        if is_file_manifest(manifest):
            return {
                "type": "file",
                "created": manifest["created"],
                "updated": manifest["updated"],
                "base_version": manifest["base_version"],
                "is_placeholder": manifest["is_placeholder"],
                "need_sync": manifest["need_sync"],
                "size": manifest["size"],
            }

        else:
            return {
                "type": "folder",
                "created": manifest["created"],
                "updated": manifest["updated"],
                "base_version": manifest["base_version"],
                "is_placeholder": manifest["is_placeholder"],
                "need_sync": manifest["need_sync"],
                "children": list(sorted(manifest["children"].keys())),
            }

    def touch(self, path):
        path = normalize_path(path)
        parent_path, child_name = path.rsplit("/", 1)
        access, manifest = self._retrieve_entry(parent_path or "/")
        if not is_folder_manifest(manifest):
            raise NotADirectoryError(20, "Not a directory", parent_path)
        if child_name in manifest["children"]:
            raise FileExistsError(17, "File exists", path)

        child_access = new_access()
        child_manifest = new_local_file_manifest(self.local_author)
        manifest["children"][child_name] = child_access
        mark_manifest_modified(manifest)
        self.set_manifest(access, manifest)
        self.set_manifest(child_access, child_manifest)
        self.signal_ns.signal("fs.entry.modified").send("local", id=access["id"])
        self.signal_ns.signal("fs.entry.created").send("local", id=child_access["id"])

    def mkdir(self, path):
        path = normalize_path(path)
        parent_path, child_name = path.rsplit("/", 1)
        access, manifest = self._retrieve_entry(parent_path or "/")
        if not is_folder_manifest(manifest):
            raise NotADirectoryError(20, "Not a directory", parent_path)
        if child_name in manifest["children"]:
            raise FileExistsError(17, "File exists", path)

        child_access = new_access()
        child_manifest = new_local_folder_manifest(self.local_author)
        manifest["children"][child_name] = child_access
        mark_manifest_modified(manifest)
        self.set_manifest(access, manifest)
        self.set_manifest(child_access, child_manifest)
        self.signal_ns.signal("fs.entry.modified").send("local", id=access["id"])
        self.signal_ns.signal("fs.entry.created").send("local", id=child_access["id"])

    def _delete(self, path, expect=None):
        path = normalize_path(path)
        if path == "/":
            raise PermissionError(13, "Permission denied", path)
        parent_path, child_name = path.rsplit("/", 1)
        parent_access, parent_manifest = self._retrieve_entry(parent_path or "/")
        if not is_folder_manifest(parent_manifest):
            raise NotADirectoryError(20, "Not a directory", parent_path)

        try:
            item_access = parent_manifest["children"].pop(child_name)
        except KeyError:
            raise FileNotFoundError(2, "No such file or directory", path)

        item_manifest = self.get_manifest(item_access)
        if is_folder_manifest(item_manifest):
            if expect == "file":
                raise IsADirectoryError(21, "Is a directory", path)
            if item_manifest["children"]:
                raise OSError(39, "Directory not empty", path)
        elif expect == "folder":
            raise NotADirectoryError(20, "Not a directory", path)

        mark_manifest_modified(parent_manifest)
        self.set_manifest(parent_access, parent_manifest)
        self.signal_ns.signal("fs.entry.modified").send("local", id=parent_access["id"])

    def delete(self, path):
        self._delete(path)

    def unlink(self, path):
        self._delete(path, expect="file")

    def rmdir(self, path):
        self._delete(path, expect="folder")

    def move(self, src, dst):
        # TODO: To symplify synchro we currently move the entry into a brand
        # new access. However this is not recursive (i.e. the entry's
        # children will keep there original access)...

        src = normalize_path(src)
        dst = normalize_path(dst)

        parent_src, child_src = src.rsplit("/", 1)
        parent_dst, child_dst = dst.rsplit("/", 1)
        parent_src = parent_src or "/"
        parent_dst = parent_dst or "/"

        if parent_src == parent_dst:
            parent_access, parent_manifest = self._retrieve_entry(parent_src)
            if not is_folder_manifest(parent_manifest):
                raise NotADirectoryError(20, "Not a directory", parent_src)

            if dst.startswith(src + "/"):
                raise OSError(22, "Invalid argument", src, None, dst)

            try:
                entry = parent_manifest["children"].pop(child_src)
            except KeyError:
                raise FileNotFoundError(2, "No such file or directory", src)

            if src == dst:
                return

            existing_entry_access = parent_manifest["children"].get(child_dst)
            src_entry_manifest = self.get_manifest(entry)
            if existing_entry_access:
                existing_entry_manifest = self.get_manifest(existing_entry_access)
                if is_folder_manifest(src_entry_manifest):
                    if is_file_manifest(existing_entry_manifest):
                        raise NotADirectoryError(20, "Not a directory")
                    elif existing_entry_manifest["children"]:
                        raise OSError(39, "Directory not empty")
                else:
                    if is_folder_manifest(existing_entry_manifest):
                        raise IsADirectoryError(21, "Is a directory")

            moved_access = new_access()
            parent_manifest["children"][child_dst] = moved_access
            src_entry_manifest["base_version"] = 0
            src_entry_manifest["is_placeholder"] = True
            src_entry_manifest["need_sync"] = True
            self.set_manifest(moved_access, src_entry_manifest)
            mark_manifest_modified(parent_manifest)

            self.set_manifest(parent_access, parent_manifest)
            self.signal_ns.signal("fs.entry.modified").send("local", id=parent_access["id"])

        else:
            parent_src_access, parent_src_manifest = self._retrieve_entry(parent_src)
            if not is_folder_manifest(parent_src_manifest):
                raise NotADirectoryError(20, "Not a directory", parent_src)

            parent_dst_access, parent_dst_manifest = self._retrieve_entry(parent_dst)
            if not is_folder_manifest(parent_dst_manifest):
                raise NotADirectoryError(20, "Not a directory", parent_dst)

            if dst.startswith(src + "/"):
                raise OSError(22, "Invalid argument", src, None, dst)

            try:
                entry = parent_src_manifest["children"].pop(child_src)
            except KeyError:
                raise FileNotFoundError(2, "No such file or directory", src)

            existing_entry_access = parent_dst_manifest["children"].get(child_dst)
            src_entry_manifest = self.get_manifest(entry)
            if existing_entry_access:
                existing_entry_manifest = self.get_manifest(existing_entry_access)
                if is_folder_manifest(src_entry_manifest):
                    if is_file_manifest(existing_entry_manifest):
                        raise NotADirectoryError(20, "Not a directory")
                    elif existing_entry_manifest["children"]:
                        raise OSError(39, "Directory not empty")
                else:
                    if is_folder_manifest(existing_entry_manifest):
                        raise IsADirectoryError(21, "Is a directory")

            moved_access = new_access()
            parent_dst_manifest["children"][child_dst] = moved_access
            src_entry_manifest["base_version"] = 0
            src_entry_manifest["is_placeholder"] = True
            src_entry_manifest["need_sync"] = True
            self.set_manifest(moved_access, src_entry_manifest)

            mark_manifest_modified(parent_src_manifest)
            mark_manifest_modified(parent_dst_manifest)

            self.set_manifest(parent_src_access, parent_src_manifest)
            self.set_manifest(parent_dst_access, parent_dst_manifest)

            self.signal_ns.signal("fs.entry.modified").send("local", id=parent_src_access["id"])
            self.signal_ns.signal("fs.entry.modified").send("local", id=parent_dst_access["id"])

import pendulum
from uuid import uuid4
from copy import deepcopy
from nacl.hash import sha256

from parsec.utils import generate_sym_key


def normalize_path(path):
    hops = path.split("/")
    for hop in hops:
        # TODO: enable this later
        # if hop == '.' or hop == '..':
        #     raise InvalidPath('Path %r is invalid' % path)
        pass
    return "/".join(hops[:-1]), hop


def is_placeholder_access(access):
    return access["type"] == "placeholder"


def is_file_manifest(manifest):
    return "children" not in manifest


def is_folder_manifest(manifest):
    return "children" in manifest


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


def new_dirty_block_access(offset, size):
    return {"id": uuid4().hex, "key": generate_sym_key(), "offset": offset, "size": size}


def new_block_access(offset, data, id=None):
    return {
        "id": id,
        "key": generate_sym_key(),
        "offset": offset,
        "size": len(data),
        "digest": sha256(bytes(data)),
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


def copy_manifest(manifest):
    return deepcopy(manifest)


def convert_to_remote_manifest(local_manifest):
    manifest = copy_manifest(local_manifest)
    del manifest["need_sync"]
    manifest["version"] = manifest.pop("base_version") + 1
    local_type = manifest["type"]
    if local_type == "local_file_manifest":
        manifest["type"] = "file_manifest"
        del manifest["dirty_blocks"]
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


def convert_to_local_manifest(manifest, as_need_sync=False):
    local_manifest = copy_manifest(manifest)
    local_manifest["base_version"] = local_manifest.pop("version")
    if as_need_sync:
        local_manifest["need_sync"] = True
        local_manifest["base_version"] -= 1
        assert local_manifest["base_version"] >= 0
    else:
        local_manifest["need_sync"] = False
    remote_type = manifest["type"]
    if remote_type == "file_manifest":
        local_manifest["type"] = "local_file_manifest"
        local_manifest["dirty_blocks"] = []
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

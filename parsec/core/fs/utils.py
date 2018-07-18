from uuid import uuid4
from hashlib import sha256
import pendulum
import nacl.secret
import nacl.utils


def _generate_secret_key():
    return nacl.utils.random(nacl.secret.SecretBox.KEY_SIZE)


def new_access():
    id = uuid4().hex
    return {"id": id, "rts": uuid4().hex, "wts": uuid4().hex, "key": _generate_secret_key()}


def new_block_access(block, offset):
    return {
        "id": uuid4().hex,
        "key": _generate_secret_key(),
        "offset": offset,
        "size": len(block),
        "digest": sha256(block).hexdigest(),
    }


def new_local_user_manifest(author):
    manifest = new_local_workspace_manifest(author)
    manifest["last_processed_message"] = 0
    manifest["type"] = "local_user_manifest"
    return manifest


def new_local_workspace_manifest(author):
    assert "@" in author
    now = pendulum.now()

    return {
        "type": "local_workspace_manifest",
        "need_sync": True,
        "is_placeholder": True,
        "author": author,
        "beacon_id": uuid4().hex,
        "created": now,
        "updated": now,
        "base_version": 0,
        "children": {},
    }


def new_local_folder_manifest(author):
    assert "@" in author
    now = pendulum.now()

    return {
        "type": "local_folder_manifest",
        "need_sync": True,
        "is_placeholder": True,
        "author": author,
        "created": now,
        "updated": now,
        "base_version": 0,
        "children": {},
    }


def new_local_file_manifest(author):
    assert "@" in author
    now = pendulum.now()

    return {
        "type": "local_file_manifest",
        "need_sync": True,
        "is_placeholder": True,
        "author": author,
        "created": now,
        "updated": now,
        "base_version": 0,
        "size": 0,
        "blocks": [],
        "dirty_blocks": [],
    }


def is_placeholder_manifest(manifest):
    return manifest["is_placeholder"]


def is_file_manifest(manifest):
    return manifest["type"].endswith("file_manifest")


def is_folder_manifest(manifest):
    for t in ("folder_manifest", "workspace_manifest", "user_manifest"):
        if manifest["type"].endswith(t):
            return True
    return False


def is_workspace_manifest(manifest):
    return manifest["type"].endswith("workspace_manifest")


def remote_to_local_manifest(manifest):
    local_manifest = manifest.copy()
    local_manifest["base_version"] = manifest["version"]
    local_manifest["need_sync"] = False
    local_manifest["is_placeholder"] = False
    local_manifest["type"] = "local_" + manifest["type"]
    if is_file_manifest(manifest):
        local_manifest["dirty_blocks"] = []
    return local_manifest


def local_to_remote_manifest(manifest):
    remote_manifest = manifest.copy()
    remote_manifest["version"] = remote_manifest.pop("base_version")
    del remote_manifest["need_sync"]
    del remote_manifest["is_placeholder"]
    remote_manifest["type"] = manifest["type"][len("local_") :]
    if is_file_manifest(manifest):
        del remote_manifest["dirty_blocks"]
    return remote_manifest

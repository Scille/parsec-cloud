from uuid import uuid4
from hashlib import sha256
import pendulum

from parsec.types import DeviceID
from parsec.crypto import generate_secret_key
from parsec.core.fs.types import (
    LocalUserManifest,
    LocalWorkspaceManifest,
    LocalFolderManifest,
    LocalFileManifest,
    LocalManifest,
    RemoteUserManifest,
    RemoteWorkspaceManifest,
    RemoteFolderManifest,
    RemoteFileManifest,
    RemoteManifest,
    Manifest,
    Access,
    BlockAccess,
)


def copy_manifest(manifest: LocalManifest):
    """
    Basically an optimized version of deepcopy
    """

    def _recursive_copy(old):
        try:
            return {
                k: [_recursive_copy(e) for e in v]
                if isinstance(v, (tuple, list))
                else _recursive_copy(v)
                if isinstance(v, dict)
                else v
                for k, v in old.items()
            }
        except AttributeError:
            # Occurs when dealing with list of strings
            return old

    return _recursive_copy(manifest)


def new_access() -> Access:
    id = uuid4()
    return Access({"id": id, "rts": uuid4().hex, "wts": uuid4().hex, "key": generate_secret_key()})


def new_block_access(block: bytes, offset: int) -> BlockAccess:
    return BlockAccess(
        {
            "id": uuid4(),
            "key": generate_secret_key(),
            "offset": offset,
            "size": len(block),
            "digest": sha256(block).hexdigest(),
        }
    )


def new_local_user_manifest(author: DeviceID) -> LocalUserManifest:
    now = pendulum.now()

    return LocalUserManifest(
        {
            "format": 1,
            "type": "local_user_manifest",
            "need_sync": True,
            "is_placeholder": True,
            "author": author,
            "created": now,
            "updated": now,
            "base_version": 0,
            "children": {},
            "last_processed_message": 0,
        }
    )


def new_local_workspace_manifest(author: DeviceID) -> LocalWorkspaceManifest:
    now = pendulum.now()

    return LocalWorkspaceManifest(
        {
            "format": 1,
            "type": "local_workspace_manifest",
            "need_sync": True,
            "is_placeholder": True,
            "author": author,
            "created": now,
            "updated": now,
            "base_version": 0,
            "children": {},
            "creator": author.user_id,
            "participants": [author.user_id],
        }
    )


def new_local_folder_manifest(author: DeviceID) -> LocalFolderManifest:
    now = pendulum.now()

    return LocalFolderManifest(
        {
            "format": 1,
            "type": "local_folder_manifest",
            "need_sync": True,
            "is_placeholder": True,
            "author": author,
            "created": now,
            "updated": now,
            "base_version": 0,
            "children": {},
        }
    )


def new_local_file_manifest(author) -> LocalFileManifest:
    assert "@" in author
    now = pendulum.now()

    return LocalFileManifest(
        {
            "format": 1,
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
    )


def is_placeholder_manifest(manifest: LocalManifest) -> bool:
    return manifest["is_placeholder"]


def is_file_manifest(manifest: Manifest) -> bool:
    return manifest["type"].endswith("file_manifest")


def is_folder_manifest(manifest: Manifest) -> bool:
    for t in ("folder_manifest", "workspace_manifest", "user_manifest"):
        if manifest["type"].endswith(t):
            return True
    return False


def is_folderish_manifest(manifest: Manifest) -> bool:
    for t in ("folder_manifest", "workspace_manifest", "user_manifest"):
        if manifest["type"].endswith(t):
            return True
    return False


def is_workspace_manifest(manifest: Manifest) -> bool:
    return manifest["type"].endswith("workspace_manifest")


def is_user_manifest(manifest: Manifest) -> bool:
    return manifest["type"].endswith("user_manifest")


def remote_to_local_manifest(manifest: RemoteManifest) -> LocalManifest:
    local_manifest = manifest.copy()
    local_manifest["base_version"] = local_manifest.pop("version")
    local_manifest["need_sync"] = False
    local_manifest["is_placeholder"] = False

    manifest_type = manifest["type"]
    if manifest_type == "user_manifest":
        local_manifest["type"] = "local_user_manifest"
        return LocalUserManifest(local_manifest)

    elif manifest_type == "workspace_manifest":
        local_manifest["type"] = "local_workspace_manifest"
        return LocalWorkspaceManifest(local_manifest)

    elif manifest_type == "folder_manifest":
        local_manifest["type"] = "local_folder_manifest"
        return LocalFolderManifest(local_manifest)

    elif manifest_type == "file_manifest":
        local_manifest["type"] = "local_file_manifest"
        local_manifest["dirty_blocks"] = []
        return LocalFileManifest(local_manifest)

    else:
        raise RuntimeError(f"Unknown manifest type: {manifest_type}")


def local_to_remote_manifest(manifest: LocalManifest) -> RemoteManifest:
    remote_manifest = manifest.copy()
    remote_manifest["version"] = remote_manifest.pop("base_version")
    del remote_manifest["need_sync"]
    del remote_manifest["is_placeholder"]
    remote_manifest["type"] = manifest["type"][len("local_") :]

    manifest_type = manifest["type"]
    if manifest_type == "local_user_manifest":
        remote_manifest["type"] = "user_manifest"
        return RemoteUserManifest(remote_manifest)

    elif manifest_type == "local_workspace_manifest":
        remote_manifest["type"] = "workspace_manifest"
        return RemoteWorkspaceManifest(remote_manifest)

    elif manifest_type == "local_folder_manifest":
        remote_manifest["type"] = "folder_manifest"
        return RemoteFolderManifest(remote_manifest)

    elif manifest_type == "local_file_manifest":
        remote_manifest["type"] = "file_manifest"
        del remote_manifest["dirty_blocks"]
        return RemoteFileManifest(remote_manifest)

    else:
        raise RuntimeError(f"Unknown manifest type: {manifest_type}")

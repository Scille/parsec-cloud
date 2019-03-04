# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from pendulum import Pendulum
import random


# TODO: shouldn't use core.fs.types.Acces here
# from parsec.core.fs.types import Access
Access = None  # TODO: hack to fix recursive import


# TODO: should be in config.py
DEFAULT_MAX_CACHE_SIZE = 128 * 1024 * 1024
DEFAULT_BLOCK_SIZE = 2 ** 16


class MemoryCache:
    def __init__(self, max_cache_size: int = DEFAULT_MAX_CACHE_SIZE):
        self.clean_blocks = {}
        self.clean_manifests = {}
        self.remote_blocks = {}
        self.remote_manifests = {}
        self.users = {}
        self.max_cache_size = max_cache_size

    @property
    def block_limit(self):
        return self.max_cache_size // DEFAULT_BLOCK_SIZE

    def get_nb_blocks(self, clean: bool):
        cache = self.clean_blocks if clean else self.remote_blocks
        return len(cache)

    # User operations

    def get_user(self, access: Access):
        if str(access.id) not in self.users:
            return
        user = self.users[str(access.id)]
        return [str(access.id), user["blob"], user["created_on"]]

    def set_user(self, access: Access, ciphered: bytes):
        self.users[str(access.id)] = {"blob": ciphered, "created_on": str(Pendulum.now())}

    # Manifest operations

    def get_manifest(self, clean: bool, access: Access):
        manifests = self.clean_manifests if clean else self.remote_manifests
        try:
            ciphered = manifests[str(access.id)]
        except KeyError:
            return
        return [str(access.id), ciphered]

    def set_manifest(self, clean: bool, access: Access, ciphered: bytes):
        manifests = self.clean_manifests if clean else self.remote_manifests
        manifests[str(access.id)] = ciphered

    def clear_manifest(self, clean: bool, access: Access):
        manifests = self.clean_manifests if clean else self.remote_manifests
        try:
            del manifests[str(access.id)]
        except KeyError:
            pass

    # Block operations

    def get_block(self, clean: bool, access: Access):
        blocks = self.clean_blocks if clean else self.remote_blocks
        try:
            return [str(access.id), blocks[str(access.id)]]
        except KeyError:
            pass

    def set_block(self, clean: bool, access: Access, ciphered: bytes):
        blocks = self.clean_blocks if clean else self.remote_blocks
        if self.get_nb_blocks(clean) >= self.block_limit:
            deleted = random.choice(list(blocks.keys()))
            blocks.pop(deleted)
        blocks[str(access.id)] = ciphered

    def clear_block(self, clean: bool, access: Access):
        blocks = self.clean_blocks if clean else self.remote_blocks
        try:
            del blocks[str(access.id)]
        except KeyError:
            pass

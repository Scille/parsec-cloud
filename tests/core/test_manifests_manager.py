# import pytest
# from trio.testing import trio_test
# from unittest.mock import Mock

# from parsec.core.fs import FS, BaseFile, BaseFolder, FSInvalidPath
# from parsec.core.manifest import (
#     LocalFileManifest, LocalFolderManifest, LocalUserManifest,
#     PlaceHolderEntry, SyncedEntry
# )
# from parsec.core.manifests_manager import ManifestsManager, ManifestDecryptionError

# from tests.common import alice


# def _manifest_manager_factory():
#     local_storage = Mock()
#     backend_storage = Mock()

#     async def fetch_manifest(*args, **kwargs):
#         backend_storage.fetch_manifest(*args, **kwargs)

#     backend_storage.a_fetch_manifest = fetch_manifest

#     return ManifestsManager(alice, local_storage, backend_storage)


# def test_manifest_manager_fetch_user_manifest_empty():
#     manifest_manager = _manifest_manager_factory()
#     manifest_manager._local_storage.fetch_user_manifest.return_value = None
#     user_manifest = manifest_manager.fetch_user_manifest()
#     assert isinstance(user_manifest, LocalUserManifest)


# def test_manifest_manager_fetch_user_manifest_invalid():
#     manifest_manager = _manifest_manager_factory()
#     manifest_manager._local_storage.fetch_user_manifest.return_value = b'fooo'
#     with pytest.raises(ManifestDecryptionError):
#         manifest_manager.fetch_user_manifest()


# # def test_manifest_manager_fetch_user_manifest_valid():
# #     manifest_manager = _manifest_manager_factory()
# #     dumped = dump_manifest(LocalUserManifest())
# #     manifest_manager._local_storage.fetch_user_manifest.return_value = dumped
# #     user_manifest = manifest_manager.fetch_user_manifest()
# #     assert isinstance(user_manifest, LocalUserManifest)


# def test_manifest_manager_flush_user_manifest_valid():
#     manifest_manager = _manifest_manager_factory()
#     manifest = LocalUserManifest()
#     manifest_manager.flush_user_manifest(manifest)
#     assert len(manifest_manager._local_storage.flush_user_manifest.call_args_list) == 1
#     dumped = manifest_manager._local_storage.flush_user_manifest.call_args_list[0][0][0]
#     assert isinstance(dumped, bytes)

#     # Reload the manifest
#     local_storage = Mock()
#     local_storage.fetch_user_manifest.return_value = dumped
#     backend_storage = Mock()
#     manifest_manager2 = ManifestsManager(alice, local_storage, backend_storage)
#     manifest2 = manifest_manager2.fetch_user_manifest()

#     assert manifest == manifest2

#     # def flush_manifest(self, entry, manifest):

#     # async def fetch_manifest(self, entry):

#     # def create_placeholder_file(self):

#     # def create_placeholder_folder(self):

from trio.testing import trio_test
from unittest.mock import patch, Mock

from parsec.core.local_fs import LocalFS
from parsec.core.local_user_manifest import LocalUserManifest

from tests.common import alice, mocked_local_storage_cls_factory, populate_local_storage_cls


@trio_test
async def test_init_local_fs():
    mocked_local_storage_cls = mocked_local_storage_cls_factory()
    populate_local_storage_cls(alice, mocked_local_storage_cls)
    with patch('parsec.core.local_fs.LocalStorage', mocked_local_storage_cls):
        fs = LocalFS(alice, 'tcp://127.0.0.1:6777')
        await fs.init(Mock())
        assert isinstance(fs.local_user_manifest, LocalUserManifest)
        assert fs.local_user_manifest.is_dirty
        assert sorted(fs.local_user_manifest.tree['children']['dir']['children'].keys()) == [
            'modified.txt', 'new.txt', 'non_local.txt', 'up_to_date.txt']


# def test_load_local_user_manifest(alice, alice_local_storage_cls):
#     local_storage_cls = mocked_local_storage_cls_factory()
#     populate_mocked_local_storage_for_alice(local_storage_cls)
#     local_storage = local_storage_cls()

#     # from parsec.local_user_manifest import load_local_user_manifest, LocalUserManifest
#     # raw_dum = local_storage.get_local_user_manifest()
#     # alice_privkey = TEST_USERS['alice@test']
#     # dum = load_local_user_manifest(alice_privkey, raw_dum)
#     # assert isinstance(dum, LocalUserManifest)
#     # assert dum.is_dirty


# class TestFileManager:
#     def test_base(self):
#         local_storage = mocked_local_storage_cls_factory()
#         # file_manager = FileManager(local_storage)

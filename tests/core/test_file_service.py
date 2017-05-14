from base64 import encodebytes
from os import path

from freezegun import freeze_time
import gnupg
import pytest

from parsec.core import (CryptoService, FileService,
                         IdentityService, GNUPGPubKeysService, MetaBlockService,
                         MockedBackendAPIService, MockedBlockService, MockedCacheService,
                         ShareService, UserManifestService)
from parsec.server import BaseServer


GNUPG_HOME = path.dirname(path.abspath(__file__)) + '/../gnupg_env'


@pytest.fixture
def user_manifest_svc():
    return UserManifestService()


@pytest.fixture
def file_svc(event_loop, user_manifest_svc):
    identity = '81DBCF6EB9C8B2965A65ACE5520D903047D69DC9'
    service = FileService()
    block_service = MetaBlockService(backends=[MockedBlockService, MockedBlockService])
    crypto_service = CryptoService()
    crypto_service.gnupg = gnupg.GPG(homedir=GNUPG_HOME + '/secret_env')
    identity_service = IdentityService()
    server = BaseServer()
    server.register_service(service)
    server.register_service(block_service)
    server.register_service(crypto_service)
    server.register_service(identity_service)
    server.register_service(user_manifest_svc)
    server.register_service(GNUPGPubKeysService())
    server.register_service(MockedBackendAPIService())
    server.register_service(MockedCacheService())
    server.register_service(ShareService())
    event_loop.run_until_complete(server.bootstrap_services())
    event_loop.run_until_complete(identity_service.load_identity(identity=identity))
    event_loop.run_until_complete(user_manifest_svc.load_user_manifest())
    yield service
    event_loop.run_until_complete(server.teardown_services())


class TestFileService:

    @pytest.mark.asyncio
    async def test_create_file(self, file_svc):
        ret = await file_svc.dispatch_msg({'cmd': 'file_create'})
        assert ret['status'] == 'ok'
        # assert ret['file']['id'] # TODO check id

    @pytest.mark.asyncio
    async def test_file_read(self, file_svc, user_manifest_svc):
        file_vlob = await user_manifest_svc.create_file('/test')
        id = file_vlob['id']
        # Empty file
        ret = await file_svc.dispatch_msg({'cmd': 'file_read', 'id': id})
        assert ret == {'status': 'ok', 'content': '', 'version': 1}
        # Not empty file
        content = encodebytes('foo'.encode()).decode()
        await file_svc.write(id, 2, content)
        ret = await file_svc.dispatch_msg({'cmd': 'file_read', 'id': id})
        assert ret == {'status': 'ok', 'content': content, 'version': 2}
        # Unknown file
        ret = await file_svc.dispatch_msg({'cmd': 'file_read',
                                           'id': '5ea26ae2479c49f58ede248cdca1a3ca'})
        assert ret == {'status': 'not_found', 'label': 'Vlob not found.'}

    @pytest.mark.asyncio
    async def test_file_write(self, file_svc, user_manifest_svc):
        file_vlob = await user_manifest_svc.create_file('/test')
        id = file_vlob['id']
        # Check with empty and not empty file
        for version, content in enumerate(('this is v2', 'this is v3'), 2):
            encoded_content = encodebytes(content.encode()).decode()
            ret = await file_svc.dispatch_msg({'cmd': 'file_write',
                                               'id': id,
                                               'version': version,
                                               'content': encoded_content})
            assert ret == {'status': 'ok'}
            file = await file_svc.read(id)
            assert file == {'content': encoded_content, 'version': version}
        # Unknown file
        content = encodebytes('foo'.encode()).decode()
        ret = await file_svc.dispatch_msg({'cmd': 'file_write',
                                           'id': '1234',
                                           'version': 1,
                                           'content': content})
        assert ret == {'status': 'not_found', 'label': 'Vlob not found.'}

    @pytest.mark.asyncio
    async def test_file_stat(self, file_svc, user_manifest_svc):
        # Good file
        with freeze_time('2012-01-01') as frozen_datetime:
            file_vlob = await user_manifest_svc.create_file('/test')
            id = file_vlob['id']
            ret = await file_svc.dispatch_msg({'cmd': 'file_stat', 'id': id})
            ctime = frozen_datetime().timestamp()
            assert ret == {'status': 'ok',
                           'id': id,
                           'ctime': ctime,
                           'mtime': ctime,
                           'atime': ctime,
                           'size': 0,
                           'version': 1}
            frozen_datetime.tick()
            mtime = frozen_datetime().timestamp()
            content = encodebytes('foo'.encode()).decode()
            await file_svc.write(id, 2, content)
            ret = await file_svc.dispatch_msg({'cmd': 'file_stat', 'id': id})
            assert ret == {'status': 'ok',
                           'id': id,
                           'ctime': mtime,
                           'mtime': mtime,
                           'atime': mtime,
                           'size': 3,
                           'version': 2}
            frozen_datetime.tick()
            await file_svc.read(id)  # TODO useless if atime is not modified
            ret = await file_svc.dispatch_msg({'cmd': 'file_stat', 'id': id})
            assert ret == {'status': 'ok',
                           'id': id,
                           'ctime': mtime,
                           'mtime': mtime,
                           'atime': mtime,
                           'size': 3,
                           'version': 2}
        # Unknown file
        ret = await file_svc.dispatch_msg({'cmd': 'file_stat', 'id': '1234'})
        assert ret == {'status': 'not_found', 'label': 'Vlob not found.'}

    @pytest.mark.asyncio
    async def test_history(self, file_svc, user_manifest_svc):
        with freeze_time('2012-01-01') as frozen_datetime:
            file_vlob = await user_manifest_svc.create_file('/test')
            id = file_vlob['id']
            original_time = frozen_datetime().timestamp()
            for version, content in enumerate(('this is v2', 'this is v3...'), 2):
                frozen_datetime.tick()
                encoded_content = encodebytes(content.encode()).decode()
                await file_svc.write(id, version, encoded_content)
        # Full history
        ret = await file_svc.dispatch_msg({'cmd': 'file_history', 'id': id})
        assert ret == {
            'status': 'ok',
            'history': [
                {
                    'version': 1,
                    'ctime': original_time,
                    'mtime': original_time,
                    'atime': original_time,
                    'size': 0
                },
                {
                    'version': 2,
                    'ctime': original_time + 1,
                    'mtime': original_time + 1,
                    'atime': original_time + 1,
                    'size': 10
                },
                {
                    'version': 3,
                    'ctime': original_time + 2,
                    'mtime': original_time + 2,
                    'atime': original_time + 2,
                    'size': 13
                }
            ]
        }
        # Partial history starting at version 2
        ret = await file_svc.dispatch_msg({'cmd': 'file_history', 'id': id, 'first_version': 2})
        assert ret == {
            'status': 'ok',
            'history': [
                {
                    'version': 2,
                    'ctime': original_time + 1,
                    'mtime': original_time + 1,
                    'atime': original_time + 1,
                    'size': 10
                },
                {
                    'version': 3,
                    'ctime': original_time + 2,
                    'mtime': original_time + 2,
                    'atime': original_time + 2,
                    'size': 13
                }
            ]
        }
        # Partial history ending at version 2
        ret = await file_svc.dispatch_msg({'cmd': 'file_history', 'id': id, 'last_version': 2})
        assert ret == {
            'status': 'ok',
            'history': [
                {
                    'version': 1,
                    'ctime': original_time,
                    'mtime': original_time,
                    'atime': original_time,
                    'size': 0
                },
                {
                    'version': 2,
                    'ctime': original_time + 1,
                    'mtime': original_time + 1,
                    'atime': original_time + 1,
                    'size': 10
                }
            ]
        }
        # First version = last version
        ret = await file_svc.dispatch_msg({'cmd': 'file_history',
                                           'id': id,
                                           'first_version': 2,
                                           'last_version': 2})
        assert ret == {
            'status': 'ok',
            'history': [
                {
                    'version': 2,
                    'ctime': original_time + 1,
                    'mtime': original_time + 1,
                    'atime': original_time + 1,
                    'size': 10
                }
            ]
        }
        # First version > last version
        ret = await file_svc.dispatch_msg({'cmd': 'file_history',
                                           'id': id,
                                           'first_version': 3,
                                           'last_version': 2})
        assert ret == {'status': 'bad_versions',
                       'label': 'First version number higher than the second one.'}

    @pytest.mark.asyncio
    async def test_restore(self, file_svc, user_manifest_svc):
        encoded_content = encodebytes('initial'.encode()).decode()
        file_vlob = await user_manifest_svc.create_file('/test', encoded_content)
        id = file_vlob['id']
        # Restore file with version 1
        file = await file_svc.read(id)
        assert file == {'content': encoded_content, 'version': 1}
        ret = await file_svc.dispatch_msg({'cmd': 'file_restore', 'id': id})
        file = await file_svc.read(id)
        assert file == {'content': encoded_content, 'version': 1}
        # Restore previous version
        for version, content in enumerate(('this is v2', 'this is v3', 'this is v4'), 2):
            encoded_content = encodebytes(content.encode()).decode()
            await file_svc.write(id, version, encoded_content)
        file = await file_svc.read(id)
        encoded_content = encodebytes('this is v4'.encode()).decode()
        assert file == {'content': encoded_content, 'version': 4}
        ret = await file_svc.dispatch_msg({'cmd': 'file_restore', 'id': id})
        file = await file_svc.read(id)
        encoded_content = encodebytes('this is v3'.encode()).decode()
        assert file == {'content': encoded_content, 'version': 5}
        # Restore old version
        ret = await file_svc.dispatch_msg({'cmd': 'file_restore', 'id': id, 'version': 2})
        file = await file_svc.read(id)
        encoded_content = encodebytes('this is v2'.encode()).decode()
        assert file == {'content': encoded_content, 'version': 6}
        # Bad version
        for version in [0, 10]:
            ret = await file_svc.dispatch_msg({'cmd': 'file_restore', 'id': id, 'version': version})
            assert ret == {'status': 'bad_version', 'label': 'Bad version number.'}

    @pytest.mark.asyncio
    async def test_reencrypt(self, file_svc, user_manifest_svc):
        encoded_content_initial = encodebytes('initial'.encode()).decode()
        encoded_content_final = encodebytes('final'.encode()).decode()
        old_vlob = await user_manifest_svc.create_file('/foo', encoded_content_initial)
        ret = await file_svc.dispatch_msg({'cmd': 'file_reencrypt', 'id': old_vlob['id']})
        assert ret['status'] == 'ok'
        del ret['status']
        new_vlob = ret
        for property in old_vlob.keys():
            assert old_vlob[property] != new_vlob[property]
        await user_manifest_svc.import_file_vlob('/bar', new_vlob)
        await file_svc.write(new_vlob['id'], 2, encoded_content_final)
        file = await file_svc.read(old_vlob['id'])
        assert file == {'content': encoded_content_initial, 'version': 1}
        file = await file_svc.read(new_vlob['id'])
        assert file == {'content': encoded_content_final, 'version': 2}

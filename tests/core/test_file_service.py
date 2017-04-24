from base64 import encodebytes
from os import path

from freezegun import freeze_time
import gnupg
import pytest

from parsec.backend import MetaBlockService, MockedBlockService
from parsec.core import (MockedBackendAPIService, CryptoService, FileService, IdentityService,
                         GNUPGPubKeysService, ShareService, UserManifestService)
from parsec.server import BaseServer


GNUPG_HOME = path.dirname(path.abspath(__file__)) + '/../gnupg_env'


@pytest.fixture
def user_manifest_svc():
    return UserManifestService()


@pytest.fixture
def file_svc(event_loop, user_manifest_svc):
    identity = '81DBCF6EB9C8B2965A65ACE5520D903047D69DC9'
    service = FileService()
    backend_api_service = MockedBackendAPIService()
    backend_api_service._block_service = MetaBlockService(backends=[MockedBlockService])
    crypto_service = CryptoService()
    crypto_service.gnupg = gnupg.GPG(homedir=GNUPG_HOME + '/secret_env')
    identity_service = IdentityService()
    server = BaseServer()
    server.register_service(service)
    server.register_service(backend_api_service)
    server.register_service(crypto_service)
    server.register_service(identity_service)
    server.register_service(user_manifest_svc)
    server.register_service(GNUPGPubKeysService())
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
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_create_file',
                                                    'path': '/test'})
        assert ret['status'] == 'ok'
        id = ret['id']
        # Empty file
        ret = await file_svc.dispatch_msg({'cmd': 'file_read', 'id': id})
        assert ret == {'status': 'ok', 'content': '', 'version': 1}
        # Not empty file
        content = encodebytes('foo'.encode()).decode()
        ret = await file_svc.dispatch_msg({'cmd': 'file_write',
                                           'id': id,
                                           'version': 2,
                                           'content': content})
        ret = await file_svc.dispatch_msg({'cmd': 'file_read', 'id': id})
        assert ret == {'status': 'ok', 'content': content, 'version': 2}
        # Unknown file
        ret = await file_svc.dispatch_msg({'cmd': 'file_read',
                                           'id': '5ea26ae2479c49f58ede248cdca1a3ca'})
        assert ret == {'status': 'not_found', 'label': 'Vlob not found.'}

    @pytest.mark.asyncio
    async def test_file_write(self, file_svc, user_manifest_svc):
        ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_create_file',
                                                    'path': '/test'})
        assert ret['status'] == 'ok'
        id = ret['id']
        # Check with empty and not empty file
        for version, content in enumerate(('this is v2', 'this is v3'), 2):
            encoded_content = encodebytes(content.encode()).decode()
            ret = await file_svc.dispatch_msg({'cmd': 'file_write',
                                               'id': id,
                                               'version': version,
                                               'content': encoded_content})
            assert ret == {'status': 'ok'}
            ret = await file_svc.dispatch_msg({'cmd': 'file_read', 'id': id})
            assert ret == {'status': 'ok',
                           'content': encoded_content,
                           'version': version}
        # Unknown file
        content = encodebytes('foo'.encode()).decode()
        ret = await file_svc.dispatch_msg({'cmd': 'file_write',
                                           'id': '1234',
                                           'version': 1,
                                           'content': content})
        assert ret == {'status': 'not_found', 'label': 'Vlob not found.'}

    @pytest.mark.asyncio
    @freeze_time("2012-01-01")
    async def test_stat_file(self, file_svc, user_manifest_svc):
            # Good file
            with freeze_time('2012-01-01') as frozen_datetime:
                ret = await user_manifest_svc.dispatch_msg({'cmd': 'user_manifest_create_file',
                                                            'path': '/test'})
                assert ret['status'] == 'ok'
                id = ret['id']
                ret = await file_svc.dispatch_msg({'cmd': 'file_stat', 'id': id})
                ctime = frozen_datetime().timestamp()
                assert ret == {'status': 'ok',
                               'id': id,
                               'ctime': ctime,
                               'mtime': ctime,
                               'atime': ctime,
                               'size': 0}
                frozen_datetime.tick()
                mtime = frozen_datetime().timestamp()
                content = encodebytes('foo'.encode()).decode()
                ret = await file_svc.dispatch_msg({'cmd': 'file_write',
                                                   'id': id,
                                                   'version': 2,
                                                   'content': content})
                ret = await file_svc.dispatch_msg({'cmd': 'file_stat', 'id': id})
                assert ret == {'status': 'ok',
                               'id': id,
                               'ctime': mtime,
                               'mtime': mtime,
                               'atime': mtime,
                               'size': 3}
                frozen_datetime.tick()
                atime = frozen_datetime().timestamp()
                ret = await file_svc.dispatch_msg({'cmd': 'file_read', 'id': id})
                ret = await file_svc.dispatch_msg({'cmd': 'file_stat', 'id': id})
                assert ret == {'status': 'ok',
                               'id': id,
                               'ctime': mtime,
                               'mtime': mtime,
                               'atime': atime,
                               'size': 3}
            # Unknown file
            ret = await file_svc.dispatch_msg({'cmd': 'file_stat', 'id': '1234'})
            assert ret == {'status': 'not_found', 'label': 'Vlob not found.'}

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_history(self, file_svc):
        raise NotImplementedError()

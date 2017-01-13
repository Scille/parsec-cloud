import pytest
import tempfile

from parsec.volume import VolumeServiceInMemoryMock, LocalVolumeClient
from parsec.crypto.crypto import CryptoEngineService
from parsec.crypto import LocalCryptoClient
from parsec.crypto.crypto_mock import MockAsymCipher, MockSymCipher
from parsec.vfs import VFSService, VFSServiceMock
from parsec.vfs.vfs_pb2 import Request, Response, Stat


class BaseTestVFSService:

    # Helpers

    def mkdir(self, path):
        ret = self.service.dispatch_msg(Request(type=Request.MAKE_DIR, path=path))
        assert ret.status_code == Response.OK

    def mkfile(self, path, content=b''):
        ret = self.service.dispatch_msg(Request(
            type=Request.CREATE_FILE, path=path, content=content))
        assert ret.status_code == Response.OK

    def rmfile(self, path):
        ret = self.service.dispatch_msg(Request(type=Request.DELETE_FILE, path=path))
        assert ret.status_code == Response.OK

    def rmdir(self, path):
        ret = self.service.dispatch_msg(Request(type=Request.REMOVE_DIR, path=path))
        assert ret.status_code == Response.OK

    # Tests

    def test_create_dir(self):
        msg = Request(type=Request.MAKE_DIR, path='/test_dir')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK
        # Make sure target has been created
        msg = Request(type=Request.MAKE_DIR, path='/test_dir')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.BAD_REQUEST
        assert ret.error_msg == 'Target already exists'

    def test_remove_dir(self):
        self.mkdir('/test_dir')
        msg = Request(type=Request.REMOVE_DIR, path='/test_dir')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK
        # Cannot remove already destroyed folder
        msg = Request(type=Request.REMOVE_DIR, path='/test_dir')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.FILE_NOT_FOUND

    def test_cant_remove_root_dir(self):
        msg = Request(type=Request.REMOVE_DIR, path='/')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.BAD_REQUEST

    def test_remove_not_empty_dir(self):
        self.mkdir('/test_dir')
        self.mkfile('/test_dir/test')
        msg = Request(type=Request.REMOVE_DIR, path='/test_dir')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.BAD_REQUEST
        # Delete file so we can remove the folder
        ret = self.service.dispatch_msg(Request(type=Request.DELETE_FILE, path='/test_dir/test'))
        assert ret.status_code == Response.OK
        msg = Request(type=Request.REMOVE_DIR, path='/test_dir')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK

    def test_list_dir(self):
        # Create folders
        self.mkdir('/countries')
        self.mkdir('/countries/France')
        self.mkdir('/countries/France/cities')
        self.mkdir('/countries/Belgium')
        self.mkdir('/countries/Belgium/cities')
        # Create multiple files
        self.mkfile('/.root')
        self.mkfile('/countries/index', b'1 - Belgium\n2 - France')
        self.mkfile('/countries/France/info', b'good=Wine, bad=Beer')
        self.mkfile('/countries/Belgium/info', b'good=Beer, bad=Wine')

        # Finally do some lookup
        def assert_ls(path, expected):
            msg = Request(type=Request.LIST_DIR, path=path)
            ret = self.service.dispatch_msg(msg)
            assert isinstance(ret, Response)
            assert ret.status_code == Response.OK
            assert list(sorted(ret.list_dir)) == list(sorted(expected))

        assert_ls('/', ['.root', 'countries'])
        assert_ls('/countries', ['index', 'Belgium', 'France'])

    def test_create_file(self):
        msg = Request(type=Request.CREATE_FILE, path='/test', content=b'test')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK
        # Try to create file inside directory as well
        self.mkdir('/test_dir')
        msg = Request(type=Request.CREATE_FILE, path='/test_dir/test', content=b'test')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK

    @pytest.mark.parametrize('path', ('/test', '/test_dir/test'))
    def test_read_file(self, path):
        self.mkfile('/test', content=b'foo')
        self.mkdir('/test_dir')
        self.mkfile('/test_dir/test', content=b'foo')
        msg = Request(type=Request.READ_FILE, path=path)
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK
        assert ret.content == b'foo'

    @pytest.mark.parametrize('path', ('/test', '/test_dir/test'))
    def test_write_file(self, path):
        self.mkdir('/test_dir')
        msg = Request(type=Request.WRITE_FILE, path=path, content=b'bar')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK
        # Make sure file has been changed
        msg = Request(type=Request.READ_FILE, path=path)
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK
        assert ret.content == b'bar'

    @pytest.mark.parametrize('path', ('/test', '/test_dir/test'))
    def test_delete_file(self, path):
        self.mkfile('/test')
        self.mkdir('/test_dir')
        self.mkfile('/test_dir/test')
        msg = Request(type=Request.DELETE_FILE, path=path)
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK
        # Make sure the file is no longer there
        msg = Request(type=Request.READ_FILE, path=path)
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.FILE_NOT_FOUND

    @pytest.mark.parametrize('path', ('/test', '/test_dir/test'))
    def test_stat_file(self, path):
        self.mkfile('/test', content=b'foo')
        self.mkdir('/test_dir')
        self.mkfile('/test_dir/test', content=b'foo')
        msg = Request(type=Request.STAT, path=path)
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK
        assert isinstance(ret.stat, Stat)
        assert ret.stat.type == Stat.FILE
        assert ret.stat.size == len(b'foo')
        assert ret.stat.atime
        assert ret.stat.ctime
        assert ret.stat.mtime
        # Remove file and make sure stat is altered as well
        ret = self.service.dispatch_msg(Request(type=Request.DELETE_FILE, path=path))
        assert ret.status_code == Response.OK
        msg = Request(type=Request.STAT, path=path)
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.FILE_NOT_FOUND

    def test_stat_dir(self):
        self.mkfile('/test')
        self.mkdir('/test_dir')
        self.mkfile('/test_dir/test')
        msg = Request(type=Request.STAT, path='/test_dir')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK
        assert isinstance(ret.stat, Stat)
        assert ret.stat.type == Stat.DIRECTORY
        assert ret.stat.size == 0
        assert ret.stat.atime
        assert ret.stat.ctime
        assert ret.stat.mtime
        # Remove dir and make sure stat is altered as well
        self.rmfile('/test_dir/test')
        self.rmdir('/test_dir')
        msg = Request(type=Request.STAT, path='/test_dir')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.FILE_NOT_FOUND

    def test_bad_stat(self):
        msg = Request(type=Request.STAT, path='/unknown')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.FILE_NOT_FOUND

    def test_service_bad_msg(self):
        msg = Request()
        msg.type = 42  # Force unknow msg type
        response = self.service.dispatch_msg(msg)
        assert response.status_code == Response.BAD_REQUEST
        assert response.error_msg == 'Unknown msg `42`'

    def test_service_bad_raw_msg(self):
        rep_buff = self.service.dispatch_raw_msg(b'dummy stuff')
        response = Response()
        response.ParseFromString(rep_buff)
        assert response.status_code == Response.BAD_REQUEST
        assert response.error_msg == 'Invalid request format'


class TestVFSService(BaseTestVFSService):

    def setup_method(self):
        params = {
            'asymetric_parameters': {
                'override': 'I SWEAR I AM ONLY USING THIS PLUGIN IN MY TEST SUITE'
            },
            'symetric_parameters': {
                'override': 'I SWEAR I AM ONLY USING THIS PLUGIN IN MY TEST SUITE'
            }
        }
        crypto_service = CryptoEngineService(symetric_cls=MockSymCipher,
                                             asymetric_cls=MockAsymCipher,
                                             **params)
        crypto_client = LocalCryptoClient(service=crypto_service)
        crypto_client.load_key(b'123456789')
        volume_client = LocalVolumeClient(VolumeServiceInMemoryMock())
        self.service = VFSService(volume_client, crypto_client)


class TestVFSServiceMock(BaseTestVFSService):

    def setup_method(self):
        self.tmpdir = tempfile.TemporaryDirectory()
        self.service = VFSServiceMock(self.tmpdir.name)

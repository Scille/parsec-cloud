import pytest
import tempfile

from parsec.volume import VolumeServiceInMemoryMock, LocalVolumeClient
from parsec.vfs import VFSService, VFSServiceMock
from parsec.vfs.vfs_pb2 import Request, Response


class BaseTestVFSService:

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
        self.test_create_dir()
        msg = Request(type=Request.REMOVE_DIR, path='/test_dir')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK

    def test_list_dir(self):

        # Create folders
        def mkdir(path):
            self.service.dispatch_msg(Request(type=Request.MAKE_DIR, path=path))

        mkdir('/countries')
        mkdir('/countries/France')
        mkdir('/countries/France/cities')
        mkdir('/countries/Belgium')
        mkdir('/countries/Belgium/cities')

        # Create multiple files
        def mkfile(path, content=b''):
            self.service.dispatch_msg(Request(type=Request.CREATE_FILE, path=path, content=content))

        mkfile('/.root')
        mkfile('/countries/index', b'1 - Belgium\n2 - France')
        mkfile('/countries/France/info', b'good=Wine, bad=Beer')
        mkfile('/countries/Belgium/info', b'good=Beer, bad=Wine')

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
        self.test_create_dir()
        msg = Request(type=Request.CREATE_FILE, path='/test_dir/test', content=b'test')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK

    @pytest.mark.parametrize('path', ('/test', '/test_dir/test'))
    def test_read_file(self, path):
        self.test_create_file()
        msg = Request(type=Request.READ_FILE, path=path)
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK
        assert ret.content == b'test'

    @pytest.mark.parametrize('path', ('/test', '/test_dir/test'))
    def test_write_file(self, path):
        self.test_create_file()
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
        self.test_create_file()
        msg = Request(type=Request.DELETE_FILE, path=path)
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK
        # Make sure the file is no longer there
        msg = Request(type=Request.READ_FILE, path=path)
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
        self.service = VFSService(LocalVolumeClient(VolumeServiceInMemoryMock()))


class TestVFSServiceMock(BaseTestVFSService):

    def setup_method(self):
        self.tmpdir = tempfile.TemporaryDirectory()
        self.service = VFSServiceMock(self.tmpdir.name)

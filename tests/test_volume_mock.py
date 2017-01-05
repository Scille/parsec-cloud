import pytest
import tempfile

from parsec.volume import (VolumeServiceMock, VolumeServiceInMemoryMock,
                           GoogleDriveVolumeService,
                           LocalVolumeClient, VolumeFileNotFoundError)
from parsec.volume.volume_pb2 import Request, Response


class TestVolumeClient:

    def setup_method(self):
        self.service = VolumeServiceInMemoryMock()
        self.client = LocalVolumeClient(service=self.service)

    def test_basic(self):
        # Create a file
        self.client.write_file('0', b'test')
        ret = self.client.read_file('0')
        assert ret.content == b'test'
        # Modify it
        self.client.write_file('0', b'')
        ret = self.client.read_file('0')
        assert ret.content == b''
        ret = self.client.read_file('0')
        assert ret.content == b''
        # Destroy it
        assert self.client.delete_file('0')
        with pytest.raises(VolumeFileNotFoundError):
            self.client.read_file('0')
        # Recreate it
        self.client.write_file('0', b'test')
        ret = self.client.read_file('0')
        assert ret.content == b'test'

    def test_read_bad_file(self):
        with pytest.raises(VolumeFileNotFoundError):
            self.client.read_file('bad_vid')

    def test_delete_bad_file(self):
        with pytest.raises(VolumeFileNotFoundError):
            self.client.delete_file('bad_vid')


class BaseTestVolumeService:

    def test_read_file(self):
        self.test_write_file()
        msg = Request(type=Request.READ_FILE, vid='000')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK
        assert ret.content == b'test'

    def test_write_file(self):
        msg = Request(type=Request.WRITE_FILE, vid='000', content=b'test')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK

    def test_overwrite_file(self):
        self.test_write_file()
        msg = Request(type=Request.WRITE_FILE, vid='000', content=b'foo')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK
        # Make sure content has change
        msg = Request(type=Request.READ_FILE, vid='000')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK
        assert ret.content == b'foo'

    def test_delete_file(self):
        self.test_write_file()
        msg = Request(type=Request.DELETE_FILE, vid='000')
        ret = self.service.dispatch_msg(msg)
        assert isinstance(ret, Response)
        assert ret.status_code == Response.OK
        # Make sure file has been removed
        msg = Request(type=Request.READ_FILE, vid='000')
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


class TestVolumeServiceInMemoryMock(BaseTestVolumeService):

    def setup_method(self):
        self.service = VolumeServiceInMemoryMock()


class TestVolumeServiceMock(BaseTestVolumeService):

    def setup_method(self):
        self.tmpdir = tempfile.TemporaryDirectory()
        self.service = VolumeServiceMock(self.tmpdir.name)


class TestGoogleDriveVolumeService(BaseTestVolumeService):

    def setup_method(self):
        self.service = GoogleDriveVolumeService()
        self.service.initialize_driver(force=True)

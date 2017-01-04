import pytest

from parsec.volume import VolumeServiceInMemoryMock, LocalVolumeClient, VolumeFileNotFoundError
from parsec.volume.volume_pb2 import Request, Response


class TestVolumeClient:
    def setup_method(self):
        self.service = VolumeServiceInMemoryMock()
        self.client = LocalVolumeClient(service=self.service)

    def test_basic(self):
        # Create a file
        self.client.write_file(0, b'test')
        ret = self.client.read_file(0)
        assert ret.content == b'test'
        # Modify it
        self.client.write_file(0, b'')
        ret = self.client.read_file(0)
        assert ret.content == b''
        ret = self.client.read_file(0)
        assert ret.content == b''
        # Destroy it
        assert self.client.delete_file(0)
        with pytest.raises(VolumeFileNotFoundError):
            self.client.read_file(0)
        # Recreate it
        self.client.write_file(0, b'test')
        ret = self.client.read_file(0)
        assert ret.content == b'test'

    def test_read_bad_file(self):
        with pytest.raises(VolumeFileNotFoundError):
            self.client.read_file(42)

    def test_delete_bad_file(self):
        with pytest.raises(VolumeFileNotFoundError):
            self.client.delete_file(42)


class TestVolumeServiceInMemoryMock:
    def setup_method(self):
        self.service = VolumeServiceInMemoryMock()

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

import pytest
import tempfile

from .common import b64

from parsec.volume import VolumeServiceMock, VolumeServiceInMemoryMock


google_drive = pytest.mark.skipif(
    not pytest.config.getoption("--run-google-drive"),
    reason="Cannot run Drive test without order"
)


class BaseTestVolumeService:

    # Helpers

    async def mkfile(self, vid, content=b''):
        return await self.service.write_file(vid=vid, content=content)

    # Tests

    @pytest.mark.asyncio
    async def test_read_file(self):
        await self.mkfile(vid='000', content=b'test')
        ret = await self.service.dispatch_msg({'cmd': 'read_file', 'vid': '000'})
        assert ret == {'status': 'ok', 'content': b64(b'test')}

    @pytest.mark.asyncio
    async def test_write_file(self):
        ret = await self.service.dispatch_msg({'cmd': 'write_file', 'vid': '000', 'content': b64(b'test')})
        assert ret == {'status': 'ok', 'size': 4}

    @pytest.mark.asyncio
    async def test_overwrite_file(self):
        await self.mkfile(vid='000', content=b'test')
        ret = await self.service.dispatch_msg({'cmd': 'write_file', 'vid': '000', 'content': b64(b'foo')})
        assert ret == {'status': 'ok', 'size': 3}
        # Make sure content has change
        ret = await self.service.dispatch_msg({'cmd': 'read_file', 'vid': '000'})
        assert ret == {'status': 'ok', 'content': b64(b'foo')}

    @pytest.mark.asyncio
    async def test_delete_file(self):
        await self.mkfile(vid='000', content=b'test')
        ret = await self.service.dispatch_msg({'cmd': 'delete_file', 'vid': '000'})
        assert ret == {'status': 'ok'}
        # Make sure file has been removed
        ret = await self.service.dispatch_msg({'cmd': 'read_file', 'vid': '000'})
        assert ret == {'status': 'not_found', 'label': 'File not found.'}


class TestVolumeServiceInMemoryMock(BaseTestVolumeService):

    def setup_method(self):
        self.service = VolumeServiceInMemoryMock()


class TestVolumeServiceMock(BaseTestVolumeService):

    def setup_method(self):
        self.tmpdir = tempfile.TemporaryDirectory()
        self.service = VolumeServiceMock(self.tmpdir.name)


# class TestLocalFolderVolumeService(BaseTestVolumeService):

#     def setup_method(self):
#         self.tmpdir = tempfile.TemporaryDirectory()
#         self.service = LocalFolderVolumeService(self.tmpdir.name)


# @google_drive
# class TestGoogleDriveVolumeService(BaseTestVolumeService):

#     def setup_method(self):
#         self.service = GoogleDriveVolumeService()
#         self.service.initialize_driver(force=True)

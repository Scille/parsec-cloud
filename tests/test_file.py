from uuid import uuid4

from freezegun import freeze_time
import pytest

from parsec.core.file_service import FileService


class BaseTestFileService:

    # Helpers

    # Tests

    @pytest.mark.asyncio
    async def test_create_file(self):
        ret = await self.service.dispatch_msg({'cmd': 'create_file'})
        assert ret['status'] == 'ok'
        # assert ret['id'] # TODO check id

    @pytest.mark.asyncio
    async def test_read_file(self):
        ret = await self.service.dispatch_msg({'cmd': 'create_file'})
        id = ret['id']
        # Empty file
        ret = await self.service.dispatch_msg({'cmd': 'read_file', 'id': id})
        assert ret == {'status': 'ok', 'content': ''}
        # Not empty file
        ret = await self.service.dispatch_msg({'cmd': 'write_file', 'id': id, 'content': 'foo'})
        ret = await self.service.dispatch_msg({'cmd': 'read_file', 'id': id})
        assert ret == {'status': 'ok', 'content': 'foo'}
        # Unknown file
        ret = await self.service.dispatch_msg({'cmd': 'read_file',
                                               'id': '5ea26ae2479c49f58ede248cdca1a3ca'})
        assert ret == {'status': 'not_found', 'label': 'File not found.'}

    @pytest.mark.asyncio
    async def test_write_file(self):
        ret = await self.service.dispatch_msg({'cmd': 'create_file'})
        id = ret['id']
        # Check with empty and not empty file
        for content in ['foo', 'bar']:
            ret = await self.service.dispatch_msg({'cmd': 'write_file',
                                                   'id': id,
                                                   'content': content})
            assert ret == {'status': 'ok'}
            ret = await self.service.dispatch_msg({'cmd': 'read_file', 'id': id})
            assert ret == {'status': 'ok', 'content': content}
        # Unknown file
        ret = await self.service.dispatch_msg({'cmd': 'write_file',
                                               'id': uuid4(),
                                               'content': 'foo'})
        assert ret == {'status': 'not_found', 'label': 'File not found.'}

    @pytest.mark.asyncio
    # @freeze_time("2012-01-01")
    async def test_stat_file(self):
            # Good file
            with freeze_time('2012-01-01') as frozen_datetime:
                ret = await self.service.dispatch_msg({'cmd': 'create_file'})
                id = ret['id']
                ret = await self.service.dispatch_msg({'cmd': 'stat_file', 'id': id})
                ctime = frozen_datetime().timestamp()
                assert ret == {'status': 'ok', 'stats': {'id': id,
                                                         'ctime': ctime,
                                                         'mtime': ctime,
                                                         'atime': ctime,
                                                         'size': 0}}
                frozen_datetime.tick()
                mtime = frozen_datetime().timestamp()
                ret = await self.service.dispatch_msg({'cmd': 'write_file',
                                                       'id': id,
                                                       'content': 'foo'})
                ret = await self.service.dispatch_msg({'cmd': 'stat_file', 'id': id})
                assert ret == {'status': 'ok', 'stats': {'id': id,
                                                         'ctime': ctime,
                                                         'mtime': mtime,
                                                         'atime': ctime,
                                                         'size': 3}}

                frozen_datetime.tick()
                atime = frozen_datetime().timestamp()
                ret = await self.service.dispatch_msg({'cmd': 'read_file', 'id': id})
                ret = await self.service.dispatch_msg({'cmd': 'stat_file', 'id': id})
                assert ret == {'status': 'ok', 'stats': {'id': id,
                                                         'ctime': ctime,
                                                         'mtime': mtime,
                                                         'atime': atime,
                                                         'size': 3}}
            # Unknown file
            ret = await self.service.dispatch_msg({'cmd': 'write_file',
                                                   'id': uuid4(),
                                                   'content': 'foo'})
            assert ret == {'status': 'not_found', 'label': 'File not found.'}

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_history(self):
        raise NotImplementedError()


class TestFileService(BaseTestFileService):

    def setup_method(self):
        self.service = FileService()

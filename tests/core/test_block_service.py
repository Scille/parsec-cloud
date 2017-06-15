import asyncio
import botocore
from collections import defaultdict
from datetime import datetime, timezone
from freezegun import freeze_time
from io import BytesIO
import pytest

from parsec.core import MetaBlockService, MockedBlockService
from parsec.core.cache import cache
from parsec.exceptions import BlockError


class MockedS3Client:
    def __init__(self):
        self._buckets = defaultdict(dict)

    def put_object(self, Bucket, Key, Body, Metadata={}):
        last_modified = datetime.now(timezone.utc)
        Body = Body if isinstance(Body, bytes) else Body.encode()
        self._buckets[Bucket][Key] = (Body, Metadata, last_modified)

    def get_object(self, Bucket, Key):
        try:
            body, meta, last_modified = self._buckets[Bucket][Key]
        except KeyError:
            message = {'Error': {'Code': 'AccessDenied', 'Message': 'Access Denied'}}
            raise botocore.exceptions.ClientError(message, 'GetObject')
        host_id = 'Qfa4CPJGv8GA12/GFbkDivaE3ehoYpkZUUElRSUdphYlIgZ2Mma5z2kzwOpeLQNkbhj8X+cYInk='
        x_amz_id_2 = 'Qfa4CPJGv8GA12/GFbkDivaE3ehoYpkZUUElRSUdphYlIgZ2Mma5z2kzwOpeLQNkbhj8X+cYInk='
        return {
            'ContentLength': 8,
            'LastModified': last_modified,
            'Metadata': meta,
            'ResponseMetadata': {
                'HostId': host_id,
                'HTTPStatusCode': 200,
                'RequestId': 'FADF5D8DC3E1BC10',
                'RetryAttempts': 0,
                'HTTPHeaders': {
                    'x-amz-id-2': x_amz_id_2,
                    'accept-ranges': 'bytes',
                    'content-length': '8',
                    'content-type': 'binary/octet-stream',
                    'server': 'AmazonS3',
                    'last-modified': 'Tue, 09 May 2017 16:57:47 GMT',
                    'etag': '"c16fe37069c61b23d689d3196e96949c"',
                    'date': 'Tue, 09 May 2017 17:47:56 GMT',
                    'x-amz-request-id': 'FADF5D8DC3E1BC10'
                }
            },
            'ContentType': 'binary/octet-stream',
            'ETag': '"c16fe37069c61b23d689d3196e96949c"',
            'Body': BytesIO(body),
            'AcceptRanges': 'bytes'}


async def bootstrap_S3BlockService(request, event_loop):
    module = pytest.importorskip('parsec.core.block_service_s3')
    svc = module.S3BlockService('region-1', 'parsec-test', '<dummy-s3-key>', '<dummy-s3-secret>')
    svc._s3 = MockedS3Client()
    return svc


async def bootstrap_MetaBlockService(request, event_loop):
    return MetaBlockService([MockedBlockService, MockedBlockService])


@pytest.fixture(params=[MockedBlockService, bootstrap_S3BlockService, bootstrap_MetaBlockService],
                ids=['mocked', 's3', 'metablock'])
def block_svc(request, event_loop):
    if asyncio.iscoroutinefunction(request.param):
        block_svc = event_loop.run_until_complete(request.param(request, event_loop))
    else:
        block_svc = request.param()
    return block_svc


@pytest.fixture
@freeze_time("2012-01-01")
def block(block_svc, event_loop, content='Whatever.'):
    return event_loop.run_until_complete(block_svc.create(content)), content


class TestBlockService:

    @pytest.mark.asyncio
    async def test_create(self, block_svc):
        block_content = 'Whatever.'
        with freeze_time('2012-01-01') as frozen_datetime:
            creation_date = frozen_datetime().isoformat()
            block_id = await block_svc.create(content=block_content)
            # Check block in cache
            response = await cache.get('read:' + block_id)
            assert response == {'content': block_content,
                                'creation_date': creation_date,
                                'status': 'ok'}
            # Check block in cache
            response = await cache.get('stat:' + block_id)
            assert response == {'creation_date': creation_date,
                                'status': 'ok'}

    @pytest.mark.asyncio
    async def test_create_with_id(self, block_svc):
        requested_block_id = '1234'
        block_content = 'Whatever.'
        with freeze_time('2012-01-01') as frozen_datetime:
            creation_date = frozen_datetime().isoformat()
            block_id = await block_svc.create(content=block_content, id=requested_block_id)
            assert block_id == requested_block_id
            # Check block in cache
            response = await cache.get('read:' + block_id)
            assert response == {'content': block_content,
                                'creation_date': creation_date,
                                'status': 'ok'}
            # Check block in cache
            response = await cache.get('stat:' + block_id)
            assert response == {'creation_date': creation_date,
                                'status': 'ok'}

    @pytest.mark.asyncio
    @freeze_time("2012-01-01")
    async def test_read(self, block_svc, block):
        block_id, block_content = block
        await cache.delete('read:' + block_id)
        # Block not found in cache
        assert await cache.get('read:' + block_id) is None
        # Read block
        with freeze_time('2012-01-01') as frozen_datetime:
            creation_date = frozen_datetime().isoformat()
            frozen_datetime.tick()
            ret = await block_svc.read(id=block_id)
            assert {'content': block_content,
                    'creation_date': creation_date} == ret
        # Check block in cache
        response = await cache.get('read:' + block_id)
        assert response == {'content': 'Whatever.',
                            'creation_date': creation_date}
        # Read using cache
        await cache.set(('read', block_id),
                        {'content': 'cached content',
                         'creation_date': creation_date})
        ret = await block_svc.read(id=block_id)
        assert {'creation_date': creation_date,
                'content': 'cached content'} == ret
        # Unknown block
        with pytest.raises(BlockError):
            await block_svc.read(id='unknown')

    @pytest.mark.asyncio
    @freeze_time("2012-01-01")
    async def test_stat(self, block_svc, block):
        block_id, block_content = block
        # Block not found in cache
        await cache.delete('stat:' + block_id)
        assert await cache.get('stat:' + block_id) is None
        # Stat block
        with freeze_time('2012-01-01') as frozen_datetime:
            creation_date = frozen_datetime().isoformat()
            ret = await block_svc.stat(id=block_id)
            assert {'creation_date': creation_date} == ret
        # Check block in cache
        response = await cache.get('stat:' + block_id)
        assert response == {'creation_date': creation_date}
        # Stat using cache
        new_date = datetime.utcnow().date()
        await cache.set(('stat', block_id),
                        {'creation_date': new_date})
        ret = await block_svc.stat(id=block_id)
        assert {'creation_date': new_date} == ret
        # Unknown block
        with pytest.raises(BlockError):
            await block_svc.stat(id='unknown')

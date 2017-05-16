import pytest
import asyncio
import botocore
from io import BytesIO
from freezegun import freeze_time
from datetime import datetime, timezone
from collections import defaultdict

from parsec.core import MetaBlockService, MockedBlockService, MockedCacheService
from parsec.core.cache_service import CacheNotFound


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
    svc = module.S3BlockService()
    svc.init('region-1', 'parsec-test', '<dummy-s3-key>', '<dummy-s3-secret>')
    svc._s3 = MockedS3Client()
    return svc


async def bootstrap_MetaBlockService(request, event_loop):
    MockedBlockService.cache_service = cache_svc()
    return MetaBlockService([MockedBlockService, MockedBlockService])


@pytest.fixture
def cache_svc():
    return MockedCacheService()


@pytest.fixture(params=[MockedBlockService, bootstrap_S3BlockService, bootstrap_MetaBlockService],
                ids=['mocked', 's3', 'metablock'])
def block_svc(request, event_loop, cache_svc):
    if asyncio.iscoroutinefunction(request.param):
        block_svc = event_loop.run_until_complete(request.param(request, event_loop))
    else:
        block_svc = request.param()
    block_svc.cache_service = cache_svc
    return block_svc


@pytest.fixture
@freeze_time("2012-01-01")
def block(block_svc, event_loop, content='Whatever.'):
    return event_loop.run_until_complete(block_svc.create(content)), content


class TestBlockServiceAPI:

    @pytest.mark.asyncio
    async def test_create(self, block_svc, cache_svc):
        block_content = 'Whatever.'
        with freeze_time('2012-01-01') as frozen_datetime:
            creation_timestamp = frozen_datetime().timestamp()
            ret = await block_svc.dispatch_msg({'cmd': 'block_create', 'content': block_content})
            assert sorted(ret.keys()) == ['id', 'status']
            assert ret['status'] == 'ok'
            assert ret['id']
            block_id = ret['id']
            # Check block in cache
            response = await cache_svc.get(('read', block_id))
            assert response == {'content': block_content,
                                'creation_timestamp': creation_timestamp,
                                'status': 'ok'}
            # Check block in cache
            response = await cache_svc.get(('stat', block_id))
            assert response == {'creation_timestamp': creation_timestamp,
                                'status': 'ok'}

    @pytest.mark.asyncio
    async def test_create_with_id(self, block_svc, cache_svc):
        block_id = '1234'
        block_content = 'Whatever.'
        with freeze_time('2012-01-01') as frozen_datetime:
            creation_timestamp = frozen_datetime().timestamp()
            ret = await block_svc.dispatch_msg({'cmd': 'block_create',
                                                'content': block_content,
                                                'id': block_id})
            assert sorted(ret.keys()) == ['id', 'status']
            assert ret['status'] == 'ok'
            assert ret['id'] == '1234'
            # Check block in cache
            response = await cache_svc.get(('read', block_id))
            assert response == {'content': block_content,
                                'creation_timestamp': creation_timestamp,
                                'status': 'ok'}
            # Check block in cache
            response = await cache_svc.get(('stat', block_id))
            assert response == {'creation_timestamp': creation_timestamp,
                                'status': 'ok'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'block_create', 'content': '...', 'bad_field': 'foo'},
        {'cmd': 'block_create', 'content': 42},
        {'cmd': 'block_create', 'content': None},
        {'cmd': 'block_create'}, {}])
    async def test_bad_msg_create(self, block_svc, bad_msg):
        ret = await block_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    @freeze_time("2012-01-01")
    async def test_read(self, block_svc, cache_svc, block):
        block_id, block_content = block
        del cache_svc.cache[('read', block_id)]  # TODO too intrusive?
        # Block not found in cache
        with pytest.raises(CacheNotFound):
            await cache_svc.get(('read', block_id))
        # Read block
        with freeze_time('2012-01-01') as frozen_datetime:
            creation_timestamp = frozen_datetime().timestamp()
            frozen_datetime.tick()
            ret = await block_svc.dispatch_msg({'cmd': 'block_read', 'id': block_id})
            assert {'status': 'ok',
                    'creation_timestamp': creation_timestamp,
                    'content': block_content} == ret
        # Check block in cache
        response = await cache_svc.get(('read', block_id))
        assert response == {'content': 'Whatever.',
                            'creation_timestamp': creation_timestamp,
                            'status': 'ok'}
        # Read using cache
        await cache_svc.set(('read', block_id),
                            {'content': 'cached content',
                             'creation_timestamp': creation_timestamp,
                             'status': 'ok'})
        ret = await block_svc.dispatch_msg({'cmd': 'block_read', 'id': block_id})
        assert {'status': 'ok',
                'creation_timestamp': creation_timestamp,
                'content': 'cached content'} == ret
        # Unknown block
        ret = await block_svc.dispatch_msg({'cmd': 'block_read', 'id': '1234'})
        assert ret['status'] == 'not_found' or ret['status'] == 'block_error'  # TODO ok?

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'block_read', 'id': '<insert_id_here>', 'bad_field': 'foo'},
        {'cmd': 'block_read', 'id': 42},
        {'cmd': 'block_read', 'id': None},
        {'cmd': 'block_read'}, {}])
    async def test_bad_msg_read(self, block_svc, block, bad_msg):
        block_id, block_content = block
        if bad_msg.get('id') == '<insert_id_here>':
            bad_msg['id'] = block_id
        ret = await block_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    @freeze_time("2012-01-01")
    async def test_stat(self, block_svc, cache_svc, block):
        block_id, block_content = block
        # Block not found in cache
        del cache_svc.cache[('stat', block_id)]  # TODO too intrusive?
        with pytest.raises(CacheNotFound):
            await cache_svc.get(('stat', block_id))
        # Stat block
        with freeze_time('2012-01-01') as frozen_datetime:
            creation_timestamp = frozen_datetime().timestamp()
            ret = await block_svc.dispatch_msg({'cmd': 'block_stat', 'id': block_id})
            assert {'status': 'ok',
                    'creation_timestamp': creation_timestamp} == ret
        # Check block in cache
        response = await cache_svc.get(('stat', block_id))
        assert response == {'creation_timestamp': creation_timestamp,
                            'status': 'ok'}
        # Stat using cache
        new_timestamp = datetime.utcnow().timestamp()
        await cache_svc.set(('stat', block_id),
                            {'creation_timestamp': new_timestamp,
                             'status': 'ok'})
        ret = await block_svc.dispatch_msg({'cmd': 'block_stat', 'id': block_id})
        assert {'status': 'ok',
                'creation_timestamp': new_timestamp} == ret
        # Unknown block
        ret = await block_svc.dispatch_msg({'cmd': 'block_stat', 'id': '1234'})
        assert ret['status'] == 'not_found' or ret['status'] == 'block_error'  # TODO ok?

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'block_stat', 'id': '<insert_id_here>', 'bad_field': 'foo'},
        {'cmd': 'block_stat', 'id': 42},
        {'cmd': 'block_stat', 'id': None},
        {'cmd': 'block_stat'}, {}])
    async def test_bad_msg_stat(self, block_svc, block, bad_msg):
        block_id, block_content = block
        if bad_msg.get('id') == '<insert_id_here>':
            bad_msg['id'] = block_id
        ret = await block_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

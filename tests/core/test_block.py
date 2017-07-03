import pytest
import io
from effect2 import Effect, asyncio_perform
# from aioeffect import perform as asyncio_perform
from unittest.mock import patch
from botocore.exceptions import (
    ClientError as S3ClientError, EndpointConnectionError as S3EndpointConnectionError
)

from parsec.core.block import (
    EBlockCreate, EBlockRead, Block, in_memory_block_dispatcher_factory,
    s3_block_dispatcher_factory
)
from parsec.exceptions import BlockError, BlockNotFound


@pytest.fixture
def s3_block_dispatcher():
    with patch('boto3.client') as mocked_boto3_client_cls:
        dispatcher = s3_block_dispatcher_factory('region', 'bucket', 'KEY', 'SECRET')
        dispatcher.mocked_boto3_client = mocked_boto3_client_cls.return_value
        yield dispatcher


class TestInMemory:
    def setup(self):
        self.dispatcher = in_memory_block_dispatcher_factory()

    async def created_block(self, id='0', content=b''):
        intent = Effect(EBlockCreate(id=id, content=content))
        return await asyncio_perform(self.dispatcher, intent)

    @pytest.mark.asyncio
    async def test_perform_block_create(self):
        intent = Effect(EBlockCreate(id='42', content=b'foo'))
        ret = await asyncio_perform(self.dispatcher, intent)
        assert ret == Block('42', b'foo')

    @pytest.mark.asyncio
    async def test_perform_block_create_duplicate_id(self):
        block = await self.created_block()
        intent = Effect(EBlockCreate(id=block.id, content=b'bar'))
        with pytest.raises(BlockError):
            await asyncio_perform(self.dispatcher, intent)

    @pytest.mark.asyncio
    async def test_perform_block_read(self):
        block = await self.created_block()
        intent = Effect(EBlockRead(id=block.id))
        ret = await asyncio_perform(self.dispatcher, intent)
        assert ret == block

    @pytest.mark.asyncio
    async def test_perform_block_read_not_found(self):
        intent = Effect(EBlockRead(id='unknown_id'))
        with pytest.raises(BlockNotFound):
            await asyncio_perform(self.dispatcher, intent)


class TestS3:

    @pytest.mark.asyncio
    async def test_perform_block_create(self, s3_block_dispatcher):
        intent = Effect(EBlockCreate(id='42', content=b'foo'))
        ret = await asyncio_perform(s3_block_dispatcher, intent)
        assert ret == Block('42', b'foo')
        s3_block_dispatcher.mocked_boto3_client.put_object.assert_called_once_with(
            Body=b'foo', Bucket='bucket', Key='42')

    @pytest.mark.asyncio
    async def test_perform_block_create_duplicate_id(self, s3_block_dispatcher):
        s3_block_dispatcher.mocked_boto3_client.put_object.side_effect = \
            S3ClientError({'Error': {}}, 'put_object')
        intent = Effect(EBlockCreate(id='42', content=b'bar'))
        with pytest.raises(BlockError):
            await asyncio_perform(s3_block_dispatcher, intent)

    @pytest.mark.asyncio
    async def test_perform_block_read(self, s3_block_dispatcher):
        s3_block_dispatcher.mocked_boto3_client.get_object.return_value = {
            'Body': io.BytesIO(b'bar')}
        intent = Effect(EBlockRead(id='42'))
        ret = await asyncio_perform(s3_block_dispatcher, intent)
        assert ret == Block(id='42', content=b'bar')

    @pytest.mark.asyncio
    async def test_perform_block_read_not_found(self, s3_block_dispatcher):
        s3_block_dispatcher.mocked_boto3_client.get_object.side_effect = \
            S3ClientError({'Error': {}}, 'get_object')
        intent = Effect(EBlockRead(id='unknown_id'))
        with pytest.raises(BlockNotFound):
            await asyncio_perform(s3_block_dispatcher, intent)

    @pytest.mark.asyncio
    async def test_perform_block_create_no_connection(self, s3_block_dispatcher):
        s3_block_dispatcher.mocked_boto3_client.put_object.side_effect = \
            S3EndpointConnectionError(endpoint_url='put_object')
        intent = Effect(EBlockCreate(id='42', content=b'foo'))
        with pytest.raises(BlockError):
            await asyncio_perform(s3_block_dispatcher, intent)

    @pytest.mark.asyncio
    async def test_perform_block_read_no_connection(self, s3_block_dispatcher):
        s3_block_dispatcher.mocked_boto3_client.get_object.side_effect = \
            S3EndpointConnectionError(endpoint_url='get_object')
        intent = Effect(EBlockRead(id='42'))
        with pytest.raises(BlockError):
            await asyncio_perform(s3_block_dispatcher, intent)

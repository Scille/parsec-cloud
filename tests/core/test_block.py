import pytest
import io
from effect2.testing import const, asyncio_perform_sequence
from unittest.mock import patch
from botocore.exceptions import (
    ClientError as S3ClientError, EndpointConnectionError as S3EndpointConnectionError
)

from parsec.core.block import (
    EBlockCreate, EBlockRead, EBackendBlockStoreGetURL, Block, BlockComponent
)
from parsec.core.block_s3 import S3BlockConnection
from parsec.exceptions import BlockError, BlockNotFound

from tests.common import AsyncMock


async def test_create_lazy_connection():
    with patch('parsec.core.block.block_connection_factory', new_callable=AsyncMock) \
            as block_connection_factory_mock:
        block_connection_factory_mock.return_value.create.set_asyncret()
        block_connection_factory_mock.return_value.close_connection.set_asyncret()
        block_component = BlockComponent()

        intent = EBlockCreate('4242', b'<content>')
        eff = block_component.get_dispatcher()(intent)(intent)
        sequence = [
            (EBackendBlockStoreGetURL(), const('http://foo')),
        ]
        resp = await asyncio_perform_sequence(sequence, eff)
        assert resp == Block('4242', b'<content>')
        block_connection_factory_mock.assert_called_once_with('http://foo')
        block_connection_factory_mock.return_value.create.assert_called_once_with(
            '4242', b'<content>')
        assert block_component.connection

        # Next create should not trigger the connection creation...
        block_connection_factory_mock.reset_mock()
        block_connection_factory_mock.return_value.create.reset_mock()
        eff = block_component.get_dispatcher()(intent)(intent)
        resp = await asyncio_perform_sequence([], eff)
        assert resp == Block('4242', b'<content>')
        block_connection_factory_mock.assert_not_called()
        block_connection_factory_mock.return_value.create.assert_called_once_with(
            '4242', b'<content>')

        # ...unless we reset the collection
        await block_component.perform_block_reset()
        assert not block_component.connection
        eff = block_component.get_dispatcher()(intent)(intent)
        sequence = [
            (EBackendBlockStoreGetURL(), const('http://foo')),

        ]
        resp = await asyncio_perform_sequence(sequence, eff)
        assert resp == Block('4242', b'<content>')
        block_connection_factory_mock.assert_called_once_with('http://foo')


async def test_read_lazy_connection():
    with patch('parsec.core.block.block_connection_factory', new_callable=AsyncMock) \
            as block_connection_factory_mock:
        block_connection_factory_mock.return_value.read.set_asyncret(b'<content>')
        block_connection_factory_mock.return_value.close_connection.set_asyncret()

        block_component = BlockComponent()
        intent = EBlockRead('4242')
        eff = block_component.get_dispatcher()(intent)(intent)
        sequence = [
            (EBackendBlockStoreGetURL(), const('http://foo')),
        ]
        resp = await asyncio_perform_sequence(sequence, eff)
        assert resp == Block('4242', b'<content>')
        block_connection_factory_mock.assert_called_once_with('http://foo')
        block_connection_factory_mock.return_value.read.assert_called_once_with('4242')
        assert block_component.connection

        # Next create should not trigger the connection creation...
        block_connection_factory_mock.reset_mock()
        block_connection_factory_mock.return_value.read.reset_mock()
        eff = block_component.get_dispatcher()(intent)(intent)
        resp = await asyncio_perform_sequence([], eff)
        assert resp == Block('4242', b'<content>')
        block_connection_factory_mock.assert_not_called()
        block_connection_factory_mock.return_value.read.assert_called_once_with('4242')

        # ...unless we reset the collection
        await block_component.perform_block_reset()
        assert not block_component.connection
        eff = block_component.get_dispatcher()(intent)(intent)
        sequence = [
            (EBackendBlockStoreGetURL(), const('http://foo')),

        ]
        resp = await asyncio_perform_sequence(sequence, eff)
        assert resp == Block('4242', b'<content>')
        block_connection_factory_mock.assert_called_once_with('http://foo')


# TODO !

# @pytest.fixture
# def rest_block_connection():
#     with patch('aiohttp.ClientSession') as mocked_aiohttp_client_session:
#         conn = RESTBlockConnection()
#         conn.mocked_aiohttp_client_session
#         yield conn


# class TestREST:
#     pass


@pytest.fixture
def s3_block_connection():
    with patch('boto3.client') as mocked_boto3_client_cls:
        conn = S3BlockConnection('region', 'bucket', 'KEY', 'SECRET')
        conn.mocked_boto3_client = mocked_boto3_client_cls.return_value
        yield conn


class TestS3:

    async def test_block_create(self, s3_block_connection):
        ret = await s3_block_connection.create('42', b'foo')
        assert ret is None
        s3_block_connection.mocked_boto3_client.put_object.assert_called_once_with(
            Body=b'foo', Bucket='bucket', Key='42')

    async def test_block_create_duplicate_id(self, s3_block_connection):
        s3_block_connection.mocked_boto3_client.put_object.side_effect = \
            S3ClientError({'Error': {}}, 'put_object')
        with pytest.raises(BlockError):
            await s3_block_connection.create('42', b'foo')

    async def test_perform_block_read(self, s3_block_connection):
        s3_block_connection.mocked_boto3_client.get_object.return_value = {
            'Body': io.BytesIO(b'bar')}
        ret = await s3_block_connection.read('42')
        assert ret == b'bar'

    async def test_perform_block_read_not_found(self, s3_block_connection):
        s3_block_connection.mocked_boto3_client.get_object.side_effect = \
            S3ClientError({'Error': {}}, 'get_object')
        with pytest.raises(BlockNotFound):
            await s3_block_connection.read('unknown_id')

    async def test_perform_block_create_no_connection(self, s3_block_connection):
        s3_block_connection.mocked_boto3_client.put_object.side_effect = \
            S3EndpointConnectionError(endpoint_url='put_object')
        with pytest.raises(BlockError):
            await s3_block_connection.create('42', b'foo')

    async def test_perform_block_read_no_connection(self, s3_block_connection):
        s3_block_connection.mocked_boto3_client.get_object.side_effect = \
            S3EndpointConnectionError(endpoint_url='get_object')
        with pytest.raises(BlockError):
            await s3_block_connection.read('42')

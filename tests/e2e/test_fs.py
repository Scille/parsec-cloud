import pytest
from freezegun import freeze_time

from parsec.tools import to_jsonb64


async def mk_file(johndoe_client, path='/foo.txt', data=b'foo'):
    await johndoe_client.send_cmd('file_create', path=path)
    await johndoe_client.send_cmd('file_write', path=path, content=data)
    return path


async def mk_folder(johndoe_client, path='/bar'):
    await johndoe_client.send_cmd('folder_create', path=path)
    return path


async def test_stat(johndoe_client):
    ret = await johndoe_client.send_cmd('stat', path='/')
    assert ret == {'children': [], 'status': 'ok', 'type': 'folder'}


async def test_stat_file(johndoe_client):
    with freeze_time('2017-12-02T12:30:23'):
        path = await mk_file(johndoe_client, path='/foo.txt', data=b'x' * 32)
    ret = await johndoe_client.send_cmd('stat', path=path)
    assert ret == {
        'status': 'ok',
        'type': 'file',
        'version': 2,
        'created': '2017-12-02T12:30:23+00:00',
        'updated': '2017-12-02T12:30:23+00:00',
        'size': 32
    }


async def test_create_file(johndoe_client):
    with freeze_time('2017-12-02T12:30:23'):
        ret = await johndoe_client.send_cmd('file_create', path='/foo.txt')
    assert ret == {'status': 'ok'}
    ret = await johndoe_client.send_cmd('stat', path='/')
    assert ret == {'children': ['foo.txt'], 'status': 'ok', 'type': 'folder'}
    ret = await johndoe_client.send_cmd('stat', path='/foo.txt')
    assert ret == {
        'status': 'ok',
        'type': 'file',
        'version': 1,
        'created': '2017-12-02T12:30:23+00:00',
        'updated': '2017-12-02T12:30:23+00:00',
        'size': 0
    }

async def test_create_duplicated_file(johndoe_client):
    path = await mk_file(johndoe_client)
    ret = await johndoe_client.send_cmd('file_create', path=path)
    assert ret == {'status': 'invalid_path', 'label': 'Path `/foo.txt` already exist'}


async def test_write_file(johndoe_client):
    with freeze_time('2017-12-02T01:00:00'):
        ret = await johndoe_client.send_cmd('file_create', path='/foo.txt')
    assert ret == {'status': 'ok'}
    with freeze_time('2017-12-02T02:00:00'):
        ret = await johndoe_client.send_cmd('file_write', path='/foo.txt', content=b'fooo')
    assert ret == {'status': 'ok'}
    ret = await johndoe_client.send_cmd('file_read', path='/foo.txt')
    assert ret == {'status': 'ok', 'content': to_jsonb64(b'fooo')}
    ret = await johndoe_client.send_cmd('stat', path='/foo.txt')
    assert ret == {
        'status': 'ok',
        'type': 'file',
        'version': 2,
        'created': '2017-12-02T01:00:00+00:00',
        'updated': '2017-12-02T02:00:00+00:00',
        'size': 4
    }


async def test_write_offset(johndoe_client):
    await mk_file(johndoe_client, '/foo.txt', data=b'1234567890')
    ret = await johndoe_client.send_cmd('file_write', path='/foo.txt', content=b'abcd', offset=3)
    assert ret == {'status': 'ok'}
    ret = await johndoe_client.send_cmd('file_read', path='/foo.txt')
    assert ret == {'status': 'ok', 'content': to_jsonb64(b'123abcd890')}


async def test_write_unknown_file(johndoe_client):
    ret = await johndoe_client.send_cmd('file_write', path='/unknown', content=b'abcd')
    assert ret == {'status': 'invalid_path', 'label': "Path `/unknown` doesn't exist"}


async def test_read_offset(johndoe_client):
    await mk_file(johndoe_client, '/foo.txt', data=b'1234567890')
    ret = await johndoe_client.send_cmd('file_read', path='/foo.txt', offset=3, size=4)
    assert ret == {'status': 'ok', 'content': to_jsonb64(b'4567')}


async def test_read_unknown_file(johndoe_client):
    ret = await johndoe_client.send_cmd('file_read', path='/unknown')
    assert ret == {'status': 'invalid_path', 'label': "Path `/unknown` doesn't exist"}


async def test_big_file(johndoe_client):
    data = b'x' * 10000  # 10ko
    await mk_file(johndoe_client, '/foo.txt', data=data)
    ret = await johndoe_client.send_cmd('file_read', path='/foo.txt', offset=42, size=1000)
    assert ret == {'status': 'ok', 'content': to_jsonb64(data[42:1042])}


async def test_file_truncate(johndoe_client):
    await mk_file(johndoe_client, '/foo.txt', data=b'1234567890')
    ret = await johndoe_client.send_cmd('file_truncate', path='/foo.txt', length=4)
    assert ret == {'status': 'ok'}
    ret = await johndoe_client.send_cmd('file_read', path='/foo.txt')
    assert ret == {'status': 'ok', 'content': to_jsonb64(b'1234')}


async def test_truncate_unknown_file(johndoe_client):
    ret = await johndoe_client.send_cmd('file_truncate', path='/unknown', length=4)
    assert ret == {'status': 'invalid_path', 'label': "Path `/unknown` doesn't exist"}


async def test_create_dir(johndoe_client):
    ret = await johndoe_client.send_cmd('folder_create', path='/bar')
    assert ret == {'status': 'ok'}
    ret = await johndoe_client.send_cmd('stat', path='/')
    assert ret == {'children': ['bar'], 'status': 'ok', 'type': 'folder'}


async def test_create_duplicated_dir(johndoe_client):
    path = await mk_folder(johndoe_client)
    ret = await johndoe_client.send_cmd('folder_create', path=path)
    assert ret == {'status': 'invalid_path', 'label': "Path `/bar` already exist"}


async def test_nested_dir(johndoe_client):
    await mk_folder(johndoe_client, '/bar')
    await mk_folder(johndoe_client, '/bar/sub')
    await mk_file(johndoe_client, '/bar/sub.txt')
    ret = await johndoe_client.send_cmd('stat', path='/bar')
    assert ret == {'children': ['sub', 'sub.txt'], 'status': 'ok', 'type': 'folder'}


async def test_move_file(johndoe_client):
    path = await mk_file(johndoe_client, path='/v1')
    new_path = '/v2'
    ret = await johndoe_client.send_cmd('move', src=path, dst=new_path)
    assert ret == {'status': 'ok'}
    ret = await johndoe_client.send_cmd('stat', path='/')
    assert ret == {'children': ['v2'], 'status': 'ok', 'type': 'folder'}


async def test_move_unknown_file(johndoe_client):
    ret = await johndoe_client.send_cmd('move', src='/unknown', dst='/whatever')
    assert ret == {'status': 'invalid_path', 'label': "Path `/unknown` doesn't exist"}


async def test_move_folder(johndoe_client):
    path = await mk_folder(johndoe_client, path='/v1')
    new_path = '/v2'
    ret = await johndoe_client.send_cmd('move', src=path, dst=new_path)
    assert ret == {'status': 'ok'}
    ret = await johndoe_client.send_cmd('stat', path='/')
    assert ret == {'children': ['v2'], 'status': 'ok', 'type': 'folder'}


async def test_delete_file(johndoe_client):
    path = await mk_file(johndoe_client)
    ret = await johndoe_client.send_cmd('delete', path=path)
    assert ret == {'status': 'ok'}
    ret = await johndoe_client.send_cmd('stat', path='/')
    assert ret == {'children': [], 'status': 'ok', 'type': 'folder'}

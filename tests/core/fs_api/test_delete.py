import pytest
from freezegun import freeze_time

from tests.common import connect_core


@pytest.mark.trio
async def test_delete_folder(core, alice):
    with freeze_time('2017-12-10T12:00:00'):
        await core.login(alice)
        await core.fs.root.create_folder('empty_dir')

    async with connect_core(core) as sock:
        with freeze_time('2017-12-10T12:50:30'):
            await sock.send({'cmd': 'delete', 'path': '/empty_dir'})
            rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Make sure the folder disappeared
        await sock.send({'cmd': 'stat', 'path': '/'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'base_version': 0,
            'created': '2017-12-10T12:00:00+00:00',
            'updated': '2017-12-10T12:50:30+00:00',
            'is_placeholder': False,
            'need_flush': False,
            'need_sync': True,
            'children': []
        }
        await sock.send({'cmd': 'stat', 'path': '/empty_dir'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/empty_dir` doesn't exists"}


@pytest.mark.trio
async def test_delete_non_empty_folder(core, alice):
    with freeze_time('2017-12-10T12:00:00'):
        await core.login(alice)
        dir_entry = await core.fs.root.create_folder('dir')
        await dir_entry.create_file('foo.txt')

    async with connect_core(core) as sock:
        with freeze_time('2017-12-10T12:50:30'):
            await sock.send({'cmd': 'delete', 'path': '/dir'})
            rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Make sure the folder disappeared
        await sock.send({'cmd': 'stat', 'path': '/'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'base_version': 0,
            'updated': '2017-12-10T12:50:30+00:00',
            'created': '2017-12-10T12:00:00+00:00',
            'is_placeholder': False,
            'need_flush': False,
            'need_sync': True,
            'children': []
        }
        await sock.send({'cmd': 'stat', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` doesn't exists"}
        # Children should have disappeared as well
        await sock.send({'cmd': 'stat', 'path': '/dir/foo.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` doesn't exists"}


@pytest.mark.trio
async def test_delete_file(core, alice):
    with freeze_time('2017-12-10T12:00:00'):
        await core.login(alice)
        dir_entry = await core.fs.root.create_folder('dir')
        await dir_entry.create_file('foo.txt')

    async with connect_core(core) as sock:
        with freeze_time('2017-12-10T12:50:30'):
            await sock.send({'cmd': 'delete', 'path': '/dir/foo.txt'})
            rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Make sure the file disappeared
        await sock.send({'cmd': 'stat', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'base_version': 0,
            'updated': '2017-12-10T12:50:30+00:00',
            'created': '2017-12-10T12:00:00+00:00',
            'is_placeholder': True,
            'need_flush': False,
            'need_sync': True,
            'children': []
        }
        await sock.send({'cmd': 'stat', 'path': '/dir/foo.txt'})
        rep = await sock.recv()
        assert rep == {
            'status': 'invalid_path',
            'reason': "Path `/dir/foo.txt` doesn't exists"
        }


@pytest.mark.trio
async def test_delete_unknow_file(core, alice):
    with freeze_time('2017-12-10T12:00:00'):
        await core.login(alice)
        dir_entry = await core.fs.root.create_folder('dir')
        await dir_entry.create_file('foo.txt')

    async with connect_core(core) as sock:
        await sock.send({'cmd': 'delete', 'path': '/dummy.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dummy.txt` doesn't exists"}
        await sock.send({'cmd': 'delete', 'path': '/dir/foo.txt/foo'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': 'Path `/dir/foo.txt` is not a directory'}

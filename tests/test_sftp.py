import pytest
import tempfile
import asyncssh

from parsec import vfs
from parsec.ui.sftp import ParsecSFTPServer


class Server(asyncssh.SSHServer):
    """Unit test SSH server"""

    def begin_auth(self, username):
        return False  # No auth is required


@pytest.yield_fixture
def tmpdir():
    with tempfile.TemporaryDirectory() as tmpdir:
        yield tmpdir


def _generate_async_key(wdir, name):
    key = asyncssh.generate_private_key('ssh-rsa')
    key_path = "%s/%s" % (wdir, name)
    key_pub_path = key_path + '.pub'
    key.write_private_key(key_path)
    key.write_public_key(key_pub_path)
    return key_path, key_pub_path


class Context:
    def __init__(self, host, port, loop, skey, skey_pub, ckey, ckey_pub):
        self.host = host
        self.port = port
        self.loop = loop
        self.skey = skey
        self.skey_pub = skey_pub
        self.ckey = ckey
        self.ckey_pub = ckey_pub
        self.server = self.connection = None
        self.vfs = vfs.VFSServiceInMemoryMock()
        self.vfs_client = vfs.LocalVFSClient(self.vfs)

    async def start(self):
        self.server = await asyncssh.create_server(
            Server,
            host=self.host,
            port=self.port,
            sftp_factory=lambda conn: ParsecSFTPServer(conn, self.vfs_client),
            server_host_keys=[self.skey],
            authorized_client_keys=[self.ckey_pub],
            loop=self.loop)
        self.connection, _ = await asyncssh.create_connection(
            None,
            # client_factory=asyncssh.SFTPClient,
            known_hosts=None,
            host=self.host, port=self.port,
            client_keys=self.ckey,
            loop=self.loop)
        client = await self.connection.start_sftp_client()
        return client

    async def finish(self):
        self.connection.close()
        self.server.close()
        await self.connection.wait_closed()
        await self.server.wait_closed()


@pytest.fixture
def ctx(event_loop, unused_tcp_port, tmpdir):
    skey, skey_pub = _generate_async_key(tmpdir, "skey")
    ckey, ckey_pub = _generate_async_key(tmpdir, "ckey")
    return Context('localhost', unused_tcp_port, event_loop, skey, skey_pub, ckey, ckey_pub)


@pytest.mark.asyncio
async def test_listdir(ctx):
    client = await ctx.start()
    try:
        ret = await client.listdir('/')
        assert ret == ['.', '..']
    finally:
        await ctx.finish()


@pytest.mark.asyncio
async def test_createfile(ctx):
    client = await ctx.start()
    try:
        async with client.open('/test.txt', 'wb') as file:
            await file.write(b'hello')
        async with client.open('/test.txt', 'rb') as file:
            assert await file.read() == b'hello'
    finally:
        await ctx.finish()


@pytest.mark.asyncio
async def test_deletefile(ctx):
    client = await ctx.start()
    try:
        ctx.vfs_client.create_file('/test.txt', content=b'hello')
        await client.remove('/test.txt')
        with pytest.raises(vfs.VFSFileNotFoundError):
            ctx.vfs_client.stat('/test.txt')
    finally:
        await ctx.finish()


@pytest.mark.asyncio
async def test_readfile(ctx):
    client = await ctx.start()
    try:
        ctx.vfs_client.create_file('/test.txt', content=b'hello')
        async with client.open('/test.txt', 'rb') as file:
            content = await file.read()
            assert content == b'hello'
            await file.seek(2)
            content = await file.read(2)
            assert content == b'll'
    finally:
        await ctx.finish()


@pytest.mark.asyncio
async def test_statfile(ctx):
    client = await ctx.start()
    try:
        ctx.vfs_client.create_file('/test.txt', content=b'hello')
        stat = await client.stat('/test.txt')
        assert stat.size == 5
    finally:
        await ctx.finish()


@pytest.mark.asyncio
async def test_writefile(ctx):
    client = await ctx.start()
    try:
        # Normal mode
        ctx.vfs_client.create_file('/test.txt', content=b'hello')
        async with client.open('/test.txt', 'wb') as file:
            await file.write(b'world')
        assert ctx.vfs_client.read_file('/test.txt').content == b'world'

        ctx.vfs_client.create_file('/test.txt', content=b'hello')
        async with client.open('/test.txt', 'wb') as file:
            await file.seek(3)
            await file.write(b'l no !')
        assert ctx.vfs_client.read_file('/test.txt').content == b'hell no !'

        ctx.vfs_client.create_file('/test.txt', content=b'hello')
        async with client.open('/test.txt', 'wb') as file:
            await file.write(b'123456789')
            await file.seek(3)
            await file.write(b'___')
            await file.seek(6)
            await file.write(b'___')
        assert ctx.vfs_client.read_file('/test.txt').content == b'123______'
    finally:
        await ctx.finish()


@pytest.mark.asyncio
async def test_writefile_appendmode(ctx):
    client = await ctx.start()
    try:
        # Normal mode
        ctx.vfs_client.create_file('/test.txt', content=b'hello')
        async with client.open('/test.txt', 'ab') as file:
            await file.write(b'world')
        assert ctx.vfs_client.read_file('/test.txt').content == b'helloworld'

        ctx.vfs_client.create_file('/test.txt', content=b'hello')
        async with client.open('/test.txt', 'ab') as file:
            # Append doesn't care about seek offset
            await file.seek(1)
            await file.write(b'wor')
            await file.seek(4)
            await file.write(b'ld')
        assert ctx.vfs_client.read_file('/test.txt').content == b'helloworld'
    finally:
        await ctx.finish()


@pytest.mark.asyncio
async def test_makedir(ctx):
    client = await ctx.start()
    try:
        await client.mkdir('/test')
        stat = ctx.vfs_client.stat('/test').stat
        assert stat.type == vfs.vfs_pb2.Stat.DIRECTORY
        # Now we can create files in the directory
        ctx.vfs_client.create_file('/test/file', content=b'whatever')
    finally:
        await ctx.finish()


@pytest.mark.asyncio
async def test_removedir(ctx):
    client = await ctx.start()
    try:
        ctx.vfs_client.make_dir('/not_empty')
        ctx.vfs_client.create_file('/not_empty/foo.txt', content=b'hello')
        ctx.vfs_client.create_file('/useless.txt', content=b'hello')
        ctx.vfs_client.make_dir('/empty')
        # Cannot remove non-empty dir
        with pytest.raises(asyncssh.SFTPError):
            await client.rmdir('/not_empty')
        await client.rmdir('/empty')
        with pytest.raises(vfs.VFSFileNotFoundError):
            ctx.vfs_client.stat('/empty')
    finally:
        await ctx.finish()

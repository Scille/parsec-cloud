import pytest
import tempfile
import asyncio
import asyncssh
from collections import namedtuple

from parsec.vfs import VFSService, LocalVFSClient
from parsec.volume import VolumeServiceInMemoryMock, LocalVolumeClient
from parsec.ui.sftp import SFTPUIServer, ParsecSFTPServer


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


def _sftp_server_factory(conn):
    volume_s = VolumeServiceInMemoryMock()
    volume_c = LocalVolumeClient(service=volume_s)
    vfs_s = VFSService(volume_c)
    vfs_c = LocalVFSClient(vfs_s)
    return ParsecSFTPServer(conn, vfs_c)


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

    async def start(self):
        self.server = await asyncssh.create_server(
            Server,
            host=self.host,
            port=self.port,
            sftp_factory=_sftp_server_factory,
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
async def test_simple(ctx):
    client = await ctx.start()
    try:
        ret = await client.listdir('/')
        assert ret == ['.', '..']
    finally:
        await ctx.finish()

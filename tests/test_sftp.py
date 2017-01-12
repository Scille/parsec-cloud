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


@pytest.fixture
def ctx(event_loop, unused_tcp_port, tmpdir):
    ctx = type('Ctx', (), {})()
    ctx.port = unused_tcp_port
    ctx.loop = event_loop
    ctx.host = 'localhost'
    ctx.skey, ctx.skey_pub = _generate_async_key(tmpdir, "skey")
    ctx.ckey, ctx.ckey_pub = _generate_async_key(tmpdir, "ckey")
    return ctx


@pytest.mark.asyncio
async def test_simple(ctx):

    def _sftp_server_factory(conn):
        volume_s = VolumeServiceInMemoryMock()
        volume_c = LocalVolumeClient(service=volume_s)
        vfs_s = VFSService(volume_c)
        vfs_c = LocalVFSClient(vfs_s)
        return ParsecSFTPServer(conn, vfs_c)

    server = await asyncssh.create_server(
        Server,
        host=ctx.host,
        port=ctx.port,
        sftp_factory=_sftp_server_factory,
        server_host_keys=[ctx.skey],
        authorized_client_keys=[ctx.ckey_pub],
        loop=ctx.loop)
    try:
        connection, _ = await asyncssh.create_connection(
            None,
            # client_factory=asyncssh.SFTPClient,
            known_hosts=None,
            host=ctx.host, port=ctx.port,
            client_keys=ctx.ckey,
            loop=ctx.loop)
        client = await connection.start_sftp_client()

        ret = await client.listdir('/')
        assert ret == ['.', '..']

        connection.close()
        await connection.wait_closed()
    finally:
        server.close()
        await server.wait_closed()

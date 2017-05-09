from os import environ
from socket import socket, AF_UNIX, SOCK_STREAM
import click

from parsec.server import UnixSocketServer, WebSocketServer
from parsec.backend import (InMemoryMessageService, MockedGroupService, MockedNamedVlobService,
                            MockedVlobService)
from parsec.core import (BackendAPIService, CryptoService, FileService, GNUPGPubKeysService,
                         IdentityService, MockedBlockService, ShareService, UserManifestService)
from parsec.ui.shell import start_shell


CORE_UNIX_SOCKET = '/tmp/parsec'


@click.group()
def cli():
    pass


@click.command()
@click.argument('id')
@click.argument('args', nargs=-1)
def cmd(id, args):
    sock = socket(AF_UNIX, SOCK_STREAM)
    sock.connect(CORE_UNIX_SOCKET)
    try:
        msg = '%s %s' % (id, args)
        sock.send(msg.encode())
        resp = sock.recv(4096)
        print(resp)
    finally:
        sock.close()


@click.command()
@click.option('--socket', '-s', default=CORE_UNIX_SOCKET,
              help='Path to the UNIX socket (default: %s).' % CORE_UNIX_SOCKET)
def shell(socket):
    start_shell(socket)


@click.command()
@click.argument('mountpoint', type=click.Path(exists=True, file_okay=False))
@click.option('--identity', '-i', type=click.STRING, default=None)
@click.option('--debug', '-d', is_flag=True, default=False)
@click.option('--nothreads', is_flag=True, default=False)
@click.option('--socket', '-s', default=CORE_UNIX_SOCKET,
              help='Path to the UNIX socket (default: %s).' % CORE_UNIX_SOCKET)
def fuse(mountpoint, identity, debug, nothreads, socket):
    # Do the import here in case fuse is not an available dependency
    from parsec.ui.fuse import start_fuse
    start_fuse(socket, mountpoint, identity, debug=debug, nothreads=nothreads)


@click.command()
@click.option('--socket', '-s', default=CORE_UNIX_SOCKET,
              help='Path to the UNIX socket exposing the core API (default: %s).' %
              CORE_UNIX_SOCKET)
@click.option('--backend-host', '-H', default='ws://localhost:6777')
@click.option('--block-store', '-B')
def core(socket, backend_host, block_store):
    server = UnixSocketServer()
    server.register_service(BackendAPIService(backend_host))
    if block_store:
        if block_store.startswith('s3:'):
            try:
                from parsec.core.block_service_s3 import S3BlockService
                _, region, bucket, key_id, key_secret = block_store.split(':')
            except ImportError as exc:
                raise SystemExit('Parsec needs boto3 to support S3 block storage (error: %s).' % exc)
            except ValueError:
                raise SystemExit('Invalid --block-store value (should be `s3:<region>:<bucket>:<id>:<secret>`.')
            block_svc = S3BlockService()
            block_svc.init(region, bucket, key_id, key_secret)
            store_type = 's3:%s:%s' % (region, bucket)
        else:
            raise SystemExit('Unknown block store `%s` (only `s3:<region>:<bucket>:<id>:<secret>`'
                             ' is supported so far.' % block_store)
    else:
        store_type = 'mocked in memory'
        block_svc = MockedBlockService()
    server.register_service(block_svc)
    server.register_service(CryptoService())
    server.register_service(FileService())
    server.register_service(GNUPGPubKeysService())
    server.register_service(IdentityService())
    server.register_service(ShareService())
    server.register_service(UserManifestService())
    print('Starting parsec core on %s (connecting to backend %s and block store %s)' % (socket, backend_host, store_type))
    server.start(socket)
    print('Bye ;-)')


@click.command()
@click.option('--gnupg-homedir', default='~/.gnupg')
@click.option('--host', '-H', default=None, help='Host to listen on (default: localhost)')
@click.option('--port', '-P', default=None, type=int, help=('Port to listen on (default: 6777)'))
@click.option('--no-client-auth', is_flag=True,
              help='Disable authentication handshake on client connection (default: false)')
@click.option('--store', '-s', default=None, help="Store configuration (default: in memory)")
def backend(host, port, gnupg_homedir, no_client_auth, store):
    host = host or environ.get('HOST', 'localhost')
    port = port or int(environ.get('PORT', 6777))
    pub_keys_service = GNUPGPubKeysService(gnupg_homedir)
    if no_client_auth:
        server = WebSocketServer()
    else:
        server = WebSocketServer(pub_keys_service.handshake)
    server.register_service(pub_keys_service)
    if store:
        if store.startswith('postgres://'):
            store_type = 'PostgreSQL'
            from parsec.backend import postgresql
            server.register_service(postgresql.PostgreSQLService(store))
            server.register_service(postgresql.PostgreSQLMessageService())
            server.register_service(postgresql.PostgreSQLGroupService())
            server.register_service(postgresql.PostgreSQLNamedVlobService())
            server.register_service(postgresql.PostgreSQLVlobService())
        else:
            raise SystemExit('Unknown store `%s` (should be a postgresql db url).' % store)
    else:
        store_type = 'mocked in memory'
        server.register_service(InMemoryMessageService())
        server.register_service(MockedGroupService())
        server.register_service(MockedNamedVlobService())
        server.register_service(MockedVlobService())
    print('Starting parsec backend on %s:%s with store %s' % (host, port, store_type))
    server.start(host, port)
    print('Bye ;-)')


cli.add_command(cmd)
cli.add_command(fuse)
cli.add_command(shell)
cli.add_command(core)
cli.add_command(backend)

try:
    from parsec.backend.postgresql import cli as postgresql_cli
    cli.add_command(postgresql_cli, 'postgresql')
except ImportError:
    raise


if __name__ == '__main__':
    cli()

from os import environ
from importlib import import_module
import asyncio
import click
from logbook import WARNING

from parsec.tools import logger_stream
from parsec.server import UnixSocketServer, WebSocketServer
from parsec.backend import (InMemoryMessageService, MockedGroupService, MockedUserVlobService,
                            MockedVlobService, InMemoryPubKeyService)
# from parsec.core import (BackendAPIService, CryptoService, FileService, GNUPGPubKeysService,
#                          IdentityService, MockedBlockService, MockedCacheService, ShareService,
#                          UserManifestService)
from parsec.core2 import CoreService, BackendAPIService, MockedBlockService, IdentityService
from parsec.ui.shell import start_shell


# TODO: remove me once RSA key loading and backend handling are easier
JOHN_DOE_IDENTITY = 'John_Doe'
JOHN_DOE_PRIVATE_KEY = b"""
-----BEGIN PRIVATE KEY-----
MIIBVQIBADANBgkqhkiG9w0BAQEFAASCAT8wggE7AgEAAkEAsSle/x4jtr+kaxiv
9BYlL+gffH/VLC+Q/WTTB+1FIU1fdmgdZVGaIlAWJHqr9qEZBfwXYzlQv8pIMn+W
5pqvHQICAQECQDv5FjKAvWW1a3Twc1ia67eQUmDugu8VFTDsV2BRUS0jlxJ0yCL+
TEBpOwH95TFgvfRBYee97APHjhvLLlzmEyECIQDZdjMg/j9N7sw602DER0ciERPI
Ps9rU8RqRXaWPYtWOQIhANCO1h/z7iFjlpENbKDOCinfsXd9ulVsoNYWhKm58gAF
AiEAzMT3XdKFUlljq+/hl/Nt0GPA8lkHDGjG5ZAaAAYnj/ECIQCB125lkuHy61LH
4INhH6azeFaUGnn7aHwJxE6beL6BgQIhALbajJWsBf5LmeO190adM2jAVN94YqVD
aOrHGFFqrjJ3
-----END PRIVATE KEY-----
"""
JOHN_DOE_PUBLIC_KEY = b"""
-----BEGIN PUBLIC KEY-----
MFswDQYJKoZIhvcNAQEBBQADSgAwRwJBALEpXv8eI7a/pGsYr/QWJS/oH3x/1Swv
kP1k0wftRSFNX3ZoHWVRmiJQFiR6q/ahGQX8F2M5UL/KSDJ/luaarx0CAgEB
-----END PUBLIC KEY-----
"""

DEFAULT_CORE_UNIX_SOCKET = '/tmp/parsec'


@click.group()
def cli():
    pass


@click.command()
@click.argument('id')
@click.argument('args', nargs=-1)
@click.option('socket_path', '--socket', '-s', default=DEFAULT_CORE_UNIX_SOCKET,
              help='Path to the UNIX socket (default: %s).' % DEFAULT_CORE_UNIX_SOCKET)
def cmd(id, args, socket_path):
    from socket import socket, AF_UNIX, SOCK_STREAM
    sock = socket(AF_UNIX, SOCK_STREAM)
    sock.connect(socket)
    try:
        msg = '%s %s' % (id, args)
        sock.send(msg.encode())
        resp = sock.recv(4096)
        print(resp)
    finally:
        sock.close()


@click.command()
@click.option('--socket', '-s', default=DEFAULT_CORE_UNIX_SOCKET,
              help='Path to the UNIX socket (default: %s).' % DEFAULT_CORE_UNIX_SOCKET)
def shell(socket):
    start_shell(socket)


@click.command()
@click.option('--socket', '-s', default=DEFAULT_CORE_UNIX_SOCKET,
              help='Path to the UNIX socket exposing the core API (default: %s).' %
              DEFAULT_CORE_UNIX_SOCKET)
@click.option('--backend-host', '-H', default='ws://localhost:6777')
@click.option('--backend-watchdog', '-W', type=click.INT, default=None)
@click.option('--block-store', '-B')
@click.option('--debug', '-d', is_flag=True)
@click.option('--identity', '-i', default=None)
@click.option('--identity-key', '-I', type=click.File(), default=None)
@click.option('--I-am-John', is_flag=True, help='Log as dummy John Doe user')
def core(socket, backend_host, backend_watchdog, block_store, debug, identity, identity_key, i_am_john):
    loop = asyncio.get_event_loop()
    server = UnixSocketServer()
    server.register_service(BackendAPIService(backend_host, backend_watchdog))
    if block_store:
        if block_store.startswith('s3:'):
            try:
                from parsec.core.block_service_s3 import S3BlockService
                _, region, bucket, key_id, key_secret = block_store.split(':')
            except ImportError as exc:
                raise SystemExit('Parsec needs boto3 to support S3 block storage (error: %s).' %
                                 exc)
            except ValueError:
                raise SystemExit('Invalid --block-store value '
                                 ' (should be `s3:<region>:<bucket>:<id>:<secret>`.')
            block_svc = S3BlockService(region, bucket, key_id, key_secret)
            store_type = 's3:%s:%s' % (region, bucket)
        else:
            raise SystemExit('Unknown block store `%s` (only `s3:<region>:<bucket>:<id>:<secret>`'
                             ' is supported so far.' % block_store)
    else:
        store_type = 'mocked in memory'
        block_svc = MockedBlockService()
    server.register_service(block_svc)
    identity_svc = IdentityService()
    server.register_service(identity_svc)
    if (identity or identity_key) and (not identity or not identity_key):
        raise SystemExit('--identity and --identity-key params should be provided together.')
    # TODO: remove me once RSA key loading and backend handling are easier
    if i_am_john:
        identity = JOHN_DOE_IDENTITY
        from io import BytesIO
        identity_key = BytesIO(JOHN_DOE_PRIVATE_KEY)
    if identity:
        @server.post_bootstrap
        async def post_bootstrap():
            await identity_svc.load(identity, identity_key.read())
    server.register_service(CoreService())
    if debug:
        loop.set_debug(True)
    else:
        logger_stream.level = WARNING
    print('Starting parsec core on %s (connecting to backend %s and block store %s)' %
          (socket, backend_host, store_type))
    server.start(socket, loop=loop)
    print('Bye ;-)')


@click.command()
@click.option('--pubkeys', default=None)
@click.option('--host', '-H', default=None, help='Host to listen on (default: localhost)')
@click.option('--port', '-P', default=None, type=int, help=('Port to listen on (default: 6777)'))
@click.option('--no-client-auth', is_flag=True,
              help='Disable authentication handshake on client connection (default: false)')
@click.option('--store', '-s', default=None, help="Store configuration (default: in memory)")
@click.option('--debug', '-d', is_flag=True)
def backend(host, port, pubkeys, no_client_auth, store, debug):
    host = host or environ.get('HOST', 'localhost')
    port = port or int(environ.get('PORT', 6777))
    # TODO load pubkeys attribute
    pubkey_svc = InMemoryPubKeyService()
    if no_client_auth:
        server = WebSocketServer()
    else:
        server = WebSocketServer(pubkey_svc.handshake)
    server.register_service(pubkey_svc)
    if store:
        if store.startswith('postgres://'):
            store_type = 'PostgreSQL'
            from parsec.backend import postgresql
            server.register_service(postgresql.PostgreSQLService(store))
            server.register_service(postgresql.PostgreSQLMessageService())
            server.register_service(postgresql.PostgreSQLGroupService())
            server.register_service(postgresql.PostgreSQLUserVlobService())
            server.register_service(postgresql.PostgreSQLVlobService())
        else:
            raise SystemExit('Unknown store `%s` (should be a postgresql db url).' % store)
    else:
        store_type = 'mocked in memory'
        server.register_service(InMemoryMessageService())
        server.register_service(MockedGroupService())
        server.register_service(MockedUserVlobService())
        server.register_service(MockedVlobService())
    loop = asyncio.get_event_loop()

    # TODO: remove me once RSA key loading and backend handling are easier
    @server.post_bootstrap
    async def post_boostrap():
        await pubkey_svc.add_pubkey(JOHN_DOE_IDENTITY, JOHN_DOE_PUBLIC_KEY)
    if debug:
        loop.set_debug(True)
    else:
        logger_stream.level = WARNING
    print('Starting parsec backend on %s:%s with store %s' % (host, port, store_type))
    server.start(host, port, loop=loop)
    print('Bye ;-)')


cli.add_command(cmd)
cli.add_command(shell)
cli.add_command(core)
cli.add_command(backend)


def _add_command_if_can_import(path, name=None):
    module_path, field = path.rsplit('.', 1)
    try:
        module = import_module(module_path)
        cli.add_command(getattr(module, field), name=name)
    except (ImportError, AttributeError):
        pass


_add_command_if_can_import('parsec.backend.postgresql.cli', 'postgresql')
_add_command_if_can_import('parsec.ui.fuse.cli', 'fuse')


if __name__ == '__main__':
    cli()

from os import environ
import pdb
import sys
import traceback
from importlib import import_module
from getpass import getpass
import asyncio
import click
from logbook import WARNING
from effect2 import Effect, do

from parsec.backend import app_factory as backend_app_factory, run_app as backend_run_app
from parsec.backend.pubkey import MockedPubKeyComponent, EPubKeyAdd, EPubKeyGet
from parsec.backend.vlob import MockedVlobComponent
from parsec.backend.user_vlob import MockedUserVlobComponent
from parsec.backend.message import InMemoryMessageComponent
from parsec.backend.group import MockedGroupComponent
from parsec.core import app_factory as core_app_factory, run_app as core_run_app
from parsec.core.backend import BackendComponent
from parsec.core.identity import IdentityComponent
from parsec.core.fs import FSComponent
from parsec.core.privkey import PrivKeyComponent
from parsec.core.synchronizer import SynchronizerComponent
from parsec.core.block import in_memory_block_dispatcher_factory, s3_block_dispatcher_factory
from parsec.core.identity import EIdentityLoad
from parsec.exceptions import PubKeyNotFound
from parsec.ui.shell import start_shell
from parsec.tools import logger_stream


# TODO: remove me once RSA key loading and backend handling are easier
JOHN_DOE_IDENTITY = 'John_Doe'
JOHN_DOE_PRIVATE_KEY = b"""
-----BEGIN RSA PRIVATE KEY-----
MIICXQIBAAKBgQDCqVQVdVhJqW9rrbObvDZ4ET6FoIyVn6ldWhOJaycMeFYBN3t+
cGr9/xHPGrYXK63nc8x4IVxhfXZ7JwrQ+AJv935S3rAV6JhDKDfDFrkzUVZmcc/g
HhjiP7rTAt4RtACvhZwrDuj3Pc4miCpGN/T3tbOKG889JN85nABKR9WkdwIDAQAB
AoGBAJFU3Dr9FgJA5rfMwpiV51CzByu61trqjgbtNkLVZhzwRr23z5Jxmd+yLHik
J6ia6sYvdUuHFLKQegGt/2xOjXn8UBpa725gLojHn2umtJDL7amTlBwiJfNXuZrF
BSKK9+xZnNDWMq1IuCqPeintbve+MNSc62JYuGGtXSz9L5f5AkEA/xBkUksBfEUl
65oEPgxvMKHNjLq48otRmCaG+i3MuQqTYQ+c8Z/l26yQL4OV2b36a8/tTaLhwhAZ
Ibtv05NKfQJBAMNgMbOsUWpY8A1Cec79Oj6RVe79E5ciZ4mW3lx5tjJRyNxwlQag
u+T6SwBIa6xMfLBQeizzxqXqxAyW/riQ6QMCQQCadUu7Re6tWZaAGTGufYsr8R/v
s/dh8ZpEwDgG8otCFzRul6zb6Y+huttJ2q55QIGQnka/N/7srSD6+23Zux1lAkBx
P30PzL6UimD7DqFUnev5AH1zPjbwz/x8AHt71wEJQebQAGIhqWHAZGS9ET14bg2I
ld172QI4glCJi6yyhyzJAkBzfmHZEE8FyLCz4z6b+Z2ghMds2Xz7RwgVqCIXt9Ku
P7Bq0eXXgyaBo+jpr3h4K7QnPh+PaHSlGqSfczZ6GIpx
-----END RSA PRIVATE KEY-----
"""
JOHN_DOE_PUBLIC_KEY = b"""
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDCqVQVdVhJqW9rrbObvDZ4ET6F
oIyVn6ldWhOJaycMeFYBN3t+cGr9/xHPGrYXK63nc8x4IVxhfXZ7JwrQ+AJv935S
3rAV6JhDKDfDFrkzUVZmcc/gHhjiP7rTAt4RtACvhZwrDuj3Pc4miCpGN/T3tbOK
G889JN85nABKR9WkdwIDAQAB
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
def cmd(id, args, socket_path, per_cmd_connection):
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


def run_with_pdb(cmd, *args, **kwargs):
    # Stolen from pdb.main
    pdb_context = pdb.Pdb()
    try:
        ret = pdb_context.runcall(cmd, **kwargs)
        print("The program finished")
        return ret
    except pdb.Restart:
        print("Restarting %s with arguments: %s, %s" % (cmd.__name__, args, kwargs))
        # Yes, that's a hack
        return run_with_pdb(cmd, *args, **kwargs)
    except SystemExit:
        # In most cases SystemExit does not warrant a post-mortem session.
        print("The program exited via sys.exit(). Exit status:", end=' ')
        print(sys.exc_info()[1])
    except SyntaxError:
        traceback.print_exc()
        sys.exit(1)
    except:
        traceback.print_exc()
        print("Uncaught exception. Entering post mortem debugging")
        print("Running 'cont' or 'step' will restart the program")
        t = sys.exc_info()[2]
        pdb_context.interaction(None, t)
        print("Post mortem debugger finished.")


@click.command()
@click.option('--socket', '-s', default=DEFAULT_CORE_UNIX_SOCKET,
              help='Path to the UNIX socket exposing the core API (default: %s).' %
              DEFAULT_CORE_UNIX_SOCKET)
@click.option('--backend-host', '-H', default='ws://localhost:6777')
@click.option('--backend-watchdog', '-W', type=click.INT, default=None)
@click.option('--block-store', '-B')
@click.option('--debug', '-d', is_flag=True)
@click.option('--pdb', is_flag=True)
@click.option('--identity', '-i', default=None)
@click.option('--identity-key', '-I', type=click.File('rb'), default=None)
@click.option('--I-am-John', is_flag=True, help='Log as dummy John Doe user')
def core(**kwargs):
    if kwargs.pop('pdb'):
        return run_with_pdb(_core, **kwargs)
    else:
        return _core(**kwargs)


def _core(socket, backend_host, backend_watchdog, block_store,
          debug, identity, identity_key, i_am_john):
    loop = asyncio.get_event_loop()
    if block_store:
        if block_store.startswith('s3:'):
            try:
                # from parsec.core.block_service_s3 import S3BlockService
                _, region, bucket, key_id, key_secret = block_store.split(':')
            except ImportError as exc:
                raise SystemExit('Parsec needs boto3 to support S3 block storage (error: %s).' %
                                 exc)
            except ValueError:
                raise SystemExit('Invalid --block-store value '
                                 ' (should be `s3:<region>:<bucket>:<id>:<secret>`.')
            raise NotImplementedError('Not yet :-(')
            block_dispatcher = s3_block_dispatcher_factory(region, bucket, key_id, key_secret)
            store_type = 's3:%s:%s' % (region, bucket)
        else:
            raise SystemExit('Unknown block store `%s` (only `s3:<region>:<bucket>:<id>:<secret>`'
                             ' is supported so far.' % block_store)
    else:
        store_type = 'mocked in memory'
        block_dispatcher = in_memory_block_dispatcher_factory()
    privkey_component = PrivKeyComponent()
    backend_component = BackendComponent(backend_host, backend_watchdog)
    fs_component = FSComponent()
    identity_component = IdentityComponent()
    synchronizer_component = SynchronizerComponent()
    app = core_app_factory(
        privkey_component.get_dispatcher(), backend_component.get_dispatcher(),
        fs_component.get_dispatcher(), synchronizer_component.get_dispatcher(),
        identity_component.get_dispatcher(), block_dispatcher)
    if (identity or identity_key) and (not identity or not identity_key):
        raise SystemExit('--identity and --identity-key params should be provided together.')
    # TODO: remove me once RSA key loading and backend handling are easier
    if i_am_john:
        @do
        def load_identity():
            yield Effect(EIdentityLoad(JOHN_DOE_IDENTITY, JOHN_DOE_PRIVATE_KEY))
            print('Welcome back M. Doe')
        loop.run_until_complete(app.async_perform(load_identity()))
    elif identity:
        @do
        def load_identity():
            password = getpass()
            yield Effect(EIdentityLoad(identity, identity_key.read(), password))
            print('Connected as %s' % identity)
        loop.run_until_complete(app.async_perform(load_identity()))
    if debug:
        loop.set_debug(True)
    else:
        logger_stream.level = WARNING
    print('Starting parsec core on %s (connecting to backend %s and block store %s)' %
          (socket, backend_host, store_type))
    core_run_app(socket, app=app, loop=loop)
    print('Bye ;-)')


@click.command()
@click.option('--pubkeys', default=None)
@click.option('--host', '-H', default=None, help='Host to listen on (default: localhost)')
@click.option('--port', '-P', default=None, type=int, help=('Port to listen on (default: 6777)'))
@click.option('--no-client-auth', is_flag=True,
              help='Disable authentication handshake on client connection (default: false)')
@click.option('--store', '-s', default=None, help="Store configuration (default: in memory)")
@click.option('--debug', '-d', is_flag=True)
@click.option('--pdb', is_flag=True)
def backend(**kwargs):
    if kwargs.pop('pdb'):
        return run_with_pdb(_backend, **kwargs)
    else:
        return _backend(**kwargs)


def _backend(host, port, pubkeys, no_client_auth, store, debug):
    host = host or environ.get('HOST', 'localhost')
    port = port or int(environ.get('PORT', 6777))
    loop = asyncio.get_event_loop()
    # TODO load pubkeys attribute
    if store:
        if store.startswith('postgres://'):
            store_type = 'PostgreSQL'
            from parsec.backend import postgresql
            conn = postgresql.PostgreSQLConnection(store)
            loop.run_until_complete(conn.open_connection())
            message_component = postgresql.PostgreSQLMessageComponent(conn)
            group_component = postgresql.PostgreSQLGroupComponent(conn)
            user_vlob_component = postgresql.PostgreSQLUserVlobComponent(conn)
            vlob_component = postgresql.PostgreSQLVlobComponent(conn)
            pubkey_component = postgresql.PostgreSQLPubKeyComponent(conn)
        else:
            raise SystemExit('Unknown store `%s` (should be a postgresql db url).' % store)
    else:
        store_type = 'mocked in memory'
        message_component = InMemoryMessageComponent()
        group_component = MockedGroupComponent()
        user_vlob_component = MockedUserVlobComponent()
        vlob_component = MockedVlobComponent()
        pubkey_component = MockedPubKeyComponent()
    app = backend_app_factory(
        message_component.get_dispatcher(),
        group_component.get_dispatcher(),
        user_vlob_component.get_dispatcher(),
        vlob_component.get_dispatcher(),
        pubkey_component.get_dispatcher()
    )

    # TODO: remove me once RSA key loading and backend handling are easier
    @do
    def insert_john():
        try:
            yield Effect(EPubKeyGet(JOHN_DOE_IDENTITY))
        except PubKeyNotFound:
            yield Effect(EPubKeyAdd(JOHN_DOE_IDENTITY, JOHN_DOE_PUBLIC_KEY))
    loop.run_until_complete(app.async_perform(insert_john()))

    if debug:
        loop.set_debug(True)
    else:
        logger_stream.level = WARNING
    print('Starting parsec backend on %s:%s with store %s' % (host, port, store_type))
    backend_run_app(host, port, app=app, loop=loop)
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

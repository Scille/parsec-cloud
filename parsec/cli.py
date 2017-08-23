from os import environ
import pdb
import sys
import traceback
from importlib import import_module
from getpass import getpass
import asyncio
import click
from logbook import WARNING
from aiohttp import web
from effect2 import Effect, asyncio_perform

from parsec import unix_socket_app
from parsec.backend import (
    postgresql_components_factory, mocked_components_factory,
    register_backend_api, register_start_api, register_in_memory_block_store_api
)
from parsec.core import (
    components_factory as core_components_factory,
    register_core_api
)
from parsec.exceptions import PubKeyNotFound, PrivKeyNotFound
from parsec.ui.shell import start_shell
from parsec.crypto import generate_asym_key
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
@click.option('--debug', '-d', is_flag=True)
@click.option('--pdb', is_flag=True)
@click.option('--identity', '-i', default=None)
@click.option('--identity-key', '-I', type=click.File('rb'), default=None)
@click.option('--I-am-John', is_flag=True, help='Log as dummy John Doe user')
@click.option('--cache-size', help='Max number of elements in cache', default=1000)
def core(**kwargs):
    if kwargs.pop('pdb'):
        return run_with_pdb(_core, **kwargs)
    else:
        return _core(**kwargs)


def _core(socket, backend_host, backend_watchdog,
          debug, identity, identity_key, i_am_john, cache_size):
    app = unix_socket_app.UnixSocketApplication()
    components = core_components_factory(app, backend_host, backend_watchdog, cache_size)
    dispatcher = components.get_dispatcher()
    register_core_api(app, dispatcher)

    # TODO: remove me once RSA key loading and backend handling are easier
    if i_am_john:
        async def load_identity(app):
            from parsec.core.identity import EIdentityLoad
            eff = Effect(EIdentityLoad(JOHN_DOE_IDENTITY, JOHN_DOE_PRIVATE_KEY))
            await asyncio_perform(dispatcher, eff)
            print('Welcome back M. Doe')
        app.on_startup.append(load_identity)
    elif identity:
        async def load_identity(app):
            from parsec.core.identity import EIdentityLoad, EIdentityLogin
            if identity_key:
                print("Reading %s's key from `%s`" % (identity, identity_key))
                password = getpass()
                eff = Effect(EIdentityLoad(identity, identity_key.read(), password))
                await asyncio_perform(dispatcher, eff)
            else:
                print("Fetching %s's key from backend privkey store." % (identity))
                password = getpass()
                eff = Effect(EIdentityLogin(identity, password))
                await asyncio_perform(dispatcher, eff)
            print('Connected as %s' % identity)
        app.on_startup.append(load_identity)

    if debug:
        loop = asyncio.get_event_loop()
        loop.set_debug(True)
    else:
        logger_stream.level = WARNING

    print('Starting parsec core on %s (connecting to backend %s)' % (socket, backend_host))
    unix_socket_app.run_app(app, path=socket)
    print('Bye ;-)')


@click.command()
@click.option('--pubkeys', default=None)
@click.option('--host', '-H', default=None, help='Host to listen on (default: localhost)')
@click.option('--port', '-P', default=None, type=int, help=('Port to listen on (default: 6777)'))
@click.option('--no-client-auth', is_flag=True,
              help='Disable authentication handshake on client connection (default: false)')
@click.option('--store', '-s', default=None, help="Store configuration (default: in memory)")
@click.option('--block-store', '-b', default=None,
    help="URL of the block store the clients should write into (default: "
    "backend creates it own in-memory block store).")
@click.option('--debug', '-d', is_flag=True)
@click.option('--pdb', is_flag=True)
def backend(**kwargs):
    if kwargs.pop('pdb'):
        return run_with_pdb(_backend, **kwargs)
    else:
        return _backend(**kwargs)


def _backend(host, port, pubkeys, no_client_auth, store, block_store, debug):
    host = host or environ.get('HOST', 'localhost')
    port = port or int(environ.get('PORT', 6777))
    app = web.Application()
    if not block_store:
        block_store = '/blockstore'
        register_in_memory_block_store_api(app, prefix=block_store)
    if store:
        if store.startswith('postgres://'):
            store_type = 'PostgreSQL'
            backend_components = postgresql_components_factory(app, store, block_store)
        else:
            raise SystemExit('Unknown store `%s` (should be a postgresql db url).' % store)
    else:
        store_type = 'mocked in memory'
        backend_components = mocked_components_factory(block_store)

    dispatcher = backend_components.get_dispatcher()
    register_backend_api(app, dispatcher)
    register_start_api(app, dispatcher)

    if debug:
        loop = asyncio.get_event_loop()
        loop.set_debug(True)
    else:
        logger_stream.level = WARNING

    # TODO: remove me once RSA key loading and backend handling are easier
    async def insert_john(app):
        from parsec.backend.pubkey import EPubKeyGet, EPubKeyAdd
        dispatcher = backend_components.get_dispatcher()
        try:
            await asyncio_perform(dispatcher, Effect(EPubKeyGet(JOHN_DOE_IDENTITY)))
        except PubKeyNotFound:
            await asyncio_perform(
                dispatcher, Effect(EPubKeyAdd(JOHN_DOE_IDENTITY, JOHN_DOE_PUBLIC_KEY)))
    app.on_startup.append(insert_john)

    print('Starting parsec backend on %s:%s with store %s' % (host, port, store_type))
    web.run_app(app, host=host, port=port)
    print('Bye ;-)')


@click.command()
@click.option('--socket', '-s', default=DEFAULT_CORE_UNIX_SOCKET,
              help='Path to the UNIX socket (default: %s).' % DEFAULT_CORE_UNIX_SOCKET)
@click.option('--identity', '-i', required=True)
@click.option('--key-size', '-S', type=int, default=2048)
def signup(socket, identity, key_size):
    while True:
        password = getpass('Password:')
        repassword = getpass('Confirm password:')
        if password == repassword:
            break
        print('Passwords missmatch, please retry')

    import asyncio
    from parsec.tools import ejson_loads, ejson_dumps

    async def run():
        try:
            reader, writer = await asyncio.open_unix_connection(path=socket)
        except (FileNotFoundError, ConnectionRefusedError):
            raise SystemExit('ERROR: Cannot connect to parsec core at %s' % socket)
        msg = {
            'cmd': 'identity_signup',
            'id': identity,
            'password': password,
            'key_size': key_size
        }
        writer.write(ejson_dumps(msg).encode())
        writer.write(b'\n')
        raw_resp = await reader.readline()
        resp = ejson_loads(raw_resp.decode())
        writer.close()
        print(resp)

    loop = asyncio.get_event_loop()
    loop.run_until_complete(run())

cli.add_command(cmd)
cli.add_command(shell)
cli.add_command(core)
cli.add_command(backend)
cli.add_command(signup)


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

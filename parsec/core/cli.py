import trio
import click

from .app import CoreApp
from .config import CONFIG


JOHN_DOE_IDENTITY = 'johndoe@test'
JOHN_DOE_PRIVATE_KEY = b']x\xd3\xa9$S\xa92\x9ex\x91\xa7\xee\x04SY\xbe\xe6\x03\xf0\x1d\xe2\xcc7\x8a\xd7L\x137\x9e\xa7\xc6'
DEFAULT_CORE_UNIX_SOCKET = 'tcp://127.0.0.1:6776'


def run_with_pdb(cmd, *args, **kwargs):
    import pdb, traceback, sys
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
@click.option('--backend-host', '-H', default='tcp://127.0.0.1:6777')
@click.option('--backend-watchdog', '-W', type=click.INT, default=None)
@click.option('--debug', '-d', is_flag=True)
@click.option('--pdb', is_flag=True)
# @click.option('--identity', '-i', default=None)
# @click.option('--identity-key', '-I', type=click.File('rb'), default=None)
@click.option('--I-am-John', is_flag=True, help='Log as dummy John Doe user')
# @click.option('--cache-size', help='Max number of elements in cache', default=1000)
def core_cmd(**kwargs):
    if kwargs.pop('pdb'):
        return run_with_pdb(_core, **kwargs)
    else:
        return _core(**kwargs)


def _core(socket, backend_host, backend_watchdog, debug, i_am_john):
    # TODO: so far LocalStorage is not implemented, so use the testing mock...
    from . import fs
    from tests.common import mocked_local_storage_cls_factory
    fs.LocalStorage = mocked_local_storage_cls_factory()
    config = {
        **CONFIG,
        'DEBUG': debug,
        'BACKEND_ADDR': backend_host,
        'BACKEND_WATCHDOG': backend_watchdog,
        'ADDR': socket
    }
    core = CoreApp(config)

    async def _run_and_login(identity, rawkey):
        async def _login_on_ready():
            await core.server_ready.wait()
            await core.login(identity, rawkey)
            print('Logged as %s' % identity)

        async with trio.open_nursery() as nursery:
            nursery.start_soon(core.run)
            nursery.start_soon(_login_on_ready)

    print('Starting Parsec Core on %s (with backend on %s)' %
        (config['ADDR'], config['BACKEND_ADDR']))
    try:
        if i_am_john:
            # TODO: well well well...
            from tests.common import User
            from tests.populate_local_storage import populate_local_storage_cls
            from nacl.public import PrivateKey
            populate_local_storage_cls(User(JOHN_DOE_IDENTITY, PrivateKey(JOHN_DOE_PRIVATE_KEY)), local_fs.LocalStorage)

            trio.run(_run_and_login, JOHN_DOE_IDENTITY, JOHN_DOE_PRIVATE_KEY)
        else:
            trio.run(core.run)
    except KeyboardInterrupt:
        print('bye ;-)')

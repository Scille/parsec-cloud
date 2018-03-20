import trio
import os.path
import click
import shutil
import tempfile
import logbook
from urllib.parse import urlparse

from parsec.core import CoreApp, CoreConfig, Device


JOHN_DOE_DEVICE_ID = 'johndoe@test'
JOHN_DOE_PRIVATE_KEY = (b']x\xd3\xa9$S\xa92\x9ex\x91\xa7\xee\x04SY\xbe\xe6'
                        b'\x03\xf0\x1d\xe2\xcc7\x8a\xd7L\x137\x9e\xa7\xc6')
JOHN_DOE_DEVICE_SIGNING_KEY = (b'w\xac\xd8\xb4\x88B:i\xd6G\xb9\xd6\xc5\x0f\xf6\x99'
                               b'\xccH\xfa\xaeY\x00:\xdeP\x84\t@\xfe\xf8\x8a\xa5')
DEFAULT_CORE_SOCKET = 'tcp://127.0.0.1:6776'


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
@click.option('--socket', '-s', default=DEFAULT_CORE_SOCKET,
              help=('Socket path (`tcp://<domain:port>` or `unix://<path>`) '
                    'exposing the core API (default: %s).' % DEFAULT_CORE_SOCKET))
@click.option('--backend-addr', '-A', default='tcp://127.0.0.1:6777')
@click.option('--backend-watchdog', '-W', type=click.INT, default=None)
@click.option('--debug', '-d', is_flag=True)
@click.option('--log-level', '-l', default='WARNING',
              type=click.Choice(('DEBUG', 'INFO', 'WARNING', 'ERROR')))
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


def _core(socket, backend_addr, backend_watchdog, debug, log_level, i_am_john):
    log_handler = logbook.StderrHandler(level=log_level.upper())
    # Push globally the log handler make it work across threads
    log_handler.push_application()
    config = CoreConfig(
        debug=debug,
        addr=socket,
        backend_addr=backend_addr,
        backend_watchdog=backend_watchdog,
        auto_sync=True
    )
    core = CoreApp(config)

    async def _login_and_run(user=None):
        async with trio.open_nursery() as nursery:
            await core.init(nursery)
            try:

                if user:
                    await core.login(user)
                    print('Logged as %s' % user.id)

                if socket.startswith('unix://'):
                    await trio.serve_unix(core.handle_client, socket[len('unix://'):])
                elif socket.startswith('tcp://'):
                    parsed = urlparse(socket)
                    await trio.serve_tcp(core.handle_client, parsed.port, host=parsed.hostname)
                else:
                    raise SystemExit('Error: Invalid --socket value `%s`' % socket)

            finally:
                await core.shutdown()

    print('Starting Parsec Core on %s (with backend on %s)' %
        (socket, config.backend_addr))
    try:
        if i_am_john:
            john_conf_dir = tempfile.mkdtemp(prefix='parsec-jdoe-conf-')
            try:
                user = Device(
                    JOHN_DOE_DEVICE_ID,
                    JOHN_DOE_DEVICE_SIGNING_KEY,
                    JOHN_DOE_DEVICE_SIGNING_KEY,
                    os.path.join(john_conf_dir, 'db.sqlite')
                )
                print('Hello Mr. Doe, your conf dir is `%s`' % john_conf_dir)
                trio.run(_login_and_run, user)
            finally:
                shutil.rmtree(john_conf_dir)
        else:
            trio.run(_login_and_run)
    except KeyboardInterrupt:
        print('bye ;-)')

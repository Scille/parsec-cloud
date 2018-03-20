import trio
import click
import logbook

from parsec.backend import BackendApp, BackendConfig
from parsec.backend.user import NotFoundError


JOHN_DOE_USER_ID = 'johndoe'
JOHN_DOE_PUBLIC_KEY = (b'1\xbc29\xc9\xce"\xf1\xcex\xea"\x83k\x1d\xede'
                       b'\x81\xbfRc\rG\xde&\x82\xbc\x80rc\xaa\xe4')
JOHN_DOE_DEVICE_NAME = 'test'
JOHN_DOE_DEVICE_VERIFY_KEY = (b'\xd5\xef\x8f\xbdPJ\xea\x9c<]qy\x06!M\xad5'
                              b'\x99m\xa0}EDqN\x06\x06c\x9e:\xe6\x80')
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
@click.option('--pubkeys', default=None)
@click.option('--host', '-H', default='127.0.0.1', help='Host to listen on (default: 127.0.0.1)')
@click.option('--port', '-P', default=6777, type=int, help=('Port to listen on (default: 6777)'))
@click.option('--store', '-s', default=None, help="Store configuration (default: in memory)")
@click.option('--block-store', '-b', default=None,
    help="URL of the block store the clients should write into (default: "
    "backend creates it own in-memory block store).")
@click.option('--log-level', '-l', default='WARNING',
              type=click.Choice(('DEBUG', 'INFO', 'WARNING', 'ERROR')))
@click.option('--debug', '-d', is_flag=True)
@click.option('--pdb', is_flag=True)
def backend_cmd(**kwargs):
    if kwargs.pop('pdb'):
        return run_with_pdb(_backend, **kwargs)
    else:
        return _backend(**kwargs)


def _backend(host, port, pubkeys, store, block_store, debug, log_level):
    log_handler = logbook.StderrHandler(level=log_level.upper())
    # Push globally the log handler make it work across threads
    log_handler.push_application()
    config = BackendConfig(
        debug=debug,
        blockstore_url=block_store,
        dburl=store,
        host=host,
        port=port
    )
    backend = BackendApp(config)

    async def _run_and_register_johndoe():
        async with trio.open_nursery() as nursery:
            await backend.init(nursery)

            try:
                await backend.user.get(JOHN_DOE_USER_ID)
            except NotFoundError:
                await backend.user.create(
                    '<backend-mock>', JOHN_DOE_USER_ID, JOHN_DOE_PUBLIC_KEY, devices={
                        JOHN_DOE_DEVICE_NAME: JOHN_DOE_DEVICE_VERIFY_KEY
                    }
                )

            try:
                await trio.serve_tcp(backend.handle_client, port)
            finally:
                await backend.shutdown()

    print('Starting Parsec Backend on %s:%s' % (host, port))
    try:
        trio.run(_run_and_register_johndoe)
    except KeyboardInterrupt:
        print('bye ;-)')

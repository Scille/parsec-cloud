import trio
import click

from .app import BackendApp


JOHN_DOE_IDENTITY = 'johndoe@test'
JOHN_DOE_PUBLIC_KEY = (b'1\xbc29\xc9\xce"\xf1\xcex\xea"\x83k\x1d\xede'
                       b'\x81\xbfRc\rG\xde&\x82\xbc\x80rc\xaa\xe4')
JOHN_DOE_VERIFY_KEY = (b'\xd5\xef\x8f\xbdPJ\xea\x9c<]qy\x06!M\xad5'
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
@click.option('--debug', '-d', is_flag=True)
@click.option('--pdb', is_flag=True)
def backend_cmd(**kwargs):
    if kwargs.pop('pdb'):
        return run_with_pdb(_backend, **kwargs)
    else:
        return _backend(**kwargs)


def _backend(host, port, pubkeys, store, block_store, debug):
    config = {
        # **CONFIG,
        'BLOCKSTORE_URL': block_store,
        'DEBUG': debug,
        'PORT': port,
        'HOST': host,
    }
    backend = BackendApp(config)

    async def _run_and_register_johndoe():
        await backend.init()
        await backend.user.create(JOHN_DOE_IDENTITY, JOHN_DOE_PUBLIC_KEY, devices={
            'main': JOHN_DOE_VERIFY_KEY
        })
        await trio.serve_tcp(backend.handle_client, port)

    print('Starting Parsec Backend on %s:%s' % (host, port))
    try:
        trio.run(_run_and_register_johndoe)
    except KeyboardInterrupt:
        print('bye ;-)')




    # # if not block_store:
    # #     block_store = '/blockstore'
    # #     register_in_memory_block_store_api(app, prefix=block_store)
    # if store:
    #     if store.startswith('postgres://'):
    #         store_type = 'PostgreSQL'
    #         backend_components = postgresql_components_factory(app, store, block_store)
    #     else:
    #         raise SystemExit('Unknown store `%s` (should be a postgresql db url).' % store)
    # else:
    #     store_type = 'mocked in memory'
    #     backend_components = mocked_components_factory(block_store)

    # dispatcher = backend_components.get_dispatcher()
    # register_backend_api(app, dispatcher)
    # register_start_api(app, dispatcher)

    # # TODO: remove me once RSA key loading and backend handling are easier
    # async def insert_john(app):
    #     from parsec.backend.pubkey import EPubKeyGet, EPubKeyAdd
    #     dispatcher = backend_components.get_dispatcher()
    #     try:
    #         await asyncio_perform(dispatcher, Effect(EPubKeyGet(JOHN_DOE_IDENTITY)))
    #     except PubKeyNotFound:
    #         await asyncio_perform(
    #             dispatcher, Effect(EPubKeyAdd(JOHN_DOE_IDENTITY, JOHN_DOE_PUBLIC_KEY)))
    # app.on_startup.append(insert_john)

    # print('Starting parsec backend on %s:%s with store %s' % (host, port, store_type))
    # web.run_app(app, host=host, port=port)
    # print('Bye ;-)')






    # async def _run_and_login(identity, rawkey):
    #     async def _login_on_ready():
    #         await core.server_ready.wait()
    #         await core.login(identity, rawkey)
    #         print('Logged as %s' % identity)

    #     async with trio.open_nursery() as nursery:
    #         nursery.start_soon(core.run)
    #         nursery.start_soon(_login_on_ready)

    # print('Starting Parsec Core on %s' % config['ADDR'])
    # try:
    #     if i_am_john:
    #         # TODO: well well well...
    #         from tests.common import User
    #         from tests.populate_local_storage import populate_local_storage_cls
    #         from nacl.public import PrivateKey
    #         populate_local_storage_cls(User(JOHN_DOE_IDENTITY, PrivateKey(JOHN_DOE_PRIVATE_KEY)), local_fs.LocalStorage)

    #         trio.run(_run_and_login, JOHN_DOE_IDENTITY, JOHN_DOE_PRIVATE_KEY)
    #     else:
    #         trio.run(core.run)
    # except KeyboardInterrupt:
    #     print('bye ;-)')

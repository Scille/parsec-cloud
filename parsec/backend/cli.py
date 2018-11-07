import os
import sys
import trio
import trio_asyncio
import click
from functools import partial

from parsec.logging import configure_logging, configure_sentry_logging
from parsec.backend import BackendApp, config_factory
from parsec.backend.user import NotFoundError
from parsec.backend.exceptions import AlreadyRevokedError


JOHN_DOE_USER_ID = "johndoe"
JOHN_DOE_PUBLIC_KEY = (
    b"zv\xf8\xa4\xf3n\x0b\xfe\xb8o9\xbe\xd8\xe705Y"
    b"\x0f<\x81\xf6\xf0o\xc0\xa5\x80 \xed\xe7\x80\x86\x0c"
)
JOHN_DOE_DEVICE_NAME = "test"
JOHN_DOE_DEVICE_VERIFY_KEY = (
    b"\xd5\xef\x8f\xbdPJ\xea\x9c<]qy\x06!M\xad5" b"\x99m\xa0}EDqN\x06\x06c\x9e:\xe6\x80"
)
DEFAULT_CORE_UNIX_SOCKET = "tcp://127.0.0.1:6776"


def run_with_pdb(cmd, *args, **kwargs):
    import pdb
    import traceback

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
        print("The program exited via sys.exit(). Exit status:", end=" ")
        print(sys.exc_info()[1])
    except SyntaxError:
        traceback.print_exc()
        sys.exit(1)
    except Exception:
        traceback.print_exc()
        print("Uncaught exception. Entering post mortem debugging")
        print("Running 'cont' or 'step' will restart the program")
        t = sys.exc_info()[2]
        pdb_context.interaction(None, t)
        print("Post mortem debugger finished.")


@click.command()
@click.option("--store", "-s", required=True, help="PostgreSQL database url")
@click.option("--force", "-f", is_flag=True)
def init_cmd(store, force):
    """
    Initialize a new backend's PostgreSQL database.
    """
    if not store.startswith("postgresql://"):
        raise SystemExit("Can only initialize a PostgreSQL database.")

    from parsec.backend.drivers.postgresql import init_db
    import trio_asyncio

    trio_asyncio.run(lambda: init_db(store, force=force))


@click.command()
@click.argument("user_id", nargs=1)
@click.option("--device-name", default=None, help="Device name")
@click.option(
    "--store", "-s", default="postgresql://127.0.0.1/parsec", help="Postgresql url of store"
)
def revoke_cmd(user_id, device_name, store):
    async def _revoke_user(user_id):
        async with trio.open_nursery() as nursery:
            await backend.init(nursery)
            try:
                if device_name:
                    await backend.user.revoke_device(f"{user_id}@{device_name}")
                else:
                    await backend.user.revoke_user(user_id)
            except AlreadyRevokedError as exc:
                print(str(exc))
            except NotFoundError as exc:
                print(str(exc))
            finally:
                await backend.teardown()

    try:
        config = config_factory(db_url=store, blockstore_types="MOCKED")
    except ValueError as exc:
        raise SystemExit(f"Invalid configuration: {exc}")

    backend = BackendApp(config)
    trio_asyncio.run(_revoke_user, user_id)


@click.command()
@click.option("--pubkeys", default=None)
@click.option("--host", "-H", default="127.0.0.1", help="Host to listen on (default: 127.0.0.1)")
@click.option("--port", "-P", default=6777, type=int, help=("Port to listen on (default: 6777)"))
@click.option(
    "--store", "-s", default="MOCKED", help="Store configuration (default: mocked in memory)"
)
@click.option(
    "--blockstore",
    "-b",
    default="MOCKED",
    type=click.Choice(("MOCKED", "POSTGRESQL", "S3", "SWIFT", "RAID1")),
    help="Block store the clients should write into (default: mocked in memory). Set environment variables accordingly.",
)
@click.option(
    "--log-level", "-l", default="WARNING", type=click.Choice(("DEBUG", "INFO", "WARNING", "ERROR"))
)
@click.option("--log-format", "-f", default="CONSOLE", type=click.Choice(("CONSOLE", "JSON")))
@click.option("--log-file", "-o")
@click.option("--log-filter", default=None)
@click.option("--debug", "-d", is_flag=True)
@click.option("--pdb", is_flag=True)
def backend_cmd(log_level, log_format, log_file, log_filter, pdb, **kwargs):
    configure_logging(log_level, log_format, log_file, log_filter)

    if pdb:
        return run_with_pdb(_backend, **kwargs)

    else:
        return _backend(**kwargs)


def _backend(host, port, pubkeys, store, blockstore, debug):
    try:
        config = config_factory(
            debug=debug, blockstore_type=blockstore, db_url=store, environ=os.environ
        )
    except ValueError as exc:
        raise SystemExit(f"Invalid configuration: {exc}")

    if config.sentry_url:
        configure_sentry_logging(config.sentry_url)

    backend = BackendApp(config)

    async def _run_and_register_johndoe():
        async with trio.open_nursery() as nursery:
            await backend.init(nursery)

            try:
                await backend.user.get(JOHN_DOE_USER_ID)
            except NotFoundError:
                await backend.user.create(
                    JOHN_DOE_USER_ID,
                    JOHN_DOE_PUBLIC_KEY,
                    devices={JOHN_DOE_DEVICE_NAME: JOHN_DOE_DEVICE_VERIFY_KEY},
                )

            try:
                await trio.serve_tcp(
                    partial(backend.handle_client, swallow_crash=True), port, host=host
                )
            finally:
                await backend.teardown()

    print(
        f"Starting Parsec Backend on {host}:{port} (db={config.db_type}, blockstore={config.blockstore_config.type})"
    )
    try:
        trio_asyncio.run(_run_and_register_johndoe)
    except KeyboardInterrupt:
        print("bye ;-)")


if __name__ == "__main__":
    backend_cmd()

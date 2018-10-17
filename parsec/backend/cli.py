import sys
import trio
import trio_asyncio
import click
from uuid import UUID

from parsec.logging import configure_logging, configure_sentry_logging
from parsec.backend import BackendApp, config_factory
from parsec.backend.user import NotFoundError


JOHN_DOE_USER_ID = "johndoe"
JOHN_DOE_PUBLIC_KEY = (
    b"zv\xf8\xa4\xf3n\x0b\xfe\xb8o9\xbe\xd8\xe705Y"
    b"\x0f<\x81\xf6\xf0o\xc0\xa5\x80 \xed\xe7\x80\x86\x0c"
)
JOHN_DOE_DEVICE_NAME = "test"
JOHN_DOE_DEVICE_VERIFY_KEY = (
    b"\xd5\xef\x8f\xbdPJ\xea\x9c<]qy\x06!M\xad5" b"\x99m\xa0}EDqN\x06\x06c\x9e:\xe6\x80"
)
JOHN_DOE_USER_VLOB = {
    "id": UUID("230165e6acd441f4a0b4f2c8c0dc91f0"),
    "rts": "c7121459551b40e78e35f49115097594",
    "wts": "3c7d3cb553854ffea524092487674a0b",
    "blob": (
        b"\xa61\r\x19\x05\x03\xdej\x18\x0c\x044~\x7fS\xa0(\xc4r4\xf68\x1bw"
        b"l\xa5\x91\x8a\x99\xfck#\xf6\x00\xcb\x00\xec\xb0U'\xc4\x89\x90a"
        b"\xa8\x1c\xb7\xe9'i\xcf\xd1x\x0c\xd6\xd2\xdc\xfe\x05\x89\x85\"\xa3\xe7"
        b"\xbb\xa4\x877\xb7R\xdd\xaa\x90K\x01\xe2\x94\x10k\x90?\x10\xbb\xad?>\x04\xdc"
        b"\xb3\xe1u\x96O,\xdb\xae\xea\x13k\xbf\xc6\x94&w\xf3\xd4{\x94\x11\xad\xf7\x1e"
        b"\x19\x14\x8b\xe0\xcf#\xa1\xc6\xf3\xfc\xf7\x17\xdf\xa5\xdc\x1fP\x05f\x12"
        b"\x84\xba\xee8\xe9E\x85\xb2\x01\x06\x80Xb\t\xaa\xcc\xc0\x90\x96Q\xcc\x18X5"
        b"L\xdcP\x90X\xd6\xb8\x84\x07\xcc\x1f\x1c\xab\x8a\xeb\xa4\x10\x12\xf8~"
        b"x}\x86\xc1\xb5\x86\x98\xa8\xc6M\xa0h\xd4\x1d@'\x0c\xf2w\x04\x18\xe6\xcf\xc0"
        b")\xd9T2\n\xab4XW\x88\xfd\xd7\x91\xde\xadLA4o\x1e\x9aaG\xfd\x98\xc0\xda\x94"
        b"zAX\xe2c\xbcwe\xa2Z\xf2\xc1\xdd\xd8\x0b\x15i\x03\xeecG=x)\xd0\xce\xe5\xbe"
        b"&J3\x05\xc7\x9f%\xfd\xadF\xbb\x1c\xd6yF\x991]\x083\x8c\x9fx\xf9)\xa9&C"
        b"\x120\xc4%0\xa6\xae@B\x84,\xdfd\xbei\x1d\xb3\x873\xc2\xd5XmHn\x01d9"
        b"\xaa\x1f\xc9\xd8\xedwN\x00\x9d%;\xa8\xbe\xdc\xa5T\xa2\xf7\xb9\xf2"
        b"\x15\x06\x88\xf3\xe16~\xc6\x13\x87\xe0\xe6\x93t.\xd0\x91\x8a\xfd\xea"
        b"\xf4\x98\xdc\x8e\xc3Q\x1b\xb0G\xc7\x93L:-7"
    ),
}
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
    type=click.Choice(("MOCKED", "POSTGRESQL", "S3", "SWIFT")),
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
        config = config_factory({"debug": debug, "blockstore_type": blockstore, "db_url": store})
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
                await backend.vlob.create(**JOHN_DOE_USER_VLOB, author="<backend-mock>")

            try:
                await trio.serve_tcp(backend.handle_client, port, host=host)
            finally:
                await backend.teardown()

    print("Starting Parsec Backend on %s:%s" % (host, port))
    try:
        trio_asyncio.run(_run_and_register_johndoe)
    except KeyboardInterrupt:
        print("bye ;-)")


if __name__ == "__main__":
    backend_cmd()

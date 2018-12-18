import os
import trio
import trio_asyncio
import click
from functools import partial

from parsec.cli_utils import spinner
from parsec.logging import configure_logging, configure_sentry_logging
from parsec.backend import BackendApp, config_factory
from parsec.backend.drivers.postgresql import init_db


__all__ = ("backend_cmd", "init_cmd", "run_cmd")


@click.command(short_help="init the database")
@click.option("--db", required=True, help="PostgreSQL database url")
@click.option("--force", "-f", is_flag=True)
def init_cmd(db, force):
    """
    Initialize a new backend's PostgreSQL database.
    """
    debug = "DEBUG" in os.environ
    if not db.startswith("postgresql://"):
        raise SystemExit("Can only initialize a PostgreSQL database.")

    try:

        async def _init_db(db, force):
            async with spinner("Initializing database"):
                already_initialized = await init_db(db, force)
            if already_initialized:
                click.echo("Database already initialized, nothing to do.")

        trio_asyncio.run(_init_db, db, force)

    except Exception as exc:
        click.echo(click.style("Error: ", fg="red") + str(exc))
        if debug:
            raise
        else:
            raise SystemExit(1)


@click.command(short_help="run the server")
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
def run_cmd(host, port, store, blockstore, log_level, log_format, log_file, log_filter, debug):
    configure_logging(log_level, log_format, log_file, log_filter)

    try:
        config = config_factory(
            debug=debug, blockstore_type=blockstore, db_url=store, environ=os.environ
        )
    except ValueError as exc:
        raise SystemExit(f"Invalid configuration: {exc}")

    if config.sentry_url:
        configure_sentry_logging(config.sentry_url)

    backend = BackendApp(config)

    async def _run_backend():
        async with trio.open_nursery() as nursery:
            await backend.init(nursery)

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
        # from tests.monitor import Monitor
        trio_asyncio.run(_run_backend)
    except KeyboardInterrupt:
        print("bye ;-)")


@click.group()
def backend_cmd():
    pass


backend_cmd.add_command(run_cmd, "run")
backend_cmd.add_command(init_cmd, "init")


if __name__ == "__main__":
    backend_cmd()

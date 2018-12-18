import os
import trio
import trio_asyncio
import click
from functools import partial

from parsec.types import DeviceID
from parsec.crypto import export_root_verify_key
from parsec.logging import configure_logging, configure_sentry_logging
from parsec.backend import BackendApp, config_factory
from parsec.backend.drivers.postgresql import init_db
from parsec.core.fs.utils import new_access
from parsec.core.config import get_default_config_dir
from parsec.core.devices_manager import LocalDevicesManager


# TODO: avoid using parsec.core.config
DEFAULT_DEVICES_PATH = os.path.join(get_default_config_dir(os.environ), "devices")


@click.command()
@click.option("--db", required=True, help="PostgreSQL database url")
@click.option("--force", "-f", is_flag=True)
@click.option("--backend-base-url", "-b", "backend_base_url", required=True)
@click.argument("device_id", metavar="user-id", type=DeviceID)
def init_cmd(db, force, backend_base_url, device_id):
    """
    Initialize a new backend's PostgreSQL database.

    Creates database's table, generates the root verify/signing key and inserts
    first user/device (signed with root siging key).

    Note root signing key is dropped after this operation to ensure only
    the first device can sign new users/devices.
    """
    if not db.startswith("postgresql://"):
        raise SystemExit("Can only initialize a PostgreSQL database.")

    from parsec.crypto import SigningKey
    from parsec.core.devices_manager import generate_new_device, save_device_with_password
    from parsec.core.config import get_default_config_dir

    config_dir = get_default_config_dir(os.environ)
    root_signing_key = SigningKey.generate()

    url_param_root_verify_key = export_root_verify_key(root_signing_key.verify_key)
    backend_addr = f"{backend_base_url}?rvk={url_param_root_verify_key}"

    device = generate_new_device(device_id, backend_addr, root_signing_key.verify_key)
    init_done = False

    async def _init_db():
        nonlocal init_done
        init_done = await init_db(db, device, root_signing_key, force=force)

    try:
        trio_asyncio.run(_init_db)

        if not init_done:
            click.secho("Database already initialized, nothing to do.", fg="green")
        else:
            password = click.prompt("Device password", hide_input=True, confirmation_prompt=True)
            save_device_with_password(config_dir, device, password)

            click.secho("Database initialized", fg="green")
            click.secho("Backend environment variables: ", fg="green", nl=False)
            click.echo(f"Root verify key: {url_param_root_verify_key}")
            if backend_base_url:
                click.secho("Backend URL: ", fg="green", nl=False)
                click.echo(backend_addr)
            else:
                click.secho("Backend URL template: ", fg="green", nl=False)
                click.echo(f"ws://<domain>?root-verify-key={url_param_root_verify_key}")

    except (KeyboardInterrupt, click.Abort):
        click.secho("Aborting...", fg="yellow")

    except Exception as exc:
        click.secho("DB initialization failed: ", fg="red", nl=False)
        raise
        click.echo(str(exc))
        raise SystemExit(1)


@click.command()
@click.option("--root-verify-key", "--rvk", required=True)
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
def backend_cmd(
    rvk, host, port, store, blockstore, log_level, log_format, log_file, log_filter, debug
):
    configure_logging(log_level, log_format, log_file, log_filter)

    try:
        config = config_factory(
            root_verify_key=rvk,
            debug=debug,
            blockstore_type=blockstore,
            db_url=store,
            environ=os.environ,
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


if __name__ == "__main__":
    backend_cmd()

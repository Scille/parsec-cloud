import os
import trio
import trio_asyncio
import click
from functools import partial

from parsec.types import UserID, DeviceName
from parsec.utils import encode_urlsafe_root_verify_key
from parsec.logging import configure_logging, configure_sentry_logging
from parsec.backend import BackendApp, config_factory
from parsec.backend.user import NotFoundError
from parsec.backend.exceptions import AlreadyRevokedError
from parsec.backend.drivers.postgresql import init_db
from parsec.core.fs.utils import new_access
from parsec.core.config import get_default_settings_path
from parsec.core.devices_manager import LocalDevicesManager


DEFAULT_DEVICES_PATH = os.path.join(get_default_settings_path(), "devices")


@click.command()
@click.option("--db", required=True, help="PostgreSQL database url")
@click.option("--force", "-f", is_flag=True)
@click.option(
    "--settings",
    type=click.Path(file_okay=False, writable=True),
    default=DEFAULT_DEVICES_PATH,
    help="Path to store new first device configuration.",
)
@click.option("--backend-base-url", "-b", "backend_base_url")
@click.argument("user_id", metavar="user-id", type=UserID)
@click.argument("device_name", metavar="device-name", type=DeviceName)
def init_cmd(db, force, settings, backend_base_url, user_id, device_name):
    """
    Initialize a new backend's PostgreSQL database.

    Creates database's table, generates the root verify/signing key and inserts
    first user/device (signed with root siging key).

    Note root signing key is dropped after this operation to ensure only
    the first device can sign new users/devices.
    """
    if not db.startswith("postgresql://"):
        raise SystemExit("Can only initialize a PostgreSQL database.")

    keys = None

    async def _init_db():
        nonlocal keys
        keys = await init_db(db, user_id, device_name, force=force)

    try:
        trio_asyncio.run(_init_db)

        if not keys:
            click.secho("Database already initialized, nothing to do.", fg="green")
        else:
            root_verify_key, user_private_key, device_signing_key = keys
            password = click.prompt("Device password", hide_input=True, confirmation_prompt=True)
            devices_manager = LocalDevicesManager(settings)
            device_id = f"{user_id}@{device_name}"
            user_manifest_access = new_access()
            devices_manager.register_new_device(
                device_id,
                root_verify_key.encode(),
                user_private_key.encode(),
                device_signing_key.encode(),
                user_manifest_access,
                password=password,
            )
            url_param_root_verify_key = encode_urlsafe_root_verify_key(root_verify_key)

            click.secho("Database initialized", fg="green")
            click.secho("Backend environment variables: ", fg="green", nl=False)
            click.echo(f"ROOT_VERIFY_KEY={url_param_root_verify_key}")
            if backend_base_url:
                click.secho("Backend URL: ", fg="green", nl=False)
                click.echo(f"{backend_base_url}?root-verify-key={url_param_root_verify_key}")
            else:
                click.secho("Backend URL template: ", fg="green", nl=False)
                click.echo(f"ws://<domain>?root-verify-key={url_param_root_verify_key}")

    except (KeyboardInterrupt, click.Abort):
        click.secho("Aborting...", fg="yellow")

    except Exception as exc:
        click.secho("DB initialization failed: ", fg="red", nl=False)
        click.echo(str(exc))
        raise SystemExit(1)


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
def backend_cmd(log_level, log_format, log_file, log_filter, **kwargs):
    configure_logging(log_level, log_format, log_file, log_filter)
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
        trio_asyncio.run(_run_backend)
    except KeyboardInterrupt:
        print("bye ;-)")


if __name__ == "__main__":
    backend_cmd()

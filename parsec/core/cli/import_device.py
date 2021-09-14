# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS
from pathlib import Path

import click

from parsec.cli_utils import cli_exception_handler, spinner
from parsec.core import CoreConfig
from parsec.core.cli.utils import (
    cli_command_base_options,
    core_config_options,
    format_available_devices,
)
from parsec.core.local_device import load_device_with_password
from parsec.core.recovery import (
    create_new_device_from_original,
    get_recovery_password_from_passphrase,
)
from parsec.utils import trio_run


async def _import_device(
    config: CoreConfig,
    destination: Path,
    recovery_password: str,
    new_password: str,
    device_label: str,
) -> None:

    async with spinner("Loading recovery device with password"):
        recovery_device = load_device_with_password(destination, password=recovery_password)

    async with spinner("Creating new device from recovery"):
        new_device = await create_new_device_from_original(
            recovery_device, device_label, new_password, config.config_dir
        )
    device_display = click.style(new_device.slughash, fg="yellow")
    click.echo(f"Saving device {device_display}")

    click.echo("New device imported:")
    click.echo(format_available_devices([new_device]))


class RecoveryPassphrase(click.ParamType):
    name = "recovery_passphrase"

    def convert(self, value, param, ctx):
        try:
            return get_recovery_password_from_passphrase(value)
        except:
            self.fail("The passphrase provided does not have the correct format", param, ctx)


@click.command(short_help="import recovery device")
@click.option(
    "--key-path",
    "-K",
    help="Path of the device key to recover (.psrk)",
    required=True,
    type=click.Path(exists=True),
)
@click.option(
    "--recovery_passphrase",
    "-R",
    help="Passphrase of the recovery device",
    type=RecoveryPassphrase(),
    prompt=True,
    confirmation_prompt=True,
)
@click.password_option("--new_password", "-P", help="Password of the new device")
@click.option("--new_label", "-L", help="Label of the new device", prompt=True)
@core_config_options
@cli_command_base_options
def import_recovery_device(
    config: CoreConfig,
    key_path: str,
    recovery_passphrase: str,
    new_password: str,
    new_label: str,
    **kwargs,
):
    """
    Create a new device from a recovery device key.
    """
    with cli_exception_handler(config.debug):
        trio_run(
            _import_device, config, Path(key_path), recovery_passphrase, new_password, new_label
        )

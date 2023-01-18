# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from pathlib import Path
from typing import Any

import click

from parsec._parsec import load_recovery_device, save_recovery_device
from parsec.api.protocol import DeviceLabel
from parsec.cli_utils import cli_exception_handler, operation, spinner
from parsec.core import CoreConfig
from parsec.core.cli.bootstrap_organization import SaveDeviceWithSelectedAuth
from parsec.core.cli.utils import (
    cli_command_base_options,
    core_config_and_device_options,
    core_config_options,
    save_device_options,
)
from parsec.core.local_device import get_recovery_device_file_name
from parsec.core.recovery import generate_new_device_from_recovery, generate_recovery_device
from parsec.core.types import LocalDevice
from parsec.utils import trio_run


async def _export_recovery_device(
    config: CoreConfig, original_device: LocalDevice, output: Path
) -> None:
    async with spinner("Creating new recovery device"):
        recovery_device = await generate_recovery_device(original_device)

    file_name = get_recovery_device_file_name(recovery_device)
    file_path = output / file_name
    file_path_display = click.style(str(file_path.absolute()), fg="yellow")
    with operation(f"Saving recovery device file in {file_path_display}"):
        passphrase = await save_recovery_device(file_path, recovery_device, force=False)

    p1 = click.style("Save the recovery passphrase in a safe place:", fg="red")
    p2 = click.style(passphrase, fg="green")
    click.echo(f"{p1} {p2}")


@click.command(short_help="export recovery device")
@click.option(
    "--output",
    "-o",
    required=False,
    default=Path("./"),
    show_default=True,
    help="Where the device keys will be exported",
    type=click.Path(exists=True, file_okay=False, dir_okay=True, writable=True, path_type=Path),
)
@core_config_and_device_options
@cli_command_base_options
def export_recovery_device(
    config: CoreConfig, device: LocalDevice, output: Path, **kwargs: Any
) -> None:
    """
    Create a new recovery device for the user.
    """
    with cli_exception_handler(config.debug):
        trio_run(_export_recovery_device, config, device, output)


async def _import_recovery_device(
    config: CoreConfig,
    recovery_file: Path,
    passphrase: str,
    new_device_label: DeviceLabel,
    save_device_with_selected_auth: SaveDeviceWithSelectedAuth,
) -> None:

    recovery_device = await load_recovery_device(recovery_file, passphrase)

    device_label_display = click.style(new_device_label, fg="yellow")
    async with spinner(f"Creating new device {device_label_display}"):
        new_device = await generate_new_device_from_recovery(recovery_device, new_device_label)

    await save_device_with_selected_auth(config_dir=config.config_dir, device=new_device)


@click.command(short_help="import recovery device")
@click.argument(
    "file",
    required=True,
    type=click.Path(exists=True, file_okay=True, dir_okay=False, path_type=Path),
)
@click.option(
    "--passphrase",
    "-P",
    help="Passphrase protecting for the recovery device file",
    prompt=True,
    required=True,
)
@click.option(
    "--device-label",
    "-L",
    help="Label for the new device",
    prompt="New device label",
    type=DeviceLabel,
)
@save_device_options
@core_config_options
@cli_command_base_options
def import_recovery_device(
    config: CoreConfig,
    file: Path,
    passphrase: str,
    device_label: DeviceLabel,
    save_device_with_selected_auth: SaveDeviceWithSelectedAuth,
    **kwargs: Any,
) -> None:
    """
    Create a new device from a .psrk recovery device file.
    """
    with cli_exception_handler(config.debug):
        trio_run(
            _import_recovery_device,
            config,
            file,
            passphrase,
            device_label,
            save_device_with_selected_auth,
        )

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import os
from pathlib import Path
from typing import Any

import click

from parsec._parsec import AvailableDevice, list_available_devices
from parsec.cli_utils import cli_exception_handler
from parsec.core.cli.utils import (
    cli_command_base_options,
    core_config_and_available_device_options,
    format_available_devices,
)
from parsec.core.config import get_default_config_dir


@click.command()
@click.option("--config-dir", type=click.Path(exists=True, file_okay=False))
@cli_command_base_options
def list_devices(
    config_dir: Path,
    debug: bool,
    **kwargs: Any,
) -> None:
    with cli_exception_handler(debug):
        config_dir = Path(config_dir) if config_dir else get_default_config_dir(os.environ)
        devices = list_available_devices(config_dir)
        num_devices_display = click.style(str(len(devices)), fg="green")
        config_dir_display = click.style(config_dir, fg="yellow")
        click.echo(f"Found {num_devices_display} device(s) in {config_dir_display}:")
        click.echo(format_available_devices(devices))


@click.command()
@cli_command_base_options
@core_config_and_available_device_options
def remove_device(
    device: AvailableDevice,
    debug: bool,
    **kwargs: Any,
) -> None:
    with cli_exception_handler(debug):
        click.echo("You are about to remove the following device:")
        click.echo(format_available_devices([device]))
        click.confirm("Are you sure?", abort=True)
        device.key_file_path.unlink()
        styled_device = click.style(f"{device.slughash}", fg="yellow")
        click.echo(f"The device {styled_device} has been removed")

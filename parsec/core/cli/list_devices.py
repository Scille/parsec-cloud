# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import os
import click
from pathlib import Path

from parsec.cli_utils import cli_exception_handler
from parsec.core.config import get_default_config_dir
from parsec.core.local_device import list_available_devices
from parsec.core.cli.utils import cli_command_base_options, format_available_devices


@click.command()
@click.option("--config-dir", type=click.Path(exists=True, file_okay=False))
@cli_command_base_options
def list_devices(config_dir, debug, **kwargs):
    with cli_exception_handler(debug):
        config_dir = Path(config_dir) if config_dir else get_default_config_dir(os.environ)
        devices = list_available_devices(config_dir)
        num_devices_display = click.style(str(len(devices)), fg="green")
        config_dir_display = click.style(str(config_dir), fg="yellow")
        click.echo(f"Found {num_devices_display} device(s) in {config_dir_display}:")
        click.echo(format_available_devices(devices))

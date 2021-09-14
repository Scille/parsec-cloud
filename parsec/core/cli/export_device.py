# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS
from pathlib import Path

import click

from parsec.core import CoreConfig
from parsec.core.recovery import (
    generate_recovery_password,
    create_new_device_from_original,
    generate_recovery_device_name,
    generate_recovery_key_name,
    generate_passphrase_from_recovery_password,
)
from parsec.core.types import LocalDevice
from parsec.utils import trio_run
from parsec.cli_utils import cli_exception_handler, spinner, aconfirm
from parsec.core.cli.utils import cli_command_base_options, core_config_and_device_options


async def _export_device(config: CoreConfig, original_device: LocalDevice, output: Path) -> None:

    device_label = generate_recovery_device_name()

    if output.is_dir():
        # .psrk stands for ps(parsec)r(recovery)k(key)
        default_key_name = generate_recovery_key_name(original_device)
        dest_key = output / default_key_name
    else:
        if output.suffix != ".psrk":
            dest_key = output / ".psrk"
        else:
            dest_key = output

    if dest_key.exists():
        rep = await aconfirm(f"Key already exist in {dest_key.absolute()}, overwrite it ?")
        if not rep:
            click.echo(
                "Aborting exporting key, you can choose another destination folder by using --output-folder"
            )
            return

    async with spinner("Generating strong password"):
        password = generate_recovery_password()
        passphrase = generate_passphrase_from_recovery_password(password)

    p1 = click.style("Save the recovery passphrase in a safe place:", fg="red")
    p2 = click.style(passphrase, fg="green")
    click.echo(f"{p1} {p2}")

    async with spinner("Creating new device used for recovery"):
        await create_new_device_from_original(
            original_device, device_label, password, config.config_dir, key_file=dest_key
        )
    device_display = click.style(str(dest_key.absolute()), fg="yellow")
    click.echo(f"Saving recovery device file in {device_display}")


@click.command(short_help="export recovery device")
@click.option(
    "--output",
    "-O",
    required=False,
    default=Path("./"),
    show_default=True,
    help="Where the device keys will be exported",
    type=click.Path(exists=True),
)
@core_config_and_device_options
@cli_command_base_options
def export_recovery_device(config: CoreConfig, device: LocalDevice, output: str, **kwargs):
    """
    Create a new recovery device for the given device.
    """
    with cli_exception_handler(config.debug):
        trio_run(_export_device, config, device, Path(output))

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import trio
import click
import multiprocessing
from pathlib import Path
from typing import Optional

from parsec.utils import trio_run
from parsec.logging import configure_sentry_logging
from parsec.cli_utils import cli_exception_handler, generate_not_available_cmd
from parsec.core import logged_core_factory
from parsec.core.types import LocalDevice
from parsec.core.config import CoreConfig
from parsec.core.cli.utils import (
    cli_command_base_options,
    gui_command_base_options,
    core_config_and_device_options,
    core_config_options,
)

try:
    from parsec.core.gui import run_gui as _run_gui

except ImportError as exc:
    _run_gui = generate_not_available_cmd(exc)

else:

    @click.command(short_help="run parsec GUI")
    # Let the GUI handle the parsing of the url to display dialog on error
    @click.argument("url", required=False)
    @click.option("--diagnose", "-d", is_flag=True)
    @core_config_options
    @gui_command_base_options
    def run_gui(
        config: CoreConfig,
        url: str,
        diagnose: bool,
        sentry_dsn: Optional[str],
        sentry_environment: str,
        **kwargs,
    ) -> None:
        """
        Run parsec GUI
        """
        # Necessary for DialogInProcess since it's not the default on windows
        # This method should only be called once which is why we do it here.
        multiprocessing.set_start_method("spawn")

        with cli_exception_handler(config.debug):
            if config.telemetry_enabled and sentry_dsn:
                configure_sentry_logging(dsn=sentry_dsn, environment=sentry_environment)

            config = config.evolve(mountpoint_enabled=True)
            try:
                _run_gui(config, start_arg=url, diagnose=diagnose)
            except KeyboardInterrupt:
                click.echo("bye ;-)")


async def _run_mountpoint(config: CoreConfig, device: LocalDevice) -> None:
    config = config.evolve(mountpoint_enabled=True)
    async with logged_core_factory(config, device):
        display_device = click.style(device.device_id, fg="yellow")
        mountpoint_display = click.style(str(config.mountpoint_base_dir.absolute()), fg="yellow")
        click.echo(f"{display_device}'s drive mounted at {mountpoint_display}")

        await trio.sleep_forever()


@click.command(short_help="run parsec mountpoint")
@click.option("--mountpoint", "-m", type=click.Path(exists=False))
@core_config_and_device_options
@cli_command_base_options
def run_mountpoint(
    config: CoreConfig,
    device: LocalDevice,
    mountpoint: Path,
    **kwargs,
) -> None:
    """
    Expose device's parsec drive on the given mountpoint.
    """
    config = config.evolve(mountpoint_enabled=True)
    if mountpoint:
        config = config.evolve(mountpoint_base_dir=Path(mountpoint))
    with cli_exception_handler(config.debug):
        try:
            trio_run(_run_mountpoint, config, device)
        except KeyboardInterrupt:
            click.echo("bye ;-)")

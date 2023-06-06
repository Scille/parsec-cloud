# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import multiprocessing
import sys
from contextlib import contextmanager
from pathlib import Path
from typing import Any, Iterator

import click
import trio

from parsec.cli_utils import cli_exception_handler, generate_not_available_cmd
from parsec.core import logged_core_factory, win_registry
from parsec.core.cli.utils import (
    cli_command_base_options,
    core_config_and_device_options,
    core_config_options,
    gui_command_base_options,
)
from parsec.core.config import CoreConfig
from parsec.core.types import LocalDevice
from parsec.logging import configure_sentry_logging
from parsec.utils import trio_run

try:
    from parsec.core.gui import run_gui as _run_gui

except ImportError as exc:
    run_gui = generate_not_available_cmd(exc)

else:

    @click.command(short_help="run parsec GUI")
    # Let the GUI handle the parsing of the url to display dialog on error
    @click.argument("url", required=False)
    @click.option("--diagnose", "-d", is_flag=True)
    @core_config_options
    @gui_command_base_options
    def run_gui(  # type: ignore
        config: CoreConfig,
        url: str,
        diagnose: bool,
        sentry_dsn: str | None,
        sentry_environment: str,
        **kwargs: Any,
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
                with parsec_quick_access_context(config):
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
    **kwargs: Any,
) -> None:
    """
    Expose device's parsec drive on the given mountpoint.
    """
    config = config.evolve(mountpoint_enabled=True)
    if mountpoint:
        config = config.evolve(mountpoint_base_dir=Path(mountpoint))
    with cli_exception_handler(config.debug):
        try:
            with parsec_quick_access_context(config):
                trio_run(_run_mountpoint, config, device)
        except KeyboardInterrupt:
            click.echo("bye ;-)")


def cleanup_artifacts(base_mountpoint_path: Path) -> None:
    # No cleanup required
    if not base_mountpoint_path.exists():
        return

    # Loop over non-existing directories in the base mountpoint path
    for path in base_mountpoint_path.iterdir():
        if path.exists():
            continue

        # Artifacts from previous run can remain listed in the directory
        # (even though `path.exists()`) returns `False`
        # In this case, `.unlink()` still works and fixes the issue
        try:
            path.unlink()
        except OSError:
            # No artifact was present
            pass
        else:
            # An artifact has been cleaned up
            pass


@contextmanager
def parsec_quick_access_context(
    config: CoreConfig, appguid: str | None = None, appname: str | None = None
) -> Iterator[None]:
    if sys.platform != "win32" or not config.mountpoint_in_directory:
        yield
        return
    try:
        win_registry.add_parsec_mountpoint_directory_to_quick_access(
            config.mountpoint_base_dir, appguid, appname
        )
        cleanup_artifacts(config.mountpoint_base_dir)
        yield
    finally:
        win_registry.remove_parsec_mountpoint_directory_from_quick_access(appguid)

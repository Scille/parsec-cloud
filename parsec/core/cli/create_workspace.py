# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import click

from parsec.api.data import EntryName
from parsec.utils import trio_run
from parsec.cli_utils import cli_exception_handler
from parsec.core import logged_core_factory
from parsec.core.types import LocalDevice
from parsec.core.config import CoreConfig
from parsec.core.cli.utils import cli_command_base_options, core_config_and_device_options


async def _create_workspace(config: CoreConfig, device: LocalDevice, name: EntryName) -> None:
    async with logged_core_factory(config, device) as core:
        await core.user_fs.workspace_create(name)


@click.command(short_help="create workspace")
@click.argument("name")
@core_config_and_device_options
@cli_command_base_options
def create_workspace(config: CoreConfig, device: LocalDevice, name: str, **kwargs) -> None:
    """
    Create a new workspace for the given device.
    """
    with cli_exception_handler(config.debug):
        trio_run(_create_workspace, config, device, EntryName(name))

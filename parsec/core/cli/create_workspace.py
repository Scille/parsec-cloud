# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import click

from parsec.utils import trio_run
from parsec.cli_utils import cli_exception_handler
from parsec.core import logged_core_factory
from parsec.core.cli.utils import cli_command_base_options, core_config_and_device_options


async def _create_workspace(config, device, name):
    async with logged_core_factory(config, device) as core:
        await core.user_fs.workspace_create(f"{name}")


@click.command(short_help="create workspace")
@click.argument("name")
@core_config_and_device_options
@cli_command_base_options
def create_workspace(config, device, name, **kwargs):
    """
    Create a new workspace for the given device.
    """
    with cli_exception_handler(config.debug):
        trio_run(_create_workspace, config, device, name)

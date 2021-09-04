# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import click

from parsec.utils import trio_run
from parsec.api.protocol import UserID
from parsec.cli_utils import cli_exception_handler
from parsec.core import logged_core_factory
from parsec.core.cli.utils import cli_command_base_options, core_config_and_device_options
from parsec.core.types import WorkspaceRole


WORKSPACE_ROLE_CHOICES = {"NONE": None, **{role.value: role for role in WorkspaceRole}}


async def _share_workspace(config, device, name, user_id, user_role):
    async with logged_core_factory(config, device) as core:
        workspace = core.find_workspace_from_name(name)
        await core.user_fs.workspace_share(workspace.id, user_id, user_role)


@click.command(short_help="share workspace")
@click.option("--workspace-name")
@click.option("--user-id", type=UserID, required=True)
@click.option(
    "--role", required=True, type=click.Choice(WORKSPACE_ROLE_CHOICES.keys(), case_sensitive=False)
)
@core_config_and_device_options
@cli_command_base_options
def share_workspace(config, device, workspace_name, user_id, role, **kwargs):
    """
    Create a new workspace for the given device.
    """
    role = WORKSPACE_ROLE_CHOICES[role]
    with cli_exception_handler(config.debug):
        trio_run(_share_workspace, config, device, workspace_name, user_id, role)

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import click

from parsec.utils import trio_run
from parsec.api.protocol import UserID
from parsec.cli_utils import cli_exception_handler
from parsec.core import logged_core_factory
from parsec.core.cli.utils import cli_command_base_options, core_config_and_device_options
from parsec.core.types import WorkspaceRole


WORKSPACE_ROLE_CHOICES = {"NONE": None, **{role.value: role for role in WorkspaceRole}}


async def _share_workspace(config, device, name, user_id, recipiant, user_role):
    if recipiant and user_id or not recipiant and not user_id:
        raise click.ClickException("Either --recipiant or --user-id should be used, but not both")
    async with logged_core_factory(config, device) as core:
        workspace = core.find_workspace_from_name(name)
        if recipiant:
            user_info_tab, nb = await core.find_humans(recipiant, 1, 100, False, False)
            if nb == 0:
                raise RuntimeError("Unknown recipiant")
            if nb != 1:
                for user in user_info_tab:
                    if user.revoked_on is not None:
                        continue
                    click.echo(f"{user.human_handle} - UserID: {user.user_id}")
                raise RuntimeError("Specify the user more precisely or use the --user-id option")
            user_id = user_info_tab[0].user_id
        await core.user_fs.workspace_share(workspace.id, user_id, user_role)


@click.command(short_help="share workspace")
@click.option("--workspace-name")
@click.option("--user-id", type=UserID)
@click.option("--recipiant", help="Name or email to whom the workspace is shared with")
@click.option(
    "--role", required=True, type=click.Choice(WORKSPACE_ROLE_CHOICES.keys(), case_sensitive=False)
)
@core_config_and_device_options
@cli_command_base_options
def share_workspace(config, device, workspace_name, user_id, recipiant, role, **kwargs):
    """
    Create a new workspace for the given device.
    """
    role = WORKSPACE_ROLE_CHOICES[role]
    with cli_exception_handler(config.debug):
        trio_run(_share_workspace, config, device, workspace_name, user_id, recipiant, role)

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import click
from typing import Optional


from parsec.utils import trio_run
from parsec.api.protocol import UserID, RealmRole
from parsec.api.data import EntryName
from parsec.cli_utils import cli_exception_handler
from parsec.core import logged_core_factory
from parsec.core.types import WorkspaceRole, LocalDevice
from parsec.core.config import CoreConfig
from parsec.core.cli.utils import cli_command_base_options, core_config_and_device_options


WORKSPACE_ROLE_CHOICES = {"NONE": None, **{role.value: role for role in WorkspaceRole.values}}


async def _share_workspace(
    config: CoreConfig,
    device: LocalDevice,
    name: EntryName,
    user_id: Optional[UserID],
    recipiant: Optional[str],
    user_role: RealmRole,
) -> None:
    if recipiant and user_id or not recipiant and not user_id:
        raise click.ClickException("Either --recipiant or --user-id should be used, but not both")
    async with logged_core_factory(config, device) as core:
        workspace = core.find_workspace_from_name(name)
        if recipiant:
            user_info_tab, nb = await core.find_humans(
                recipiant, page=1, per_page=100, omit_revoked=True, omit_non_human=False
            )
            if nb == 0:
                raise RuntimeError("Unknown recipiant")
            if nb != 1:
                for user in user_info_tab:
                    click.echo(f"{user.human_handle} - UserID: {user.user_id.str}")
                raise RuntimeError("Specify the user more precisely or use the --user-id option")
            user_id = user_info_tab[0].user_id
        await core.user_fs.workspace_share(workspace.id, user_id, user_role)


@click.command(short_help="share workspace")
@click.option("--workspace-name", required=True, type=EntryName)
@click.option("--user-id", type=UserID)
@click.option("--recipiant", help="Name or email to whom the workspace is shared with")
@click.option(
    "--role", required=True, type=click.Choice(WORKSPACE_ROLE_CHOICES.keys(), case_sensitive=False)
)
@core_config_and_device_options
@cli_command_base_options
def share_workspace(
    config: CoreConfig,
    device: LocalDevice,
    workspace_name: EntryName,
    user_id: Optional[UserID],
    recipiant: Optional[str],
    role: RealmRole,
    **kwargs,
) -> None:
    """
    Share a workspace with someone
    """
    role = WORKSPACE_ROLE_CHOICES[role]
    with cli_exception_handler(config.debug):
        trio_run(_share_workspace, config, device, workspace_name, user_id, recipiant, role)

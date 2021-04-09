# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import click

from parsec.utils import trio_run
from parsec.api.protocol import UserID
from parsec.cli_utils import cli_exception_handler
from parsec.core import logged_core_factory
from parsec.core.cli.utils import core_config_and_device_options
from parsec.core.types import WorkspaceRole


def cli_role_to_workspacerole(role):
    roles = {
        "READER": WorkspaceRole.READER,
        "CONTRIBUTOR": WorkspaceRole.CONTRIBUTOR,
        "MANAGER": WorkspaceRole.MANAGER,
        "OWNER": WorkspaceRole.OWNER,
    }
    return roles[role]


async def _share_workspace(config, device, name, user_id, user_role):
    async with logged_core_factory(config, device) as core:
        role = cli_role_to_workspacerole(user_role)
        await core.user_fs.workspace_share(f"/{name}", user_id, role)


@click.command(short_help="share workspace")
@core_config_and_device_options
@click.argument("workspace_name")
@click.argument("user_id", type=UserID, required=True)
@click.argument(
    "user_role", default="READER", type=click.Choice(("READER", "CONTRIBUTOR", "MANAGER", "OWNER"))
)
def share_workspace(config, device, workspace_name, user_id, user_role, **kwargs):
    """
    Create a new workspace for the given device.
    """
    with cli_exception_handler(config.debug):
        trio_run(_share_workspace, config, device, workspace_name, user_id, user_role)

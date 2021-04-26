# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import click

from parsec.utils import trio_run
from parsec.api.protocol import UserID
from parsec.cli_utils import cli_exception_handler
from parsec.core import logged_core_factory
from parsec.core.cli.utils import core_config_and_device_options
from parsec.core.types import WorkspaceRole


class WorkspaceRoleChoice(click.Choice):
    STR_TO_ROLE = {"NONE": None, **{role.value: role for role in WorkspaceRole}}

    def __init__(self, **kwargs):
        super().__init__(self.STR_TO_ROLE, **kwargs)

    def convert(self, value, param, ctx):
        ret = super().convert(value, param, ctx)
        return self.STR_TO_ROLE[ret.upper()]


async def _share_workspace(config, device, name, user_id, user_role):
    async with logged_core_factory(config, device) as core:
        await core.user_fs.workspace_share(f"/{name}", user_id, user_role)


@click.command(short_help="share workspace")
@core_config_and_device_options
@click.argument("workspace_name")
@click.argument("user_id", type=UserID, required=True)
@click.option("--role", type=WorkspaceRoleChoice(case_sensitive=False))
def share_workspace(config, device, workspace_name, user_id, role, **kwargs):
    """
    Create a new workspace for the given device.
    """
    with cli_exception_handler(config.debug):
        trio_run(_share_workspace, config, device, workspace_name, user_id, role)

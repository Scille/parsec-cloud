# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import click


from parsec.utils import trio_run
from parsec.api.data import EntryName
from parsec.cli_utils import cli_exception_handler, spinner
from parsec.core import logged_core_factory
from parsec.core.types import LocalDevice
from parsec.core.config import CoreConfig
from parsec.core.cli.utils import cli_command_base_options, core_config_and_device_options


async def _reencrypt_workspace(config: CoreConfig, device: LocalDevice, name: EntryName) -> None:
    async with logged_core_factory(config, device) as core:
        workspace = core.find_workspace_from_name(name)
        workspace_id = workspace.id
        workspace_fs = core.user_fs.get_workspace(workspace_id)
        reenc_needs = await workspace_fs.get_reencryption_need()
        if not reenc_needs.need_reencryption:
            raise RuntimeError("The workspace does not need to be reencrypted")
        async with spinner("Reencrypting the workspace"):
            if reenc_needs.reencryption_already_in_progress:
                job = await core.user_fs.workspace_continue_reencryption(workspace_id)
            else:
                job = await core.user_fs.workspace_start_reencryption(workspace_id)
            while True:
                total, done = await job.do_one_batch()
                if total == done:
                    break
        click.echo("The workspace has been reencrypted")


@click.command(short_help="reencrypt workspace")
@click.option("--workspace-name", required=True, type=EntryName)
@core_config_and_device_options
@cli_command_base_options
def reencrypt_workspace(
    config: CoreConfig, device: LocalDevice, workspace_name: EntryName, **kwargs
) -> None:
    """
    Reencrypt a workspace
    """
    with cli_exception_handler(config.debug):
        trio_run(_reencrypt_workspace, config, device, workspace_name)

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import click

from parsec.utils import trio_run
from parsec.core import logged_core_factory
from parsec.core.config import CoreConfig
from parsec.core.types import LocalDevice
from parsec.core.cli.utils import cli_command_base_options, core_config_and_device_options
from parsec.cli_utils import cli_exception_handler


async def _human_find(
    config: CoreConfig,
    device: LocalDevice,
    query: str,
    omit_revoked: bool,
    omit_non_human: bool = False,
    page: int = 1,
    per_page: int = 100,
) -> None:
    async with logged_core_factory(config, device) as core:
        user_info_tab, nb = await core.find_humans(
            query, page=1, per_page=100, omit_revoked=omit_revoked, omit_non_human=False
        )
    for user in user_info_tab:
        is_revoked = " (revoked)" if user.revoked_on is not None else ""
        click.echo(f"{user.human_handle} - UserID: {user.user_id}{is_revoked}")
    if not nb:
        click.echo("No human found!")


@click.command(short_help="Retrieve user ID from human email/label")
@click.argument("query", type=str)
@click.option("--include-revoked", is_flag=True)
@core_config_and_device_options
@cli_command_base_options
def human_find(
    config: CoreConfig, device: LocalDevice, query: str, include_revoked: bool, **kwargs
) -> None:
    with cli_exception_handler(config.debug):
        trio_run(_human_find, config, device, query, not include_revoked)

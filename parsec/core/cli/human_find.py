# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import click

from parsec.utils import trio_run
from parsec.core import logged_core_factory
from parsec.core.cli.utils import cli_command_base_options, core_config_and_device_options
from parsec.cli_utils import cli_exception_handler


async def _human_find(
    config,
    device,
    query,
    page: int = 1,
    per_page: int = 100,
    omit_revoked: bool = False,
    omit_non_human: bool = False,
):
    async with logged_core_factory(config, device) as core:
        user_info_tab, nb = await core.find_humans(
            query, page, per_page, omit_revoked, omit_non_human
        )
    for user in user_info_tab:
        click.echo(f"{user.human_handle} - UserID: {user.user_id}")
    if not nb:
        click.echo("No human found!")


@click.command(short_help="search user id from email address")
@click.argument("query", type=str)
@core_config_and_device_options
@cli_command_base_options
def human_find(config, device, query, **kwargs) -> dict:
    with cli_exception_handler(config.debug):
        trio_run(_human_find, config, device, query)

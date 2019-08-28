# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import click

from parsec.utils import trio_run
from parsec.cli_utils import spinner, cli_exception_handler
from parsec.api.protocol import UserID
from parsec.core.invite_claim import generate_invitation_token, invite_and_create_user
from parsec.core.cli.utils import core_config_and_device_options


async def _invite_user(config, device, invited_user_id, admin):
    token = generate_invitation_token()

    organization_addr_display = click.style(device.organization_addr, fg="yellow")
    token_display = click.style(token, fg="yellow")
    click.echo(f"Backend url: {organization_addr_display}")
    click.echo(f"Invitation token: {token_display}")

    async with spinner("Waiting for invitation reply"):
        invite_device_id = await invite_and_create_user(device, invited_user_id, token, admin)

    display_device = click.style(invite_device_id, fg="yellow")
    click.echo(f"Device {display_device} has been created")


@click.command()
@core_config_and_device_options
@click.option("--admin", is_flag=True)
@click.argument("invited_user_id", type=UserID, required=True)
def invite_user(config, device, admin, invited_user_id, **kwargs):
    with cli_exception_handler(config.debug):
        trio_run(_invite_user, config, device, invited_user_id, admin)

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import click

from parsec.utils import trio_run
from parsec.cli_utils import spinner, cli_exception_handler
from parsec.api.protocol import DeviceName
from parsec.core.invite_claim import generate_invitation_token, invite_and_create_device
from parsec.core.cli.utils import core_config_and_device_options


async def _invite_device(config, device, new_device_name):
    token = generate_invitation_token()

    organization_addr_display = click.style(device.organization_addr, fg="yellow")
    token_display = click.style(token, fg="yellow")
    click.echo(f"Backend url: {organization_addr_display}")
    click.echo(f"Invitation token: {token_display}")

    async with spinner("Waiting for invitation reply"):
        await invite_and_create_device(device, new_device_name, token)

    display_device = click.style(f"{device.device_name}@{new_device_name}", fg="yellow")
    click.echo(f"Device {display_device} is ready !")


@click.command()
@core_config_and_device_options
@click.argument("new_device_name", type=DeviceName, required=True)
def invite_device(config, device, new_device_name, **kwargs):
    with cli_exception_handler(config.debug):
        trio_run(_invite_device, config, device, new_device_name)

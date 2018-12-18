import trio
import click

from parsec.spinner import spinner
from parsec.types import UserID
from parsec.core import logged_core_factory
from parsec.core.invite_claim import (
    InviteClaimError,
    generate_invitation_token,
    invite_and_create_user,
)
from parsec.core.cli.utils import core_config_and_device_options


async def _invite_user(config, device, invited_user_id):
    async with logged_core_factory(config, device) as core:
        try:
            token = generate_invitation_token(config.invitation_token_size)

            backend_addr_display = click.style(core.device.backend_addr, fg="yellow")
            token_display = click.style(token, fg="yellow")
            click.echo(f"Backend url: {backend_addr_display}")
            click.echo(f"Invitation token: {token_display}")

            async with spinner("Waiting for invitation reply"):
                invite_device_id = await invite_and_create_user(core, invited_user_id, token)
            click.secho("✔", fg="green")

            display_device = click.style(invite_device_id, fg="yellow")
            click.echo(f"Device {display_device} has been created")

        except Exception as exc:
            click.secho("✘", fg="red")
            click.echo(click.style("Error: ", fg="red") + str(exc))
            if config.debug:
                raise
            else:
                raise SystemExit(1)


@click.command()
@core_config_and_device_options
@click.argument("invited_user_id", type=UserID, required=True)
def invite_user(config, device, invited_user_id, **kwargs):
    from tests.monitor import Monitor

    trio.run(_invite_user, config, device, invited_user_id, instruments=[Monitor()])

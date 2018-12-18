import os
import trio
import click
import pendulum

from parsec.logging import configure_logging
from parsec.spinner import spinner, operation
from parsec.types import DeviceID, BackendOrganizationBootstrapAddr
from parsec.crypto import SigningKey
from parsec.trustchain import certify_user, certify_device
from parsec.api.constants import root_device_id
from parsec.core.config import get_default_config_dir
from parsec.core.backend_connection import backend_anonymous_cmds_factory
from parsec.core.devices_manager import (
    generate_new_device,
    save_device_with_password,
    DeviceConfigAleadyExists,
)


async def _bootstrap_organization(debug, device_id, organization_bootstrap_addr, config_dir, force):
    root_signing_key = SigningKey.generate()
    root_verify_key = root_signing_key.verify_key
    organization_addr = organization_bootstrap_addr.generate_organization_addr(root_verify_key)

    device_display = click.style(device_id, fg="yellow")
    password = click.prompt("Device's password", hide_input=True, confirmation_prompt=True)
    device = generate_new_device(device_id, organization_addr, root_verify_key)

    try:
        with operation(f"Creating locally {device_display}"):
            save_device_with_password(config_dir, device, password, force=force)

    except DeviceConfigAleadyExists as exc:
        raise ValueError("Device already exists.") from exc

    now = pendulum.now()
    certified_user = certify_user(
        root_device_id, root_signing_key, device.user_id, device.public_key, now
    )
    certified_device = certify_device(
        root_device_id, root_signing_key, device_id, device.verify_key, now
    )

    async with spinner(f"Sending {device_display} to server"):
        async with backend_anonymous_cmds_factory(organization_bootstrap_addr) as cmds:
            await cmds.organization_bootstrap(
                organization_bootstrap_addr.get_organization(),
                organization_bootstrap_addr.get_bootstrap_token(),
                root_verify_key,
                certified_user,
                certified_device,
            )

    organization_addr_display = click.style(organization_addr, fg="yellow")
    click.echo(f"Organization url: {organization_addr_display}")


@click.command()
@click.argument("device", type=DeviceID, required=True)
@click.option("--addr", "-B", type=BackendOrganizationBootstrapAddr, required=True)
@click.option("--config-dir", type=click.Path(exists=True, file_okay=False))
@click.option("--force", is_flag=True)
def bootstrap_organization(device, addr, config_dir, force):
    config_dir = config_dir or get_default_config_dir(os.environ)
    debug = "DEBUG" in os.environ
    configure_logging(log_level="DEBUG" if debug else "WARNING")

    try:
        trio.run(_bootstrap_organization, debug, device, addr, config_dir, force)

    except Exception as exc:
        click.echo(click.style("Error: ", fg="red") + str(exc))
        if debug:
            raise
        else:
            raise SystemExit(1)

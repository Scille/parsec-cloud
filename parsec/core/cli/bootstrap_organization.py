# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import trio
import click
import pendulum
from pathlib import Path

from parsec.logging import configure_logging
from parsec.cli_utils import spinner, operation, cli_exception_handler
from parsec.types import DeviceID, BackendOrganizationBootstrapAddr
from parsec.crypto import SigningKey
from parsec.trustchain import certify_user, certify_device
from parsec.core.config import get_default_config_dir
from parsec.core.backend_connection import backend_anonymous_cmds_factory
from parsec.core.devices_manager import generate_new_device, save_device_with_password


async def _bootstrap_organization(
    debug, device_id, organization_bootstrap_addr, config_dir, force, password
):
    root_signing_key = SigningKey.generate()
    root_verify_key = root_signing_key.verify_key
    organization_addr = organization_bootstrap_addr.generate_organization_addr(root_verify_key)

    device_display = click.style(device_id, fg="yellow")
    device = generate_new_device(device_id, organization_addr)

    with operation(f"Creating locally {device_display}"):
        save_device_with_password(config_dir, device, password, force=force)

    now = pendulum.now()
    certified_user = certify_user(None, root_signing_key, device.user_id, device.public_key, now)
    certified_device = certify_device(None, root_signing_key, device_id, device.verify_key, now)

    async with spinner(f"Sending {device_display} to server"):
        async with backend_anonymous_cmds_factory(organization_bootstrap_addr) as cmds:
            await cmds.organization_bootstrap(
                organization_bootstrap_addr.organization_id,
                organization_bootstrap_addr.bootstrap_token,
                root_verify_key,
                certified_user,
                certified_device,
            )

    organization_addr_display = click.style(organization_addr, fg="yellow")
    click.echo(f"Organization url: {organization_addr_display}")


@click.command(short_help="configure new organization")
@click.argument("device", type=DeviceID, required=True)
@click.option("--addr", "-B", type=BackendOrganizationBootstrapAddr, required=True)
@click.option("--config-dir", type=click.Path(exists=True, file_okay=False))
@click.option("--force", is_flag=True)
@click.password_option()
def bootstrap_organization(device, addr, config_dir, force, password):
    """
    Configure the organization and register it first user&device.
    """

    config_dir = Path(config_dir) if config_dir else get_default_config_dir(os.environ)
    debug = "DEBUG" in os.environ
    configure_logging(log_level="DEBUG" if debug else "WARNING")

    with cli_exception_handler(debug):
        trio.run(_bootstrap_organization, debug, device, addr, config_dir, force, password)

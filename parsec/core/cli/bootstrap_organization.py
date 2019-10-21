# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import click
import pendulum
from pathlib import Path

from parsec.utils import trio_run
from parsec.logging import configure_logging
from parsec.cli_utils import spinner, operation, cli_exception_handler
from parsec.crypto import SigningKey
from parsec.api.protocol import DeviceID
from parsec.api.data import UserCertificateContent, DeviceCertificateContent
from parsec.core.types import BackendOrganizationBootstrapAddr
from parsec.core.config import get_default_config_dir
from parsec.core.backend_connection import backend_anonymous_cmds_factory
from parsec.core.local_device import generate_new_device, save_device_with_password


async def _bootstrap_organization(
    debug, device_id, organization_bootstrap_addr, config_dir, force, password
):
    root_signing_key = SigningKey.generate()
    root_verify_key = root_signing_key.verify_key
    organization_addr = organization_bootstrap_addr.generate_organization_addr(root_verify_key)

    device_display = click.style(device_id, fg="yellow")
    device = generate_new_device(device_id, organization_addr, True)

    with operation(f"Creating locally {device_display}"):
        save_device_with_password(config_dir, device, password, force=force)

    now = pendulum.now()
    user_certificate = UserCertificateContent(
        author=None,
        timestamp=now,
        user_id=device.user_id,
        public_key=device.public_key,
        is_admin=device.is_admin,
    ).dump_and_sign(root_signing_key)
    device_certificate = DeviceCertificateContent(
        author=None, timestamp=now, device_id=device_id, verify_key=device.verify_key
    ).dump_and_sign(root_signing_key)

    async with spinner(f"Sending {device_display} to server"):
        async with backend_anonymous_cmds_factory(organization_bootstrap_addr) as cmds:
            await cmds.organization_bootstrap(
                organization_bootstrap_addr.organization_id,
                organization_bootstrap_addr.token,
                root_verify_key,
                user_certificate,
                device_certificate,
            )

    organization_addr_display = click.style(organization_addr.to_url(), fg="yellow")
    click.echo(f"Organization url: {organization_addr_display}")


@click.command(short_help="configure new organization")
@click.argument("device", type=DeviceID, required=True)
@click.option("--addr", "-B", type=BackendOrganizationBootstrapAddr.from_url, required=True)
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
        trio_run(_bootstrap_organization, debug, device, addr, config_dir, force, password)

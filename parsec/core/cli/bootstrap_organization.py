# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import click
import platform

from parsec.utils import trio_run
from parsec.cli_utils import spinner, operation, cli_exception_handler, aprompt
from parsec.api.protocol import HumanHandle
from parsec.core.types import BackendOrganizationBootstrapAddr
from parsec.core.backend_connection import apiv1_backend_anonymous_cmds_factory
from parsec.core.local_device import save_device_with_password
from parsec.core.invite import bootstrap_organization as do_bootstrap_organization
from parsec.core.cli.utils import cli_command_base_options, core_config_options


async def _bootstrap_organization(config, addr, password, device_label, human_label, human_email):
    if not human_label:
        human_label = await aprompt("User fullname")
    if not human_email:
        human_email = await aprompt("User email")
    human_handle = HumanHandle(email=human_email, label=human_label)
    if not device_label:
        device_label = await aprompt("Device label", default=platform.node())

    async with apiv1_backend_anonymous_cmds_factory(addr=addr) as cmds:
        async with spinner("Bootstrapping organization in the backend"):
            new_device = await do_bootstrap_organization(
                cmds=cmds, human_handle=human_handle, device_label=device_label
            )

        device_display = click.style(new_device.slughash, fg="yellow")
        # We don't have to worry about overwritting an existing keyfile
        # given their names are base on the device's slughash which is intended
        # to be globally unique.
        with operation(f"Saving device {device_display}"):
            save_device_with_password(
                config_dir=config.config_dir, device=new_device, password=password
            )


@click.command(short_help="configure new organization")
@click.argument("addr", type=BackendOrganizationBootstrapAddr.from_url)
@click.password_option(prompt="Choose a password for the device")
@click.option("--device-label")
@click.option("--human-label")
@click.option("--human-email")
@core_config_options
@cli_command_base_options
def bootstrap_organization(
    config, addr, password, device_label, human_label, human_email, **kwargs
):
    """
    Configure the organization and register it first user&device.
    """
    with cli_exception_handler(config.debug):
        # Disable task monitoring given user prompt will block the coroutine
        trio_run(
            _bootstrap_organization, config, addr, password, device_label, human_label, human_email
        )

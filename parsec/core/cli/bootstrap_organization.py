# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import click
import platform

from parsec.utils import trio_run
from parsec.cli_utils import spinner, cli_exception_handler, aprompt
from parsec.api.protocol import HumanHandle
from parsec.core.types import BackendOrganizationBootstrapAddr
from parsec.core.backend_connection import apiv1_backend_anonymous_cmds_factory
from parsec.core.fs.storage.user_storage import user_storage_non_speculative_init
from parsec.core.invite import bootstrap_organization as do_bootstrap_organization
from parsec.core.cli.utils import cli_command_base_options, core_config_options, save_device_options


async def _bootstrap_organization(
    config, addr, device_label, human_label, human_email, save_device_with_selected_auth
):
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

        # We don't have to worry about overwritting an existing keyfile
        # given their names are base on the device's slughash which is intended
        # to be globally unique.

        # The organization is brand new, of course there is no existing
        # remote user manifest, hence our placeholder is non-speculative.
        await user_storage_non_speculative_init(
            data_base_dir=config.data_base_dir, device=new_device
        )
        await save_device_with_selected_auth(config_dir=config.config_dir, device=new_device)


@click.command(short_help="configure new organization")
@click.argument("addr", type=BackendOrganizationBootstrapAddr.from_url)
@click.option("--device-label")
@click.option("--human-label")
@click.option("--human-email")
@save_device_options
@core_config_options
@cli_command_base_options
def bootstrap_organization(
    config, addr, device_label, human_label, human_email, save_device_with_selected_auth, **kwargs
):
    """
    Configure the organization and register it first user&device.
    """
    with cli_exception_handler(config.debug):
        # Disable task monitoring given user prompt will block the coroutine
        trio_run(
            _bootstrap_organization,
            config,
            addr,
            device_label,
            human_label,
            human_email,
            save_device_with_selected_auth,
        )

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import trio
import click

from parsec.types import BackendOrganizationAddr, DeviceID
from parsec.cli_utils import spinner, operation, cli_exception_handler
from parsec.core.cli.utils import core_config_options
from parsec.core.devices_manager import save_device_with_password, save_device_with_pkcs11
from parsec.core.invite_claim import claim_user as actual_claim_user
from parsec.core.backend_connection import backend_anonymous_cmds_factory


async def _claim_user(config, backend_addr, token, new_device_id, password, pkcs11):
    async with backend_anonymous_cmds_factory(backend_addr) as cmds:

        async with spinner("Waiting for referee to reply"):
            device = await actual_claim_user(cmds, new_device_id, token)

    device_display = click.style(new_device_id, fg="yellow")
    with operation(f"Saving locally {device_display}"):
        if pkcs11:
            token_id = click.prompt("PCKS11 token id", type=int)
            key_id = click.prompt("PCKS11 key id", type=int)
            save_device_with_pkcs11(config.config_dir, device, token_id, key_id)

        else:
            save_device_with_password(config.config_dir, device, password)


@click.command()
@core_config_options
@click.argument("device", type=DeviceID, required=True)
@click.option("--token", required=True)
@click.option("--addr", "-B", required=True, type=BackendOrganizationAddr)
@click.password_option()
@click.option("--pkcs11", is_flag=True)
def claim_user(config, addr, device, token, password, pkcs11, **kwargs):
    if password and pkcs11:
        raise SystemExit("Password are PKCS11 options are exclusives.")

    debug = "DEBUG" in os.environ
    with cli_exception_handler(debug):
        trio.run(_claim_user, config, addr, token, device, password, pkcs11)

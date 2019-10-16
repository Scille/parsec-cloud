# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import click
import platform

from parsec.utils import trio_run
from parsec.api.protocol import DeviceID
from parsec.cli_utils import spinner, operation, cli_exception_handler
from parsec.core.types import BackendOrganizationClaimUserAddr
from parsec.core.cli.utils import core_config_options
from parsec.core.local_device import save_device_with_password, save_device_with_pkcs11
from parsec.core.invite_claim import claim_user as actual_claim_user


async def _claim_user(config, organization_addr, new_device_id, token, password, pkcs11):

    async with spinner("Waiting for referee to reply"):
        device = await actual_claim_user(
            organization_addr=organization_addr,
            new_device_id=new_device_id,
            token=token,
            keepalive=config.backend_connection_keepalive,
        )

    device_display = click.style(new_device_id, fg="yellow")
    with operation(f"Saving locally {device_display}"):
        if pkcs11:
            token_id = click.prompt("PCKS11 token id", type=int)
            key_id = click.prompt("PCKS11 key id", type=int)
            save_device_with_pkcs11(config.config_dir, device, token_id, key_id)

        else:
            save_device_with_password(config.config_dir, device, password)


@click.command()
@click.argument("device-name", default=platform.node())
@core_config_options
@click.option("--addr", required=True, type=BackendOrganizationClaimUserAddr.from_url)
@click.option("--token")
@click.password_option()
@click.option("--pkcs11", is_flag=True)
def claim_user(config, addr, device_name, token, password, pkcs11, **kwargs):
    try:
        new_device_id = DeviceID(f"{addr.user_id}@{device_name}")
    except ValueError:
        raise SystemExit(f"Invalid device name value `{device_name}`")

    if token and addr.token:
        raise SystemExit("token already specified in the address")
    token = token or addr.token
    if not token:
        raise SystemExit("Missing token value")

    if password and pkcs11:
        raise SystemExit("Password are PKCS11 options are exclusives.")

    debug = "DEBUG" in os.environ
    with cli_exception_handler(debug):
        trio_run(
            _claim_user, config, addr.to_organization_addr(), new_device_id, token, password, pkcs11
        )

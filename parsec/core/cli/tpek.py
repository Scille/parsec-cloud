# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import click
from parsec.api.protocol.tpek import TpekServiceType
from parsec.core.cli.utils import cli_command_base_options, core_config_and_device_options

from parsec.core.config import CoreConfig
from parsec.core.types.local_device import LocalDevice
from pathlib import Path


TPEK_SERVICE_CHOICES = {role.value: role for role in TpekServiceType}


# TODO update service key
@click.command(short_help="Register a new encryption key for a parsec service")
@click.argument(
    "der_encryption_key_file",
    type=click.Path(exists=True, file_okay=True, dir_okay=False, path_type=Path),
)
@click.argument(
    "der_signing_key_file",
    type=click.Path(exists=True, file_okay=True, dir_okay=False, path_type=Path),
)
@click.option(
    "--service_type",
    required=True,
    type=click.Choice(TPEK_SERVICE_CHOICES.keys(), case_sensitive=False),
)
@core_config_and_device_options
@cli_command_base_options
def tpek_register_service(
    config: CoreConfig,
    device: LocalDevice,
    service_type: TpekServiceType,
    der_signing_key_file: Path,
    der_encryption_key_file: Path,
    **kwargs
):
    pass

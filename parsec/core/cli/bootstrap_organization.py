# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from pathlib import Path
import click
import platform
from typing import Optional, Callable

from parsec.sequester_crypto import SequesterVerifyKeyDer
from parsec.utils import trio_run
from parsec.cli_utils import spinner, cli_exception_handler, aprompt, aconfirm
from parsec.api.protocol import HumanHandle, DeviceLabel
from parsec.core.config import CoreConfig
from parsec.core.types import BackendOrganizationBootstrapAddr
from parsec.core.fs.storage.user_storage import user_storage_non_speculative_init
from parsec.core.invite import bootstrap_organization as do_bootstrap_organization
from parsec.core.cli.utils import cli_command_base_options, core_config_options, save_device_options


SEQUESTER_BRIEF = """A sequestered organization is able to ask it users to encrypt
their data with third party asymetric keys (called "sequester service").

Those "sequester services" must themselve be signed by the "sequester authority" key
that is configured during organization bootstrap.
This have the following implications:
- All data remain encrypted (with the Parsec server incapable of reading them)
- The sequester services are in position to bypass end-to-end user encryption
- The Parsec Server has no way of modifying the sequester services keys
- Sequester services can be added and removed to handle key rotation
- A regular organization cannot be turned in a sequestered one (and vice-versa)

A typicall usecase for sequestered organization is data recovery or legal requirement for
company to have access to all data produced by former employees.
"""


async def _bootstrap_organization(
    config: CoreConfig,
    addr: BackendOrganizationBootstrapAddr,
    device_label: Optional[str],
    human_label: Optional[str],
    human_email: Optional[str],
    save_device_with_selected_auth: Callable,
    sequester_verify_key: Optional[Path],
) -> None:
    if sequester_verify_key is not None:
        answer = await aconfirm(
            f"""You are about to bootstrap a sequestered organization.

{SEQUESTER_BRIEF}

File {sequester_verify_key} is going to be use has sequester authority private key.
Do you want to continue ?""",
            default=False,
        )
        if not answer:
            raise SystemExit("Bootstrap aborted")
        sequester_verify_key = SequesterVerifyKeyDer(sequester_verify_key.read_bytes())

    if not human_label:
        human_label = await aprompt("User fullname")
    if not human_email:
        human_email = await aprompt("User email")
    human_handle = HumanHandle(email=human_email, label=human_label)
    if not device_label:
        device_label_raw = await aprompt("Device label", default=platform.node())
        device_label = DeviceLabel(device_label_raw)
    else:
        device_label = DeviceLabel(device_label)
    async with spinner("Bootstrapping organization in the backend"):
        new_device = await do_bootstrap_organization(
            addr,
            human_handle=human_handle,
            device_label=device_label,
            sequester_authority_verify_key=sequester_verify_key,
        )

    # We don't have to worry about overwritting an existing keyfile
    # given their names are base on the device's slughash which is intended
    # to be globally unique.

    # The organization is brand new, of course there is no existing
    # remote user manifest, hence our placeholder is non-speculative.
    await user_storage_non_speculative_init(data_base_dir=config.data_base_dir, device=new_device)
    await save_device_with_selected_auth(config_dir=config.config_dir, device=new_device)


@click.command(short_help="configure new organization")
@click.argument("addr", type=BackendOrganizationBootstrapAddr.from_url)
@click.option("--device-label")
@click.option("--human-label")
@click.option("--human-email")
@click.option(
    "--sequester-verify-key",
    type=click.Path(exists=True, file_okay=True, dir_okay=False, path_type=Path),
    help=f"Enable sequestered organization feature.\n{SEQUESTER_BRIEF}",
)
@save_device_options
@core_config_options
@cli_command_base_options
def bootstrap_organization(
    config: CoreConfig,
    addr: BackendOrganizationBootstrapAddr,
    device_label: Optional[str],
    human_label: Optional[str],
    human_email: Optional[str],
    save_device_with_selected_auth: Callable,
    sequester_verify_key: Optional[Path],
    **kwargs,
) -> None:
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
            sequester_verify_key,
        )

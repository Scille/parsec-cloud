# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from pathlib import Path
import click
import platform
from typing import Any, Protocol

from parsec.sequester_crypto import SequesterVerifyKeyDer
from parsec.utils import trio_run
from parsec.cli_utils import spinner, cli_exception_handler, async_prompt, async_confirm
from parsec.api.protocol import HumanHandle, DeviceLabel
from parsec.core.config import CoreConfig
from parsec.core.types import BackendOrganizationBootstrapAddr
from parsec.core.fs.storage.user_storage import user_storage_non_speculative_init
from parsec.core.invite import bootstrap_organization as do_bootstrap_organization
from parsec.core.cli.utils import cli_command_base_options, core_config_options, save_device_options

from parsec._parsec import LocalDevice


SEQUESTER_BRIEF = """A sequestered organization is able to ask it users to encrypt
their data with third party asymmetric keys (called "sequester service").

Those "sequester services" must themselves be signed by the "sequester authority" key
that is configured during organization bootstrap.
This have the following implications:
- All data remain encrypted (with the Parsec server incapable of reading them)
- The sequester services are in position to bypass end-to-end user encryption
- The Parsec Server has no way of modifying the sequester services keys
- Sequester services can be added and removed to handle key rotation
- A regular organization cannot be turned in a sequestered one (and vice-versa)

A typical use case for sequestered organization is data recovery or legal requirement for
company to have access to all data produced by former employees.
"""


class SaveDeviceWithSelectedAuth(Protocol):
    def __call__(self, config_dir: Path, device: LocalDevice) -> Any:
        pass


async def _bootstrap_organization(
    config: CoreConfig,
    addr: BackendOrganizationBootstrapAddr,
    device_label: str | None,
    human_label: str | None,
    human_email: str | None,
    save_device_with_selected_auth: SaveDeviceWithSelectedAuth,
    sequester_verify_key: Path | None,
) -> None:
    sequester_vrf_key = None
    if sequester_verify_key is not None:
        answer = await async_confirm(
            f"""You are about to bootstrap a sequestered organization.

{SEQUESTER_BRIEF}

File {sequester_verify_key} is going to be use as sequester authority key.
Do you want to continue ?""",
            default=False,
        )
        if not answer:
            raise SystemExit("Bootstrap aborted")
        sequester_vrf_key = SequesterVerifyKeyDer(sequester_verify_key.read_bytes())

    human_label: str = human_label or await async_prompt("User fullname")
    human_email: str = human_email or await async_prompt("User email")
    human_handle = HumanHandle(email=human_email, label=human_label)
    if not device_label:
        device_label_raw: str = await async_prompt("Device label", default=platform.node())
        dev_label = DeviceLabel(device_label_raw)
    else:
        dev_label = DeviceLabel(device_label)
    async with spinner("Bootstrapping organization in the backend"):
        new_device = await do_bootstrap_organization(
            addr,
            human_handle=human_handle,
            device_label=dev_label,
            sequester_authority_verify_key=sequester_vrf_key,
        )

    # We don't have to worry about overwriting an existing keyfile
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
    device_label: str | None,
    human_label: str | None,
    human_email: str | None,
    save_device_with_selected_auth: SaveDeviceWithSelectedAuth,
    sequester_verify_key: Path | None,
    **kwargs: Any,
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

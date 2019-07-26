# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import click

from parsec.core.cli import list_devices
from parsec.core.cli import invite_user
from parsec.core.cli import claim_user
from parsec.core.cli import invite_device
from parsec.core.cli import claim_device
from parsec.core.cli import create_organization
from parsec.core.cli import create_workspace
from parsec.core.cli import share_workspace
from parsec.core.cli import bootstrap_organization
from parsec.core.cli import run
from parsec.core.cli import profile


__all__ = ("core_cmd",)


@click.group()
def core_cmd():
    pass


core_cmd.add_command(run.run_gui, "gui")
core_cmd.add_command(run.run_mountpoint, "run")
core_cmd.add_command(create_workspace.create_workspace, "create_workspace")
core_cmd.add_command(share_workspace.share_workspace, "share_workspace")
core_cmd.add_command(list_devices.list_devices, "list_devices")
core_cmd.add_command(invite_user.invite_user, "invite_user")
core_cmd.add_command(claim_user.claim_user, "claim_user")
core_cmd.add_command(invite_device.invite_device, "invite_device")
core_cmd.add_command(claim_device.claim_device, "claim_device")
core_cmd.add_command(create_organization.create_organization, "create_organization")
core_cmd.add_command(bootstrap_organization.bootstrap_organization, "bootstrap_organization")
core_cmd.add_command(profile.benchmark, "benchmark")

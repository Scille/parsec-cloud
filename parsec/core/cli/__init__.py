# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import click

from parsec.core.cli import human_find
from parsec.core.cli import list_devices
from parsec.core.cli import status_organization
from parsec.core.cli import recovery
from parsec.core.cli import invitation
from parsec.core.cli import create_organization
from parsec.core.cli import stats_organization
from parsec.core.cli import create_workspace
from parsec.core.cli import share_workspace
from parsec.core.cli import bootstrap_organization
from parsec.core.cli import rsync
from parsec.core.cli import run


__all__ = ("core_cmd",)


@click.group()
def core_cmd():
    pass


core_cmd.add_command(run.run_gui, "gui")
core_cmd.add_command(run.run_mountpoint, "run")
core_cmd.add_command(rsync.run_rsync, "rsync")
core_cmd.add_command(create_workspace.create_workspace, "create_workspace")
core_cmd.add_command(share_workspace.share_workspace, "share_workspace")
core_cmd.add_command(list_devices.list_devices, "list_devices")
core_cmd.add_command(recovery.export_recovery_device, "export_recovery_device")
core_cmd.add_command(recovery.import_recovery_device, "import_recovery_device")
core_cmd.add_command(human_find.human_find, "human_find")

core_cmd.add_command(invitation.invite_user, "invite_user")
core_cmd.add_command(invitation.invite_device, "invite_device")
core_cmd.add_command(invitation.list_invitations, "list_invitations")
core_cmd.add_command(invitation.greet_invitation, "greet_invitation")
core_cmd.add_command(invitation.claim_invitation, "claim_invitation")
core_cmd.add_command(invitation.cancel_invitation, "cancel_invitation")

core_cmd.add_command(create_organization.create_organization, "create_organization")
core_cmd.add_command(stats_organization.stats_organization, "stats_organization")
core_cmd.add_command(status_organization.status_organization, "status_organization")
core_cmd.add_command(bootstrap_organization.bootstrap_organization, "bootstrap_organization")

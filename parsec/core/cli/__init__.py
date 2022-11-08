# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

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
from parsec.core.cli import reencrypt_workspace
from parsec.core.cli import bootstrap_organization
from parsec.core.cli import rsync
from parsec.core.cli import run
from parsec.core.cli import pki


__all__ = ("core_cmd_group",)


@click.group()
def core_cmd_group() -> None:
    pass


core_cmd_group.add_command(run.run_gui, "gui")
core_cmd_group.add_command(run.run_mountpoint, "run")
core_cmd_group.add_command(rsync.run_rsync, "rsync")
core_cmd_group.add_command(create_workspace.create_workspace, "create_workspace")
core_cmd_group.add_command(share_workspace.share_workspace, "share_workspace")
core_cmd_group.add_command(reencrypt_workspace.reencrypt_workspace, "reencrypt_workspace")
core_cmd_group.add_command(list_devices.list_devices, "list_devices")
core_cmd_group.add_command(list_devices.remove_device, "remove_device")
core_cmd_group.add_command(recovery.export_recovery_device, "export_recovery_device")
core_cmd_group.add_command(recovery.import_recovery_device, "import_recovery_device")
core_cmd_group.add_command(human_find.human_find, "human_find")

core_cmd_group.add_command(invitation.invite_user, "invite_user")
core_cmd_group.add_command(invitation.invite_device, "invite_device")
core_cmd_group.add_command(invitation.list_invitations, "list_invitations")
core_cmd_group.add_command(invitation.greet_invitation, "greet_invitation")
core_cmd_group.add_command(invitation.claim_invitation, "claim_invitation")
core_cmd_group.add_command(invitation.cancel_invitation, "cancel_invitation")

core_cmd_group.add_command(create_organization.create_organization, "create_organization")
core_cmd_group.add_command(stats_organization.stats_organization, "stats_organization")
core_cmd_group.add_command(status_organization.status_organization, "status_organization")
core_cmd_group.add_command(stats_organization.stats_server, "stats_server")
core_cmd_group.add_command(bootstrap_organization.bootstrap_organization, "bootstrap_organization")

core_cmd_group.add_command(pki.pki_enrollment_submit, "pki_enrollment_submit")
core_cmd_group.add_command(pki.pki_enrollment_poll, "pki_enrollment_poll")
core_cmd_group.add_command(pki.pki_enrollment_review_pendings, "pki_enrollment_review_pendings")

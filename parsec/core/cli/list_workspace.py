# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any

import click

from parsec._parsec import DateTime
from parsec.cli_utils import cli_exception_handler
from parsec.core import logged_core_factory
from parsec.core.cli.utils import cli_command_base_options, core_config_and_device_options
from parsec.core.config import CoreConfig
from parsec.core.fs.workspacefs import WorkspaceFS
from parsec.core.logged_core import LoggedCore
from parsec.core.types import LocalDevice
from parsec.utils import trio_run


async def _show_workspace(logged_core: LoggedCore, workspace: WorkspaceFS) -> None:
    entry_id = workspace.workspace_id
    workspace_name = workspace.get_workspace_name()
    archiving_configuration, configured_on, _ = workspace.get_archiving_configuration()
    remanence_info = workspace.get_remanence_manager_info()
    is_block_remanent = remanence_info.is_block_remanent
    entry = workspace.get_workspace_entry()
    user_roles = await workspace.get_user_roles()

    styled_id = click.style(entry_id.hex, fg="yellow")
    styled_name = click.style(workspace_name, fg="yellow")
    styled_role = click.style(entry.role.str if entry.role is not None else "revoked", fg="yellow")
    styled_remanence = click.style("enabled" if is_block_remanent else "disabled", fg="yellow")

    if archiving_configuration.is_available():
        styled_archiving = click.style("available", fg="yellow")
    elif archiving_configuration.is_archived():
        styled_archiving = click.style(f"archived", fg="yellow")
    elif archiving_configuration.is_deleted(DateTime.now()):
        styled_archiving = click.style(
            f"deleted since {archiving_configuration.deletion_date.to_rfc3339()}", fg="yellow"
        )
    elif archiving_configuration.is_deletion_planned():
        styled_archiving = click.style(
            f"deletion planned for {archiving_configuration.deletion_date.to_rfc3339()}",
            fg="yellow",
        )
    else:
        assert False
    styled_configured_on = (
        click.style(f"{configured_on.to_rfc3339()}", fg="yellow") if configured_on else None
    )

    click.echo(f"━━━━━━━━━━━━━━  {styled_name}  ━━━━━━━━━━━━━━")
    click.echo(f"Internal ID: {styled_id}")
    click.echo(f"Current role: {styled_role}")
    click.echo(f"Offline availability: {styled_remanence}")
    click.echo(f"Status: {styled_archiving}")
    if styled_configured_on:
        click.echo(f"Status configured on: {styled_configured_on}")

    click.echo(f"Other users:")
    for user_id, other_role in user_roles.items():
        if user_id == logged_core.device.user_id:
            continue
        user_certificate, _ = await logged_core._remote_devices_manager.get_user(user_id)
        if user_certificate.human_handle is not None:
            styled_user_human_handle = click.style(user_certificate.human_handle.str, fg="yellow")
        else:
            styled_user_human_handle = click.style(user_certificate.user_id.str, fg="yellow")
        styled_other_role = click.style(f"{other_role.str}", fg="yellow")
        click.echo(f"- {styled_user_human_handle}: {styled_other_role}")


async def _list_workspaces(config: CoreConfig, device: LocalDevice) -> None:
    async with logged_core_factory(config, device) as core:
        # Force the archiving status to update
        await core.user_fs.update_archiving_status()

        for workspace in core.user_fs.get_available_workspaces():
            await _show_workspace(core, workspace)


@click.command(short_help="list workspaces")
@core_config_and_device_options
@cli_command_base_options
def list_workspaces(config: CoreConfig, device: LocalDevice, **kwargs: Any) -> None:
    """
    Create a new workspace for the given device.
    """
    with cli_exception_handler(config.debug):
        trio_run(_list_workspaces, config, device)

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any

import click

from parsec._parsec import DateTime, RealmArchivingConfiguration
from parsec.api.data import EntryName
from parsec.cli_utils import ParsecDateTimeClickType, cli_exception_handler
from parsec.core import logged_core_factory
from parsec.core.cli.utils import cli_command_base_options, core_config_and_device_options
from parsec.core.config import CoreConfig
from parsec.core.types import LocalDevice
from parsec.utils import trio_run


async def _set_workspace_archiving(
    config: CoreConfig,
    device: LocalDevice,
    name: EntryName,
    configuration: RealmArchivingConfiguration,
) -> None:
    async with logged_core_factory(config, device) as core:
        # Force the archiving status to update
        await core.user_fs.update_archiving_status()

        workspace = core.find_workspace_from_name(name)
        workspace_fs = core.user_fs.get_workspace(workspace.id)
        current_configuration, _, _ = workspace_fs.get_archiving_configuration()
        if current_configuration == configuration:
            raise click.ClickException(f"This configuration is already applied (`{configuration}`)")
        await workspace_fs.configure_archiving(configuration)
        click.echo(f"Configuration `{configuration}` applied!")


@click.command(short_help="archive workspace")
@click.option("--workspace-name", required=True, type=EntryName)
@core_config_and_device_options
@cli_command_base_options
def archive_workspace(
    config: CoreConfig, device: LocalDevice, workspace_name: EntryName, **kwargs: Any
) -> None:
    """
    Create a new workspace for the given device.
    """
    with cli_exception_handler(config.debug):
        trio_run(
            _set_workspace_archiving,
            config,
            device,
            workspace_name,
            RealmArchivingConfiguration.archived(),
        )


@click.command(short_help="plan a workspace deletion")
@click.option("--workspace-name", required=True, type=EntryName)
@click.option("--deletion-date", required=True, type=ParsecDateTimeClickType())
@core_config_and_device_options
@cli_command_base_options
def delete_workspace(
    config: CoreConfig,
    device: LocalDevice,
    workspace_name: EntryName,
    deletion_date: DateTime,
    **kwargs: Any,
) -> None:
    """
    Create a new workspace for the given device.
    """
    with cli_exception_handler(config.debug):
        trio_run(
            _set_workspace_archiving,
            config,
            device,
            workspace_name,
            RealmArchivingConfiguration.deletion_planned(deletion_date),
        )


@click.command(short_help="Unarchive or unplan a workspace deletion")
@click.option("--workspace-name", required=True, type=EntryName)
@core_config_and_device_options
@cli_command_base_options
def unarchive_workspace(
    config: CoreConfig, device: LocalDevice, workspace_name: EntryName, **kwargs: Any
) -> None:
    """
    Create a new workspace for the given device.
    """
    with cli_exception_handler(config.debug):
        trio_run(
            _set_workspace_archiving,
            config,
            device,
            workspace_name,
            RealmArchivingConfiguration.available(),
        )

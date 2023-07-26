# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any

import click

from parsec._parsec import (
    DeviceLabel,
    HumanHandle,
    ShamirRecoveryBriefCertificate,
)
from parsec.cli_utils import async_confirm, async_prompt, cli_exception_handler, spinner
from parsec.core import CoreConfig
from parsec.core.cli.utils import (
    cli_command_base_options,
    core_config_and_device_options,
)
from parsec.core.logged_core import LoggedCore, logged_core_factory
from parsec.core.shamir import (
    ShamirRecoveryAlreadySetError,
    ShamirRecoveryError,
    ShamirRecoveryInvalidCertificationError,
    ShamirRecoveryInvalidDataError,
    ShamirRecoveryNotSetError,
    create_shamir_recovery_device,
    get_shamir_recovery_others_list,
    get_shamir_recovery_self_info,
    remove_shamir_recovery_device,
)
from parsec.core.types import LocalDevice
from parsec.utils import trio_run


async def _create_shared_recovery_device(
    config: CoreConfig, device: LocalDevice, threshold: int
) -> None:
    # Connect to the backend
    async with logged_core_factory(config, device) as core:
        # List all administrators and fetch certificates
        user_infos, total = await core.find_humans(
            query=None, page=1, per_page=100, omit_revoked=True, omit_non_human=True
        )
        assert len(user_infos) == total
        admin_infos = [
            user
            for user in user_infos
            if user.profile == user.profile.ADMIN
            and user.human_handle is not None
            and user.user_id != device.user_id
        ]
        certificates = [
            (await core._remote_devices_manager.get_user(admin_info.user_id))[0]
            for admin_info in admin_infos
        ]

        # Prompt the user for confirmation
        if len(admin_infos) < threshold:
            raise click.UsageError(
                f"There are only {len(admin_infos)} administrator(s) but the configured threshold is {threshold}"
            )
        click.echo("You are about to create and share a recovery device with these administrators:")
        for admin_info in admin_infos:
            user_display = click.style(admin_info.user_display, fg="yellow")
            click.echo(f"- {user_display}")
        threshold_display = click.style(threshold, fg="yellow")
        click.echo(
            f"At least {threshold_display} of them will be necessary to perform the recovery procedure."
        )
        await async_confirm("Do you want to continue?", abort=True)

        try:
            await create_shamir_recovery_device(core, certificates, threshold)
        except ShamirRecoveryInvalidCertificationError:
            raise click.ClickException("Invalid certification")
        except ShamirRecoveryInvalidDataError:
            raise click.ClickException("Invalid data")
        except ShamirRecoveryAlreadySetError:
            raise click.ClickException("Already set")
        except ShamirRecoveryError as exc:
            raise click.ClickException(str(exc))
        click.echo("Shared recovery device successfully created.")


@click.command(short_help="create a new shared recovery device")
@click.option(
    "--threshold",
    "-t",
    required=True,
    help="Number of shared required to perform the recovery",
    type=click.IntRange(min=1),
)
@core_config_and_device_options
@cli_command_base_options
def create_shared_recovery_device(
    config: CoreConfig, device: LocalDevice, threshold: int, **kwargs: Any
) -> None:
    """
    Create a shared recovery device using the shamir algorithm.
    """
    with cli_exception_handler(config.debug):
        trio_run(_create_shared_recovery_device, config, device, threshold)


async def _show_brief_certificate(
    logged_core: LoggedCore,
    human_handle: HumanHandle,
    device_label: DeviceLabel,
    brief_certificate: ShamirRecoveryBriefCertificate,
) -> None:
    styled_human_handle = click.style(human_handle.str, fg="yellow")
    styled_device_label = click.style(device_label.str, fg="yellow")
    styled_timestamp = click.style(brief_certificate.timestamp.to_rfc3339(), fg="yellow")
    styled_threshold = click.style(brief_certificate.threshold, fg="yellow")
    click.echo(f"━━━━━━  Shared recovery device for {styled_human_handle}  ━━━━━━")
    click.echo(f"Configured from device {styled_device_label} on {styled_timestamp}")
    click.echo(f"At least {styled_threshold} shares are required from the following users:")
    for user_id, count in brief_certificate.per_recipient_shares.items():
        user_certificate, _ = await logged_core._remote_devices_manager.get_user(user_id)
        assert user_certificate.human_handle is not None
        styled_user_human_handle = click.style(user_certificate.human_handle.str, fg="yellow")
        styled_share = click.style(count, fg="yellow")
        styled_share = f"{styled_share} share" if count == 1 else f"{styled_share} shares"
        if user_id == logged_core.device.user_id:
            click.echo(f"- {styled_user_human_handle} ({styled_share}) ← This is you")
        else:
            click.echo(f"- {styled_user_human_handle} ({styled_share})")


async def _shared_recovery_device_info(config: CoreConfig, device: LocalDevice) -> bool:
    assert device.human_handle is not None
    styled_human_handle = click.style(device.human_handle.str, fg="yellow")

    # Connect to the server
    async with logged_core_factory(config, device) as core:
        try:
            author_certificate, brief_certificate = await get_shamir_recovery_self_info(core)
        except ShamirRecoveryNotSetError:
            click.echo(f"No shared recovery device configured for {styled_human_handle}")
            return False
        except ShamirRecoveryError as exc:
            raise click.ClickException(str(exc))
        # Indicate a shared recovery device has been found
        assert author_certificate.device_label is not None
        await _show_brief_certificate(
            core,
            device.human_handle,
            author_certificate.device_label,
            brief_certificate,
        )
        return True


@click.command(short_help="get info for the current shared recovery device")
@core_config_and_device_options
@cli_command_base_options
def shared_recovery_device_info(config: CoreConfig, device: LocalDevice, **kwargs: Any) -> None:
    """
    Get information for the user's current shared recovery device
    """
    with cli_exception_handler(config.debug):
        trio_run(_shared_recovery_device_info, config, device)


async def _remove_shared_recovery_device(config: CoreConfig, device: LocalDevice) -> None:
    # Print device and info and return if none has been found
    if not await _shared_recovery_device_info(config, device):
        return

    # Confirmation prompt
    click.echo("You're about to remove this shared recovery device")
    await async_confirm("Do you want to continue?", abort=True)

    # Perform the request
    async with logged_core_factory(config, device) as core:
        await remove_shamir_recovery_device(core)
    click.echo("Shared recovery device successfully removed.")


@click.command(short_help="remove the current shared recovery device")
@core_config_and_device_options
@cli_command_base_options
def remove_shared_recovery_device(config: CoreConfig, device: LocalDevice, **kwargs: Any) -> None:
    """
    Remove the user's current shared recovery device
    """
    with cli_exception_handler(config.debug):
        trio_run(_remove_shared_recovery_device, config, device)


async def _list_shared_recovery_devices(config: CoreConfig, device: LocalDevice) -> None:
    # Perform the request
    async with logged_core_factory(config, device) as core:
        result = await get_shamir_recovery_others_list(core)
        for i, (
            author_certificate,
            user_certificate,
            brief_certificate,
            maybe_share_data,
        ) in enumerate(result):
            share_number = 0 if maybe_share_data is None else len(maybe_share_data.weighted_share)

            # Display brief certificate
            if i != 0:
                click.echo()
            assert author_certificate.device_label is not None
            assert user_certificate.human_handle is not None
            await _show_brief_certificate(
                core,
                user_certificate.human_handle,
                author_certificate.device_label,
                brief_certificate,
            )
            styled_share_number = click.style(share_number, fg="yellow")
            click.echo(f"{styled_share_number} valid share(s) has been successfully retrieved")


@click.command(short_help="list shared recovery devices")
@core_config_and_device_options
@cli_command_base_options
def list_shared_recovery_devices(config: CoreConfig, device: LocalDevice, **kwargs: Any) -> None:
    """
    List the shared recovery devices for other users
    """
    with cli_exception_handler(config.debug):
        trio_run(_list_shared_recovery_devices, config, device)


async def _invite_shared_recovery(
    config: CoreConfig, device: LocalDevice, send_email: bool
) -> None:
    # Connect to the backend
    async with logged_core_factory(config, device) as core:
        result = await get_shamir_recovery_others_list(core)
        for i, (_, user_certificate, _, _) in enumerate(result):
            assert user_certificate.human_handle is not None
            display_choice = click.style(user_certificate.human_handle.str, fg="yellow")
            click.echo(f" {i} - {display_choice}")
        choices = [str(x) for x in range(len(result))]
        choice_index = await async_prompt("User to invite", type=click.Choice(choices))
        _, user_certificate, _, _ = result[int(choice_index)]
        user_id = user_certificate.user_id

        async with spinner("Creating device invitation"):
            address, sent_status = await core.new_shamir_recovery_invitation(
                user_id, send_email=send_email
            )
            if sent_status != sent_status.SUCCESS:
                click.secho(f"Email could not be sent ({sent_status.str})", fg="red")

    action_addr_display = click.style(address.to_url(), fg="yellow")
    click.echo(f"url: {action_addr_display}")


@click.command(short_help="invite shared recovery")
@core_config_and_device_options
@cli_command_base_options
@click.option("--send-email", is_flag=True)
def invite_shared_recovery(
    config: CoreConfig,
    device: LocalDevice,
    send_email: bool,
    **kwargs: Any,
) -> None:
    """
    Create a new invitation to start a shared recovery procedure
    """
    with cli_exception_handler(config.debug):
        trio_run(_invite_shared_recovery, config, device, send_email)

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any

import click

from parsec._parsec import (
    DateTime,
    DeviceLabel,
    HumanHandle,
    SecretKey,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryOthersListRepNotAllowed,
    ShamirRecoveryOthersListRepOk,
    ShamirRecoveryOthersListRepUnknownStatus,
    ShamirRecoverySecret,
    ShamirRecoverySelfInfoRepOk,
    ShamirRecoverySelfInfoRepUnknownStatus,
    ShamirRecoverySetup,
    ShamirRecoverySetupRepAlreadySet,
    ShamirRecoverySetupRepInvalidCertification,
    ShamirRecoverySetupRepInvalidData,
    ShamirRecoverySetupRepOk,
    ShamirRecoverySetupRepUnknownStatus,
    ShamirRecoveryShareCertificate,
    ShamirRecoveryShareData,
    ShamirRevealToken,
    UserID,
    shamir_make_shares,
)
from parsec.cli_utils import async_confirm, cli_exception_handler
from parsec.core import CoreConfig
from parsec.core.backend_connection import backend_authenticated_cmds_factory
from parsec.core.cli.utils import (
    cli_command_base_options,
    core_config_and_device_options,
)
from parsec.core.logged_core import RemoteDevicesManager, logged_core_factory
from parsec.core.recovery import generate_recovery_device
from parsec.core.types import LocalDevice
from parsec.utils import trio_run


async def _create_shared_recovery_device(
    config: CoreConfig, device: LocalDevice, threshold: int
) -> None:
    # Connect to the backend
    async with logged_core_factory(config, device) as core:
        # List all adminisrators and fetch certificates
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

        # Encrypt a recovery device
        secret_key = SecretKey.generate()
        reveal_token = ShamirRevealToken.new()
        recovery_device = await generate_recovery_device(device)
        secret = ShamirRecoverySecret(secret_key, reveal_token)
        ciphered_data = secret_key.encrypt(recovery_device.dump())

        # Create the shares
        now = DateTime.now()
        nb_shares = len(certificates)
        shares = shamir_make_shares(threshold, secret.dump(), nb_shares)

        # Create the share certificates
        share_certificates: list[ShamirRecoveryShareCertificate] = []
        for certificate, share in zip(certificates, shares):
            share_data = ShamirRecoveryShareData([share])
            share_data_encrypted = share_data.dump_sign_and_encrypt_for(
                device.signing_key, certificate.public_key
            )
            share_certificate = ShamirRecoveryShareCertificate(
                device.device_id, now, certificate.user_id, share_data_encrypted
            )
            share_certificates.append(share_certificate)

        # Create the brief certificate
        brief_certificate = ShamirRecoveryBriefCertificate(
            device.device_id,
            now,
            threshold,
            {certificate.user_id: 1 for certificate in certificates},
        )

        # Submit shamir recovery setup
        setup = ShamirRecoverySetup(
            ciphered_data,
            reveal_token,
            brief_certificate.dump_and_sign(device.signing_key),
            [
                share_certificate.dump_and_sign(device.signing_key)
                for share_certificate in share_certificates
            ],
        )

        rep = await core._backend_conn.cmds.shamir_recovery_setup(setup)
        if isinstance(rep, ShamirRecoverySetupRepInvalidCertification):
            raise click.ClickException("Invalid certification")
        if isinstance(rep, ShamirRecoverySetupRepInvalidData):
            raise click.ClickException("Invalid data")
        if isinstance(rep, ShamirRecoverySetupRepAlreadySet):
            raise click.ClickException("Already set")
        if isinstance(rep, ShamirRecoverySetupRepUnknownStatus):
            raise click.ClickException(f"Unknown status: {rep.status}")
        assert isinstance(rep, ShamirRecoverySetupRepOk)
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
    our_device: LocalDevice,
    remote_devices_manager: RemoteDevicesManager,
    human_handle: HumanHandle,
    device_label: DeviceLabel,
    brief_certificate: ShamirRecoveryBriefCertificate,
) -> None:
    styled_human_handle = click.style(human_handle.str, fg="yellow")
    styled_device_label = click.style(device_label.str, fg="yellow")
    styled_timestamp = click.style(brief_certificate.timestamp.to_rfc3339(), fg="yellow")
    styled_threshold = click.style(brief_certificate.threshold)
    click.echo(f"━━━━━━  Shared recovery device for {styled_human_handle}  ━━━━━━")
    click.echo(f"Configured from device {styled_device_label} on {styled_timestamp}")
    click.echo(f"At least {styled_threshold} shares are required from the following users:")
    for user_id, count in brief_certificate.per_recipient_shares.items():
        user_certificate, _ = await remote_devices_manager.get_user(user_id)
        assert user_certificate.human_handle is not None
        styled_user_human_handle = click.style(user_certificate.human_handle.str, fg="yellow")
        styled_share = click.style(count, fg="yellow")
        styled_share = f"{styled_share} share" if count == 1 else f"{styled_share} shares"
        if user_id == our_device.user_id:
            click.echo(f"- {styled_user_human_handle} ({styled_share}) ← This is you")
        else:
            click.echo(f"- {styled_user_human_handle} ({styled_share})")


async def _shared_recovery_device_info(config: CoreConfig, device: LocalDevice) -> bool:
    assert device.human_handle is not None
    styled_human_handle = click.style(device.human_handle.str, fg="yellow")

    # Connect to the server
    async with backend_authenticated_cmds_factory(
        addr=device.organization_addr,
        device_id=device.device_id,
        signing_key=device.signing_key,
        keepalive=config.backend_connection_keepalive,
    ) as cmds:
        # Ask the server
        rep = await cmds.shamir_recovery_self_info()
        if isinstance(rep, ShamirRecoverySelfInfoRepUnknownStatus):
            raise click.ClickException(f"Unknown status: {rep.status}")
        assert isinstance(rep, ShamirRecoverySelfInfoRepOk)
        if rep.self_info is None:
            click.echo(f"No shared recovery device configured for {styled_human_handle}")
            return False

        # Load brief certificate
        unsecure_certificate = ShamirRecoveryBriefCertificate.unsecure_load(rep.self_info)
        remote_devices_manager = RemoteDevicesManager(
            cmds, device.root_verify_key, device.time_provider
        )
        author_certificate = await remote_devices_manager.get_device(unsecure_certificate.author)
        brief_certificate = ShamirRecoveryBriefCertificate.verify_and_load(
            rep.self_info, author_certificate.verify_key, unsecure_certificate.author
        )

        # Indicate a shared recovery device has been found
        assert author_certificate.device_label is not None
        await _show_brief_certificate(
            device,
            remote_devices_manager,
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
    async with backend_authenticated_cmds_factory(
        addr=device.organization_addr,
        device_id=device.device_id,
        signing_key=device.signing_key,
        keepalive=config.backend_connection_keepalive,
    ) as cmds:
        rep = await cmds.shamir_recovery_setup(None)

    # Unpack reply
    if isinstance(rep, ShamirRecoverySetupRepInvalidCertification):
        raise click.ClickException("Invalid certification")
    if isinstance(rep, ShamirRecoverySetupRepInvalidData):
        raise click.ClickException("Invalid data")
    if isinstance(rep, ShamirRecoverySetupRepAlreadySet):
        raise click.ClickException("Already set")
    if isinstance(rep, ShamirRecoverySetupRepUnknownStatus):
        raise click.ClickException(f"Unknown status: {rep.status}")
    assert isinstance(rep, ShamirRecoverySetupRepOk)
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
    async with backend_authenticated_cmds_factory(
        addr=device.organization_addr,
        device_id=device.device_id,
        signing_key=device.signing_key,
        keepalive=config.backend_connection_keepalive,
    ) as cmds:
        rep = await cmds.shamir_recovery_others_list()
        if isinstance(rep, ShamirRecoveryOthersListRepNotAllowed):
            raise click.ClickException("Not allowed")
        if isinstance(rep, ShamirRecoveryOthersListRepUnknownStatus):
            raise click.ClickException(f"Unknown status: {rep.status}")
        assert isinstance(rep, ShamirRecoveryOthersListRepOk)

        # Load share certificates
        share_certificates: dict[UserID, ShamirRecoveryShareData] = {}
        for raw in rep.share_certificates:
            unsecure_share_certificate = ShamirRecoveryShareCertificate.unsecure_load(raw)
            remote_devices_manager = RemoteDevicesManager(
                cmds, device.root_verify_key, device.time_provider
            )
            author_certificate = await remote_devices_manager.get_device(
                unsecure_share_certificate.author
            )
            share_certificate = ShamirRecoveryShareCertificate.verify_and_load(
                raw, author_certificate.verify_key, unsecure_share_certificate.author
            )
            share_data = ShamirRecoveryShareData.decrypt_verify_and_load_for(
                share_certificate.ciphered_share, device.private_key, author_certificate.verify_key
            )
            share_certificates[author_certificate.device_id.user_id] = share_data

        # Load brief certificates
        for i, raw in enumerate(rep.brief_certificates):
            unsecure_brief_certificate = ShamirRecoveryBriefCertificate.unsecure_load(raw)
            remote_devices_manager = RemoteDevicesManager(
                cmds, device.root_verify_key, device.time_provider
            )
            author_certificate = await remote_devices_manager.get_device(
                unsecure_brief_certificate.author
            )
            brief_certificate = ShamirRecoveryBriefCertificate.verify_and_load(
                raw, author_certificate.verify_key, unsecure_brief_certificate.author
            )
            user_certificate, _ = await remote_devices_manager.get_user(
                author_certificate.device_id.user_id
            )
            maybe_share_data = share_certificates.get(author_certificate.device_id.user_id)
            share_number = 0 if maybe_share_data is None else len(maybe_share_data.weighted_share)

            # Display brief certificate
            if i != 0:
                click.echo()
            assert author_certificate.device_label is not None
            assert user_certificate.human_handle is not None
            await _show_brief_certificate(
                device,
                remote_devices_manager,
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

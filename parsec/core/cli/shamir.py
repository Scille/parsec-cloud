# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any

import click

from parsec._parsec import (
    DateTime,
    SecretKey,
    ShamirRecoveryBriefCertificate,
    ShamirRecoverySecret,
    ShamirRecoverySetup,
    ShamirRecoverySetupRepAlreadySet,
    ShamirRecoverySetupRepInvalidCertification,
    ShamirRecoverySetupRepInvalidData,
    ShamirRecoverySetupRepOk,
    ShamirRecoverySetupRepUnknownStatus,
    ShamirRecoveryShareCertificate,
    ShamirRecoveryShareData,
    ShamirRevealToken,
    shamir_make_shares,
)
from parsec.cli_utils import async_confirm, cli_exception_handler
from parsec.core import CoreConfig
from parsec.core.cli.utils import (
    cli_command_base_options,
    core_config_and_device_options,
)
from parsec.core.logged_core import logged_core_factory
from parsec.core.recovery import generate_recovery_device
from parsec.core.types import LocalDevice
from parsec.utils import trio_run


async def _share_recovery_device(config: CoreConfig, device: LocalDevice, threshold: int) -> None:
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
            if user.profile == user.profile.ADMIN and user.human_handle is not None
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
        click.echo("Recovery device successfully shared.")


@click.command(short_help="share recovery device")
@click.option(
    "--threshold",
    "-t",
    required=True,
    help="Number of shared required to perform the recovery",
    type=click.IntRange(min=1),
)
@core_config_and_device_options
@cli_command_base_options
def share_recovery_device(
    config: CoreConfig, device: LocalDevice, threshold: int, **kwargs: Any
) -> None:
    """
    Share a new recovery device using the shamir algorithm.
    """
    with cli_exception_handler(config.debug):
        trio_run(_share_recovery_device, config, device, threshold)

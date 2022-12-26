# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import platform
from functools import partial
from typing import Any, Tuple, Union

import click

from parsec._parsec import (
    InvitationDeletedReason,
    InvitationEmailSentStatus,
    InvitationStatus,
    InvitationType,
    InviteDeleteRepOk,
    InviteListRepOk,
    InviteNewRepOk,
)
from parsec.api.protocol import DeviceLabel, HumanHandle, InvitationToken, UserProfile
from parsec.cli_utils import async_prompt, cli_exception_handler, spinner
from parsec.core.backend_connection import (
    BackendConnectionRefused,
    backend_authenticated_cmds_factory,
    backend_invited_cmds_factory,
)
from parsec.core.cli.bootstrap_organization import SaveDeviceWithSelectedAuth
from parsec.core.cli.utils import (
    cli_command_base_options,
    core_config_and_device_options,
    core_config_options,
    save_device_options,
)
from parsec.core.config import CoreConfig
from parsec.core.fs.storage.user_storage import user_storage_non_speculative_init
from parsec.core.invite import (
    DeviceClaimInitialCtx,
    DeviceGreetInitialCtx,
    InviteError,
    UserClaimInitialCtx,
    UserGreetInitialCtx,
    claimer_retrieve_info,
)
from parsec.core.types import BackendInvitationAddr, LocalDevice
from parsec.utils import trio_run


async def _invite_device(config: CoreConfig, device: LocalDevice) -> None:
    async with spinner("Creating device invitation"):
        async with backend_authenticated_cmds_factory(
            addr=device.organization_addr,
            device_id=device.device_id,
            signing_key=device.signing_key,
            keepalive=config.backend_connection_keepalive,
        ) as cmds:
            rep = await cmds.invite_new(type=InvitationType.DEVICE)
            if not isinstance(rep, InviteNewRepOk):
                raise RuntimeError(f"Backend refused to create device invitation: {rep}")
            try:
                if rep.email_sent and rep.email_sent != InvitationEmailSentStatus.SUCCESS:
                    click.secho("Email could not be sent", fg="red")
            except AttributeError:
                pass

    action_addr = BackendInvitationAddr.build(
        backend_addr=device.organization_addr.get_backend_addr(),
        organization_id=device.organization_id,
        invitation_type=InvitationType.DEVICE,
        token=rep.token,
    )
    action_addr_display = click.style(action_addr.to_url(), fg="yellow")
    click.echo(f"url: {action_addr_display}")


@click.command(short_help="create device invitation")
@core_config_and_device_options
@cli_command_base_options
def invite_device(
    config: CoreConfig,
    device: LocalDevice,
    **kwargs: Any,
) -> None:
    """
    Create new device invitation
    """
    with cli_exception_handler(config.debug):
        trio_run(_invite_device, config, device)


async def _invite_user(
    config: CoreConfig, device: LocalDevice, email: str, send_email: bool
) -> None:
    async with spinner("Creating user invitation"):
        async with backend_authenticated_cmds_factory(
            addr=device.organization_addr,
            device_id=device.device_id,
            signing_key=device.signing_key,
            keepalive=config.backend_connection_keepalive,
        ) as cmds:
            rep = await cmds.invite_new(
                type=InvitationType.USER, claimer_email=email, send_email=send_email
            )
            if not isinstance(rep, InviteNewRepOk):
                raise RuntimeError(f"Backend refused to create user invitation: {rep}")

            try:
                if rep.email_sent and rep.email_sent != InvitationEmailSentStatus.SUCCESS:
                    click.secho("Email could not be sent", fg="red")
            except AttributeError:
                pass

    action_addr = BackendInvitationAddr.build(
        backend_addr=device.organization_addr.get_backend_addr(),
        organization_id=device.organization_id,
        invitation_type=InvitationType.USER,
        token=rep.token,
    )
    action_addr_display = click.style(action_addr.to_url(), fg="yellow")
    click.echo(f"url: {action_addr_display}")


@click.command(short_help="create user invitation")
@click.argument("email")
@click.option("--send-email", is_flag=True)
@core_config_and_device_options
@cli_command_base_options
def invite_user(
    config: CoreConfig,
    device: LocalDevice,
    email: str,
    send_email: bool,
    **kwargs: Any,
) -> None:
    """
    Create new user invitation
    """
    with cli_exception_handler(config.debug):
        trio_run(_invite_user, config, device, email, send_email)


async def ask_info_new_user(
    default_device_label: DeviceLabel | None,
    default_user_label: str | None,
    default_user_email: str | None,
) -> Tuple[DeviceLabel, HumanHandle, UserProfile]:
    while True:
        granted_label = await async_prompt("New user label", default=default_user_label)
        granted_email = await async_prompt("New user email", default=default_user_email)
        try:
            granted_human_handle = HumanHandle(email=granted_email, label=granted_label)
            break
        except ValueError:
            click.echo("Invalid user label and/or email")
            continue

    while True:
        try:
            granted_device_label = DeviceLabel(
                await async_prompt(
                    "New user device label",
                    default=default_device_label.str if default_device_label is not None else None,
                )
            )
            break
        except ValueError:
            click.echo("Invalid value")
            continue

    choices = UserProfile.VALUES
    for i, choice in enumerate(choices):
        display_choice = click.style(choice.str, fg="yellow")
        click.echo(f" {i} - {display_choice}")
    choice_index = await async_prompt(
        "New user profile", default="1", type=click.Choice([str(i) for i, _ in enumerate(choices)])
    )
    granted_profile = choices[int(choice_index)]
    return granted_device_label, granted_human_handle, granted_profile


async def _do_greet_user(device: LocalDevice, initial_ctx: UserGreetInitialCtx) -> bool:
    async with spinner("Waiting for claimer"):
        in_progress1_ctx = await initial_ctx.do_wait_peer()

    display_greeter_sas = click.style(str(in_progress1_ctx.greeter_sas), fg="yellow")
    click.echo(f"Code to provide to claimer: {display_greeter_sas}")
    async with spinner("Waiting for claimer"):
        in_progress2_ctx = await in_progress1_ctx.do_wait_peer_trust()

    choices = in_progress2_ctx.generate_claimer_sas_choices(size=3)
    for i, choice in enumerate(choices):
        display_choice = click.style(choice, fg="yellow")
        click.echo(f" {i} - {display_choice}")
    code = await async_prompt(
        f"Select code provided by claimer",
        type=click.Choice([str(i) for i, _ in enumerate(choices)]),
    )
    if choices[int(code)] != in_progress2_ctx.claimer_sas:
        click.secho("Wrong code provided", fg="red")
        return False

    async with spinner("Waiting for claimer"):
        in_progress3_ctx = await in_progress2_ctx.do_signify_trust()
        in_progress4_ctx = await in_progress3_ctx.do_get_claim_requests()

    granted_device_label, granted_human_handle, granted_profile = await ask_info_new_user(
        default_device_label=in_progress4_ctx.requested_device_label,
        default_user_label=in_progress4_ctx.requested_human_handle.label
        if in_progress4_ctx.requested_human_handle is not None
        else None,
        default_user_email=in_progress4_ctx.requested_human_handle.email
        if in_progress4_ctx.requested_human_handle is not None
        else None,
    )

    async with spinner("Creating the user in the backend"):
        await in_progress4_ctx.do_create_new_user(
            author=device,
            device_label=granted_device_label,
            human_handle=granted_human_handle,
            profile=granted_profile,
        )

    return True


async def _do_greet_device(device: LocalDevice, initial_ctx: DeviceGreetInitialCtx) -> bool:
    async with spinner("Waiting for claimer"):
        in_progress_ctx = await initial_ctx.do_wait_peer()

    display_greeter_sas = click.style(str(in_progress_ctx.greeter_sas), fg="yellow")
    click.echo(f"Code to provide to claimer: {display_greeter_sas}")
    async with spinner("Waiting for claimer"):
        in_progress2_ctx = await in_progress_ctx.do_wait_peer_trust()

    choices = in_progress2_ctx.generate_claimer_sas_choices(size=3)
    for i, choice in enumerate(choices):
        display_choice = click.style(choice, fg="yellow")
        click.echo(f" {i} - {display_choice}")
    code = await async_prompt(
        f"Select code provided by claimer", type=click.Choice([str(x) for x in range(len(choices))])
    )
    if choices[int(code)] != in_progress2_ctx.claimer_sas:
        click.secho("Wrong code provided", fg="red")
        return False

    async with spinner("Waiting for claimer"):
        in_progress3_ctx = await in_progress2_ctx.do_signify_trust()
        in_progress4_ctx = await in_progress3_ctx.do_get_claim_requests()

    granted_device_label = await async_prompt(
        "New device label",
        default=in_progress4_ctx.requested_device_label.str
        if in_progress4_ctx.requested_device_label is not None
        else None,
    )
    async with spinner("Creating the device in the backend"):
        await in_progress4_ctx.do_create_new_device(
            author=device, device_label=DeviceLabel(granted_device_label)
        )

    return True


async def _greet_invitation(
    config: CoreConfig, device: LocalDevice, token: InvitationToken
) -> None:
    async with backend_authenticated_cmds_factory(
        addr=device.organization_addr,
        device_id=device.device_id,
        signing_key=device.signing_key,
        keepalive=config.backend_connection_keepalive,
    ) as cmds:
        async with spinner("Retrieving invitation info"):
            rep = await cmds.invite_list()
            if not isinstance(rep, InviteListRepOk):
                raise RuntimeError(f"Backend error: {rep}")
        for invitation in rep.invitations:
            if invitation.token == token:
                break
        else:
            raise RuntimeError(f"Invitation not found")

        if invitation.type == InvitationType.USER:
            user_initial_ctx = UserGreetInitialCtx(cmds=cmds, token=token)
            do_greet = partial(_do_greet_user, device, user_initial_ctx)
        else:
            assert invitation.type == InvitationType.DEVICE
            device_initial_ctx = DeviceGreetInitialCtx(cmds=cmds, token=token)
            do_greet = partial(_do_greet_device, device, device_initial_ctx)

        while True:
            try:
                greet_done = await do_greet()
                if greet_done:
                    break
            except InviteError as exc:
                click.secho(str(exc), fg="red")
            click.secho("Restarting the invitation process", fg="red")


def _parse_invitation_token_or_url(raw: str) -> Union[BackendInvitationAddr, InvitationToken]:
    try:
        return InvitationToken.from_hex(raw)
    except ValueError:
        try:
            return BackendInvitationAddr.from_url(raw)
        except ValueError:
            raise ValueError("Must be an invitation URL or Token")


def extract_token_from_token_or_url(
    token_or_url: Union[BackendInvitationAddr, InvitationToken], device: LocalDevice
) -> InvitationToken:
    if isinstance(token_or_url, BackendInvitationAddr):
        if device.organization_addr != token_or_url.generate_organization_addr(
            device.root_verify_key
        ):
            raise ValueError("Invitation URL comes from a different organization")
        return token_or_url.token
    else:
        assert isinstance(token_or_url, InvitationToken)
        return token_or_url


@click.command(short_help="greet invitation")
@click.argument("token_or_url", type=_parse_invitation_token_or_url)
@core_config_and_device_options
@cli_command_base_options
def greet_invitation(
    config: CoreConfig,
    device: LocalDevice,
    token_or_url: Union[BackendInvitationAddr, InvitationToken],
    **kwargs: Any,
) -> None:
    """
    Greet a new device or user into the organization
    """
    with cli_exception_handler(config.debug):
        token = extract_token_from_token_or_url(token_or_url, device)
        # Disable task monitoring given user prompt will block the coroutine
        trio_run(_greet_invitation, config, device, token)


async def _do_claim_user(initial_ctx: UserClaimInitialCtx) -> LocalDevice | None:
    async with spinner("Initializing connection with greeter for claiming user"):
        in_progress_ctx = await initial_ctx.do_wait_peer()

    choices = in_progress_ctx.generate_greeter_sas_choices(size=3)
    for i, choice in enumerate(choices):
        display_choice = click.style(choice, fg="yellow")
        click.echo(f" {i} - {display_choice}")
    code = await async_prompt(
        f"Select code provided by greeter", type=click.Choice([str(x) for x in range(len(choices))])
    )
    if choices[int(code)] != in_progress_ctx.greeter_sas:
        click.secho("Wrong code provided", fg="red")
        return None

    in_progress2_ctx = await in_progress_ctx.do_signify_trust()
    display_claimer_sas = click.style(str(in_progress2_ctx.claimer_sas), fg="yellow")
    click.echo(f"Code to provide to greeter: {display_claimer_sas}")
    async with spinner("Waiting for greeter"):
        in_progress3_ctx = await in_progress2_ctx.do_wait_peer_trust()

    requested_label = await async_prompt("User fullname")
    requested_email = initial_ctx.claimer_email
    requested_device_label = await async_prompt("Device label", default=platform.node())
    async with spinner("Waiting for greeter (finalizing)"):
        new_device = await in_progress3_ctx.do_claim_user(
            requested_device_label=DeviceLabel(requested_device_label),
            requested_human_handle=HumanHandle(email=requested_email, label=requested_label),
        )

    return new_device


async def _do_claim_device(initial_ctx: DeviceClaimInitialCtx) -> LocalDevice | None:
    async with spinner("Initializing connection with greeter for claiming device"):
        in_progress_ctx = await initial_ctx.do_wait_peer()

    choices = in_progress_ctx.generate_greeter_sas_choices(size=3)
    for i, choice in enumerate(choices):
        display_choice = click.style(choice, fg="yellow")
        click.echo(f" {i} - {display_choice}")
    code = await async_prompt(
        f"Select code provided by greeter", type=click.Choice([str(x) for x in range(len(choices))])
    )
    if choices[int(code)] != in_progress_ctx.greeter_sas:
        click.secho("Wrong code provided", fg="red")
        return None

    in_progress2_ctx = await in_progress_ctx.do_signify_trust()
    display_claimer_sas = click.style(str(in_progress2_ctx.claimer_sas), fg="yellow")
    click.echo(f"Code to provide to greeter: {display_claimer_sas}")
    async with spinner("Waiting for greeter"):
        in_progress3_ctx = await in_progress2_ctx.do_wait_peer_trust()

    requested_device_label = await async_prompt("Device label", default=platform.node())
    async with spinner("Waiting for greeter (finalizing)"):
        new_device = await in_progress3_ctx.do_claim_device(
            requested_device_label=DeviceLabel(requested_device_label)
        )

    return new_device


async def _claim_invitation(
    config: CoreConfig,
    addr: BackendInvitationAddr,
    save_device_with_selected_auth: SaveDeviceWithSelectedAuth,
) -> None:
    async with backend_invited_cmds_factory(
        addr=addr, keepalive=config.backend_connection_keepalive
    ) as cmds:
        try:
            async with spinner("Retrieving invitation info"):
                initial_ctx = await claimer_retrieve_info(cmds)
        except BackendConnectionRefused:
            raise RuntimeError("Invitation not found")

        if initial_ctx.greeter_human_handle:
            display_greeter = click.style(str(initial_ctx.greeter_human_handle), fg="yellow")
        else:
            display_greeter = click.style(initial_ctx.greeter_user_id, fg="yellow")
        click.echo(f"Invitation greeter: {display_greeter}")
        while True:
            try:
                if isinstance(initial_ctx, DeviceClaimInitialCtx):
                    new_device = await _do_claim_device(initial_ctx)
                else:
                    assert isinstance(initial_ctx, UserClaimInitialCtx)
                    new_device = await _do_claim_user(initial_ctx)
                if new_device:
                    break
            except InviteError as exc:
                click.secho(str(exc), fg="red")
            click.secho("Restarting the invitation process", fg="red")

        # Claiming a user means we are it first device, hence we know there
        # is no existing user manifest (hence our placeholder is non-speculative)
        if addr.invitation_type == InvitationType.USER:
            await user_storage_non_speculative_init(
                data_base_dir=config.data_base_dir, device=new_device
            )
        await save_device_with_selected_auth(config_dir=config.config_dir, device=new_device)


@click.command(short_help="claim invitation")
@click.argument("addr", type=BackendInvitationAddr.from_url)
@save_device_options
@core_config_options
@cli_command_base_options
def claim_invitation(
    config: CoreConfig,
    addr: BackendInvitationAddr,
    save_device_with_selected_auth: SaveDeviceWithSelectedAuth,
    **kwargs: Any,
) -> None:
    """
    Claim a device or user from a invitation
    """
    with cli_exception_handler(config.debug):
        # Disable task monitoring given user prompt will block the coroutine
        trio_run(_claim_invitation, config, addr, save_device_with_selected_auth)


async def _list_invitations(config: CoreConfig, device: LocalDevice) -> None:
    async with backend_authenticated_cmds_factory(
        addr=device.organization_addr,
        device_id=device.device_id,
        signing_key=device.signing_key,
        keepalive=config.backend_connection_keepalive,
    ) as cmds:
        rep = await cmds.invite_list()
        if not isinstance(rep, InviteListRepOk):
            raise RuntimeError(f"Backend error while listing invitations: {rep}")
        display_statuses = {
            InvitationStatus.READY: click.style("ready", fg="green"),
            InvitationStatus.IDLE: click.style("idle", fg="yellow"),
            InvitationStatus.DELETED: click.style("deleted", fg="red"),
        }
        for invitation in rep.invitations:
            display_status = display_statuses[invitation.status]
            display_token = invitation.token.hex
            if invitation.type == InvitationType.USER:
                display_type = f"user (email={invitation.claimer_email})"
            else:  # Device
                display_type = f"device"
            click.echo(f"{display_token}\t{display_status}\t{display_type}")
        if not rep.invitations:
            click.echo("No invitations.")


@click.command(short_help="list invitations")
@core_config_and_device_options
@cli_command_base_options
def list_invitations(config: CoreConfig, device: LocalDevice, **kwargs: Any) -> None:
    """
    List invitations
    """
    with cli_exception_handler(config.debug):
        trio_run(_list_invitations, config, device)


async def _cancel_invitation(
    config: CoreConfig, device: LocalDevice, token: InvitationToken
) -> None:
    async with backend_authenticated_cmds_factory(
        addr=device.organization_addr,
        device_id=device.device_id,
        signing_key=device.signing_key,
        keepalive=config.backend_connection_keepalive,
    ) as cmds:
        rep = await cmds.invite_delete(token=token, reason=InvitationDeletedReason.CANCELLED)
        if not isinstance(rep, InviteDeleteRepOk):
            raise RuntimeError(f"Backend error while cancelling invitation: {rep}")
        click.echo("Invitation deleted.")


@click.command(short_help="cancel invitations")
@click.argument("token_or_url", type=_parse_invitation_token_or_url)
@core_config_and_device_options
@cli_command_base_options
def cancel_invitation(
    config: CoreConfig,
    device: LocalDevice,
    token_or_url: Union[BackendInvitationAddr, InvitationToken],
    **kwargs: Any,
) -> None:
    """
    Cancel invitation
    """
    with cli_exception_handler(config.debug):
        token = extract_token_from_token_or_url(token_or_url, device)
        trio_run(_cancel_invitation, config, device, token)

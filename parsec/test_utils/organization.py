# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


import os
import trio
from typing import Tuple
from pathlib import Path
from uuid import uuid4
import random

from parsec.logging import configure_logging
from parsec.core import logged_core_factory
from parsec.api.data import UserProfile
from parsec.api.protocol import OrganizationID, HumanHandle, InvitationType
from parsec.core.types import (
    WorkspaceRole,
    BackendAddr,
    BackendOrganizationBootstrapAddr,
    BackendInvitationAddr,
    LocalDevice,
)
from parsec.core.config import load_config
from parsec.core.backend_connection import (
    apiv1_backend_administration_cmds_factory,
    apiv1_backend_anonymous_cmds_factory,
    backend_authenticated_cmds_factory,
    backend_invited_cmds_factory,
)
from parsec.core.local_device import save_device_with_password
from parsec.core.invite import (
    bootstrap_organization,
    DeviceGreetInitialCtx,
    UserGreetInitialCtx,
    claimer_retrieve_info,
)


async def initialize_test_organization(
    config_dir: Path,
    backend_address: BackendAddr,
    password: str,
    administration_token: str,
    force: bool,
    additional_users_number: int,
) -> Tuple[LocalDevice, LocalDevice, LocalDevice]:
    configure_logging("WARNING")
    organization_id = OrganizationID("Org")

    # Create organization

    async with apiv1_backend_administration_cmds_factory(
        backend_address, administration_token
    ) as administration_cmds:

        rep = await administration_cmds.organization_create(organization_id)
        assert rep["status"] == "ok"
        bootstrap_token = rep["bootstrap_token"]

        organization_bootstrap_addr = BackendOrganizationBootstrapAddr.build(
            backend_address, organization_id, bootstrap_token
        )
    # Bootstrap organization and Alice user and create device "laptop" for Alice

    async with apiv1_backend_anonymous_cmds_factory(organization_bootstrap_addr) as anonymous_cmds:
        alice_device = await bootstrap_organization(
            cmds=anonymous_cmds,
            human_handle=HumanHandle(label="Alice", email="alice@example.com"),
            device_label="laptop",
        )
        save_device_with_password(config_dir, alice_device, password, force=force)

    config = load_config(config_dir, debug="DEBUG" in os.environ)
    # Create context manager, alice_core will be needed for the rest of the script
    async with logged_core_factory(config, alice_device) as alice_core:
        async with backend_authenticated_cmds_factory(
            addr=alice_device.organization_addr,
            device_id=alice_device.device_id,
            signing_key=alice_device.signing_key,
        ) as alice_cmds:

            # Create new device "pc" for Alice
            other_alice_device = await _register_new_device(
                cmds=alice_cmds,
                password=password,
                config_dir=config_dir,
                device=alice_device,
                requested_device_label="pc",
                force=force,
            )
            # Invite Bob in organization
            bob_device = await _invite_user_to_organization(
                cmds=alice_cmds,
                host_device=alice_device,
                config_dir=config_dir,
                password=password,
                claimer_email="bob@example.com",
                requested_user_label="Bob",
                requested_device_label="laptop",
                force=force,
                profile=UserProfile.STANDARD,
            )
            # Create Alice workspace
            alice_ws_id = await alice_core.user_fs.workspace_create("alice_workspace")
            # Create context manager
            async with logged_core_factory(config, bob_device) as bob_core:
                # Create Bob workspace
                bob_ws_id = await bob_core.user_fs.workspace_create("bob_workspace")
                # Bob share workspace with Alice
                await bob_core.user_fs.workspace_share(
                    bob_ws_id, alice_device.user_id, WorkspaceRole.MANAGER
                )
                # Alice share workspace with Bob
                await alice_core.user_fs.workspace_share(
                    alice_ws_id, bob_device.user_id, WorkspaceRole.MANAGER
                )
                # Add additional random users
                if additional_users_number > 0:
                    await _add_random_users(
                        alice_cmds,
                        alice_device,
                        config_dir,
                        password,
                        force,
                        alice_core,
                        bob_core,
                        alice_ws_id,
                        bob_ws_id,
                        additional_users_number,
                    )

    # Synchronize every device
    for device in (alice_device, other_alice_device, bob_device):
        async with logged_core_factory(config, device) as core:
            await core.user_fs.process_last_messages()
            await core.user_fs.sync()
    return (alice_device, other_alice_device, bob_device)


async def _add_random_users(
    host_cmds,
    host_device,
    config_dir,
    password,
    force,
    first_core,
    second_core,
    first_ws_id,
    second_ws_id,
    additional_users_number: int,
):
    """ Add random number of users with random role, and share workspaces with them.
        1 out of 5 users will be revoked from organization.
    """
    for _ in range(additional_users_number):
        name = "test_" + str(uuid4())[:9]
        user_profile = random.choice(list(UserProfile))
        realm_role = random.choice(list(WorkspaceRole))
        if user_profile == UserProfile.OUTSIDER and (
            realm_role == WorkspaceRole.OWNER or realm_role == WorkspaceRole.MANAGER
        ):
            realm_role = WorkspaceRole.READER
        # Workspace_choice : 0 = add user to first_ws, 1 = add to second_ws, 2 = add in both workspace, other = nothing
        workspace_choice = random.randint(0, 3)
        # invite user to organization
        user_device = await _invite_user_to_organization(
            cmds=host_cmds,
            host_device=host_device,
            config_dir=config_dir,
            password=password,
            claimer_email=f"{name}@gmail.com",
            requested_user_label=name,
            requested_device_label="no_device",
            force=force,
            profile=user_profile,
        )
        # Share workspace with new user
        if workspace_choice == 0 or workspace_choice == 2:
            await first_core.user_fs.workspace_share(first_ws_id, user_device.user_id, realm_role)
        if workspace_choice == 1 or workspace_choice == 2:
            await second_core.user_fs.workspace_share(second_ws_id, user_device.user_id, realm_role)
        # One chance out of 5 to be revoked from organization
        if not random.randint(0, 4):
            await first_core.revoke_user(user_device.user_id)


async def _init_ctx_create(cmds, token):
    initial_ctx = UserGreetInitialCtx(cmds=cmds, token=token)
    in_progress_ctx = await initial_ctx.do_wait_peer()
    in_progress_ctx = await in_progress_ctx.do_wait_peer_trust()
    in_progress_ctx = await in_progress_ctx.do_signify_trust()
    in_progress_ctx = await in_progress_ctx.do_get_claim_requests()
    return in_progress_ctx


async def _init_ctx_claim(cmds):
    initial_ctx = await claimer_retrieve_info(cmds=cmds)
    in_progress_ctx = await initial_ctx.do_wait_peer()
    in_progress_ctx = await in_progress_ctx.do_signify_trust()
    in_progress_ctx = await in_progress_ctx.do_wait_peer_trust()
    return in_progress_ctx


async def _invite_user_task(cmds, token, host_device, profile: UserProfile = UserProfile.STANDARD):
    in_progress_ctx = await _init_ctx_create(cmds=cmds, token=token)
    await in_progress_ctx.do_create_new_user(
        author=host_device,
        human_handle=in_progress_ctx.requested_human_handle,
        device_label=in_progress_ctx.requested_device_label,
        profile=profile,
    )


async def _invite_device_task(cmds, device, device_label, token):
    initial_ctx = DeviceGreetInitialCtx(cmds=cmds, token=token)
    in_progress_ctx = await initial_ctx.do_wait_peer()
    in_progress_ctx = await in_progress_ctx.do_wait_peer_trust()
    in_progress_ctx = await in_progress_ctx.do_signify_trust()
    in_progress_ctx = await in_progress_ctx.do_get_claim_requests()
    await in_progress_ctx.do_create_new_device(
        author=device, device_label=in_progress_ctx.requested_device_label
    )


async def _claim_user(cmds, claimer_email, requested_device_label, requested_user_label):
    in_progress_ctx = await _init_ctx_claim(cmds)
    new_device = await in_progress_ctx.do_claim_user(
        requested_human_handle=HumanHandle(label=requested_user_label, email=claimer_email),
        requested_device_label=requested_device_label,
    )
    return new_device


async def _claim_device(cmds, requested_device_label):
    initial_ctx = await claimer_retrieve_info(cmds=cmds)
    in_progress_ctx = await initial_ctx.do_wait_peer()
    in_progress_ctx = await in_progress_ctx.do_signify_trust()
    in_progress_ctx = await in_progress_ctx.do_wait_peer_trust()
    new_device = await in_progress_ctx.do_claim_device(
        requested_device_label=requested_device_label
    )
    return new_device


async def _invite_user_to_organization(
    cmds,
    host_device,
    config_dir,
    password,
    claimer_email,
    requested_user_label,
    requested_device_label,
    force,
    profile,
):
    rep = await cmds.invite_new(type=InvitationType.USER, claimer_email=claimer_email)
    assert rep["status"] == "ok"
    invitation_addr = BackendInvitationAddr.build(
        backend_addr=host_device.organization_addr,
        organization_id=host_device.organization_id,
        invitation_type=InvitationType.USER,
        token=rep["token"],
    )
    async with backend_invited_cmds_factory(addr=invitation_addr) as invited_cmds:

        async with trio.open_service_nursery() as nursery:
            nursery.start_soon(_invite_user_task, cmds, invitation_addr.token, host_device, profile)
            new_device = await _claim_user(
                invited_cmds, claimer_email, requested_device_label, requested_user_label
            )
    if requested_device_label != "no_device":
        save_device_with_password(config_dir, new_device, password, force=force)
    return new_device


async def _register_new_device(cmds, config_dir, device, password, force, requested_device_label):
    rep = await cmds.invite_new(type=InvitationType.DEVICE)
    assert rep["status"] == "ok"
    invitation_addr = BackendInvitationAddr.build(
        backend_addr=device.organization_addr,
        organization_id=device.organization_id,
        invitation_type=InvitationType.DEVICE,
        token=rep["token"],
    )
    async with backend_invited_cmds_factory(addr=invitation_addr) as invited_cmds:

        async with trio.open_service_nursery() as nursery:
            nursery.start_soon(
                _invite_device_task, cmds, device, requested_device_label, invitation_addr.token
            )
            new_device = await _claim_device(invited_cmds, requested_device_label)
    if requested_device_label != "no_device":
        save_device_with_password(config_dir, new_device, password, force=force)
    return new_device

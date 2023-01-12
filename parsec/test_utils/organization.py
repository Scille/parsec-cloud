# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import os
import random
from pathlib import Path
from typing import Tuple, cast
from uuid import uuid4

from parsec._parsec import (
    DateTime,
    DeviceCreateRepOk,
    UserCreateRepOk,
    save_device_with_password_in_config,
)
from parsec.api.data import DeviceCertificate, EntryID, EntryName, UserCertificate
from parsec.api.protocol import (
    DeviceID,
    DeviceLabel,
    DeviceName,
    HumanHandle,
    OrganizationID,
    UserProfile,
)
from parsec.core import logged_core_factory
from parsec.core.backend_connection import (
    BackendAuthenticatedCmds,
    backend_authenticated_cmds_factory,
)
from parsec.core.cli.create_organization import create_organization_req
from parsec.core.config import load_config
from parsec.core.fs.storage.user_storage import user_storage_non_speculative_init
from parsec.core.invite import bootstrap_organization
from parsec.core.local_device import generate_new_device
from parsec.core.logged_core import LoggedCore
from parsec.core.types import (
    BackendAddr,
    BackendOrganizationBootstrapAddr,
    LocalDevice,
    WorkspaceRole,
)
from parsec.crypto import SigningKey


async def initialize_test_organization(
    config_dir: Path,
    backend_address: BackendAddr,
    password: str,
    administration_token: str,
    additional_users_number: int,
    additional_devices_number: int,
) -> Tuple[LocalDevice, LocalDevice, LocalDevice]:
    organization_id = OrganizationID("Org")
    config = load_config(config_dir, debug="DEBUG" in os.environ)

    # Create organization

    bootstrap_token = await create_organization_req(
        organization_id, backend_address, administration_token
    )
    organization_bootstrap_addr = BackendOrganizationBootstrapAddr.build(
        backend_address, organization_id, bootstrap_token
    )

    # Bootstrap organization and Alice user and create device "laptop" for Alice

    alice_device = await bootstrap_organization(
        # Casting is fine here because `bootstrap_organization` needs the stored backend addr and
        # `organization_id`. These attributes are present in `BackendOrganizationFileLinkAddr` too.
        cast(BackendOrganizationBootstrapAddr, organization_bootstrap_addr),
        human_handle=HumanHandle(label="Alice", email="alice@example.com"),
        device_label=DeviceLabel("laptop"),
    )
    await user_storage_non_speculative_init(data_base_dir=config.data_base_dir, device=alice_device)
    save_device_with_password_in_config(
        config_dir=config_dir, device=alice_device, password=password
    )

    # Create context manager, alice_core will be needed for the rest of the script
    async with logged_core_factory(config, alice_device) as alice_core:
        async with backend_authenticated_cmds_factory(
            addr=alice_device.organization_addr,
            device_id=alice_device.device_id,
            signing_key=alice_device.signing_key,
        ) as alice_cmds:

            # Create new device "pc" for Alice
            other_alice_device = await _register_new_device(
                cmds=alice_cmds, author=alice_device, device_label=DeviceLabel("pc")
            )
            save_device_with_password_in_config(
                config_dir=config_dir, device=other_alice_device, password=password
            )
            # Invite Bob in organization
            bob_device = await _register_new_user(
                cmds=alice_cmds,
                author=alice_device,
                device_label=DeviceLabel("laptop"),
                human_handle=HumanHandle(email="bob@example.com", label="Bob"),
                profile=UserProfile.STANDARD,
            )
            await user_storage_non_speculative_init(
                data_base_dir=config.data_base_dir, device=bob_device
            )
            save_device_with_password_in_config(
                config_dir=config_dir, device=bob_device, password=password
            )

            # Invite Toto in organization
            toto_device = await _register_new_user(
                cmds=alice_cmds,
                author=alice_device,
                device_label=DeviceLabel("laptop"),
                human_handle=HumanHandle(email="toto@example.com", label="Toto"),
                profile=UserProfile.OUTSIDER,
            )
            await user_storage_non_speculative_init(
                data_base_dir=config.data_base_dir, device=toto_device
            )
            save_device_with_password_in_config(
                config_dir=config_dir, device=toto_device, password=password
            )
            # Create Alice workspace
            alice_ws_id = await alice_core.user_fs.workspace_create(EntryName("alice_workspace"))
            # Create context manager
            async with logged_core_factory(config, bob_device) as bob_core:
                # Create Bob workspace
                bob_ws_id = await bob_core.user_fs.workspace_create(EntryName("bob_workspace"))
                # Bob share workspace with Alice
                await bob_core.user_fs.workspace_share(
                    bob_ws_id, alice_device.user_id, WorkspaceRole.MANAGER
                )
                # Alice share workspace with Bob
                await alice_core.user_fs.workspace_share(
                    alice_ws_id, bob_device.user_id, WorkspaceRole.MANAGER
                )
                # Add additional random users
                await _add_random_users(
                    cmds=alice_cmds,
                    author=alice_device,
                    alice_core=alice_core,
                    bob_core=bob_core,
                    alice_ws_id=alice_ws_id,
                    bob_ws_id=bob_ws_id,
                    additional_users_number=additional_users_number,
                )
                # Add additional random device for alice
                await _add_random_device(
                    cmds=alice_cmds,
                    device=alice_device,
                    additional_devices_number=additional_devices_number,
                )

    # Synchronize every device
    for device in (alice_device, other_alice_device, bob_device):
        async with logged_core_factory(config, device) as core:
            await core.user_fs.process_last_messages()
            await core.user_fs.sync()
    return (alice_device, other_alice_device, bob_device)


async def _add_random_device(
    cmds: BackendAuthenticatedCmds, device: LocalDevice, additional_devices_number: int
) -> None:
    for _ in range(additional_devices_number):
        device_label = DeviceLabel("device_" + str(uuid4())[:9])
        await _register_new_device(cmds=cmds, author=device, device_label=device_label)


async def _add_random_users(
    cmds: BackendAuthenticatedCmds,
    author: LocalDevice,
    alice_core: LoggedCore,
    bob_core: LoggedCore,
    alice_ws_id: EntryID,
    bob_ws_id: EntryID,
    additional_users_number: int,
) -> None:
    """
    Add random number of users with random role, and share workspaces with them.
    1 out of 5 users will be revoked from organization.
    """
    for _ in range(additional_users_number):
        name = "test_" + str(uuid4())[:9]
        user_profile = random.choice(UserProfile.VALUES)
        if user_profile == UserProfile.OUTSIDER:
            realm_role = random.choice([WorkspaceRole.READER, WorkspaceRole.CONTRIBUTOR])
        else:
            realm_role = random.choice(WorkspaceRole.VALUES)
        # Workspace_choice : 0 = add user to first_ws, 1 = add to second_ws, 2 = add in both workspace, other = nothing
        workspace_choice = random.randint(0, 3)
        # invite user to organization
        user_device = await _register_new_user(
            cmds=cmds,
            author=author,
            device_label=DeviceLabel("desktop"),
            human_handle=HumanHandle(email=f"{name}@gmail.com", label=name),
            profile=user_profile,
        )
        # Share workspace with new user
        if workspace_choice == 0 or workspace_choice == 2:
            await alice_core.user_fs.workspace_share(alice_ws_id, user_device.user_id, realm_role)
        if workspace_choice == 1 or workspace_choice == 2:
            await bob_core.user_fs.workspace_share(bob_ws_id, user_device.user_id, realm_role)
        # One chance out of 5 to be revoked from organization
        if not random.randint(0, 4):
            await alice_core.revoke_user(user_device.user_id)


async def _register_new_user(
    cmds: BackendAuthenticatedCmds,
    author: LocalDevice,
    device_label: DeviceLabel | None,
    human_handle: HumanHandle | None,
    profile: UserProfile,
) -> LocalDevice:
    new_device = generate_new_device(
        organization_addr=cmds.addr,
        device_label=device_label,
        human_handle=human_handle,
        profile=profile,
    )
    now = DateTime.now()

    user_certificate = UserCertificate(
        author=author.device_id,
        timestamp=now,
        user_id=new_device.device_id.user_id,
        human_handle=new_device.human_handle,
        public_key=new_device.public_key,
        profile=new_device.profile,
    )
    redacted_user_certificate = user_certificate.evolve(human_handle=None)

    device_certificate = DeviceCertificate(
        author=author.device_id,
        timestamp=now,
        device_id=new_device.device_id,
        device_label=new_device.device_label,
        verify_key=new_device.verify_key,
    )
    redacted_device_certificate = device_certificate.evolve(device_label=None)

    user_certificate = user_certificate.dump_and_sign(author.signing_key)
    redacted_user_certificate = redacted_user_certificate.dump_and_sign(author.signing_key)
    device_certificate = device_certificate.dump_and_sign(author.signing_key)
    redacted_device_certificate = redacted_device_certificate.dump_and_sign(author.signing_key)

    rep = await cmds.user_create(
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )
    if not isinstance(rep, UserCreateRepOk):
        raise RuntimeError(f"Cannot create user: {rep}")

    return new_device


async def _register_new_device(
    cmds: BackendAuthenticatedCmds,
    author: LocalDevice,
    device_label: DeviceLabel | None,
) -> LocalDevice:

    new_device = LocalDevice(
        organization_addr=author.organization_addr,
        device_id=DeviceID(f"{author.user_id.str}@{DeviceName.new().str}"),
        device_label=device_label,
        human_handle=author.human_handle,
        profile=author.profile,
        private_key=author.private_key,
        signing_key=SigningKey.generate(),
        user_manifest_id=author.user_manifest_id,
        user_manifest_key=author.user_manifest_key,
        local_symkey=author.local_symkey,
    )
    now = DateTime.now()

    device_certificate = DeviceCertificate(
        author=author.device_id,
        timestamp=now,
        device_id=new_device.device_id,
        device_label=new_device.device_label,
        verify_key=new_device.verify_key,
    )
    redacted_device_certificate = device_certificate.evolve(device_label=None)

    device_certificate = device_certificate.dump_and_sign(author.signing_key)
    redacted_device_certificate = redacted_device_certificate.dump_and_sign(author.signing_key)

    rep = await cmds.device_create(
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )

    if not isinstance(rep, DeviceCreateRepOk):
        raise RuntimeError(f"Cannot create device: {rep}")

    return new_device

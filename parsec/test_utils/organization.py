# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


import os
import itertools

import trio
import pendulum

from parsec.crypto import SigningKey
from parsec.logging import configure_logging
from parsec.core import logged_core_factory
from parsec.api.protocol import DeviceID
from parsec.core.types import WorkspaceRole, BackendOrganizationBootstrapAddr
from parsec.core.config import get_default_config_dir, load_config
from parsec.core.backend_connection import (
    backend_administration_cmds_factory,
    backend_anonymous_cmds_factory,
)
from parsec.core.local_device import generate_new_device, save_device_with_password
from parsec.core.invite_claim import (
    generate_invitation_token,
    invite_and_create_device,
    invite_and_create_user,
    claim_device,
    claim_user,
    InviteClaimError,
)
from parsec.api.data import UserCertificateContent, DeviceCertificateContent


async def retry_claim(corofn, *args, retries=10, tick=0.1):
    for i in itertools.count():
        try:
            return await corofn(*args)
        except InviteClaimError:
            if i >= retries:
                raise
            await trio.sleep(tick)


async def initialize_test_organization(
    backend_address,
    organization_id,
    alice_device_id,
    bob_device_id,
    other_device_name,
    alice_workspace,
    bob_workspace,
    password,
    administration_token,
    force,
):

    configure_logging("WARNING")

    config_dir = get_default_config_dir(os.environ)
    alice_slugid = f"{organization_id}:{alice_device_id}"
    bob_slugid = f"{organization_id}:{bob_device_id}"

    # Create organization

    async with backend_administration_cmds_factory(backend_address, administration_token) as cmds:

        rep = await cmds.organization_create(organization_id)
        assert rep["status"] == "ok"
        bootstrap_token = rep["bootstrap_token"]

        organization_bootstrap_addr = BackendOrganizationBootstrapAddr.build(
            backend_address, organization_id, bootstrap_token
        )

    # Bootstrap organization and Alice user

    async with backend_anonymous_cmds_factory(organization_bootstrap_addr) as cmds:
        root_signing_key = SigningKey.generate()
        root_verify_key = root_signing_key.verify_key
        organization_addr = organization_bootstrap_addr.generate_organization_addr(root_verify_key)

        alice_device = generate_new_device(alice_device_id, organization_addr, True)

        save_device_with_password(config_dir, alice_device, password, force=force)

        now = pendulum.now()
        user_certificate = UserCertificateContent(
            author=None,
            timestamp=now,
            user_id=alice_device.user_id,
            public_key=alice_device.public_key,
            is_admin=True,
        ).dump_and_sign(author_signkey=root_signing_key)
        device_certificate = DeviceCertificateContent(
            author=None,
            timestamp=now,
            device_id=alice_device.device_id,
            verify_key=alice_device.verify_key,
        ).dump_and_sign(author_signkey=root_signing_key)

        rep = await cmds.organization_bootstrap(
            organization_bootstrap_addr.organization_id,
            organization_bootstrap_addr.token,
            root_verify_key,
            user_certificate,
            device_certificate,
        )
        assert rep["status"] == "ok"

    # Create a workspace for Alice

    config = load_config(config_dir, debug="DEBUG" in os.environ)
    async with logged_core_factory(config, alice_device) as core:
        alice_ws_id = await core.user_fs.workspace_create(f"{alice_workspace}")
        await core.user_fs.sync()

    # Register a new device for Alice

    token = generate_invitation_token()
    other_alice_device_id = DeviceID(f"{alice_device.user_id}@{other_device_name}")
    other_alice_slugid = f"{organization_id}:{other_alice_device_id}"

    async def invite_task():
        await invite_and_create_device(alice_device, other_device_name, token)

    other_alice_device = None

    async def claim_task():
        nonlocal other_alice_device
        other_alice_device = await retry_claim(
            claim_device, alice_device.organization_addr, other_alice_device_id, token
        )
        save_device_with_password(config_dir, other_alice_device, password, force=force)

    async with trio.open_nursery() as nursery:
        nursery.start_soon(invite_task)
        nursery.start_soon(claim_task)

    # Invite Bob in

    token = generate_invitation_token()
    bob_device = None

    async def invite_task():
        await invite_and_create_user(alice_device, bob_device_id.user_id, token, is_admin=False)

    async def claim_task():
        nonlocal bob_device
        bob_device = await retry_claim(
            claim_user, alice_device.organization_addr, bob_device_id, token
        )
        save_device_with_password(config_dir, bob_device, password, force=force)

    async with trio.open_nursery() as nursery:
        nursery.start_soon(invite_task)
        nursery.start_soon(claim_task)

    # Create bob workspace and share with Alice

    async with logged_core_factory(config, bob_device) as core:
        bob_ws_id = await core.user_fs.workspace_create(f"{bob_workspace}")
        await core.user_fs.workspace_share(
            bob_ws_id, alice_device_id.user_id, WorkspaceRole.MANAGER
        )

    # Share Alice workspace with bob

    async with logged_core_factory(config, alice_device) as core:
        await core.user_fs.workspace_share(
            alice_ws_id, bob_device_id.user_id, WorkspaceRole.MANAGER
        )

    # Synchronize every device
    for device in (alice_device, other_alice_device, bob_device):
        async with logged_core_factory(config, device) as core:
            await core.user_fs.process_last_messages()
            await core.user_fs.sync()

    return alice_slugid, other_alice_slugid, bob_slugid

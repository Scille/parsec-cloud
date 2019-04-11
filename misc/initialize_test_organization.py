#! /usr/bin/env python3

import os
import trio
import itertools

import click
import pendulum

from parsec.types import BackendAddr, OrganizationID, DeviceID, BackendOrganizationBootstrapAddr
from parsec.crypto import SigningKey, build_user_certificate, build_device_certificate
from parsec.core import logged_core_factory
from parsec.logging import configure_logging
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
from parsec.backend.config import DEFAULT_ADMINISTRATION_TOKEN


async def retry_claim(corofn, *args, retries=10, tick=0.1):
    for i in itertools.count():
        try:
            return await corofn(*args)
        except InviteClaimError:
            if i >= retries:
                raise
            await trio.sleep(tick)


@click.command()
@click.option("-B", "--backend-address", default="ws://localhost:6777")
@click.option("-O", "--organization-id", default="corp")
@click.option("-a", "--alice-device-id", default="alice@laptop")
@click.option("-b", "--bob-device-id", default="bob@laptop")
@click.option("-o", "--other-device-name", default="pc")
@click.option("-x", "--alice-workspace", default="alice_workspace")
@click.option("-y", "--bob-workspace", default="bob_workspace")
@click.option("-P", "--password", default="test")
@click.option("--force/--no-force", default=False)
def main(*args, **kwargs):
    """Initialize a test origanization for parsec from a clean environment.

    You might want to create a test environment beforehand with the
    following commands:

        TMP=`mktemp -d`
        export XDG_CACHE_HOME="$TMP/cache"
        export XDG_DATA_HOME="$TMP/share"
        export XDG_CONFIG_HOME="$TMP/config"
        mkdir $XDG_CACHE_HOME $XDG_DATA_HOME $XDG_CONFIG_HOME
        parsec backend run -b MOCKED -P 6888 &

    And use `-B ws://localhost:6888` as a backend adress.

    This scripts create two users, alice and bob who both own two devices,
    laptop and pc. They each have their workspace, respectively
    alice_workspace and bob_workspace, that their sharing with each other.
    """
    trio.run(lambda: amain(*args, **kwargs))


async def amain(
    backend_address="ws://localhost:6777",
    organization_id="vcorp",
    alice_device_id="alice@laptop",
    bob_device_id="bob@laptop",
    other_device_name="pc",
    alice_workspace="alicews",
    bob_workspace="bobws",
    password="test",
    administration_token=DEFAULT_ADMINISTRATION_TOKEN,
    force=False,
):

    configure_logging("WARNING")

    config_dir = get_default_config_dir(os.environ)
    organization_id = OrganizationID(organization_id)
    backend_address = BackendAddr(backend_address)
    alice_device_id = DeviceID(alice_device_id)
    bob_device_id = DeviceID(bob_device_id)
    alice_slugid = f"{organization_id}:{alice_device_id}"
    bob_slugid = f"{organization_id}:{bob_device_id}"

    # Create organization

    async with backend_administration_cmds_factory(backend_address, administration_token) as cmds:

        bootstrap_token = await cmds.organization_create(organization_id)

        organization_bootstrap_addr = BackendOrganizationBootstrapAddr.build(
            backend_address, organization_id, bootstrap_token
        )

    # Bootstrap organization and Alice user

    async with backend_anonymous_cmds_factory(organization_bootstrap_addr) as cmds:
        root_signing_key = SigningKey.generate()
        root_verify_key = root_signing_key.verify_key
        organization_addr = organization_bootstrap_addr.generate_organization_addr(root_verify_key)

        alice_device = generate_new_device(alice_device_id, organization_addr)

        save_device_with_password(config_dir, alice_device, password, force=force)

        now = pendulum.now()
        user_certificate = build_user_certificate(
            None, root_signing_key, alice_device.user_id, alice_device.public_key, now
        )
        device_certificate = build_device_certificate(
            None, root_signing_key, alice_device_id, alice_device.verify_key, now
        )

        await cmds.organization_bootstrap(
            organization_bootstrap_addr.organization_id,
            organization_bootstrap_addr.bootstrap_token,
            root_verify_key,
            user_certificate,
            device_certificate,
        )

    # Create a workspace for Alice

    config = load_config(config_dir, debug="DEBUG" in os.environ)
    async with logged_core_factory(config, alice_device) as core:
        await core.fs.workspace_create(f"/{alice_workspace}")

    # Register a new device for Alice

    token = generate_invitation_token()
    other_alice_device_id = DeviceID("@".join((alice_device.user_id, other_device_name)))
    other_alice_slugid = f"{organization_id}:{other_alice_device_id}"

    async def invite_task():
        await invite_and_create_device(alice_device, other_device_name, token)

    async def claim_task():
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
        await invite_and_create_user(alice_device, bob_device_id.user_id, token, is_admin=True)

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
        await core.fs.workspace_create(f"/{bob_workspace}")
        await core.fs.share(f"/{bob_workspace}", alice_device_id.user_id)

    # Share Alice workspace with bob

    async with logged_core_factory(config, alice_device) as core:
        await core.fs.share(f"/{alice_workspace}", bob_device_id.user_id)

    # Print out

    click.echo(
        f"""
Mount alice and bob drives using:

    $ parsec core run -P {password} -D {alice_slugid}
    $ parsec core run -P {password} -D {other_alice_slugid}
    $ parsec core run -P {password} -D {bob_slugid}
"""
    )


if __name__ == "__main__":
    main()

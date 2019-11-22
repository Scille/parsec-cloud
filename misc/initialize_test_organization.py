#! /usr/bin/env python3
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


import click

from parsec.utils import trio_run
from parsec.core.types import BackendAddr
from parsec.api.protocol import OrganizationID, DeviceID
from parsec.test_utils import initialize_test_organization

DEFAULT_ADMINISTRATION_TOKEN = "s3cr3t"


@click.command()
@click.option(
    "-B",
    "--backend-address",
    show_default=True,
    type=BackendAddr.from_url,
    default="parsec://localhost:6777?no_ssl=true",
)
@click.option("-O", "--organization-id", show_default=True, type=OrganizationID, default="corp")
@click.option("-a", "--alice-device-id", show_default=True, type=DeviceID, default="alice@laptop")
@click.option("-b", "--bob-device-id", show_default=True, type=DeviceID, default="bob@laptop")
@click.option("-o", "--other-device-name", show_default=True, default="pc")
@click.option("-x", "--alice-workspace", show_default=True, default="alice_workspace")
@click.option("-y", "--bob-workspace", show_default=True, default="bob_workspace")
@click.option("-P", "--password", show_default=True, default="test")
@click.option(
    "-T", "--administration-token", show_default=True, default=DEFAULT_ADMINISTRATION_TOKEN
)
@click.option("--force/--no-force", show_default=True, default=False)
def main(**kwargs):
    """Initialize a test origanization for parsec from a clean environment.

    You might want to create a test environment beforehand with the
    following commands:

        \b
        TMP=`mktemp -d`
        export XDG_CACHE_HOME="$TMP/cache"
        export XDG_DATA_HOME="$TMP/share"
        export XDG_CONFIG_HOME="$TMP/config"
        mkdir $XDG_CACHE_HOME $XDG_DATA_HOME $XDG_CONFIG_HOME
        parsec backend run --dev -P 6888 &

    And use `-B parsec://localhost:6888?no_ssl=true` as a backend adress.

    This scripts create two users, alice and bob who both own two devices,
    laptop and pc. They each have their workspace, respectively
    alice_workspace and bob_workspace, that their sharing with each other.
    """
    trio_run(lambda: amain(**kwargs))


async def amain(
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

    alice_slugid, other_alice_slugid, bob_slugid = await initialize_test_organization(
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
    )

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

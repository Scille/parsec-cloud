#! /usr/bin/env python3
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

"""
Create a temporary environment and initialize a test setup for parsec.

Run `misc/run_test_environment.sh --help` for more information.
"""

import os
import tempfile

import trio
import click

from parsec.utils import trio_run
from parsec.core.types import BackendAddr
from parsec.api.protocol import OrganizationID, DeviceID
from parsec.test_utils import initialize_test_organization

DEFAULT_BACKEND_PORT = 6888
DEFAULT_ADMINISTRATION_TOKEN = "V8VjaXrOz6gUC6ZEHPab0DSsjfq6DmcJ"


# Helpers


async def new_environment(source_file=None):
    export_lines = []
    tempdir = tempfile.mkdtemp()
    env = {
        "XDG_CACHE_HOME": f"{tempdir}/cache",
        "XDG_DATA_HOME": f"{tempdir}/share",
        "XDG_CONFIG_HOME": f"{tempdir}/config",
    }
    for key, value in env.items():
        await trio.Path(value).mkdir()
        os.environ[key] = value
        export_lines.append(f"export {key}={value}")

    if source_file is None:
        click.echo(
            """
[Warning] This script has not been sourced.
Please configure your environment with the following commands:
"""
        )
    else:
        click.echo(
            """
Your environment will be configured with the following commands:
"""
        )

    for line in export_lines:
        click.echo("   " + line)
    click.echo()

    if source_file is None:
        return

    async with await trio.open_file(source_file, "w") as f:
        for line in export_lines:
            await f.write(line + "\n")


async def configure_mime_types():
    XDG_DATA_HOME = os.environ["XDG_DATA_HOME"]
    desktop_file = trio.Path(f"{XDG_DATA_HOME}/applications/parsec.desktop")
    await desktop_file.parent.mkdir(exist_ok=True, parents=True)
    await desktop_file.write_text(
        """\
[Desktop Entry]
Name=Parsec
Exec=parsec core gui %u
Type=Application
Terminal=false
StartupNotify=false
StartupWMClass=Parsec
MimeType=x-scheme-handler/parsec;
"""
    )
    await trio.run_process("update-desktop-database -q".split(), check=False)
    await trio.run_process("xdg-mime default parsec.desktop x-scheme-handler/parsec".split())


async def restart_local_backend(administration_token):
    first_half = "python3 -Wignore -m parsec.cli backend run -b MOCKED --db MOCKED "
    second_half = f"-P {DEFAULT_BACKEND_PORT} --administration-token {administration_token}"
    command = first_half + second_half
    await trio.run_process(["pkill", "-f", first_half], check=False)
    await trio.open_process(command.split())
    await trio.sleep(1)
    url = f"parsec://localhost:{DEFAULT_BACKEND_PORT}?no_ssl=true"
    return BackendAddr.from_url(url)


@click.command()
@click.option("-B", "--backend-address", type=BackendAddr.from_url)
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
@click.option("-e", "--empty")
@click.option("--source-file", hidden=True)
def main(**kwargs):
    """Create a temporary environment and initialize a test setup for parsec.

    WARNING: it also leaves an in-memory backend running in the background
    on port 6888.

    It is typically a good idea to source this script in order to export the XDG
    variables so the upcoming parsec commands point to the test environment:

        \b
        $ source misc/run_test_environment.sh

    This scripts create two users, alice and bob who both own two devices,
    laptop and pc. They each have their workspace, respectively
    alice_workspace and bob_workspace, that their sharing with each other.

    The --empty (or -e) argument may be used to bypass the initialization of the
    test environment:

        \b
        $ source misc/run_test_environment.sh --empty

    This can be used to perform a user or device enrollment on the same machine.
    For instance, consider the following scenario:

        \b
        $ source misc/run_test_environment.sh
        $ parsec core gui
        # Connect as bob@laptop and register a new device called pc
        # Copy the URL

    Then, in a second terminal:

        \b
        $ source misc/run_test_environment.sh --empty
        $ xdg-open "<paste the URL here>"  # Or
        $ firefox --no-remote "<paste the URL here>"
        # A second instance of parsec pops-up
        # Enter the token to complete the registration
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
    empty,
    source_file,
):
    # Set up the temporary environment
    await new_environment(source_file)

    # Configure MIME types locally
    await configure_mime_types()

    # Keep the environment empty
    if empty:
        return

    # Start a local backend
    if backend_address is None:
        backend_address = await restart_local_backend(administration_token)

    # Initialize the test organization
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

    # Report
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

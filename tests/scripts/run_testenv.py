#! /usr/bin/env python
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations


"""
Create a temporary environment and initialize a test setup for parsec.

Run `tests/scripts/run_testenv.sh --help` for more information.
"""


import pkg_resources

# Make sure parsec is fully installed (core, backend)
pkg_resources.require("parsec-cloud[core,backend]")

import os
import sys
import re
import tempfile
import subprocess
import json
import uuid

import trio
import click
import psutil

from parsec import __version__ as PARSEC_VERSION
from parsec.utils import trio_run
from parsec.cli_utils import logging_config_options
from parsec.core.types import BackendAddr
from parsec.core.config import get_default_config_dir
from parsec.test_utils import initialize_test_organization


DEFAULT_BACKEND_PORT = 6888
DEFAULT_ADMINISTRATION_TOKEN = "V8VjaXrOz6gUC6ZEHPab0DSsjfq6DmcJ"
DEFAULT_EMAIL_HOST = "MOCKED"
DEFAULT_DEVICE_PASSWORD = "test"
DEFAULT_DATABASE = "MOCKED"
DEFAULT_BLOCKSTORE = "MOCKED"


# Helpers


async def new_environment(source_file=None):
    export_lines = []
    tempdir = tempfile.mkdtemp(prefix="parsec-testenv-")
    if sys.platform == "win32":
        export = "set"
        env = {"APPDATA": tempdir}
    else:
        export = "export"
        env = {
            "XDG_CACHE_HOME": f"{tempdir}/cache",
            "XDG_DATA_HOME": f"{tempdir}/share",
            "XDG_CONFIG_HOME": f"{tempdir}/config",
        }
    for key, value in env.items():
        os.environ[key] = value
        export_lines.append(f"{export} {key}={value}")

    if source_file is None:
        click.echo(
            """\
[Warning] This script has not been sourced.
Please configure your environment with the following commands:
"""
        )
    else:
        click.echo(
            """\
Your environment will be configured with the following commands:
"""
        )

    for line in export_lines:
        click.echo("   " + line)
    click.echo()

    if source_file is None:
        return

    async with await trio.open_file(source_file, "a") as f:
        for line in export_lines:
            await f.write(line + "\n")


async def generate_gui_config(backend_address):
    config_dir = None
    if sys.platform == "win32":
        config_dir = trio.Path(os.environ["APPDATA"]) / "parsec/config"
    else:
        config_dir = trio.Path(os.environ["XDG_CONFIG_HOME"]) / "parsec"
    await config_dir.mkdir(parents=True, exist_ok=True)

    config_file = config_dir / "config.json"

    config = {
        "gui_first_launch": False,
        "gui_check_version_at_startup": False,
        "gui_tray_enabled": False,
        "gui_last_version": PARSEC_VERSION,
        "gui_show_confined": True,
        "ipc_win32_mutex_name": f"parsec-clould-{uuid.uuid4()}",
    }
    if backend_address is not None:
        config["preferred_org_creation_backend_addr"] = backend_address.to_url()
    await config_file.write_text(json.dumps(config, indent=4))


async def configure_mime_types():
    if sys.platform == "win32" or sys.platform == "darwin":
        return
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
    try:
        await trio.run_process("update-desktop-database -q".split(), check=False)
    except FileNotFoundError:
        # Ignore if command is not available
        pass
    try:
        await trio.run_process("xdg-mime default parsec.desktop x-scheme-handler/parsec".split())
    except FileNotFoundError:
        # Ignore if command is not available
        pass


async def restart_local_backend(administration_token, backend_port, email_host, db, blockstore):
    pattern = f"parsec.* backend.* run.* -P {backend_port}"
    command = (
        f"{sys.executable} -Wignore -m parsec.cli backend run --log-level=WARNING "
        f"-b {blockstore} --db {db} "
        f"--email-host={email_host} -P {backend_port} "
        f"--spontaneous-organization-bootstrap "
        f"--administration-token {administration_token} --backend-addr parsec://localhost:{backend_port}?no_ssl=true"
    )

    # Trio does not support subprocess in windows yet

    def _windows_target():
        for proc in psutil.process_iter():
            if "python" in proc.name():
                arguments = " ".join(proc.cmdline())
                if re.search(pattern, arguments):
                    proc.kill()
        backend_process = subprocess.Popen(command.split(), stdout=subprocess.PIPE)
        for data in backend_process.stdout:
            print(data.decode(), end="")
            break
        backend_process.stdout.close()

    # Windows restart
    if sys.platform == "win32":
        await trio.to_thread.run_sync(_windows_target)

    # Linux restart
    else:

        await trio.run_process(["pkill", "-f", pattern], check=False)
        backend_process = await trio.lowlevel.open_process(command.split(), stdout=subprocess.PIPE)
        async with backend_process.stdout:
            async for data in backend_process.stdout:
                print(data.decode(), end="")
                break

    # Make sure the backend is actually started
    await trio.sleep(0.2)
    url = f"parsec://localhost:{backend_port}?no_ssl=true"
    return BackendAddr.from_url(url)


@click.command()
@click.option("-B", "--backend-address", type=BackendAddr.from_url)
@click.option("-p", "--backend-port", show_default=True, type=int, default=DEFAULT_BACKEND_PORT)
@click.option("--db", show_default=True, type=str, default=DEFAULT_DATABASE)
@click.option("-b", "--blockstore", show_default=True, type=str, default=DEFAULT_BLOCKSTORE)
@click.option("-P", "--password", show_default=True, default=DEFAULT_DEVICE_PASSWORD)
@click.option(
    "-T", "--administration-token", show_default=True, default=DEFAULT_ADMINISTRATION_TOKEN
)
@click.option("--email-host", show_default=True, default=DEFAULT_EMAIL_HOST)
@click.option("--add-random-users", show_default=True, default=0)
@click.option("--add-random-devices", show_default=True, default=0)
@click.option("-e", "--empty", is_flag=True)
@click.option("--source-file", hidden=True)
@logging_config_options(default_log_level="WARNING")
def main(log_level, log_file, log_format, **kwargs):
    """Create a temporary environment and initialize a test setup for parsec.

    WARNING: it also leaves an in-memory backend running in the background
    on port 6888.

    It is typically a good idea to source this script in order to export the XDG
    variables so the upcoming parsec commands point to the test environment:

        \b
        $ source tests/scripts/run_testenv.sh

    This scripts create two users, Alice and Bob who both own two devices,
    laptop and pc. They each have their workspace, respectively
    alice_workspace and bob_workspace, that their sharing with each other.

    The --empty (or -e) argument may be used to bypass the initialization of the
    test environment:

        \b
        $ source tests/scripts/run_testenv.sh --empty

    This can be used to perform a user or device enrollment on the same machine.
    For instance, consider the following scenario:

        \b
        $ source tests/scripts/run_testenv.sh
        $ parsec core gui
        # Connect as bob@laptop and register a new device called pc
        # Copy the URL

    Then, in a second terminal:

        \b
        $ source tests/scripts/run_testenv.sh --empty
        $ xdg-open "<paste the URL here>"  # Or
        $ firefox --no-remote "<paste the URL here>"
        # A second instance of parsec pops-up
        # Enter the token to complete the registration
    """
    trio_run(lambda: amain(**kwargs))


async def amain(
    backend_address,
    backend_port,
    db,
    blockstore,
    password,
    administration_token,
    email_host,
    add_random_users,
    add_random_devices,
    empty,
    source_file,
):
    # Set up the temporary environment
    click.echo()
    await new_environment(source_file)

    # Configure MIME types locally
    await configure_mime_types()

    # Keep the environment empty
    if empty:
        await generate_gui_config(backend_address)
        return

    # Start a local backend
    if backend_address is None:
        backend_address = await restart_local_backend(
            administration_token, backend_port, email_host, db, blockstore
        )
        click.echo(
            f"""\
A fresh backend server is now running: {backend_address.to_url()}
"""
        )
    else:
        click.echo(
            f"""\
Using existing backend: {backend_address.to_url()}
"""
        )

    # Generate dummy config file for gui
    await generate_gui_config(backend_address)

    # Initialize the test organization
    config_dir = get_default_config_dir(os.environ)
    alice_device, other_alice_device, bob_device = await initialize_test_organization(
        config_dir=config_dir,
        backend_address=backend_address,
        password=password,
        administration_token=administration_token,
        additional_users_number=add_random_users,
        additional_devices_number=add_random_devices,
    )

    # Report
    click.echo(
        f"""\
Mount alice and bob drives using:

    $ parsec core run -P {password} -D {alice_device.slughash[:3]}  # Alice
    $ parsec core run -P {password} -D {other_alice_device.slughash[:3]}  # Alice 2nd device
    $ parsec core run -P {password} -D {bob_device.slughash[:3]}  # Bob
"""
    )


if __name__ == "__main__":
    main()

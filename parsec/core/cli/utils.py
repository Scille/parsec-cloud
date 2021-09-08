# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import os
import click
from typing import List
from functools import wraps
from pathlib import Path

from parsec.core.config import get_default_config_dir, load_config
from parsec.core.local_device import (
    AvailableDevice,
    list_available_devices,
    load_device_with_password,
    LocalDeviceError,
)
from parsec.cli_utils import logging_config_options, debug_config_options, sentry_config_options


def gui_command_base_options(fn):
    # Skip INFO logs by default allows to know in a glance if something went
    # wrong when running the GUI from a terminal
    for decorator in (
        # Add --log-level/--log-format/--log-file
        logging_config_options(default_log_level="WARNING"),
        # Add --sentry-url
        sentry_config_options(configure_sentry=False),
        # Add --debug
        debug_config_options,
    ):
        fn = decorator(fn)
    return fn


def cli_command_base_options(fn):
    # CLI command have a meaningful output, so we should avoid polluting it
    # with INFO logs.
    # On top of that, they are mostly short-running command so we don't
    # bother enabling Sentry.
    for decorator in (
        # Add --log-level/--log-format/--log-file
        logging_config_options(default_log_level="WARNING"),
        # Add --debug
        debug_config_options,
    ):
        fn = decorator(fn)
    return fn


def core_config_options(fn):
    @click.option(
        "--config-dir", envvar="PARSEC_CONFIG_DIR", type=click.Path(exists=True, file_okay=False)
    )
    @wraps(fn)
    def wrapper(**kwargs):
        assert "config" not in kwargs
        config_dir = kwargs["config_dir"]
        # --sentry-url is only present for gui command
        sentry_url = kwargs.get("sentry_url")

        config_dir = Path(config_dir) if config_dir else get_default_config_dir(os.environ)
        config = load_config(config_dir=config_dir, sentry_url=sentry_url, debug=kwargs["debug"])

        kwargs["config"] = config
        return fn(**kwargs)

    return wrapper


def format_available_devices(devices: List[AvailableDevice]) -> str:
    # Try to shorten the slughash to make it easier to work with
    slughashes = [d.slughash for d in devices]
    for slughash_len in range(3, 64):
        if len({h[:slughash_len] for h in slughashes}) == len(slughashes):
            break

    out = []
    for device in devices:
        display_slughash = click.style(device.slughash[:slughash_len], fg="yellow")
        out.append(
            f"{display_slughash} - {device.organization_id}: {device.user_display} @ {device.device_display}"
        )
    return "\n".join(out)


def core_config_and_device_options(fn):
    @click.option(
        "--device",
        "-D",
        required=True,
        envvar="PARSEC_DEVICE",
        help="Device to use designed by it ID, see `list_devices` command to get the available IDs",
    )
    @click.option(
        "--password",
        "-P",
        envvar="PARSEC_DEVICE_PASSWORD",
        help="Password to decrypt Device, if not set a prompt will ask for it",
    )
    @core_config_options
    @wraps(fn)
    def wrapper(**kwargs):
        config = kwargs["config"]
        password = kwargs["password"]
        device_slughash = kwargs.pop("device")

        all_available_devices = list_available_devices(config.config_dir)
        devices = []
        for device in all_available_devices:
            if device.slughash.startswith(device_slughash):
                devices.append(device)

        if not devices:
            raise SystemExit(
                f"Device `{device_slughash}` not found, available devices:\n"
                f"{format_available_devices(all_available_devices)}"
            )
        elif len(devices) > 1:
            raise SystemExit(
                f"Multiple devices found for `{device_slughash}`:\n"
                f"{format_available_devices(devices)}"
            )

        try:
            if password is None:
                password = click.prompt("password", hide_input=True)
            device = load_device_with_password(devices[0].key_file_path, password)

        except LocalDeviceError as exc:
            raise SystemExit(f"Cannot load device {device_slughash}: {exc}")

        kwargs["device"] = device
        return fn(**kwargs)

    return wrapper

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import os
import trio
import click
from typing import Any, List, TypeVar, Callable, cast
from functools import wraps, partial
from pathlib import Path

from parsec.core.config import CoreConfig, get_default_config_dir, load_config
from parsec.core.local_device import (
    LocalDeviceError,
    DeviceFileType,
    AvailableDevice,
    list_available_devices,
    is_smartcard_extension_available,
    load_device_with_password,
    load_device_with_smartcard_sync,
    save_device_with_password_in_config,
    save_device_with_smartcard_in_config,
)
from parsec.cli_utils import (
    logging_config_options,
    debug_config_options,
    sentry_config_options,
    operation,
    async_prompt,
)
from parsec.core.types import LocalDevice


R = TypeVar("R")


def gui_command_base_options(fn: Callable[..., R]) -> Callable[..., R]:
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
        fn = cast(Callable[..., R], decorator(fn))
    return fn


def cli_command_base_options(fn: Callable[..., R]) -> Callable[..., R]:
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
        fn = cast(Callable[..., R], decorator(fn))
    return fn


def core_config_options(fn: Callable[..., R]) -> Callable[..., R]:
    @click.option(
        "--pki-extra-trust-root",
        multiple=True,
        default=(),
        type=Path,
        envvar="PARSEC_PKI_EXTRA_TRUST_ROOT",
        help="Additional path to a PKI root certificate",
    )
    @click.option(
        "--config-dir",
        envvar="PARSEC_CONFIG_DIR",
        type=click.Path(exists=True, file_okay=False, path_type=Path),
    )
    @wraps(fn)
    def wrapper(pki_extra_trust_root: List[Path], config_dir: None | Path, **kwargs: object) -> R:
        assert "config" not in kwargs
        # `--sentry-*` are only present for gui command
        sentry_dsn = kwargs.get("sentry_dsn")
        sentry_environment = kwargs.get("sentry_environment", "")

        config_dir = config_dir or get_default_config_dir(os.environ)
        config = load_config(
            config_dir=config_dir,
            sentry_dsn=sentry_dsn,
            sentry_environment=sentry_environment,
            debug=kwargs["debug"],
            pki_extra_trust_roots=pki_extra_trust_root,
        )

        kwargs["pki_extra_trust_root"] = pki_extra_trust_root
        kwargs["config_dir"] = config_dir
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
            f"{display_slughash} - {device.organization_id.str}: {device.user_display} @ {device.device_display}"
        )
    return "\n".join(out)


def core_config_and_available_device_options(fn: Callable[..., R]) -> Callable[..., R]:
    @click.option(
        "--device",
        "-D",
        "device_slughash",
        envvar="PARSEC_DEVICE",
        help="Device to use designed by it ID, see `list_devices` command to get the available IDs",
    )
    @core_config_options
    @wraps(fn)
    # MyPy don't like Any in decorator
    # type: ignore[misc]
    def wrapper(device_slughash: str | None, **kwargs: Any) -> R:
        config = kwargs["config"]
        assert isinstance(config, CoreConfig)

        all_available_devices = list_available_devices(config.config_dir)

        if device_slughash is None:
            ctx = click.get_current_context()
            click.echo(ctx.get_usage())
            click.echo(f"Try 'parsec core {ctx.command.name} --help' for help.")
            click.echo()
            raise SystemExit(
                f"Error: Missing option '--device' / '-D'.\n\n"
                f"Available devices are:\n"
                f"{format_available_devices(all_available_devices)}"
            )

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

        kwargs["device"] = devices[0]
        return fn(**kwargs)

    return wrapper


def core_config_and_device_options(fn: Callable[..., R]) -> Callable[..., R]:
    @click.option(
        "--password",
        "-P",
        envvar="PARSEC_DEVICE_PASSWORD",
        help="Password to decrypt Device, if not set a prompt will ask for it",
    )
    @core_config_and_available_device_options
    @wraps(fn)
    # MyPy don't like `Any` in decorator
    # type: ignore[misc]
    def wrapper(password: str | None, **kwargs: object) -> R:
        assert password is None or isinstance(password, str)
        available_device = kwargs.pop("device")
        assert isinstance(available_device, AvailableDevice)

        try:
            if available_device.type == DeviceFileType.PASSWORD:
                passwd: str = password or click.prompt("password", hide_input=True)
                device = load_device_with_password(available_device.key_file_path, passwd)
            elif available_device.type == DeviceFileType.SMARTCARD:
                # It's ok to be blocking here, we're not in async land yet
                device = load_device_with_smartcard_sync(available_device.key_file_path)
            else:
                raise SystemExit(
                    f"Unsupported device file authentication `{available_device.type}`"
                )

        except LocalDeviceError as exc:
            raise SystemExit(f"Cannot load device {available_device.slughash}: {exc}")

        kwargs["device"] = device
        return fn(**kwargs)

    return wrapper


def save_device_options(fn: Callable[..., R]) -> Callable[..., R]:
    @click.option(
        "--password",
        help="Password to protect the new device (you'll be prompted if not set)",
    )
    @wraps(fn)
    def wrapper(password: str | None, **kwargs: object) -> R:
        async def _save_device(config_dir: Path, device: LocalDevice, password: str | None) -> Path:

            device_display = click.style(device.slughash, fg="yellow")
            while True:
                try:
                    if password is None and is_smartcard_extension_available():
                        click.echo("Choose how to protect the new device:")
                        click.echo(f"1 - {click.style('Password', fg='yellow')} (default)")
                        click.echo(f"2 - {click.style('Smartcard', fg='yellow')}")
                        choice = await async_prompt(
                            "Your choice", type=click.Choice(["1", "2"]), default="1"
                        )
                        auth_type = "password" if choice == "1" else "smartcard"
                    else:
                        auth_type = "password"

                    if auth_type == "password":
                        password = password or await async_prompt(
                            "Select password for the new device",
                            confirmation_prompt=True,
                            hide_input=True,
                        )
                        with operation(f"Saving device {device_display}"):
                            return await trio.to_thread.run_sync(
                                partial(
                                    save_device_with_password_in_config,
                                    config_dir=config_dir,
                                    device=device,
                                    password=password,
                                )
                            )

                    else:  # smartcard
                        with operation(f"Saving device {device_display}"):
                            return await save_device_with_smartcard_in_config(
                                config_dir=config_dir, device=device
                            )

                except LocalDeviceError as exc:
                    password = None  # Reset choice
                    click.echo(f"{click.style('Error:', fg='red')} Cannot save new device ({exc})")

        kwargs["save_device_with_selected_auth"] = partial(_save_device, password=password)
        return fn(**kwargs)

    return wrapper

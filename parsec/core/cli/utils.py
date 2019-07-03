# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import trio
import click
from functools import wraps
from pathlib import Path
import pendulum

from parsec.types import DeviceID, OrganizationID
from parsec.logging import configure_logging, configure_sentry_logging
from parsec.core.config import get_default_config_dir, load_config
from parsec.core.local_device import (
    list_available_devices,
    load_device_with_password,
    load_device_with_pkcs11,
    LocalDeviceError,
)


def core_config_options(fn):
    @click.option("--config-dir", type=click.Path(exists=True, file_okay=False))
    @click.option(
        "--ssl-keyfile", type=click.Path(exists=True, dir_okay=False), help="SSL key file"
    )
    @click.option(
        "--ssl-certfile", type=click.Path(exists=True, dir_okay=False), help="SSL certificate file"
    )
    @click.option(
        "--log-level",
        "-l",
        default="WARNING",
        type=click.Choice(("DEBUG", "INFO", "WARNING", "ERROR")),
    )
    @click.option("--log-format", "-f", type=click.Choice(("CONSOLE", "JSON")))
    @click.option("--log-file", "-o")
    @click.option("--log-filter")
    @wraps(fn)
    def wrapper(config_dir, *args, **kwargs):
        assert "config" not in kwargs

        ssl_keyfile = kwargs["ssl_keyfile"]
        ssl_certfile = kwargs["ssl_certfile"]

        if ssl_certfile or ssl_keyfile:
            ssl_context = trio.ssl.create_default_context(trio.ssl.Purpose.SERVER_CLIENT)
            if ssl_certfile:
                ssl_context.load_cert_chain(ssl_certfile, ssl_keyfile)
            else:
                ssl_context.load_default_certs()
            kwargs["ssl_context"] = ssl_context

        configure_logging(
            kwargs["log_level"], kwargs["log_format"], kwargs["log_file"], kwargs["log_filter"]
        )

        config_dir = Path(config_dir) if config_dir else get_default_config_dir(os.environ)
        config = load_config(config_dir, debug="DEBUG" in os.environ)

        if config.telemetry_enabled and config.sentry_url:
            configure_sentry_logging(config.sentry_url)

        kwargs["config"] = config
        return fn(**kwargs)

    return wrapper


def _unslug(val):
    parts = val.split(":")
    if len(parts) == 1:
        return (None, DeviceID(val), val)
    elif len(parts) == 2:
        raw_org, raw_device_id = parts
        return (OrganizationID(raw_org), DeviceID(raw_device_id), val)
    else:
        raise ValueError("Must follow format `[<organization>:]<user_id>@<device_name>`")


def core_config_and_device_options(fn):
    @core_config_options
    @click.option("--device", "-D", type=_unslug, required=True)
    @click.option("--timestamp", "-t", type=pendulum.parse)
    @click.option("--password", "-P")
    @click.option("--pkcs11", is_flag=True)
    @wraps(fn)
    def wrapper(**kwargs):
        config = kwargs["config"]
        password = kwargs["password"]
        if password and kwargs["pkcs11"]:
            raise SystemExit("`--password` and `--pkcs11` options are exclusive")

        organization_id, device_id, slugname = kwargs["device"]
        devices = [
            (o, d, t, kf)
            for o, d, t, kf in list_available_devices(config.config_dir)
            if (not organization_id or o == organization_id) and d == device_id
        ]
        if not devices:
            raise SystemExit(f"Device `{slugname}` not found")
        elif len(devices) > 1:
            found = "\n".join([str(kf.parent) for *_, kf in devices])
            raise SystemExit(f"Multiple devices found for `{slugname}`:\n{found}")
        else:
            _, _, cipher, key_file = devices[0]

        try:
            if kwargs["pkcs11"]:
                if cipher != "pkcs11":
                    raise SystemExit(f"Device {slugname} is ciphered with {cipher}.")

                token_id = click.prompt("PCKS11 token id", type=int)
                key_id = click.prompt("PCKS11 key id", type=int)
                pin = click.prompt("PCKS11 pin", hide_input=True)
                device = load_device_with_pkcs11(key_file, token_id, key_id, pin)

            else:
                if cipher != "password":
                    raise SystemExit(f"Device {slugname} is ciphered with {cipher}.")

                if password is None:
                    password = click.prompt("password", hide_input=True)
                device = load_device_with_password(key_file, password)

        except LocalDeviceError as exc:
            raise SystemExit(f"Cannot load device {slugname}: {exc}")

        kwargs["device"] = device
        return fn(**kwargs)

    return wrapper

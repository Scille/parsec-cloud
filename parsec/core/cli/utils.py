import os
import trio
import click
from functools import wraps
from pathlib import Path

from parsec.types import DeviceID
from parsec.logging import configure_logging, configure_sentry_logging
from parsec.core.config import get_default_config_dir, load_config
from parsec.core.devices_manager import (
    get_cipher_info,
    load_device_with_password,
    load_device_with_pkcs11,
    DeviceManagerError,
)


def core_config_options(fn):
    @click.option("--config-dir", type=click.Path(exists=True, file_okay=False))
    @click.option(
        "--ssl-keyfile",
        default=None,
        type=click.Path(exists=True, dir_okay=False),
        help="SSL key file",
    )
    @click.option(
        "--ssl-certfile",
        default=None,
        type=click.Path(exists=True, dir_okay=False),
        help="SSL certificate file",
    )
    @click.option(
        "--log-level",
        "-l",
        default="WARNING",
        type=click.Choice(("DEBUG", "INFO", "WARNING", "ERROR")),
    )
    @click.option("--log-format", "-f", default="CONSOLE", type=click.Choice(("CONSOLE", "JSON")))
    @click.option("--log-file", "-o")
    @click.option("--log-filter", default=None)
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

        if config.sentry_url:
            configure_sentry_logging(config.sentry_url)

        kwargs["config"] = config
        return fn(**kwargs)

    return wrapper


def core_config_and_device_options(fn):
    @core_config_options
    @click.option("--device", "-D", type=DeviceID, required=True)
    @click.option("--password", "-P")
    @click.option("--pkcs11", is_flag=True)
    @wraps(fn)
    def wrapper(**kwargs):
        config = kwargs["config"]
        password = kwargs["password"]
        if password and kwargs["pkcs11"]:
            raise SystemExit("Password are PKCS11 options are exclusives.")

        device_id = kwargs["device"]
        try:
            cipher = get_cipher_info(config.config_dir, device_id)
        except DeviceManagerError as exc:
            raise SystemExit(f"Error with device {device_id}: {exc}")

        try:
            if kwargs["pkcs11"]:
                if cipher != "pkcs11":
                    raise SystemExit(f"Device {device_id} is ciphered with {cipher}.")

                token_id = click.prompt("PCKS11 token id", type=int)
                key_id = click.prompt("PCKS11 key id", type=int)
                pin = click.prompt("PCKS11 pin", hide_input=True)
                device = load_device_with_pkcs11(
                    config.config_dir, device_id, token_id, key_id, pin
                )

            else:
                if cipher != "password":
                    raise SystemExit(f"Device {device_id} is ciphered with {cipher}.")

                if password is None:
                    password = click.prompt("password", hide_input=True)
                device = load_device_with_password(config.config_dir, device_id, password)

        except DeviceManagerError as exc:
            raise SystemExit(f"Cannot load device {device_id}: {exc}")

        kwargs["device"] = device
        return fn(**kwargs)

    return wrapper

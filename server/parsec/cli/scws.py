# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import base64
from collections.abc import Callable
from functools import wraps
from pathlib import Path
from typing import Concatenate, Protocol

import click

from parsec.config import ScwsConfig


class WithScwsConfig[**P, R](Protocol):
    def __call__(
        self,
        *args: P.args,
        # Pyright complain about `scws_config` being after `*args` using ParamSpec,
        # But here I want to defined a new named parameters, I don't want it to be positional
        scws_config: ScwsConfig | None,  # type: ignore[reportGeneralTypeIssues]
        **kwargs: P.kwargs,
    ) -> R: ...


def scws_server_options[**P, R](
    fn: WithScwsConfig[P, R],
) -> Callable[Concatenate[Path | None, Path | None, str | None, P], R]:
    @click.option(
        "--scws-idopte-public-keys-file",
        type=click.Path(exists=True, dir_okay=False, file_okay=True, path_type=Path),
        envvar="PARSEC_SCWS_IDOPTE_PUBLIC_KEYS_FILE",
        show_envvar=True,
        help="PEM file containing Idopte public keys used to verify SCWS service challenge signatures",
    )
    @click.option(
        "--scws-web-application-private-key-file",
        type=click.Path(exists=True, dir_okay=False, file_okay=True, path_type=Path),
        envvar="PARSEC_SCWS_WEB_APPLICATION_PRIVATE_KEY_FILE",
        show_envvar=True,
        help="PEM file containing the web application private key used to sign SCWS web application challenges",
    )
    @click.option(
        "--scws-web-application-private-key-content",
        type=str,
        envvar="PARSEC_SCWS_WEB_APPLICATION_PRIVATE_KEY_CONTENT",
        show_envvar=True,
        help="PEM containing the web application private key used to sign SCWS web application challenges",
    )
    @wraps(fn)
    def wrapper(
        scws_idopte_public_keys_file: Path | None,
        scws_web_application_private_key_file: Path | None,
        scws_web_application_private_key_content: str | None,
        *args: P.args,
        **kwargs: P.kwargs,
    ) -> R:
        idopte_pubkeys_pem = None
        if scws_idopte_public_keys_file is not None:
            idopte_pubkeys_pem = scws_idopte_public_keys_file.read_bytes()

        match (scws_web_application_private_key_file, scws_web_application_private_key_content):
            case (None, str() as content):
                webapp_pkey_pem = base64.b64decode(content)
            case (Path() as file, None):
                webapp_pkey_pem = file.read_bytes()
            case (Path() as file, str() as content):
                raise click.UsageError("Cannot have both a file & content set for SCWS private key")
            case (None, None):
                webapp_pkey_pem = None

        scws_config = None
        if idopte_pubkeys_pem is not None and webapp_pkey_pem is not None:
            try:
                scws_config = ScwsConfig.new(idopte_pubkeys_pem, webapp_pkey_pem)
            except ValueError as e:
                raise ValueError("Invalid SCWS configuration") from e
        elif idopte_pubkeys_pem is not None or webapp_pkey_pem is not None:
            raise ValueError(
                "Both idopte public key and SCWS service private key should be provided together"
            )

        return fn(*args, **kwargs, scws_config=scws_config)

    return wrapper

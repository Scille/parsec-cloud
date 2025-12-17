# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from collections.abc import Callable
from pathlib import Path
from typing import Any

import click

from parsec._parsec import TrustAnchor
from parsec.logging import get_logger

logger = get_logger()


def _load_x509_trust_anchor(file: Path) -> TrustAnchor:
    logger.debug(f"Loading trust anchor from {file}")
    raw_cert = file.read_bytes()
    return TrustAnchor.try_from_pem(raw_cert)


def _load_x509_trust_anchor_from_dirs(_ctx, _param, raw_dirs: tuple[Path, ...]) -> list[Any]:
    trust_anchors = []
    dirs = list(raw_dirs)
    while dirs:
        dir = dirs.pop()
        logger.info(f"Loading trust anchors from {dir}")
        for f in dir.iterdir():
            if f.is_dir():
                dirs.append(f)
            else:
                trust_anchors.append(_load_x509_trust_anchor(f))
    return trust_anchors


def pki_server_options[**P, T](fn: Callable[P, T]) -> Callable[P, T]:
    decorators = [
        click.option(
            "--trusted-x509-root-dir",
            multiple=True,
            type=click.Path(dir_okay=True, file_okay=False, exists=True, path_type=Path),
            callback=_load_x509_trust_anchor_from_dirs,
            envvar="PARSEC_TRUSTED_X509_ROOT_DIR",
            show_envvar=True,
            help="A path to a directory containing x509 to trust as root certificate for PKI verification steps (not to be confused with TLS configuration)",
        )
    ]
    for decorator in decorators:
        fn = decorator(fn)
    return fn

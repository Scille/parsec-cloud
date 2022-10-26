# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec.logging import configure_sentry_logging, disable_sentry_logging
from parsec.core.config import CoreConfig


def init(config: CoreConfig) -> None:
    if config.telemetry_enabled and config.sentry_dsn:
        configure_sentry_logging(dsn=config.sentry_dsn, environment=config.sentry_environment)
    else:
        disable_sentry_logging()

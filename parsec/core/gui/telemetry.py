# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from parsec.logging import configure_sentry_logging, disable_sentry_logging
from parsec.core.config import CoreConfig


def init(config: CoreConfig):
    if config.telemetry_enabled and config.sentry_dsn:
        configure_sentry_logging(dsn=config.sentry_dsn, environment=config.sentry_environment)
    else:
        disable_sentry_logging()

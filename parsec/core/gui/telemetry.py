# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from parsec.logging import configure_sentry_logging, disable_sentry_logging


def init(config):
    if config.telemetry_enabled and config.sentry_url:
        configure_sentry_logging(config.sentry_url)
    else:
        disable_sentry_logging()

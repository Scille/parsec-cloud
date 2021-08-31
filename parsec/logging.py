# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import Optional, TextIO
import structlog
import logging
import sentry_sdk
from sentry_sdk.integrations.logging import LoggingIntegration

from parsec import __version__


def _build_formatter_renderer(log_format: str):
    if log_format == "CONSOLE":
        return structlog.dev.ConsoleRenderer()
    elif log_format == "JSON":
        return structlog.processors.JSONRenderer()
    else:
        raise ValueError(f"Unknown log format `{log_format}`")


def _build_timestamper():
    # UTC everywhere is good to avoid confusions (i.e. the `Z` suffix from
    # iso format make it very explicit). Of course if you live in place
    # such as Chatham Islands you'll have some fun comparing to your
    # local clock, but I assume you're already used to it ;)
    return structlog.processors.TimeStamper(fmt="iso", utc=True)


def _cook_log_level(log_level: Optional[str]) -> int:
    log_level = log_level.upper()
    if log_level not in ("DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL", "FATAL"):
        raise ValueError(f"Unknown log level `{log_level}`")
    return getattr(logging, log_level)


def build_structlog_configuration(log_level: str, log_format: str, log_stream: TextIO) -> dict:
    log_level = _cook_log_level(log_level)
    return {
        "processors": [
            structlog.stdlib.add_log_level,
            _build_timestamper(),
            structlog.dev.set_exc_info,
            structlog.processors.format_exc_info,
            # Finally flatten everything into a printable string
            _build_formatter_renderer(log_format),
        ],
        "wrapper_class": structlog.make_filtering_bound_logger(log_level),
        "logger_factory": structlog.PrintLoggerFactory(file=log_stream),
        "cache_logger_on_first_use": True,
    }


def configure_stdlib_logger(
    logger: logging.Logger, log_level: str, log_format: str, log_stream: TextIO
) -> None:
    log_level = _cook_log_level(log_level)

    # Use structlog to format stdlib logging records.
    # Note this is only cosmetic: stdlib is still responsible for outputing them.
    formatter = structlog.stdlib.ProcessorFormatter(
        foreign_pre_chain=[
            structlog.stdlib.add_log_level,
            _build_timestamper(),
            structlog.processors.format_exc_info,
        ],
        processor=_build_formatter_renderer(log_format),
    )
    handler = logging.StreamHandler(log_stream)
    handler.setFormatter(formatter)

    logger.addHandler(handler)
    logger.setLevel(log_level)


def configure_logging(log_level: str, log_format: str, log_stream: TextIO) -> None:
    config = build_structlog_configuration(
        log_level=log_level, log_format=log_format, log_stream=log_stream
    )
    structlog.configure(**config)
    root_stdlib_logger = logging.getLogger()
    configure_stdlib_logger(
        root_stdlib_logger, log_level=log_level, log_format=log_format, log_stream=log_stream
    )


def configure_sentry_logging(sentry_url: str) -> None:
    sentry_logging = LoggingIntegration(
        level=logging.WARNING,  # Capture warning and above as breadcrumbs
        event_level=logging.ERROR,  # Send errors as events
    )
    sentry_sdk.init(dsn=sentry_url, integrations=[sentry_logging], release=__version__)


def disable_sentry_logging() -> None:
    sentry_sdk.init(dsn=None)

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import sys
from datetime import datetime
from typing import Optional, TextIO
import structlog
import logging
from sentry_sdk import Hub as sentry_Hub, init as sentry_init
from sentry_sdk.integrations.logging import LoggingIntegration
from sentry_sdk.utils import event_from_exception

from parsec import __version__


# Long story short Python's logging is an overengineering mess, adding
# structlog and Sentry brings another layer of complexity :/
#
# What we want to achieve here:
# - Nice colored output for console
# - JSON output for log processing tools
# - structured logging and stacktrace formatting
# - output to stdout (for 12 factor app server) or file (for client)
# - Sentry integration for on-error telemetry
#
# What we need to do:
# - Configure structlog with a stream handler to spit logs
# - Configure stdlib with a separate stream handler to spit logs (redirecting
#   structlog-to-stdlib or stdlib-to-structlog is very error-prone)
# - Configure stdlib with a structlog formatter to have consistent output format
# - Configure Sentry as a processor in structlog before it formatting processor
#   (so Sentry don't end up with a single string log blob)
# - Configure Sentry stdlib integration (so 3rd party logs are also captured)
#
# Note that makes stdlib and structlog strictly separated: they both happened to
# output the same format at the same destination but don't know about each other.
#
# By the way, did you know Python's stdlib logging module was inspired by Java's ?


def _build_formatter_renderer(log_format: str):
    if log_format == "CONSOLE":
        return structlog.dev.ConsoleRenderer()
    elif log_format == "JSON":
        return structlog.processors.JSONRenderer()
    else:
        raise ValueError(f"Unknown log format `{log_format}`")


# UTC everywhere is good to avoid confusions (i.e. the `Z` suffix from
# iso format make it very explicit). Of course if you live in place
# such as Chatham Islands you'll have some fun comparing to your
# local clock, but I assume you're already used to it ;)


def _add_timestamp(logger, method_name: str, event: dict) -> dict:
    # Don't use pendulum here given this will be passed to sentry_sdk which
    # expects a regular timezone-naive datetime in UTC
    event["timestamp"] = datetime.utcnow()
    return event


def _format_timestamp(logger, method_name: str, event: dict) -> dict:
    event["timestamp"] = event["timestamp"].isoformat() + "Z"
    return event


def _cook_log_level(log_level: Optional[str]) -> int:
    log_level = log_level.upper()
    if log_level not in ("DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL", "FATAL"):
        raise ValueError(f"Unknown log level `{log_level}`")
    return getattr(logging, log_level)


_SENTRY_BREADCRUMB_LEVELS = {"info", "warning", "error", "exception"}
_SENTRY_EVENT_LEVELS = {"error", "exception"}


def _structlog_to_sentry_processor(logger, method_name: str, event: dict) -> dict:
    sentry_client = sentry_Hub.current.client
    if not sentry_client:
        return event

    # Unlike stdlib logging, structlog doesn't provide a separate logger name
    # for each place a logger is created. This is fine given event message
    # should be unique enough.
    logger_name = "parsec.structlog"
    data = event.copy()
    level = data.pop("level")
    msg = data.pop("event")
    timestamp = data.pop("timestamp")

    if level in _SENTRY_EVENT_LEVELS:
        # Cook the exception as a (class, exc, traceback) tuple
        v = data.pop("exc_info", None)
        if isinstance(v, BaseException):
            exc_info = (v.__class__, v, v.__traceback__)
        elif isinstance(v, tuple):
            exc_info = v
        elif v:
            exc_info = sys.exc_info()
            if exc_info[0] is None:
                exc_info = None
        else:
            exc_info = None

        if exc_info:
            sentry_event, _ = event_from_exception(
                exc_info=exc_info,
                client_options=sentry_client.options,
                mechanism={"type": "structlog", "handled": True},
            )
        else:
            sentry_event = {}

        sentry_event["level"] = level
        sentry_event["logger"] = logger_name
        sentry_event["timestamp"] = timestamp
        # sentry_sdk's stdlib logging integration stores the message under
        # `logentry` key with a dict containing `message` and `params`.
        # However it seems rather exotic given everywhere else the simpler
        # `message` key is used.
        sentry_event["message"] = msg
        sentry_event["extra"] = data

        sentry_Hub.current.capture_event(sentry_event)

    if level in _SENTRY_BREADCRUMB_LEVELS:
        sentry_Hub.current.add_breadcrumb(
            {
                "type": "log",
                "category": logger_name,
                "level": level,
                "message": msg,
                "timestamp": timestamp,
                "data": data,
            }
        )

    return event


def build_structlog_configuration(log_level: str, log_format: str, log_stream: TextIO) -> dict:
    log_level = _cook_log_level(log_level)
    # A bit of struclog architecture:
    # - lazy proxy: component obtained through `structlog.get_logger()`, lazyness
    #     is needed given it is imported very early on (later it bind operation
    #     will initialize the logger wrapper)
    # - logger wrapper: component responsible for all the cooking (e.g. calling processors)
    # - logger: actual component that will spit out the log to stdout/file etc.
    return {
        "processors": [
            structlog.stdlib.add_log_level,
            _add_timestamp,
            # Set `exc_info=True` if method name is `exception` and `exc_info` not set
            structlog.dev.set_exc_info,
            # Given sentry need the whole event context as a dictionary,
            # this processor must be kept just before we start formatting
            _structlog_to_sentry_processor,
            # Finally flatten everything into a printable string
            _format_timestamp,
            structlog.processors.format_exc_info,
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
            _add_timestamp,
            _format_timestamp,
            structlog.processors.format_exc_info,
        ],
        processor=_build_formatter_renderer(log_format),
    )
    handler = logging.StreamHandler(log_stream)
    handler.setFormatter(formatter)

    logger.addHandler(handler)
    logger.setLevel(log_level)


def build_sentry_configuration(dsn: str, environment: str) -> dict:
    sentry_logging = LoggingIntegration(
        level=logging.INFO,  # Capture as breadcrumbs
        event_level=logging.ERROR,  # Send as Sentry event
    )
    return {
        "dsn": dsn,
        "environment": environment,
        "release": __version__,
        "integrations": [sentry_logging],
    }


def configure_logging(log_level: str, log_format: str, log_stream: TextIO) -> None:
    config = build_structlog_configuration(
        log_level=log_level, log_format=log_format, log_stream=log_stream
    )
    structlog.configure(**config)
    root_stdlib_logger = logging.getLogger()
    configure_stdlib_logger(
        root_stdlib_logger, log_level=log_level, log_format=log_format, log_stream=log_stream
    )


def configure_sentry_logging(dsn: str, environment: str) -> None:
    config = build_sentry_configuration(dsn, environment)
    sentry_init(**config)


def disable_sentry_logging() -> None:
    sentry_init(dsn=None)

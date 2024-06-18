# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import logging
import sys
from datetime import date, datetime
from typing import Any, Callable, MutableMapping, TextIO, Union, cast

import structlog
from sentry_sdk import Hub as sentry_Hub
from sentry_sdk import init as sentry_init
from sentry_sdk.integrations.logging import LoggingIntegration
from sentry_sdk.utils import event_from_exception
from structlog._log_levels import NAME_TO_LEVEL
from structlog.dev import ConsoleRenderer
from structlog.processors import JSONRenderer
from structlog.types import EventDict, ExcInfo

from parsec._version import __version__
from parsec.config import LogLevel

# Re-expose structlog's get_logger with the correct typing
get_logger: Callable[[], ParsecBoundLogger] = structlog.get_logger


class ParsecBoundLogger(structlog.typing.FilteringBoundLogger):
    """
    Note this class is a *Protocol*, hence it is never instantiated and instead
    is only used for type checking.
    """

    # Custom helpers

    def info_with_debug_extra(self, event: str, debug_extra: dict[str, Any], **kwargs: Any) -> None:
        """
        Log at info level, but also include extra information if the log level is DEBUG.
        """
        ...

    # Method from structlog needing re-typing

    def bind(self, **new_values: Any) -> ParsecBoundLogger:
        """
        Return a new logger with *new_values* added to the existing ones.
        """
        ...

    def unbind(self, *keys: str) -> ParsecBoundLogger:
        """
        Return a new logger with *keys* removed from the context.
        """
        ...

    def try_unbind(self, *keys: str) -> ParsecBoundLogger:
        """
        Like :meth:`unbind`, but best effort: missing keys are ignored.
        """
        ...

    def new(self, **new_values: Any) -> ParsecBoundLogger:
        """
        Clear context and binds *initial_values* using `bind`.
        """
        ...


def _make_filtering_bound_logger(min_level: int) -> type[ParsecBoundLogger]:
    bound_logger_cls = structlog.make_filtering_bound_logger(min_level)

    if min_level <= logging.DEBUG:

        def info_with_debug_extra(
            self, event: str, debug_extra: dict[str, Any], **kwargs: Any
        ) -> None:
            if "event" in debug_extra:
                debug_extra["event_"] = debug_extra.pop("event")
            kwargs |= debug_extra
            self.info(event, **kwargs)
    else:

        def info_with_debug_extra(
            self, event: str, debug_extra: dict[str, Any], **kwargs: Any
        ) -> None:
            self.info(event, **kwargs)

    bound_logger_cls.info_with_debug_extra = info_with_debug_extra  # type: ignore

    return bound_logger_cls  # type: ignore


# Long story short Python's logging is an over-engineering mess, adding
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


def _build_formatter_renderer(log_format: str) -> ConsoleRenderer | JSONRenderer:
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


def _add_timestamp(
    logger: logging.Logger, method_name: str, event: MutableMapping[str, object]
) -> MutableMapping[str, object]:
    # Don't use pendulum here given this will be passed to sentry_sdk which
    # expects a regular timezone-naive datetime in UTC
    event["timestamp"] = datetime.utcnow()
    return event


def _format_timestamp(
    logger: logging.Logger, method_name: str, event: MutableMapping[str, object]
) -> MutableMapping[str, object]:
    event["timestamp"] = cast(date, event["timestamp"]).isoformat() + "Z"
    return event


def _structlog_to_sentry_processor(
    logger: logging.Logger, method_name: str, event: EventDict
) -> EventDict:
    sentry_client = sentry_Hub.current.client
    if not sentry_client:
        return event

    # Unlike stdlib logging, structlog doesn't provide a separate logger name
    # for each place a logger is created. This is fine given event message
    # should be unique enough.
    logger_name = "parsec.structlog"
    data: EventDict = {**event}
    level = data.pop("level")
    msg = data.pop("event")
    timestamp = data.pop("timestamp")

    std_level = NAME_TO_LEVEL.get(level, logging.NOTSET)
    if std_level >= logging.ERROR:
        # Cook the exception as a (class, exc, traceback) tuple
        v = cast(Union[ExcInfo, Exception, None], data.pop("exc_info", None))
        exc_info: ExcInfo | None
        if isinstance(v, BaseException):
            exc_info = (v.__class__, v, v.__traceback__)
        elif isinstance(v, tuple):
            exc_info = v
        elif v:
            match sys.exc_info():
                case (None, None, None):
                    exc_info = None
                case exc_info:
                    pass
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

        # structlog's level is type-compatible with sentry_sdk's LogLevelStr (see sentry_sdk._types)
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

    if std_level >= logging.INFO:
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


def build_structlog_configuration(
    log_level: LogLevel, log_format: str, log_stream: TextIO
) -> dict[str, Any]:
    # A bit of structlog architecture:
    # - lazy proxy: component obtained through `structlog.get_logger()`, laziness
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
        "wrapper_class": _make_filtering_bound_logger(log_level.value),
        "logger_factory": structlog.PrintLoggerFactory(file=log_stream),
        "cache_logger_on_first_use": True,
    }


def configure_stdlib_logger(
    logger: logging.Logger, log_level: LogLevel, log_format: str, log_stream: TextIO
) -> None:
    # Use structlog to format stdlib logging records.
    # Note this is only cosmetic: stdlib is still responsible for outputting them.
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
    logger.setLevel(log_level.value)


def build_sentry_configuration(dsn: str, environment: str) -> dict[str, Any]:
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


def configure_logging(log_level: LogLevel, log_format: str, log_stream: TextIO) -> None:
    config = build_structlog_configuration(
        log_level=log_level, log_format=log_format, log_stream=log_stream
    )
    structlog.configure(**config)
    root_stdlib_logger = logging.getLogger()
    configure_stdlib_logger(
        root_stdlib_logger,
        log_level=log_level,
        log_format=log_format,
        log_stream=log_stream,
    )


def configure_sentry_logging(dsn: str, environment: str) -> None:
    config = build_sentry_configuration(dsn, environment)
    sentry_init(**config)


def disable_sentry_logging() -> None:
    sentry_init(dsn=None)

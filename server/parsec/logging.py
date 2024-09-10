# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import logging
import sys
from datetime import datetime, timezone
from typing import Any, Callable, MutableMapping, TextIO, Union, cast

import sentry_sdk
import structlog
from sentry_sdk.integrations.logging import LoggingIntegration
from sentry_sdk.types import Event
from sentry_sdk.utils import event_from_exception
from structlog.dev import ConsoleRenderer
from structlog.processors import JSONRenderer
from structlog.types import EventDict, ExcInfo

from parsec._version import __version__
from parsec.config import LogLevel

# Re-expose structlog's get_logger with the correct typing
get_logger: Callable[[], ParsecBoundLogger] = structlog.get_logger


class ParsecBoundLogger(structlog.typing.FilteringBoundLogger):
    """
    This class is a *Protocol*: it is never instantiated, it is only used for type checking.
    """

    # Custom helpers

    def info_with_debug_extra(self, event: str, debug_extra: dict[str, Any], **kwargs: Any) -> None:
        """
        Log at INFO level, but also include extra information if log level is DEBUG.
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
        Like :meth:`unbind`, but missing keys are ignored.
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
# - Configure Sentry as a processor in structlog before any formatting processor
#   (so Sentry doesn't end up with a single string log blob)
# - Configure Sentry stdlib integration (so 3rd party logs are also captured)
#
# Note this makes stdlib and structlog strictly decoupled: they both output the
# same format to the same destination but don't know about each other.
#
# By the way, did you know Python's stdlib logging module was inspired by Java's ?


# -- custom structlog processors
#
# A log processor is a regular callable that process a log message and can be
# chained with other log processors. The last processor in the chain is
# a "renderer" which is used by structlog to output the log message.
#
# See: https://www.structlog.org/en/stable/processors.html


def _format_renderer_processor(log_format: str) -> ConsoleRenderer | JSONRenderer:
    if log_format == "CONSOLE":
        return structlog.dev.ConsoleRenderer()
    elif log_format == "JSON":
        return structlog.processors.JSONRenderer()
    else:
        raise ValueError(f"Unknown log format `{log_format}`")


def _add_timestamp_processor(
    logger: logging.Logger, method_name: str, event: MutableMapping[str, object]
) -> MutableMapping[str, object]:
    # Use UTC to avoid confusions
    event["timestamp"] = datetime.now(timezone.utc)
    return event


def _format_timestamp_processor(
    logger: logging.Logger, method_name: str, event: MutableMapping[str, object]
) -> MutableMapping[str, object]:
    event["timestamp"] = cast(datetime, event["timestamp"]).isoformat().replace("+00:00", "Z")
    return event


def _structlog_to_sentry_processor(
    logger: logging.Logger, method_name: str, event: EventDict
) -> EventDict:
    """A structlog processor to report events to Sentry

    - INFO & WARNING messages are captured as 'breadcrumbs'
    - ERROR & CRITICAL messages are captures as 'events'

    See ./docs/development/how-to-log-with-sentry.md
    """
    # The level number is only used for comparison below,
    # no need to keep it in the event dict
    level_number = event.pop("level_number")

    # Clone the event and pop information that will be added to the
    # Sentry event. Anything that remains in the data dictionary
    # will also be sent to Sentry but in the 'extra' key.
    data: EventDict = {**event}
    level = data.pop("level")
    message = data.pop("event")
    timestamp = data.pop("timestamp")

    # Error & Critical log levels
    if level_number >= logging.ERROR:
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
                client_options=sentry_sdk.get_client().options,
                mechanism={"type": "structlog", "handled": True},
            )
        else:
            sentry_event: Event = {}

        # Add basic attributes to Sentry event
        # See https://develop.sentry.dev/sdk/event-payloads/

        # Safe to use structlog's level as it is type-compatible with
        # sentry_sdk's LogLevelStr (see sentry_sdk._types)
        sentry_event["level"] = level
        # Unlike stdlib logging, structlog doesn't provide a separate logger name
        # for each place a logger is created. So we set a fixed logger name here.
        # Event messages should be unique enough to know where they come from.
        sentry_event["logger"] = "parsec.structlog"
        sentry_event["timestamp"] = timestamp
        # sentry_sdk's stdlib logging integration stores the message under
        # `logentry` key with a dict containing `message` and `params`.
        # However it seems rather exotic given everywhere else the simpler
        # `message` key is used.
        sentry_event["message"] = message
        # Everything that remains in the data dictionary is sent as extra data
        sentry_event["extra"] = data

        # Send the event
        sentry_sdk.capture_event(sentry_event)

    # Info & Warning log levels
    if level_number >= logging.INFO:
        # Breadcrumbs do not generate an issue. Instead, they are recorded
        # by Sentry until an event/error occurs, then they are added to
        # the issue as contextual information (useful for debugging!).
        #
        # All breadcrumb keys are optional.
        # - 'type' and 'category' are used to render breadcrumb color and icon in sentry.io
        # - 'timestamp' supports RFC3339
        # - 'data' is the place to put any additional information you'd like the
        #   breadcrumb to include.
        # See https://docs.sentry.io/product/issues/issue-details/breadcrumbs/
        sentry_sdk.add_breadcrumb(
            {
                "type": "default",
                "category": "console",
                "level": level,
                "message": message,
                "timestamp": timestamp,
                "data": data,
            }
        )

    return event


# -- Logger set up and initialization


def configure_structlog_logger(log_level: LogLevel, log_format: str, log_stream: TextIO) -> None:
    # A bit of structlog architecture:
    # - lazy proxy: component obtained through `structlog.get_logger()`, laziness
    #     is needed given it is imported very early on (later it bind operation
    #     will initialize the logger wrapper)
    # - logger wrapper: component responsible for all the cooking (e.g. calling processors)
    # - logger: actual component that will spit out the log to stdout/file etc.
    structlog.configure(
        processors=[
            structlog.processors.add_log_level,
            structlog.stdlib.add_log_level_number,
            _add_timestamp_processor,
            # Set `exc_info=True` if method name is `exception` and `exc_info` not set
            structlog.dev.set_exc_info,
            # Given Sentry needs the whole event context as a dictionary,
            # this processor must be kept just before we start formatting
            _structlog_to_sentry_processor,
            # Finally flatten everything into a printable string
            _format_timestamp_processor,
            structlog.processors.format_exc_info,
            _format_renderer_processor(log_format),
        ],
        wrapper_class=_make_filtering_bound_logger(log_level.value),
        logger_factory=structlog.PrintLoggerFactory(file=log_stream),
        cache_logger_on_first_use=True,
    )


def configure_stdlib_logger(
    logger: logging.Logger, log_level: LogLevel, log_format: str, log_stream: TextIO
) -> None:
    # Use structlog to format stdlib logging records.
    # Note this is only cosmetic: stdlib is still responsible for outputting them.
    formatter = structlog.stdlib.ProcessorFormatter(
        foreign_pre_chain=[
            structlog.stdlib.add_log_level,
            _add_timestamp_processor,
            _format_timestamp_processor,
            structlog.processors.format_exc_info,
        ],
        processor=_format_renderer_processor(log_format),
    )
    handler = logging.StreamHandler(log_stream)
    handler.setFormatter(formatter)

    logger.addHandler(handler)
    logger.setLevel(log_level.value)


def configure_logging(log_level: LogLevel, log_format: str, log_stream: TextIO) -> None:
    configure_structlog_logger(log_level, log_format, log_stream)
    configure_stdlib_logger(
        logging.getLogger(),  # root stdlib logger
        log_level,
        log_format,
        log_stream,
    )


def enable_sentry_logging(dsn: str, environment: str) -> None:
    sentry_logging = LoggingIntegration(
        level=logging.INFO,  # Capture as breadcrumbs
        event_level=logging.ERROR,  # Send as Sentry event
    )
    sentry_sdk.init(
        dsn=dsn,
        environment=environment,
        release=__version__,
        integrations=[sentry_logging],
        # Set traces_sample_rate to 1.0 to capture 100% of transactions for tracing.
        # Changing the error sample rate requires re-deployment, so instead of
        # dropping events here, it may be better to sate a rate limit at
        # project-level on https://scille.sentry.io/
        # See https://docs.sentry.io/platforms/python/configuration/sampling/
        traces_sample_rate=1.0,
        # Set profiles_sample_rate to 1.0 to profile 100% of sampled transactions.
        # With profiling, Sentry tracks performance by sampling the program's
        # call stack. This function-level information can help to improve
        # performance as Sentry highlights areas that can be optimized.
        # See https://docs.sentry.io/platforms/python/profiling/
        profiles_sample_rate=1.0,
    )


def disable_sentry_logging() -> None:
    sentry_sdk.init(dsn=None)

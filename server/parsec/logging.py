# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import enum
import logging
import sys
from datetime import datetime, timezone
from typing import Any, Callable, Literal, MutableMapping, TextIO, Union, cast

import sentry_sdk
import structlog
from sentry_sdk.integrations.asyncio import AsyncioIntegration
from sentry_sdk.types import Event
from sentry_sdk.utils import event_from_exception
from structlog.dev import ConsoleRenderer
from structlog.processors import JSONRenderer
from structlog.types import EventDict, ExcInfo

from parsec._version import __version__
from parsec.config import LogLevel

StructlogLevels = Literal["fatal", "critical", "error", "warning", "info", "debug"]


# Re-expose structlog's get_logger with the correct typing
get_logger: Callable[[], ParsecBoundLogger] = structlog.get_logger


class LogFormat(enum.Enum):
    CONSOLE = enum.auto()
    """
    Default output: provides the timestamp and a colored output in human readable format.

    e.g. `2024-12-23T10:40:51.719267Z [info     ] Parsec version                 version=3.2.1-a.0+dev`
    """

    CONSOLE_NO_TIMESTAMP = enum.auto()
    """
    Provides a colored output in human-readable format without timestamp informations.
    This is useful when used with an external log collector that adds a timestamp by itself
    (typical for 12 factor apps on PAAS like Heroku).

    e.g. `[info     ] Parsec version                 version=3.2.1-a.0+dev``
    """

    JSON = enum.auto()
    """
    Provides a JSON output without color.

    e.g. `{"version": "3.2.1-a.0+dev", "event": "Parsec version", "level": "info", "timestamp": "2024-12-23T13:17:58.739691Z"}`
    """


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
# - Configure structlog with a stream handler to spit logs.
# - Configure stdlib with a separate stream handler to spit logs (redirecting
#   structlog-to-stdlib or stdlib-to-structlog is very error-prone).
# - Configure stdlib with a structlog formatter to have consistent output format.
# - Configure Sentry as a processor in structlog before any formatting processor
#   (so Sentry doesn't end up with a single string log blob).
# - Configure Sentry stdlib integration (so 3rd party logs are also captured).
#   Note this integration uses its own handler, so it will be totally isolated
#   from our own structlog-based stdlib configuration.
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


def _format_stdlib_positional_args(
    logger: logging.Logger, method_name: str, event: MutableMapping[str, object]
) -> MutableMapping[str, object]:
    positional_args = event.pop("positional_args", ())
    if not positional_args:
        return event

    event["event"] = event["_record"].msg % positional_args  # type: ignore

    return event


def _format_stdlib_positional_args_with_color(
    logger: logging.Logger, method_name: str, event: MutableMapping[str, object]
) -> MutableMapping[str, object]:
    positional_args = event.pop("positional_args", ())
    if not positional_args:
        return event

    record = event["_record"]
    event["event"] = getattr(record, "color_message", record.msg) % positional_args  # type: ignore

    return event


def _structlog_to_sentry_processor(
    logger: logging.Logger, method_name: str, event: EventDict
) -> EventDict:
    """A structlog processor to report events to Sentry

    - INFO & WARNING messages are captured as 'breadcrumbs'
    - ERROR & CRITICAL messages are captures as 'events'

    See ./docs/development/how-to-log-with-sentry.md
    """
    # Clone the event and pop information that will be added to the
    # Sentry event. Anything that remains in the data dictionary
    # will also be sent to Sentry but in the 'extra' key.
    data: EventDict = {**event}
    level: StructlogLevels = data.pop("level")
    message = data.pop("event")
    timestamp = data.pop("timestamp")

    # Error & Critical log levels
    match level:
        case "info" | "debug" | "warning":
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
                    # Safe to use structlog's level as it is type-compatible with
                    # sentry_sdk's `LogLevelStr` (see sentry_sdk._types)
                    "level": level,
                    "message": message,
                    "timestamp": timestamp,
                    "data": data,
                }
            )

        case "error" | "critical" | "fatal":
            # Cook the exception as a (class, exc, traceback) tuple
            # This is exactly what the structlog.processors.format_exc_info does
            # (see  _figure_out_exc_info in structlog.processors.py)
            # If the event_dict contains the key "exc_info":
            #   1. If the value is a tuple, use it as-is
            #   2. If the value is an Exception, cook the tuple from the exception
            #   3. If the value is true but no tuple, obtain exc_info ourselves and cook the tuple
            #
            # NOTE: Why not directly using structlog's internal function?
            #       v = cast(Union[ExcInfo, Exception, None], data.pop("exc_info", None))
            #       exc_info: ExcInfo | None = _figure_out_exc_info(v)
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
            # sentry_sdk's `LogLevelStr` (see sentry_sdk._types)
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

        # Given `level`'s type has been obtained by a cast, we play defensive here
        # and force this sanity check that pyright would otherwise consider useless.
        case unknown:  # pyright: ignore[reportUnnecessaryComparison]
            assert False, unknown

    return event


# -- Logger set up and initialization


def _configure_structlog_logger(
    log_level: LogLevel, log_format: LogFormat, log_stream: TextIO
) -> None:
    # A bit of structlog architecture:
    # - lazy proxy: component obtained through `structlog.get_logger()`, laziness
    #     is needed given it is imported very early on (later it bind operation
    #     will initialize the logger wrapper)
    # - logger wrapper: component responsible for all the cooking (e.g. calling processors)
    # - logger: actual component that will spit out the log to stdout/file etc.

    match log_format:
        case LogFormat.CONSOLE:
            processors = [
                structlog.processors.add_log_level,
                _add_timestamp_processor,
                # Set `exc_info=True` if method name is `exception` and `exc_info` not set
                # when set to True, the format_exc_info processor below will extract
                # the exception info from sys.exc_info()
                structlog.dev.set_exc_info,
                # Given Sentry needs the whole event context as a dictionary,
                # this processor must be kept just before we start formatting
                _structlog_to_sentry_processor,
                # Finally flatten everything into a printable string
                # (note `exc_info` is taken care of by `ConsoleRenderer`)
                _format_timestamp_processor,
                ConsoleRenderer(),
            ]
        case LogFormat.CONSOLE_NO_TIMESTAMP:
            console_renderer = ConsoleRenderer()
            # Remove the first column which is supposed to format the timestamp
            timestamp_column, *console_renderer._columns = console_renderer._columns
            assert timestamp_column.key == "timestamp"  # Sanity check

            processors = [
                structlog.processors.add_log_level,
                # Even if timestamp is not displayed, we still need it for Sentry
                _add_timestamp_processor,
                # Set `exc_info=True` if method name is `exception` and `exc_info` not set
                # when set to True, the format_exc_info processor below will extract
                # the exception info from sys.exc_info()
                structlog.dev.set_exc_info,
                # Given Sentry needs the whole event context as a dictionary,
                # this processor must be kept just before we start formatting
                _structlog_to_sentry_processor,
                # Finally flatten everything into a printable string.
                # Notes:
                # - `exc_info` is taken care of by `ConsoleRenderer`)
                # - `timestamp` field is ignored by the renderer
                console_renderer,
            ]
        case LogFormat.JSON:
            processors = [
                structlog.processors.add_log_level,
                _add_timestamp_processor,
                # Set `exc_info=True` if method name is `exception` and `exc_info` not set
                # when set to True, the format_exc_info processor below will extract
                # the exception info from sys.exc_info()
                structlog.dev.set_exc_info,
                # Given Sentry needs the whole event context as a dictionary,
                # this processor must be kept just before we start formatting
                _structlog_to_sentry_processor,
                # Finally flatten everything into a printable string
                _format_timestamp_processor,
                structlog.processors.format_exc_info,
                JSONRenderer(),
            ]

    structlog.configure(
        processors=processors,
        wrapper_class=_make_filtering_bound_logger(log_level.value),
        logger_factory=structlog.PrintLoggerFactory(file=log_stream),
        cache_logger_on_first_use=True,
    )


def _configure_stdlib_logger(
    logger: logging.Logger, log_level: LogLevel, log_format: LogFormat, log_stream: TextIO
) -> None:
    logger.setLevel(log_level.value)

    # Given we want a consistent output between our logs (that uses structlog) and 3rd
    # party ones (that uses stdlib's logging module), we configure here a handler
    # that will handle formatting with structlog tools.
    #
    # Notes:
    # - This is only cosmetic: stdlib is still responsible for outputting the logs.
    # - Structlog integration with stdlib logger is done with a dedicated handler,
    #   hence this configuration won't affect it.

    match log_format:
        case LogFormat.CONSOLE:
            processors = [
                structlog.processors.add_log_level,
                _add_timestamp_processor,
                # Set `exc_info=True` if method name is `exception` and `exc_info` not set
                # when set to True, the format_exc_info processor below will extract
                # the exception info from sys.exc_info()
                structlog.dev.set_exc_info,
                # Finally flatten everything into a printable string
                # (note `exc_info` is taken care of by `ConsoleRenderer`)
                _format_stdlib_positional_args_with_color,
                _format_timestamp_processor,
                # Remove ``_record`` and ``_from_structlog`` from *event_dict*.
                # This must be done last since we don't want them displayed by the renderer,
                # but `_format_stdlib_positional_args` needs them.
                structlog.stdlib.ProcessorFormatter.remove_processors_meta,
                ConsoleRenderer(),
            ]
        case LogFormat.CONSOLE_NO_TIMESTAMP:
            console_renderer = ConsoleRenderer()
            # Remove the first column which is supposed to format the timestamp
            timestamp_column, *console_renderer._columns = console_renderer._columns
            assert timestamp_column.key == "timestamp"  # Sanity check

            processors = [
                structlog.processors.add_log_level,
                # Set `exc_info=True` if method name is `exception` and `exc_info` not set
                # when set to True, the format_exc_info processor below will extract
                # the exception info from sys.exc_info()
                structlog.dev.set_exc_info,
                # Finally flatten everything into a printable string.
                # Notes:
                # - `exc_info` is taken care of by `ConsoleRenderer`)
                # - `timestamp` field is ignored by the renderer
                _format_stdlib_positional_args_with_color,
                # Remove ``_record`` and ``_from_structlog`` from *event_dict*.
                # This must be done last since we don't want them displayed by the renderer,
                # but `_format_stdlib_positional_args` needs them.
                structlog.stdlib.ProcessorFormatter.remove_processors_meta,
                console_renderer,
            ]
        case LogFormat.JSON:
            processors = [
                structlog.processors.add_log_level,
                _add_timestamp_processor,
                # Set `exc_info=True` if method name is `exception` and `exc_info` not set
                # when set to True, the format_exc_info processor below will extract
                # the exception info from sys.exc_info()
                structlog.dev.set_exc_info,
                # Finally flatten everything into a printable string
                _format_stdlib_positional_args,
                _format_timestamp_processor,
                structlog.processors.format_exc_info,
                # Remove ``_record`` and ``_from_structlog`` from *event_dict*.
                # This must be done last since we don't want them displayed by the renderer,
                # but `_format_stdlib_positional_args` needs them.
                structlog.stdlib.ProcessorFormatter.remove_processors_meta,
                JSONRenderer(),
            ]

    formatter = structlog.stdlib.ProcessorFormatter(
        # By default structlog formats the log message (i.e.`Started server process [%d]` -> `Started server process [255046]`),
        # however we need to disable this behavior to support Uvicorn's colored output.
        #
        # Indeed, Uvicorn's log message sometime contains an additional `color_message` field
        # that should be used instead of the regular `msg` field during formatting to end
        # up with a colored message.
        # So the solution is:
        # - Set `use_get_message=False` to disable default formatting
        # - Set `pass_foreign_args=True` to ask structlog to keep the args in a dedicated
        #   `positional_args` field (since structlogs clears the log's `args` attribute that
        #   contains them in the first place).
        # - Add a processor that will do the actual formatting using `msg` or `color_message` field.
        use_get_message=False,
        pass_foreign_args=True,
        processors=processors,
    )
    handler = logging.StreamHandler(log_stream)
    handler.setFormatter(formatter)

    logger.addHandler(handler)


def configure_logging(log_level: LogLevel, log_format: LogFormat, log_stream: TextIO) -> None:
    _configure_structlog_logger(log_level, log_format, log_stream)
    _configure_stdlib_logger(
        logging.getLogger(),  # root stdlib logger
        log_level,
        log_format,
        log_stream,
    )


def enable_sentry_logging(dsn: str, environment: str) -> None:
    sentry_sdk.init(
        dsn=dsn,
        environment=environment,
        release=__version__,
        integrations=[
            # Note that Sentry automatically enables some integrations, like:
            # AsyncPGIntegration (https://docs.sentry.io/platforms/python/integrations/asyncpg/)
            # FastApiIntegration (https://docs.sentry.io/platforms/python/integrations/fastapi/)
            # LoggingIntegration (https://docs.sentry.io/platforms/python/integrations/logging/)
            # StarletteIntegration (https://docs.sentry.io/platforms/python/integrations/starlette/)
            #
            # The asyncio integration is not enabled automatically
            # and needs to be added manually.
            # See https://docs.sentry.io/platforms/python/integrations/asyncio/
            AsyncioIntegration(),
        ],
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

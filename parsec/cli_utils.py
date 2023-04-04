# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import datetime
import sys
import traceback
from contextlib import asynccontextmanager, contextmanager
from functools import partial, wraps
from typing import (
    Any,
    AsyncIterator,
    Awaitable,
    Callable,
    Dict,
    Iterator,
    NoReturn,
    TextIO,
    TypedDict,
    TypeVar,
    cast,
)

import click
import trio
from typing_extensions import Concatenate, ParamSpec

from parsec._parsec import DateTime
from parsec._version import __version__
from parsec.logging import configure_logging, configure_sentry_logging
from parsec.utils import open_service_nursery

P = ParamSpec("P")
R = TypeVar("R")


class SchemesInternalType(TypedDict):
    interval: int
    frames: list[str]


# Scheme stolen from py-spinners
# MIT License Copyright (c) 2017 Manraj Singh
# (https://github.com/manrajgrover/py-spinners)
SCHEMES: dict[str, SchemesInternalType] = {
    "dots": {"interval": 80, "frames": ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]}
}

ok = click.style("✔", fg="green")
ko = click.style("✘", fg="red")


@contextmanager
def operation(txt: str) -> Iterator[None]:
    click.echo(txt, nl=False)
    try:
        yield

    except Exception:
        click.echo(f"\r\033[K{txt} {ko}")
        raise

    else:
        click.echo(f"\r\033[K{txt} {ok}")


@asynccontextmanager
async def spinner(
    txt: str, sep: str = " ", scheme: str = "dots", color: str = "magenta"
) -> AsyncIterator[None]:
    scheme_theme = SCHEMES[scheme]
    interval = scheme_theme["interval"]
    frames = scheme_theme["frames"]
    result: str | None = None

    def _render_line(frame: str | None) -> None:
        # Clear line then re-print it
        click.echo(f"\r\033[K{txt}{sep}{frame}", nl=False)

    async def _update_spinner() -> NoReturn:
        try:
            i: int = 1
            while True:
                await trio.sleep(interval / 1000)
                _render_line(click.style(frames[i], fg=color))
                i = (i + 1) % len(frames)
        finally:
            # Last render for result
            _render_line(result)
            click.echo()

    async with open_service_nursery() as nursery:
        _render_line(frames[0])
        nursery.start_soon(_update_spinner)

        try:
            yield

        except Exception:
            result = ko
            raise

        else:
            result = ok

        finally:
            nursery.cancel_scope.cancel()


@contextmanager
def cli_exception_handler(debug: bool) -> Iterator[bool]:
    try:
        yield debug

    except KeyboardInterrupt:
        click.echo("bye ;-)")
        raise SystemExit(0)

    except Exception as exc:
        exc_msg = str(exc)
        if not exc_msg.strip():
            exc_msg = repr(exc)
        click.echo(click.style("Error: ", fg="red") + exc_msg)
        if debug:
            raise
        else:
            raise SystemExit(1)


def generate_not_available_cmd_group(exc: BaseException, hint: str | None = None) -> click.Group:
    error_msg = "".join(
        [
            click.style("Not available: ", fg="red"),
            "Importing this module has failed with error:\n\n",
            *traceback.format_exception(type(exc), exc, exc.__traceback__),
            f"\n\n{hint}\n" if hint else "",
        ]
    )

    class NotAvailableGroup(click.Group):
        def get_command(self, ctx: click.Context, cmd_name: str) -> click.Command | None:
            raise SystemExit(error_msg)

        def list_commands(self, ctx: click.Context) -> list[str]:
            raise SystemExit(error_msg)

    return NotAvailableGroup(
        help=f"Not available{' (' + hint + ')' if hint else ''}",
    )


def generate_not_available_cmd(exc: BaseException, hint: str | None = None) -> click.Command:
    error_msg = "".join(
        [
            click.style("Not available: ", fg="red"),
            "Importing this module has failed with error:\n\n",
            *traceback.format_exception(type(exc), exc, exc.__traceback__),
            f"\n\n{hint}\n" if hint else "",
        ]
    )

    @click.command(
        context_settings=dict(ignore_unknown_options=True),
        help=f"Not available{' (' + hint + ')' if hint else ''}",
    )
    @click.argument("args", nargs=-1, type=click.UNPROCESSED)
    def bad_cmd(args: Any) -> NoReturn:
        raise SystemExit(error_msg)

    return bad_cmd


def async_wrapper(fn: Callable[P, R]) -> Callable[P, Awaitable[R]]:
    async def wrapper(*args: P.args, **kwargs: P.kwargs) -> R:
        return await trio.to_thread.run_sync(partial(fn, *args, **kwargs))

    return wrapper


async_confirm = async_wrapper(click.confirm)
async_prompt = async_wrapper(click.prompt)


def logging_config_options(
    default_log_level: str,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    LOG_LEVELS = ("DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL")
    assert default_log_level in LOG_LEVELS

    def _logging_config_options(fn: Callable[P, R]) -> Callable[Concatenate[str, str, str, P], R]:
        @click.option(
            "--log-level",
            "-l",
            type=click.Choice(LOG_LEVELS, case_sensitive=False),
            default=default_log_level,
            show_default=True,
            envvar="PARSEC_LOG_LEVEL",
        )
        @click.option(
            "--log-format",
            "-f",
            type=click.Choice(("CONSOLE", "JSON"), case_sensitive=False),
            default="CONSOLE",
            show_default=True,
            envvar="PARSEC_LOG_FORMAT",
        )
        @click.option(
            "--log-file", "-o", default=None, envvar="PARSEC_LOG_FILE", help="[default: stderr]"
        )
        @wraps(fn)
        def wrapper(
            log_level: str, log_format: str, log_file: str | None, *args: P.args, **kwargs: P.kwargs
        ) -> R:
            # `click.open_file` considers "-" to be stdout
            if log_file in (None, "-"):

                @contextmanager
                def open_log_file() -> Iterator[TextIO]:
                    yield sys.stderr

            else:

                @contextmanager
                def open_log_file() -> Iterator[TextIO]:
                    assert log_file is not None
                    yield cast(TextIO, click.open_file(filename=log_file, mode="w"))

            kwargs["log_level"] = log_level
            kwargs["log_format"] = log_format
            kwargs["log_file"] = log_file

            with open_log_file() as fd:
                configure_logging(log_level=log_level, log_format=log_format, log_stream=fd)

                return fn(*args, **kwargs)

        return wrapper

    return _logging_config_options


def sentry_config_options(
    configure_sentry: bool,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    def _sentry_config_options(fn: Callable[P, R]) -> Callable[Concatenate[str, str, P], R]:
        # Sentry SKD uses 3 environ variables during it configuration phase:
        # - `SENTRY_DSN`
        # - `SENTRY_ENVIRONMENT`
        # - `SENTRY_RELEASE`
        # Those variable are only used if the corresponding parameter is not
        # explicitly provided while calling `sentry_init(**config)`.
        # Hence we make sure we provide the three parameters (note the release
        # is determined from Parsec's version) so those `PARSEC_*` env vars
        # are never read and don't clash with the `PARSEC_SENTRY_*` ones.
        @click.option(
            "--sentry-dsn",
            metavar="URL",
            envvar="PARSEC_SENTRY_DSN",
            help="Sentry Data Source Name for telemetry report",
        )
        @click.option(
            "--sentry-environment",
            metavar="NAME",
            envvar="PARSEC_SENTRY_ENVIRONMENT",
            default="production",
            show_default=True,
            help="Sentry environment for telemetry report",
        )
        @wraps(fn)
        def wrapper(
            sentry_dsn: str | None, sentry_environment: str, *args: P.args, **kwargs: P.kwargs
        ) -> R:
            if configure_sentry and sentry_dsn:
                configure_sentry_logging(dsn=sentry_dsn, environment=sentry_environment)

            kwargs["sentry_dsn"] = sentry_dsn
            kwargs["sentry_environment"] = sentry_environment

            return fn(*args, **kwargs)

        return wrapper

    return _sentry_config_options


def version_option(fn: Callable[P, R]) -> Callable[P, R]:
    return click.version_option(version=__version__, prog_name="parsec")(fn)


def debug_config_options(fn: Callable[P, R]) -> Callable[Concatenate[bool, P], R]:
    for decorator in (
        click.option(
            "--debug",
            is_flag=True,
            # Don't prefix with `PARSEC_` given devs are lazy
            envvar="DEBUG",
        ),
        version_option,
    ):
        fn = decorator(fn)  # type: ignore[operator]

    return cast(Callable[Concatenate[bool, P], R], fn)


class ParsecDateTimeClickType(click.ParamType):
    """
    Add support for RFC3339 date time to `click.DateTime`.

    Funny enough, `click.DateTime` only support local time (e.g. 2000-01-01T00:00:00)
    while we precisely want the exact opposite: only support time in Zoulou
    format (e.g. 2000-01-01T00:00:00Z).

    The rational for this is using local time is very error prone:
    - Copy/pasting between computers `2000-01-01T00:00:00` may changes it meaning
    - The convenient date only format (e.g. `2000-01-01`) becomes ambiguous given
      we don't know if it should use local time or not (for instance with a CET
      timezone `2000-01-01` gets translated to `1999-12-31T23:00:00Z`, even if
      we are in summer and hence current local time is UTC+1 :/)

    So the simple fix is to only allow the Zoulou format (e.g. `2000-01-01T00:00:00Z`)
    and consider the date only format as shortcut for not typing the final `T00:00:00Z`.

    On top of that, we don't support the full range of timezone but only `Z` (so
    `2000-01-01T00:00:00+01:00` is not supported), this makes code much simpler
    and should be enough in most cases.
    """

    name = "datetime"
    formats = {
        "short": "%Y-%m-%d",
        "long": "%Y-%m-%dT%H:%M:%SZ",
        "long_with_millisecond": "%Y-%m-%dT%H:%M:%S.%fZ",
    }

    def __repr__(self) -> str:
        return "ParsecDateTimeClickType"

    def to_info_dict(self) -> Dict[str, Any]:
        info_dict = super().to_info_dict()
        info_dict["formats"] = self.formats
        return info_dict

    def get_metavar(self, param: click.Parameter) -> str:
        return f"[2000-01-01|2000-01-01T00:00:00Z]"

    def convert(
        self, value: Any, param: click.Parameter | None, ctx: click.Context | None
    ) -> DateTime:
        if isinstance(value, DateTime):
            return value

        assert isinstance(value, str)

        py_datetime: datetime.datetime | None = None
        try:
            # Try short format
            py_datetime = datetime.datetime.strptime(value, self.formats["short"])
        except ValueError:
            try:
                # Try long format
                py_datetime = datetime.datetime.strptime(value, self.formats["long"])
            except ValueError:
                try:
                    # Long format with Microsecond ?
                    py_datetime = datetime.datetime.strptime(
                        value, self.formats["long_with_millisecond"]
                    )
                except ValueError:
                    pass

        if not py_datetime:
            self.fail(
                f"`{value}` must be a RFC3339 date/datetime (e.g. `2000-01-01` or `2000-01-01T00:00:00Z`)",
                param,
                ctx,
            )

        # strptime consider the provided datetime to be in local time,
        # so we must correct it given we know it is in fact a UTC
        py_datetime = py_datetime.replace(tzinfo=datetime.timezone.utc)

        return DateTime.from_timestamp(py_datetime.timestamp())

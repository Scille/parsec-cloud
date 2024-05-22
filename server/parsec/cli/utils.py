# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import datetime
import traceback
from contextlib import asynccontextmanager, contextmanager
from functools import partial
from typing import (
    Any,
    AsyncIterator,
    Awaitable,
    Callable,
    ClassVar,
    Iterator,
    NoReturn,
    ParamSpec,
    TypedDict,
    TypeVar,
)

import anyio
import click

from parsec._parsec import DateTime


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
                await anyio.sleep(interval / 1000)
                _render_line(click.style(frames[i], fg=color))
                i = (i + 1) % len(frames)
        finally:
            # Last render for result
            _render_line(result)
            click.echo()

    async with anyio.create_task_group() as task_group:
        _render_line(frames[0])
        task_group.start_soon(_update_spinner)

        try:
            yield

        except Exception:
            result = ko
            raise

        else:
            result = ok

        finally:
            task_group.cancel_scope.cancel()


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


P = ParamSpec("P")
R = TypeVar("R")


def async_wrapper(fn: Callable[P, R]) -> Callable[P, Awaitable[R]]:
    async def wrapper(*args: P.args, **kwargs: P.kwargs) -> R:
        return await anyio.to_thread.run_sync(partial(fn, *args, **kwargs))

    return wrapper


async_confirm = async_wrapper(click.confirm)
async_prompt = async_wrapper(click.prompt)


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
    formats: ClassVar = {
        "short": "%Y-%m-%d",
        "long": "%Y-%m-%dT%H:%M:%SZ",
        "long_with_millisecond": "%Y-%m-%dT%H:%M:%S.%fZ",
    }

    def __repr__(self) -> str:
        return "ParsecDateTimeClickType"

    def to_info_dict(self) -> dict[str, Any]:
        info_dict = super().to_info_dict()
        info_dict["formats"] = self.formats
        return info_dict

    def get_metavar(self, param: click.Parameter) -> str:
        return "[2000-01-01|2000-01-01T00:00:00Z]"

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

        return DateTime.from_timestamp_micros(int(py_datetime.timestamp() * 1_000_000))

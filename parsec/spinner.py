import trio
import click
from async_generator import asynccontextmanager
from contextlib import contextmanager

# Scheme stolen from py-spinners
# MIT License Copyright (c) 2017 Manraj Singh
# (https://github.com/manrajgrover/py-spinners)
SCHEMES = {"dots": {"interval": 80, "frames": ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]}}

ok = click.style("✔", fg="green")
ko = click.style("✘", fg="red")


@contextmanager
def operation(txt):

    click.echo(txt, nl=False)
    try:

        yield

    except Exception:
        click.echo(f"\r\033[K{txt} {ko}")
        raise

    else:
        click.echo(f"\r\033[K{txt} {ok}")


@asynccontextmanager
async def spinner(txt, sep=" ", scheme="dots", color="magenta"):
    interval = SCHEMES[scheme]["interval"]
    frames = SCHEMES[scheme]["frames"]
    result = None

    def _render_line(frame):
        # Clear line then re-print it
        click.echo(f"\r\033[K{txt}{sep}{frame}", nl=False)

    async def _update_spinner():
        try:
            i = 1
            while True:
                await trio.sleep(interval / 1000)
                _render_line(click.style(frames[i], fg=color))
                i = (i + 1) % len(frames)
        finally:
            # Last render for result
            _render_line(result)
            click.echo()

    async with trio.open_nursery() as nursery:
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

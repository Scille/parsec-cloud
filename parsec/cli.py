import click

from parsec.cli_utils import generate_not_available_cmd


try:
    from parsec.core.cli import core_cmd
except ImportError as exc:
    core_cmd = generate_not_available_cmd(exc)


try:
    from parsec.backend.cli import backend_cmd
except ImportError as exc:
    backend_cmd = generate_not_available_cmd(exc)


@click.group()
def cli():
    pass


cli.add_command(core_cmd, "core")
cli.add_command(backend_cmd, "backend")


if __name__ == "__main__":
    cli()

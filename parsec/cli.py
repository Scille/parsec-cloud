import click

from parsec.core.cli import core_cmd
from parsec.backend.cli import backend_cmd


@click.group()
def cli():
    pass


cli.add_command(core_cmd, 'core')
cli.add_command(backend_cmd, 'backend')


if __name__ == '__main__':
    cli()

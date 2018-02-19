import click

from parsec.core.cli import core_cmd
from parsec.backend.cli import backend_cmd
from parsec.ui import fuse, shell


@click.group()
def cli():
    pass


cli.add_command(core_cmd, 'core')
cli.add_command(backend_cmd, 'backend')
cli.add_command(fuse.cli, 'fuse')
cli.add_command(shell.cli, 'shell')


if __name__ == '__main__':
    cli()

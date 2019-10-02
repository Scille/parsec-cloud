# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import click
import sys
import os

from parsec._version import __version__
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
@click.version_option(version=__version__, prog_name="parsec")
def cli():
    pass


cli.add_command(core_cmd, "core")
cli.add_command(backend_cmd, "backend")


if __name__ == "__main__":
    raw_extra_args = os.environ.get("PARSEC_CMD_ARGS", "")
    extra_args = [os.path.expandvars(x) for x in raw_extra_args.split()]
    argv = sys.argv[1:] + extra_args
    cli(argv)

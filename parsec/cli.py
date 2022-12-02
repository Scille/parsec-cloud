# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import os
import sys
from typing import Any, Sequence

import click

from parsec._version import __version__
from parsec.cli_utils import generate_not_available_cmd_group


try:
    from parsec.core.cli import core_cmd_group
except ImportError as exc:
    core_cmd_group = generate_not_available_cmd_group(exc)


try:
    from parsec.backend.cli import backend_cmd_group
except ImportError as exc:
    backend_cmd_group = generate_not_available_cmd_group(exc)


@click.group()
@click.version_option(version=__version__, prog_name="parsec")
def cli() -> None:
    pass


cli.add_command(core_cmd_group, "core")
cli.add_command(backend_cmd_group, "backend")

# Add support for PARSEC_CMD_ARGS env var

vanilla_cli_main = cli.main


def patched_cli_main(args: Sequence[str] | None = None, **kwargs: Any) -> Any:
    args_list = list(args) if args is not None else sys.argv[1:]

    raw_extra_args = os.environ.get("PARSEC_CMD_ARGS", "")
    args_list += [os.path.expandvars(x) for x in raw_extra_args.split()]

    return vanilla_cli_main(args=args_list[:], **kwargs)


# Mypy: here we want to use the patched main instead of the provided one by `BaseCommand`
# Note: We could use monkeypatch instead :thinking:
cli.main = patched_cli_main  # type: ignore[assignment]


if __name__ == "__main__":
    cli()

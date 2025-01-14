# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import os
import sys
from typing import Any, Sequence

import click

from parsec.cli.export import export_realm
from parsec.cli.inspect import human_accesses
from parsec.cli.migration import migrate
from parsec.cli.options import version_option
from parsec.cli.run import run_cmd
from parsec.cli.sequester_create import create_service, generate_service_certificate
from parsec.cli.sequester_list import list_services
from parsec.cli.sequester_revoke import generate_service_revocation_certificate, revoke_service
from parsec.cli.testbed import TESTBED_AVAILABLE, testbed_cmd

__all__ = ("cli",)


@click.group(
    short_help="Handle sequestered organization",
)
@version_option
def server_sequester_cmd() -> None:
    pass


server_sequester_cmd.add_command(list_services, "list_services")
server_sequester_cmd.add_command(generate_service_certificate, "generate_service_certificate")
server_sequester_cmd.add_command(create_service, "create_service")
server_sequester_cmd.add_command(
    generate_service_revocation_certificate, "generate_service_revocation_certificate"
)
server_sequester_cmd.add_command(revoke_service, "revoke_service")


@click.group()
@version_option
def cli() -> None:
    pass


cli.add_command(run_cmd, "run")
cli.add_command(migrate, "migrate")
cli.add_command(export_realm, "export_realm")
cli.add_command(human_accesses, "human_accesses")
cli.add_command(server_sequester_cmd, "sequester")
if TESTBED_AVAILABLE:
    cli.add_command(testbed_cmd, "testbed")


# Add support for PARSEC_CMD_ARGS env var

vanilla_cli_main = cli.main


def patched_cli_main(args: Sequence[str] | None = None, **kwargs: Any) -> Any:
    args_list = list(args) if args is not None else sys.argv[1:]

    raw_extra_args = os.environ.get("PARSEC_CMD_ARGS", "")
    args_list += [os.path.expandvars(x) for x in raw_extra_args.split()]

    return vanilla_cli_main(args=args_list[:], **kwargs)


cli.main = patched_cli_main  # pyright: ignore[reportAttributeAccessIssue]

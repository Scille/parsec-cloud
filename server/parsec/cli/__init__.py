# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import click

from parsec.cli.migration import migrate
from parsec.cli.options import version_option
from parsec.cli.run import run_cmd
from parsec.cli.sequester import (
    create_service,
    export_realm,
    extract_realm_export,
    generate_service_certificate,
    human_accesses,
    import_service_certificate,
    list_services,
    update_service,
)
from parsec.cli.testbed import TESTBED_AVAILABLE, testbed_cmd

__all__ = ("cli",)


@click.group(short_help="Handle sequestered organization")
@version_option
def server_sequester_cmd() -> None:
    pass


server_sequester_cmd.add_command(create_service, "create_service")
server_sequester_cmd.add_command(list_services, "list_services")
server_sequester_cmd.add_command(update_service, "update_service")
server_sequester_cmd.add_command(export_realm, "export_realm")
server_sequester_cmd.add_command(extract_realm_export, "extract_realm_export")
server_sequester_cmd.add_command(generate_service_certificate, "generate_service_certificate")
server_sequester_cmd.add_command(import_service_certificate, "import_service_certificate")


@click.group()
@version_option
def cli() -> None:
    pass


cli.add_command(run_cmd, "run")
cli.add_command(migrate, "migrate")
cli.add_command(human_accesses, "human_accesses")
cli.add_command(server_sequester_cmd, "sequester")
if TESTBED_AVAILABLE:
    cli.add_command(testbed_cmd, "testbed")

# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import click

from parsec.backend.cli.run import run_cmd
from parsec.backend.cli.migration import migrate
from parsec.backend.cli.sequester import (
    create_service,
    list_services,
    update_service,
    human_accesses,
    export_realm,
    extract_realm_export,
)


__all__ = ("backend_cmd",)


@click.group(short_help="Handle sequestered organization")
def backend_sequester_cmd():
    pass


backend_sequester_cmd.add_command(create_service, "create_service")
backend_sequester_cmd.add_command(list_services, "list_services")
backend_sequester_cmd.add_command(update_service, "update_service")
backend_sequester_cmd.add_command(export_realm, "export_realm")
backend_sequester_cmd.add_command(extract_realm_export, "extract_realm_export")


@click.group()
def backend_cmd():
    pass


backend_cmd.add_command(run_cmd, "run")
backend_cmd.add_command(migrate, "migrate")
backend_cmd.add_command(human_accesses, "human_accesses")
backend_cmd.add_command(backend_sequester_cmd, "sequester")

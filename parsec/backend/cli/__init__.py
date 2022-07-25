# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

import click

from parsec.backend.cli.run import run_cmd
from parsec.backend.cli.migration import migrate
from parsec.backend.cli.sequester import (
    create_service,
    list_services,
    update_service,
    human_accesses,
    export_realm,
)


__all__ = ("backend_cmd",)


@click.group(short_help="Handle sequestered organization")
def backend_sequester_cmd():
    pass


backend_sequester_cmd.add_command(create_service, "create_service")
backend_sequester_cmd.add_command(list_services, "list_services")
backend_sequester_cmd.add_command(update_service, "update_service")
backend_sequester_cmd.add_command(export_realm, "export_realm")


@click.group()
def backend_cmd():
    pass


backend_cmd.add_command(run_cmd, "run")
backend_cmd.add_command(migrate, "migrate")
backend_cmd.add_command(human_accesses, "human_accesses")
backend_cmd.add_command(backend_sequester_cmd, "sequester")

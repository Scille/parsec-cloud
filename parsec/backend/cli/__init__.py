# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import click

from parsec.backend.cli.migration import migrate
from parsec.backend.cli.run import run_cmd
from parsec.backend.cli.sequester import (
    create_service,
    export_realm,
    extract_realm_export,
    generate_service_certificate,
    human_accesses,
    import_service_certificate,
    list_services,
    update_service,
)


__all__ = ("backend_cmd_group",)


@click.group(short_help="Handle sequestered organization")
def backend_sequester_cmd() -> None:
    pass


backend_sequester_cmd.add_command(create_service, "create_service")
backend_sequester_cmd.add_command(list_services, "list_services")
backend_sequester_cmd.add_command(update_service, "update_service")
backend_sequester_cmd.add_command(export_realm, "export_realm")
backend_sequester_cmd.add_command(extract_realm_export, "extract_realm_export")
backend_sequester_cmd.add_command(generate_service_certificate, "generate_service_certificate")
backend_sequester_cmd.add_command(import_service_certificate, "import_service_certificate")


@click.group()
def backend_cmd_group() -> None:
    pass


backend_cmd_group.add_command(run_cmd, "run")
backend_cmd_group.add_command(migrate, "migrate")
backend_cmd_group.add_command(human_accesses, "human_accesses")
backend_cmd_group.add_command(backend_sequester_cmd, "sequester")

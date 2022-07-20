# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

import click

from parsec.backend.cli.run import run_cmd
from parsec.backend.cli.migration import migrate
from parsec.backend.cli.sequester import (
    create_service,
    list_services,
    update_service,
    human_accesses,
)


__all__ = ("backend_cmd",)


@click.group()
def backend_cmd():
    pass


backend_cmd.add_command(run_cmd, "run")
backend_cmd.add_command(migrate, "migrate")
backend_cmd.add_command(create_service, "create_service")
backend_cmd.add_command(list_services, "list_services")
backend_cmd.add_command(update_service, "update_service")
backend_cmd.add_command(human_accesses, "human_accesses")

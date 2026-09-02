# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import click

from parsec.cli.options import version_option

from . import list_organization


@click.group(name="tasks", short_help="Server tasks collections")
@version_option
def server_tasks_cmd_group() -> None:
    pass


server_tasks_cmd_group.add_command(list_organization.cmd)

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any

import click

from parsec.api.protocol import OrganizationID
from parsec.api.rest import organization_config_rep_serializer
from parsec.cli_utils import cli_exception_handler
from parsec.core.backend_connection.transport import http_request
from parsec.core.cli.utils import cli_command_base_options
from parsec.core.types import BackendAddr
from parsec.utils import trio_run


async def _status_organization(
    organization_id: OrganizationID, backend_addr: BackendAddr, administration_token: str
) -> None:
    url = backend_addr.to_http_domain_url(f"/administration/organizations/{organization_id.str}")

    rep_data = await http_request(
        url=url, method="GET", headers={"authorization": f"Bearer {administration_token}"}
    )

    cooked_rep_data = organization_config_rep_serializer.loads(rep_data)
    for key, value in cooked_rep_data.items():
        click.echo(f"{key}: {value}")


@click.command(short_help="get organization status")
@click.argument("organization_id", required=True, type=OrganizationID)
@click.option("--addr", "-B", required=True, type=BackendAddr.from_url, envvar="PARSEC_ADDR")
@click.option("--administration-token", "-T", required=True, envvar="PARSEC_ADMINISTRATION_TOKEN")
@cli_command_base_options
def status_organization(
    organization_id: OrganizationID,
    addr: BackendAddr,
    administration_token: str,
    debug: bool,
    **kwargs: Any,
) -> None:
    with cli_exception_handler(debug):
        trio_run(_status_organization, organization_id, addr, administration_token)

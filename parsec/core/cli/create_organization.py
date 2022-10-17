# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations
from typing import Any

import click

from parsec.utils import trio_run
from parsec.api.protocol import OrganizationID
from parsec.api.rest import organization_create_req_serializer, organization_create_rep_serializer
from parsec.cli_utils import spinner, cli_exception_handler
from parsec.core.types import BackendAddr, BackendOrganizationBootstrapAddr
from parsec.core.backend_connection.transport import http_request
from parsec.core.cli.utils import cli_command_base_options


async def create_organization_req(
    organization_id: OrganizationID, backend_addr: BackendAddr, administration_token: str
) -> str:
    url = backend_addr.to_http_domain_url("/administration/organizations")
    data = organization_create_req_serializer.dumps({"organization_id": organization_id})

    rep_data = await http_request(
        url=url,
        method="POST",
        headers={"authorization": f"Bearer {administration_token}"},
        data=data,
    )

    cooked_rep_data = organization_create_rep_serializer.loads(rep_data)
    return cooked_rep_data["bootstrap_token"]


async def _create_organization(
    organization_id: OrganizationID, backend_addr: BackendAddr, administration_token: str
) -> None:
    async with spinner("Creating organization in backend"):
        bootstrap_token = await create_organization_req(
            organization_id, backend_addr, administration_token
        )

    organization_addr = BackendOrganizationBootstrapAddr.build(
        backend_addr, organization_id, bootstrap_token
    )
    organization_addr_display = click.style(organization_addr.to_url(), fg="yellow")
    click.echo(f"Bootstrap organization url: {organization_addr_display}")


@click.command(short_help="create new organization")
@click.argument("organization_id", required=True, type=OrganizationID)
@click.option("--addr", "-B", required=True, type=BackendAddr.from_url, envvar="PARSEC_ADDR")
@click.option("--administration-token", "-T", required=True, envvar="PARSEC_ADMINISTRATION_TOKEN")
@cli_command_base_options
def create_organization(
    organization_id: OrganizationID,
    addr: BackendAddr,
    administration_token: str,
    debug: bool,
    **kwargs: Any,
) -> None:
    with cli_exception_handler(debug):
        trio_run(_create_organization, organization_id, addr, administration_token)

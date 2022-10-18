# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import re
import click
from click.exceptions import BadArgumentUsage

from parsec.utils import DateTime, trio_run
from parsec.api.protocol import OrganizationID
from parsec.api.rest import organization_stats_rep_serializer
from parsec.cli_utils import cli_exception_handler
from parsec.core.types import BackendAddr
from parsec.core.backend_connection.transport import http_request
from parsec.core.cli.utils import cli_command_base_options


def _date_from_str(input: str) -> DateTime:
    regex = re.compile(r"(\d{4})[-/](\d{1,2})[-/](\d{1,2})")
    matched = regex.match(input)

    if not matched:
        raise BadArgumentUsage("Provided date is not a valid timestamp")

    return DateTime(
        int(matched.group(1)),
        int(matched.group(2)),
        int(matched.group(3)),
        hour=0,
        minute=0,
        second=0,
    )


async def _stats_organization(
    organization_id: OrganizationID, backend_addr: BackendAddr, administration_token: str
) -> None:
    url = backend_addr.to_http_domain_url(
        f"/administration/organizations/{organization_id.str}/stats"
    )

    rep_data = await http_request(
        url=url, method="GET", headers={"authorization": f"Bearer {administration_token}"}
    )

    cooked_rep_data = organization_stats_rep_serializer.loads(rep_data)
    for key, value in cooked_rep_data.items():
        click.echo(f"{key}: {value}")


@click.command(short_help="get data&user statistics on organization")
@click.argument("organization_id", required=True, type=OrganizationID)
@click.option("--addr", "-B", required=True, type=BackendAddr.from_url, envvar="PARSEC_ADDR")
@click.option("--administration-token", "-T", required=True, envvar="PARSEC_ADMINISTRATION_TOKEN")
@cli_command_base_options
def stats_organization(
    organization_id: OrganizationID,
    addr: BackendAddr,
    administration_token: str,
    debug: bool,
    **kwargs,
) -> None:
    with cli_exception_handler(debug):
        trio_run(_stats_organization, organization_id, addr, administration_token)


@click.command(short_help="Get a per-organization report of monthly usage")
@click.option("--addr", "-B", required=True, type=BackendAddr.from_url, envvar="PARSEC_ADDR")
@click.option("--admin-token", "-T", required=True, envvar="PARSEC_ADMINISTRATION_TOKEN")
@click.option("--date-from", required=True, type=_date_from_str)
@click.option("--date-to", required=True, type=_date_from_str)
@click.option("--output", required=True)
@click.option("--format", default="csv")
@cli_command_base_options
def stats_server(
    addr: BackendAddr,
    admin_token: str,
    date_from: DateTime,
    date_to: DateTime,
    output: str,
    format: str,
    **kwargs,
) -> None:
    pass

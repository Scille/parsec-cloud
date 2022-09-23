# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Optional
import click

from parsec.utils import DateTime, trio_run
from parsec.api.protocol import OrganizationID
from parsec.api.rest import organization_stats_rep_serializer
from parsec.cli_utils import cli_exception_handler
from parsec.core.types import BackendAddr
from parsec.core.backend_connection.transport import http_request
from parsec.core.cli.utils import cli_command_base_options


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


async def _stats_server(
    backend_addr: BackendAddr,
    administration_token: str,
    from_date: DateTime,
    to_date: DateTime,
    output: Optional[str],
    format: str,
) -> None:
    url = backend_addr.to_http_domain_url(
        f"/administration/stats?format={format}&from={from_date.to_rfc3339()}&to={to_date.to_rfc3339()}"
    )
    rep = await http_request(
        url=url, method="GET", headers={"authorization": f"Bearer {administration_token}"}
    )
    result_str = rep.decode()

    if output is not None:
        with open(output, "w") as f:
            f.write(result_str)
    else:
        print(result_str)


def _validate_date(ctx, param, value):
    try:
        return DateTime.from_rfc3339(value)
    except ValueError as e:
        raise click.BadParameter(f"invalid value '{e}'")


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


@click.command(short_help="Get a per-organization report of server usage within a period of time")
@click.option("--addr", "-B", required=True, type=BackendAddr.from_url, envvar="PARSEC_ADDR")
@click.option("--admin-token", "-T", required=True, envvar="PARSEC_ADMINISTRATION_TOKEN")
@click.option(
    "--date-from",
    required=True,
    type=click.UNPROCESSED,
    callback=_validate_date,
    help="A RFC 3339 compliant timestamp eg. (2020-12-09 16:09:53+00:00).",
)
@click.option(
    "--date-to",
    default=DateTime.now().to_rfc3339(),
    type=click.UNPROCESSED,
    callback=_validate_date,
    help="A RFC 3339 compliant timestamp eg. (2020-12-09 16:09:53+00:00)",
)
@click.option("--output", type=str)
@click.option("--format", default="json")
@cli_command_base_options
def stats_server(
    addr: BackendAddr,
    admin_token: str,
    date_from: DateTime,
    date_to: DateTime,
    output: str,
    format: str,
    debug: bool,
    **kwargs,
) -> None:
    with cli_exception_handler(debug):
        trio_run(_stats_server, addr, admin_token, date_from, date_to, output, format)

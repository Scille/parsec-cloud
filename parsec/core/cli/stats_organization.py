# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import trio
import click
from urllib.request import urlopen, Request

from parsec.utils import trio_run
from parsec.api.protocol import OrganizationID
from parsec.api.rest import organization_stats_rep_serializer
from parsec.cli_utils import cli_exception_handler
from parsec.core.types import BackendAddr
from parsec.core.cli.utils import cli_command_base_options


async def _stats_organization(organization_id, backend_addr, administration_token):
    url = backend_addr.to_http_domain_url(f"/administration/organizations/{organization_id}/stats")

    def _do_req():
        req = Request(
            url=url, method="GET", headers={"authorization": f"Bearer {administration_token}"}
        )
        with urlopen(req) as rep:
            return rep.read()

    rep_data = await trio.to_thread.run_sync(_do_req)

    cooked_rep_data = organization_stats_rep_serializer.loads(rep_data)
    for key, value in cooked_rep_data.items():
        click.echo(f"{key}: {value}")


@click.command(short_help="get data&user statistics on organization")
@click.argument("organization_id", required=True, type=OrganizationID)
@click.option("--addr", "-B", required=True, type=BackendAddr.from_url, envvar="PARSEC_ADDR")
@click.option("--administration-token", "-T", required=True, envvar="PARSEC_ADMINISTRATION_TOKEN")
@cli_command_base_options
def stats_organization(organization_id, addr, administration_token, debug, **kwargs):
    with cli_exception_handler(debug):
        trio_run(_stats_organization, organization_id, addr, administration_token)

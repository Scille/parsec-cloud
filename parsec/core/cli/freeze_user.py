# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import TypedDict, cast

import click

from parsec.api.protocol import OrganizationID, UserID
from parsec.api.rest import (
    freeze_user_rep_serializer,
    freeze_user_req_serializer,
    user_list_rep_serializer,
)
from parsec.cli_utils import cli_exception_handler
from parsec.core.backend_connection.transport import http_request
from parsec.core.cli.utils import cli_command_base_options
from parsec.core.types import BackendAddr
from parsec.utils import trio_run


class User(TypedDict):
    user_name: str
    user_id: UserID
    user_email: str
    frozen: bool


def show_user(user: User, prefix: str = "") -> None:
    user_name = click.style(user["user_name"], fg="yellow")
    user_id = click.style(user["user_id"].str, fg="yellow")
    user_email = click.style(user["user_email"], fg="yellow")
    frozen = user["frozen"]
    frozen = click.style("Frozen", fg="red") if frozen else click.style("Not frozen", fg="yellow")
    empty_prefix = " " * len(prefix)
    click.echo(f"{prefix}{user_name} <{user_email}> ")
    click.echo(f"{empty_prefix}- Parsec ID: {user_id} ")
    click.echo(f"{empty_prefix}- Status: {frozen} ")


async def _list_users(
    organization_id: OrganizationID, backend_addr: BackendAddr, administration_token: str
) -> None:
    url = backend_addr.to_http_domain_url(
        f"/administration/organizations/{organization_id.str}/users"
    )

    rep_data = await http_request(
        url=url, method="GET", headers={"authorization": f"Bearer {administration_token}"}
    )

    cooked_rep_data = user_list_rep_serializer.loads(rep_data)
    users = cooked_rep_data["users"]
    for user in users:
        show_user(cast(User, user), prefix="• ")
        click.echo("")
    number_of_users = click.style(len(users), fg="yellow")
    click.echo(f"Total users: {number_of_users}")


@click.command(short_help="List users of an organization")
@click.argument("organization_id", required=True, type=OrganizationID)
@click.option("--addr", "-B", required=True, type=BackendAddr.from_url, envvar="PARSEC_ADDR")
@click.option("--administration-token", "-T", required=True, envvar="PARSEC_ADMINISTRATION_TOKEN")
@cli_command_base_options
def list_users(
    organization_id: OrganizationID,
    addr: BackendAddr,
    administration_token: str,
    debug: bool,
    **kwargs: object,
) -> None:
    """List users of an organization using the administration API.

    The following information is returned for each user:

    \b
    • USER_NAME <USER_EMAIL>
      - Parsec ID: USER_ID
      - Status: Frozen/Not frozen
    """
    with cli_exception_handler(debug):
        trio_run(_list_users, organization_id, addr, administration_token)


async def _freeze_user(
    organization_id: OrganizationID,
    backend_addr: BackendAddr,
    administration_token: str,
    user: str,
    freeze: bool,
) -> None:
    url = backend_addr.to_http_domain_url(
        f"/administration/organizations/{organization_id.str}/users/freeze"
    )
    if "@" in user:
        json = {"user_email": user, "frozen": freeze}
    else:
        json = {"user_id": user, "frozen": freeze}

    data = freeze_user_req_serializer.dumps(json)
    rep_data = await http_request(
        url=url,
        method="POST",
        headers={"authorization": f"Bearer {administration_token}"},
        data=data,
    )

    cooked_rep_data = freeze_user_rep_serializer.loads(rep_data)
    show_user(cast(User, cooked_rep_data))


@click.command(short_help="Freeze or unfreeze a specific user")
@click.argument("organization_id", required=True, type=OrganizationID)
@click.argument("user", required=True, type=str)
@click.option("--freeze/--unfreeze", "-F/-U", required=False, default=True)
@click.option("--addr", "-B", required=True, type=BackendAddr.from_url, envvar="PARSEC_ADDR")
@click.option("--administration-token", "-T", required=True, envvar="PARSEC_ADMINISTRATION_TOKEN")
@cli_command_base_options
def freeze_user(
    organization_id: OrganizationID,
    addr: BackendAddr,
    administration_token: str,
    user: str,
    freeze: bool,
    debug: bool,
    **kwargs: object,
) -> None:
    """Freeze or unfreeze a specific user using the administration API.

    USER can be either a Parsec ID or an email address.
    """
    with cli_exception_handler(debug):
        trio_run(_freeze_user, organization_id, addr, administration_token, user, freeze)

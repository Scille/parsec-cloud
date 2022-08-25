# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from quart import Response, Blueprint, abort, request, g

from parsec.serde import SerdePackingError, packb, unpackb
from parsec.api.protocol import OrganizationID
from parsec.backend.app import BackendApp
from parsec.backend.client_context import AnonymousClientContext, ClientType
from parsec.backend.organization import OrganizationNotFoundError, OrganizationAlreadyExistsError


def rpc_msgpack_rep(data: dict) -> Response:
    return Response(
        response=packb(data),
        # Unlike REST, RPC doesn't use status to encode operational result
        status=200,
        content_type="application/msgpack",
    )


rpc_bp = Blueprint("anonymous_api", __name__)


@rpc_bp.route("/anonymous/<raw_organization_id>", methods=["GET", "POST"])
async def anonymous_api(raw_organization_id: str):
    backend: BackendApp = g.backend
    # Check whether the organization exists
    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        abort(404)
    try:
        await backend.organization.get(organization_id)
    except OrganizationNotFoundError:
        organization_exists = False
    else:
        organization_exists = True

    # Reply to GET
    if request.method == "GET":
        if not organization_exists:
            abort(404)
        else:
            return rpc_msgpack_rep({})

    # Reply early to POST when the organization doesn't exists
    if not organization_exists and not backend.config.organization_spontaneous_bootstrap:
        abort(404)

    # Get and unpack the body
    body: bytes = await request.get_data(cache=False)
    try:
        msg = unpackb(body)
    except SerdePackingError:
        return rpc_msgpack_rep({"status": "invalid_msg_format"})

    # Lazy creation of the organization if necessary
    cmd = msg.get("cmd")
    if cmd == "organization_bootstrap" and not organization_exists:
        assert backend.config.organization_spontaneous_bootstrap
        try:
            await backend.organization.create(id=organization_id, bootstrap_token="")
        except OrganizationAlreadyExistsError:
            pass

    # Retrieve command
    client_ctx = AnonymousClientContext(organization_id)
    try:
        if not isinstance(cmd, str):
            raise KeyError()
        cmd_func = backend.apis[ClientType.ANONYMOUS][cmd]
    except KeyError:
        return rpc_msgpack_rep({"status": "unknown_command"})

    # Run command
    rep = await cmd_func(client_ctx, msg)
    return rpc_msgpack_rep(rep)

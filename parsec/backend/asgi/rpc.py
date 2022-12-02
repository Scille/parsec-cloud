# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from base64 import b64decode
from typing import NoReturn, Tuple

from nacl.exceptions import CryptoError
from quart import Blueprint, Response, current_app, g, request

from parsec._parsec import ClientType, DateTime
from parsec.api.protocol import (
    DeviceID,
    IncompatibleAPIVersionsError,
    OrganizationID,
    settle_compatible_versions,
)
from parsec.api.version import API_V2_VERSION, API_V3_VERSION, ApiVersion
from parsec.backend.app import BackendApp
from parsec.backend.client_context import AnonymousClientContext, AuthenticatedClientContext
from parsec.backend.organization import (
    Organization,
    OrganizationAlreadyExistsError,
    OrganizationNotFoundError,
)
from parsec.backend.user import UserNotFoundError
from parsec.backend.user_type import Device, User
from parsec.serde import SerdePackingError, packb, unpackb


CONTENT_TYPE_MSGPACK = "application/msgpack"
AUTHORIZATION_PARSEC_ED25519 = "PARSEC-SIGN-ED25519"
SUPPORTED_API_VERSIONS = (
    API_V2_VERSION,
    API_V3_VERSION,
)


rpc_bp = Blueprint("anonymous_api", __name__)


def _rpc_msgpack_rep(data: dict[str, object], api_version: ApiVersion) -> Response:
    return Response(
        response=packb(data),
        # Unlike REST, RPC doesn't use status to encode operational result
        status=200,
        content_type=CONTENT_TYPE_MSGPACK,
        headers={"Api-Version": str(api_version)},
    )


# The HTTP RPC API is divided into two layers: handshake and actual command processing
# The command processing is based on msgpack and is common with the Websocket API.
#
# On the other hand, the handshake part is specific to how works HTTP (i.e. the
# handshake is done for each query in HTTP using headers, unlike the Websocket
# handshake which is done once by sending challenge/reply messages).
#
# So the RPC API handshake cannot return a body in msgpack given at this level
# we are not settled on what should be used yet (who knows ! maybe in the future
# we will use another serialization format for the command processing).
# Instead we rely on the following HTTP status code:
# - 404: Organization non found or invalid organization ID
# - 401: Bad authentication info (bad Author/Signature/Timestamp headers)
# - 415: Bad content-type
# - 422: Unsupported API version


def _handshake_abort(status_code: int, api_version: ApiVersion) -> NoReturn:
    current_app.aborter(
        Response(response="", status=status_code, headers={"Api-Version": str(api_version)})
    )


async def _do_handshake(
    raw_organization_id: str,
    backend: BackendApp,
    allow_missing_organization: bool,
    check_authentication: bool,
) -> Tuple[ApiVersion, OrganizationID, Organization | None, User | None, Device | None]:
    # The anonymous RPC API existed before the `Api-Version`/`Content-Type` fields
    # check where introduced, hence we have this workaround to provide backward compatibility
    # TODO: remove me once Parsec 2.11.1 is deprecated
    request.headers.setdefault("Api-Version", "3.0")
    # `urllib.request.Request` uses this `Content-Type` by default if none are provided
    # (and, you guessed it, we used to not provide one...)
    # TODO: remove me once Parsec 2.11.1 is deprecated
    if request.headers.get("Content-Type") == "application/x-www-form-urlencoded":
        request.headers["Content-Type"] = CONTENT_TYPE_MSGPACK

    # 1) Check API version
    # Parse `Api-version` from the HTTP Header and return the version implemented
    # by the server that is compatible with the client.
    try:
        client_api_version = ApiVersion.from_str(request.headers.get("Api-Version", ""))
        api_version, _ = settle_compatible_versions(SUPPORTED_API_VERSIONS, [client_api_version])
    except (ValueError, IncompatibleAPIVersionsError):
        supported_api_versions = ";".join(
            str(api_version) for api_version in SUPPORTED_API_VERSIONS
        )
        current_app.aborter(
            Response(
                response="", status=422, headers={"Supported-Api-Versions": supported_api_versions}
            )
        )

    # From now on the version is settled, our reply must have the `Api-Version` header

    # 2) Check organization
    # Check whether the organization exists
    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        _handshake_abort(404, api_version=api_version)
    organization: Organization | None
    try:
        organization = await backend.organization.get(organization_id)
    except OrganizationNotFoundError:
        if not allow_missing_organization:
            _handshake_abort(404, api_version=api_version)
        else:
            organization = None
    else:
        if organization.is_expired:
            current_app.aborter(_rpc_msgpack_rep({"status": "expired_organization"}, api_version))

    # 3) Check Content-Type
    if request.headers.get("Content-Type") != CONTENT_TYPE_MSGPACK:
        _handshake_abort(415, api_version=api_version)

    # 4) Check authentication
    if not check_authentication:
        user = None
        device = None

    else:
        try:
            authorization_method = request.headers["Authorization"]
        except KeyError:
            _handshake_abort(401, api_version=api_version)

        if authorization_method != AUTHORIZATION_PARSEC_ED25519:
            _handshake_abort(401, api_version=api_version)

        try:
            raw_device_id = request.headers["Author"]
            raw_signature_b64 = request.headers["Signature"]
        except KeyError:
            _handshake_abort(401, api_version=api_version)

        try:
            signature_bytes = b64decode(raw_signature_b64)
            device_id = DeviceID(b64decode(raw_device_id).decode())
        except ValueError:
            _handshake_abort(401, api_version=api_version)

        body: bytes = await request.get_data()
        try:
            user, device = await backend.user.get_user_with_device(organization_id, device_id)
        except UserNotFoundError:
            _handshake_abort(401, api_version=api_version)
        else:
            if user.revoked_on:
                current_app.aborter(_rpc_msgpack_rep({"status": "revoked_user"}, api_version))

        try:
            device.verify_key.verify_with_signature(
                signature=signature_bytes,
                message=body,
            )
        except CryptoError:
            _handshake_abort(401, api_version=api_version)

    return api_version, organization_id, organization, user, device


@rpc_bp.route("/anonymous/<raw_organization_id>", methods=["GET", "POST"])
async def anonymous_api(raw_organization_id: str) -> Response:
    backend: BackendApp = g.backend

    allow_missing_organization = (
        request.method == "POST" and backend.config.organization_spontaneous_bootstrap
    )

    api_version, organization_id, organization, _, _ = await _do_handshake(
        raw_organization_id=raw_organization_id,
        backend=backend,
        allow_missing_organization=allow_missing_organization,
        check_authentication=False,
    )

    # Reply to GET
    if request.method == "GET":
        return _rpc_msgpack_rep({}, api_version)

    body: bytes = await request.get_data(cache=False)
    try:
        msg = unpackb(body)
    except SerdePackingError:
        return _rpc_msgpack_rep({"status": "invalid_msg_format"}, api_version)

    # Lazy creation of the organization if necessary
    cmd = msg.get("cmd")
    if cmd == "organization_bootstrap" and not organization:
        assert backend.config.organization_spontaneous_bootstrap
        try:
            await backend.organization.create(
                id=organization_id, bootstrap_token="", created_on=DateTime.now()
            )
        except OrganizationAlreadyExistsError:
            pass

    # Retrieve command
    client_ctx = AnonymousClientContext(organization_id)
    if not isinstance(cmd, str):
        return _rpc_msgpack_rep({"status": "unknown_command"}, api_version)

    try:
        cmd_func = backend.apis[ClientType.ANONYMOUS][cmd]
    except KeyError:
        return _rpc_msgpack_rep({"status": "unknown_command"}, api_version)

    # Run command
    rep = await cmd_func(client_ctx, msg)
    return _rpc_msgpack_rep(rep, api_version)


@rpc_bp.route("/authenticated/<raw_organization_id>", methods=["POST"])
async def authenticated_api(raw_organization_id: str) -> Response:
    backend: BackendApp = g.backend

    api_version, organization_id, _, user, device = await _do_handshake(
        raw_organization_id=raw_organization_id,
        backend=backend,
        allow_missing_organization=False,
        check_authentication=True,
    )
    assert isinstance(user, User)
    assert isinstance(device, Device)

    # Unpack verified body
    body: bytes = await request.get_data(cache=False)
    try:
        msg = unpackb(body)
    except SerdePackingError:
        return _rpc_msgpack_rep({"status": "invalid_msg_format"}, api_version)

    cmd = msg.get("cmd")
    if not isinstance(cmd, str):
        return _rpc_msgpack_rep({"status": "unknown_command"}, api_version)

    try:
        cmd_func = backend.apis[ClientType.AUTHENTICATED][cmd]
    except KeyError:
        return _rpc_msgpack_rep({"status": "unknown_command"}, api_version)

    client_ctx = AuthenticatedClientContext(
        api_version=api_version,
        organization_id=organization_id,
        device_id=device.device_id,
        human_handle=user.human_handle,
        device_label=device.device_label,
        profile=user.profile,
        public_key=user.public_key,
        verify_key=device.verify_key,
    )

    cmd_rep = await cmd_func(client_ctx, msg)
    return _rpc_msgpack_rep(cmd_rep, api_version)

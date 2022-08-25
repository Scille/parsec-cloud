# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import binascii
import base64
from typing import Tuple
from quart import Response, Blueprint, abort, request, g
from quart.datastructures import Headers
from nacl.exceptions import CryptoError

from parsec.api.protocol.handshake import ServerHandshake
from parsec.api.protocol.types import DeviceID
from parsec.api.version import ApiVersion
from parsec.backend.asgi.error import (
    AuthenticationProtocolError,
    CannotVerifySignatureError,
    DeviceNotFoundError,
    InvalidHeaderValueError,
    HttpProtocolError,
    UserIsRevokedError,
)
from parsec.backend.user import UserNotFoundError
from parsec.backend.user_type import Device, User

from parsec.serde import SerdePackingError, packb, unpackb
from parsec.api.protocol import (
    OrganizationID,
    IncompatibleAPIVersionsError,
    settle_compatible_versions,
)
from parsec.backend.app import BackendApp
from parsec.backend.client_context import (
    AnonymousClientContext,
    AuthenticatedClientContext,
    ClientType,
)
from parsec.backend.organization import (
    Organization,
    OrganizationNotFoundError,
    OrganizationAlreadyExistsError,
)


CONTENT_TYPE_MSGPACK = "application/msgpack"
AUTHORIZATION_PARSEC_ED25519 = "PARSEC-SIGN-ED25519"


def rpc_msgpack_rep(data: dict) -> Response:
    return Response(
        response=packb(data),
        # Unlike REST, RPC doesn't use status to encode operational result
        status=200,
        content_type=CONTENT_TYPE_MSGPACK,
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
    if not isinstance(cmd, str):
        return rpc_msgpack_rep({"status": "unknown_command"})

    try:
        cmd_func = backend.apis[ClientType.ANONYMOUS][cmd]
    except KeyError:
        return rpc_msgpack_rep({"status": "unknown_command"})

    # Run command
    rep = await cmd_func(client_ctx, msg)
    return rpc_msgpack_rep(rep)


@rpc_bp.route("/authenticated/<raw_organization_id>", methods=["POST"])
async def authenticated_api(raw_organization_id: str):
    backend: BackendApp = g.backend

    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        abort(404)

    try:
        organization = await backend.organization.get(organization_id)
    except OrganizationNotFoundError:
        abort(400)
    else:
        if organization.is_expired:
            abort(400)

    body: bytes = await request.get_data(cache=False)
    try:
        user, device = await verify_auth_request(body, request.headers, organization, backend)
    except (HttpProtocolError, DeviceNotFoundError):
        abort(400)
    except AuthenticationProtocolError:
        abort(401)

    api_version = get_api_version(request.headers)

    client_ctx = build_client_context(organization, user, device, api_version)

    # Unpack the verified body
    if request.headers["Content-Type"] != CONTENT_TYPE_MSGPACK:
        abort(400)
    try:
        msg = unpackb(body)
    except SerdePackingError:
        return rpc_msgpack_rep({"status": "invalid_msg_format"})

    cmd = msg.get("cmd")
    if not isinstance(cmd, str):
        return rpc_msgpack_rep({"status": "unknown_command"})

    try:
        cmd_func = backend.apis[ClientType.AUTHENTICATED][cmd]
    except KeyError:
        return rpc_msgpack_rep({"status": "unknown_command"})

    cmd_rep = await cmd_func(client_ctx, msg)
    rep = rpc_msgpack_rep(cmd_rep)
    rep.headers.add("Api-Version", str(client_ctx.api_version))
    return rep


def get_api_version(headers: Headers) -> ApiVersion:
    """Parse `Api-version` from the HTTP Header and return a compatible `ApiVersion` that can be used by the server & client"""

    try:
        raw_api_version = request.headers["Api-Version"]
        client_api_version = ApiVersion.from_str(raw_api_version)
        settle_api_version, _client_api_version = settle_compatible_versions(
            ServerHandshake.SUPPORTED_API_VERSIONS, [client_api_version]
        )
    except (ValueError, KeyError):
        abort(400)
    except IncompatibleAPIVersionsError:
        abort(400)
    return settle_api_version


async def verify_auth_request(
    body: bytes, headers: Headers, organization: Organization, backend: BackendApp
) -> Tuple[User, Device]:

    try:
        authorization_method = headers["Authorization"]
    except KeyError:
        abort(400)

    if authorization_method == AUTHORIZATION_PARSEC_ED25519:
        try:
            raw_device_id = headers["Author"]
            raw_timestamp = headers["Timestamp"].encode()
            raw_signature_b64 = headers["Signature"]
        except KeyError:
            abort(400)

        try:
            signature_bytes = base64.b64decode(raw_signature_b64)
        except binascii.Error:
            abort(400)

        try:
            device_id = DeviceID(base64.b64decode(raw_device_id).decode())
        except binascii.Error:
            abort(400)
        except ValueError:
            abort(400)
        return await verify_auth_request_ed25519(
            body=body,
            device_id=device_id,
            raw_timestamp=raw_timestamp,
            signature=signature_bytes,
            organization=organization,
            backend=backend,
        )
    else:
        raise InvalidHeaderValueError("Authorization", authorization_method)


async def verify_auth_request_ed25519(
    body: bytes,
    device_id: DeviceID,
    raw_timestamp: bytes,
    signature: bytes,
    organization: Organization,
    backend: BackendApp,
) -> Tuple[User, Device]:
    try:
        user, device = await backend.user.get_user_with_device(
            organization.organization_id, device_id
        )
    except UserNotFoundError:
        raise DeviceNotFoundError(organization.organization_id, device_id)
    else:
        if user.revoked_on:
            raise UserIsRevokedError(organization.organization_id, user.user_id)

    verify_key = device.verify_key

    try:
        # when the verify failed, `verify()` raise the exception `BadSignatureError`
        verify_key.verify_with_signature(
            signature=signature,
            message=base64.b64encode(device_id.str.encode()) + raw_timestamp + body,
        )
    except CryptoError as e:
        raise CannotVerifySignatureError(e)

    return (user, device)


def build_client_context(
    organization: Organization, user: User, device: Device, settle_api_version: ApiVersion
) -> AuthenticatedClientContext:
    client_ctx = AuthenticatedClientContext(
        api_version=settle_api_version,
        organization_id=organization.organization_id,
        device_id=device.device_id,
        human_handle=user.human_handle,
        device_label=device.device_label,
        profile=user.profile,
        public_key=user.public_key,
        verify_key=device.verify_key,
    )

    return client_ctx

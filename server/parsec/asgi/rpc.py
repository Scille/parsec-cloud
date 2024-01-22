# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from base64 import b64decode
from dataclasses import dataclass
from enum import Enum
from typing import (
    Annotated,
    Any,
    AsyncGenerator,
    NoReturn,
    Sequence,
    assert_never,
)

import anyio
import structlog
from fastapi import APIRouter, HTTPException, Request, Response
from fastapi.datastructures import Headers
from fastapi.responses import StreamingResponse
from pydantic import (
    GetPydanticSchema,
    TypeAdapter,
)
from pydantic_core import core_schema

from parsec._parsec import (
    ApiVersion,
    DateTime,
    DeviceID,
    InvitationToken,
    OrganizationID,
    anonymous_cmds,
    authenticated_cmds,
    invited_cmds,
)
from parsec.backend import Backend
from parsec.client_context import (
    AnonymousClientContext,
    AuthenticatedClientContext,
    InvitedClientContext,
)
from parsec.components.auth import (
    AnonymousAuthInfo,
    AuthAnonymousAuthBadOutcome,
    AuthAuthenticatedAuthBadOutcome,
    AuthenticatedAuthInfo,
    AuthInvitedAuthBadOutcome,
    InvitedAuthInfo,
)
from parsec.components.events import SseAPiEventsListenBadOutcome

logger: structlog.stdlib.BoundLogger = structlog.get_logger()


CONTENT_TYPE_MSGPACK = "application/msgpack"
ACCEPT_TYPE_SSE = "text/event-stream"
AUTHORIZATION_PARSEC_ED25519 = "PARSEC-SIGN-ED25519"
SUPPORTED_API_VERSIONS = (ApiVersion.API_V4_VERSION,)
# Max size for HTTP body, 1Mo seems plenty given our API never upload big chunk of data
# (biggest request should be the `block_create` command with typically ~512Ko of data)
MAX_CONTENT_LENGTH = 1 * 1024**2


AUTHENTICATED_CMDS_LOAD_FN = {
    int(v_version[1:]): getattr(authenticated_cmds, v_version).AnyCmdReq.load
    for v_version in dir(authenticated_cmds)
    if v_version.startswith("v")
}
INVITED_CMDS_LOAD_FN = {
    int(v_version[1:]): getattr(invited_cmds, v_version).AnyCmdReq.load
    for v_version in dir(invited_cmds)
    if v_version.startswith("v")
}
ANONYMOUS_CMDS_LOAD_FN = {
    int(v_version[1:]): getattr(anonymous_cmds, v_version).AnyCmdReq.load
    for v_version in dir(anonymous_cmds)
    if v_version.startswith("v")
}
ANONYMOUS_ORGANIZATION_BOOTSTRAP_CMD_ALL_API_REQS = tuple(
    getattr(anonymous_cmds, v_version).organization_bootstrap.Req
    for v_version in dir(anonymous_cmds)
    if v_version.startswith("v")
)


rpc_router = APIRouter()


OrganizationIDField = TypeAdapter(
    Annotated[
        OrganizationID,
        GetPydanticSchema(
            lambda tp, handler: core_schema.json_or_python_schema(
                json_schema=core_schema.chain_schema(
                    [
                        core_schema.str_schema(),
                        core_schema.no_info_plain_validator_function(OrganizationID),
                    ]
                ),
                python_schema=core_schema.chain_schema(
                    [
                        core_schema.is_instance_schema(OrganizationID),
                        core_schema.no_info_plain_validator_function(OrganizationID),
                    ]
                ),
                serialization=core_schema.plain_serializer_function_ser_schema(lambda x: x.str),
            )
        ),
    ]
)


async def _rpc_get_body_with_limit_check(request: Request) -> bytes:
    try:
        content_length = int(request.headers["Content-Length"])

    except ValueError:
        raise HTTPException(status_code=413)

    except KeyError:
        # Header missing, we must be en chunk-encoding mode
        content_length = MAX_CONTENT_LENGTH

    else:
        if content_length > MAX_CONTENT_LENGTH:
            raise HTTPException(status_code=413)

    chunks = []
    async for chunk in request.stream():
        chunks.append(chunk)
        if sum(len(c) for c in chunks) > content_length:
            raise HTTPException(status_code=413)

    return b"".join(chunks)


def _rpc_rep(rep: Any, api_version: ApiVersion) -> Response:
    return Response(
        content=rep.dump(),
        # Unlike REST, RPC doesn't use status to encode operational result
        status_code=200,
        headers={"Api-Version": str(api_version), "Content-Type": CONTENT_TYPE_MSGPACK},
    )


class IncompatibleAPIVersionsError(Exception):
    def __init__(
        self,
        backend_versions: Sequence[ApiVersion],
        client_versions: Sequence[ApiVersion] = [],
    ):
        self.client_versions = client_versions
        self.backend_versions = backend_versions
        client_versions_str = "{" + ", ".join(map(str, client_versions)) + "}"
        backend_versions_str = "{" + ", ".join(map(str, backend_versions)) + "}"
        self.message = (
            f"No overlap between client API versions {client_versions_str} "
            f"and backend API versions {backend_versions_str}"
        )

    def __str__(self) -> str:
        return self.message


def settle_compatible_versions(
    backend_versions: Sequence[ApiVersion], client_versions: Sequence[ApiVersion]
) -> tuple[ApiVersion, ApiVersion]:
    """
    Try to find compatible API version between the server and the client.

    raise an exception if no compatible version is found
    """
    # Try to use the newest version first
    for client_version in reversed(sorted(client_versions)):
        # No need to compare `revision` because only `version` field breaks compatibility
        compatible_backend_versions = filter(
            lambda bv: client_version.version == bv.version, backend_versions
        )
        backend_version = next(compatible_backend_versions, None)

        if backend_version:
            return backend_version, client_version
    raise IncompatibleAPIVersionsError(backend_versions, client_versions)


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
# - 401: Bad authentication info (bad Author/Signature/Timestamp headers)
# - 404: Organization / Invitation not found or invalid organization ID
# - 406: Bad accept type (for the SSE events route)
# - 410: Invitation already deleted / used
# - 415: Bad content-type, body is not a valid message or unknown command
# - 422: Unsupported API version
# - 460: Organization is expired
# - 461: User is revoked


class CustomHttpStatus(Enum):
    BadAuthenticationInfo = 401
    OrganizationOrInvitationInvalidOrNotFound = 404
    BadAcceptType = 406
    InvitationAlreadyUsedOrDeleted = 410
    BadContentTypeOrInvalidBodyOrUnknownCommand = 415
    UnsupportedApiVersion = 422
    OrganizationExpired = 460
    UserRevoked = 461


def _handshake_abort(status_code: int, api_version: ApiVersion) -> NoReturn:
    detail: str | None
    match status_code:
        case 422:
            detail = "Unsupported api version"
        case 460:
            detail = "Organization expired"
        case 461:
            detail = "User revoked"
        case _:
            detail = None
    raise HTTPException(
        status_code=status_code,
        headers={"Api-Version": str(api_version)},
        detail=detail,
    )


def _handshake_abort_bad_content(api_version: ApiVersion) -> NoReturn:
    _handshake_abort(
        CustomHttpStatus.BadContentTypeOrInvalidBodyOrUnknownCommand.value, api_version
    )


@dataclass
class ParsedAuthHeaders:
    organization_id: OrganizationID
    settled_api_version: ApiVersion
    client_api_version: ApiVersion
    user_agent: str
    authenticated_device_id: DeviceID | None
    authenticated_signature: bytes | None
    invited_token: InvitationToken | None
    last_event_id: str | None


def _parse_auth_headers_or_abort(
    headers: Headers,
    # TODO: Use FastAPI' path parsing to handle this once it is fixed upstream
    # (see https://github.com/tiangolo/fastapi/pull/10109)
    # Organization ID is not strictly part of the headers, but it's convenient
    # to handle it there !
    raw_organization_id: str,
    with_authenticated_headers: bool,
    with_invited_headers: bool,
    with_sse_headers: bool,
    expected_content_type: str | None,
    expected_accept_type: str | None,
) -> ParsedAuthHeaders:
    # 1) Check API version
    # Parse `Api-version` from the HTTP Header and return the version implemented
    # by the server that is compatible with the client.
    try:
        client_api_version = ApiVersion.from_str(headers.get("Api-Version", ""))
        settled_api_version, _ = settle_compatible_versions(
            SUPPORTED_API_VERSIONS, [client_api_version]
        )
    except (ValueError, IncompatibleAPIVersionsError):
        supported_api_versions = ";".join(
            str(api_version) for api_version in SUPPORTED_API_VERSIONS
        )
        raise HTTPException(
            status_code=CustomHttpStatus.UnsupportedApiVersion.value,
            headers={"Supported-Api-Versions": supported_api_versions},
        )

    # From now on the version is settled, our reply must have the `Api-Version` header

    # 2) Check organization ID
    # Check whether the organization exists
    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        _handshake_abort(
            CustomHttpStatus.OrganizationOrInvitationInvalidOrNotFound.value,
            api_version=settled_api_version,
        )

    # 3) Check User-Agent, Content-Type & Accept
    user_agent = headers.get("User-Agent", "unknown")
    if expected_content_type and headers.get("Content-Type") != expected_content_type:
        _handshake_abort_bad_content(api_version=settled_api_version)
    if expected_accept_type and headers.get("Accept") != expected_accept_type:
        _handshake_abort(CustomHttpStatus.BadAcceptType.value, api_version=settled_api_version)

    # 4) Check authenticated headers
    if not with_authenticated_headers:
        authenticated_device_id = None
        authenticated_signature = None

    else:
        try:
            authorization_method = headers["Authorization"]
        except KeyError:
            _handshake_abort(
                CustomHttpStatus.BadAuthenticationInfo.value, api_version=settled_api_version
            )

        if authorization_method != AUTHORIZATION_PARSEC_ED25519:
            _handshake_abort(
                CustomHttpStatus.BadAuthenticationInfo.value, api_version=settled_api_version
            )

        try:
            raw_device_id = headers["Author"]
            raw_signature_b64 = headers["Signature"]
        except KeyError:
            _handshake_abort(
                CustomHttpStatus.BadAuthenticationInfo.value, api_version=settled_api_version
            )

        try:
            authenticated_signature = b64decode(raw_signature_b64)
            authenticated_device_id = DeviceID(b64decode(raw_device_id).decode())
        except ValueError:
            _handshake_abort(
                CustomHttpStatus.BadAuthenticationInfo.value, api_version=settled_api_version
            )

    # 5) Check invited headers
    if not with_invited_headers:
        invited_token = None

    else:
        raw_invitation_token = headers.get("Invitation-Token", "")

        try:
            invited_token = InvitationToken.from_hex(raw_invitation_token)
        except ValueError:
            _handshake_abort_bad_content(api_version=settled_api_version)

    if not with_sse_headers:
        last_event_id = None
    else:
        last_event_id = headers.get("Last-Event-Id")

    return ParsedAuthHeaders(
        organization_id=organization_id,
        settled_api_version=settled_api_version,
        client_api_version=client_api_version,
        user_agent=user_agent,
        last_event_id=last_event_id,
        authenticated_device_id=authenticated_device_id,
        authenticated_signature=authenticated_signature,
        invited_token=invited_token,
    )


@rpc_router.get("/anonymous/{raw_organization_id}")
@rpc_router.post("/anonymous/{raw_organization_id}")
async def anonymous_api(raw_organization_id: str, request: Request) -> Response:
    backend: Backend = request.app.state.backend

    parsed = _parse_auth_headers_or_abort(
        headers=request.headers,
        raw_organization_id=raw_organization_id,
        with_authenticated_headers=False,
        with_invited_headers=False,
        with_sse_headers=False,
        expected_accept_type=None,
        expected_content_type=CONTENT_TYPE_MSGPACK,
    )

    spontaneous_bootstrap = (
        request.method == "POST" and backend.config.organization_spontaneous_bootstrap
    )

    outcome = await backend.auth.anonymous_auth(
        DateTime.now(), parsed.organization_id, spontaneous_bootstrap
    )
    match outcome:
        case AnonymousAuthInfo() as auth_info:
            pass
        case AuthAnonymousAuthBadOutcome.ORGANIZATION_EXPIRED:
            _handshake_abort(
                CustomHttpStatus.OrganizationExpired.value, api_version=parsed.settled_api_version
            )
        case AuthAnonymousAuthBadOutcome.ORGANIZATION_NOT_FOUND:
            _handshake_abort(
                CustomHttpStatus.OrganizationOrInvitationInvalidOrNotFound.value,
                api_version=parsed.settled_api_version,
            )
        case unknown:
            assert_never(unknown)

    # Handshake is done

    client_ctx = AnonymousClientContext(
        client_api_version=parsed.client_api_version,
        settled_api_version=parsed.settled_api_version,
        organization_id=auth_info.organization_id,
        organization_internal_id=auth_info.organization_internal_id,
    )

    # Reply to GET
    if request.method == "GET":
        return Response(
            status_code=200,
            headers={
                "Api-Version": str(parsed.settled_api_version),
                "Content-Type": CONTENT_TYPE_MSGPACK,
            },
        )

    body: bytes = await _rpc_get_body_with_limit_check(request)

    try:
        req = ANONYMOUS_CMDS_LOAD_FN[parsed.settled_api_version.version](body)
    except ValueError:
        _handshake_abort_bad_content(api_version=parsed.settled_api_version)

    cmd_func = backend.apis[type(req)]
    rep = await cmd_func(client_ctx, req)
    return _rpc_rep(rep, parsed.settled_api_version)


@rpc_router.post("/invited/{raw_organization_id}")
async def invited_api(raw_organization_id: str, request: Request) -> Response:
    backend: Backend = request.app.state.backend

    parsed = _parse_auth_headers_or_abort(
        headers=request.headers,
        raw_organization_id=raw_organization_id,
        with_authenticated_headers=False,
        with_invited_headers=True,
        with_sse_headers=False,
        expected_accept_type=None,
        expected_content_type=CONTENT_TYPE_MSGPACK,
    )
    assert parsed.invited_token is not None

    outcome = await backend.auth.invited_auth(
        DateTime.now(), parsed.organization_id, parsed.invited_token
    )
    match outcome:
        case InvitedAuthInfo() as auth_info:
            pass
        case AuthInvitedAuthBadOutcome.ORGANIZATION_EXPIRED:
            _handshake_abort(
                CustomHttpStatus.OrganizationExpired.value, api_version=parsed.settled_api_version
            )
        case (
            AuthInvitedAuthBadOutcome.ORGANIZATION_NOT_FOUND
            | AuthInvitedAuthBadOutcome.INVITATION_NOT_FOUND
        ):
            _handshake_abort(
                CustomHttpStatus.OrganizationOrInvitationInvalidOrNotFound.value,
                api_version=parsed.settled_api_version,
            )
        case AuthInvitedAuthBadOutcome.INVITATION_ALREADY_USED:
            _handshake_abort(
                CustomHttpStatus.InvitationAlreadyUsedOrDeleted.value,
                api_version=parsed.settled_api_version,
            )
        case unknown:
            assert_never(unknown)

    # Handshake is done

    client_ctx = InvitedClientContext(
        client_api_version=parsed.client_api_version,
        settled_api_version=parsed.settled_api_version,
        organization_id=auth_info.organization_id,
        organization_internal_id=auth_info.organization_internal_id,
        type=auth_info.type,
        token=auth_info.token,
        invitation_internal_id=auth_info.invitation_internal_id,
    )

    body: bytes = await _rpc_get_body_with_limit_check(request)

    try:
        req = INVITED_CMDS_LOAD_FN[parsed.settled_api_version.version](body)
    except ValueError:
        _handshake_abort_bad_content(api_version=parsed.settled_api_version)

    cmd_func = backend.apis[type(req)]
    rep = await cmd_func(client_ctx, req)
    return _rpc_rep(rep, parsed.settled_api_version)


@rpc_router.post("/authenticated/{raw_organization_id}")
async def authenticated_api(raw_organization_id: str, request: Request) -> Response:
    backend: Backend = request.app.state.backend

    parsed = _parse_auth_headers_or_abort(
        headers=request.headers,
        raw_organization_id=raw_organization_id,
        with_authenticated_headers=True,
        with_invited_headers=False,
        with_sse_headers=False,
        expected_accept_type=None,
        expected_content_type=CONTENT_TYPE_MSGPACK,
    )
    assert parsed.authenticated_device_id is not None
    assert parsed.authenticated_signature is not None

    body: bytes = await _rpc_get_body_with_limit_check(request)
    outcome = await backend.auth.authenticated_auth(
        now=DateTime.now(),
        organization_id=parsed.organization_id,
        device_id=parsed.authenticated_device_id,
        signature=parsed.authenticated_signature,
        body=body,
    )
    match outcome:
        case AuthenticatedAuthInfo() as auth_info:
            pass
        case AuthAuthenticatedAuthBadOutcome.ORGANIZATION_EXPIRED:
            _handshake_abort(
                CustomHttpStatus.OrganizationExpired.value, api_version=parsed.settled_api_version
            )
        case AuthAuthenticatedAuthBadOutcome.ORGANIZATION_NOT_FOUND:
            _handshake_abort(
                CustomHttpStatus.OrganizationOrInvitationInvalidOrNotFound.value,
                api_version=parsed.settled_api_version,
            )
        case (
            AuthAuthenticatedAuthBadOutcome.DEVICE_NOT_FOUND
            | AuthAuthenticatedAuthBadOutcome.INVALID_SIGNATURE
        ):
            _handshake_abort(
                CustomHttpStatus.BadAuthenticationInfo.value,
                api_version=parsed.settled_api_version,
            )
        case AuthAuthenticatedAuthBadOutcome.USER_REVOKED:
            _handshake_abort(
                CustomHttpStatus.UserRevoked.value,
                api_version=parsed.settled_api_version,
            )
        case unknown:
            assert_never(unknown)

    # Handshake is done

    client_ctx = AuthenticatedClientContext(
        client_api_version=parsed.client_api_version,
        settled_api_version=parsed.settled_api_version,
        organization_id=auth_info.organization_id,
        organization_internal_id=auth_info.organization_internal_id,
        device_id=auth_info.device_id,
        device_internal_id=auth_info.device_internal_id,
        device_verify_key=auth_info.device_verify_key,
    )

    try:
        req = AUTHENTICATED_CMDS_LOAD_FN[parsed.settled_api_version.version](body)
    except ValueError:
        _handshake_abort_bad_content(api_version=parsed.settled_api_version)

    cmd_func = backend.apis[type(req)]
    rep = await cmd_func(client_ctx, req)
    return _rpc_rep(rep, parsed.settled_api_version)


@rpc_router.get("/authenticated/{raw_organization_id}/events")
async def authenticated_events_api(raw_organization_id: str, request: Request) -> Response:
    backend: Backend = request.app.state.backend

    parsed = _parse_auth_headers_or_abort(
        headers=request.headers,
        raw_organization_id=raw_organization_id,
        with_authenticated_headers=True,
        with_invited_headers=False,
        with_sse_headers=True,
        expected_accept_type=ACCEPT_TYPE_SSE,
        # We don't care of Content-Type given the request has no body
        expected_content_type=None,
    )
    assert parsed.authenticated_device_id is not None
    assert parsed.authenticated_signature is not None

    outcome = await backend.auth.authenticated_auth(
        now=DateTime.now(),
        organization_id=parsed.organization_id,
        device_id=parsed.authenticated_device_id,
        signature=parsed.authenticated_signature,
        body=b"",  # Body is empty for a get !
    )
    match outcome:
        case AuthenticatedAuthInfo() as auth_info:
            pass
        case AuthAuthenticatedAuthBadOutcome.ORGANIZATION_EXPIRED:
            _handshake_abort(
                CustomHttpStatus.OrganizationExpired.value, api_version=parsed.settled_api_version
            )
        case AuthAuthenticatedAuthBadOutcome.ORGANIZATION_NOT_FOUND:
            _handshake_abort(
                CustomHttpStatus.OrganizationOrInvitationInvalidOrNotFound.value,
                api_version=parsed.settled_api_version,
            )
        case (
            AuthAuthenticatedAuthBadOutcome.DEVICE_NOT_FOUND
            | AuthAuthenticatedAuthBadOutcome.INVALID_SIGNATURE
        ):
            _handshake_abort(
                CustomHttpStatus.BadAuthenticationInfo.value,
                api_version=parsed.settled_api_version,
            )
        case AuthAuthenticatedAuthBadOutcome.USER_REVOKED:
            _handshake_abort(
                CustomHttpStatus.UserRevoked.value,
                api_version=parsed.settled_api_version,
            )
        case unknown:
            assert_never(unknown)

    # Handshake is done

    client_ctx = AuthenticatedClientContext(
        client_api_version=parsed.client_api_version,
        settled_api_version=parsed.settled_api_version,
        organization_id=auth_info.organization_id,
        organization_internal_id=auth_info.organization_internal_id,
        device_id=auth_info.device_id,
        device_internal_id=auth_info.device_internal_id,
        device_verify_key=auth_info.device_verify_key,
    )

    async def _stream() -> AsyncGenerator[bytes, None]:
        async with backend.events.sse_api_events_listen(
            client_ctx=client_ctx, last_event_id=parsed.last_event_id
        ) as outcome:
            match outcome:
                case (initial_organization_config_event, additional_events_receiver):
                    pass
                case SseAPiEventsListenBadOutcome():
                    # Force the closing of the connection
                    return
                case unknown:
                    assert_never(unknown)

            # In SSE, the HTTP status code & headers are sent with the first event.
            # This means the client has to wait for this first event to know for
            # sure the connection was successful (in practice the server responds
            # fast in case of error and potentially up to the keepalive time in case
            # everything is ok).
            # So, while not strictly needed, we it is better to send an event right
            # away the client knows it is correctly connected without delay.
            # Fortunately we just have the right thing for that: the organization
            # config (given it may have changed since the last time the client connected).

            yield initial_organization_config_event.dump_as_apiv4_sse_payload()
            del initial_organization_config_event

            while True:
                next_event = None
                with anyio.move_on_after(backend.config.sse_keepalive) as scope:
                    try:
                        next_event = await additional_events_receiver.receive()
                    except StopAsyncIteration:
                        return

                if scope.cancel_called:
                    yield b":keepalive\n\n"

                else:
                    if next_event is None:
                        # We have missed some events, most likely because the last event id
                        # provided by the client is too old. In this case we have to
                        # notify the client with a special `missed_events` message
                        yield b"event:missed_events\n\n"

                    else:
                        (event, apiv4_sse_payload) = next_event
                        if apiv4_sse_payload is None:
                            apiv4_sse_payload = event.dump_as_apiv4_sse_payload()
                        yield apiv4_sse_payload

    return StreamingResponse(
        content=_stream(),
        status_code=200,
        headers={
            "Cache-Control": "no-cache",
            "Transfer-Encoding": "chunked",
        },
        media_type=ACCEPT_TYPE_SSE,
    )
    # TODO: ensure server doesn't drop this SSE long-polling query due to inactivity timeout

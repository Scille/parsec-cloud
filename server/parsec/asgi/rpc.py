# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from typing import (
    Annotated,
    Any,
    AsyncGenerator,
    Mapping,
    NoReturn,
    Sequence,
)
from uuid import UUID

import anyio
from fastapi import APIRouter, HTTPException, Request, Response
from fastapi.datastructures import Headers
from fastapi.responses import StreamingResponse
from pydantic import (
    GetPydanticSchema,
)
from pydantic_core import core_schema

from parsec._parsec import (
    ApiVersion,
    DateTime,
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
    AuthenticatedToken,
    AuthInvitedAuthBadOutcome,
    InvitedAuthInfo,
)
from parsec.components.events import ClientBroadcastableEventStream, SseAPiEventsListenBadOutcome
from parsec.events import EventOrganizationConfig
from parsec.logging import get_logger

logger = get_logger()


def block_repr(block: bytes, MAX_LOGGED_BLOCK_SIZE=64) -> str:
    if len(block) <= MAX_LOGGED_BLOCK_SIZE:
        return repr(block)
    return f"{block[:MAX_LOGGED_BLOCK_SIZE]}... ({len(block)} bytes)"


@dataclass
class LoggedReq:
    req: object

    def __repr__(self) -> str:
        # The `block_create` requests have by far the largest payloads (up to 512K).
        # In debug mode, each call to those commands will add up to 1.5MB of content to the logs (due to ASCII representation).
        # Truncating those payloads allows to reduce the logs to a reasonable size.
        if isinstance(self.req, authenticated_cmds.latest.block_create.Req):
            return f"BlockCreateReq {{ block: {block_repr(self.req.block)} }}"
        else:
            return repr(self.req)


@dataclass
class LoggedRep:
    rep: object

    def __repr__(self) -> str:
        # The `block_read` replies have by far the largest payloads (up to 512K).
        # In debug mode, each call to those commands will add up to 1.5MB of content to the logs (due to ASCII representation).
        # Truncating those payloads allows to reduce the logs to a reasonable size.
        if isinstance(self.rep, authenticated_cmds.latest.block_read.RepOk):
            return (
                "Ok { "
                f"block: {block_repr(self.rep.block)}, "
                f"key_index: {self.rep.key_index!r}, "
                f"needed_realm_certificate_timestamp: {self.rep.needed_realm_certificate_timestamp!r}"
                " }"
            )
        else:
            return repr(self.rep)


CONTENT_TYPE_MSGPACK = "application/msgpack"
ACCEPT_TYPE_SSE = "text/event-stream"
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


OrganizationIDField = Annotated[
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
# - 401: Missing authentication info (no or invalid `Authorization` header)
# - 403: Bad authentication info (user/invitation not found, or invalid auth token)
# - 404: Organization not found or invalid organization ID
# - 406: Bad accept type (for the SSE events route)
# - 410: Invitation already deleted / used
# - 415: Bad content-type, body is not a valid message or unknown command
# - 422: Unsupported API version
# - 460: Organization is expired
# - 461: User is revoked
# - 462: User is frozen
# - 498: Authentication token expired


class CustomHttpStatus(Enum):
    MissingAuthenticationInfo = 401
    BadAuthenticationInfo = 403
    OrganizationNotFound = 404
    BadAcceptType = 406
    InvitationAlreadyUsedOrDeleted = 410
    BadContentTypeOrInvalidBodyOrUnknownCommand = 415
    UnsupportedApiVersion = 422
    OrganizationExpired = 460
    UserRevoked = 461
    UserFrozen = 462
    TokenExpired = 498


# e.g. `InvitationAlreadyUsedOrDeleted` -> `Invitation already used or deleted`
CUSTOM_HTTP_STATUS_DETAILS = {
    status: ("".join(" " + c.lower() if c.isupper() else c for c in status.name))
    .strip()
    .capitalize()
    for status in CustomHttpStatus
}


def _handshake_abort(status: CustomHttpStatus, api_version: ApiVersion, **headers: str) -> NoReturn:
    detail = CUSTOM_HTTP_STATUS_DETAILS[status]
    raise HTTPException(
        status_code=status.value,
        headers={"Api-Version": str(api_version), **headers},
        detail=detail,
    )


def _handshake_abort_bad_content(api_version: ApiVersion) -> NoReturn:
    _handshake_abort(CustomHttpStatus.BadContentTypeOrInvalidBodyOrUnknownCommand, api_version)


@dataclass
class ParsedAuthHeaders:
    organization_id: OrganizationID
    settled_api_version: ApiVersion
    client_api_version: ApiVersion
    user_agent: str
    authenticated_token: AuthenticatedToken | None
    invited_token: InvitationToken | None
    last_event_id: UUID | None


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
    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        _handshake_abort(
            CustomHttpStatus.OrganizationNotFound,
            api_version=settled_api_version,
        )

    # 3) Check User-Agent, Content-Type & Accept
    user_agent = headers.get("User-Agent", "unknown")
    if expected_content_type and headers.get("Content-Type") != expected_content_type:
        _handshake_abort_bad_content(api_version=settled_api_version)
    if expected_accept_type and headers.get("Accept") != expected_accept_type:
        _handshake_abort(CustomHttpStatus.BadAcceptType, api_version=settled_api_version)

    # 4) Check authenticated headers
    if not with_authenticated_headers:
        authenticated_token = None

    else:
        try:
            raw_authorization = headers["Authorization"]
        except KeyError:
            _handshake_abort(
                CustomHttpStatus.MissingAuthenticationInfo, api_version=settled_api_version
            )

        try:
            expected_bearer, raw_authenticated_token = raw_authorization.split()
            if expected_bearer.lower() != "bearer":
                raise ValueError

            authenticated_token = AuthenticatedToken.from_raw(raw_authenticated_token.encode())
        except ValueError:
            _handshake_abort(
                CustomHttpStatus.MissingAuthenticationInfo, api_version=settled_api_version
            )

    # 5) Check invited headers
    if not with_invited_headers:
        invited_token = None

    else:
        try:
            raw_authorization = headers["Authorization"]
        except KeyError:
            _handshake_abort(
                CustomHttpStatus.MissingAuthenticationInfo, api_version=settled_api_version
            )

        try:
            expected_bearer, raw_invitation_token = raw_authorization.split()
            invited_token = InvitationToken.from_hex(raw_invitation_token)
            if expected_bearer.lower() != "bearer":
                raise ValueError
        except ValueError:
            _handshake_abort(
                CustomHttpStatus.MissingAuthenticationInfo, api_version=settled_api_version
            )

    if not with_sse_headers:
        last_event_id = None
    else:
        last_event_id = headers.get("Last-Event-Id")
        if last_event_id is not None:
            try:
                last_event_id = UUID(last_event_id)
            except ValueError:
                last_event_id = None

    return ParsedAuthHeaders(
        organization_id=organization_id,
        settled_api_version=settled_api_version,
        client_api_version=client_api_version,
        user_agent=user_agent,
        last_event_id=last_event_id,
        authenticated_token=authenticated_token,
        invited_token=invited_token,
    )


async def run_request(
    backend: Backend,
    client_ctx: AuthenticatedClientContext | InvitedClientContext | AnonymousClientContext,
    request: object,
) -> object:
    cmd_func = backend.apis[type(request)]

    # Bind the logger with the command name before logging the request
    cmd_name = cmd_func._api_info["cmd"]  # type: ignore
    client_ctx.logger = client_ctx.logger.bind(cmd=cmd_name)

    client_ctx.logger.debug(
        "RPC request",
        req=LoggedReq(request),
    )
    try:
        rep = await cmd_func(client_ctx, request)
    except HTTPException as exc:
        logger.info(
            "RPC HTTP error",
            status_code=exc.status_code,
            detail=exc.detail,
        )
        raise
    except Exception as exc:
        logger.error("RPC exception", exc_info=exc)
        raise
    except BaseException as exc:
        logger.debug("RPC base exception", exc_info=exc)
        raise
    client_ctx.logger.info_with_debug_extra(
        "RPC reply",
        status=type(rep).__name__,
        debug_extra={"rep": LoggedRep(rep)},
    )
    return rep


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
                CustomHttpStatus.OrganizationExpired, api_version=parsed.settled_api_version
            )
        case AuthAnonymousAuthBadOutcome.ORGANIZATION_NOT_FOUND:
            _handshake_abort(
                CustomHttpStatus.OrganizationNotFound,
                api_version=parsed.settled_api_version,
            )

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

    rep = await run_request(backend, client_ctx, req)

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
                CustomHttpStatus.OrganizationExpired, api_version=parsed.settled_api_version
            )
        case AuthInvitedAuthBadOutcome.ORGANIZATION_NOT_FOUND:
            _handshake_abort(
                CustomHttpStatus.OrganizationNotFound,
                api_version=parsed.settled_api_version,
            )
        case AuthInvitedAuthBadOutcome.INVITATION_NOT_FOUND:
            _handshake_abort(
                CustomHttpStatus.BadAuthenticationInfo,
                api_version=parsed.settled_api_version,
            )
        case AuthInvitedAuthBadOutcome.INVITATION_ALREADY_USED:
            _handshake_abort(
                CustomHttpStatus.InvitationAlreadyUsedOrDeleted,
                api_version=parsed.settled_api_version,
            )

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

    rep = await run_request(backend, client_ctx, req)

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
    assert parsed.authenticated_token is not None

    body: bytes = await _rpc_get_body_with_limit_check(request)
    outcome = await backend.auth.authenticated_auth(
        now=DateTime.now(),
        organization_id=parsed.organization_id,
        token=parsed.authenticated_token,
    )
    match outcome:
        case AuthenticatedAuthInfo() as auth_info:
            pass
        case AuthAuthenticatedAuthBadOutcome.ORGANIZATION_EXPIRED:
            _handshake_abort(
                CustomHttpStatus.OrganizationExpired, api_version=parsed.settled_api_version
            )
        case AuthAuthenticatedAuthBadOutcome.ORGANIZATION_NOT_FOUND:
            _handshake_abort(
                CustomHttpStatus.OrganizationNotFound,
                api_version=parsed.settled_api_version,
            )
        case AuthAuthenticatedAuthBadOutcome.TOKEN_TOO_OLD:
            _handshake_abort(
                CustomHttpStatus.TokenExpired,
                api_version=parsed.settled_api_version,
            )
        case (
            AuthAuthenticatedAuthBadOutcome.DEVICE_NOT_FOUND
            | AuthAuthenticatedAuthBadOutcome.INVALID_SIGNATURE
        ):
            _handshake_abort(
                CustomHttpStatus.BadAuthenticationInfo,
                api_version=parsed.settled_api_version,
            )
        case AuthAuthenticatedAuthBadOutcome.USER_REVOKED:
            _handshake_abort(
                CustomHttpStatus.UserRevoked,
                api_version=parsed.settled_api_version,
            )
        case AuthAuthenticatedAuthBadOutcome.USER_FROZEN:
            _handshake_abort(
                CustomHttpStatus.UserFrozen,
                api_version=parsed.settled_api_version,
            )

    # Handshake is done

    client_ctx = AuthenticatedClientContext(
        client_api_version=parsed.client_api_version,
        settled_api_version=parsed.settled_api_version,
        organization_id=auth_info.organization_id,
        organization_internal_id=auth_info.organization_internal_id,
        user_id=auth_info.user_id,
        device_id=auth_info.device_id,
        device_internal_id=auth_info.device_internal_id,
        device_verify_key=auth_info.device_verify_key,
    )

    try:
        req = AUTHENTICATED_CMDS_LOAD_FN[parsed.settled_api_version.version](body)
    except ValueError:
        _handshake_abort_bad_content(api_version=parsed.settled_api_version)

    rep = await run_request(backend, client_ctx, req)

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
    assert parsed.authenticated_token is not None

    outcome = await backend.auth.authenticated_auth(
        now=DateTime.now(),
        organization_id=parsed.organization_id,
        token=parsed.authenticated_token,
    )
    match outcome:
        case AuthenticatedAuthInfo() as auth_info:
            pass
        case AuthAuthenticatedAuthBadOutcome.ORGANIZATION_EXPIRED:
            _handshake_abort(
                CustomHttpStatus.OrganizationExpired, api_version=parsed.settled_api_version
            )
        case AuthAuthenticatedAuthBadOutcome.ORGANIZATION_NOT_FOUND:
            _handshake_abort(
                CustomHttpStatus.OrganizationNotFound,
                api_version=parsed.settled_api_version,
            )
        case AuthAuthenticatedAuthBadOutcome.TOKEN_TOO_OLD:
            _handshake_abort(
                CustomHttpStatus.TokenExpired,
                api_version=parsed.settled_api_version,
            )
        case (
            AuthAuthenticatedAuthBadOutcome.DEVICE_NOT_FOUND
            | AuthAuthenticatedAuthBadOutcome.INVALID_SIGNATURE
        ):
            _handshake_abort(
                CustomHttpStatus.BadAuthenticationInfo,
                api_version=parsed.settled_api_version,
            )
        case AuthAuthenticatedAuthBadOutcome.USER_REVOKED:
            _handshake_abort(
                CustomHttpStatus.UserRevoked,
                api_version=parsed.settled_api_version,
            )
        case AuthAuthenticatedAuthBadOutcome.USER_FROZEN:
            _handshake_abort(
                CustomHttpStatus.UserFrozen,
                api_version=parsed.settled_api_version,
            )

    # Handshake is done

    client_ctx = AuthenticatedClientContext(
        client_api_version=parsed.client_api_version,
        settled_api_version=parsed.settled_api_version,
        organization_id=auth_info.organization_id,
        organization_internal_id=auth_info.organization_internal_id,
        user_id=auth_info.user_id,
        device_id=auth_info.device_id,
        device_internal_id=auth_info.device_internal_id,
        device_verify_key=auth_info.device_verify_key,
    )

    return StreamingResponseMiddleware(
        backend,
        client_ctx,
        parsed.last_event_id,
        parsed.settled_api_version,
        status_code=200,
        headers={
            "Cache-Control": "no-cache",
            "Transfer-Encoding": "chunked",
        },
        media_type=ACCEPT_TYPE_SSE,
    )
    # TODO: ensure server doesn't drop this SSE long-polling query due to inactivity timeout


class StreamingResponseMiddleware(StreamingResponse):
    """
    StreamingResponse is subclassed to have better control over the stream
    throughout the lifetime of the response.

    In particular, the async `__call__` method has the same lifetime as the
    response, which allows proper use of async context managers.

    This is useful since the async generator used to stream the events should
    not contain context managers due to the fact that it is iterated without
    proper control over its lifetime, breaking the principles of structured concurrency.
    """

    def __init__(
        self,
        backend: Backend,
        client_ctx: AuthenticatedClientContext,
        last_event_id: UUID | None,
        settled_api_version: ApiVersion,
        status_code: int = 200,
        headers: Mapping[str, str] | None = None,
        media_type: str | None = None,
    ):
        self.backend = backend
        self.client_ctx = client_ctx
        self.last_event_id = last_event_id
        self.settled_api_version = settled_api_version
        self.initial_organization_config_event: EventOrganizationConfig
        self.additional_events_receiver: ClientBroadcastableEventStream
        super().__init__(
            content=self._stream(),
            status_code=status_code,
            headers=headers,
            media_type=media_type,
        )

    async def _stream(self) -> AsyncGenerator[bytes, None]:
        # In SSE, the HTTP status code & headers are sent with the first event.
        # This means the client has to wait for this first event to know for
        # sure the connection was successful (in practice the server responds
        # fast in case of error and potentially up to the keepalive time in case
        # everything is ok).
        # So, while not strictly needed, it is better to send an event right
        # away so the client knows it is correctly connected without delay.
        # Fortunately we just have the right thing for that: the organization
        # config (given it may have changed since the last time the client connected).
        yield self.initial_organization_config_event.dump_as_apiv4_sse_payload()

        while True:
            next_event = None
            # Context manager should be avoided in async generator, but it is fine
            # in this case since `move_on_after` doesn't use any resources that
            # would be kept alive after the context manager is exited.
            with anyio.move_on_after(self.backend.config.sse_keepalive) as scope:
                try:
                    next_event = await self.additional_events_receiver.receive()
                except StopAsyncIteration:
                    return

            if scope.cancel_called:
                self.client_ctx.logger.debug("SSE keepalive")
                yield b":keepalive\n\n"

            else:
                if next_event is None:
                    self.client_ctx.logger.debug("SSE missed events")
                    # We have missed some events, most likely because the last event id
                    # provided by the client is too old. In this case we have to
                    # notify the client with a special `missed_events` message
                    #
                    # Event if it is empty, `data` field must be provided or SSE
                    # client will silently ignore the event.
                    # (cf. https://html.spec.whatwg.org/multipage/server-sent-events.html#dispatchMessage)
                    yield b"event:missed_events\ndata:\n\n"

                else:
                    (event, apiv4_sse_payload) = next_event
                    if apiv4_sse_payload is None:
                        apiv4_sse_payload = event.dump_as_apiv4_sse_payload()
                    self.client_ctx.logger.debug("SSE event", event_=event)
                    yield apiv4_sse_payload

    async def __call__(self, scope, receive, send) -> None:
        self.client_ctx.logger.info("SSE session start")
        try:
            await self._run_session(scope, receive, send)
        except HTTPException as exc:
            self.client_ctx.logger.info(
                "SSE session HTTP error",
                status_code=exc.status_code,
                detail=exc.detail,
            )
            raise
        except Exception as exc:
            self.client_ctx.logger.error("SSE session exception", exc_info=exc)
            raise
        except BaseException as exc:
            self.client_ctx.logger.debug("SSE session base exception", exc_info=exc)
            raise
        else:
            self.client_ctx.logger.info("SSE session end")

    async def _run_session(self, scope, receive, send) -> None:
        """
        This method is called by the ASGI server to handle the response.

        It lives as long as events are being produced, which is the right place
        to put the `sse_api_events_listen` context manager.
        """
        async with self.backend.events.sse_api_events_listen(
            client_ctx=self.client_ctx, last_event_id=self.last_event_id
        ) as outcome:
            match outcome:
                case tuple() as item:
                    self.initial_organization_config_event, self.additional_events_receiver = item
                    self.client_ctx.logger.debug("SSE client registered")
                    return await super().__call__(scope, receive, send)
                case SseAPiEventsListenBadOutcome.ORGANIZATION_NOT_FOUND:
                    _handshake_abort(
                        CustomHttpStatus.OrganizationNotFound,
                        api_version=self.settled_api_version,
                        **self.headers,
                    )
                case SseAPiEventsListenBadOutcome.ORGANIZATION_EXPIRED:
                    _handshake_abort(
                        CustomHttpStatus.OrganizationExpired,
                        api_version=self.settled_api_version,
                        **self.headers,
                    )
                case SseAPiEventsListenBadOutcome.AUTHOR_NOT_FOUND:
                    _handshake_abort(
                        CustomHttpStatus.BadAuthenticationInfo,
                        api_version=self.settled_api_version,
                        **self.headers,
                    )
                case SseAPiEventsListenBadOutcome.AUTHOR_REVOKED:
                    _handshake_abort(
                        CustomHttpStatus.UserRevoked,
                        api_version=self.settled_api_version,
                        **self.headers,
                    )

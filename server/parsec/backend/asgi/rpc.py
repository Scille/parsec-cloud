# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from base64 import b64decode, b64encode
from contextlib import asynccontextmanager, contextmanager
from enum import Enum
from typing import Any, AsyncIterable, AsyncIterator, Iterator, NoReturn, Type

import trio
from quart import Blueprint, Response, current_app, g, request
from quart.wrappers.response import ResponseBody

from parsec._parsec import (
    ApiVersion,
    BackendEvent,
    BackendEventOrganizationExpired,
    BackendEventUserUpdatedOrRevoked,
    CryptoError,
    DateTime,
    DeviceID,
    InvitationToken,
    OrganizationID,
    ProtocolError,
    anonymous_cmds,
    authenticated_cmds,
    invited_cmds,
)
from parsec.api.protocol import (
    IncompatibleAPIVersionsError,
    settle_compatible_versions,
)
from parsec.backend.app import BackendApp
from parsec.backend.client_context import (
    AnonymousClientContext,
    AuthenticatedClientContext,
    InvitedClientContext,
)
from parsec.backend.invite import (
    CloseInviteConnection,
    Invitation,
    InvitationAlreadyDeletedError,
    InvitationError,
    InvitationNotFoundError,
)
from parsec.backend.organization import (
    Organization,
    OrganizationAlreadyExistsError,
    OrganizationNotFoundError,
)
from parsec.backend.user import UserNotFoundError
from parsec.backend.user_type import Device, User

CONTENT_TYPE_MSGPACK = "application/msgpack"
ACCEPT_TYPE_SSE = "text/event-stream"
AUTHORIZATION_PARSEC_ED25519 = "PARSEC-SIGN-ED25519"
SUPPORTED_API_VERSIONS = (
    ApiVersion.API_V2_VERSION,
    ApiVersion.API_V3_VERSION,
    ApiVersion.API_V4_VERSION,
)
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

rpc_bp = Blueprint("anonymous_api", __name__)


def _rpc_rep(rep: Any, api_version: ApiVersion) -> Response:
    return Response(
        response=rep.dump(),
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
    current_app.aborter(
        Response(response="", status=status_code, headers={"Api-Version": str(api_version)})
    )


def _handshake_abort_bad_content(api_version: ApiVersion) -> NoReturn:
    _handshake_abort(
        CustomHttpStatus.BadContentTypeOrInvalidBodyOrUnknownCommand.value, api_version
    )


async def _do_handshake(
    raw_organization_id: str,
    backend: BackendApp,
    allow_missing_organization: bool,
    check_authentication: bool,
    check_invitation: bool,
    expected_content_type: str | None,
    expected_accept_type: str | None,
) -> tuple[
    ApiVersion,
    ApiVersion,
    OrganizationID,
    Organization | None,
    User | None,
    Device | None,
    Invitation | None,
]:
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
                response="",
                status=CustomHttpStatus.UnsupportedApiVersion.value,
                headers={"Supported-Api-Versions": supported_api_versions},
            )
        )

    # From now on the version is settled, our reply must have the `Api-Version` header

    # 2) Check organization
    # Check whether the organization exists
    try:
        organization_id = OrganizationID(raw_organization_id)
    except ValueError:
        _handshake_abort(
            CustomHttpStatus.OrganizationOrInvitationInvalidOrNotFound.value,
            api_version=api_version,
        )
    organization: Organization | None
    try:
        organization = await backend.organization.get(organization_id)
    except OrganizationNotFoundError:
        if not allow_missing_organization:
            _handshake_abort(
                CustomHttpStatus.OrganizationOrInvitationInvalidOrNotFound.value,
                api_version=api_version,
            )
        else:
            organization = None
    else:
        if organization.is_expired:
            _handshake_abort(CustomHttpStatus.OrganizationExpired.value, api_version=api_version)

    # 3) Check Content-Type & Accept
    if expected_content_type and request.headers.get("Content-Type") != expected_content_type:
        _handshake_abort_bad_content(api_version=api_version)
    if expected_accept_type and request.headers.get("Accept") != expected_accept_type:
        _handshake_abort(CustomHttpStatus.BadAcceptType.value, api_version=api_version)

    # 4) Check authentication
    if not check_authentication:
        user = None
        device = None

    else:
        try:
            authorization_method = request.headers["Authorization"]
        except KeyError:
            _handshake_abort(CustomHttpStatus.BadAuthenticationInfo.value, api_version=api_version)

        if authorization_method != AUTHORIZATION_PARSEC_ED25519:
            _handshake_abort(CustomHttpStatus.BadAuthenticationInfo.value, api_version=api_version)

        try:
            raw_device_id = request.headers["Author"]
            raw_signature_b64 = request.headers["Signature"]
        except KeyError:
            _handshake_abort(CustomHttpStatus.BadAuthenticationInfo.value, api_version=api_version)

        try:
            signature_bytes = b64decode(raw_signature_b64)
            device_id = DeviceID(b64decode(raw_device_id).decode())
        except ValueError:
            _handshake_abort(CustomHttpStatus.BadAuthenticationInfo.value, api_version=api_version)

        body: bytes = await request.get_data()
        try:
            user, device = await backend.user.get_user_with_device(organization_id, device_id)
        except UserNotFoundError:
            _handshake_abort(CustomHttpStatus.BadAuthenticationInfo.value, api_version=api_version)
        else:
            if user.revoked_on:
                _handshake_abort(CustomHttpStatus.UserRevoked.value, api_version=api_version)

        try:
            device.verify_key.verify_with_signature(
                signature=signature_bytes,
                message=body,
            )
        except CryptoError:
            _handshake_abort(CustomHttpStatus.BadAuthenticationInfo.value, api_version=api_version)

    if not check_invitation:
        invitation = None
    else:
        raw_invitation_token = request.headers.get("Invitation-Token", "")

        try:
            token = InvitationToken.from_hex(raw_invitation_token)
        except ValueError:
            _handshake_abort_bad_content(api_version=api_version)

        try:
            invitation = await backend.invite.info(organization_id=organization_id, token=token)
        except InvitationAlreadyDeletedError:
            _handshake_abort(
                CustomHttpStatus.InvitationAlreadyUsedOrDeleted.value, api_version=api_version
            )
        except InvitationNotFoundError:
            _handshake_abort(404, api_version=api_version)
        except InvitationError:
            _handshake_abort(CustomHttpStatus.BadAuthenticationInfo.value, api_version=api_version)

    return api_version, client_api_version, organization_id, organization, user, device, invitation


@rpc_bp.route("/anonymous/<raw_organization_id>", methods=["GET", "POST"])
async def anonymous_api(raw_organization_id: str) -> Response:
    backend: BackendApp = g.backend

    allow_missing_organization = (
        request.method == "POST" and backend.config.organization_spontaneous_bootstrap
    )

    api_version, client_api_version, organization_id, organization, _, _, _ = await _do_handshake(
        raw_organization_id=raw_organization_id,
        backend=backend,
        allow_missing_organization=allow_missing_organization,
        check_authentication=False,
        check_invitation=False,
        expected_content_type=CONTENT_TYPE_MSGPACK,
        expected_accept_type=None,
    )

    # Reply to GET
    if request.method == "GET":
        return Response(
            response=b"",
            status=200,
            content_type=CONTENT_TYPE_MSGPACK,
            headers={"Api-Version": str(api_version)},
        )

    body: bytes = await request.get_data(cache=False)

    try:
        req = ANONYMOUS_CMDS_LOAD_FN[api_version.version](body)
    except ProtocolError:
        _handshake_abort_bad_content(api_version=api_version)

    # Lazy creation of the organization if necessary
    if isinstance(req, ANONYMOUS_ORGANIZATION_BOOTSTRAP_CMD_ALL_API_REQS) and not organization:
        assert backend.config.organization_spontaneous_bootstrap
        try:
            await backend.organization.create(
                id=organization_id, bootstrap_token="", created_on=DateTime.now()
            )
        except OrganizationAlreadyExistsError:
            pass

    # Retrieve command
    client_ctx = AnonymousClientContext(api_version, client_api_version, organization_id)
    client_ctx.logger.info(
        f"Anonymous client successfully connected (client/server API version: {client_api_version}/{api_version})"
    )

    cmd_func = backend.apis[type(req)]

    # Run command
    try:
        rep = await cmd_func(client_ctx, req)
    except Exception as exc:
        print("rpc didn't handle this exception:", type(exc))
        raise exc

    return _rpc_rep(rep, api_version)


@rpc_bp.route("/invited/<raw_organization_id>", methods=["POST"])
async def invited_api(raw_organization_id: str) -> Response:
    backend: BackendApp = g.backend

    api_version, client_api_version, organization_id, _, _, _, invitation = await _do_handshake(
        raw_organization_id=raw_organization_id,
        backend=backend,
        allow_missing_organization=False,
        check_authentication=False,
        check_invitation=True,
        expected_content_type=CONTENT_TYPE_MSGPACK,
        expected_accept_type=None,
    )

    # Unpack verified body
    body: bytes = await request.get_data(cache=False)

    try:
        req = INVITED_CMDS_LOAD_FN[api_version.version](body)
    except ProtocolError:
        _handshake_abort_bad_content(api_version=api_version)

    cmd_func = backend.apis[type(req)]

    assert invitation is not None
    client_ctx = InvitedClientContext(
        api_version=api_version,
        client_api_version=client_api_version,
        organization_id=organization_id,
        invitation=invitation,
    )
    client_ctx.logger.info(
        f"Invited client successfully connected (client/server API version: {client_api_version}/{api_version})"
    )

    try:
        rep = await cmd_func(client_ctx, req)
    except CloseInviteConnection:
        _handshake_abort(
            CustomHttpStatus.InvitationAlreadyUsedOrDeleted.value, api_version=api_version
        )
    except Exception as exc:
        print("rpc didn't handle this exception:", type(exc))
        raise exc

    return _rpc_rep(rep, api_version)


@rpc_bp.route("/authenticated/<raw_organization_id>", methods=["POST"])
async def authenticated_api(raw_organization_id: str) -> Response:
    backend: BackendApp = g.backend

    api_version, client_api_version, organization_id, _, user, device, _ = await _do_handshake(
        raw_organization_id=raw_organization_id,
        backend=backend,
        allow_missing_organization=False,
        check_authentication=True,
        check_invitation=False,
        expected_content_type=CONTENT_TYPE_MSGPACK,
        expected_accept_type=None,
    )
    assert isinstance(user, User)
    assert isinstance(device, Device)

    # Unpack verified body
    body: bytes = await request.get_data(cache=False)

    try:
        req = AUTHENTICATED_CMDS_LOAD_FN[api_version.version](body)
    except ProtocolError:
        _handshake_abort_bad_content(api_version=api_version)

    cmd_func = backend.apis[type(req)]

    client_ctx = AuthenticatedClientContext(
        api_version=api_version,
        client_api_version=client_api_version,
        organization_id=organization_id,
        device_id=device.device_id,
        human_handle=user.human_handle,
        device_label=device.device_label,
        profile=user.profile,
        public_key=user.public_key,
        verify_key=device.verify_key,
    )
    client_ctx.logger.info(
        f"Authenticated client successfully connected (client/server API version: {client_api_version}/{api_version})"
    )

    try:
        rep = await cmd_func(client_ctx, req)
    except Exception as exc:
        print("rpc didn't handle this exception:", type(exc))
        raise exc

    return _rpc_rep(rep, api_version)


# SSE in Quart-Trio is a bit more complicated than expected:
# In theory we would just have to return as body an async iterator that yields
# bytes each time it needs to provide a SSE event.
# However in practice this doesn't work at all given async iterator doesn't
# properly support trio scopes (e.g. using a cancel scope in an async iterator
# won't work as expected !).
# The proper way to combine scope management and async iterator is to first have
# an async context manager that itself provides an async iterator, the latter being
# just a dummy channel (so that no scope logic is done in the async iterator).
# Fortunately Quart-Trio knows about this shortcoming, and wraps the user-provided
# async iterator into an `IterableBody` that is used as async context manager
# returning the async iterator.
# So long story short, given we need to have access on the async context manager
# (so that we close the connection's scope in case backpressure gets too high),
# we have to implement our own custom `IterableBody` here.


class SSEResponseIterableBody(ResponseBody):
    def __init__(
        self,
        backend: BackendApp,
        client_ctx: AuthenticatedClientContext,
        last_event_id: str | None,
    ) -> None:
        super().__init__()
        # Cancel scope will be closed automatically in case of backpressure issue
        # Note we create it here instead of in `_make_contextmanager` to avoid concurrency
        # issue if the cancellation kicks in between the moment we return the
        # response object and when quart starts entering the async context manager
        # returned by `_make_contextmanager`.
        assert client_ctx.cancel_scope is None
        client_ctx.cancel_scope = trio.CancelScope()
        # Zero-sized channel to avoid backpressure issue: send is blocking until a receive occurs
        self._sse_payload_sender, self._sse_payload_receiver = trio.open_memory_channel[bytes](0)
        self._contextmanager = self._make_contextmanager(backend, client_ctx, last_event_id)

    @asynccontextmanager
    async def _make_contextmanager(
        self, backend: BackendApp, client_ctx: AuthenticatedClientContext, last_event_id: str | None
    ) -> AsyncIterator[None]:
        @contextmanager
        def _stop_listen_when_peer_becomes_invalid(
            backend: BackendApp, client_ctx: AuthenticatedClientContext
        ) -> Iterator[None]:
            with backend.event_bus.connection_context() as client_ctx.event_bus_ctx:

                def _on_updated_or_revoked(
                    event: Type[BackendEvent],
                    event_id: str,
                    payload: BackendEventUserUpdatedOrRevoked,
                ) -> None:
                    if (
                        payload.organization_id == client_ctx.organization_id
                        and payload.user_id == client_ctx.user_id
                    ):
                        # We close the connection even if the event is about a profile
                        # change and not a revocation given `client_ctx` is outdated
                        client_ctx.close_connection_asap()

                def _on_expired(
                    event: Type[BackendEvent],
                    event_id: str,
                    payload: BackendEventOrganizationExpired,
                ) -> None:
                    if payload.organization_id == client_ctx.organization_id:
                        client_ctx.close_connection_asap()

                client_ctx.event_bus_ctx.connect(
                    BackendEventUserUpdatedOrRevoked,
                    _on_updated_or_revoked,  # type: ignore
                )
                client_ctx.event_bus_ctx.connect(
                    BackendEventOrganizationExpired,
                    _on_expired,  # type: ignore
                )

                yield

        async def _events_into_sse_payloads() -> None:
            with _stop_listen_when_peer_becomes_invalid(backend, client_ctx):
                # Closing sender end of the channel will cause Quart stop iterating
                # on the receiver end and hence terminate the request
                with self._sse_payload_sender:
                    # In SSE, the HTTP status code & headers are sent with the first event.
                    # This means the client has to wait for this first event to know for
                    # sure the connection was successful (in practice the server responds
                    # fast in case of error and potentially up to the keepalive time in case
                    # everything is ok).
                    # While not strictly needed, we send a keepalive event (i.e. an event
                    # with no name and any comment, see https://html.spec.whatwg.org/multipage/server-sent-events.html#authoring-notes
                    # ) right away so that the client knows it is correctly connected without delay.
                    await self._sse_payload_sender.send(b":keepalive\n\n")

                    next_event_cb = await backend.events.sse_api_events_listen(
                        client_ctx, last_event_id
                    )
                    while True:
                        next_event = None
                        with trio.move_on_after(backend.config.sse_keepalive) as scope:
                            try:
                                next_event = await next_event_cb()
                            except StopAsyncIteration:
                                return

                        if scope.cancelled_caught:
                            await self._sse_payload_sender.send(b":keepalive\n\n")

                        else:
                            if next_event is None:
                                # We have missed some events, most likely because the last event id
                                # provided by the client is too old. In this case we have to
                                # notify the client with a special `missed_events` message
                                await self._sse_payload_sender.send(b"event:missed_events\n\n")

                            else:
                                (event_id, rep) = next_event
                                sse_payload = (
                                    b"data:"
                                    + b64encode(rep.dump())
                                    + b"\nid:"
                                    + event_id.encode("ascii")
                                    + b"\n\n"
                                )
                                await self._sse_payload_sender.send(sse_payload)

        assert client_ctx.cancel_scope is not None
        with client_ctx.cancel_scope:
            async with trio.open_nursery() as nursery:
                with backend.event_bus.connection_context() as client_ctx.event_bus_ctx:
                    nursery.start_soon(_events_into_sse_payloads)

                    # Note we don't use `with self._sse_payload_receiver:` context manager
                    # here.
                    # This is because `client_ctx.cancel_scope` gets closed as soon as
                    # the yield returns, hence `_events_into_sse_payloads` only have to
                    # deal with `trio.Cancelled` (while closing `self._sse_payload_receiver`
                    # would cause `trio.BrokenResourceError` on top of that)

                    yield

                    # Once here, Quart has decided to stop pulling for body lines, this
                    # could means three things:
                    #
                    # 1) Backpressure has caused the cancellation of the request
                    # 2) There is no more events to pull (i.e.
                    #    `self._sse_payload_receiver.close()` has been called)
                    # 3) Quart has decided to cancel the request (e.g. the server is
                    #    shutting down). Or an unhandled exception is bubbling up.
                    #
                    # In case of 1), `client_ctx.cancel_scope` has been cancelled so we
                    # have a `trio.Cancelled` exception propagating and doing a clean
                    # teardown for us.
                    #
                    # Case 2) is not possible given our query pull for events forever,
                    # however we play safe here and fallback to case 1) handling by
                    # explicitly cancel `client_ctx.cancel_scope` once the yield returns.
                    #
                    # Case 3) is similar to case 1) in that an exception gets propagated
                    # (the only difference is it won't stop once `client_ctx.cancel_scope`
                    # is reached), so we still have a clean teardown.
                    client_ctx.cancel_scope.cancel()

    async def __aenter__(self) -> AsyncIterable[bytes]:
        await self._contextmanager.__aenter__()
        return self

    async def __aexit__(self, *args: Any, **kwargs: Any) -> Any:
        return await self._contextmanager.__aexit__(*args, **kwargs)

    def __aiter__(self) -> AsyncIterator[bytes]:
        return self._sse_payload_receiver


@rpc_bp.route("/authenticated/<raw_organization_id>/events", methods=["GET"])
async def authenticated_events_api(raw_organization_id: str) -> Response:
    backend: BackendApp = g.backend

    api_version, client_api_version, organization_id, _, user, device, _ = await _do_handshake(
        raw_organization_id=raw_organization_id,
        backend=backend,
        allow_missing_organization=False,
        check_authentication=True,
        check_invitation=False,
        # We don't care of Content-Type given the request has no body
        expected_content_type=None,
        expected_accept_type=ACCEPT_TYPE_SSE,
    )
    assert user is not None
    assert device is not None

    # API<=3 is older than SSE (and hence never needs it)
    if api_version.version < 4:
        _handshake_abort_bad_content(api_version=api_version)

    client_ctx = AuthenticatedClientContext(
        api_version=api_version,
        client_api_version=client_api_version,
        organization_id=organization_id,
        device_id=device.device_id,
        human_handle=user.human_handle,
        device_label=device.device_label,
        profile=user.profile,
        public_key=user.public_key,
        verify_key=device.verify_key,
    )
    last_event_id = request.headers.get("Last-Event-Id")

    response = current_app.response_class(
        response=SSEResponseIterableBody(backend, client_ctx, last_event_id),
        headers={
            "Content-Type": ACCEPT_TYPE_SSE,
            "Cache-Control": "no-cache",
            "Transfer-Encoding": "chunked",
        },
        status=200,
    )
    # SSE are long lasting query, so we must disable Quart DOS protection here
    response.timeout = None
    return response

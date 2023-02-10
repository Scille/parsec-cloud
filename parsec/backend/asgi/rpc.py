# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from __future__ import annotations

from base64 import b64decode, b64encode
from contextlib import asynccontextmanager
from typing import Any, AsyncIterable, AsyncIterator, NoReturn, Tuple

import trio
from nacl.exceptions import CryptoError
from quart import Blueprint, Response, current_app, g, request
from quart.wrappers.response import ResponseBody

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
ACCEPT_TYPE_SSE = "text/event-stream"
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
# - 406: Bad accept type (for the SSE events route)
# - 415: Bad content-type, body is not a valid message or unknown command
# - 422: Unsupported API version
# - 460: Organization is expired
# - 461: User is revoked


def _handshake_abort(status_code: int, api_version: ApiVersion) -> NoReturn:
    current_app.aborter(
        Response(response="", status=status_code, headers={"Api-Version": str(api_version)})
    )


def _handshake_abort_bad_content(api_version: ApiVersion) -> NoReturn:
    _handshake_abort(415, api_version)


async def _do_handshake(
    raw_organization_id: str,
    backend: BackendApp,
    allow_missing_organization: bool,
    check_authentication: bool,
    expected_content_type: str | None,
    expected_accept_type: str | None,
) -> Tuple[ApiVersion, ApiVersion, OrganizationID, Organization | None, User | None, Device | None]:
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
            _handshake_abort(460, api_version=api_version)

    # 3) Check Content-Type & Accept
    if expected_content_type and request.headers.get("Content-Type") != expected_content_type:
        _handshake_abort_bad_content(api_version=api_version)
    if expected_accept_type and request.headers.get("Accept") != expected_accept_type:
        _handshake_abort(406, api_version=api_version)

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
                _handshake_abort(461, api_version=api_version)

        try:
            device.verify_key.verify_with_signature(
                signature=signature_bytes,
                message=body,
            )
        except CryptoError:
            _handshake_abort(401, api_version=api_version)

    return api_version, client_api_version, organization_id, organization, user, device


@rpc_bp.route("/anonymous/<raw_organization_id>", methods=["GET", "POST"])
async def anonymous_api(raw_organization_id: str) -> Response:
    backend: BackendApp = g.backend

    allow_missing_organization = (
        request.method == "POST" and backend.config.organization_spontaneous_bootstrap
    )

    api_version, client_api_version, organization_id, organization, _, _ = await _do_handshake(
        raw_organization_id=raw_organization_id,
        backend=backend,
        allow_missing_organization=allow_missing_organization,
        check_authentication=False,
        expected_content_type=CONTENT_TYPE_MSGPACK,
        expected_accept_type=None,
    )

    # Reply to GET
    if request.method == "GET":
        return _rpc_msgpack_rep({}, api_version)

    body: bytes = await request.get_data(cache=False)
    try:
        msg = unpackb(body)
    except SerdePackingError:
        _handshake_abort_bad_content(api_version=api_version)

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
    client_ctx = AnonymousClientContext(api_version, client_api_version, organization_id)
    client_ctx.logger.info(
        f"Anonymous client successfully connected (client/server API version: {client_api_version}/{api_version})"
    )
    if not isinstance(cmd, str):
        _handshake_abort_bad_content(api_version=api_version)

    try:
        cmd_func = backend.apis[ClientType.ANONYMOUS][cmd]
    except KeyError:
        _handshake_abort_bad_content(api_version=api_version)

    # Run command
    rep = await cmd_func(client_ctx, msg)
    return _rpc_msgpack_rep(rep, api_version)


@rpc_bp.route("/authenticated/<raw_organization_id>", methods=["POST"])
async def authenticated_api(raw_organization_id: str) -> Response:
    backend: BackendApp = g.backend

    api_version, client_api_version, organization_id, _, user, device = await _do_handshake(
        raw_organization_id=raw_organization_id,
        backend=backend,
        allow_missing_organization=False,
        check_authentication=True,
        expected_content_type=CONTENT_TYPE_MSGPACK,
        expected_accept_type=None,
    )
    assert isinstance(user, User)
    assert isinstance(device, Device)

    # Unpack verified body
    body: bytes = await request.get_data(cache=False)
    try:
        msg = unpackb(body)
    except SerdePackingError:
        _handshake_abort_bad_content(api_version=api_version)

    cmd = msg.get("cmd")
    if not isinstance(cmd, str):
        _handshake_abort_bad_content(api_version=api_version)

    try:
        cmd_func = backend.apis[ClientType.AUTHENTICATED][cmd]
    except KeyError:
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
    client_ctx.logger.info(
        f"Authenticated client successfully connected (client/server API version: {client_api_version}/{api_version})"
    )

    cmd_rep = await cmd_func(client_ctx, msg)
    return _rpc_msgpack_rep(cmd_rep, api_version)


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
    def __init__(self, client_ctx: AuthenticatedClientContext, backend: BackendApp) -> None:
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
        self._contextmanager = self._make_contextmanager(client_ctx, backend)

    @asynccontextmanager
    async def _make_contextmanager(
        self, client_ctx: AuthenticatedClientContext, backend: BackendApp
    ) -> AsyncIterator[None]:
        async def _events_into_sse_payloads() -> None:
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

                while True:
                    with trio.move_on_after(backend.config.sse_keepalive) as scope:
                        try:
                            event = await client_ctx.receive_events_channel.receive()
                        except trio.EndOfChannel:
                            break
                        sse_payload = b"data:" + b64encode(event.dump()) + b"\n\n"
                        await self._sse_payload_sender.send(sse_payload)

                    if scope.cancelled_caught:
                        sse_payload = b":keepalive\n\n"
                        await self._sse_payload_sender.send(sse_payload)

        assert client_ctx.cancel_scope is not None
        with client_ctx.cancel_scope:
            async with trio.open_nursery() as nursery:

                with backend.event_bus.connection_context() as client_ctx.event_bus_ctx:
                    await backend.events.connect_events(client_ctx)

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

    api_version, client_api_version, organization_id, _, user, device = await _do_handshake(
        raw_organization_id=raw_organization_id,
        backend=backend,
        allow_missing_organization=False,
        check_authentication=True,
        # We don't care of Content-Type given the request has no body
        expected_content_type=None,
        expected_accept_type=ACCEPT_TYPE_SSE,
    )
    assert user is not None
    assert device is not None

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

    response = current_app.response_class(
        response=SSEResponseIterableBody(client_ctx, backend),
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

# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import os
import trio
import ssl
import certifi
import urllib.request
from structlog import get_logger
from contextlib import asynccontextmanager
from typing import Optional, Dict, Callable, Awaitable, AsyncContextManager

from parsec.crypto import SigningKey
from parsec.api.transport import Transport, TransportError, TransportClosedByPeer
from parsec.api.protocol import (
    DeviceID,
    ProtocolError,
    HandshakeError,
    BaseClientHandshake,
    AuthenticatedClientHandshake,
    InvitedClientHandshake,
    APIV1_AnonymousClientHandshake,
    HandshakeOutOfBallparkError,
)
from parsec.core.types import (
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
    BackendInvitationAddr,
)
from parsec.core.backend_connection.exceptions import (
    BackendNotAvailable,
    BackendConnectionRefused,
    BackendInvitationAlreadyUsed,
    BackendInvitationNotFound,
    BackendOutOfBallparkError,
    BackendProtocolError,
)
from parsec.core.backend_connection.proxy import maybe_connect_through_proxy, blocking_io_get_proxy


logger = get_logger()


async def apiv1_connect(
    addr: BackendOrganizationBootstrapAddr, keepalive: Optional[int] = None
) -> Transport:
    """
    Raises:
        BackendConnectionError
    """
    handshake = APIV1_AnonymousClientHandshake(addr.organization_id)
    return await _connect(addr.hostname, addr.port, addr.use_ssl, keepalive, handshake)


async def connect_as_invited(
    addr: BackendInvitationAddr, keepalive: Optional[int] = None
) -> Transport:
    handshake = InvitedClientHandshake(
        organization_id=addr.organization_id, invitation_type=addr.invitation_type, token=addr.token
    )
    return await _connect(addr.hostname, addr.port, addr.use_ssl, keepalive, handshake)


async def connect_as_authenticated(
    addr: BackendOrganizationAddr,
    device_id: DeviceID,
    signing_key: SigningKey,
    keepalive: Optional[int] = None,
) -> Transport:
    handshake = AuthenticatedClientHandshake(
        organization_id=addr.organization_id,
        device_id=device_id,
        user_signkey=signing_key,
        root_verify_key=addr.root_verify_key,
    )
    return await _connect(addr.hostname, addr.port, addr.use_ssl, keepalive, handshake)


async def _connect(
    hostname: str,
    port: int,
    use_ssl: bool,
    keepalive: Optional[int],
    handshake: BaseClientHandshake,
) -> Transport:
    stream = await maybe_connect_through_proxy(hostname, port, use_ssl)

    if not stream:
        logger.debug("Using direct (no proxy) connection", hostname=hostname, port=port)
        try:
            stream = await trio.open_tcp_stream(hostname, port)

        except OSError as exc:
            logger.debug(
                "Impossible to connect to backend", hostname=hostname, port=port, exc_info=exc
            )
            raise BackendNotAvailable(exc) from exc

    if use_ssl:
        logger.debug("Using TLS to secure connection", hostname=hostname)
        stream = _upgrade_stream_to_ssl(stream, hostname)

    try:
        transport = await Transport.init_for_client(stream, host=hostname)
        transport.keepalive = keepalive

    except TransportError as exc:
        logger.warning("Connection lost during transport creation", exc_info=exc)
        raise BackendNotAvailable(exc) from exc

    try:
        await _do_handshake(transport, handshake)

    except BackendOutOfBallparkError:
        transport.logger.info("Abort handshake due to the system clock being out of sync")
        await transport.aclose()
        raise

    except Exception as exc:
        transport.logger.warning("Connection lost during handshake", exc_info=exc)
        await transport.aclose()
        raise

    return transport


def _upgrade_stream_to_ssl(raw_stream: trio.abc.Stream, hostname: str) -> trio.abc.Stream:
    # The ssl context should be generated once and stored into the config
    # however this is tricky (should ssl configuration be stored per device ?)

    # Don't load default system certificates and rely on our own instead.
    # This is because system certificates are less reliable (and system
    # certificates are tried first, so they can lead to a failure even if
    # we bundle a valid certificate...)
    # Certifi provides Mozilla's carefully curated collection of Root Certificates.
    ssl_context = ssl.create_default_context(
        purpose=ssl.Purpose.SERVER_AUTH, cadata=certifi.contents()
    )

    # Also provide custom certificates if any
    cafile = os.environ.get("SSL_CAFILE")
    if cafile:
        ssl_context.load_verify_locations(cafile)

    return trio.SSLStream(raw_stream, ssl_context, server_hostname=hostname)


async def _do_handshake(transport: Transport, handshake: BaseClientHandshake) -> None:
    try:
        challenge_req = await transport.recv()
        answer_req = handshake.process_challenge_req(challenge_req)
        await transport.send(answer_req)
        result_req = await transport.recv()
        handshake.process_result_req(result_req)

    except TransportError as exc:
        raise BackendNotAvailable(exc) from exc

    except HandshakeOutOfBallparkError as exc:
        raise BackendOutOfBallparkError(exc) from exc

    except HandshakeError as exc:
        if str(exc) == "Invalid handshake: Invitation not found":
            raise BackendInvitationNotFound(str(exc)) from exc
        elif str(exc) == "Invalid handshake: Invitation already deleted":
            raise BackendInvitationAlreadyUsed(str(exc)) from exc
        else:
            raise BackendConnectionRefused(str(exc)) from exc

    except ProtocolError as exc:
        transport.logger.exception("Protocol error during handshake")
        raise BackendProtocolError(exc) from exc


class TransportPool:
    def __init__(self, connect_cb: Callable[[], Awaitable[Transport]], max_pool: int):
        self._connect_cb = connect_cb
        self._transports = []
        self._closed = False
        self._lock = trio.Semaphore(max_pool)

    @asynccontextmanager
    async def acquire(self, force_fresh: bool = False) -> AsyncContextManager[Transport]:
        """
        Raises:
            BackendConnectionError
            trio.ClosedResourceError: if used after having being closed
        """
        async with self._lock:
            transport = None
            if not force_fresh:
                try:
                    # Fifo style to retrieve oldest first
                    transport = self._transports.pop(0)
                except IndexError:
                    pass

            if not transport:
                if self._closed:
                    raise trio.ClosedResourceError()

                transport = await self._connect_cb()

            try:
                yield transport

            except TransportClosedByPeer:
                raise

            except Exception:
                await transport.aclose()
                raise

            else:
                self._transports.append(transport)


async def http_request(
    url: str,
    data: Optional[bytes] = None,
    headers: Dict[str, str] = {},
    method: Optional[str] = None,
) -> bytes:
    """Raises: urllib.error.URLError or OSError"""

    def _target():
        request = urllib.request.Request(url, data=data, headers=headers, method=method)
        handlers = []
        proxy_url = blocking_io_get_proxy(target_url=url, hostname=request.host)
        if proxy_url:
            logger.debug("Using proxy to connect", proxy_url=proxy_url, target_url=url)
            handlers.append(urllib.request.ProxyHandler(proxies={request.type: proxy_url}))
        else:
            # Prevent urllib from trying to handle proxy configuration by itself.
            # We use `blocking_io_get_proxy` to give us the proxy to use
            # (or, in our case, the fact we should not use proxy).
            # However this information may come from a weird place (e.g. PAC config)
            # that urllib is not aware of, so urllib may use an invalid configuration
            # (e.g. using HTTP_PROXY env var) if we let it handle proxy.
            handlers.append(urllib.request.ProxyHandler(proxies={}))

        if request.type == "https":
            context = ssl.create_default_context(cafile=certifi.where())
            # Also provide custom certificates if any
            cafile = os.environ.get("SSL_CAFILE")
            if cafile:
                context.load_verify_locations(cafile)
            handlers.append(urllib.request.HTTPSHandler(context=context))

        with urllib.request.build_opener(*handlers).open(request) as rep:
            return rep.read()

    return await trio.to_thread.run_sync(_target)

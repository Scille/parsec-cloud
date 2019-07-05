# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import trio
from async_generator import asynccontextmanager
from structlog import get_logger
from typing import Optional, Union

from parsec.types import (
    DeviceID,
    BackendAddr,
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
)
from parsec.crypto import SigningKey
from parsec.api.transport import Transport, TransportError, TransportClosedByPeer
from parsec.api.protocole import (
    ProtocoleError,
    HandshakeRevokedDevice,
    HandshakeAPIVersionError,
    AnonymousClientHandshake,
    AuthenticatedClientHandshake,
    AdministrationClientHandshake,
)
from parsec.core.backend_connection.exceptions import (
    BackendConnectionError,
    BackendNotAvailable,
    BackendIncompatibleVersion,
    BackendHandshakeError,
    BackendHandshakeAPIVersionError,
    BackendDeviceRevokedError,
)


__all__ = (
    "anonymous_transport_factory",
    "administration_transport_factory",
    "authenticated_transport_pool_factory",
    "AuthenticatedTransportPool",
)


logger = get_logger()


async def _connect(
    addr: Union[BackendAddr, BackendOrganizationBootstrapAddr, BackendOrganizationAddr],
    device_id: Optional[DeviceID] = None,
    signing_key: Optional[SigningKey] = None,
    administration_token: Optional[str] = None,
):
    """
    Raises:
        BackendConnectionError
        BackendNotAvailable
        BackendIncompatibleVersion
        BackendHandshakeError
        BackendDeviceRevokedError
    """
    if administration_token:
        if not isinstance(addr, BackendAddr):
            raise BackendConnectionError(f"Invalid url format `{addr}`")
        ch = AdministrationClientHandshake(administration_token)

    elif not device_id:
        if isinstance(addr, BackendOrganizationBootstrapAddr):
            ch = AnonymousClientHandshake(addr.organization_id)
        elif isinstance(addr, BackendOrganizationAddr):
            ch = AnonymousClientHandshake(addr.organization_id, addr.root_verify_key)
        else:
            raise BackendConnectionError(
                f"Invalid url format `{addr}` "
                "(should be an organization url or organization bootstrap url)"
            )

    else:
        if not isinstance(addr, BackendOrganizationAddr):
            raise BackendConnectionError(
                f"Invalid url format `{addr}` (should be an organization url)"
            )

        if not signing_key:
            raise BackendConnectionError(f"Missing signing_key to connect as `{device_id}`")
        ch = AuthenticatedClientHandshake(
            addr.organization_id, device_id, signing_key, addr.root_verify_key
        )

    try:
        stream = await trio.open_tcp_stream(addr.hostname, addr.port)

    except OSError as exc:
        logger.debug("Impossible to connect to backend", reason=exc)
        raise BackendNotAvailable(exc) from exc

    if addr.scheme == "parsecs":
        stream = _upgrade_stream_to_ssl(stream, addr.hostname)

    try:
        transport = await Transport.init_for_client(stream, addr.hostname)
    except TransportError as exc:
        logger.debug("Connection lost during transport creation", reason=exc)
        raise BackendNotAvailable(exc) from exc

    try:
        await _do_handshade(transport, ch)

    except BackendHandshakeAPIVersionError as exc:
        logger.debug("Incompatible API version", reason=f"Server API version {str(exc)}")
        raise BackendIncompatibleVersion(exc) from exc

    except Exception as exc:
        transport.logger.debug("Connection lost during handshake", reason=exc)
        await transport.aclose()
        raise

    return transport


def _upgrade_stream_to_ssl(raw_stream, hostname):
    # The ssl context should be generated once and stored into the config
    # however this is tricky (should ssl configuration be stored per device ?)
    keyfile = os.environ.get("SSL_KEYFILE")
    certfile = os.environ.get("SSL_CERTFILE")

    ssl_context = trio.ssl.create_default_context(trio.ssl.Purpose.CLIENT_AUTH)
    if certfile:
        ssl_context.load_cert_chain(certfile, keyfile)
    else:
        ssl_context.load_default_certs()

    return trio.ssl.SSLStream(raw_stream, ssl_context, server_hostname=hostname)


async def _do_handshade(transport: Transport, ch):
    try:
        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)
        await transport.send(answer_req)
        result_req = await transport.recv()
        ch.process_result_req(result_req)
        transport.logger.debug("Handshake done")

    except TransportError as exc:
        raise BackendNotAvailable(exc) from exc

    except HandshakeAPIVersionError as exc:
        transport.logger.debug("Handshake failed", reason=exc)
        raise BackendHandshakeAPIVersionError(exc) from exc

    except HandshakeRevokedDevice as exc:
        transport.logger.warning("Handshake failed", reason=exc)
        raise BackendDeviceRevokedError(exc) from exc

    except ProtocoleError as exc:
        transport.logger.warning("Handshake failed", reason=exc)
        raise BackendHandshakeError(exc) from exc


@asynccontextmanager
async def anonymous_transport_factory(addr: BackendOrganizationAddr) -> Transport:
    """
    Raises:
        BackendConnectionError
        BackendNotAvailable
        BackendIncompatibleVersion
        BackendHandshakeError
        BackendDeviceRevokedError
    """
    transport = await _connect(addr)
    transport.logger = transport.logger.bind(auth="<anonymous>")
    try:
        yield transport

    finally:
        await transport.aclose()


@asynccontextmanager
async def administration_transport_factory(addr: BackendAddr, token: str) -> Transport:
    """
    Raises:
        BackendConnectionError
        BackendNotAvailable
        BackendIncompatibleVersion
        BackendHandshakeError
        BackendDeviceRevokedError
    """
    transport = await _connect(addr, administration_token=token)
    transport.logger = transport.logger.bind(auth="<anonymous>")
    try:
        yield transport

    finally:
        await transport.aclose()


class AuthenticatedTransportPool:
    def __init__(self, addr, device_id, signing_key, max_pool, keepalive_time):
        self.addr = addr
        self.device_id = device_id
        self.signing_key = signing_key
        self.keepalive_time = keepalive_time
        self.transports = []
        self._closed = False
        self._lock = trio.Semaphore(max_pool)

    @asynccontextmanager
    async def acquire(self, force_fresh=False):
        """
        Raises:
            BackendConnectionError
            BackendNotAvailable
            BackendIncompatibleVersion
            BackendHandshakeError
            BackendDeviceRevokedError
            trio.ClosedResourceError: if used after having being closed
        """
        async with self._lock:
            transport = None
            if not force_fresh:
                try:
                    # Fifo style to retreive oldest first
                    transport = self.transports.pop(0)
                except IndexError:
                    pass

            if not transport:
                if self._closed:
                    raise trio.ClosedResourceError()

                transport = await _connect(self.addr, self.device_id, self.signing_key)
                transport.keepalive_time = self.keepalive_time
                transport.logger = transport.logger.bind(device_id=self.device_id)

            try:
                yield transport

            except TransportClosedByPeer:
                raise

            except Exception:
                await transport.aclose()
                raise

            else:
                self.transports.append(transport)


@asynccontextmanager
async def authenticated_transport_pool_factory(
    addr: BackendOrganizationAddr,
    device_id: DeviceID,
    signing_key: SigningKey,
    max_pool: int = 4,
    keepalive_time: int = 30,
) -> AuthenticatedTransportPool:
    """
    Raises: nothing !
    """
    pool = AuthenticatedTransportPool(addr, device_id, signing_key, max_pool, keepalive_time)
    try:
        yield pool

    finally:
        pool._closed = True
        async with trio.open_nursery() as nursery:
            for transport in pool.transports:
                nursery.start_soon(transport.aclose)

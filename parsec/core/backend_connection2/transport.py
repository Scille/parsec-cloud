import trio
from uuid import uuid4
from async_generator import asynccontextmanager
from structlog import get_logger
from urllib.parse import urlparse

from parsec.types import DeviceID
from parsec.crypto import SigningKey
from parsec.api.transport import BaseTransport, TransportError, PatateTCPTransport
from parsec.api.protocole import (
    ProtocoleError,
    HandshakeRevokedDevice,
    AnonymousClientHandshake,
    ClientHandshake,
)
from parsec.core.backend_connection2.exceptions import (
    BackendNotAvailable,
    BackendHandshakeError,
    BackendDeviceRevokedError,
)


__all__ = (
    "authenticated_transport_factory",
    "anonymous_transport_factory",
    "transport_pool_factory",
    "TransportPool",
)


logger = get_logger()


async def _transport_factory(addr: str) -> BaseTransport:
    # TODO: handle ssl and websocket here
    parsed_addr = urlparse(addr)
    try:
        stream = await trio.open_tcp_stream(parsed_addr.hostname, parsed_addr.port)
        return PatateTCPTransport(stream)

    except OSError as exc:
        logger.debug("Impossible to connect to backend", reason=exc)
        raise BackendNotAvailable() from exc


async def _do_handshade(
    transport: BaseTransport, device_id: DeviceID = None, signing_key: SigningKey = None
):
    if device_id and not signing_key:
        raise ValueError("Signing key is mandatory for non anonymous authentication")

    try:
        if not device_id:
            ch = AnonymousClientHandshake()
        else:
            ch = ClientHandshake(device_id, signing_key)
        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)
        await transport.send(answer_req)
        result_req = await transport.recv()
        ch.process_result_req(result_req)
        transport.log.debug("Connected")

    except TransportError as exc:
        transport.log.debug("Connection lost during handshake", reason=exc)
        await transport.aclose()
        raise BackendNotAvailable() from exc

    except HandshakeRevokedDevice as exc:
        transport.log.warning("Handshake failed", reason=exc)
        await transport.aclose()
        raise BackendDeviceRevokedError() from exc

    except ProtocoleError as exc:
        transport.log.warning("Handshake failed", reason=exc)
        await transport.aclose()
        raise BackendHandshakeError() from exc


async def _authenticated_transport_factory(
    addr: str, device_id: DeviceID, signing_key: SigningKey
) -> BaseTransport:
    transport = await _transport_factory(addr)
    # TODO: a bit ugly to configure and connect a logger here,
    # use contextvar instead ?
    transport.log = logger.bind(addr=addr, auth=device_id, id=uuid4().hex)
    try:
        await _do_handshade(transport, device_id, signing_key)

    except:
        await transport.aclose()
        raise

    else:
        return transport


@asynccontextmanager
async def authenticated_transport_factory(
    addr: str, device_id: DeviceID, signing_key: SigningKey
) -> BaseTransport:
    transport = await _authenticated_transport_factory(addr, device_id, signing_key)
    try:
        yield transport

    finally:
        await transport.aclose()


@asynccontextmanager
async def anonymous_transport_factory(addr: str) -> BaseTransport:
    transport = await _transport_factory(addr)
    transport.log = logger.bind(addr=addr, auth="<anonymous>", id=uuid4().hex)
    await _do_handshade(transport)
    try:
        yield transport

    finally:
        await transport.aclose()


class TransportPool:
    def __init__(self, addr, device_id, signing_key, max):
        self.addr = addr
        self.device_id = device_id
        self.signing_key = signing_key
        self.transports = []
        self._closed = False
        self._lock = trio.Semaphore(max)

    @asynccontextmanager
    async def acquire(self, force_fresh=False):
        async with self._lock:
            transport = None
            if not force_fresh:
                try:
                    transport = self.transports.pop()
                except IndexError:
                    pass
            if not transport:
                if self._closed:
                    raise trio.ClosedResourceError()

                transport = await _authenticated_transport_factory(
                    self.addr, self.device_id, self.signing_key
                )

            try:
                yield transport

            except TransportError:
                await transport.aclose()
                raise

            else:
                self.transports.append(transport)


@asynccontextmanager
async def transport_pool_factory(
    addr: str, device_id: DeviceID, signing_key: SigningKey, max: int = 4
) -> TransportPool:
    pool = TransportPool(addr, device_id, signing_key, max)
    try:
        yield pool

    finally:
        pool._closed = True
        for transport in pool.transports:
            try:
                await transport.aclose()
            except TransportError:
                pass

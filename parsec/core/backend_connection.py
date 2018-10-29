from async_generator import asynccontextmanager
from queue import Queue, Empty, Full
import trio
from structlog import get_logger
from urllib.parse import urlparse

from parsec.networking import CookedSocket
from parsec.handshake import (
    AnonymousClientHandshake,
    ClientHandshake,
    HandshakeBadIdentity,
    HandshakeError,
)


logger = get_logger()


class BackendError(Exception):
    pass


class BackendNotAvailable(BackendError):
    pass


async def backend_connection_factory(addr: str, auth_id=None, auth_signkey=None) -> CookedSocket:
    """
    Connect and authenticate to the given backend.

    Args:
        addr: Address of the backend.
        auth_id: Device ID to authenticate as. If left to None, anonymous
                 authentifaction will be performed.
        auth_signkey: Device signkey to use for authentication

    Raises:
        BackendNotAvailable: if connection with backend failed.
        HandshakeError: if handshake failed.

    Returns:
        A cooked socket ready to communicate with the backend.
    """
    if auth_id and not auth_signkey:
        raise ValueError("Signing key is mandatory for non anonymous authentication")

    log = logger.bind(addr=addr, auth=auth_id or "<anonymous>")

    parsed_addr = urlparse(addr)
    try:
        sockstream = await trio.open_tcp_stream(parsed_addr.hostname, parsed_addr.port)

    except OSError as exc:
        log.debug("Impossible to connect to backend", reason=exc)
        raise BackendNotAvailable() from exc

    try:
        sock = CookedSocket(sockstream)
        if not auth_id:
            ch = AnonymousClientHandshake()
        else:
            ch = ClientHandshake(auth_id, auth_signkey)
        challenge_req = await sock.recv()
        answer_req = ch.process_challenge_req(challenge_req)
        await sock.send(answer_req)
        result_req = await sock.recv()
        ch.process_result_req(result_req)
        log.debug("Connected")

    except trio.BrokenStreamError as exc:
        log.debug("Connection lost during handshake", reason=exc)
        await sockstream.aclose()
        raise BackendNotAvailable() from exc

    except HandshakeError as exc:
        log.warning("Handshake failed", reason=exc)
        await sockstream.aclose()
        raise

    return sock


async def backend_send_anonymous_cmd(addr: str, req: dict) -> dict:
    """
    Send a single request to the backend as anonymous user.

    Args:
        addr: Address of the backend.
        req: Request data to send.

    Raises:
        BackendNotAvailable: if connection with backend failed.
        HandshakeError: if handshake failed.
        TypeError: if provided msg is not a valid JSON serializable object.
        json.JSONDecodeError: if the backend answer is not a valid json.

    Returns:
        The backend reponse deserialized as a dict.
    """
    sock = await backend_connection_factory(addr)
    # TODO: avoid this hack by splitting BackendConnection functions
    try:
        await sock.send(req)
        return await sock.recv()

    except trio.BrokenStreamError as exc:
        raise BackendNotAvailable() from exc

    finally:
        await sock.aclose()


class BackendConnectionPool:
    def __init__(self, addr, auth_id, auth_signkey, max_connections=100):
        self.addr = addr
        self.auth_id = auth_id
        self.auth_signkey = auth_signkey
        self.max_connections = max_connections
        self.created_connections = 0
        self.pool = Queue(self.max_connections)
        self.semaphore = trio.Semaphore(self.max_connections)

    def size(self):
        return self.created_connections

    async def make_connection(self):
        connection = await backend_connection_factory(self.addr, self.auth_id, self.auth_signkey)
        self.created_connections += 1
        return connection

    def release(self, connection):
        try:
            self.pool.put_nowait(connection)
        except Full:
            pass

    async def disconnect(self):
        try:
            for connection in iter(self.pool.get_nowait, None):
                try:
                    await connection.aclose()
                except Exception:
                    pass
        except Empty:
            pass
        self.created_connections = 0
        self.semaphore = trio.Semaphore(self.max_connections)

    @asynccontextmanager
    async def connection(self, fresh=False):
        connection = None
        await self.semaphore.acquire()
        if not fresh:
            try:
                connection = self.pool.get_nowait()
            except Empty:
                pass
        if not connection:
            try:
                connection = await self.make_connection()
            except HandshakeBadIdentity as exc:
                # TODO: think about the handling of this kind of exception...
                self.created_connections -= 1
                self.semaphore.release()
                raise BackendNotAvailable() from exc
        try:
            yield connection
        except (trio.TooSlowError, trio.BrokenStreamError, trio.ClosedStreamError) as exc:
            await connection.aclose()
            self.created_connections -= 1
            self.semaphore.release()
            raise BackendNotAvailable() from exc
        else:
            self.semaphore.release()
            self.release(connection)

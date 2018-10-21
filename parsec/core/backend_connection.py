from async_generator import asynccontextmanager
from queue import LifoQueue, Empty, Full
import trio
from structlog import get_logger
from urllib.parse import urlparse

from parsec.networking import CookedSocket
from parsec.handshake import ClientHandshake, AnonymousClientHandshake, HandshakeError


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


class ConnectionPool:
    def __init__(self, connection_factory, min_connections=0, max_connections=1000, timeout=None):
        self.connection_factory = connection_factory
        self.min_connections = min_connections
        self.max_connections = max_connections
        self.timeout = timeout
        self.pool = None
        self.created_connections = 0

    async def init(self):
        await self.reset()

    def size(self):
        return self.created_connections

    async def reset(self):
        self.created_connections = 0
        self.pool = LifoQueue(self.max_connections)
        for _ in range(self.max_connections - self.min_connections):
            try:
                self.pool.put_nowait(None)
            except Full:
                break
        for _ in range(self.min_connections):
            connection = await self.make_connection()
            self.pool.put_nowait(connection)

    async def make_connection(self):
        connection = await self.connection_factory()
        self.created_connections += 1
        return connection

    def release(self, connection):
        if self.pool:
            try:
                self.pool.put_nowait(connection)
            except Full:
                pass

    async def disconnect(self):
        if self.pool:
            for connection in iter(self.pool.get, None):
                await connection.aclose()
        self.created_connections = 0
        self.pool = None

    @asynccontextmanager
    async def connection(self):
        if not self.pool:
            await self.init()

        connection = None
        try:
            connection = self.pool.get(block=True, timeout=self.timeout)
        except Empty:
            raise ConnectionError("No connection available.")

        if connection is None:
            if self.created_connections >= self.max_connections:
                raise ConnectionError("Too many connections")
            connection = await self.make_connection()

        try:
            yield connection
        except (trio.TooSlowError, trio.BrokenStreamError, trio.ClosedStreamError) as exc:
            await connection.aclose()
            connection = await self.make_connection()
            raise BackendNotAvailable() from exc
        finally:
            self.release(connection)

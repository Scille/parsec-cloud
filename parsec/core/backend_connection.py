import trio
from structlog import get_logger
from urllib.parse import urlparse

from parsec.cert import CA
from parsec.networking import CookedSocket, client_cooked_socket_factory
from parsec.handshake import ClientHandshake, AnonymousClientHandshake, HandshakeError


logger = get_logger()


class BackendError(Exception):
    pass


class BackendNotAvailable(BackendError):
    pass


def upgrade_stream_to_ssl(raw_stream, hostname):
    ssl_context = trio.ssl.create_default_context()
    CA.configure_trust(ssl_context)
    return trio.ssl.SSLStream(raw_stream, ssl_context, server_hostname=hostname)


async def backend_connection_factory(
    addr: str, auth_id=None, auth_signkey=None, ssl=False
) -> CookedSocket:
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
        conn = await trio.open_tcp_stream(parsed_addr.hostname, parsed_addr.port)

    except OSError as exc:
        log.debug("Impossible to connect to backend", reason=exc)
        raise BackendNotAvailable() from exc

    if parsed_addr.scheme == "wss":
        conn = upgrade_stream_to_ssl(conn, parsed_addr.hostname)

    try:
        sock = client_cooked_socket_factory(conn, parsed_addr.hostname)
        await sock.init()
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
        raise BackendNotAvailable() from exc

    except HandshakeError as exc:
        log.warning("Handshake failed", reason=exc)
        await sock.aclose()
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

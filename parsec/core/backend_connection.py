import trio
import logbook
from urllib.parse import urlparse

from parsec.networking import CookedSocket
from parsec.handshake import ClientHandshake, AnonymousClientHandshake, HandshakeError


logger = logbook.Logger("parsec.core.backend_connection")


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

    parsed_addr = urlparse(addr)
    logger.debug(
        "Connecting to backend {}:{} as {}",
        parsed_addr.hostname,
        parsed_addr.port,
        auth_id or "<anonymous>",
    )

    try:
        sockstream = await trio.open_tcp_stream(parsed_addr.hostname, parsed_addr.port)

    except OSError as exc:
        logger.debug("Impossible to connect to backend: {!r}", exc)
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

    except trio.BrokenStreamError as exc:
        logger.debug("Connection with backend lost during handshake: {!r}", exc)
        await sockstream.aclose()
        raise BackendNotAvailable() from exc

    except HandshakeError as exc:
        logger.warning("Handshake failed: {!r}", exc)
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

import trio
import logbook

from parsec.core.base import BaseAsyncComponent
from parsec.core.backend_connection import BackendNotAvailable, backend_connection_factory
from parsec.core.devices_manager import Device
from parsec.handshake import HandshakeBadIdentity


logger = logbook.Logger("parsec.core.backend_cmds_sender")


class BackendCmdsSender(BaseAsyncComponent):
    def __init__(self, device: Device, backend_addr: str):
        super().__init__()
        self.device = device
        self.backend_addr = backend_addr
        self._sock = None

    async def _init(self, nursery):
        # TODO: Try to open connection with the backend on a dedicated
        # coroutine to save time for first request ?
        # TODO: setup a connection pool for concurrent requests ?
        pass

    async def _teardown(self):
        if self._sock:
            await self._sock.aclose()

    async def _init_send_connection(self):
        if self._sock:
            await self._sock.aclose()
        try:
            self._sock = await backend_connection_factory(
                self.backend_addr, self.device.id, self.device.device_signkey
            )
        except HandshakeBadIdentity as exc:
            # TODO: think about the handling of this kind of exception...
            raise BackendNotAvailable() from exc

    async def _naive_send(self, req):
        if not self._sock:
            raise BackendNotAvailable()

        try:
            await self._sock.send(req)
            return await self._sock.recv()

        except (trio.BrokenStreamError, trio.ClosedStreamError) as exc:
            raise BackendNotAvailable() from exc

    async def send(self, req: dict) -> dict:
        """
        Send a request to the backend.

        Args:
            req: Request data to send.

        Raises:
            BackendNotAvailable: if connection with backend failed.
            HandshakeError: if handshake failed.
            TypeError: if provided msg is not a valid JSON serializable object.
            json.JSONDecodeError: if the backend answer is not a valid json.

        Returns:
            The backend reponse deserialized as a dict.
        """

        async with self._lock:
            # Try to use the already connected socket
            try:
                logger.debug("send {}", req)
                rep = await self._naive_send(req)
                logger.debug("recv {}", rep)
                return rep

            except BackendNotAvailable as exc:
                logger.debug("retrying, cannot reach backend: {!r}", exc)
                try:
                    # If it failed, reopen the socket and retry the request
                    await self._init_send_connection()
                    logger.debug("send {}", req)
                    rep = await self._naive_send(req)
                    logger.debug("recv {}", rep)
                    return rep

                except BackendNotAvailable as e:
                    logger.debug("aborting, cannot reach backend: {!r}", e)
                    # Failed again, it seems we are offline
                    raise

    async def ping(self):
        await self.send({"cmd": "ping", "ping": ""})

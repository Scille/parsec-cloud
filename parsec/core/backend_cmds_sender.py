import trio
from uuid import uuid4
from structlog import get_logger

from parsec.core.base import BaseAsyncComponent
from parsec.core.backend_connection import BackendNotAvailable, BackendConnectionPool
from parsec.core.devices_manager import Device
from parsec.handshake import HandshakeBadIdentity


PER_CMD_TIMEOUT = 30
logger = get_logger()


class BackendCmdsSender(BaseAsyncComponent):
    def __init__(self, device: Device, backend_addr: str):
        super().__init__()
        self.device = device
        self.backend_addr = backend_addr

    async def _init(self, nursery):
        self.connection_pool = BackendConnectionPool(
            self.backend_addr, self.device.id, self.device.device_signkey
        )

    async def _teardown(self):
        await self.connection_pool.disconnect()

    async def _naive_send(self, connection, req):
        # This timeout is a bit tricky: on one hand choosing a small value
        # makes poor connection unusable, but on the other hand a tcp socket
        # can hang for a long, long time (for instance when the network is
        # shutdown after a connection has been setup).
        # This is especially a trouble with requests going through FUSE
        # given they must be all finished before the unmount, with the GUI
        # doing a frozen wait for all this time...
        with trio.fail_after(PER_CMD_TIMEOUT):
            await connection.send(req)
            return await connection.recv()

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

        # TODO: Should find a way to avoid using this filder if we're not in log debug...
        def _filter_big_fields(data):
            # As hacky as arbitrary... but works well so far !
            filtered_data = data.copy()
            try:
                if len(data["block"]) > 200:
                    filtered_data["block"] = f"{data['block'][:100]}[...]{data['block'][-100:]}"
            except (KeyError, ValueError, TypeError):
                pass
            try:
                if len(data["blob"]) > 200:
                    filtered_data["blob"] = f"{data['blob'][:100]}[...]{data['blob'][-100:]}"
            except (KeyError, ValueError, TypeError):
                pass
            return filtered_data

        log = logger.bind(req_id=uuid4().hex)
        try:
            async with self.connection_pool.connection() as connection:
                # Try to use the already connected socket
                log.debug("Sending request", req=_filter_big_fields(req))
                rep = await self._naive_send(connection, req)
                log.debug("Receiving response", rep=_filter_big_fields(rep))
                return rep

        except BackendNotAvailable as exc:
            log.debug("Cannot reach backend, retrying", reason=exc)
            try:
                async with self.connection_pool.connection(fresh=True) as connection:
                    # If it failed, reopen the socket and retry the request
                    log.debug("Sending request", req=_filter_big_fields(req))
                    rep = await self._naive_send(connection, req)
                    log.debug("Receiving response", rep=_filter_big_fields(rep))
                    return rep

            except BackendNotAvailable as e:
                log.debug("Cannot reach backend, aborting", reason=exc)
                # Failed again, it seems we are offline
                raise

    async def ping(self):
        await self.send({"cmd": "ping", "ping": ""})

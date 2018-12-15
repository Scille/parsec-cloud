from structlog import get_logger
from async_generator import asynccontextmanager

from parsec.types import DeviceID
from parsec.crypto import SigningKey
from parsec.core.backend_connection.exceptions import BackendNotAvailable
from parsec.core.backend_connection.transport import transport_pool_factory, TransportPool
from parsec.core.backend_connection.cmds import BackendCmds


__all__ = ("backend_cmds_pool_factory", "BackendCmdsPool")


logger = get_logger()


# TODO: create&use a BaseBackendCmds for the inheritance
class BackendCmdsPool(BackendCmds):
    def __init__(self, transport_pool: TransportPool, log=None):
        self.transport_pool = transport_pool
        # TODO: use logger...
        self.log = log or logger

    def _expose_cmds_with_retrier(name):
        async def wrapper(self, *args, **kwargs):
            try:
                async with self.transport_pool.acquire() as transport:
                    cmds = getattr(transport, "cmds", None)
                    if not cmds:
                        cmds = BackendCmds(transport, transport.log)
                        transport.cmds = cmds

                    return await getattr(cmds, name)(*args, **kwargs)

            except BackendNotAvailable as exc:
                async with self.transport_pool.acquire(force_fresh=True) as transport:
                    cmds = BackendCmds(transport, transport.log)
                    transport.cmds = cmds

                    return await getattr(cmds, name)(*args, **kwargs)

        wrapper.__name__ = name

        return wrapper

    ping = _expose_cmds_with_retrier("ping")

    user_get = _expose_cmds_with_retrier("user_get")
    user_find = _expose_cmds_with_retrier("user_find")
    user_invite = _expose_cmds_with_retrier("user_invite")
    user_cancel_invitation = _expose_cmds_with_retrier("user_cancel_invitation")
    user_create = _expose_cmds_with_retrier("user_create")

    device_invite = _expose_cmds_with_retrier("device_invite")
    device_cancel_invitation = _expose_cmds_with_retrier("device_cancel_invitation")
    device_create = _expose_cmds_with_retrier("device_create")
    device_revoke = _expose_cmds_with_retrier("device_revoke")


@asynccontextmanager
async def backend_cmds_pool_factory(
    addr: str, device_id: DeviceID, signing_key: SigningKey, max: int = 4
) -> BackendCmdsPool:
    async with transport_pool_factory(addr, device_id, signing_key, max) as transport_pool:
        yield BackendCmdsPool(transport_pool)

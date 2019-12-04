# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from structlog import get_logger
from typing import Optional

from parsec.event_bus import EventBus
from parsec.api.transport import TransportError
from parsec.core.types import LocalDevice, EntryID
from parsec.core.backend_connection.exceptions import (
    BackendNotAvailable,
    BackendConnectionError,
    BackendIncompatibleVersion,
    BackendNotAvailableRVKMismatch,
    BackendHandshakeAPIVersionError,
    BackendHandshakeRVKMismatchError,
    BackendDeviceRevokedError,
)
from parsec.core.backend_connection.porcelain import backend_cmds_pool_factory


MAX_COOLDOWN = 30
logger = get_logger()


class BackendEventsManager:
    def __init__(self, device: LocalDevice, event_bus: EventBus, keepalive: Optional[int]):
        self.device = device
        self.event_bus = event_bus
        self.keepalive = keepalive
        self._backend_incompatible_version = False
        self._backend_rvk_mismatch = False
        self._backend_user_revoked = False
        self._backend_connection_failures = 0

    def _event_pump_incompatible_version(self):
        if self._backend_incompatible_version is not True:
            self._backend_incompatible_version = True
            self.event_bus.send("backend.incompatible_version")
        # Send "incompatible_version" before "offline" so it can be read correctly
        self._event_pump_lost()

    def _event_pump_rvk_mismatch(self):
        if self._backend_rvk_mismatch is not True:
            self._backend_rvk_mismatch = True
            self.event_bus.send("backend.rvk_mismatch")
        # Send "rvk_mismatch" before "offline" so it can be read correctly
        self._event_pump_lost()

    def _event_pump_user_revoked(self):
        if self._backend_user_revoked is not True:
            self._backend_user_revoked = True
            self.event_bus.send("backend.user_revoked")
        # Send "user_revoked" before "offline" so it can be read correctly
        self._event_pump_lost()

    def _event_pump_lost(self):
        self._backend_connection_failures += 1
        # Only send the offline even when we have just lost the connection
        if self._backend_connection_failures == 1:
            self.event_bus.send("backend.offline")

    def _event_pump_ready(self):
        self._backend_incompatible_version = False
        self._backend_rvk_mismatch = False
        self._backend_user_revoked = False
        self._backend_connection_failures = 0
        self.event_bus.send("backend.online")
        # Given the backend won't notify us for messages that arrived while
        # we were offline, we must actively check this ourself.
        self.event_bus.send("backend.message.polling_needed")

    async def _event_listener_manager(self):
        while True:
            try:
                logger.info("Try to connect to backend...")
                await self._event_pump()

            except (BackendHandshakeRVKMismatchError, BackendNotAvailableRVKMismatch) as exc:
                # No need to retry, given these keys wont change...
                self._event_pump_rvk_mismatch()
                logger.info(f"Cannot connect to backend : RVK mismatch {str(exc)}")
                return

            except (BackendHandshakeAPIVersionError, BackendIncompatibleVersion) as exc:
                # No need to retry, client should update or wait for backend update first
                self._event_pump_incompatible_version()
                logger.info("Cannot connect to backend: incompatible version", exc_info=exc)
                return

            except BackendDeviceRevokedError as exc:
                # No need to retry, backend no longer wants to talk to us :'(
                self._event_pump_user_revoked()
                logger.info("Cannot connect to backend: user revoked", exc_info=exc)
                return

            except (TransportError, BackendNotAvailable) as exc:
                # In case of connection failure, wait a bit and restart
                self._event_pump_lost()
                cooldown_time = 2 ** self._backend_connection_failures
                if cooldown_time > MAX_COOLDOWN:
                    cooldown_time = MAX_COOLDOWN
                logger.info("Backend offline", reason=exc, cooldown_time=cooldown_time)
                await trio.sleep(cooldown_time)

            except BackendConnectionError:
                # Unexpected error from the backend, don't retry to avoid flooding
                # with error logs (retry will be done at next login anyway)
                self._event_pump_lost()
                logger.exception("Invalid response sent by backend")
                return

    async def _event_pump(self):
        async with backend_cmds_pool_factory(
            self.device.organization_addr,
            self.device.device_id,
            self.device.signing_key,
            max_pool=1,
            keepalive=self.keepalive,
        ) as cmds:
            await cmds.events_subscribe()
            self._event_pump_ready()
            logger.info("Backend online")
            await self._event_pump_do(cmds)

    async def _event_pump_do(self, cmds):
        while True:
            rep = await cmds.events_listen()

            if rep["event"] == "message.received":
                self.event_bus.send("backend.message.received", index=rep["index"])

            elif rep["event"] == "pinged":
                self.event_bus.send("backend.pinged", ping=rep["ping"])

            elif rep["event"] == "realm.roles_updated":
                realm_id = EntryID(rep["realm_id"])
                self.event_bus.send(
                    "backend.realm.roles_updated", realm_id=realm_id, role=rep["role"]
                )

            elif rep["event"] == "realm.vlobs_updated":
                src_id = EntryID(rep["src_id"])
                realm_id = EntryID(rep["realm_id"])
                self.event_bus.send(
                    "backend.realm.vlobs_updated",
                    realm_id=realm_id,
                    checkpoint=rep["checkpoint"],
                    src_id=src_id,
                    src_version=rep["src_version"],
                )

            elif rep["event"] == "realm.maintenance_started":
                self.event_bus.send(
                    "backend.realm.maintenance_started",
                    realm_id=rep["realm_id"],
                    encryption_revision=rep["encryption_revision"],
                )

            elif rep["event"] == "realm.maintenance_finished":
                self.event_bus.send(
                    "backend.realm.maintenance_finished",
                    realm_id=rep["realm_id"],
                    encryption_revision=rep["encryption_revision"],
                )

            else:
                logger.warning("Backend sent unknown event", event_msg=rep)


async def backend_listen_events(
    device: LocalDevice,
    event_bus: EventBus,
    keepalive: Optional[int],
    *,
    task_status=trio.TASK_STATUS_IGNORED,
):
    with event_bus.connection_context() as event_bus_ctx:
        backend_events_manager = BackendEventsManager(device, event_bus_ctx, keepalive)
        task_status.started()
        await backend_events_manager._event_listener_manager()

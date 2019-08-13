# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from structlog import get_logger

from parsec.event_bus import EventBus
from parsec.api.transport import TransportError
from parsec.core.types import LocalDevice, EntryID
from parsec.core.backend_connection.exceptions import (
    BackendNotAvailable,
    BackendIncompatibleVersion,
    BackendHandshakeError,
    BackendHandshakeAPIVersionError,
    BackendDeviceRevokedError,
    BackendCmdsInvalidResponse,
    BackendCmdsBadResponse,
)
from parsec.core.backend_connection.porcelain import backend_cmds_pool_factory


MAX_COOLDOWN = 30
logger = get_logger()


class BackendEventsManager:
    def __init__(self, device: LocalDevice, event_bus: EventBus):
        self.device = device
        self.event_bus = event_bus
        self._backend_online = None
        self._backend_incompatible_version = None

    def _event_pump_incompatible_version(self):
        if self._backend_incompatible_version is not True:
            self._backend_incompatible_version = True
            self.event_bus.send("backend.incompatible_version")
        # Send "incompatible_version" before "offline" so it can be read correctly
        self._event_pump_lost()

    def _event_pump_lost(self):
        if self._backend_online is not False:
            self._backend_online = False
            self.event_bus.send("backend.offline")

    def _event_pump_ready(self):
        if self._backend_incompatible_version is True:
            self._backend_incompatible_version = False
        if self._backend_online is not True:
            self._backend_online = True
            self.event_bus.send("backend.online")
        self.event_bus.send("backend.listener.restarted")

    async def run(self, *, task_status=trio.TASK_STATUS_IGNORED):
        closed_event = trio.Event()
        try:
            self.event_bus.send("backend.listener.started")
            async with trio.open_nursery() as nursery:
                # If backend is online, we want to wait before calling
                # `task_status.started` until we are connected to the backend
                # with events listener ready.
                with self.event_bus.waiter_on_first("backend.online", "backend.offline") as waiter:

                    async def _wait_first_backend_connection_outcome():
                        await waiter.wait()
                        task_status.started((nursery.cancel_scope, closed_event))

                    nursery.start_soon(_wait_first_backend_connection_outcome)
                    await self._event_listener_manager()

        finally:
            closed_event.set()

    async def _event_listener_manager(self):
        backend_connection_failures = 0
        while True:
            try:

                try:

                    async with trio.open_nursery() as nursery:
                        logger.info("Try to connect to backend...")
                        await nursery.start(self._event_pump)
                        backend_connection_failures = 0
                        logger.info("Backend online")
                        self._event_pump_ready()
                        await trio.sleep_forever()

                except trio.MultiError as exc:
                    # MultiError can contains:
                    # - only Cancelled exceptions (most likely case), this
                    #   is taken care of by trio so we can safely reraise it
                    # - a regular exception and multiple Cancelled, in such
                    #  case we only reraise the regular exception to have
                    #  lower exception handlers deal with it
                    # - multiple regular exceptions... this case is unlikely and
                    #  not well handled so far: we just let the MultiError pop
                    #  up (which is likely to cause issue with upper layers...)
                    # TODO: better multi-regular-exception handling !!!
                    filtered_exc = trio.MultiError.filter(
                        lambda x: None if isinstance(x, trio.Cancelled) else x, exc
                    )
                    raise filtered_exc if filtered_exc else exc

            except (BackendHandshakeAPIVersionError, BackendIncompatibleVersion) as exc:
                # No need to retry, client should update or wait for backend update first
                self._event_pump_incompatible_version()
                logger.info(f"Cannot connect to backend : incompatible version {str(exc)}")
                return

            except (TransportError, BackendNotAvailable) as exc:
                # In case of connection failure, wait a bit and restart
                self._event_pump_lost()
                cooldown_time = 2 ** backend_connection_failures
                backend_connection_failures += 1
                if cooldown_time > MAX_COOLDOWN:
                    cooldown_time = MAX_COOLDOWN
                logger.info("Backend offline", reason=exc, cooldown_time=cooldown_time)
                await trio.sleep(cooldown_time)

            except (BackendCmdsInvalidResponse, BackendCmdsBadResponse):
                # Backend is drunk, what can we do about it ?
                self._event_pump_lost()
                logger.exception("Invalid response sent by backend, restarting connection in 1s...")
                await trio.sleep(1)

            except (BackendHandshakeError, BackendDeviceRevokedError):
                # No need to retry given backend don't want to talk to us...
                self._event_pump_lost()
                logger.exception("Cannot connect to the backend")
                return

    async def _event_pump(self, *, task_status=trio.TASK_STATUS_IGNORED):
        async with backend_cmds_pool_factory(
            self.device.organization_addr,
            self.device.device_id,
            self.device.signing_key,
            max_pool=1,
        ) as cmds:
            await cmds.events_subscribe()

            # Given the backend won't notify us for messages that arrived while
            # we were offline, we must actively check this ourself.
            self.event_bus.send("backend.message.polling_needed")

            task_status.started()
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


async def backend_listen_events(device, event_bus, *, task_status=trio.TASK_STATUS_IGNORED):
    with event_bus.connection_context() as event_bus_ctx:
        backend_events_manager = BackendEventsManager(device, event_bus_ctx)
        await backend_events_manager.run(task_status=task_status)

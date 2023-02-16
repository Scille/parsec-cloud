# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import AbstractAsyncContextManager, asynccontextmanager
from enum import Enum
from typing import (
    TYPE_CHECKING,
    AsyncGenerator,
    AsyncIterator,
    Awaitable,
    Callable,
    List,
    Protocol,
    TypeVar,
)

import trio
from structlog import get_logger
from trio_typing import TaskStatus

from parsec._parsec import (
    ActiveUsersLimit,
    CoreEvent,
    EventsListenRep,
    EventsListenRepOk,
    EventsListenRepOkMessageReceived,
    EventsListenRepOkPinged,
    EventsListenRepOkPkiEnrollmentUpdated,
    EventsListenRepOkRealmMaintenanceFinished,
    EventsListenRepOkRealmMaintenanceStarted,
    EventsListenRepOkRealmRolesUpdated,
    EventsListenRepOkRealmVlobsUpdated,
    EventsSubscribeRepOk,
    OrganizationConfig,
    OrganizationConfigRepOk,
    OrganizationConfigRepUnknownStatus,
)
from parsec._parsec import AuthenticatedCmds as RsBackendAuthenticatedCmds
from parsec.api.protocol import AUTHENTICATED_CMDS, DeviceID
from parsec.api.transport import Transport
from parsec.core.backend_connection import (
    BackendConnectionRefused,
    BackendNotAvailable,
    BackendOutOfBallparkError,
    cmds,
)
from parsec.core.backend_connection.expose_cmds import expose_cmds_with_retrier
from parsec.core.backend_connection.transport import TransportPool, connect_as_authenticated

if TYPE_CHECKING:
    from parsec.core.fs.userfs.userfs import UserFS
from parsec.core.types import BackendOrganizationAddr, LocalDevice
from parsec.crypto import SigningKey
from parsec.event_bus import EventBus
from parsec.utils import open_service_nursery

# This global variable activates RsBackendAuthenticatedCmds binding
OXIDIZED = False

logger = get_logger()

# Time before we try to reconnect after a desync
DESYNC_RETRY_TIME = 10  # seconds


BackendConnStatus = Enum("BackendConnStatus", "READY LOST INITIALIZING REFUSED CRASHED DESYNC")


# Helper to copy exceptions (discarding the traceback but keeping the cause)

BaseExceptionTypeVar = TypeVar("BaseExceptionTypeVar", bound=BaseException)

T = TypeVar("T")


class AcquireTransport(Protocol):
    def __call__(
        self,
        force_fresh: bool = False,
        ignore_status: bool = False,
        allow_not_available: bool = False,
    ) -> AbstractAsyncContextManager[Transport]:
        ...


class MonitorCallback(Protocol):
    def __call__(
        self,
        task_status: MonitorTaskStatus,
        user_fs: UserFS | None = None,
        event_bus: EventBus | None = None,
    ) -> Awaitable[None]:
        ...


def copy_exception(exception: BaseExceptionTypeVar) -> BaseExceptionTypeVar:
    result = type(exception)(*exception.args)
    result.__cause__ = exception.__cause__
    return result


class BackendAuthenticatedCmds:
    def __init__(
        self,
        addr: BackendOrganizationAddr,
        acquire_transport: AcquireTransport,
    ):
        self.addr = addr
        self.acquire_transport = acquire_transport

    events_subscribe = expose_cmds_with_retrier(cmds.events_subscribe)
    events_listen = expose_cmds_with_retrier(cmds.events_listen)
    ping = expose_cmds_with_retrier(cmds.authenticated_ping)
    message_get = expose_cmds_with_retrier(cmds.message_get)
    user_get = expose_cmds_with_retrier(cmds.user_get)
    user_create = expose_cmds_with_retrier(cmds.user_create)
    user_revoke = expose_cmds_with_retrier(cmds.user_revoke)
    device_create = expose_cmds_with_retrier(cmds.device_create)
    human_find = expose_cmds_with_retrier(cmds.human_find)
    invite_new = expose_cmds_with_retrier(cmds.invite_new)
    invite_delete = expose_cmds_with_retrier(cmds.invite_delete)
    invite_list = expose_cmds_with_retrier(cmds.invite_list)
    invite_1_greeter_wait_peer = expose_cmds_with_retrier(cmds.invite_1_greeter_wait_peer)
    invite_2a_greeter_get_hashed_nonce = expose_cmds_with_retrier(
        cmds.invite_2a_greeter_get_hashed_nonce
    )
    invite_2b_greeter_send_nonce = expose_cmds_with_retrier(cmds.invite_2b_greeter_send_nonce)
    invite_3a_greeter_wait_peer_trust = expose_cmds_with_retrier(
        cmds.invite_3a_greeter_wait_peer_trust
    )
    invite_3b_greeter_signify_trust = expose_cmds_with_retrier(cmds.invite_3b_greeter_signify_trust)
    invite_4_greeter_communicate = expose_cmds_with_retrier(cmds.invite_4_greeter_communicate)
    block_create = expose_cmds_with_retrier(cmds.block_create)
    block_read = expose_cmds_with_retrier(cmds.block_read)
    vlob_poll_changes = expose_cmds_with_retrier(cmds.vlob_poll_changes)
    vlob_create = expose_cmds_with_retrier(cmds.vlob_create)
    vlob_read = expose_cmds_with_retrier(cmds.vlob_read)
    vlob_update = expose_cmds_with_retrier(cmds.vlob_update)
    vlob_list_versions = expose_cmds_with_retrier(cmds.vlob_list_versions)
    vlob_maintenance_get_reencryption_batch = expose_cmds_with_retrier(
        cmds.vlob_maintenance_get_reencryption_batch
    )
    vlob_maintenance_save_reencryption_batch = expose_cmds_with_retrier(
        cmds.vlob_maintenance_save_reencryption_batch
    )
    realm_create = expose_cmds_with_retrier(cmds.realm_create)
    realm_status = expose_cmds_with_retrier(cmds.realm_status)
    realm_get_role_certificates = expose_cmds_with_retrier(cmds.realm_get_role_certificates)
    realm_update_roles = expose_cmds_with_retrier(cmds.realm_update_roles)
    realm_start_reencryption_maintenance = expose_cmds_with_retrier(
        cmds.realm_start_reencryption_maintenance
    )
    realm_finish_reencryption_maintenance = expose_cmds_with_retrier(
        cmds.realm_finish_reencryption_maintenance
    )
    organization_stats = expose_cmds_with_retrier(cmds.organization_stats)
    organization_config = expose_cmds_with_retrier(cmds.organization_config)
    pki_enrollment_list = expose_cmds_with_retrier(cmds.pki_enrollment_list)
    pki_enrollment_reject = expose_cmds_with_retrier(cmds.pki_enrollment_reject)
    pki_enrollment_accept = expose_cmds_with_retrier(cmds.pki_enrollment_accept)


for cmd in AUTHENTICATED_CMDS:
    assert hasattr(BackendAuthenticatedCmds, cmd)


# TODO: can we avoid the conversions RealmID/VlobID -> EntryID
def _handle_event(event_bus: EventBus, rep: EventsListenRep) -> None:
    if not isinstance(rep, EventsListenRepOk):
        logger.warning("Bad response to `events_listen` command", rep=rep)
        return

    if isinstance(rep, EventsListenRepOkMessageReceived):
        event_bus.send(CoreEvent.BACKEND_MESSAGE_RECEIVED, index=rep.index)

    elif isinstance(rep, EventsListenRepOkPinged):
        event_bus.send(CoreEvent.BACKEND_PINGED, ping=rep.ping)

    elif isinstance(rep, EventsListenRepOkRealmRolesUpdated):
        event_bus.send(
            CoreEvent.BACKEND_REALM_ROLES_UPDATED,
            realm_id=rep.realm_id.to_entry_id(),
            role=rep.role,
        )

    elif isinstance(rep, EventsListenRepOkRealmVlobsUpdated):
        event_bus.send(
            CoreEvent.BACKEND_REALM_VLOBS_UPDATED,
            realm_id=rep.realm_id.to_entry_id(),
            checkpoint=rep.checkpoint,
            src_id=rep.src_id.to_entry_id(),
            src_version=rep.src_version,
        )

    elif isinstance(rep, EventsListenRepOkRealmMaintenanceStarted):
        event_bus.send(
            CoreEvent.BACKEND_REALM_MAINTENANCE_STARTED,
            realm_id=rep.realm_id.to_entry_id(),
            encryption_revision=rep.encryption_revision,
        )

    elif isinstance(rep, EventsListenRepOkRealmMaintenanceFinished):
        event_bus.send(
            CoreEvent.BACKEND_REALM_MAINTENANCE_FINISHED,
            realm_id=rep.realm_id.to_entry_id(),
            encryption_revision=rep.encryption_revision,
        )

    elif isinstance(rep, EventsListenRepOkPkiEnrollmentUpdated):
        event_bus.send(CoreEvent.PKI_ENROLLMENTS_UPDATED)


def _transport_pool_factory(
    addr: BackendOrganizationAddr,
    device_id: DeviceID,
    signing_key: SigningKey,
    max_pool: int,
    keepalive: int | None,
) -> TransportPool:
    async def _connect() -> Transport:
        transport = await connect_as_authenticated(
            addr, device_id=device_id, signing_key=signing_key, keepalive=keepalive
        )
        transport.logger = transport.logger.bind(device_id=device_id)
        return transport

    return TransportPool(_connect, max_pool=max_pool)


class MonitorTaskState(Enum):
    STALLED = "STALLED"
    IDLE = "IDLE"
    AWAKE = "AWAKE"


class MonitorTaskStatus(TaskStatus[None]):
    state: MonitorTaskState
    monitor_callback: MonitorCallback
    task_status: TaskStatus[None] | None

    def __init__(
        self, monitor_callback: MonitorCallback, state_changed: Callable[[MonitorTaskState], None]
    ):
        self.state = MonitorTaskState.STALLED
        self.monitor_callback = monitor_callback
        self.task_status = None
        self._state_changed: Callable[[MonitorTaskState], None] = state_changed

    def reset(self) -> None:
        self.state = MonitorTaskState.STALLED
        self.task_status = None

    async def run_monitor(self, task_status: TaskStatus[None] = trio.TASK_STATUS_IGNORED) -> None:
        self.task_status = task_status
        self.state = MonitorTaskState.STALLED
        await self.monitor_callback(task_status=self)

    def started(self, value: None = None) -> None:
        assert self.task_status is not None
        self.task_status.started(value)

    def idle(self) -> None:
        self.state = MonitorTaskState.IDLE
        self._state_changed(self.state)

    def awake(self) -> None:
        self.state = MonitorTaskState.AWAKE
        self._state_changed(self.state)


class BackendAuthenticatedConn:
    def __init__(
        self,
        device: LocalDevice,
        event_bus: EventBus,
        max_cooldown: int = 30,
        max_pool: int = 4,
        keepalive: int | None = None,
    ):
        if max_pool < 2:
            raise ValueError("max_pool must be at least 2 (for event listener + query sender)")

        addr = device.organization_addr
        self._device = device
        self._started = False
        self._transport_pool = _transport_pool_factory(
            addr, device.device_id, device.signing_key, max_pool, keepalive
        )
        self._status = BackendConnStatus.LOST
        self._status_exc: Exception | None = None
        self._status_event_sent = False
        self._cmds: RsBackendAuthenticatedCmds | BackendAuthenticatedCmds = (
            RsBackendAuthenticatedCmds(addr, device.device_id, device.signing_key)
            if OXIDIZED
            else BackendAuthenticatedCmds(addr, self._acquire_transport)
        )
        self._manager_connect_cancel_scope: trio.CancelScope | None = None
        self._monitors_task_statuses: List[MonitorTaskStatus] = []
        self._monitors_idle_event = trio.Event()
        self._monitors_idle_event.set()  # No monitors
        self._backend_connection_failures = 0
        # organization config is very unlikely to change, hence we query it
        # once when backend connection bootstraps, then keep the value in cache.
        # On top of that, we pre-populate the cache with a "good enough" default
        # value so organization config is guaranteed to be always available \o/
        self._organization_config = OrganizationConfig(
            user_profile_outsider_allowed=False,
            active_users_limit=ActiveUsersLimit.NO_LIMIT,
            sequester_authority=None,
            sequester_services=None,
        )
        self.event_bus = event_bus
        self.max_cooldown = max_cooldown

    @property
    def status(self) -> BackendConnStatus:
        return self._status

    @property
    def status_exc(self) -> Exception | None:
        # This exception still contains contextual information (e.g. cause, traceback)
        # For this reason, it shouldn't be re-raised as it mutates its internal state
        # Instead, the exception should be copied using `copy_exception`
        return self._status_exc

    @property
    def cmds(self) -> BackendAuthenticatedCmds | RsBackendAuthenticatedCmds:
        return self._cmds

    async def set_status(
        self, status: BackendConnStatus, status_exc: Exception | None = None
    ) -> None:
        # Do not set the status if we're being cancelled
        # Not performing this check can lead to complicated race conditions.
        # In particular, we have to remember that a cancelled task might still
        # run code before the cancellation actually triggers. That means that
        # a particular status might be overwritten between the call to cancel
        # and the actual cancellation.
        await trio.lowlevel.checkpoint_if_cancelled()
        old_status, self._status = self._status, status
        self._status_exc = status_exc
        if not self._status_event_sent or old_status != status:
            self.event_bus.send(
                CoreEvent.BACKEND_CONNECTION_CHANGED,
                status=self._status,
                status_exc=self._status_exc,
            )
        # This is a fix to make sure an event is sent on the first call to set_status
        # A better approach would be to make sure that components using this status
        # do not rely on this redundant event.
        self._status_event_sent = True

    def get_organization_config(self) -> OrganizationConfig:
        return self._organization_config

    def register_monitor(self, monitor_cb: MonitorCallback) -> None:
        if self._started:
            raise RuntimeError("Cannot register monitor once started !")
        self._monitors_task_statuses.append(
            MonitorTaskStatus(monitor_cb, self.on_monitor_state_changed)
        )

    def on_monitor_state_changed(self, state: MonitorTaskState) -> None:
        if state == MonitorTaskState.IDLE:
            if all(
                status.state == MonitorTaskState.IDLE for status in self._monitors_task_statuses
            ):
                self._monitors_idle_event.set()
        elif state == MonitorTaskState.AWAKE:
            if self._monitors_idle_event.is_set():
                self._monitors_idle_event = trio.Event()

    def are_monitors_idle(self) -> bool:
        return self._monitors_idle_event.is_set()

    async def wait_idle_monitors(self) -> None:
        await self._monitors_idle_event.wait()

    def reset_monitors_status(self) -> None:
        for monitor_task_status in self._monitors_task_statuses:
            monitor_task_status.reset()
        if self._monitors_idle_event.is_set():
            self._monitors_idle_event = trio.Event()

    @asynccontextmanager
    async def run(self) -> AsyncIterator[None]:
        if self._started:
            raise RuntimeError("Already started")
        async with open_service_nursery() as nursery:
            nursery.start_soon(self._run_manager)
            yield
            nursery.cancel_scope.cancel()

    async def _run_manager(self) -> None:
        while True:
            try:
                with trio.CancelScope() as self._manager_connect_cancel_scope:
                    try:
                        await self._manager_connect()
                    except (BackendNotAvailable, BackendConnectionRefused):
                        pass
                    except Exception as exc:
                        await self.set_status(
                            BackendConnStatus.CRASHED,
                            BackendNotAvailable(f"Backend connection manager has crashed: {exc}"),
                        )
                        logger.exception("Unhandled exception")
            finally:
                self._manager_connect_cancel_scope = None

            assert self.status not in (
                BackendConnStatus.READY,
                BackendConnStatus.INITIALIZING,
            )
            if self.status == BackendConnStatus.LOST:
                # Start with a 0s cooldown and increase by power of 2 until
                # max cooldown every time the connection trial fails
                # (e.g. 0, 1, 2, 4, 8, 15, 15, 15 etc. if max cooldown is 15s)
                if self._backend_connection_failures < 1:
                    # A cooldown time of 0 second is useful for the specific case of a
                    # revocation when the event listener is the only running transport.
                    # This way, we don't have to wait 1 second before the revocation is
                    # detected.
                    cooldown_time = 0
                else:
                    cooldown_time = 2 ** (self._backend_connection_failures - 1)
                if cooldown_time > self.max_cooldown:
                    cooldown_time = self.max_cooldown
                self._backend_connection_failures += 1
                logger.info("Backend offline", cooldown_time=cooldown_time)
                await self._device.time_provider.sleep(cooldown_time)
            if self.status == BackendConnStatus.REFUSED:
                # It's most likely useless to retry connection anyway
                logger.info("Backend connection refused", status=self.status)
                await trio.sleep_forever()
            if self.status == BackendConnStatus.CRASHED:
                # It's most likely useless to retry connection anyway
                logger.info("Backend connection has crashed", status=self.status)
                await trio.sleep_forever()
            if self.status == BackendConnStatus.DESYNC:
                # Try again in 10 seconds
                logger.info("Backend connection is desync", status=self.status)
                await self._device.time_provider.sleep(DESYNC_RETRY_TIME)

    def _cancel_manager_connect(self) -> None:
        if self._manager_connect_cancel_scope:
            self._manager_connect_cancel_scope.cancel()

    async def _manager_connect(self) -> None:
        async with self._acquire_transport(ignore_status=True, force_fresh=True) as transport:
            await self.set_status(BackendConnStatus.INITIALIZING)
            self._backend_connection_failures = 0
            logger.info("Backend online")

            rep = await cmds.organization_config(transport)
            if not isinstance(rep, OrganizationConfigRepOk):
                # Authenticated's organization_config command has been introduced in API v2.2
                # So a cheap trick to keep backward compatibility here is to
                # stick with the pre-populated organization config cached value.
                if (
                    isinstance(rep, OrganizationConfigRepUnknownStatus)
                    and rep.status != "unknown_command"
                ):
                    raise BackendConnectionRefused(
                        f"Error while fetching organization config: {rep}"
                    )

            else:
                # `organization_config` also provide sequester configuration, however
                # we just ignore it (the remote_loader will lazily load the config
                # the first time it tries a manifest upload with the wrong config)
                self._organization_config = OrganizationConfig(
                    user_profile_outsider_allowed=rep.user_profile_outsider_allowed,
                    active_users_limit=rep.active_users_limit,
                    # Sequester introduced in APIv2.8/3.2
                    sequester_authority=rep.sequester_authority_certificate,
                    sequester_services=rep.sequester_services_certificates,
                )

            rep = await cmds.events_subscribe(transport)
            if not isinstance(rep, EventsSubscribeRepOk):
                raise BackendConnectionRefused(f"Error while events subscribing : {rep}")

            try:
                async with open_service_nursery() as monitors_nursery:

                    async with open_service_nursery() as monitors_bootstrap_nursery:

                        # Make sure we start from a clean state
                        self.reset_monitors_status()
                        for task_status in self._monitors_task_statuses:
                            monitors_bootstrap_nursery.start_soon(
                                monitors_nursery.start,
                                task_status.run_monitor,
                            )

                    await self.set_status(BackendConnStatus.READY)

                    while True:
                        listen_rep = await cmds.events_listen(transport, wait=True)
                        _handle_event(self.event_bus, listen_rep)

            finally:
                # No more monitors are running
                self._monitors_idle_event.set()

    @asynccontextmanager
    async def _acquire_transport(
        self,
        force_fresh: bool = False,
        ignore_status: bool = False,
        allow_not_available: bool = False,
    ) -> AsyncIterator[Transport]:
        if not ignore_status:
            if self.status_exc:
                # Re-raising an already raised exception is bad practice
                # as its internal state gets mutated every time is raised.
                # Note that this copy preserves the __cause__ attribute.
                raise copy_exception(self.status_exc)

        try:
            async with self._transport_pool.acquire(force_fresh=force_fresh) as transport:
                yield transport

        except BackendNotAvailable as exc:
            if not allow_not_available:
                await self.set_status(BackendConnStatus.LOST, exc)
                self._cancel_manager_connect()
            raise

        except BackendConnectionRefused as exc:
            await self.set_status(BackendConnStatus.REFUSED, exc)
            self._cancel_manager_connect()
            raise

        except BackendOutOfBallparkError as exc:
            # Caller doesn't need to know about the desync,
            # simply pretend that we lost the connection instead
            new_exc = BackendNotAvailable()
            new_exc.__cause__ = exc
            await self.set_status(BackendConnStatus.DESYNC, new_exc)
            self._cancel_manager_connect()
            raise new_exc


@asynccontextmanager
async def backend_authenticated_cmds_factory(
    addr: BackendOrganizationAddr,
    device_id: DeviceID,
    signing_key: SigningKey,
    keepalive: int | None = None,
) -> AsyncGenerator[BackendAuthenticatedCmds | RsBackendAuthenticatedCmds, None]:
    """
    Raises:
        BackendConnectionError
    """
    transport_lock = trio.Lock()
    transport: Transport | None = None
    closed = False

    async def _init_transport() -> Transport:
        nonlocal transport
        if transport is None:
            if closed:
                raise trio.ClosedResourceError
            transport = await connect_as_authenticated(
                addr, device_id=device_id, signing_key=signing_key, keepalive=keepalive
            )
            transport.logger = transport.logger.bind(device_id=device_id)

        return transport

    async def _destroy_transport() -> None:
        nonlocal transport
        if transport is not None:
            await transport.aclose()
            transport = None

    @asynccontextmanager
    async def _acquire_transport(
        force_fresh: bool = False, ignore_status: bool = False, allow_not_available: bool = False
    ) -> AsyncIterator[Transport]:
        async with transport_lock:
            transport = await _init_transport()
            try:
                yield transport
            except BackendNotAvailable:
                await _destroy_transport()
                raise

    try:
        yield RsBackendAuthenticatedCmds(
            addr, device_id, signing_key
        ) if OXIDIZED else BackendAuthenticatedCmds(addr, _acquire_transport)

    finally:
        async with transport_lock:
            closed = True
            await _destroy_transport()

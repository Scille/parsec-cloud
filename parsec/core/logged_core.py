# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import importlib.resources
from contextlib import asynccontextmanager
from functools import partial
from pathlib import Path
from typing import AsyncIterator, List, Tuple, Union

import attr
from structlog import get_logger

from parsec._parsec import (
    HumanFindRepOk,
    InvitationDeletedReason,
    InvitationEmailSentStatus,
    InvitationType,
    InviteDeleteRepAlreadyDeleted,
    InviteDeleteRepNotFound,
    InviteDeleteRepOk,
    InviteListItem,
    InviteListRepOk,
    InviteNewRepAlreadyMember,
    InviteNewRepOk,
    OrganizationConfig,
    OrganizationStats,
    OrganizationStatsRepOk,
    Regex,
    UserRevokeRepOk,
    WorkspaceEntry,
)
from parsec.api.data import EntryName, RevokedUserCertificate
from parsec.api.protocol import InvitationToken, UserID
from parsec.core import resources as core_resources
from parsec.core.backend_connection import (
    BackendAuthenticatedConn,
    BackendConnectionError,
    BackendConnStatus,
    BackendInvitationAlreadyUsed,
    BackendInvitationNotFound,
    BackendInvitationOnExistingMember,
    BackendNotAvailable,
    BackendNotFoundError,
)
from parsec.core.config import CoreConfig
from parsec.core.fs import UserFS
from parsec.core.fs.exceptions import FSWorkspaceNotFoundError
from parsec.core.fs.storage.workspace_storage import FAILSAFE_PATTERN_FILTER
from parsec.core.invite import (
    DeviceGreetInitialCtx,
    DeviceGreetInProgress1Ctx,
    UserGreetInitialCtx,
    UserGreetInProgress1Ctx,
)
from parsec.core.messages_monitor import monitor_messages
from parsec.core.mountpoint import MountpointManager, mountpoint_manager_factory
from parsec.core.pki import accepter_list_submitted_from_backend
from parsec.core.pki.accepter import (
    PkiEnrollmentAccepterInvalidSubmittedCtx,
    PkiEnrollmentAccepterValidSubmittedCtx,
)
from parsec.core.remote_devices_manager import (
    RemoteDevicesManager,
    RemoteDevicesManagerBackendOfflineError,
    RemoteDevicesManagerError,
    RemoteDevicesManagerNotFoundError,
)
from parsec.core.sync_monitor import monitor_sync
from parsec.core.types import BackendInvitationAddr, DeviceInfo, LocalDevice, UserInfo
from parsec.event_bus import EventBus

logger = get_logger()


def _get_prevent_sync_pattern(prevent_sync_pattern_path: Path) -> Regex | None:
    try:
        return Regex.from_file(str(prevent_sync_pattern_path))
    except OSError as exc:
        logger.warning(
            "Failed to load the file containing the filename patterns to ignore",
            exc_info=exc,
        )
        return None


def get_prevent_sync_pattern(prevent_sync_pattern_path: Path | None = None) -> Regex:
    pattern = None
    # Get the pattern from the path defined in the core config
    if prevent_sync_pattern_path is not None:
        pattern = _get_prevent_sync_pattern(prevent_sync_pattern_path)
    # Default to the pattern from the ignore file in the core resources
    if pattern is None:
        with importlib.resources.files(core_resources).joinpath("default_pattern.ignore") as path:  # type: ignore[attr-defined]
            pattern = _get_prevent_sync_pattern(path)
    # As a last resort use the failsafe
    if pattern is None:
        return FAILSAFE_PATTERN_FILTER
    return pattern


@attr.s(frozen=True, slots=True, auto_attribs=True)
class LoggedCore:
    config: CoreConfig
    device: LocalDevice
    event_bus: EventBus
    mountpoint_manager: MountpointManager
    user_fs: UserFS
    _remote_devices_manager: RemoteDevicesManager
    _backend_conn: BackendAuthenticatedConn

    def are_monitors_idle(self) -> bool:
        return self._backend_conn.are_monitors_idle()

    async def wait_idle_monitors(self) -> None:
        await self._backend_conn.wait_idle_monitors()

    @property
    def backend_status(self) -> BackendConnStatus:
        return self._backend_conn.status

    @property
    def backend_status_exc(self) -> Exception | None:
        return self._backend_conn.status_exc

    def find_workspace_from_name(self, workspace_name: EntryName) -> WorkspaceEntry:
        for workspace in self.user_fs.get_user_manifest().workspaces:
            if workspace_name == workspace.name:
                return workspace
        raise FSWorkspaceNotFoundError(f"Unknown workspace {workspace_name.str}")

    async def find_humans(
        self,
        query: str | None = None,
        page: int = 1,
        per_page: int = 100,
        omit_revoked: bool = False,
        omit_non_human: bool = False,
    ) -> Tuple[List[UserInfo], int]:
        """
        Raises:
            BackendConnectionError
        """
        rep = await self._backend_conn.cmds.human_find(
            query=query,
            page=page,
            per_page=per_page,
            omit_revoked=omit_revoked,
            omit_non_human=omit_non_human,
        )
        if not isinstance(rep, HumanFindRepOk):
            raise BackendConnectionError(f"Backend error: {rep}")
        results = []
        for item in rep.results:
            # Note `BackendNotFoundError` should never occurs (unless backend is broken !)
            # here given we are feeding the backend the user IDs it has provided us
            user_info = await self.get_user_info(item.user_id)
            results.append(user_info)
        return (results, rep.total)

    async def get_organization_stats(self) -> OrganizationStats:
        """
        Raises:
            BackendConnectionError
        """

        rep = await self._backend_conn.cmds.organization_stats()
        if not isinstance(rep, OrganizationStatsRepOk):
            raise BackendConnectionError(f"Backend error: {rep}")
        return OrganizationStats(
            data_size=rep.data_size,
            metadata_size=rep.metadata_size,
            realms=rep.realms,
            users=rep.users,
            active_users=rep.active_users,
            users_per_profile_detail=rep.users_per_profile_detail,
        )

    async def get_user_info(self, user_id: UserID) -> UserInfo:
        """
        Raises:
            BackendConnectionError
        """
        try:
            user_certif, revoked_user_certif = await self._remote_devices_manager.get_user(user_id)
        except RemoteDevicesManagerBackendOfflineError as exc:
            raise BackendNotAvailable(str(exc)) from exc
        except RemoteDevicesManagerNotFoundError as exc:
            raise BackendNotFoundError(str(exc)) from exc
        except RemoteDevicesManagerError as exc:
            # TODO: we should be using our own kind of exception instead of borrowing BackendConnectionError...
            raise BackendConnectionError(
                f"Error while fetching user {user_id.str} certificates"
            ) from exc
        return UserInfo(
            user_id=user_certif.user_id,
            human_handle=user_certif.human_handle,
            profile=user_certif.profile,
            revoked_on=revoked_user_certif.timestamp if revoked_user_certif else None,
            created_on=user_certif.timestamp,
        )

    async def get_user_devices_info(self, user_id: UserID | None = None) -> List[DeviceInfo]:
        """
        Raises:
            BackendConnectionError
        """
        user_id = user_id or self.device.user_id
        try:
            (
                user_certif,
                revoked_user_certif,
                device_certifs,
            ) = await self._remote_devices_manager.get_user_and_devices(user_id, no_cache=True)
        except RemoteDevicesManagerBackendOfflineError as exc:
            raise BackendNotAvailable(str(exc)) from exc
        except RemoteDevicesManagerNotFoundError as exc:
            raise BackendNotFoundError(str(exc)) from exc
        except RemoteDevicesManagerError as exc:
            # TODO: we should be using our own kind of exception instead of borrowing BackendConnectionError...
            raise BackendConnectionError(
                f"Error while fetching user {user_id.str} certificates"
            ) from exc
        results = []
        for device_certif in device_certifs:
            results.append(
                DeviceInfo(
                    device_id=device_certif.device_id,
                    device_label=device_certif.device_label,
                    created_on=device_certif.timestamp,
                )
            )
        results.sort(key=lambda device: device.created_on, reverse=True)
        return results

    async def revoke_user(self, user_id: UserID) -> None:
        """
        Raises:
            BackendConnectionError
        """
        timestamp = self.device.timestamp()
        revoked_user_certificate = RevokedUserCertificate(
            author=self.device.device_id, timestamp=timestamp, user_id=user_id
        ).dump_and_sign(self.device.signing_key)
        rep = await self._backend_conn.cmds.user_revoke(
            revoked_user_certificate=revoked_user_certificate
        )
        if not isinstance(rep, UserRevokeRepOk):
            raise BackendConnectionError(f"Error while trying to revoke user {user_id}: {rep}")

        # Invalidate potential cache to avoid displaying the user as not-revoked
        self._remote_devices_manager.invalidate_user_cache(user_id)

    async def new_user_invitation(
        self, email: str, send_email: bool
    ) -> Tuple[BackendInvitationAddr, InvitationEmailSentStatus]:
        """
        Raises:
            BackendConnectionError
        """
        rep = await self._backend_conn.cmds.invite_new(
            type=InvitationType.USER, claimer_email=email, send_email=send_email
        )
        if isinstance(rep, InviteNewRepAlreadyMember):
            raise BackendInvitationOnExistingMember("An user already exist with this email")
        elif not isinstance(rep, InviteNewRepOk):
            raise BackendConnectionError(f"Backend error: {rep}")

        try:
            email_sent = rep.email_sent
        except AttributeError:
            email_sent = InvitationEmailSentStatus.SUCCESS

        return (
            BackendInvitationAddr.build(
                backend_addr=self.device.organization_addr.get_backend_addr(),
                organization_id=self.device.organization_id,
                invitation_type=InvitationType.USER,
                token=rep.token,
            ),
            email_sent,
        )

    async def new_device_invitation(
        self, send_email: bool
    ) -> Tuple[BackendInvitationAddr, InvitationEmailSentStatus]:
        """
        Raises:
            BackendConnectionError
        """
        rep = await self._backend_conn.cmds.invite_new(
            type=InvitationType.DEVICE, send_email=send_email
        )
        if not isinstance(rep, InviteNewRepOk):
            raise BackendConnectionError(f"Backend error: {rep}")

        try:
            email_sent = rep.email_sent
        except AttributeError:
            email_sent = InvitationEmailSentStatus.SUCCESS

        return (
            BackendInvitationAddr.build(
                backend_addr=self.device.organization_addr.get_backend_addr(),
                organization_id=self.device.organization_id,
                invitation_type=InvitationType.DEVICE,
                token=rep.token,
            ),
            email_sent,
        )

    async def delete_invitation(
        self,
        token: InvitationToken,
        reason: InvitationDeletedReason = InvitationDeletedReason.CANCELLED,
    ) -> None:
        """
        Raises:
            BackendConnectionError
        """
        rep = await self._backend_conn.cmds.invite_delete(token=token, reason=reason)
        if isinstance(rep, InviteDeleteRepNotFound):
            raise BackendInvitationNotFound("Invitation not found")
        elif isinstance(rep, InviteDeleteRepAlreadyDeleted):
            raise BackendInvitationAlreadyUsed("Invitation already used")
        elif not isinstance(rep, InviteDeleteRepOk):
            raise BackendConnectionError(f"Backend error: {rep}")

    async def list_invitations(self) -> List[InviteListItem]:
        """
        Raises:
            BackendConnectionError
        """
        rep = await self._backend_conn.cmds.invite_list()
        if not isinstance(rep, InviteListRepOk):
            raise BackendConnectionError(f"Backend error: {rep}")
        return rep.invitations

    async def start_greeting_user(self, token: InvitationToken) -> UserGreetInProgress1Ctx:
        """
        Raises:
            BackendConnectionError
            InviteError
        """
        initial_ctx = UserGreetInitialCtx(cmds=self._backend_conn.cmds, token=token)
        return await initial_ctx.do_wait_peer()

    async def start_greeting_device(self, token: InvitationToken) -> DeviceGreetInProgress1Ctx:
        """
        Raises:
            BackendConnectionError
            InviteError
        """
        initial_ctx = DeviceGreetInitialCtx(cmds=self._backend_conn.cmds, token=token)
        return await initial_ctx.do_wait_peer()

    def get_organization_config(self) -> OrganizationConfig:
        return self._backend_conn.get_organization_config()

    async def list_submitted_enrollment_requests(
        self,
    ) -> List[
        Union[PkiEnrollmentAccepterInvalidSubmittedCtx, PkiEnrollmentAccepterValidSubmittedCtx]
    ]:
        return await accepter_list_submitted_from_backend(
            cmds=self._backend_conn.cmds, extra_trust_roots=self.config.pki_extra_trust_roots
        )


@asynccontextmanager
async def logged_core_factory(
    config: CoreConfig, device: LocalDevice, event_bus: EventBus | None = None
) -> AsyncIterator[LoggedCore]:
    event_bus = event_bus or EventBus()
    prevent_sync_pattern = get_prevent_sync_pattern(config.prevent_sync_pattern_path)
    backend_conn = BackendAuthenticatedConn(
        device=device,
        event_bus=event_bus,
        max_cooldown=config.backend_max_cooldown,
        max_pool=config.backend_max_connections,
        keepalive=config.backend_connection_keepalive,
    )

    remote_devices_manager = RemoteDevicesManager(
        backend_conn.cmds, device.root_verify_key, device.time_provider
    )
    async with UserFS.run(
        data_base_dir=config.data_base_dir,
        device=device,
        backend_cmds=backend_conn.cmds,
        remote_devices_manager=remote_devices_manager,
        event_bus=event_bus,
        prevent_sync_pattern=prevent_sync_pattern,
        preferred_language=config.gui_language,
        workspace_storage_cache_size=config.workspace_storage_cache_size,
    ) as user_fs:

        backend_conn.register_monitor(partial(monitor_messages, user_fs, event_bus))
        backend_conn.register_monitor(partial(monitor_sync, user_fs, event_bus))

        async with backend_conn.run():
            async with mountpoint_manager_factory(
                user_fs,
                event_bus,
                config.mountpoint_base_dir,
                mount_all=config.mountpoint_enabled,
                mount_on_workspace_created=config.mountpoint_enabled,
                mount_on_workspace_shared=config.mountpoint_enabled,
                unmount_on_workspace_revoked=config.mountpoint_enabled,
                exclude_from_mount_all=config.disabled_workspaces,
            ) as mountpoint_manager:

                yield LoggedCore(
                    config=config,
                    device=device,
                    event_bus=event_bus,
                    mountpoint_manager=mountpoint_manager,
                    user_fs=user_fs,
                    remote_devices_manager=remote_devices_manager,
                    backend_conn=backend_conn,
                )

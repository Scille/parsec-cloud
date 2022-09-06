# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from collections import defaultdict
from typing import AsyncIterator, Dict, Optional, Pattern, Set, Union

from async_generator import asynccontextmanager
import trio


try:
    from ._libparsec import (
        BackendAddr,
        BackendOrganizationAddr,
        BackendOrganizationBootstrapAddr,
        BackendOrganizationFileLinkAddr,
        BackendInvitationAddr,
        BackendActionAddr,
        EntryID,
        BackendPkiEnrollmentAddr,
        InvitationToken,
        BlockID,
        ChunkID,
        SASCode,
        generate_sas_codes,
        generate_sas_code_candidates,
        InviteUserData,
        InviteUserConfirmation,
        InviteDeviceData,
        InviteDeviceConfirmation,
        EntryName,
        WorkspaceEntry,
        BlockAccess,
        FileManifest,
        FolderManifest,
        WorkspaceManifest,
        UserManifest,
        Chunk,
        LocalFileManifest,
        LocalFolderManifest,
        LocalWorkspaceManifest,
        LocalUserManifest,
        # Block
        BlockCreateReq,
        BlockCreateRep,
        BlockReadReq,
        BlockReadRep,
        # Cmd
        AuthenticatedAnyCmdReq,
        InvitedAnyCmdReq,
        # Events
        EventsListenReq,
        EventsListenRep,
        EventsSubscribeReq,
        EventsSubscribeRep,
        # Invite
        InviteNewReq,
        InviteNewRep,
        InviteDeleteReq,
        InviteDeleteRep,
        InviteListReq,
        InviteListRep,
        InviteInfoReq,
        InviteInfoRep,
        Invite1ClaimerWaitPeerReq,
        Invite1ClaimerWaitPeerRep,
        Invite1GreeterWaitPeerReq,
        Invite1GreeterWaitPeerRep,
        Invite2aClaimerSendHashedNonceHashNonceReq,
        Invite2aClaimerSendHashedNonceHashNonceRep,
        Invite2aGreeterGetHashedNonceReq,
        Invite2aGreeterGetHashedNonceRep,
        Invite2bClaimerSendNonceReq,
        Invite2bClaimerSendNonceRep,
        Invite2bGreeterSendNonceReq,
        Invite2bGreeterSendNonceRep,
        Invite3aClaimerSignifyTrustReq,
        Invite3aClaimerSignifyTrustRep,
        Invite3aGreeterWaitPeerTrustReq,
        Invite3aGreeterWaitPeerTrustRep,
        Invite3bClaimerWaitPeerTrustReq,
        Invite3bClaimerWaitPeerTrustRep,
        Invite3bGreeterSignifyTrustReq,
        Invite3bGreeterSignifyTrustRep,
        Invite4ClaimerCommunicateReq,
        Invite4ClaimerCommunicateRep,
        Invite4GreeterCommunicateReq,
        Invite4GreeterCommunicateRep,
        InviteListItem,
        # Ping
        AuthenticatedPingReq,
        AuthenticatedPingRep,
        InvitedPingReq,
        InvitedPingRep,
        # Realm
        RealmCreateReq,
        RealmCreateRep,
        RealmStatusReq,
        RealmStatusRep,
        RealmStatsReq,
        RealmStatsRep,
        RealmGetRoleCertificateReq,
        RealmGetRoleCertificateRep,
        RealmUpdateRolesReq,
        RealmUpdateRolesRep,
        RealmStartReencryptionMaintenanceReq,
        RealmStartReencryptionMaintenanceRep,
        RealmFinishReencryptionMaintenanceReq,
        RealmFinishReencryptionMaintenanceRep,
        # User
        UserGetReq,
        UserGetRep,
        UserCreateReq,
        UserCreateRep,
        UserRevokeReq,
        UserRevokeRep,
        DeviceCreateReq,
        DeviceCreateRep,
        HumanFindReq,
        HumanFindRep,
        Trustchain,
        HumanFindResultItem,
        # LocalDevice
        LocalDevice,
        # Storage
        WorkspaceStorage as _SyncWorkspaceStorage,
        # File operations
        prepare_read,
        prepare_write,
        prepare_resize,
        prepare_reshape,
    )
except ImportError as exc:
    print(f"Import error in libparsec/types: {exc}")


class WorkspaceStorage:
    def __init__(self, *args, **kwargs):
        self.sync_instance = _SyncWorkspaceStorage(*args, **kwargs)
        # Locking structures
        self.locking_tasks: Dict[EntryID, trio.lowlevel.Task] = {}
        self.entry_locks: Dict[EntryID, trio.Lock] = defaultdict(trio.Lock)

    @property
    def device(self) -> LocalDevice:
        return self.sync_instance.device

    @property
    def workspace_id(self) -> EntryID:
        return self.sync_instance.workspace_id

    @classmethod
    @asynccontextmanager
    async def run(cls, *args, **kwargs):
        self = cls(*args, **kwargs)
        try:
            yield self
        finally:
            self.sync_instance.clear_memory_cache(flush=True)

    @asynccontextmanager
    async def lock_entry_id(self, entry_id: EntryID) -> AsyncIterator[EntryID]:
        async with self.entry_locks[entry_id]:
            try:
                self.locking_tasks[entry_id] = trio.lowlevel.current_task()
                yield entry_id
            finally:
                del self.locking_tasks[entry_id]

    @asynccontextmanager
    async def lock_manifest(self, entry_id: EntryID) -> AsyncIterator:
        async with self.lock_entry_id(entry_id):
            yield await self.get_manifest(entry_id)

    def _check_lock_status(self, entry_id: EntryID) -> None:
        task = self.locking_tasks.get(entry_id)
        if task != trio.lowlevel.current_task():
            raise RuntimeError(f"Entry `{entry_id}` modified without beeing locked")

    def create_file_descriptor(self, manifest: LocalFileManifest) -> int:
        return self.sync_instance.create_file_descriptor(manifest)

    def remove_file_descriptor(self, fd: int) -> None:
        return self.sync_instance.remove_file_descriptor(fd)

    def get_workspace_manifest(self) -> LocalWorkspaceManifest:
        return self.sync_instance.get_workspace_manifest()

    def get_prevent_sync_pattern(self) -> Pattern[str]:
        return self.sync_instance.get_prevent_sync_pattern()

    def get_prevent_sync_pattern_fully_applied(self) -> bool:
        return self.sync_instance.get_prevent_sync_pattern_fully_applied()

    async def set_manifest(
        self,
        entry_id: EntryID,
        manifest,
        cache_only: bool = False,
        check_lock_status: bool = True,
        removed_ids: Optional[Set[Union[BlockID, ChunkID]]] = None,
    ) -> None:
        if check_lock_status:
            self._check_lock_status(entry_id)
        await trio.to_thread.run_sync(
            self.sync_instance.set_manifest, entry_id, manifest, cache_only, removed_ids
        )

    async def ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        self._check_lock_status(entry_id)
        await trio.to_thread.run_sync(self.sync_instance.ensure_manifest_persistent, entry_id)

    async def clear_manifest(self, entry_id: EntryID) -> None:
        self._check_lock_status(entry_id)
        await trio.to_thread.run_sync(self.sync_instance.clear_manifest, entry_id)

    def __getattr__(self, method_name):
        method = getattr(self.sync_instance, method_name)

        async def wrapper(self, *args, **kwargs):
            return await trio.to_thread.run_sync(lambda: method(*args, **kwargs))

        return wrapper.__get__(self)


__all__ = (
    "BackendAddr",
    "BackendOrganizationAddr",
    "BackendOrganizationBootstrapAddr",
    "BackendOrganizationFileLinkAddr",
    "BackendInvitationAddr",
    "BackendPkiEnrollmentAddr",
    "BackendActionAddr",
    "InvitationToken",
    "SASCode",
    "generate_sas_codes",
    "generate_sas_code_candidates",
    "InviteUserData",
    "InviteUserConfirmation",
    "InviteDeviceData",
    "InviteDeviceConfirmation",
    "EntryName",
    "WorkspaceEntry",
    "BlockAccess",
    "FileManifest",
    "FolderManifest",
    "WorkspaceManifest",
    "UserManifest",
    "Chunk",
    "LocalFileManifest",
    "LocalFolderManifest",
    "LocalWorkspaceManifest",
    "LocalUserManifest",
    # Block
    "BlockCreateReq",
    "BlockCreateRep",
    "BlockReadReq",
    "BlockReadRep",
    # Cmd
    "AuthenticatedAnyCmdReq",
    "InvitedAnyCmdReq",
    # Events
    "EventsListenReq",
    "EventsListenRep",
    "EventsSubscribeReq",
    "EventsSubscribeRep",
    # Invite
    "InviteNewReq",
    "InviteNewRep",
    "InviteDeleteReq",
    "InviteDeleteRep",
    "InviteListReq",
    "InviteListRep",
    "InviteInfoReq",
    "InviteInfoRep",
    "Invite1ClaimerWaitPeerReq",
    "Invite1ClaimerWaitPeerRep",
    "Invite1GreeterWaitPeerReq",
    "Invite1GreeterWaitPeerRep",
    "Invite2aClaimerSendHashedNonceHashNonceReq",
    "Invite2aClaimerSendHashedNonceHashNonceRep",
    "Invite2aGreeterGetHashedNonceReq",
    "Invite2aGreeterGetHashedNonceRep",
    "Invite2bClaimerSendNonceReq",
    "Invite2bClaimerSendNonceRep",
    "Invite2bGreeterSendNonceReq",
    "Invite2bGreeterSendNonceRep",
    "Invite3aClaimerSignifyTrustReq",
    "Invite3aClaimerSignifyTrustRep",
    "Invite3aGreeterWaitPeerTrustReq",
    "Invite3aGreeterWaitPeerTrustRep",
    "Invite3bClaimerWaitPeerTrustReq",
    "Invite3bClaimerWaitPeerTrustRep",
    "Invite3bGreeterSignifyTrustReq",
    "Invite3bGreeterSignifyTrustRep",
    "Invite4ClaimerCommunicateReq",
    "Invite4ClaimerCommunicateRep",
    "Invite4GreeterCommunicateReq",
    "Invite4GreeterCommunicateRep",
    "InviteListItem",
    # Ping
    "AuthenticatedPingReq",
    "AuthenticatedPingRep",
    "InvitedPingReq",
    "InvitedPingRep",
    # Realm
    "RealmCreateReq",
    "RealmCreateRep",
    "RealmStatusReq",
    "RealmStatusRep",
    "RealmStatsReq",
    "RealmStatsRep",
    "RealmGetRoleCertificateReq",
    "RealmGetRoleCertificateRep",
    "RealmUpdateRolesReq",
    "RealmUpdateRolesRep",
    "RealmStartReencryptionMaintenanceReq",
    "RealmStartReencryptionMaintenanceRep",
    "RealmFinishReencryptionMaintenanceReq",
    "RealmFinishReencryptionMaintenanceRep",
    # User
    "UserGetReq",
    "UserGetRep",
    "UserCreateReq",
    "UserCreateRep",
    "UserRevokeReq",
    "UserRevokeRep",
    "DeviceCreateReq",
    "DeviceCreateRep",
    "HumanFindReq",
    "HumanFindRep",
    "Trustchain",
    "HumanFindResultItem",
    # LocalDevice
    "LocalDevice",
    # Storage
    "WorkspaceStorage",
    # File operations
    "prepare_read",
    "prepare_write",
    "prepare_resize",
    "prepare_reshape",
)

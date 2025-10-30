# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from .addr import (
    ParsecWorkspacePathAddr,
)
from .common import (
    U64,
    DateTime,
    DeviceID,
    EntryName,
    ErrorVariant,
    FsPath,
    Handle,
    Path,
    RealmRole,
    Ref,
    Result,
    SizeInt,
    Structure,
    U32BasedType,
    Variant,
    VariantItemUnit,
    VersionInt,
    VlobID,
)


class ClientStartWorkspaceError(ErrorVariant):
    class WorkspaceNotFound:
        pass

    class Internal:
        pass


async def client_start_workspace(
    client: Handle, realm_id: VlobID
) -> Result[Handle, ClientStartWorkspaceError]:
    raise NotImplementedError


class WorkspaceStopError(ErrorVariant):
    class Internal:
        pass


async def workspace_stop(workspace: Handle) -> Result[None, WorkspaceStopError]:
    raise NotImplementedError


class StartedWorkspaceInfo(Structure):
    client: Handle
    id: VlobID
    current_name: EntryName
    current_self_role: RealmRole
    mountpoints: list[tuple[Handle, Path]]


class WorkspaceInfoError(ErrorVariant):
    class Internal:
        pass


async def workspace_info(workspace: Handle) -> Result[StartedWorkspaceInfo, WorkspaceInfoError]:
    raise NotImplementedError


class WorkspaceWatchEntryOneShotError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class EntryNotFound:
        pass

    class NoRealmAccess:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


async def workspace_watch_entry_oneshot(
    workspace: Handle, path: FsPath
) -> Result[VlobID, WorkspaceWatchEntryOneShotError]:
    raise NotImplementedError


class WorkspaceMountError(ErrorVariant):
    class Disabled:
        pass

    class Internal:
        pass


async def workspace_mount(workspace: Handle) -> Result[tuple[Handle, Path], WorkspaceMountError]:
    raise NotImplementedError


class MountpointUnmountError(ErrorVariant):
    class Internal:
        pass


async def mountpoint_unmount(mountpoint: Handle) -> Result[None, MountpointUnmountError]:
    raise NotImplementedError


class MountpointToOsPathError(ErrorVariant):
    class Internal:
        pass


async def mountpoint_to_os_path(
    mountpoint: Handle, parsec_path: FsPath
) -> Result[Path, MountpointToOsPathError]:
    raise NotImplementedError


class WorkspaceCreateFileError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class ReadOnlyRealm:
        pass

    class NoRealmAccess:
        pass

    class ParentNotFound:
        pass

    class ParentNotAFolder:
        pass

    class EntryExists:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


class WorkspaceCreateFolderError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class ReadOnlyRealm:
        pass

    class NoRealmAccess:
        pass

    class ParentNotFound:
        pass

    class ParentNotAFolder:
        pass

    class EntryExists:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


class WorkspaceRemoveEntryError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class ReadOnlyRealm:
        pass

    class CannotRemoveRoot:
        pass

    class EntryNotFound:
        pass

    class NoRealmAccess:
        pass

    class EntryIsFile:
        pass

    class EntryIsFolder:
        pass

    class EntryIsNonEmptyFolder:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


class WorkspaceMoveEntryError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class SourceNotFound:
        pass

    class CannotMoveRoot:
        pass

    class ReadOnlyRealm:
        pass

    class NoRealmAccess:
        pass

    class DestinationExists:
        pass

    class DestinationNotFound:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


class WorkspaceStatEntryError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class EntryNotFound:
        pass

    class NoRealmAccess:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


class EntryStat(Variant):
    class File:
        confinement_point: VlobID | None
        id: VlobID
        parent: VlobID
        created: DateTime
        updated: DateTime
        base_version: VersionInt
        is_placeholder: bool
        need_sync: bool
        size: SizeInt
        last_updater: DeviceID

    class Folder:
        confinement_point: VlobID | None
        id: VlobID
        parent: VlobID
        created: DateTime
        updated: DateTime
        base_version: VersionInt
        is_placeholder: bool
        need_sync: bool
        last_updater: DeviceID


async def workspace_stat_entry(
    workspace: Handle,
    path: Ref[FsPath],
) -> Result[EntryStat, WorkspaceStatEntryError]:
    raise NotImplementedError


async def workspace_stat_entry_by_id_ignore_confinement_point(
    workspace: Handle,
    entry_id: VlobID,
) -> Result[EntryStat, WorkspaceStatEntryError]:
    raise NotImplementedError


async def workspace_stat_entry_by_id(
    workspace: Handle,
    entry_id: VlobID,
) -> Result[EntryStat, WorkspaceStatEntryError]:
    raise NotImplementedError


class WorkspaceStatFolderChildrenError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class EntryNotFound:
        pass

    class EntryIsFile:
        pass

    class NoRealmAccess:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


async def workspace_stat_folder_children(
    workspace: Handle,
    path: Ref[FsPath],
) -> Result[list[tuple[EntryName, EntryStat]], WorkspaceStatFolderChildrenError]:
    raise NotImplementedError


async def workspace_stat_folder_children_by_id(
    workspace: Handle,
    entry_id: VlobID,
) -> Result[list[tuple[EntryName, EntryStat]], WorkspaceStatFolderChildrenError]:
    raise NotImplementedError


class MoveEntryMode(Variant):
    CanReplace = VariantItemUnit()
    CanReplaceFileOnly = VariantItemUnit()
    NoReplace = VariantItemUnit()
    Exchange = VariantItemUnit()


async def workspace_move_entry(
    workspace: Handle,
    src: FsPath,
    dst: FsPath,
    mode: MoveEntryMode,
) -> Result[None, WorkspaceMoveEntryError]:
    raise NotImplementedError


async def workspace_rename_entry_by_id(
    workspace: Handle,
    src_parent_id: VlobID,
    src_name: EntryName,
    dst_name: EntryName,
    mode: MoveEntryMode,
) -> Result[None, WorkspaceMoveEntryError]:
    raise NotImplementedError


async def workspace_create_folder(
    workspace: Handle, path: FsPath
) -> Result[VlobID, WorkspaceCreateFolderError]:
    raise NotImplementedError


async def workspace_create_folder_all(
    workspace: Handle, path: FsPath
) -> Result[VlobID, WorkspaceCreateFolderError]:
    raise NotImplementedError


async def workspace_create_file(
    workspace: Handle, path: FsPath
) -> Result[VlobID, WorkspaceCreateFileError]:
    raise NotImplementedError


async def workspace_remove_entry(
    workspace: Handle, path: FsPath
) -> Result[None, WorkspaceRemoveEntryError]:
    raise NotImplementedError


async def workspace_remove_file(
    workspace: Handle, path: FsPath
) -> Result[None, WorkspaceRemoveEntryError]:
    raise NotImplementedError


async def workspace_remove_folder(
    workspace: Handle, path: FsPath
) -> Result[None, WorkspaceRemoveEntryError]:
    raise NotImplementedError


async def workspace_remove_folder_all(
    workspace: Handle, path: FsPath
) -> Result[None, WorkspaceRemoveEntryError]:
    raise NotImplementedError


class OpenOptions(Structure):
    read: bool
    write: bool
    truncate: bool
    create: bool
    create_new: bool


class FileDescriptor(U32BasedType):
    custom_from_rs_u32 = "|raw: u32| -> Result<_, String> { Ok(libparsec::FileDescriptor(raw)) }"
    custom_to_rs_u32 = "|fd: libparsec::FileDescriptor| -> Result<_, &'static str> { Ok(fd.0) }"


class WorkspaceIsFileContentLocalError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class NoRealmAccess:
        pass

    class EntryNotFound:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass

    class NotAFile:
        pass


async def workspace_is_file_content_local(
    workspace: Handle, path: FsPath
) -> Result[bool, WorkspaceIsFileContentLocalError]:
    raise NotImplementedError


class WorkspaceOpenFileError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class ReadOnlyRealm:
        pass

    class NoRealmAccess:
        pass

    class EntryNotFound:
        pass

    class EntryNotAFile:
        pass

    class EntryExistsInCreateNewMode:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class InvalidManifest:
        pass

    class Internal:
        pass


async def workspace_open_file(
    workspace: Handle, path: FsPath, mode: OpenOptions
) -> Result[FileDescriptor, WorkspaceOpenFileError]:
    raise NotImplementedError


async def workspace_open_file_by_id(
    workspace: Handle,
    entry_id: VlobID,
    mode: OpenOptions,
) -> Result[FileDescriptor, WorkspaceOpenFileError]:
    raise NotImplementedError


async def workspace_open_file_and_get_id(
    workspace: Handle, path: FsPath, mode: OpenOptions
) -> Result[tuple[FileDescriptor, VlobID], WorkspaceOpenFileError]:
    raise NotImplementedError


class WorkspaceFdCloseError(ErrorVariant):
    class Stopped:
        pass

    class BadFileDescriptor:
        pass

    class Internal:
        pass


async def workspace_fd_close(
    workspace: Handle, fd: FileDescriptor
) -> Result[None, WorkspaceFdCloseError]:
    raise NotImplementedError


class WorkspaceFdStatError(ErrorVariant):
    class BadFileDescriptor:
        pass

    class Internal:
        pass


class FileStat(Structure):
    id: VlobID
    created: DateTime
    updated: DateTime
    base_version: VersionInt
    is_placeholder: bool
    need_sync: bool
    size: SizeInt


async def workspace_fd_stat(
    workspace: Handle, fd: FileDescriptor
) -> Result[FileStat, WorkspaceFdStatError]:
    raise NotImplementedError


class WorkspaceFdFlushError(ErrorVariant):
    class Stopped:
        pass

    class BadFileDescriptor:
        pass

    class NotInWriteMode:
        pass

    class Internal:
        pass


async def workspace_fd_flush(
    workspace: Handle, fd: FileDescriptor
) -> Result[None, WorkspaceFdFlushError]:
    raise NotImplementedError


class WorkspaceFdReadError(ErrorVariant):
    class Offline:
        pass

    class ServerBlockstoreUnavailable:
        pass

    class Stopped:
        pass

    class BadFileDescriptor:
        pass

    class NotInReadMode:
        pass

    class NoRealmAccess:
        pass

    class InvalidBlockAccess:
        pass

    class InvalidKeysBundle:
        pass

    class InvalidCertificate:
        pass

    class Internal:
        pass


async def workspace_fd_read(
    workspace: Handle, fd: FileDescriptor, offset: U64, size: U64
) -> Result[bytes, WorkspaceFdReadError]:
    raise NotImplementedError


class WorkspaceFdResizeError(ErrorVariant):
    class BadFileDescriptor:
        pass

    class NotInWriteMode:
        pass

    class Internal:
        pass


async def workspace_fd_resize(
    workspace: Handle, fd: FileDescriptor, length: U64, truncate_only: bool
) -> Result[None, WorkspaceFdResizeError]:
    raise NotImplementedError


class WorkspaceFdWriteError(ErrorVariant):
    class BadFileDescriptor:
        pass

    class NotInWriteMode:
        pass

    class Internal:
        pass


async def workspace_fd_write(
    workspace: Handle, fd: FileDescriptor, offset: U64, data: Ref[bytes]
) -> Result[U64, WorkspaceFdWriteError]:
    raise NotImplementedError


async def workspace_fd_write_constrained_io(
    workspace: Handle, fd: FileDescriptor, offset: U64, data: Ref[bytes]
) -> Result[U64, WorkspaceFdWriteError]:
    raise NotImplementedError


async def workspace_fd_write_start_eof(
    workspace: Handle,
    fd: FileDescriptor,
    data: Ref[bytes],
) -> Result[U64, WorkspaceFdWriteError]:
    raise NotImplementedError


class WorkspaceGeneratePathAddrError(ErrorVariant):
    class Stopped:
        pass

    class Offline:
        pass

    class NotAllowed:
        pass

    class NoKey:
        pass

    class InvalidKeysBundle:
        pass

    class Internal:
        pass


async def workspace_generate_path_addr(
    workspace: Handle,
    path: Ref[FsPath],
) -> Result[ParsecWorkspacePathAddr, WorkspaceGeneratePathAddrError]:
    raise NotImplementedError


class WorkspaceDecryptPathAddrError(ErrorVariant):
    class Stopped:
        pass

    class Offline:
        pass

    class NotAllowed:
        pass

    class KeyNotFound:
        pass

    class CorruptedKey:
        pass

    class CorruptedData:
        pass

    class InvalidCertificate:
        pass

    class InvalidKeysBundle:
        pass

    class Internal:
        pass


async def workspace_decrypt_path_addr(
    workspace: Handle,
    link: Ref[ParsecWorkspacePathAddr],
) -> Result[FsPath, WorkspaceDecryptPathAddrError]:
    raise NotImplementedError

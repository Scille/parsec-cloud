# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Optional

from .common import (
    U64,
    DateTime,
    EntryName,
    ErrorVariant,
    FsPath,
    Handle,
    Ref,
    Result,
    SizeInt,
    Variant,
    VersionInt,
    VlobID,
    Structure,
    U32BasedType,
    Path,
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


class WorkspaceMountError(ErrorVariant):
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

    class ParentIsFile:
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

    class ParentIsFile:
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


class WorkspaceRenameEntryError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class EntryNotFound:
        pass

    class CannotRenameRoot:
        pass

    class ReadOnlyRealm:
        pass

    class NoRealmAccess:
        pass

    class DestinationExists:
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
        confinement_point: Optional[VlobID]
        id: VlobID
        created: DateTime
        updated: DateTime
        base_version: VersionInt
        is_placeholder: bool
        need_sync: bool
        size: SizeInt

    class Folder:
        confinement_point: Optional[VlobID]
        id: VlobID
        created: DateTime
        updated: DateTime
        base_version: VersionInt
        is_placeholder: bool
        need_sync: bool
        children: list[tuple[EntryName, VlobID]]


async def workspace_stat_entry(
    workspace: Handle,
    path: Ref[FsPath],
) -> Result[EntryStat, WorkspaceStatEntryError]:
    raise NotImplementedError


async def workspace_rename_entry(
    workspace: Handle,
    path: FsPath,
    new_name: EntryName,
    overwrite: bool,
) -> Result[None, WorkspaceRenameEntryError]:
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


class WorkspaceFdCloseError(ErrorVariant):
    class Stopped:
        pass

    class BadFileDescriptor:
        pass

    class Internal:
        pass


async def fd_close(workspace: Handle, fd: FileDescriptor) -> Result[None, WorkspaceFdCloseError]:
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


async def fd_flush(workspace: Handle, fd: FileDescriptor) -> Result[None, WorkspaceFdFlushError]:
    raise NotImplementedError


class WorkspaceFdReadError(ErrorVariant):
    class Offline:
        pass

    class Stopped:
        pass

    class BadFileDescriptor:
        pass

    class NotInReadMode:
        pass

    class Internal:
        pass


async def fd_read(
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


async def fd_resize(
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


async def fd_write(
    workspace: Handle, fd: FileDescriptor, offset: U64, data: bytes
) -> Result[U64, WorkspaceFdWriteError]:
    raise NotImplementedError


async def fd_write_with_constrained_io(
    workspace: Handle, fd: FileDescriptor, offset: U64, data: bytes
) -> Result[U64, WorkspaceFdWriteError]:
    raise NotImplementedError

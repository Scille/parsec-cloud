// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export {
  BootstrapOrganizationErrorTag,
  ClaimInProgressErrorTag,
  ClaimerRetrieveInfoErrorTag,
  ClientCancelInvitationErrorTag,
  ClientChangeAuthenticationErrorTag,
  ClientCreateWorkspaceErrorTag,
  ClientEventTag,
  ClientInfoErrorTag,
  ClientListUserDevicesErrorTag,
  ClientListUsersErrorTag,
  ClientListWorkspaceUsersErrorTag,
  ClientListWorkspacesErrorTag,
  ClientNewDeviceInvitationErrorTag,
  ClientNewUserInvitationErrorTag,
  ClientRevokeUserErrorTag,
  ClientShareWorkspaceErrorTag,
  ClientStartErrorTag,
  ClientStartInvitationGreetErrorTag,
  ClientStartWorkspaceErrorTag,
  ClientStopErrorTag,
  DeviceAccessStrategyTag,
  DeviceFileType,
  DeviceSaveStrategyTag,
  EntryStatTag as FileType,
  GreetInProgressErrorTag,
  InvitationEmailSentStatus,
  InvitationStatus,
  ListInvitationsErrorTag,
  MountpointToOsPathErrorTag,
  ParseBackendAddrErrorTag,
  ParsedParsecAddrTag,
  Platform,
  UserOrDeviceClaimInitialInfoTag,
  UserProfile,
  WorkspaceCreateFileErrorTag,
  WorkspaceCreateFolderErrorTag,
  WorkspaceFdCloseErrorTag,
  WorkspaceFdWriteErrorTag,
  WorkspaceInfoErrorTag,
  WorkspaceMountErrorTag,
  WorkspaceOpenFileErrorTag,
  WorkspaceRemoveEntryErrorTag,
  WorkspaceRenameEntryErrorTag,
  WorkspaceStatEntryErrorTag,
  WorkspaceStopErrorTag,
} from '@/plugins/libparsec';
export type {
  AvailableDevice,
  BootstrapOrganizationError,
  ClaimInProgressError,
  ClaimerRetrieveInfoError,
  ClientCancelInvitationError,
  ClientChangeAuthenticationError,
  ClientConfig,
  ClientCreateWorkspaceError,
  ClientEvent,
  ClientEventInvitationChanged,
  ClientEventPing,
  ClientInfo,
  ClientInfoError,
  ClientListUserDevicesError,
  ClientListUsersError,
  ClientListWorkspaceUsersError,
  ClientListWorkspacesError,
  ClientNewDeviceInvitationError,
  ClientNewUserInvitationError,
  ClientRevokeUserError,
  ClientShareWorkspaceError,
  ClientStartError,
  ClientStartInvitationGreetError,
  ClientStartWorkspaceError,
  ClientStopError,
  DeviceAccessStrategy,
  DeviceAccessStrategyPassword,
  DeviceClaimFinalizeInfo,
  DeviceClaimInProgress1Info,
  DeviceClaimInProgress2Info,
  DeviceClaimInProgress3Info,
  DeviceGreetInProgress1Info,
  DeviceGreetInProgress2Info,
  DeviceGreetInProgress3Info,
  DeviceGreetInProgress4Info,
  DeviceGreetInitialInfo,
  DeviceInfo,
  DeviceLabel,
  DeviceSaveStrategy,
  DeviceSaveStrategyKeyring,
  DeviceSaveStrategyPassword,
  EntryName,
  FileDescriptor,
  VlobID as FileID,
  FsPath,
  GreetInProgressError,
  InvitationToken,
  ListInvitationsError,
  MountpointToOsPathError,
  NewInvitationInfo,
  ParseBackendAddrError,
  ParsecAddr,
  ParsecOrganizationFileLinkAddr,
  ParsedParsecAddr,
  ParsedParsecAddrInvitationDevice,
  ParsedParsecAddrInvitationUser,
  ParsedParsecAddrOrganization,
  ParsedParsecAddrOrganizationBootstrap,
  ParsedParsecAddrOrganizationFileLink,
  ParsedParsecAddrPkiEnrollment,
  ParsedParsecAddrServer,
  Result,
  SASCode,
  SizeInt,
  UserClaimFinalizeInfo,
  UserClaimInProgress1Info,
  UserClaimInProgress2Info,
  UserClaimInProgress3Info,
  UserGreetInProgress1Info,
  UserGreetInProgress2Info,
  UserGreetInProgress3Info,
  UserGreetInProgress4Info,
  UserGreetInitialInfo,
  InviteListItemUser as UserInvitation,
  UserOrDeviceClaimInitialInfoDevice,
  UserOrDeviceClaimInitialInfoUser,
  WorkspaceCreateFileError,
  WorkspaceCreateFolderError,
  WorkspaceFdCloseError,
  WorkspaceFdResizeError,
  WorkspaceFdWriteError,
  VlobID as WorkspaceID,
  WorkspaceInfoError,
  WorkspaceMountError,
  WorkspaceOpenFileError,
  WorkspaceRemoveEntryError,
  WorkspaceRenameEntryError,
  WorkspaceStatEntryError,
  WorkspaceStopError,
} from '@/plugins/libparsec';

import type {
  DateTime,
  DeviceInfo,
  EntryName,
  Handle,
  HumanHandle,
  OrganizationID,
  EntryStatFile as ParsecEntryStatFile,
  EntryStatFolder as ParsecEntryStatFolder,
  ParsecOrganizationAddr,
  StartedWorkspaceInfo as ParsecStartedWorkspaceInfo,
  UserInfo as ParsecUserInfo,
  WorkspaceInfo as ParsecWorkspaceInfo,
  Path,
  UserID,
  UserProfile,
  VlobID,
} from '@/plugins/libparsec';

import { RealmRole as WorkspaceRole } from '@/plugins/libparsec';

type WorkspaceHandle = Handle;
type EntryID = VlobID;
type WorkspaceName = EntryName;
type ConnectionHandle = Handle;
type MountpointHandle = Handle;
type SystemPath = Path;

interface UserInfo extends ParsecUserInfo {
  isRevoked: () => boolean;
}

interface OwnDeviceInfo extends DeviceInfo {
  isCurrent: boolean;
}

interface EntryStatFolder extends ParsecEntryStatFolder {
  isFile: () => boolean;
  name: EntryName;
}

interface EntryStatFile extends ParsecEntryStatFile {
  isFile: () => boolean;
  name: EntryName;
}

interface OpenOptions {
  read?: boolean;
  write?: boolean;
  append?: boolean;
  truncate?: boolean;
  create?: boolean;
  createNew?: boolean;
}

type EntryStat = EntryStatFile | EntryStatFolder;

enum GetWorkspaceNameErrorTag {
  NotFound = 'NotFound',
}

interface GetWorkspaceNameError {
  tag: GetWorkspaceNameErrorTag.NotFound;
}

enum GetAbsolutePathErrorTag {
  NotFound = 'NotFound',
}

interface GetAbsolutePathError {
  tag: GetAbsolutePathErrorTag.NotFound;
}

enum LinkErrorTag {
  WorkspaceNotFound = 'WorkspaceNotFound',
  PathNotFound = 'PathNotFound',
  Internal = 'Internal',
}

interface LinkErrorWorkspaceNotFound {
  tag: LinkErrorTag.WorkspaceNotFound;
}

interface LinkErrorPathNotFound {
  tag: LinkErrorTag.PathNotFound;
}

interface LinkErrorInternal {
  tag: LinkErrorTag.Internal;
}

export type LinkError = LinkErrorWorkspaceNotFound | LinkErrorPathNotFound | LinkErrorInternal;

interface UserTuple {
  id: UserID;
  humanHandle: HumanHandle;
  profile: UserProfile;
}

interface WorkspaceInfo extends ParsecWorkspaceInfo {
  sharing: Array<[UserTuple, WorkspaceRole | null]>;
  size: number;
  lastUpdated: DateTime;
  availableOffline: boolean;
  handle: WorkspaceHandle;
  mountpoints: [MountpointHandle, SystemPath][];
}

interface StartedWorkspaceInfo extends ParsecStartedWorkspaceInfo {
  handle: WorkspaceHandle;
}

enum OrganizationInfoErrorTag {
  Internal = 'Internal',
}

interface OrganizationInfoErrorInternal {
  tag: OrganizationInfoErrorTag.Internal;
}

type OrganizationInfoError = OrganizationInfoErrorInternal;

interface OrganizationInfo {
  users: {
    revoked: number;
    total: number;
    active: number;
    admins: number;
    standards: number;
    outsiders: number;
  };
  size: {
    metadata: number;
    data: number;
  };
  outsidersAllowed: boolean;
  userLimit?: number;
  hasUserLimit: boolean;
  organizationAddr: ParsecOrganizationAddr;
  organizationId: OrganizationID;
}

export {
  ConnectionHandle,
  DateTime,
  EntryID,
  EntryStat,
  EntryStatFile,
  EntryStatFolder,
  GetAbsolutePathError,
  GetAbsolutePathErrorTag,
  GetWorkspaceNameError,
  GetWorkspaceNameErrorTag,
  HumanHandle,
  MountpointHandle,
  OpenOptions,
  OrganizationID,
  OrganizationInfo,
  OrganizationInfoError,
  OrganizationInfoErrorTag,
  OwnDeviceInfo,
  StartedWorkspaceInfo,
  SystemPath,
  UserID,
  UserInfo,
  UserTuple,
  WorkspaceHandle,
  WorkspaceInfo,
  WorkspaceName,
  WorkspaceRole,
};

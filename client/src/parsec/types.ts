// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export {
  AnyClaimRetrievedInfoTag,
  BootstrapOrganizationErrorTag,
  CancelledGreetingAttemptReason,
  ClaimerRetrieveInfoErrorTag,
  ClaimInProgressErrorTag,
  ClientCancelInvitationErrorTag,
  ClientCreateWorkspaceErrorTag,
  ClientEventTag,
  ClientExportRecoveryDeviceErrorTag,
  ClientInfoErrorTag,
  ClientListUserDevicesErrorTag,
  ClientListUsersErrorTag,
  ClientListWorkspacesErrorTag,
  ClientListWorkspaceUsersErrorTag,
  ClientNewDeviceInvitationErrorTag,
  ClientNewUserInvitationErrorTag,
  ClientRenameWorkspaceErrorTag,
  ClientRevokeUserErrorTag,
  ClientShareWorkspaceErrorTag,
  ClientStartErrorTag,
  ClientStartInvitationGreetErrorTag,
  ClientStartWorkspaceErrorTag,
  ClientStopErrorTag,
  ClientUserUpdateProfileErrorTag,
  DeviceAccessStrategyTag,
  DeviceFileType,
  DevicePurpose,
  DeviceSaveStrategyTag,
  EntryStatTag as FileType,
  GreetInProgressErrorTag,
  ImportRecoveryDeviceErrorTag,
  InvitationEmailSentStatus,
  InvitationStatus,
  InviteListInvitationCreatedByTag,
  InviteListItemTag,
  ListAvailableDeviceErrorTag,
  ListInvitationsErrorTag,
  MountpointToOsPathErrorTag,
  ParsedParsecAddrTag,
  ParseParsecAddrErrorTag,
  Platform,
  UpdateDeviceErrorTag,
  UserOnlineStatus,
  UserProfile,
  WorkspaceCreateFileErrorTag,
  WorkspaceCreateFolderErrorTag,
  WorkspaceDecryptPathAddrErrorTag,
  WorkspaceFdCloseErrorTag,
  WorkspaceFdReadErrorTag,
  WorkspaceFdResizeErrorTag,
  WorkspaceFdWriteErrorTag,
  WorkspaceHistory2EntryStatTag,
  WorkspaceHistory2FdCloseErrorTag,
  WorkspaceHistory2FdReadErrorTag,
  WorkspaceHistory2InternalOnlyErrorTag,
  WorkspaceHistory2OpenFileErrorTag,
  WorkspaceHistory2SetTimestampOfInterestErrorTag,
  WorkspaceHistory2StatEntryErrorTag,
  WorkspaceHistory2StatFolderChildrenErrorTag,
  WorkspaceHistoryEntryStatTag,
  WorkspaceHistoryFdCloseErrorTag,
  WorkspaceHistoryFdReadErrorTag,
  WorkspaceHistoryOpenFileErrorTag,
  WorkspaceHistoryStatEntryErrorTag,
  WorkspaceHistoryStatFolderChildrenErrorTag,
  WorkspaceInfoErrorTag,
  WorkspaceMountErrorTag,
  WorkspaceMoveEntryErrorTag,
  WorkspaceOpenFileErrorTag,
  WorkspaceRemoveEntryErrorTag,
  WorkspaceStatEntryErrorTag,
  WorkspaceStatFolderChildrenErrorTag,
  WorkspaceStopErrorTag,
} from '@/plugins/libparsec';
export type {
  AnyClaimRetrievedInfoDevice,
  AnyClaimRetrievedInfoUser,
  ApiVersion,
  ArchiveDeviceError,
  AvailableDevice,
  BootstrapOrganizationError,
  ClaimerRetrieveInfoError,
  ClaimInProgressError,
  ClientAcceptTosError,
  ClientCancelInvitationError,
  ClientConfig,
  ClientCreateWorkspaceError,
  ClientEvent,
  ClientEventGreetingAttemptCancelled,
  ClientEventGreetingAttemptJoined,
  ClientEventGreetingAttemptReady,
  ClientEventInvitationChanged,
  ClientEventPing,
  ClientExportRecoveryDeviceError,
  ClientGetTosError,
  ClientInfo,
  ClientInfoError,
  ClientListUserDevicesError,
  ClientListUsersError,
  ClientListWorkspacesError,
  ClientListWorkspaceUsersError,
  ClientNewDeviceInvitationError,
  ClientNewUserInvitationError,
  ClientRenameWorkspaceError,
  ClientRevokeUserError,
  ClientShareWorkspaceError,
  ClientStartError,
  ClientStartInvitationGreetError,
  ClientStartWorkspaceError,
  ClientStopError,
  ClientUserUpdateProfileError,
  DeviceAccessStrategy,
  DeviceAccessStrategyPassword,
  DeviceClaimFinalizeInfo,
  DeviceClaimInProgress1Info,
  DeviceClaimInProgress2Info,
  DeviceClaimInProgress3Info,
  DeviceGreetInitialInfo,
  DeviceGreetInProgress1Info,
  DeviceGreetInProgress2Info,
  DeviceGreetInProgress3Info,
  DeviceGreetInProgress4Info,
  DeviceID,
  DeviceInfo,
  InviteListItemDevice as DeviceInvitation,
  DeviceLabel,
  DeviceSaveStrategy,
  DeviceSaveStrategyKeyring,
  DeviceSaveStrategyPassword,
  EntryName,
  FileDescriptor,
  VlobID as FileID,
  GreetInProgressError,
  ImportRecoveryDeviceError,
  InvitationToken,
  InviteListItem,
  ListAvailableDeviceError,
  ListInvitationsError,
  MountpointToOsPathError,
  NewInvitationInfo,
  ParsecAddr,
  ParsecWorkspacePathAddr,
  ParsedParsecAddr,
  ParsedParsecAddrInvitationDevice,
  ParsedParsecAddrInvitationUser,
  ParsedParsecAddrOrganization,
  ParsedParsecAddrOrganizationBootstrap,
  ParsedParsecAddrPkiEnrollment,
  ParsedParsecAddrServer,
  ParsedParsecAddrWorkspacePath,
  ParseParsecAddrError,
  Result,
  SASCode,
  SizeInt,
  Tos,
  UpdateDeviceError,
  UserClaimFinalizeInfo,
  UserClaimInProgress1Info,
  UserClaimInProgress2Info,
  UserClaimInProgress3Info,
  UserGreetInitialInfo,
  UserGreetInProgress1Info,
  UserGreetInProgress2Info,
  UserGreetInProgress3Info,
  UserGreetInProgress4Info,
  InviteListItemUser as UserInvitation,
  WorkspaceCreateFileError,
  WorkspaceCreateFolderError,
  WorkspaceDecryptPathAddrError,
  WorkspaceFdCloseError,
  WorkspaceFdReadError,
  WorkspaceFdResizeError,
  WorkspaceFdWriteError,
  WorkspaceGeneratePathAddrError,
  WorkspaceHistory2EntryStat,
  WorkspaceHistory2FdCloseError,
  WorkspaceHistory2FdReadError,
  WorkspaceHistory2InternalOnlyError,
  WorkspaceHistory2OpenFileError,
  WorkspaceHistory2SetTimestampOfInterestError,
  WorkspaceHistory2StartError,
  WorkspaceHistory2StatEntryError,
  WorkspaceHistory2StatFolderChildrenError,
  WorkspaceHistoryFdCloseError,
  WorkspaceHistoryFdReadError,
  WorkspaceHistoryOpenFileError,
  WorkspaceHistoryStatEntryError,
  WorkspaceHistoryStatFolderChildrenError,
  VlobID as WorkspaceID,
  WorkspaceInfoError,
  WorkspaceMountError,
  WorkspaceMoveEntryError,
  WorkspaceOpenFileError,
  WorkspaceRemoveEntryError,
  WorkspaceStatEntryError,
  WorkspaceStatFolderChildrenError,
  WorkspaceStopError,
} from '@/plugins/libparsec';

import type {
  DateTime,
  DeviceInfo,
  EntryName,
  FsPath,
  Handle,
  HumanHandle,
  OrganizationID,
  EntryStatFile as ParsecEntryStatFile,
  EntryStatFolder as ParsecEntryStatFolder,
  ParsecOrganizationAddr,
  StartedWorkspaceInfo as ParsecStartedWorkspaceInfo,
  UserInfo as ParsecUserInfo,
  WorkspaceHistory2EntryStatFile as ParsecWorkspaceHistory2EntryStatFile,
  WorkspaceHistory2EntryStatFolder as ParsecWorkspaceHistory2EntryStatFolder,
  WorkspaceInfo as ParsecWorkspaceInfo,
  Path,
  UserID,
  UserProfile,
  VlobID,
} from '@/plugins/libparsec';

import { RealmRole as WorkspaceRole } from '@/plugins/libparsec';

type WorkspaceHistoryHandle = Handle;
type WorkspaceHandle = Handle;
type EntryID = VlobID;
type WorkspaceName = EntryName;
type ConnectionHandle = Handle;
type MountpointHandle = Handle;
type SystemPath = Path;

interface UserInfo extends ParsecUserInfo {
  isRevoked: () => boolean;
  isFrozen: () => boolean;
  isActive: () => boolean;
}

interface OwnDeviceInfo extends DeviceInfo {
  isCurrent: boolean;
  isRecovery: boolean;
}

interface EntryStatFolder extends ParsecEntryStatFolder {
  isFile: () => boolean;
  isConfined: () => boolean;
  path: FsPath;
  name: EntryName;
}

interface EntryStatFile extends Omit<ParsecEntryStatFile, 'size'> {
  isFile: () => boolean;
  isConfined: () => boolean;
  path: FsPath;
  name: EntryName;
  size: number;
}

type EntryStat = EntryStatFile | EntryStatFolder;

interface WorkspaceHistoryEntryStatFile extends Omit<ParsecWorkspaceHistory2EntryStatFile, 'size'> {
  isFile: () => boolean;
  path: FsPath;
  name: EntryName;
  size: number;
}

interface WorkspaceHistoryEntryStatFolder extends ParsecWorkspaceHistory2EntryStatFolder {
  isFile: () => boolean;
  path: FsPath;
  name: EntryName;
}

type WorkspaceHistoryEntryStat = WorkspaceHistoryEntryStatFile | WorkspaceHistoryEntryStatFolder;

interface OpenOptions {
  read?: boolean;
  write?: boolean;
  append?: boolean;
  truncate?: boolean;
  create?: boolean;
  createNew?: boolean;
}

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

interface UserTuple {
  id: UserID;
  humanHandle: HumanHandle;
  profile: UserProfile;
}

interface WorkspaceInfo extends ParsecWorkspaceInfo {
  sharing: Array<[UserTuple, WorkspaceRole | null]>;
  size: number;
  lastUpdated: DateTime;
  created?: DateTime;
  availableOffline: boolean;
  handle: WorkspaceHandle;
  mountpoints: [MountpointHandle, SystemPath][];
}

interface StartedWorkspaceInfo extends ParsecStartedWorkspaceInfo {
  handle: WorkspaceHandle;
  created?: DateTime;
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
    frozen: number;
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
  FsPath,
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
  ParsecOrganizationAddr,
  StartedWorkspaceInfo,
  SystemPath,
  UserID,
  UserInfo,
  UserTuple,
  WorkspaceHandle,
  WorkspaceHistoryEntryStat,
  WorkspaceHistoryEntryStatFile,
  WorkspaceHistoryEntryStatFolder,
  WorkspaceHistoryHandle,
  WorkspaceInfo,
  WorkspaceName,
  WorkspaceRole,
};

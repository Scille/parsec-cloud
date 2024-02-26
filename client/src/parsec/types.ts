// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export {
  BootstrapOrganizationErrorTag,
  ClaimInProgressErrorTag,
  ClaimerRetrieveInfoErrorTag,
  ClientCancelInvitationErrorTag,
  ClientChangeAuthentificationErrorTag,
  ClientCreateWorkspaceErrorTag,
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
  ParseBackendAddrErrorTag,
  ParsedParsecAddrTag,
  Platform,
  UserProfile,
  WorkspaceCreateFileErrorTag,
  WorkspaceCreateFolderErrorTag,
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
  ClientChangeAuthentificationError,
  ClientConfig,
  ClientCreateWorkspaceError,
  ClientEvent,
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
  EntryName,
  VlobID as FileID,
  FsPath,
  GreetInProgressError,
  InvitationToken,
  ListInvitationsError,
  NewInvitationInfo,
  OrganizationID,
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
  VlobID as WorkspaceID,
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
  EntryStatFile as ParsecEntryStatFile,
  EntryStatFolder as ParsecEntryStatFolder,
  UserInfo as ParsecUserInfo,
  WorkspaceInfo as ParsecWorkspaceInfo,
  UserID,
  UserProfile,
  VlobID,
} from '@/plugins/libparsec';

import { RealmRole as WorkspaceRole } from '@/plugins/libparsec';

type WorkspaceHandle = Handle;
type EntryID = VlobID;
type WorkspaceName = EntryName;
type ConnectionHandle = Handle;

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
}

enum OrganizationInfoErrorTag {
  Internal = 'Internal',
  Offline = 'Offline',
}

interface OrganizationInfoErrorInternal {
  tag: OrganizationInfoErrorTag.Internal;
}

interface OrganizationInfoErrorOffline {
  tag: OrganizationInfoErrorTag.Offline;
}

type OrganizationInfoError = OrganizationInfoErrorOffline | OrganizationInfoErrorInternal;

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
    total: number;
  };
  outsidersAllowed: boolean;
  userLimit: number;
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
  OrganizationInfo,
  OrganizationInfoError,
  OrganizationInfoErrorTag,
  OwnDeviceInfo,
  UserID,
  UserInfo,
  UserTuple,
  WorkspaceHandle,
  WorkspaceInfo,
  WorkspaceName,
  WorkspaceRole,
};

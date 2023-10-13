// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export type {
  DeviceLabel,
  ClientStopError,
  ListInvitationsError,
  ClientStartError,
  NewUserInvitationError,
  NewDeviceInvitationError,
  ClaimerRetrieveInfoError,
  ClaimInProgressError,
  BootstrapOrganizationError,
  ParseBackendAddrError,
  DeleteInvitationError,
  ClientListWorkspacesError,
  ClientCreateWorkspaceError,
  ClientInfoError,
  ClientStartInvitationGreetError,
  GreetInProgressError,
  ClientListUserDevicesError,
  ClientListWorkspaceUsersError,
  ClientShareWorkspaceError,
  ClientListUsersError,
  ClientStartWorkspaceError,
  WorkspaceFsOperationError,
  VlobID as WorkspaceID,
  AvailableDevice,
  Result,
  Handle,
  ClientEvent,
  ClientEventPing,
  DeviceAccessStrategyPassword,
  ClientConfig,
  InvitationToken,
  InviteListItemUser as UserInvitation,
  UserClaimInProgress1Info,
  UserClaimInProgress2Info,
  UserClaimInProgress3Info,
  UserClaimFinalizeInfo,
  DeviceClaimFinalizeInfo,
  DeviceClaimInProgress1Info,
  DeviceClaimInProgress2Info,
  DeviceClaimInProgress3Info,
  SASCode,
  UserOrDeviceClaimInitialInfoUser,
  UserOrDeviceClaimInitialInfoDevice,
  ParsedBackendAddr,
  OrganizationID,
  BackendAddr,
  ParsedBackendAddrInvitationUser,
  ClientInfo,
  UserGreetInitialInfo,
  UserGreetInProgress1Info,
  UserGreetInProgress2Info,
  UserGreetInProgress3Info,
  UserGreetInProgress4Info,
  DeviceInfo,
  FsPath,
  VlobID as FileID,
  EntryName,
  NewInvitationInfo,
} from '@/plugins/libparsec';
export {
  DeviceFileType,
  InvitationEmailSentStatus,
  InvitationStatus,
  UserProfile,
  Platform,
  NewUserInvitationErrorTag as InvitationErrorTag,
  ClientStartErrorTag,
  NewUserInvitationErrorTag,
  NewDeviceInvitationErrorTag,
  ListInvitationsErrorTag,
  DeleteInvitationErrorTag,
  ClientListWorkspacesErrorTag,
  ClientCreateWorkspaceErrorTag,
  ClientStopErrorTag,
  ClaimerRetrieveInfoErrorTag,
  ClaimInProgressErrorTag,
  BootstrapOrganizationErrorTag,
  ParseBackendAddrErrorTag,
  ClientInfoErrorTag,
  ClientStartInvitationGreetErrorTag,
  GreetInProgressErrorTag,
  ClientListUsersErrorTag,
  ClientListUserDevicesErrorTag,
  ClientListWorkspaceUsersErrorTag,
  ClientShareWorkspaceErrorTag,
  ClientStartWorkspaceErrorTag,
  WorkspaceFsOperationErrorTag,
  EntryStatTag as FileType,
} from '@/plugins/libparsec';

import type {
  UserInfo as ParsecUserInfo,
  DateTime,
  WorkspaceInfo as ParsecWorkspaceInfo,
  EntryName,
  EntryStatFolder as ParsecEntryStatFolder,
  EntryStatFile as ParsecEntryStatFile,
  UserID,
  HumanHandle,
} from '@/plugins/libparsec';

import {
  RealmRole as WorkspaceRole,
} from '@/plugins/libparsec';

type WorkspaceHandle = number;

interface UserInfo extends ParsecUserInfo {
  isRevoked: () => boolean
}

interface EntryStatFolder extends ParsecEntryStatFolder {
  isFile: () => boolean
  name: EntryName
}

interface EntryStatFile extends ParsecEntryStatFile {
  isFile: () => boolean
  name: EntryName
}

type EntryStat =
  | EntryStatFile
  | EntryStatFolder;

interface GetWorkspaceNameError {
  tag: 'NotFound'
}

interface UserTuple {
  id: UserID,
  humanHandle: HumanHandle,
}

interface WorkspaceInfo extends ParsecWorkspaceInfo {
  sharing: Array<[UserTuple, WorkspaceRole | null]>,
  size: number,
  lastUpdated: DateTime,
  availableOffline: boolean,
}

export type WorkspaceName = EntryName;

export {
  UserInfo,
  WorkspaceInfo,
  UserTuple,
  WorkspaceHandle,
  EntryStatFolder,
  EntryStatFile,
  EntryStat,
  DateTime,
  WorkspaceRole,
  GetWorkspaceNameError,
  UserID,
  HumanHandle,
};

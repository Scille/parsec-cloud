// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export {
  BootstrapOrganizationErrorTag,
  ClaimInProgressErrorTag,
  ClaimerRetrieveInfoErrorTag,
  ClientCreateWorkspaceErrorTag,
  ClientInfoErrorTag,
  ClientListUserDevicesErrorTag,
  ClientListUsersErrorTag,
  ClientListWorkspaceUsersErrorTag,
  ClientListWorkspacesErrorTag,
  ClientShareWorkspaceErrorTag,
  ClientStartErrorTag,
  ClientStartInvitationGreetErrorTag,
  ClientStartWorkspaceErrorTag,
  ClientStopErrorTag,
  DeleteInvitationErrorTag,
  DeviceFileType,
  EntryStatTag as FileType,
  GreetInProgressErrorTag,
  InvitationEmailSentStatus,
  NewUserInvitationErrorTag as InvitationErrorTag,
  InvitationStatus,
  ListInvitationsErrorTag,
  NewDeviceInvitationErrorTag,
  NewUserInvitationErrorTag,
  ParseBackendAddrErrorTag,
  ParsedBackendAddrTag,
  Platform,
  UserProfile,
  WorkspaceFsOperationErrorTag,
} from '@/plugins/libparsec';
export type {
  AvailableDevice,
  BackendAddr,
  BackendOrganizationFileLinkAddr,
  BootstrapOrganizationError,
  ClaimInProgressError,
  ClaimerRetrieveInfoError,
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
  ClientShareWorkspaceError,
  ClientStartError,
  ClientStartInvitationGreetError,
  ClientStartWorkspaceError,
  ClientStopError,
  DeleteInvitationError,
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
  EntryName,
  VlobID as FileID,
  FsPath,
  GreetInProgressError,
  InvitationToken,
  ListInvitationsError,
  NewDeviceInvitationError,
  NewInvitationInfo,
  NewUserInvitationError,
  OrganizationID,
  ParseBackendAddrError,
  ParsedBackendAddr,
  ParsedBackendAddrInvitationDevice,
  ParsedBackendAddrInvitationUser,
  ParsedBackendAddrOrganization,
  ParsedBackendAddrOrganizationBootstrap,
  ParsedBackendAddrOrganizationFileLink,
  ParsedBackendAddrPkiEnrollment,
  ParsedBackendAddrServer,
  Result,
  SASCode,
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
  WorkspaceFsOperationError,
  VlobID as WorkspaceID,
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

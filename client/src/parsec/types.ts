// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import type {
  DeviceLabel,
  VlobID as WorkspaceID,
  AvailableDevice,
  Result,
  Handle,
  ClientEvent,
  DateTime,
  ClientEventPing,
  DeviceAccessStrategyPassword,
  ClientConfig,
  EntryName as WorkspaceName,
  InvitationToken,
  NewUserInvitationError,
  NewDeviceInvitationError,
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
  HumanHandle,
  UserOrDeviceClaimInitialInfoUser,
  UserOrDeviceClaimInitialInfoDevice,
  ParsedBackendAddr,
  ClientStartError,
  ListInvitationsError,
  DeleteInvitationError,
  ClientListWorkspacesError,
  ClientCreateWorkspaceError,
  ClientStopError,
  ClaimerRetrieveInfoError,
  ClaimInProgressError,
  BootstrapOrganizationError,
  ParseBackendAddrError,
  OrganizationID,
  BackendAddr,
  ParsedBackendAddrInvitationUser,
  ClientInfo,
  ClientInfoError,
  ClientStartInvitationGreetError,
  UserGreetInitialInfo,
  UserGreetInProgress1Info,
  GreetInProgressError,
  UserGreetInProgress2Info,
  UserGreetInProgress3Info,
  UserGreetInProgress4Info,
  UserInfo as ParsecUserInfo,
  ClientListUsersError,
  DeviceInfo,
  ClientListUserDevicesError,
  UserID,
  WorkspaceInfo as ParsecWorkspaceInfo,
  ClientListWorkspaceUsersError,
  ClientShareWorkspaceError,
  ClientStartWorkspaceError,
  FsPath,
  VlobID as FileID,
  WorkspaceFsOperationError,
  EntryName,
  EntryStatFolder as ParsecEntryStatFolder,
  EntryStatFile as ParsecEntryStatFile,
  NewInvitationInfo,
} from '@/plugins/libparsec';
// Enums have to be imported separately
import {
  DeviceFileType,
  InvitationEmailSentStatus,
  InvitationStatus,
  UserProfile,
  RealmRole as WorkspaceRole,
  Platform,
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

enum BackendAddrType {
  Invalid = 'InvalidUrl',
  Server = 'Server',
  Organization = 'Organization',
  InvitationUser = 'InvitationUser',
  InvitationDevice = 'InvitationDevice',
  OrganizationBootstrap = 'OrganizationBootstrap',
  OrganizationFileLink = 'OrganizationFileLink',
  PkiEnrollment = 'PkiEnrollment',
}

enum CreateOrganizationError {
  AlreadyUsedToken = 'AlreadyUsedToken',
  BadTimestamp = 'BadTimestamp',
  Internal = 'Internal',
  InvalidToken = 'InvalidToken',
  Offline = 'Offline',
  SaveDevice = 'SaveDeviceError',
}

enum InviteUserError {
  AlreadyMember = 'AlreadyMember',
  Internal = 'Internal',
  NotAllowed = 'NotAllowed',
  Offline = 'Offline',
}

enum DeleteInviteError {
  AlreadyDeleted = 'AlreadyDeleted',
  Internal = 'Internal',
  NotFound = 'NotFound',
  Offline = 'Offline',
}

enum FsOperationError {
  BadTimestamp = 'BadTimestamp',
  CannotRenameRoot = 'CannotRenameRoot',
  EntryExists = 'EntryExists',
  EntryNotFound = 'EntryNotFound',
  FolderNotEmpty = 'FolderNotEmpty',
  Internal = 'Internal',
  InvalidCertificate = 'InvalidCertificate',
  InvalidManifest = 'InvalidManifest',
  IsAFolder = 'IsAFolder',
  NoRealmAccess = 'NoRealmAccess',
  NotAFolder = 'NotAFolder',
  Offline = 'Offline',
  ReadOnlyRealm = 'ReadOnlyRealm',
  NotFound = 'NotFound',
}

enum FileType {
  Folder = 'Folder',
  File = 'File',
}

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

export {
  DeviceLabel,
  UserInvitation,
  WorkspaceID,
  WorkspaceName,
  AvailableDevice,
  ClientConfig,
  Result,
  DeviceAccessStrategyPassword,
  ClientEvent,
  Handle,
  ClientEventPing,
  InvitationStatus,
  ParsedBackendAddr,
  DeviceFileType,
  HumanHandle,
  SASCode,
  InvitationEmailSentStatus,
  InvitationToken,
  BackendAddr,
  OrganizationID,
  UserClaimInProgress1Info,
  UserClaimInProgress2Info,
  UserClaimInProgress3Info,
  UserClaimFinalizeInfo,
  DeviceClaimFinalizeInfo,
  DeviceClaimInProgress1Info,
  DeviceClaimInProgress2Info,
  DeviceClaimInProgress3Info,
  UserOrDeviceClaimInitialInfoUser,
  UserOrDeviceClaimInitialInfoDevice,
  ParsedBackendAddrInvitationUser,
  BackendAddrType,
  UserInfo,
  UserProfile,
  UserGreetInitialInfo,
  UserGreetInProgress1Info,
  UserGreetInProgress2Info,
  UserGreetInProgress3Info,
  UserGreetInProgress4Info,
  WorkspaceRole,
  WorkspaceInfo,
  ClientInfo,
  DeviceInfo,
  UserTuple,
  UserID,
  WorkspaceHandle,
  FsPath,
  FileID,
  EntryName,
  EntryStatFolder,
  EntryStatFile,
  FileType,
  EntryStat,
  NewInvitationInfo,
  Platform,
};

export {
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
  CreateOrganizationError,
  ClientListUserDevicesError,
  ClientListWorkspaceUsersError,
  ClientShareWorkspaceError,
  ClientListUsersError,
  InviteUserError,
  DeleteInviteError,
  ClientStartWorkspaceError,
  FsOperationError,
  WorkspaceFsOperationError,
  GetWorkspaceNameError,
};

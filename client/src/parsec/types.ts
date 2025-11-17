// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export {
  AcceptFinalizeAsyncEnrollmentIdentityStrategyTag,
  AccountAuthMethodStrategyTag,
  AccountCreateErrorTag,
  AccountCreateRegistrationDeviceErrorTag,
  AccountCreateSendValidationEmailErrorTag,
  AccountDeleteProceedErrorTag,
  AccountDeleteSendValidationEmailErrorTag,
  AccountInfoErrorTag,
  AccountListAuthMethodsErrorTag,
  AccountListInvitationsErrorTag,
  AccountListRegistrationDevicesErrorTag,
  AccountLoginErrorTag,
  AccountLoginStrategyTag,
  AccountLogoutErrorTag,
  AccountRecoverProceedErrorTag,
  AccountRecoverSendValidationEmailErrorTag,
  AccountRegisterNewDeviceErrorTag,
  AnyClaimRetrievedInfoTag,
  AsyncEnrollmentIdentitySystemTag,
  AvailableDeviceTypeTag,
  AvailablePendingAsyncEnrollmentIdentitySystemTag,
  BootstrapOrganizationErrorTag,
  CancelledGreetingAttemptReason,
  ClaimerRetrieveInfoErrorTag,
  ClaimFinalizeErrorTag,
  ClaimInProgressErrorTag,
  ClientAcceptAsyncEnrollmentErrorTag,
  ClientCancelInvitationErrorTag,
  ClientCreateWorkspaceErrorTag,
  ClientEventTag,
  ClientExportRecoveryDeviceErrorTag,
  ClientGetOrganizationBootstrapDateErrorTag,
  ClientGetUserDeviceErrorTag,
  ClientGetUserInfoErrorTag,
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
  DevicePurpose,
  DeviceSaveStrategyTag,
  EntryStatTag as FileType,
  GetServerConfigErrorTag,
  GreetInProgressErrorTag,
  ImportRecoveryDeviceErrorTag,
  InvalidityReasonTag,
  InvitationEmailSentStatus,
  InvitationStatus,
  InviteListInvitationCreatedByTag,
  InviteListItemTag,
  ListAvailableDeviceErrorTag,
  ListInvitationsErrorTag,
  MountpointToOsPathErrorTag,
  OpenBaoAuthConfigTag,
  ParsedParsecAddrTag,
  ParseParsecAddrErrorTag,
  PendingAsyncEnrollmentInfoTag,
  Platform,
  ShowCertificateSelectionDialogErrorTag,
  SubmitAsyncEnrollmentErrorTag,
  SubmitAsyncEnrollmentIdentityStrategyTag,
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
  WorkspaceHistoryEntryStatTag,
  WorkspaceHistoryFdCloseErrorTag,
  WorkspaceHistoryFdReadErrorTag,
  WorkspaceHistoryInternalOnlyErrorTag,
  WorkspaceHistoryOpenFileErrorTag,
  WorkspaceHistorySetTimestampOfInterestErrorTag,
  WorkspaceHistoryStatEntryErrorTag,
  WorkspaceHistoryStatFolderChildrenErrorTag,
  WorkspaceInfoErrorTag,
  WorkspaceIsFileContentLocalErrorTag,
  WorkspaceMountErrorTag,
  WorkspaceMoveEntryErrorTag,
  WorkspaceOpenFileErrorTag,
  WorkspaceRemoveEntryErrorTag,
  WorkspaceStatEntryErrorTag,
  WorkspaceStatFolderChildrenErrorTag,
  WorkspaceStopErrorTag,
} from '@/plugins/libparsec';
export type {
  AcceptFinalizeAsyncEnrollmentIdentityStrategy,
  AcceptFinalizeAsyncEnrollmentIdentityStrategyOpenBao,
  AcceptFinalizeAsyncEnrollmentIdentityStrategyPKI,
  AccountAuthMethodStrategy,
  AccountCreateAuthMethodError,
  AccountCreateError,
  AccountCreateRegistrationDeviceError,
  AccountCreateSendValidationEmailError,
  AccountDeleteProceedError,
  AccountDeleteSendValidationEmailError,
  AccountInfo,
  AccountInfoError,
  AccountListAuthMethodsError,
  AccountListInvitationsError,
  AccountListRegistrationDevicesError,
  AccountLoginError,
  AccountLoginStrategy,
  AccountLogoutError,
  AccountRecoverProceedError,
  AccountRecoverSendValidationEmailError,
  AccountRegisterNewDeviceError,
  AccountVaultItemOpaqueKeyID,
  AnyClaimRetrievedInfoDevice,
  AnyClaimRetrievedInfoUser,
  ApiVersion,
  ArchiveDeviceError,
  AsyncEnrollmentUntrusted,
  AvailableDevice,
  AvailableDeviceType,
  AvailableDeviceTypeOpenBao,
  AvailablePendingAsyncEnrollmentIdentitySystemPKI,
  BootstrapOrganizationError,
  ClaimerRetrieveInfoError,
  ClaimFinalizeError,
  ClaimInProgressError,
  ClientAcceptAsyncEnrollmentError,
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
  ClientGetAsyncEnrollmentAddrError,
  ClientGetOrganizationBootstrapDateError,
  ClientGetTosError,
  ClientGetUserDeviceError,
  ClientGetUserInfoError,
  ClientInfo,
  ClientInfoError,
  ClientListAsyncEnrollmentsError,
  ClientListUserDevicesError,
  ClientListUsersError,
  ClientListWorkspacesError,
  ClientListWorkspaceUsersError,
  ClientNewDeviceInvitationError,
  ClientNewUserInvitationError,
  ClientRejectAsyncEnrollmentError,
  ClientRenameWorkspaceError,
  ClientRevokeUserError,
  ClientShareWorkspaceError,
  ClientStartError,
  ClientStartInvitationGreetError,
  ClientStartWorkspaceError,
  ClientStopError,
  ClientUserUpdateProfileError,
  DeviceAccessStrategy,
  DeviceAccessStrategyAccountVault,
  DeviceAccessStrategyKeyring,
  DeviceAccessStrategyOpenBao,
  DeviceAccessStrategyPassword,
  DeviceAccessStrategyPKI,
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
  DeviceSaveStrategyAccountVault,
  DeviceSaveStrategyKeyring,
  DeviceSaveStrategyOpenBao,
  DeviceSaveStrategyPassword,
  DeviceSaveStrategyPKI,
  EntryName,
  FileDescriptor,
  VlobID as FileID,
  GetServerConfigError,
  GreetingAttemptID,
  GreetInProgressError,
  ImportRecoveryDeviceError,
  InviteListItem,
  ListAvailableDeviceError,
  ListInvitationsError,
  MountpointToOsPathError,
  NewInvitationInfo,
  OpenBaoAuthConfig,
  OpenBaoListSelfEmailsError,
  ParsecAddr,
  ParsecAsyncEnrollmentAddr,
  ParsecWorkspacePathAddr,
  ParsedParsecAddr,
  ParsedParsecAddrAsyncEnrollment,
  ParsedParsecAddrInvitationDevice,
  ParsedParsecAddrInvitationUser,
  ParsedParsecAddrOrganization,
  ParsedParsecAddrOrganizationBootstrap,
  ParsedParsecAddrServer,
  ParsedParsecAddrWorkspacePath,
  ParseParsecAddrError,
  RemoveDeviceError,
  Result,
  SASCode,
  SecretKey,
  ServerConfig,
  ShowCertificateSelectionDialogError,
  SizeInt,
  SubmitAsyncEnrollmentError,
  SubmitAsyncEnrollmentIdentityStrategy,
  SubmitAsyncEnrollmentIdentityStrategyOpenBao,
  SubmitAsyncEnrollmentIdentityStrategyPKI,
  SubmitterCancelAsyncEnrollmentError,
  SubmitterFinalizeAsyncEnrollmentError,
  SubmitterListLocalAsyncEnrollmentsError,
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
  WorkspaceHistoryFdCloseError,
  WorkspaceHistoryFdReadError,
  WorkspaceHistoryInternalOnlyError,
  WorkspaceHistoryOpenFileError,
  WorkspaceHistorySetTimestampOfInterestError,
  WorkspaceHistoryStartError,
  WorkspaceHistoryStatEntryError,
  WorkspaceHistoryStatFolderChildrenError,
  VlobID as WorkspaceID,
  WorkspaceInfoError,
  WorkspaceIsFileContentLocalError,
  WorkspaceMountError,
  WorkspaceMoveEntryError,
  WorkspaceOpenFileError,
  WorkspaceRemoveEntryError,
  WorkspaceStatEntryError,
  WorkspaceStatFolderChildrenError,
  WorkspaceStopError,
  X509CertificateReference,
} from '@/plugins/libparsec';

import type {
  AccessToken,
  AvailablePendingAsyncEnrollment,
  DateTime,
  DeviceInfo,
  EntryName,
  FsPath,
  Handle,
  HumanHandle,
  OrganizationID,
  AuthMethodInfo as ParsecAuthMethodInfo,
  EntryStatFile as ParsecEntryStatFile,
  EntryStatFolder as ParsecEntryStatFolder,
  ParsecInvitationAddr,
  ParsecOrganizationAddr,
  StartedWorkspaceInfo as ParsecStartedWorkspaceInfo,
  UserInfo as ParsecUserInfo,
  WorkspaceHistoryEntryStatFile as ParsecWorkspaceHistoryEntryStatFile,
  WorkspaceHistoryEntryStatFolder as ParsecWorkspaceHistoryEntryStatFolder,
  WorkspaceInfo as ParsecWorkspaceInfo,
  Path,
  PendingAsyncEnrollmentInfo,
  UserID,
  UserProfile,
  VlobID,
} from '@/plugins/libparsec';

import { InvitationType, RealmRole as WorkspaceRole } from '@/plugins/libparsec';

type WorkspaceHistoryHandle = Handle;
type WorkspaceHandle = Handle;
type EntryID = VlobID;
type WorkspaceName = EntryName;
type ConnectionHandle = Handle;
type MountpointHandle = Handle;
type SystemPath = Path;
type AccountHandle = Handle;

interface UserInfo extends ParsecUserInfo {
  isRevoked: () => boolean;
  isFrozen: () => boolean;
  isActive: () => boolean;
}

interface OwnDeviceInfo extends DeviceInfo {
  isCurrent: boolean;
  isRecovery: boolean;
  isRegistration: boolean;
  isShamir: boolean;
}

interface EntryStatFolder extends Omit<ParsecEntryStatFolder, 'lastUpdater'> {
  isFile: () => boolean;
  isConfined: () => boolean;
  path: FsPath;
  name: EntryName;
  lastUpdater?: UserInfo;
}

interface EntryStatFile extends Omit<ParsecEntryStatFile, 'size' | 'lastUpdater'> {
  isFile: () => boolean;
  isConfined: () => boolean;
  path: FsPath;
  name: EntryName;
  size: number;
  lastUpdater?: UserInfo;
}

type EntryStat = EntryStatFile | EntryStatFolder;

interface WorkspaceHistoryEntryStatFile extends Omit<ParsecWorkspaceHistoryEntryStatFile, 'size' | 'lastUpdater'> {
  isFile: () => boolean;
  path: FsPath;
  name: EntryName;
  size: number;
  lastUpdater?: UserInfo;
}

interface WorkspaceHistoryEntryStatFolder extends Omit<ParsecWorkspaceHistoryEntryStatFolder, 'lastUpdater'> {
  isFile: () => boolean;
  path: FsPath;
  name: EntryName;
  lastUpdater?: UserInfo;
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
  size?: {
    metadata: number;
    data: number;
  };
  outsidersAllowed: boolean;
  userLimit?: number;
  hasUserLimit: boolean;
  organizationAddr: ParsecOrganizationAddr;
  organizationId: OrganizationID;
  creationDate?: DateTime;
}

interface AccountInvitation {
  addr: ParsecInvitationAddr;
  organizationId: OrganizationID;
  token: AccessToken;
  type: InvitationType;
}

interface RegistrationDevice {
  organizationId: OrganizationID;
  userId: UserID;
}

interface AuthMethodInfo extends ParsecAuthMethodInfo {
  current: boolean;
}

interface AsyncEnrollmentRequest {
  info: PendingAsyncEnrollmentInfo;
  enrollment: AvailablePendingAsyncEnrollment;
  organizationId: OrganizationID;
}

export {
  AccessToken,
  AccountHandle,
  AccountInvitation,
  AsyncEnrollmentRequest,
  AuthMethodInfo,
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
  InvitationType,
  MountpointHandle,
  OpenOptions,
  OrganizationID,
  OrganizationInfo,
  OrganizationInfoError,
  OrganizationInfoErrorTag,
  OwnDeviceInfo,
  ParsecOrganizationAddr,
  RegistrationDevice,
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

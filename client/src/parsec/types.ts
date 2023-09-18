// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  VlobID as WorkspaceID,
  AvailableDevice,
  Result,
  Handle,
  ClientEvent,
  ClientEventPing,
  DeviceAccessStrategyPassword,
  ClientConfig,
  EntryName as WorkspaceName,
  InvitationToken,
  NewUserInvitationError,
  NewDeviceInvitationError,
  InviteListItemUser,
  UserClaimInProgress1Info,
  UserClaimInProgress2Info,
  UserClaimInProgress3Info,
  UserClaimFinalizeInfo,
  SASCode,
  HumanHandle,
  UserOrDeviceClaimInitialInfoUser,
  InvitationEmailSentStatus,
  DeviceFileType,
  ParsedBackendAddr,
  InvitationStatus,
  ClientStartError,
  ListInvitationsError,
  DeleteInvitationError,
  ClientListWorkspacesError,
  ClientWorkspaceCreateError,
  ClientStopError,
  ClaimerRetrieveInfoError,
  ClaimInProgressError,
  BootstrapOrganizationError,
  ParseBackendAddrError,
  OrganizationID,
  BackendAddr,
} from '@/plugins/libparsec';
import { DateTime } from 'luxon';

const DEFAULT_HANDLE = 42;

interface UserInvitation extends InviteListItemUser {
  date: DateTime
}

export {
  UserInvitation,
  WorkspaceID,
  WorkspaceName,
  AvailableDevice,
  DEFAULT_HANDLE,
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
  UserOrDeviceClaimInitialInfoUser,
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
  ClientWorkspaceCreateError,
};

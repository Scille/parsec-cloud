// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getWorkspaceSharing, listOwnDevices, listWorkspaces, WorkspaceInfo, WorkspaceRole } from '@/parsec';

export enum RecommendationAction {
  AddDevice = 'add-device',
  CreateRecoveryFiles = 'create-recovery-files',
  AddWorkspaceOwner = 'add-workspace-owner',
}

export interface SecurityWarnings {
  hasRecoveryDevice: boolean;
  hasMultipleDevices: boolean;
  isWorkspaceOwner: boolean;
  soloOwnerWorkspaces: Array<WorkspaceInfo>;
}

export async function getSecurityWarnings(): Promise<SecurityWarnings> {
  const warnings: SecurityWarnings = {
    hasRecoveryDevice: false,
    hasMultipleDevices: false,
    isWorkspaceOwner: false,
    soloOwnerWorkspaces: [],
  };
  const devicesResult = await listOwnDevices();
  const recoveryDevices = !devicesResult.ok ? [] : devicesResult.value.filter((d) => d.isRecovery);
  warnings.hasRecoveryDevice = recoveryDevices.length > 0;
  warnings.hasMultipleDevices = devicesResult.ok && devicesResult.value.length > recoveryDevices.length + 1;

  const workspacesResult = await listWorkspaces();
  for (const workspace of workspacesResult.ok ? workspacesResult.value : []) {
    if (workspace.currentSelfRole === WorkspaceRole.Owner) {
      warnings.isWorkspaceOwner = true;
      const sharingResult = await getWorkspaceSharing(workspace.id, false, false);
      if (sharingResult.ok && !sharingResult.value.some(([_user, role]) => role === WorkspaceRole.Owner)) {
        warnings.soloOwnerWorkspaces.push(workspace);
      }
    }
  }
  return warnings;
}

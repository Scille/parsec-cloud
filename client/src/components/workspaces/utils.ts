// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { RoleUpdateAuthorization } from '@/components/workspaces/types';
import { UserProfile, WorkspaceRole } from '@/parsec';
import { Translatable } from 'megashark-lib';

export function canChangeRole(
  clientProfile: UserProfile,
  userProfile: UserProfile,
  clientRole: WorkspaceRole | null,
  userRole: WorkspaceRole | null,
  targetRole: WorkspaceRole | null,
): RoleUpdateAuthorization {
  // Outsiders cannot do anything
  if (clientProfile === UserProfile.Outsider) {
    return { authorized: false, reason: 'workspaceRoles.updateRejectedReasons.outsiderProfile' };
  }
  // Outsiders cannot be set to Managers or Owners
  if (userProfile === UserProfile.Outsider && (targetRole === WorkspaceRole.Manager || targetRole === WorkspaceRole.Owner)) {
    return { authorized: false, reason: 'workspaceRoles.updateRejectedReasons.outsiderLimitedRole' };
  }
  // Contributors or Readers cannot update roles
  if (clientRole === null || clientRole === WorkspaceRole.Contributor || clientRole === WorkspaceRole.Reader) {
    return { authorized: false, reason: 'workspaceRoles.updateRejectedReasons.insufficientRole' };
  }
  // Managers cannot update the role of other Managers
  if (clientRole === WorkspaceRole.Manager && userRole === WorkspaceRole.Manager) {
    return { authorized: false, reason: 'workspaceRoles.updateRejectedReasons.managerCannotUpdateManagers' };
  }
  // Managers cannot promote to Managers
  if (clientRole === WorkspaceRole.Manager && targetRole === WorkspaceRole.Manager) {
    return { authorized: false, reason: 'workspaceRoles.updateRejectedReasons.managerCannotPromoteToManager' };
  }
  // Managers cannot promote to Owners
  if (clientRole === WorkspaceRole.Manager && targetRole === WorkspaceRole.Owner) {
    return { authorized: false, reason: 'workspaceRoles.updateRejectedReasons.managerCannotPromoteToOwner' };
  }

  return { authorized: true };
}

/**
 *
 * @param role1 A workspace role
 * @param role2 A workspace role
 * @returns -1 if role2 is inferior to role1, 0 if they're equal, 1 if role1 is superior to role2
 */
export function compareWorkspaceRoles(role1: WorkspaceRole, role2: WorkspaceRole): -1 | 0 | 1 {
  const WEIGHTS = new Map<WorkspaceRole, number>([
    [WorkspaceRole.Owner, 4],
    [WorkspaceRole.Manager, 3],
    [WorkspaceRole.Contributor, 2],
    [WorkspaceRole.Reader, 1],
  ]);

  const diff = (WEIGHTS.get(role1) as number) - (WEIGHTS.get(role2) as number);
  if (diff < 0) {
    return -1;
  } else if (diff > 0) {
    return 1;
  }
  return 0;
}

export function formatWorkspaceDeletionDelay(duration: number | undefined): Translatable {
  if (duration === undefined) {
    return 'WorkspacesPage.trashWorkspace.deletionDelay.default';
  } else if (duration < 60) {
    return { key: 'WorkspacesPage.trashWorkspace.deletionDelay.seconds', data: { amount: duration }, count: duration };
  } else if (duration < 3600) {
    // 60 * 60
    duration = ~~(duration / 60);
    return { key: 'WorkspacesPage.trashWorkspace.deletionDelay.minutes', data: { amount: duration }, count: duration };
  } else if (duration < 86400) {
    // 60 * 60 * 24
    duration = ~~(duration / 3600);
    return { key: 'WorkspacesPage.trashWorkspace.deletionDelay.hours', data: { amount: duration }, count: duration };
  } else {
    duration = ~~(duration / 86400);
    return { key: 'WorkspacesPage.trashWorkspace.deletionDelay.days', data: { amount: duration }, count: duration };
  }
}

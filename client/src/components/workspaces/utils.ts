// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { UserProfile, WorkspaceRole } from '@/parsec';
import { translate } from '@/services/translation';

interface RoleUpdateAuthorization {
  authorized: boolean;
  reason?: string;
}

export function canChangeRole(
  clientProfile: UserProfile,
  userProfile: UserProfile,
  clientRole: WorkspaceRole | null,
  userRole: WorkspaceRole | null,
  targetRole: WorkspaceRole | null,
): RoleUpdateAuthorization {
  // Outsiders cannot do anything
  if (clientProfile === UserProfile.Outsider) {
    return { authorized: false, reason: translate('workspaceRoles.updateRejectedReasons.outsiderProfile') };
  }
  // Outsiders cannot be set to Managers or Owners
  if (userProfile === UserProfile.Outsider && (targetRole === WorkspaceRole.Manager || targetRole === WorkspaceRole.Owner)) {
    return { authorized: false, reason: translate('workspaceRoles.updateRejectedReasons.outsiderLimitedRole') };
  }
  // Contributors or Readers cannot update roles
  if (clientRole === null || clientRole === WorkspaceRole.Contributor || clientRole === WorkspaceRole.Reader) {
    return { authorized: false, reason: translate('workspaceRoles.updateRejectedReasons.insufficientRole') };
  }
  // Cannot change role of an Owner
  if (userRole === WorkspaceRole.Owner) {
    return { authorized: false, reason: translate('workspaceRoles.updateRejectedReasons.ownerImmunity') };
  }
  // Managers cannot update the role of other Managers
  if (clientRole === WorkspaceRole.Manager && userRole === WorkspaceRole.Manager) {
    return { authorized: false, reason: translate('workspaceRoles.updateRejectedReasons.managerCannotUpdateManagers') };
  }
  // Managers cannot promote to Managers
  if (clientRole === WorkspaceRole.Manager && targetRole === WorkspaceRole.Manager) {
    return { authorized: false, reason: translate('workspaceRoles.updateRejectedReasons.managerCannotPromoteToManager') };
  }
  // Managers cannot promote to Owners
  if (clientRole === WorkspaceRole.Manager && targetRole === WorkspaceRole.Owner) {
    return { authorized: false, reason: translate('workspaceRoles.updateRejectedReasons.managerCannotPromoteToOwner') };
  }

  return { authorized: true };
}

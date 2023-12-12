// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { it } from 'vitest';
import { canChangeRole, UserProfile, WorkspaceRole } from '@/parsec';

describe('Workspace role', () => {
  it.each([
    // client is reader, can not change role of anyone
    [
      UserProfile.Outsider,
      WorkspaceRole.Reader,
      UserProfile.Admin,
      WorkspaceRole.Reader,
      WorkspaceRole.Contributor,
      false,
      'workspaceRoles.updateRejectedReasons.insufficientRole',
    ],
    [
      UserProfile.Standard,
      WorkspaceRole.Reader,
      UserProfile.Admin,
      WorkspaceRole.Reader,
      WorkspaceRole.Contributor,
      false,
      'workspaceRoles.updateRejectedReasons.insufficientRole',
    ],
    [
      UserProfile.Outsider,
      WorkspaceRole.Contributor,
      UserProfile.Admin,
      WorkspaceRole.Reader,
      WorkspaceRole.Reader,
      false,
      'workspaceRoles.updateRejectedReasons.insufficientRole',
    ],
    // client is contributor, can not change role of anyone
    [
      UserProfile.Outsider,
      WorkspaceRole.Reader,
      UserProfile.Admin,
      WorkspaceRole.Contributor,
      WorkspaceRole.Contributor,
      false,
      'workspaceRoles.updateRejectedReasons.insufficientRole',
    ],
    [
      UserProfile.Standard,
      WorkspaceRole.Reader,
      UserProfile.Admin,
      WorkspaceRole.Contributor,
      WorkspaceRole.Contributor,
      false,
      'workspaceRoles.updateRejectedReasons.insufficientRole',
    ],
    [
      UserProfile.Outsider,
      WorkspaceRole.Contributor,
      UserProfile.Admin,
      WorkspaceRole.Contributor,
      WorkspaceRole.Reader,
      false,
      'workspaceRoles.updateRejectedReasons.insufficientRole',
    ],
    // user is outsider, can be reader or contributor
    [UserProfile.Outsider, WorkspaceRole.Contributor, UserProfile.Admin, WorkspaceRole.Owner, WorkspaceRole.Reader, true, undefined],
    [UserProfile.Outsider, WorkspaceRole.Contributor, UserProfile.Admin, WorkspaceRole.Owner, WorkspaceRole.Contributor, true, undefined],
    // ... but cannot be manager or owner
    [
      UserProfile.Outsider,
      WorkspaceRole.Contributor,
      UserProfile.Admin,
      WorkspaceRole.Owner,
      WorkspaceRole.Manager,
      false,
      'workspaceRoles.updateRejectedReasons.outsiderLimitedRole',
    ],
    [
      UserProfile.Outsider,
      WorkspaceRole.Contributor,
      UserProfile.Admin,
      WorkspaceRole.Owner,
      WorkspaceRole.Owner,
      false,
      'workspaceRoles.updateRejectedReasons.outsiderLimitedRole',
    ],
    // both are managers, cannot change role
    [
      UserProfile.Standard,
      WorkspaceRole.Manager,
      UserProfile.Admin,
      WorkspaceRole.Manager,
      WorkspaceRole.Contributor,
      false,
      'workspaceRoles.updateRejectedReasons.managerCannotUpdateManagers',
    ],
    // both are owners, cannot change role
    [
      UserProfile.Standard,
      WorkspaceRole.Owner,
      UserProfile.Admin,
      WorkspaceRole.Owner,
      WorkspaceRole.Contributor,
      false,
      'workspaceRoles.updateRejectedReasons.ownerImmunity',
    ],
    // client is outsider, can't do anything
    [
      UserProfile.Standard,
      WorkspaceRole.Reader,
      UserProfile.Outsider,
      WorkspaceRole.Manager,
      WorkspaceRole.Contributor,
      false,
      'workspaceRoles.updateRejectedReasons.outsiderProfile',
    ],
    [
      UserProfile.Standard,
      WorkspaceRole.Reader,
      UserProfile.Outsider,
      WorkspaceRole.Manager,
      WorkspaceRole.Reader,
      false,
      'workspaceRoles.updateRejectedReasons.outsiderProfile',
    ],
    // few cases that should be no problem
    // reader to contributor
    [UserProfile.Standard, WorkspaceRole.Reader, UserProfile.Standard, WorkspaceRole.Manager, WorkspaceRole.Contributor, true, undefined],
    // reader to manager
    [UserProfile.Standard, WorkspaceRole.Reader, UserProfile.Standard, WorkspaceRole.Owner, WorkspaceRole.Manager, true, undefined],
    // reader to owner
    [UserProfile.Standard, WorkspaceRole.Reader, UserProfile.Standard, WorkspaceRole.Owner, WorkspaceRole.Owner, true, undefined],
    // contributor back to reader
    [UserProfile.Standard, WorkspaceRole.Contributor, UserProfile.Standard, WorkspaceRole.Owner, WorkspaceRole.Reader, true, undefined],
    // manager back to reader
    [UserProfile.Standard, WorkspaceRole.Manager, UserProfile.Standard, WorkspaceRole.Owner, WorkspaceRole.Reader, true, undefined],
  ])('test workspace role can change', async (userProfile, currentUserRole, clientProfile, clientRole, targetRole, expected, reason) => {
    function translate(s: string): string {
      return s;
    }

    expect(canChangeRole(clientProfile, userProfile, clientRole, currentUserRole, targetRole, translate).authorized).to.equal(expected);
    expect(canChangeRole(clientProfile, userProfile, clientRole, currentUserRole, targetRole, translate).reason).to.equal(reason);
  });
});

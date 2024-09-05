// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { canChangeRole, compareWorkspaceRoles } from '@/components/workspaces/utils';
import { UserProfile, WorkspaceRole } from '@/parsec';
import { I18n } from 'megashark-lib';
import { it } from 'vitest';

describe('Workspace role', () => {
  it.each([
    // client is reader, can not change role of anyone
    [
      UserProfile.External,
      WorkspaceRole.Reader,
      UserProfile.Admin,
      WorkspaceRole.Reader,
      WorkspaceRole.Contributor,
      false,
      'Only Managers and Owners can change roles',
    ],
    [
      UserProfile.Standard,
      WorkspaceRole.Reader,
      UserProfile.Admin,
      WorkspaceRole.Reader,
      WorkspaceRole.Contributor,
      false,
      'Only Managers and Owners can change roles',
    ],
    [
      UserProfile.External,
      WorkspaceRole.Contributor,
      UserProfile.Admin,
      WorkspaceRole.Reader,
      WorkspaceRole.Reader,
      false,
      'Only Managers and Owners can change roles',
    ],
    // client is contributor, can not change role of anyone
    [
      UserProfile.External,
      WorkspaceRole.Reader,
      UserProfile.Admin,
      WorkspaceRole.Contributor,
      WorkspaceRole.Contributor,
      false,
      'Only Managers and Owners can change roles',
    ],
    [
      UserProfile.Standard,
      WorkspaceRole.Reader,
      UserProfile.Admin,
      WorkspaceRole.Contributor,
      WorkspaceRole.Contributor,
      false,
      'Only Managers and Owners can change roles',
    ],
    [
      UserProfile.External,
      WorkspaceRole.Contributor,
      UserProfile.Admin,
      WorkspaceRole.Contributor,
      WorkspaceRole.Reader,
      false,
      'Only Managers and Owners can change roles',
    ],
    // user is external, can be reader or contributor
    [UserProfile.External, WorkspaceRole.Contributor, UserProfile.Admin, WorkspaceRole.Owner, WorkspaceRole.Reader, true, ''],
    // ... but cannot be manager or owner
    [
      UserProfile.External,
      WorkspaceRole.Contributor,
      UserProfile.Admin,
      WorkspaceRole.Owner,
      WorkspaceRole.Manager,
      false,
      'Externals can only be Readers or Contributors',
    ],
    [
      UserProfile.External,
      WorkspaceRole.Contributor,
      UserProfile.Admin,
      WorkspaceRole.Owner,
      WorkspaceRole.Owner,
      false,
      'Externals can only be Readers or Contributors',
    ],
    // both are managers, cannot change role
    [
      UserProfile.Standard,
      WorkspaceRole.Manager,
      UserProfile.Admin,
      WorkspaceRole.Manager,
      WorkspaceRole.Contributor,
      false,
      'Managers cannot change the role of other managers',
    ],
    // both are owners, change to contributor
    [UserProfile.Standard, WorkspaceRole.Owner, UserProfile.Admin, WorkspaceRole.Owner, WorkspaceRole.Contributor, true, ''],
    // client is external, can't do anything
    [
      UserProfile.Standard,
      WorkspaceRole.Reader,
      UserProfile.External,
      WorkspaceRole.Manager,
      WorkspaceRole.Contributor,
      false,
      'Externals cannot change roles',
    ],
    [
      UserProfile.Standard,
      WorkspaceRole.Reader,
      UserProfile.External,
      WorkspaceRole.Manager,
      WorkspaceRole.Reader,
      false,
      'Externals cannot change roles',
    ],
    // few cases that should be no problem
    // reader to contributor
    [UserProfile.Standard, WorkspaceRole.Reader, UserProfile.Standard, WorkspaceRole.Manager, WorkspaceRole.Contributor, true, ''],
    // reader to manager
    [UserProfile.Standard, WorkspaceRole.Reader, UserProfile.Standard, WorkspaceRole.Owner, WorkspaceRole.Manager, true, ''],
    // reader to owner
    [UserProfile.Standard, WorkspaceRole.Reader, UserProfile.Standard, WorkspaceRole.Owner, WorkspaceRole.Owner, true, ''],
    // contributor back to reader
    [UserProfile.Standard, WorkspaceRole.Contributor, UserProfile.Standard, WorkspaceRole.Owner, WorkspaceRole.Reader, true, ''],
    // manager back to reader
    [UserProfile.Standard, WorkspaceRole.Manager, UserProfile.Standard, WorkspaceRole.Owner, WorkspaceRole.Reader, true, ''],
  ])('test workspace role can change (%#)', (userProfile, currentUserRole, clientProfile, clientRole, targetRole, expected, reason) => {
    expect(canChangeRole(clientProfile, userProfile, clientRole, currentUserRole, targetRole).authorized).to.equal(expected);
    expect(I18n.translate(canChangeRole(clientProfile, userProfile, clientRole, currentUserRole, targetRole).reason)).to.equal(reason);
  });

  it.each([
    [WorkspaceRole.Owner, WorkspaceRole.Owner, 0],
    [WorkspaceRole.Owner, WorkspaceRole.Manager, 1],
    [WorkspaceRole.Owner, WorkspaceRole.Contributor, 1],
    [WorkspaceRole.Owner, WorkspaceRole.Reader, 1],
    [WorkspaceRole.Manager, WorkspaceRole.Owner, -1],
    [WorkspaceRole.Manager, WorkspaceRole.Manager, 0],
    [WorkspaceRole.Manager, WorkspaceRole.Contributor, 1],
    [WorkspaceRole.Manager, WorkspaceRole.Reader, 1],
    [WorkspaceRole.Contributor, WorkspaceRole.Owner, -1],
    [WorkspaceRole.Contributor, WorkspaceRole.Manager, -1],
    [WorkspaceRole.Contributor, WorkspaceRole.Contributor, 0],
    [WorkspaceRole.Contributor, WorkspaceRole.Reader, 1],
    [WorkspaceRole.Reader, WorkspaceRole.Owner, -1],
    [WorkspaceRole.Reader, WorkspaceRole.Manager, -1],
    [WorkspaceRole.Reader, WorkspaceRole.Contributor, -1],
    [WorkspaceRole.Reader, WorkspaceRole.Reader, 0],
  ])('test workspace role comparaison', async (role1, role2, expected) => {
    expect(compareWorkspaceRoles(role1, role2)).to.equal(expected);
  });
});

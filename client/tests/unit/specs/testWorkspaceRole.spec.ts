// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { canChangeRole } from '@/components/workspaces/utils';
import { UserProfile, WorkspaceRole } from '@/parsec';
import { I18n } from 'megashark-lib';
import { it } from 'vitest';

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
      UserProfile.Outsider,
      WorkspaceRole.Contributor,
      UserProfile.Admin,
      WorkspaceRole.Reader,
      WorkspaceRole.Reader,
      false,
      'Only Managers and Owners can change roles',
    ],
    // client is contributor, can not change role of anyone
    [
      UserProfile.Outsider,
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
      UserProfile.Outsider,
      WorkspaceRole.Contributor,
      UserProfile.Admin,
      WorkspaceRole.Contributor,
      WorkspaceRole.Reader,
      false,
      'Only Managers and Owners can change roles',
    ],
    // user is outsider, can be reader or contributor
    [UserProfile.Outsider, WorkspaceRole.Contributor, UserProfile.Admin, WorkspaceRole.Owner, WorkspaceRole.Reader, true, ''],
    [UserProfile.Outsider, WorkspaceRole.Contributor, UserProfile.Admin, WorkspaceRole.Owner, WorkspaceRole.Contributor, true, ''],
    // ... but cannot be manager or owner
    [
      UserProfile.Outsider,
      WorkspaceRole.Contributor,
      UserProfile.Admin,
      WorkspaceRole.Owner,
      WorkspaceRole.Manager,
      false,
      'Outsiders can only be Readers or Contributors',
    ],
    [
      UserProfile.Outsider,
      WorkspaceRole.Contributor,
      UserProfile.Admin,
      WorkspaceRole.Owner,
      WorkspaceRole.Owner,
      false,
      'Outsiders can only be Readers or Contributors',
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
    // both are owners, cannot change role
    [
      UserProfile.Standard,
      WorkspaceRole.Owner,
      UserProfile.Admin,
      WorkspaceRole.Owner,
      WorkspaceRole.Contributor,
      false,
      'Cannot change the role of an Owner',
    ],
    // client is outsider, can't do anything
    [
      UserProfile.Standard,
      WorkspaceRole.Reader,
      UserProfile.Outsider,
      WorkspaceRole.Manager,
      WorkspaceRole.Contributor,
      false,
      'Outsiders cannot change roles',
    ],
    [
      UserProfile.Standard,
      WorkspaceRole.Reader,
      UserProfile.Outsider,
      WorkspaceRole.Manager,
      WorkspaceRole.Reader,
      false,
      'Outsiders cannot change roles',
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
  ])('test workspace role can change', async (userProfile, currentUserRole, clientProfile, clientRole, targetRole, expected, reason) => {
    expect(canChangeRole(clientProfile, userProfile, clientRole, currentUserRole, targetRole).authorized).to.equal(expected);
    expect(I18n.translate(canChangeRole(clientProfile, userProfile, clientRole, currentUserRole, targetRole).reason)).to.equal(reason);
  });
});

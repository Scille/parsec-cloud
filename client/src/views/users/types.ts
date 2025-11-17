// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

enum UserAction {
  Invite = 'user-invite',
  CopyAsyncEnrollmentLink = 'user-copy-async-enrollment-link',
  Revoke = 'user-revoke',
  Details = 'user-details',
  AssignRoles = 'user-assign-roles',
  UpdateProfile = 'user-update-profile',
  ToggleSelect = 'user-toggle-select',
  SelectAll = 'user-select-all',
  UnselectAll = 'user-unselect-all',
}

function isUserAction(value: any): value is UserAction {
  return Object.values(UserAction).includes(value);
}

export { UserAction, isUserAction };

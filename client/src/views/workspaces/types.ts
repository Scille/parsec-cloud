// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

enum WorkspaceAction {
  CreateWorkspace = 'workspace-create-workspace',
  Rename = 'workspace-rename',
  MakeAvailableOffline = 'workspace-make-available-offline',
  CopyLink = 'workspace-copy-link',
  ShowDetails = 'workspace-show-details',
  Share = 'workspace-share',
  ShowHistory = 'workspace-show-history',
  OpenInExplorer = 'workspace-open-in-explorer',
  Mount = 'workspace-mount',
  Favorite = 'workspace-favorite',
}

enum WorkspaceMenu {
  All = 'all',
  Recents = 'recents',
  Favorites = 'favorites',
}

function isWorkspaceAction(value: any): value is WorkspaceAction {
  return Object.values(WorkspaceAction).includes(value);
}

export { WorkspaceAction, WorkspaceMenu, isWorkspaceAction };

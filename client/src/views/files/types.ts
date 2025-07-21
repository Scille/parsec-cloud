// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

enum FolderGlobalAction {
  CreateFolder = 'folder-create-folder',
  ImportFiles = 'folder-import-files',
  ImportFolder = 'folder-import-folder',
  OpenInExplorer = 'folder-open-in-explorer',
  CopyLink = 'folder-copy-link',
  ToggleSelect = 'folder-toggle-select',
  SelectAll = 'folder-select-all',
  Share = 'folder-share',
}

enum FileAction {
  Rename = 'file-rename',
  MoveTo = 'file-move-to',
  MakeACopy = 'file-copy-to',
  Delete = 'file-delete',
  Open = 'file-open',
  Edit = 'file-edit',
  ShowHistory = 'file-show-history',
  Download = 'file-download',
  ShowDetails = 'file-show-details',
  CopyLink = 'file-copy-link',
  SeeInExplorer = 'file-see-in-explorer',
}

function isFileAction(value: any): value is FileAction {
  return Object.values(FileAction).includes(value);
}

function isFolderGlobalAction(value: any): value is FolderGlobalAction {
  return Object.values(FolderGlobalAction).includes(value);
}

export { FileAction, FolderGlobalAction, isFileAction, isFolderGlobalAction };

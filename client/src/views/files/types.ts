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
  Preview = 'file-preview',
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

enum FileHandlerAction {
  Details = 'details',
  CopyLink = 'copyLink',
  Download = 'download',
  Edit = 'edit',
  OpenWithSystem = 'openWithSystem',
}

function isFileHandlerAction(value: any): value is FileHandlerAction {
  return Object.values(FileHandlerAction).includes(value);
}

function isFileAction(value: any): value is FileAction {
  return Object.values(FileAction).includes(value);
}

function isFolderGlobalAction(value: any): value is FolderGlobalAction {
  return Object.values(FolderGlobalAction).includes(value);
}

export { FileAction, FileHandlerAction, FolderGlobalAction, isFileAction, isFileHandlerAction, isFolderGlobalAction };

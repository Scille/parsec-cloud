// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export enum FolderGlobalAction {
  CreateFolder = 'folder-create-folder',
  ImportFiles = 'folder-import-files',
  ImportFolder = 'folder-import-folder',
  OpenInExplorer = 'folder-open-in-explorer',
  CopyLink = 'folder-copy-link',
  ToggleSelect = 'folder-toggle-select',
  SelectAll = 'folder-select-all',
  Share = 'folder-share',
  CreateFileDocument = 'folder-create-file-document',
  CreateFileSpreadsheet = 'folder-create-file-spreadsheet',
  CreateFilePresentation = 'folder-create-file-presentation',
  CreateFileText = 'folder-create-file-text',
}

export enum FileAction {
  Preview = 'file-preview',
  Rename = 'file-rename',
  MoveTo = 'file-move-to',
  MakeACopy = 'file-copy-to',
  Delete = 'file-delete',
  Open = 'file-open',
  Edit = 'file-edit',
  ShowHistory = 'file-show-history',
  Download = 'file-download',
  DownloadAsArchive = 'file-download-as-archive',
  ShowDetails = 'file-show-details',
  CopyLink = 'file-copy-link',
  SeeInExplorer = 'file-see-in-explorer',
  ShowEnclosingFolder = 'file-show-enclosing-folder',
}

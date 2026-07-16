// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { FileAction, FolderGlobalAction } from '@/services/contextMenu';

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

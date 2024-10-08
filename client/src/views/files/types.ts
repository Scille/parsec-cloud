// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

enum FolderGlobalAction {
  CreateFolder,
  ImportFiles,
  ImportFolder,
}

enum FileAction {
  Rename,
  MoveTo,
  MakeACopy,
  Delete,
  Open,
  ShowHistory,
  Download,
  ShowDetails,
  CopyLink,
}

export { FileAction, FolderGlobalAction };

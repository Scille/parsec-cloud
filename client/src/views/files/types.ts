// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

enum FolderGlobalAction {
  CreateFolder,
  ImportFiles,
  ImportFolder,
  OpenInExplorer,
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
  SeeInExplorer,
  Select,
  SelectAll,
  UnselectAll,
  Share,
}

export { FileAction, FolderGlobalAction };

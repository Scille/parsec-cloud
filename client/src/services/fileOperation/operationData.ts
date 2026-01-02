// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryStat, EntryTree, FsPath, WorkspaceHandle, WorkspaceID } from '@/parsec';
import { DateTime } from 'luxon';
import { FileSystemFileHandle } from 'native-file-system-adapter';

export enum FileOperationDataType {
  Import = 'import',
  Copy = 'copy',
  Move = 'move',
  Restore = 'restore',
  Download = 'download',
  DownloadArchive = 'download-archive',
}

export type FileOperationID = string;

interface _FileOperationData {
  type: FileOperationDataType;
  id: FileOperationID;
  workspaceHandle: WorkspaceHandle;
  workspaceId: WorkspaceID;
}

export interface FileOperationImportData extends _FileOperationData {
  files: Array<File>;
  destination: FsPath;
  replace: boolean;
}

export interface FileOperationCopyData extends _FileOperationData {
  sources: Array<FsPath>;
  destination: FsPath;
  replace: boolean;
}

export interface FileOperationMoveData extends _FileOperationData {
  sources: Array<FsPath>;
  destination: FsPath;
  replace: boolean;
}

export interface FileOperationRestoreData extends _FileOperationData {
  paths: Array<FsPath>;
  dateTime: DateTime;
  replace: boolean;
}

export interface FileOperationDownloadData extends _FileOperationData {
  path: FsPath;
  dateTime?: DateTime;
  saveHandle: FileSystemFileHandle;
}

export interface FileOperationDownloadArchiveData extends _FileOperationData {
  trees: Array<EntryTree>;
  saveHandle: FileSystemFileHandle;
  rootPath: FsPath;
  totalFiles: number;
  totalSize: number;
}

export type FileOperationData = FileOperationImportData | FileOperationCopyData | FileOperationMoveData | FileOperationRestoreData | FileOperationDownloadData | FileOperationDownloadArchiveData;

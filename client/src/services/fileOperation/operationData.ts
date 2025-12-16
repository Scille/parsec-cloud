// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryName, EntryStat, EntryStatFile, EntryTree, FsPath, WorkspaceHandle, WorkspaceHistoryEntryStat, WorkspaceID } from '@/parsec';
import { DuplicatePolicy } from '@/services/fileOperation/types';
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
  workspaceName: EntryName;
}

export interface FileOperationImportData extends _FileOperationData {
  files: Array<File>;
  destination: FsPath;
  dupPolicy: DuplicatePolicy;
}

export interface FileOperationCopyData extends _FileOperationData {
  sources: Array<EntryStat>;
  destination: FsPath;
  dupPolicy: DuplicatePolicy;
}

export interface FileOperationMoveData extends _FileOperationData {
  sources: Array<EntryStat>;
  destination: FsPath;
  dupPolicy: DuplicatePolicy;
}

export interface FileOperationRestoreData extends _FileOperationData {
  entries: Array<WorkspaceHistoryEntryStat>;
  dateTime: DateTime;
  dupPolicy: DuplicatePolicy;
}

export interface FileOperationDownloadData extends _FileOperationData {
  entry: EntryStatFile;
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

export type FileOperationData =
  | FileOperationImportData
  | FileOperationCopyData
  | FileOperationMoveData
  | FileOperationRestoreData
  | FileOperationDownloadData
  | FileOperationDownloadArchiveData;

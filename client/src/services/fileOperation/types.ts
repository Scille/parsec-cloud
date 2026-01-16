// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryStatFile, FsPath, WorkspaceHandle } from '@/parsec';

export enum OperationFailedErrors {
  MaxRecursionReached = 'max-recursion-reached',
  MaxFilesReached = 'max-files-reached',
  SourceDoesNotExist = 'source-does-not-exist',
  OneFailed = 'one-failed',
  Unhandled = 'unhandled',
  LibParsecCallFailed = 'libparsec-call-failed',
}

export enum DuplicatePolicy {
  Ignore = 'ignore',
  Replace = 'replace',
  AddCounter = 'add-counter',
}

export interface FileEntryPointer {
  workspace: WorkspaceHandle;
  path: FsPath;
}

export type FileProgressCallback = (currentSize: number, totalSize: number) => void;
export type FolderProgressCallback = (currentFile: EntryStatFile, fileIndex: number, currentSize: number, totalSize: number) => void;

export class FileOperationException extends Error {
  public error: OperationFailedErrors;
  public details?: string;

  constructor(error: OperationFailedErrors, details?: string) {
    super(`File operation error '${error}' (${details ?? 'no info'})`);
    this.error = error;
    this.details = details;
    Object.setPrototypeOf(this, new.target.prototype);
  }
}

export class FileOperationCancelled extends Error {
  constructor() {
    super('File operation was cancelled.');
    Object.setPrototypeOf(this, new.target.prototype);
  }
}

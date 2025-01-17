// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Path } from '@/parsec/path';
import {
  FileDescriptor,
  FsPath,
  Result,
  WorkspaceHandle,
  WorkspaceHistoryEntryStat,
  WorkspaceHistoryEntryStatFile,
  WorkspaceHistoryEntryStatFolder,
  WorkspaceHistoryEntryStatTag,
  WorkspaceHistoryFdCloseError,
  WorkspaceHistoryFdReadError,
  WorkspaceHistoryOpenFileError,
  WorkspaceHistoryStatEntryError,
  WorkspaceHistoryStatFolderChildrenError,
} from '@/parsec/types';
import { libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

export async function statFolderChildrenAt(
  handle: WorkspaceHandle,
  path: FsPath,
  at: DateTime,
): Promise<Result<Array<WorkspaceHistoryEntryStat>, WorkspaceHistoryStatFolderChildrenError>> {
  const result = await libparsec.workspaceHistoryStatFolderChildren(handle, at.toSeconds() as any as DateTime, path);
  if (result.ok) {
    const cooked: Array<WorkspaceHistoryEntryStat> = [];
    for (const [name, stat] of result.value) {
      stat.created = DateTime.fromSeconds(stat.created as any as number);
      stat.updated = DateTime.fromSeconds(stat.updated as any as number);
      if (stat.tag === WorkspaceHistoryEntryStatTag.File) {
        (stat as WorkspaceHistoryEntryStatFile).isFile = (): boolean => true;
        (stat as WorkspaceHistoryEntryStatFile).name = name;
        (stat as WorkspaceHistoryEntryStatFile).path = await Path.join(path, name);
      } else {
        (stat as WorkspaceHistoryEntryStatFolder).isFile = (): boolean => false;
        (stat as WorkspaceHistoryEntryStatFolder).name = name;
        (stat as WorkspaceHistoryEntryStatFolder).path = await Path.join(path, name);
      }
      cooked.push(stat as WorkspaceHistoryEntryStat);
    }
    return { ok: true, value: cooked };
  }
  return result;
}

export async function entryStatAt(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  at: DateTime,
): Promise<Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError>> {
  const fileName = (await Path.filename(path)) || '';
  const result = await libparsec.workspaceHistoryStatEntry(workspaceHandle, at.toSeconds() as any as DateTime, path);
  if (result.ok) {
    if (result.value.tag === WorkspaceHistoryEntryStatTag.File) {
      (result.value as WorkspaceHistoryEntryStatFile).isFile = (): boolean => true;
      (result.value as WorkspaceHistoryEntryStatFile).name = fileName;
      (result.value as WorkspaceHistoryEntryStatFile).path = path;
    } else {
      (result.value as WorkspaceHistoryEntryStatFolder).isFile = (): boolean => false;
      (result.value as WorkspaceHistoryEntryStatFolder).name = fileName;
      (result.value as WorkspaceHistoryEntryStatFolder).path = path;
    }
    return result as Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError>;
  }
  return result;
}

export async function openFileAt(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  at: DateTime,
): Promise<Result<FileDescriptor, WorkspaceHistoryOpenFileError>> {
  return await libparsec.workspaceHistoryOpenFile(workspaceHandle, at.toSeconds() as any as DateTime, path);
}

export async function closeHistoryFile(
  workspaceHandle: WorkspaceHandle,
  fd: FileDescriptor,
): Promise<Result<null, WorkspaceHistoryFdCloseError>> {
  return await libparsec.workspaceHistoryFdClose(workspaceHandle, fd);
}

export async function readHistoryFile(
  workspaceHandle: WorkspaceHandle,
  fd: FileDescriptor,
  offset: number,
  size: number,
): Promise<Result<Uint8Array, WorkspaceHistoryFdReadError>> {
  return (await libparsec.workspaceHistoryFdRead(workspaceHandle, fd, BigInt(offset), BigInt(size))) as Result<
    Uint8Array,
    WorkspaceHistoryFdReadError
  >;
}

export interface HistoryEntryTree {
  totalSize: number;
  entries: Array<WorkspaceHistoryEntryStatFile>;
  maxRecursionReached: boolean;
  maxFilesReached: boolean;
}

export async function listTreeAt(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  at: DateTime,
  depthLimit = 12,
  filesLimit = 10000,
): Promise<HistoryEntryTree> {
  async function _innerListTreeAt(workspaceHandle: WorkspaceHandle, path: FsPath, at: DateTime, depth: number): Promise<HistoryEntryTree> {
    const tree: HistoryEntryTree = { totalSize: 0, entries: [], maxRecursionReached: false, maxFilesReached: false };

    if (depth > depthLimit) {
      console.warn('Max depth reached for listTree');
      tree.maxRecursionReached = true;
      return tree;
    }
    const result = await statFolderChildrenAt(workspaceHandle, path, at);
    if (result.ok) {
      for (const entry of result.value) {
        if (tree.maxRecursionReached || tree.maxFilesReached) {
          break;
        }
        if (!entry.isFile()) {
          const subTree = await _innerListTreeAt(workspaceHandle, entry.path, at, depth + 1);
          if (subTree.maxRecursionReached) {
            tree.maxRecursionReached = true;
            return tree;
          }
          if (subTree.maxFilesReached) {
            tree.maxFilesReached = true;
            return tree;
          }
          tree.totalSize += subTree.totalSize;
          tree.entries.push(...subTree.entries);
          if (tree.entries.length > filesLimit) {
            tree.maxFilesReached = true;
          }
        } else {
          tree.totalSize += Number((entry as WorkspaceHistoryEntryStatFile).size);
          tree.entries.push(entry as WorkspaceHistoryEntryStatFile);
          if (tree.entries.length > filesLimit) {
            tree.maxFilesReached = true;
          }
        }
      }
    }
    return tree;
  }

  return await _innerListTreeAt(workspaceHandle, path, at, 0);
}

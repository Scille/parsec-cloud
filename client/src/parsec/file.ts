// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Path } from '@/parsec/path';
import {
  EntryName,
  EntryStat,
  EntryStatFile,
  EntryStatFolder,
  FileDescriptor,
  FileID,
  FileType,
  FsPath,
  OpenOptions,
  ParseParsecAnyAddrError,
  ParseParsecAnyAddrErrorTag,
  ParsedParsecAnyAddrTag,
  ParsedParsecAnyAddrWorkspacePath,
  Result,
  WorkspaceCreateFileError,
  WorkspaceCreateFolderError,
  WorkspaceFdCloseError,
  WorkspaceFdReadError,
  WorkspaceFdResizeError,
  WorkspaceFdWriteError,
  WorkspaceHandle,
  WorkspaceMoveEntryError,
  WorkspaceOpenFileError,
  WorkspaceRemoveEntryError,
  WorkspaceStatEntryError,
  WorkspaceStatFolderChildrenError,
} from '@/parsec/types';
import { getUserInfoFromDeviceID } from '@/parsec/user';
import { MoveEntryModeTag, libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

export const DEFAULT_READ_SIZE = 1_024_000;

export async function createFile(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<Result<FileID, WorkspaceCreateFileError>> {
  return await libparsec.workspaceCreateFile(workspaceHandle, path);
}

export async function createFolder(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<Result<FileID, WorkspaceCreateFolderError>> {
  return await libparsec.workspaceCreateFolderAll(workspaceHandle, path);
}

export async function deleteFile(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<Result<null, WorkspaceRemoveEntryError>> {
  return await libparsec.workspaceRemoveFile(workspaceHandle, path);
}

export async function deleteFolder(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<Result<null, WorkspaceRemoveEntryError>> {
  return await libparsec.workspaceRemoveFolderAll(workspaceHandle, path);
}

export async function rename(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  newName: EntryName,
  canReplace = false,
): Promise<Result<FsPath, WorkspaceMoveEntryError>> {
  const newPath = await Path.join(await Path.parent(path), newName);
  const result = await libparsec.workspaceMoveEntry(workspaceHandle, path, newPath, {
    tag: canReplace ? MoveEntryModeTag.CanReplace : MoveEntryModeTag.NoReplace,
  });
  if (result.ok) {
    return { ok: true, value: newPath };
  }
  return result;
}

export async function entryStat(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<Result<EntryStat, WorkspaceStatEntryError>> {
  const fileName = (await Path.filename(path)) || '';

  const result = await libparsec.workspaceStatEntry(workspaceHandle, path);
  if (result.ok) {
    result.value.created = DateTime.fromSeconds(result.value.created as any as number);
    result.value.updated = DateTime.fromSeconds(result.value.updated as any as number);
    const userInfoResult = await getUserInfoFromDeviceID(result.value.lastUpdater);
    if (result.value.tag === FileType.File) {
      (result.value as unknown as EntryStatFile).size = Number(result.value.size);
      (result.value as unknown as EntryStatFile).isFile = (): boolean => true;
      (result.value as unknown as EntryStatFile).name = fileName;
      (result.value as unknown as EntryStatFile).path = path;
      (result.value as unknown as EntryStatFile).isConfined = (): boolean => result.value.confinementPoint !== null;
      (result.value as unknown as EntryStatFile).lastUpdater = userInfoResult.ok ? userInfoResult.value : undefined;
    } else {
      (result.value as unknown as EntryStatFolder).isFile = (): boolean => false;
      (result.value as unknown as EntryStatFolder).name = fileName;
      (result.value as unknown as EntryStatFolder).path = path;
      (result.value as unknown as EntryStatFolder).isConfined = (): boolean => result.value.confinementPoint !== null;
      (result.value as unknown as EntryStatFolder).lastUpdater = userInfoResult.ok ? userInfoResult.value : undefined;
    }
  }
  return result as Result<EntryStat, WorkspaceStatEntryError>;
}

export async function statFolderChildren(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  excludeConfined = true,
): Promise<Result<Array<EntryStat>, WorkspaceStatFolderChildrenError>> {
  const watchResult = await libparsec.workspaceWatchEntryOneshot(workspaceHandle, path);

  let result;
  if (!watchResult.ok) {
    result = await libparsec.workspaceStatFolderChildren(workspaceHandle, path);
  } else {
    result = await libparsec.workspaceStatFolderChildrenById(workspaceHandle, watchResult.value);
  }

  if (!result.ok) {
    return result;
  }

  const cooked: Array<EntryStat> = [];
  for (const [name, stat] of result.value) {
    if (name === undefined || stat === undefined) {
      continue;
    }
    if (!stat.confinementPoint || !excludeConfined) {
      const userInfoResult = await getUserInfoFromDeviceID(stat.lastUpdater);
      stat.created = DateTime.fromSeconds(stat.created as any as number);
      stat.updated = DateTime.fromSeconds(stat.updated as any as number);
      if (stat.tag === FileType.File) {
        (stat as unknown as EntryStatFile).size = Number(stat.size);
        (stat as unknown as EntryStatFile).isFile = (): boolean => true;
        (stat as unknown as EntryStatFile).name = name;
        (stat as unknown as EntryStatFile).path = await Path.join(path, name);
        (stat as unknown as EntryStatFile).isConfined = (): boolean => stat.confinementPoint !== null;
        (stat as unknown as EntryStatFile).lastUpdater = userInfoResult.ok ? userInfoResult.value : undefined;
      } else {
        (stat as unknown as EntryStatFolder).isFile = (): boolean => false;
        (stat as unknown as EntryStatFolder).name = name;
        (stat as unknown as EntryStatFolder).path = await Path.join(path, name);
        (stat as unknown as EntryStatFolder).isConfined = (): boolean => stat.confinementPoint !== null;
        (stat as unknown as EntryStatFolder).lastUpdater = userInfoResult.ok ? userInfoResult.value : undefined;
      }
      cooked.push(stat as unknown as EntryStat);
    }
  }

  return {
    ok: true,
    value: cooked,
  };
}

export async function moveEntry(
  workspaceHandle: WorkspaceHandle,
  source: FsPath,
  destination: FsPath,
  forceReplace = false,
): Promise<Result<null, WorkspaceMoveEntryError>> {
  return libparsec.workspaceMoveEntry(
    workspaceHandle,
    source,
    destination,
    forceReplace ? { tag: MoveEntryModeTag.CanReplace } : { tag: MoveEntryModeTag.NoReplace },
  );
}

export enum CopyErrorTag {
  Internal = 'Internal',
}

export interface CopyError {
  tag: CopyErrorTag.Internal;
}

export async function parseFileLink(link: string): Promise<Result<ParsedParsecAnyAddrWorkspacePath, ParseParsecAnyAddrError>> {
  const result = await libparsec.parseParsecAnyAddr(link);
  if (result.ok && result.value.tag !== ParsedParsecAnyAddrTag.WorkspacePath) {
    return { ok: false, error: { tag: ParseParsecAnyAddrErrorTag.InvalidUrl, error: 'not a file link' } };
  }
  return result as Result<ParsedParsecAnyAddrWorkspacePath, ParseParsecAnyAddrError>;
}

export async function createReadStream(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  onReadCallback?: (readSize: number) => Promise<void>,
): Promise<ReadableStream> {
  let fd: FileDescriptor | undefined;
  let offset = 0;

  return new ReadableStream({
    async start(controller: ReadableStreamDefaultController): Promise<void> {
      const result = await openFile(workspaceHandle, path, { read: true });
      if (!result.ok) {
        return controller.error(result.error);
      }
      fd = result.value;
    },

    async pull(controller: ReadableStreamDefaultController): Promise<void> {
      if (fd === undefined) {
        controller.error('No file descriptor');
        return;
      }
      const result = await readFile(workspaceHandle, fd, offset, DEFAULT_READ_SIZE);

      if (!result.ok) {
        return controller.error(result.error);
      }
      if (result.value.length === 0) {
        controller.close();
        await closeFile(workspaceHandle, fd);
        fd = undefined;
      } else {
        offset += result.value.length;
        if (onReadCallback) {
          await onReadCallback(result.value.length);
        }
        // Keep at the end
        controller.enqueue(result.value);
      }
    },

    async cancel(): Promise<void> {
      if (fd !== undefined) {
        await closeFile(workspaceHandle, fd);
      }
    },

    type: 'bytes',
  });
}

export async function openFile(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  options: OpenOptions,
): Promise<Result<FileDescriptor, WorkspaceOpenFileError>> {
  const parsecOptions = {
    read: options.read ? true : false,
    write: options.write ? true : false,
    append: options.append ? true : false,
    truncate: options.truncate ? true : false,
    create: options.create ? true : false,
    createNew: options.createNew ? true : false,
  };

  return await libparsec.workspaceOpenFile(workspaceHandle, path, parsecOptions);
}

export async function closeFile(workspaceHandle: WorkspaceHandle, fd: FileDescriptor): Promise<Result<null, WorkspaceFdCloseError>> {
  return await libparsec.workspaceFdClose(workspaceHandle, fd);
}

export async function resizeFile(
  workspaceHandle: WorkspaceHandle,
  fd: FileDescriptor,
  length: number,
): Promise<Result<null, WorkspaceFdResizeError>> {
  return await libparsec.workspaceFdResize(workspaceHandle, fd, BigInt(length), true);
}

export async function writeFile(
  workspaceHandle: WorkspaceHandle,
  fd: FileDescriptor,
  offset: number,
  data: Uint8Array,
): Promise<Result<number, WorkspaceFdWriteError>> {
  const result = await libparsec.workspaceFdWrite(workspaceHandle, fd, BigInt(offset), data);
  if (result.ok) {
    return { ok: true, value: Number(result.value) };
  }
  return result;
}

export async function readFile(
  workspaceHandle: WorkspaceHandle,
  fd: FileDescriptor,
  offset: number,
  size: number,
): Promise<Result<Uint8Array, WorkspaceFdReadError>> {
  return await libparsec.workspaceFdRead(workspaceHandle, fd, BigInt(offset), BigInt(size));
}

export interface EntryTree {
  totalSize: number;
  entries: Array<EntryStatFile>;
  maxRecursionReached: boolean;
  maxFilesReached: boolean;
}

export async function listTree(workspaceHandle: WorkspaceHandle, path: FsPath, depthLimit = 12, filesLimit = 10000): Promise<EntryTree> {
  async function _innerListTree(workspaceHandle: WorkspaceHandle, path: FsPath, depth: number): Promise<EntryTree> {
    const tree: EntryTree = { totalSize: 0, entries: [], maxRecursionReached: false, maxFilesReached: false };

    if (depth > depthLimit) {
      console.warn('Max depth reached for listTree');
      tree.maxRecursionReached = true;
      return tree;
    }
    const result = await statFolderChildren(workspaceHandle, path);
    if (result.ok) {
      for (const entry of result.value) {
        if (tree.maxRecursionReached || tree.maxFilesReached) {
          break;
        }
        if (!entry.isFile()) {
          const subTree = await _innerListTree(workspaceHandle, entry.path, depth + 1);
          if (subTree.maxRecursionReached) {
            tree.maxRecursionReached = true;
            return tree;
          }
          if (subTree.maxFilesReached) {
            console.warn('Max file count reached for listTree');
            tree.maxFilesReached = true;
            return tree;
          }
          tree.totalSize += subTree.totalSize;
          tree.entries.push(...subTree.entries);
          if (tree.entries.length > filesLimit) {
            tree.maxFilesReached = true;
          }
        } else {
          tree.totalSize += Number((entry as EntryStatFile).size);
          tree.entries.push(entry as EntryStatFile);
          if (tree.entries.length > filesLimit) {
            tree.maxFilesReached = true;
          }
        }
      }
    }
    return tree;
  }

  return await _innerListTree(workspaceHandle, path, 0);
}

export async function isFileContentAvailable(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<boolean> {
  const result = await libparsec.workspaceIsFileContentLocal(workspaceHandle, path);

  return result.ok && result.value;
}

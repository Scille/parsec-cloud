// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { wait } from '@/parsec/internals';
import { MockEntry, generateEntries, generateFile, generateFolder } from '@/parsec/mock_generator';
import { Path } from '@/parsec/path';
import { getParsecHandle, getWorkspaceHandle } from '@/parsec/routing';
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
  ParseParsecAddrError,
  ParseParsecAddrErrorTag,
  ParsedParsecAddrWorkspacePath,
  Result,
  WorkspaceCreateFileError,
  WorkspaceCreateFolderError,
  WorkspaceCreateFolderErrorTag,
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
import { MoveEntryModeTag, ParsedParsecAddrTag, libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

export async function createFile(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<Result<FileID, WorkspaceCreateFileError>> {
  if (!needsMocks()) {
    return await libparsec.workspaceCreateFile(workspaceHandle, path);
  } else {
    return { ok: true, value: '42' };
  }
}

export async function createFolder(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<Result<FileID, WorkspaceCreateFolderError>> {
  if (!needsMocks()) {
    return await libparsec.workspaceCreateFolderAll(workspaceHandle, path);
  } else {
    return { ok: false, error: { tag: WorkspaceCreateFolderErrorTag.EntryExists, error: 'already exists' } };
  }
}

export async function deleteFile(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<Result<null, WorkspaceRemoveEntryError>> {
  if (!needsMocks()) {
    return await libparsec.workspaceRemoveFile(workspaceHandle, path);
  } else {
    return { ok: true, value: null };
  }
}

export async function deleteFolder(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<Result<null, WorkspaceRemoveEntryError>> {
  if (!needsMocks()) {
    return await libparsec.workspaceRemoveFolderAll(workspaceHandle, path);
  } else {
    return { ok: true, value: null };
  }
}

export async function rename(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  newName: EntryName,
): Promise<Result<FsPath, WorkspaceMoveEntryError>> {
  if (!needsMocks()) {
    const newPath = await Path.join(await Path.parent(path), newName);
    const result = await libparsec.workspaceMoveEntry(workspaceHandle, path, newPath, { tag: MoveEntryModeTag.NoReplace });
    if (result.ok) {
      return { ok: true, value: newPath };
    }
    return result;
  } else {
    return { ok: true, value: '/a/b.txt' };
  }
}

let MOCK_FILE_ID = 1;

export async function entryStat(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<Result<EntryStat, WorkspaceStatEntryError>> {
  const fileName = (await Path.filename(path)) || '';

  if (!needsMocks()) {
    const result = await libparsec.workspaceStatEntry(workspaceHandle, path);
    if (result.ok) {
      result.value.created = DateTime.fromSeconds(result.value.created as any as number);
      result.value.updated = DateTime.fromSeconds(result.value.updated as any as number);
      if (result.value.tag === FileType.File) {
        (result.value as EntryStatFile).isFile = (): boolean => true;
        (result.value as EntryStatFile).name = fileName;
        (result.value as EntryStatFile).path = path;
        (result.value as EntryStatFile).isConfined = (): boolean => result.value.confinementPoint !== null;
      } else {
        (result.value as EntryStatFolder).isFile = (): boolean => false;
        (result.value as EntryStatFolder).name = fileName;
        (result.value as EntryStatFolder).path = path;
        (result.value as EntryStatFolder).isConfined = (): boolean => result.value.confinementPoint !== null;
      }
    }
    return result as Result<EntryStat, WorkspaceStatEntryError>;
  }

  MOCK_FILE_ID += 1;

  let entry: MockEntry;

  if (path !== '/' && fileName.startsWith('File_')) {
    entry = await generateFile(path, `${MOCK_FILE_ID}`);
  } else {
    entry = await generateFolder(path, `${MOCK_FILE_ID}`);
  }
  (entry as any as EntryStat).baseVersion = entry.version;
  (entry as any as EntryStat).confinementPoint = null;
  (entry as any as EntryStat).isConfined = (): boolean => false;
  (entry as any as EntryStat).needSync = Math.floor(Math.random() * 2) === 1;
  (entry as any as EntryStat).isPlaceholder = false;
  return { ok: true, value: entry as any as EntryStat };
}

export async function statFolderChildren(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
): Promise<Result<Array<EntryStat>, WorkspaceStatFolderChildrenError>> {
  if (!needsMocks()) {
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
      stat.created = DateTime.fromSeconds(stat.created as any as number);
      stat.updated = DateTime.fromSeconds(stat.updated as any as number);
      if (stat.tag === FileType.File) {
        (stat as EntryStatFile).isFile = (): boolean => true;
        (stat as EntryStatFile).name = name;
        (stat as EntryStatFile).path = await Path.join(path, name);
        (stat as EntryStatFile).isConfined = (): boolean => stat.confinementPoint !== null;
      } else {
        (stat as EntryStatFolder).isFile = (): boolean => false;
        (stat as EntryStatFolder).name = name;
        (stat as EntryStatFolder).path = await Path.join(path, name);
        (stat as EntryStatFolder).isConfined = (): boolean => stat.confinementPoint !== null;
      }
      cooked.push(stat as EntryStat);
    }

    return {
      ok: true,
      value: cooked,
    };
  }

  const items = (await generateEntries(path)).map((entry) => {
    (entry as any as EntryStat).baseVersion = entry.version;
    (entry as any as EntryStat).confinementPoint = null;
    (entry as any as EntryStat).isConfined = (): boolean => false;
    (entry as any as EntryStat).needSync = Math.floor(Math.random() * 2) === 1;
    (entry as any as EntryStat).isPlaceholder = false;
    return entry as any as EntryStat;
  });
  return { ok: true, value: items };
}

export async function moveEntry(
  workspaceHandle: WorkspaceHandle,
  source: FsPath,
  destination: FsPath,
  forceReplace = false,
): Promise<Result<null, WorkspaceMoveEntryError>> {
  if (workspaceHandle && !needsMocks()) {
    return libparsec.workspaceMoveEntry(
      workspaceHandle,
      source,
      destination,
      forceReplace ? { tag: MoveEntryModeTag.CanReplace } : { tag: MoveEntryModeTag.NoReplace },
    );
  } else {
    return { ok: true, value: null };
  }
}

export enum CopyErrorTag {
  Internal = 'Internal',
}

export interface CopyError {
  tag: CopyErrorTag.Internal;
}

export async function copyEntry(_source: FsPath, _destination: FsPath): Promise<Result<null, CopyError>> {
  const clientHandle = getParsecHandle();
  const workspaceHandle = getWorkspaceHandle();

  if (clientHandle && workspaceHandle && !needsMocks()) {
    return { ok: true, value: null };
  } else {
    return { ok: true, value: null };
  }
}

export async function parseFileLink(link: string): Promise<Result<ParsedParsecAddrWorkspacePath, ParseParsecAddrError>> {
  const result = await libparsec.parseParsecAddr(link);
  if (result.ok && result.value.tag !== ParsedParsecAddrTag.WorkspacePath) {
    return { ok: false, error: { tag: ParseParsecAddrErrorTag.InvalidUrl, error: 'not a file link' } };
  }
  return result as Result<ParsedParsecAddrWorkspacePath, ParseParsecAddrError>;
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

  if (workspaceHandle && !needsMocks()) {
    return await libparsec.workspaceOpenFile(workspaceHandle, path, parsecOptions);
  } else {
    return { ok: true, value: 42 };
  }
}

export async function closeFile(workspaceHandle: WorkspaceHandle, fd: FileDescriptor): Promise<Result<null, WorkspaceFdCloseError>> {
  if (!needsMocks()) {
    return await libparsec.workspaceFdClose(workspaceHandle, fd);
  } else {
    return { ok: true, value: null };
  }
}

export async function resizeFile(
  workspaceHandle: WorkspaceHandle,
  fd: FileDescriptor,
  length: number,
): Promise<Result<null, WorkspaceFdResizeError>> {
  if (workspaceHandle && !needsMocks()) {
    return await libparsec.workspaceFdResize(workspaceHandle, fd, length, true);
  } else {
    return { ok: true, value: null };
  }
}

export async function writeFile(
  workspaceHandle: WorkspaceHandle,
  fd: FileDescriptor,
  offset: number,
  data: Uint8Array,
): Promise<Result<number, WorkspaceFdWriteError>> {
  if (!needsMocks()) {
    return await libparsec.workspaceFdWrite(workspaceHandle, fd, offset, data);
  } else {
    await wait(100);
    return { ok: true, value: data.length };
  }
}

export async function readFile(
  workspaceHandle: WorkspaceHandle,
  fd: FileDescriptor,
  offset: number,
  size: number,
): Promise<Result<ArrayBuffer, WorkspaceFdReadError>> {
  if (!needsMocks()) {
    return await libparsec.workspaceFdRead(workspaceHandle, fd, offset, size);
  } else {
    await wait(100);
    return { ok: true, value: new Uint8Array([77, 97, 120, 32, 105, 115, 32, 115, 101, 120, 121]) };
  }
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
            tree.maxFilesReached = true;
            return tree;
          }
          tree.totalSize += subTree.totalSize;
          tree.entries.push(...subTree.entries);
          if (tree.entries.length > filesLimit) {
            tree.maxFilesReached = true;
          }
        } else {
          tree.totalSize += (entry as EntryStatFile).size;
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

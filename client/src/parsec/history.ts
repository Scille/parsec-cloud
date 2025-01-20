// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { wait } from '@/parsec/internals';
import { MockFiles } from '@/parsec/mock_files';
import { generateEntries, generateFile, generateFolder } from '@/parsec/mock_generator';
import { Path } from '@/parsec/path';
import {
  EntryName,
  FileDescriptor,
  FsPath,
  Result,
  WorkspaceHandle,
  WorkspaceHistoryEntryStat,
  WorkspaceHistoryEntryStatFile,
  WorkspaceHistoryEntryStatFolder,
  WorkspaceHistoryEntryStatTag,
  WorkspaceHistoryFdCloseError,
  WorkspaceHistoryFdCloseErrorTag,
  WorkspaceHistoryFdReadError,
  WorkspaceHistoryFdReadErrorTag,
  WorkspaceHistoryOpenFileError,
  WorkspaceHistoryStatEntryError,
  WorkspaceHistoryStatFolderChildrenError,
} from '@/parsec/types';
import type { U64 } from '@/plugins/libparsec';
import { libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

const MOCK_OPENED_FILES = new Map<FileDescriptor, FsPath>();
let MOCK_CURRENT_FD = 1;

export async function statFolderChildrenAt(
  handle: WorkspaceHandle,
  path: FsPath,
  at: DateTime,
): Promise<Result<Array<WorkspaceHistoryEntryStat>, WorkspaceHistoryStatFolderChildrenError>> {
  if (!needsMocks()) {
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
  // Take some time to load
  await wait(1500);
  const items = (await generateEntries(path)).map((entry) => {
    return entry as any as WorkspaceHistoryEntryStat;
  });
  return { ok: true, value: items };
}

export async function entryStatAt(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  at: DateTime,
): Promise<Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError>> {
  const fileName = (await Path.filename(path)) || '';

  if (!needsMocks()) {
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
  const entry = fileName.startsWith('File_') ? await generateFile(path, { fileName: fileName }) : await generateFolder(path);
  return { ok: true, value: entry as any as WorkspaceHistoryEntryStat };
}

export async function openFileAt(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  at: DateTime,
): Promise<Result<FileDescriptor, WorkspaceHistoryOpenFileError>> {
  if (workspaceHandle && !needsMocks()) {
    const result = await libparsec.workspaceHistoryOpenFile(workspaceHandle, at.toSeconds() as any as DateTime, path);
    return result;
  } else {
    const fd = MOCK_CURRENT_FD;
    MOCK_CURRENT_FD += 1;
    MOCK_OPENED_FILES.set(fd, path);
    return { ok: true, value: fd };
  }
}

export async function closeHistoryFile(
  workspaceHandle: WorkspaceHandle,
  fd: FileDescriptor,
): Promise<Result<null, WorkspaceHistoryFdCloseError>> {
  if (!needsMocks()) {
    return await libparsec.workspaceHistoryFdClose(workspaceHandle, fd);
  } else {
    if (!MOCK_OPENED_FILES.has(fd)) {
      return { ok: false, error: { tag: WorkspaceHistoryFdCloseErrorTag.BadFileDescriptor, error: 'Invalid file descriptor' } };
    }
    MOCK_OPENED_FILES.delete(fd);
    return { ok: true, value: null };
  }
}

export async function readHistoryFile(
  workspaceHandle: WorkspaceHandle,
  fd: FileDescriptor,
  offset: U64,
  size: U64,
): Promise<Result<ArrayBuffer, WorkspaceHistoryFdReadError>> {
  if (!needsMocks()) {
    return await libparsec.workspaceHistoryFdRead(workspaceHandle, fd, offset, size);
  } else {
    if (!MOCK_OPENED_FILES.has(fd)) {
      return { ok: false, error: { tag: WorkspaceHistoryFdReadErrorTag.BadFileDescriptor, error: 'Invalid file descriptor' } };
    }
    await wait(100);
    const path = MOCK_OPENED_FILES.get(fd) as string;
    const fileName = (await Path.filename(path)) as EntryName;
    const ext = Path.getFileExtension(fileName);

    switch (ext) {
      case 'xlsx':
        console.log('Using XLSX content');
        return { ok: true, value: MockFiles.XLSX };
      case 'png':
        console.log('Using PNG content');
        return { ok: true, value: MockFiles.PNG };
      case 'docx':
        console.log('Using DOCX content');
        return { ok: true, value: MockFiles.DOCX };
      case 'txt':
        console.log('Using TXT content');
        return { ok: true, value: MockFiles.TXT };
      case 'py':
        console.log('Using PY content');
        return { ok: true, value: MockFiles.PY };
      case 'pdf':
        console.log('Using PDF content');
        return { ok: true, value: MockFiles.PDF };
      case 'mp3':
        console.log('Using MP3 content');
        return { ok: true, value: MockFiles.MP3 };
      case 'mp4':
        console.log('Using MP4 content');
        return { ok: true, value: MockFiles.MP4 };
    }
    console.log('Using default file content');
    return {
      ok: true,
      value: new Uint8Array([
        137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 5, 0, 0, 0, 5, 8, 6, 0, 0, 0, 141, 111, 38, 229, 0, 0, 0, 28,
        73, 68, 65, 84, 8, 215, 99, 248, 255, 255, 63, 195, 127, 6, 32, 5, 195, 32, 18, 132, 208, 49, 241, 130, 88, 205, 4, 0, 14, 245, 53,
        203, 209, 142, 14, 31, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
      ]),
    };
  }
}

export interface HistoryEntryTree {
  totalSize: U64;
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
    const tree: HistoryEntryTree = { totalSize: 0n, entries: [], maxRecursionReached: false, maxFilesReached: false };

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
          tree.totalSize += (entry as WorkspaceHistoryEntryStatFile).size;
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

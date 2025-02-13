// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { wait } from '@/parsec/internals';
import { MockFileType, getMockFileContent } from '@/parsec/mock_files';
import { MockEntry, generateEntriesForEachFileType, generateFile, generateFolder } from '@/parsec/mock_generator';
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
  ParsedParsecAddrTag,
  ParsedParsecAddrWorkspacePath,
  Result,
  WorkspaceCreateFileError,
  WorkspaceCreateFolderError,
  WorkspaceCreateFolderErrorTag,
  WorkspaceFdCloseError,
  WorkspaceFdCloseErrorTag,
  WorkspaceFdReadError,
  WorkspaceFdReadErrorTag,
  WorkspaceFdResizeError,
  WorkspaceFdResizeErrorTag,
  WorkspaceFdWriteError,
  WorkspaceFdWriteErrorTag,
  WorkspaceHandle,
  WorkspaceMoveEntryError,
  WorkspaceOpenFileError,
  WorkspaceRemoveEntryError,
  WorkspaceStatEntryError,
  WorkspaceStatFolderChildrenError,
} from '@/parsec/types';
import { MoveEntryModeTag, libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

const MOCK_OPENED_FILES = new Map<FileDescriptor, FsPath>();
let MOCK_CURRENT_FD = 1;

export const DEFAULT_READ_SIZE = 256_000;

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
    entry = await generateFile(await Path.parent(path), { parentId: `${MOCK_FILE_ID}`, fileName: fileName });
  } else {
    entry = await generateFolder(path, { parentId: `${MOCK_FILE_ID}`, fileName: fileName });
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
  excludeConfined = true,
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
      if (!stat.confinementPoint || !excludeConfined) {
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
    }

    return {
      ok: true,
      value: cooked,
    };
  }

  await wait(500);
  const items = (await generateEntriesForEachFileType(path)).map((entry) => {
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

export async function createReadStream(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<ReadableStream> {
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

  if (workspaceHandle && !needsMocks()) {
    return await libparsec.workspaceOpenFile(workspaceHandle, path, parsecOptions);
  } else {
    const fd = MOCK_CURRENT_FD;
    MOCK_CURRENT_FD += 1;
    MOCK_OPENED_FILES.set(fd, path);
    return { ok: true, value: fd };
  }
}

export async function closeFile(workspaceHandle: WorkspaceHandle, fd: FileDescriptor): Promise<Result<null, WorkspaceFdCloseError>> {
  if (!needsMocks()) {
    return await libparsec.workspaceFdClose(workspaceHandle, fd);
  } else {
    if (!MOCK_OPENED_FILES.has(fd)) {
      return { ok: false, error: { tag: WorkspaceFdCloseErrorTag.BadFileDescriptor, error: 'Invalid file descriptor' } };
    }
    MOCK_OPENED_FILES.delete(fd);
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
    if (!MOCK_OPENED_FILES.has(fd)) {
      return { ok: false, error: { tag: WorkspaceFdResizeErrorTag.BadFileDescriptor, error: 'Invalid file descriptor' } };
    }
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
    if (!MOCK_OPENED_FILES.has(fd)) {
      return { ok: false, error: { tag: WorkspaceFdWriteErrorTag.BadFileDescriptor, error: 'Invalid file descriptor' } };
    }
    await wait(100);
    return { ok: true, value: data.length };
  }
}

export async function readFile(
  workspaceHandle: WorkspaceHandle,
  fd: FileDescriptor,
  offset: number,
  size: number,
): Promise<Result<Uint8Array, WorkspaceFdReadError>> {
  if (!needsMocks()) {
    return await libparsec.workspaceFdRead(workspaceHandle, fd, offset, size);
  } else {
    if (!MOCK_OPENED_FILES.has(fd)) {
      return { ok: false, error: { tag: WorkspaceFdReadErrorTag.BadFileDescriptor, error: 'Invalid file descriptor' } };
    }
    await wait(100);
    const path = MOCK_OPENED_FILES.get(fd) as string;
    const fileName = (await Path.filename(path)) as EntryName;
    const ext = Path.getFileExtension(fileName);

    switch (ext) {
      case 'xlsx':
        offset === 0 && console.log('Using XLSX content');
        return { ok: true, value: (await getMockFileContent(MockFileType.XLSX)).slice(offset, offset + size) };
      case 'png':
        offset === 0 && console.log('Using PNG content');
        return { ok: true, value: (await getMockFileContent(MockFileType.PNG)).slice(offset, offset + size) };
      case 'jpg':
        offset === 0 && console.log('Using JPG content');
        return { ok: true, value: (await getMockFileContent(MockFileType.JPG)).slice(offset, offset + size) };
      case 'docx':
        offset === 0 && console.log('Using DOCX content');
        return { ok: true, value: (await getMockFileContent(MockFileType.DOCX)).slice(offset, offset + size) };
      case 'txt':
        offset === 0 && console.log('Using TXT content');
        return { ok: true, value: (await getMockFileContent(MockFileType.TXT)).slice(offset, offset + size) };
      case 'py':
        offset === 0 && console.log('Using PY content');
        return { ok: true, value: (await getMockFileContent(MockFileType.PY)).slice(offset, offset + size) };
      case 'pdf':
        offset === 0 && console.log('Using PDF content');
        return { ok: true, value: (await getMockFileContent(MockFileType.PDF)).slice(offset, offset + size) };
      case 'mp3':
        offset === 0 && console.log('Using MP3 content');
        return { ok: true, value: (await getMockFileContent(MockFileType.MP3)).slice(offset, offset + size) };
      case 'mp4':
        offset === 0 && console.log('Using MP4 content');
        return { ok: true, value: (await getMockFileContent(MockFileType.MP4)).slice(offset, offset + size) };
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

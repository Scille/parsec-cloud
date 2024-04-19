// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
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
  WorkspaceFdResizeError,
  WorkspaceFdWriteError,
  WorkspaceHandle,
  WorkspaceOpenFileError,
  WorkspaceRemoveEntryError,
  WorkspaceRenameEntryError,
  WorkspaceStatEntryError,
  WorkspaceStatFolderChildrenError,
} from '@/parsec/types';
import { ParsedParsecAddrTag, libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';
import { adjectives, animals, uniqueNamesGenerator } from 'unique-names-generator';

export async function createFile(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<Result<FileID, WorkspaceCreateFileError>> {
  const clientHandle = getParsecHandle();

  if (clientHandle && !needsMocks()) {
    return await libparsec.workspaceCreateFile(workspaceHandle, path);
  } else {
    return { ok: true, value: '42' };
  }
}

export async function createFolder(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<Result<FileID, WorkspaceCreateFolderError>> {
  const clientHandle = getParsecHandle();

  if (clientHandle && !needsMocks()) {
    return await libparsec.workspaceCreateFolderAll(workspaceHandle, path);
  } else {
    return { ok: false, error: { tag: WorkspaceCreateFolderErrorTag.EntryExists, error: 'already exists' } };
    // return { ok: true, value: '42' };
  }
}

export async function deleteFile(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<Result<null, WorkspaceRemoveEntryError>> {
  const clientHandle = getParsecHandle();

  if (clientHandle && !needsMocks()) {
    return await libparsec.workspaceRemoveFile(workspaceHandle, path);
  } else {
    return { ok: true, value: null };
  }
}

export async function deleteFolder(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<Result<null, WorkspaceRemoveEntryError>> {
  const clientHandle = getParsecHandle();

  if (clientHandle && !needsMocks()) {
    return await libparsec.workspaceRemoveFolderAll(workspaceHandle, path);
  } else {
    return { ok: true, value: null };
  }
}

export async function rename(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  newName: EntryName,
): Promise<Result<null, WorkspaceRenameEntryError>> {
  const clientHandle = getParsecHandle();

  if (clientHandle && !needsMocks()) {
    return await libparsec.workspaceRenameEntry(workspaceHandle, path, newName, false);
  } else {
    return { ok: true, value: null };
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
      } else {
        (result.value as EntryStatFolder).isFile = (): boolean => false;
        (result.value as EntryStatFolder).name = fileName;
      }
    }
    return result as Result<EntryStat, WorkspaceStatEntryError>;
  }

  // Mocked version

  function generateDate(start?: DateTime): DateTime {
    if (!start) {
      start = DateTime.now();
    }
    return DateTime.now().minus({ minutes: Math.floor(Math.random() * 60), seconds: Math.floor(Math.random() * 60) });
  }

  const FILE_PREFIX = 'File_';

  MOCK_FILE_ID += 1;

  const createdDate = generateDate();
  if (path !== '/' && fileName.startsWith(FILE_PREFIX)) {
    return {
      ok: true,
      value: {
        tag: FileType.File,
        confinementPoint: null,
        id: `${MOCK_FILE_ID}`,
        // Invalid parent ID, but hard to craft the correct one...
        parent: `${MOCK_FILE_ID}`,
        created: createdDate,
        updated: generateDate(createdDate),
        baseVersion: 1,
        isPlaceholder: false,
        needSync: Math.floor(Math.random() * 2) === 1,
        size: Math.floor(Math.random() * 1_000_000),
        isFile: (): boolean => true,
        name: fileName,
        path: path,
      },
    };
  } else {
    return {
      ok: true,
      value: {
        tag: FileType.Folder,
        confinementPoint: null,
        id: `${MOCK_FILE_ID}`,
        // Invalid parent ID, but hard to craft the correct one...
        parent: `${MOCK_FILE_ID}`,
        created: createdDate,
        updated: generateDate(createdDate),
        baseVersion: 1,
        isPlaceholder: false,
        needSync: Math.floor(Math.random() * 2) === 1,
        isFile: (): boolean => false,
        name: fileName,
        path: path,
      },
    };
  }
}

export async function statFolderChildren(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
): Promise<Result<Array<EntryStat>, WorkspaceStatFolderChildrenError>> {
  if (!needsMocks()) {
    const result = await libparsec.workspaceStatFolderChildren(workspaceHandle, path);
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
        (stat as EntryStatFile).path = path;
      } else {
        (stat as EntryStatFolder).isFile = (): boolean => false;
        (stat as EntryStatFolder).name = name;
        (stat as EntryStatFolder).path = path;
      }
      cooked.push(stat as EntryStat);
    }

    return {
      ok: true,
      value: cooked,
    };
  }

  // Mocked version

  const FILE_PREFIX = 'File_';
  const FOLDER_PREFIX = 'Dir_';
  const fileCount = Math.floor(Math.random() * 1) + 2;
  const folderCount = Math.floor(Math.random() * 1) + 2;

  function generateEntryName(prefix: string = '', addExtension = false): string {
    const EXTENSIONS = ['.mp4', '.docx', '.pdf', '.png', '.mp3', '.xls', '.zip'];
    const ext = addExtension ? EXTENSIONS[Math.floor(Math.random() * EXTENSIONS.length)] : '';
    return `${prefix}${uniqueNamesGenerator({ dictionaries: [adjectives, animals] })}${ext}`;
  }

  function generateDate(start?: DateTime): DateTime {
    if (!start) {
      start = DateTime.now();
    }
    return DateTime.now().minus({ minutes: Math.floor(Math.random() * 60), seconds: Math.floor(Math.random() * 60) });
  }

  const items: Array<EntryStat> = [];

  const parentId = crypto.randomUUID().toString();

  // Add files
  for (let i = 0; i < fileCount; i++) {
    const name = generateEntryName(FILE_PREFIX, true);
    const createdDate = generateDate();
    const stat: EntryStat = {
      tag: FileType.File,
      confinementPoint: null,
      id: crypto.randomUUID().toString(),
      parent: parentId,
      created: createdDate,
      updated: generateDate(createdDate),
      baseVersion: 1,
      isPlaceholder: false,
      needSync: Math.floor(Math.random() * 2) === 1,
      size: Math.floor(Math.random() * 1_000_000),
      isFile: (): boolean => true,
      name: name,
      path: path,
    };

    items.push(stat);
  }

  // Add folders
  for (let i = 0; i < folderCount; i++) {
    const name = generateEntryName(FOLDER_PREFIX, false);
    const createdDate = generateDate();
    const stat: EntryStat = {
      tag: FileType.Folder,
      confinementPoint: null,
      id: crypto.randomUUID().toString(),
      parent: parentId,
      created: createdDate,
      updated: generateDate(createdDate),
      baseVersion: 1,
      isPlaceholder: false,
      needSync: Math.floor(Math.random() * 2) === 1,
      isFile: (): boolean => false,
      name: name,
      path: path,
    };

    items.push(stat);
  }

  return {
    ok: true,
    value: items,
  };
}

export enum MoveErrorTag {
  Internal = 'Internal',
}

export interface MoveError {
  tag: MoveErrorTag.Internal;
}

export async function moveEntry(_source: FsPath, _destination: FsPath): Promise<Result<null, MoveError>> {
  const clientHandle = getParsecHandle();
  const workspaceHandle = getWorkspaceHandle();

  if (clientHandle && workspaceHandle && !needsMocks()) {
    return { ok: true, value: null };
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
  const clientHandle = getParsecHandle();

  const parsecOptions = {
    read: options.read ? true : false,
    write: options.write ? true : false,
    append: options.append ? true : false,
    truncate: options.truncate ? true : false,
    create: options.create ? true : false,
    createNew: options.createNew ? true : false,
  };

  if (clientHandle && workspaceHandle && !needsMocks()) {
    return await libparsec.workspaceOpenFile(workspaceHandle, path, parsecOptions);
  } else {
    return { ok: true, value: 42 };
  }
}

export async function closeFile(workspaceHandle: WorkspaceHandle, fd: FileDescriptor): Promise<Result<null, WorkspaceFdCloseError>> {
  const clientHandle = getParsecHandle();

  if (clientHandle && !needsMocks()) {
    return await libparsec.fdClose(workspaceHandle, fd);
  } else {
    return { ok: true, value: null };
  }
}

export async function resizeFile(
  workspaceHandle: WorkspaceHandle,
  fd: FileDescriptor,
  length: number,
): Promise<Result<null, WorkspaceFdResizeError>> {
  const clientHandle = getParsecHandle();

  if (clientHandle && workspaceHandle && !needsMocks()) {
    return await libparsec.fdResize(workspaceHandle, fd, length, true);
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
  const clientHandle = getParsecHandle();

  if (clientHandle && !needsMocks()) {
    return await libparsec.fdWrite(workspaceHandle, fd, offset, data);
  } else {
    return { ok: true, value: 1337 };
  }
}

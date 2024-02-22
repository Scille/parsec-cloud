// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { Path } from '@/parsec/path';
import { getParsecHandle, getWorkspaceHandle } from '@/parsec/routing';
import {
  EntryName,
  EntryStat,
  EntryStatFile,
  EntryStatFolder,
  FileID,
  FileType,
  FsPath,
  GetAbsolutePathError,
  GetAbsolutePathErrorTag,
  OrganizationID,
  Result,
  WorkspaceCreateFileError,
  WorkspaceCreateFolderError,
  WorkspaceHandle,
  WorkspaceID,
  WorkspaceRemoveEntryError,
  WorkspaceRenameEntryError,
  WorkspaceStatEntryError,
} from '@/parsec/types';
import { libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';
import { adjectives, animals, uniqueNamesGenerator } from 'unique-names-generator';

export async function createFile(path: FsPath): Promise<Result<FileID, WorkspaceCreateFileError>> {
  const clientHandle = getParsecHandle();
  const workspaceHandle = getWorkspaceHandle();

  if (clientHandle && workspaceHandle && !needsMocks()) {
    return await libparsec.workspaceCreateFile(workspaceHandle, path);
  } else {
    return { ok: true, value: '42' };
  }
}

export async function createFolder(path: FsPath): Promise<Result<FileID, WorkspaceCreateFolderError>> {
  const clientHandle = getParsecHandle();
  const workspaceHandle = getWorkspaceHandle();

  if (clientHandle && workspaceHandle && !needsMocks()) {
    return await libparsec.workspaceCreateFolder(workspaceHandle, path);
  } else {
    return { ok: true, value: '7' };
  }
}

export async function deleteFile(path: FsPath): Promise<Result<null, WorkspaceRemoveEntryError>> {
  const clientHandle = getParsecHandle();
  const workspaceHandle = getWorkspaceHandle();

  if (clientHandle && workspaceHandle && !needsMocks()) {
    return await libparsec.workspaceRemoveFile(workspaceHandle, path);
  } else {
    return { ok: true, value: null };
  }
}

export async function deleteFolder(path: FsPath): Promise<Result<null, WorkspaceRemoveEntryError>> {
  const clientHandle = getParsecHandle();
  const workspaceHandle = getWorkspaceHandle();

  if (clientHandle && workspaceHandle && !needsMocks()) {
    return await libparsec.workspaceRemoveFolder(workspaceHandle, path);
  } else {
    return { ok: true, value: null };
  }
}

export async function rename(path: FsPath, newName: EntryName): Promise<Result<null, WorkspaceRenameEntryError>> {
  const clientHandle = getParsecHandle();
  const workspaceHandle = getWorkspaceHandle();

  if (clientHandle && workspaceHandle && !needsMocks()) {
    return await libparsec.workspaceRenameEntry(workspaceHandle, path, newName, true);
  } else {
    return { ok: true, value: null };
  }
}

let MOCK_FILE_ID = 1;

export async function entryStat(path: FsPath): Promise<Result<EntryStat, WorkspaceStatEntryError>> {
  function generateEntryName(prefix: string = '', addExtension = false): string {
    const EXTENSIONS = ['.mp4', '.docx', '.pdf', '.png', '.mp3', '.xls', '.zip'];
    const ext = addExtension ? EXTENSIONS[Math.floor(Math.random() * EXTENSIONS.length)] : '';
    return `${prefix}${uniqueNamesGenerator({ dictionaries: [adjectives, animals] })}${ext}`;
  }

  function generateChildren(): Array<string> {
    const fileCount = Math.floor(Math.random() * 15) + 1;
    const folderCount = Math.floor(Math.random() * 15) + 1;
    const result = [];
    for (let i = 0; i < fileCount + folderCount; i++) {
      result.push(generateEntryName(i < fileCount ? FILE_PREFIX : FOLDER_PREFIX, i < fileCount));
    }
    return result;
  }

  function generateDate(start?: DateTime): DateTime {
    if (!start) {
      start = DateTime.now();
    }
    return DateTime.now().minus({ minutes: Math.floor(Math.random() * 60), seconds: Math.floor(Math.random() * 60) });
  }

  const FOLDER_PREFIX = 'Dir_';
  const FILE_PREFIX = 'File_';

  const clientHandle = getParsecHandle();
  const workspaceHandle = getWorkspaceHandle();

  const fileName = (await Path.filename(path)) || '';

  if (clientHandle && workspaceHandle && !needsMocks()) {
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
  } else {
    MOCK_FILE_ID += 1;
    const createdDate = generateDate();
    if (path !== '/' && fileName.startsWith(FILE_PREFIX)) {
      return {
        ok: true,
        value: {
          tag: FileType.File,
          confinementPoint: null,
          id: `${MOCK_FILE_ID}`,
          created: createdDate,
          updated: generateDate(createdDate),
          baseVersion: 1,
          isPlaceholder: false,
          needSync: Math.floor(Math.random() * 2) === 1,
          size: Math.floor(Math.random() * 1_000_000),
          name: fileName,
          isFile: (): boolean => true,
        },
      };
    } else {
      return {
        ok: true,
        value: {
          tag: FileType.Folder,
          confinementPoint: null,
          id: `${MOCK_FILE_ID}`,
          created: createdDate,
          updated: generateDate(createdDate),
          baseVersion: 1,
          isPlaceholder: false,
          needSync: Math.floor(Math.random() * 2) === 1,
          name: fileName,
          isFile: (): boolean => false,
          children: generateChildren(),
        },
      };
    }
  }
}

export async function getAbsolutePath(_workspaceHandle: WorkspaceHandle, entry: EntryStat): Promise<Result<FsPath, GetAbsolutePathError>> {
  // Helps for tests, will be replaced with bindings
  if (entry.name === 'f') {
    return { ok: true, value: './tsconfig.json' };
  } else if (entry.name === 'd') {
    return { ok: true, value: './src' };
  } else if (entry.name === 'e') {
    return { ok: false, error: { tag: GetAbsolutePathErrorTag.NotFound } };
  } else {
    return { ok: true, value: '/unknown' };
  }
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

export interface ParseLinkError {
  error: string;
}

export interface FileLinkData {
  organizationId: OrganizationID;
  path: FsPath;
  workspaceId: WorkspaceID;
}

export async function parseFileLink(_link: string): Promise<Result<FileLinkData, ParseLinkError>> {
  const clientHandle = getParsecHandle();

  if (clientHandle && !needsMocks()) {
    return {
      ok: true,
      value: {
        organizationId: 'MyOrg',
        path: '/Dir/file.txt',
        workspaceId: 'abcd',
      },
    };
  } else {
    return {
      ok: true,
      value: {
        organizationId: 'MyOrg',
        path: '/Dir/file.txt',
        workspaceId: 'abcd',
      },
    };
  }
}

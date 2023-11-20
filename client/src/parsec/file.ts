// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { libparsec } from '@/plugins/libparsec';
import {
  FsPath,
  FileID,
  WorkspaceFsOperationError,
  Result,
  EntryStatFolder,
  EntryStatFile,
  EntryName,
  FileType,
  EntryStat,
  WorkspaceHandle,
  GetAbsolutePathError,
  GetAbsolutePathErrorTag,
} from '@/parsec/types';
import { getParsecHandle, getWorkspaceHandle } from '@/parsec/routing';
import { DateTime } from 'luxon';
import { Path } from '@/parsec/path';
import { needsMocks } from '@/parsec/environment';

export async function createFile(path: FsPath): Promise<Result<FileID, WorkspaceFsOperationError>> {
  const clientHandle = getParsecHandle();
  const workspaceHandle = getWorkspaceHandle();

  if (clientHandle && workspaceHandle && !needsMocks()) {
    return await libparsec.workspaceCreateFile(workspaceHandle, path);
  } else {
    return { ok: true, value: '42' };
  }
}

export async function createFolder(path: FsPath): Promise<Result<FileID, WorkspaceFsOperationError>> {
  const clientHandle = getParsecHandle();
  const workspaceHandle = getWorkspaceHandle();

  if (clientHandle && workspaceHandle && !needsMocks()) {
    return await libparsec.workspaceCreateFolder(workspaceHandle, path);
  } else {
    return { ok: true, value: '7' };
  }
}

export async function deleteFile(path: FsPath): Promise<Result<null, WorkspaceFsOperationError>> {
  const clientHandle = getParsecHandle();
  const workspaceHandle = getWorkspaceHandle();

  if (clientHandle && workspaceHandle && !needsMocks()) {
    return await libparsec.workspaceRemoveFile(workspaceHandle, path);
  } else {
    return { ok: true, value: null };
  }
}

export async function deleteFolder(path: FsPath): Promise<Result<null, WorkspaceFsOperationError>> {
  const clientHandle = getParsecHandle();
  const workspaceHandle = getWorkspaceHandle();

  if (clientHandle && workspaceHandle && !needsMocks()) {
    return await libparsec.workspaceRemoveFolder(workspaceHandle, path);
  } else {
    return { ok: true, value: null };
  }
}

export async function rename(path: FsPath, newName: EntryName): Promise<Result<null, WorkspaceFsOperationError>> {
  const clientHandle = getParsecHandle();
  const workspaceHandle = getWorkspaceHandle();

  if (clientHandle && workspaceHandle && !needsMocks()) {
    return await libparsec.workspaceRenameEntry(workspaceHandle, path, newName, true);
  } else {
    return { ok: true, value: null };
  }
}

export async function entryStat(path: FsPath): Promise<Result<EntryStat, WorkspaceFsOperationError>> {
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
    return result as Result<EntryStat, WorkspaceFsOperationError>;
  } else {
    if (path !== '/' && fileName !== 'Dir1') {
      return {
        ok: true,
        value: {
          tag: FileType.File,
          confinementPoint: null,
          id: '67',
          created: DateTime.now(),
          updated: DateTime.now(),
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
          id: '68',
          created: DateTime.now(),
          updated: DateTime.now(),
          baseVersion: 1,
          isPlaceholder: false,
          needSync: Math.floor(Math.random() * 2) === 1,
          name: fileName,
          isFile: (): boolean => false,
          children: ['File1.txt', 'File2.jpeg', 'Dir1'],
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

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Path } from '@/parsec/path';
import { EntryName, FileType, FsPath } from '@/parsec/types';
import { DateTime } from 'luxon';
import { adjectives, animals, uniqueNamesGenerator } from 'unique-names-generator';

export interface MockEntry {
  tag: FileType;
  id: string;
  parent: string;
  created: DateTime;
  updated: DateTime;
  version: number;
  size?: number;
  name: EntryName;
  path: FsPath;
  isFile: () => boolean;
}

const FOLDER_PREFIX = 'Dir_';
const FILE_PREFIX = 'File_';

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

export async function generateFile(basePath: FsPath, parentId = 'fakeId', prefix = FILE_PREFIX): Promise<MockEntry> {
  const name = generateEntryName(prefix, true);
  const createdDate = generateDate();
  const entry: MockEntry = {
    tag: FileType.File,
    id: crypto.randomUUID().toString(),
    parent: parentId,
    created: createdDate,
    updated: generateDate(createdDate),
    version: 1,
    size: Math.floor(Math.random() * 1_000_000),
    isFile: (): boolean => true,
    name: name,
    path: await Path.join(basePath, name),
  };
  return entry;
}

export async function generateFolder(basePath: FsPath, parentId = 'fakeId', prefix = FOLDER_PREFIX): Promise<MockEntry> {
  const name = generateEntryName(prefix, false);
  const createdDate = generateDate();
  const entry: MockEntry = {
    tag: FileType.Folder,
    id: crypto.randomUUID().toString(),
    parent: parentId,
    created: createdDate,
    updated: generateDate(createdDate),
    version: 1,
    isFile: (): boolean => false,
    name: name,
    path: await Path.join(basePath, name),
  };
  return entry;
}

export async function generateEntries(
  basePath: FsPath,
  folderCount = 2,
  fileCount = 2,
  filePrefix = FILE_PREFIX,
  folderPrefix = FOLDER_PREFIX,
): Promise<Array<MockEntry>> {
  const items: Array<MockEntry> = [];
  const parentId = crypto.randomUUID().toString();

  // Add files
  for (let i = 0; i < fileCount; i++) {
    items.push(await generateFile(basePath, parentId, filePrefix));
  }

  // Add folders
  for (let i = 0; i < folderCount; i++) {
    items.push(await generateFolder(basePath, parentId, folderPrefix));
  }
  return items;
}

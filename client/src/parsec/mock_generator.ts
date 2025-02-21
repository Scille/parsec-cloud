// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MockFileType, getMockFileContent } from '@/parsec/mock_files';
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
const EXTENSIONS = ['.mp4', '.docx', '.pdf', '.png', '.jpg', '.mp3', '.xlsx', '.txt', '.py'];

function generateEntryName(prefix: string = '', addExtension = false, extension = ''): EntryName {
  const ext = addExtension && extension.length === 0 ? EXTENSIONS[Math.floor(Math.random() * EXTENSIONS.length)] : extension;
  return `${prefix}${uniqueNamesGenerator({ dictionaries: [adjectives, animals] })}${ext}`;
}

function generateDate(start?: DateTime): DateTime {
  if (!start) {
    start = DateTime.now();
  }
  return DateTime.now().minus({ minutes: Math.floor(Math.random() * 60), seconds: Math.floor(Math.random() * 60) });
}

interface GenerateEntryOptions {
  parentId?: string;
  prefix?: string;
  fileName?: EntryName;
  extension?: string;
}

export async function generateFile(
  basePath: FsPath,
  opts: GenerateEntryOptions = { parentId: 'fakeId', prefix: FILE_PREFIX },
): Promise<MockEntry> {
  const name = opts.fileName ?? generateEntryName(opts.prefix ?? FILE_PREFIX, true, opts.extension ?? '');
  const ext = Path.getFileExtension(name);
  const createdDate = generateDate();
  const entry: MockEntry = {
    tag: FileType.File,
    id: crypto.randomUUID().toString(),
    parent: opts.parentId ?? 'fakeId',
    created: createdDate,
    updated: generateDate(createdDate),
    version: 1,
    isFile: (): boolean => true,
    name: name,
    path: await Path.join(basePath, name),
  };

  switch (ext) {
    case 'xlsx':
      entry.size = (await getMockFileContent(MockFileType.XLSX)).byteLength;
      break;
    case 'png':
      entry.size = (await getMockFileContent(MockFileType.PNG)).byteLength;
      break;
    case 'jpg':
      entry.size = (await getMockFileContent(MockFileType.JPG)).byteLength;
      break;
    case 'docx':
      entry.size = (await getMockFileContent(MockFileType.DOCX)).byteLength;
      break;
    case 'txt':
      entry.size = (await getMockFileContent(MockFileType.TXT)).byteLength;
      break;
    case 'py':
      entry.size = (await getMockFileContent(MockFileType.PY)).byteLength;
      break;
    case 'pdf':
      entry.size = (await getMockFileContent(MockFileType.PDF)).byteLength;
      break;
    case 'mp3':
      entry.size = (await getMockFileContent(MockFileType.MP3)).byteLength;
      break;
    case 'mp4':
      entry.size = (await getMockFileContent(MockFileType.MP4)).byteLength;
      break;
    default:
      console.log('Using random file size');
      entry.size = Math.floor(Math.random() * 1_000_000);
      break;
  }

  return entry;
}

export async function generateFolder(
  basePath: FsPath,
  opts: GenerateEntryOptions = { parentId: 'fakeId', prefix: FOLDER_PREFIX },
): Promise<MockEntry> {
  const name = opts.fileName ?? generateEntryName(opts.prefix ?? FOLDER_PREFIX, false);
  const createdDate = generateDate();
  const entry: MockEntry = {
    tag: FileType.Folder,
    id: crypto.randomUUID().toString(),
    parent: opts.parentId ?? 'fakeId',
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
    items.push(await generateFile(basePath, { parentId: parentId, prefix: filePrefix }));
  }

  // Add folders
  for (let i = 0; i < folderCount; i++) {
    items.push(await generateFolder(basePath, { parentId: parentId, prefix: folderPrefix }));
  }
  return items;
}

export async function generateEntriesForEachFileType(
  basePath: FsPath,
  folderCount = 2,
  filePrefix = FILE_PREFIX,
  folderPrefix = FOLDER_PREFIX,
  extensions = ['.pdf', '.mp4', '.mp3', '.png', '.jpg', '.docx', '.xlsx', '.py', '.txt'],
): Promise<Array<MockEntry>> {
  const items: Array<MockEntry> = [];
  const parentId = crypto.randomUUID().toString();

  // Add files
  for (const ext of extensions) {
    items.push(await generateFile(basePath, { parentId: parentId, prefix: filePrefix, extension: ext }));
  }
  // Add folders
  for (let i = 0; i < folderCount; i++) {
    items.push(await generateFolder(basePath, { parentId: parentId, prefix: folderPrefix }));
  }
  return items;
}

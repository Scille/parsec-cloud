// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryName, FsPath } from '@/parsec/types';
import { libparsec } from '@/plugins/libparsec';

async function parse(path: FsPath): Promise<EntryName[]> {
  return await libparsec.pathSplit(path);
}

async function join(path: FsPath, entryName: EntryName): Promise<FsPath> {
  return await libparsec.pathJoin(path, entryName);
}

async function normalize(path: FsPath): Promise<FsPath> {
  return await libparsec.pathNormalize(path);
}

async function parent(path: FsPath): Promise<FsPath> {
  return await libparsec.pathParent(path);
}

async function filename(path: FsPath): Promise<EntryName | null> {
  return await libparsec.pathFilename(path);
}

function getFileExtension(filename: EntryName): string {
  while (filename.startsWith('.')) {
    filename = filename.slice(1);
  }
  const pathComponents = filename.split('.');
  if (pathComponents.length === 1 || pathComponents.length === 0) {
    return '';
  }
  return pathComponents[pathComponents.length - 1].toLowerCase();
}

function areSame(pathA: FsPath, pathB: FsPath): boolean {
  return pathA === pathB;
}

function filenameWithoutExtension(filename: EntryName): EntryName {
  const ext = getFileExtension(filename);

  if (ext.length) {
    return filename.substring(0, filename.length - ext.length - 1);
  }
  return filename;
}

async function joinMultiple(path: FsPath, entries: EntryName[]): Promise<FsPath> {
  let result: FsPath = path;
  for (const entry of entries) {
    result = await libparsec.pathJoin(result, entry);
  }
  return result;
}

async function joinPaths(path1: FsPath, path2: FsPath): Promise<FsPath> {
  const parts = await Path.parse(path2);
  return await Path.joinMultiple(path1, parts);
}

export const Path = {
  parse,
  join,
  joinMultiple,
  normalize,
  parent,
  filename,
  getFileExtension,
  areSame,
  filenameWithoutExtension,
  joinPaths,
};

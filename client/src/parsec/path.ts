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

function areSame(pathA: FsPath, pathB: FsPath): boolean {
  return pathA === pathB;
}

function getFileExtension(filename: EntryName): string {
  return splitName(filename).extension;
}

function filenameWithoutExtension(filename: EntryName): EntryName {
  return splitName(filename).nameWithoutExtension;
}

function splitName(filename: EntryName): { nameWithoutExtension: EntryName; extension: string } {
  const index = filename.lastIndexOf('.');

  if (index === -1 || index === 0) {
    return { nameWithoutExtension: filename, extension: '' };
  }

  return { nameWithoutExtension: filename.slice(0, index), extension: filename.slice(index + 1).toLocaleLowerCase() };
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

function quickJoin(path1: FsPath, path2: FsPath): FsPath {
  if (path2.startsWith('/')) {
    path2 = path2.slice(1);
  }
  return path1.endsWith('/') ? `${path1}${path2}` : `${path1}/${path2}`;
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
  quickJoin,
  splitName,
};

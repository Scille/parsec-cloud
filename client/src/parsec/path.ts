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

async function filename(path: FsPath): Promise<FsPath | null> {
  return await libparsec.pathFilename(path);
}

function getFileExtension(path: FsPath): string {
  while (path.startsWith('.')) {
    path = path.slice(1);
  }
  const pathComponents = path.split('.');
  if (pathComponents.length === 1 || pathComponents.length === 0) {
    return '';
  }
  return pathComponents[pathComponents.length - 1].toLowerCase();
}

export const Path = {
  parse,
  join,
  normalize,
  parent,
  filename,
  getFileExtension,
};

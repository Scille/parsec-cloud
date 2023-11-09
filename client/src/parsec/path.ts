// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { FsPath, EntryName } from '@/parsec/types';
import { parse as pathParse, join as pathJoin } from '@/common/path';

async function parse(path: FsPath): Promise<EntryName[]> {
  return pathParse(path);
}

async function join(path: FsPath, entryName: EntryName): Promise<FsPath> {
  return pathJoin(path, entryName);
}

export const Path = {
  parse,
  join,
};

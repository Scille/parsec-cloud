// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryName, FsPath } from '@/parsec';

export interface FileContentInfo {
  data: Uint8Array;
  extension: string;
  mimeType: string;
  fileName: EntryName;
  path: FsPath;
}

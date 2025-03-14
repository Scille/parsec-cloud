// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { FileContentType } from '@/common/fileTypes';
import { EntryName, FsPath } from '@/parsec';

export interface FileContentInfo {
  data: Uint8Array;
  extension: string;
  contentType: FileContentType;
  fileName: EntryName;
  path: FsPath;
}

export const PlaybackSpeeds = [0.25, 0.5, 1, 1.5, 2];

export enum PlaybackSpeed {
  Speed_0_25 = 0,
  Speed_0_5 = 1,
  Speed_1 = 2,
  Speed_1_5 = 3,
  Speed_2 = 4,
}

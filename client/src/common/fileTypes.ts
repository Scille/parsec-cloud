// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { fileTypeFromBuffer } from 'file-type';

enum FileType {
  Image = 'image',
  Video = 'video',
  Audio = 'audio',
  Unknown = 'unknown',
}

interface DetectedFileType {
  type: FileType;
  extension: string;
  mimeType: string;
}

const IMAGES = ['image/png', 'image/webp', 'image/jpeg', 'image/svg+xml', 'image/bmp', 'image/gif'];

async function detectFileType(buffer: Uint8Array): Promise<DetectedFileType> {
  const result = await fileTypeFromBuffer(buffer);

  if (!result) {
    return { type: FileType.Unknown, extension: '', mimeType: 'application/octet-stream' };
  }

  if (IMAGES.includes(result.mime)) {
    return { type: FileType.Image, extension: result.ext, mimeType: result.mime };
  }

  return { type: FileType.Unknown, extension: result.ext, mimeType: result.mime };
}

export { DetectedFileType, FileType, detectFileType };

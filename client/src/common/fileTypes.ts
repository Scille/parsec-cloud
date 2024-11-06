// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { fileTypeFromBuffer } from 'file-type';

enum FileContentType {
  Image = 'image',
  Video = 'video',
  Audio = 'audio',
  Spreadsheet = 'spreadsheet',
  Document = 'document',
  Unknown = 'unknown',
}

interface DetectedFileType {
  type: FileContentType;
  extension: string;
  mimeType: string;
}

const IMAGES = ['image/png', 'image/webp', 'image/jpeg', 'image/svg+xml', 'image/bmp', 'image/gif'];
const SPREADSHEETS = ['application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'];

async function detectFileContentType(buffer: Uint8Array): Promise<DetectedFileType> {
  const result = await fileTypeFromBuffer(buffer);

  if (!result) {
    return { type: FileContentType.Unknown, extension: '', mimeType: 'application/octet-stream' };
  }

  if (IMAGES.includes(result.mime)) {
    return { type: FileContentType.Image, extension: result.ext, mimeType: result.mime };
  }
  if (SPREADSHEETS.includes(result.mime)) {
    return { type: FileContentType.Spreadsheet, extension: result.ext, mimeType: result.mime };
  }
  console.log(`Unhandled mimetype ${result.mime}`);

  return { type: FileContentType.Unknown, extension: result.ext, mimeType: result.mime };
}

export { DetectedFileType, FileContentType, detectFileContentType };

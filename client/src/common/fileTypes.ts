// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  closeFile,
  closeHistoryFile,
  FileDescriptor,
  FsPath,
  openFile,
  openFileAt,
  Path,
  readFile,
  readHistoryFile,
  WorkspaceHandle,
} from '@/parsec';
import { fileTypeFromBuffer } from 'file-type';
import { DateTime } from 'luxon';

enum FileContentType {
  Image = 'image',
  Video = 'video',
  Audio = 'audio',
  Spreadsheet = 'spreadsheet',
  Document = 'document',
  Text = 'text',
  PdfDocument = 'pdf-document',
  Unknown = 'unknown',
}

interface DetectedFileType {
  type: FileContentType;
  extension: string;
  mimeType: string;
}

const IMAGES = ['image/png', 'image/webp', 'image/jpeg', 'image/svg+xml', 'image/bmp', 'image/gif'];
const SPREADSHEETS = ['application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'];
const DOCUMENTS = ['application/vnd.openxmlformats-officedocument.wordprocessingml.document'];
const PDF_DOCUMENTS = ['application/pdf'];
const AUDIOS = ['audio/x-wav', 'audio/mpeg'];
const VIDEOS = ['video/mp4', 'video/mpeg'];

const TEXTS = new Map<string, string>([
  ['xml', 'application/xml'],
  ['json', 'application/json'],
  ['js', 'text/javascript'],
  ['html', 'text/html'],
  ['htm', 'text/html'],
  ['txt', 'text/plain'],
  ['sh', 'application/x-sh'],
  ['csv', 'text/csv'],
  ['css', 'text/css'],
  ['py', 'text/x-python'],
]);

async function detectFileContentTypeFromBuffer(buffer: Uint8Array, fileExt?: string): Promise<DetectedFileType> {
  const result = await fileTypeFromBuffer(buffer);

  if (!result) {
    return { type: FileContentType.Unknown, extension: fileExt ?? '', mimeType: 'application/octet-stream' };
  }

  if (IMAGES.includes(result.mime)) {
    return { type: FileContentType.Image, extension: fileExt ?? result.ext, mimeType: result.mime };
  }
  if (SPREADSHEETS.includes(result.mime)) {
    return { type: FileContentType.Spreadsheet, extension: fileExt ?? result.ext, mimeType: result.mime };
  }
  console.log(result.mime, result.ext);
  if (DOCUMENTS.includes(result.mime) || (result.mime === 'application/zip' && fileExt === 'docx')) {
    return { type: FileContentType.Document, extension: fileExt ?? result.ext, mimeType: result.mime };
  }
  if (PDF_DOCUMENTS.includes(result.mime)) {
    return { type: FileContentType.PdfDocument, extension: fileExt ?? result.ext, mimeType: result.mime };
  }
  if (AUDIOS.includes(result.mime)) {
    return { type: FileContentType.Audio, extension: fileExt ?? result.ext, mimeType: result.mime };
  }
  if (VIDEOS.includes(result.mime)) {
    return { type: FileContentType.Video, extension: fileExt ?? result.ext, mimeType: result.mime };
  }
  console.log(`Unhandled mimetype ${result.mime}`);

  return { type: FileContentType.Unknown, extension: fileExt ?? result.ext, mimeType: result.mime };
}

async function detectFileContentType(workspaceHandle: WorkspaceHandle, path: FsPath, at?: DateTime): Promise<DetectedFileType | undefined> {
  const fileName = await Path.filename(path);

  if (!fileName) {
    return;
  }
  const ext = Path.getFileExtension(fileName).toLocaleLowerCase();

  if (TEXTS.has(ext)) {
    return { type: FileContentType.Text, extension: ext, mimeType: TEXTS.get(ext) as string };
  }

  const READ_CHUNK_SIZE = 512n;
  let fd: FileDescriptor | null = null;
  try {
    let openResult;
    if (at) {
      openResult = await openFileAt(workspaceHandle, path, at);
    } else {
      openResult = await openFile(workspaceHandle, path, { read: true });
    }
    if (!openResult.ok) {
      return;
    }
    fd = openResult.value;
    let readResult;
    if (at) {
      readResult = await readHistoryFile(workspaceHandle, fd, 0n, READ_CHUNK_SIZE);
    } else {
      readResult = await readFile(workspaceHandle, fd, 0n, READ_CHUNK_SIZE);
    }
    if (!readResult.ok) {
      return;
    }
    const buffer = new Uint8Array(readResult.value);
    return await detectFileContentTypeFromBuffer(buffer, ext);
  } finally {
    if (fd) {
      if (at) {
        closeHistoryFile(workspaceHandle, fd);
      } else {
        closeFile(workspaceHandle, fd);
      }
    }
  }
}

export { DetectedFileType, detectFileContentType, detectFileContentTypeFromBuffer, FileContentType };

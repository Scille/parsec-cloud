// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { closeFile, FileDescriptor, FsPath, getWorkspaceInfo, openFile, Path, readFile, WorkspaceHandle, WorkspaceHistory } from '@/parsec';
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
const SPREADSHEETS = [
  'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
  'application/vnd.oasis.opendocument.spreadsheet',
];
const DOCUMENTS = ['application/vnd.openxmlformats-officedocument.wordprocessingml.document'];
const PDF_DOCUMENTS = ['application/pdf'];
const AUDIOS = ['audio/x-wav', 'audio/mpeg'];
const VIDEOS = ['video/mp4', 'video/mpeg', 'video/webm'];

// For custom mimetypes
const SPECIAL_TEXTS = new Map<string, string>([
  ['xml', 'application/xml'],
  ['json', 'application/json'],
  ['js', 'text/javascript'],
  ['html', 'text/html'],
  ['htm', 'text/html'],
  ['xhtml', 'text/html'],
  ['sh', 'application/x-sh'],
  ['csv', 'text/csv'],
  ['css', 'text/css'],
  ['py', 'text/x-python'],
  ['php', 'application/x-httpd-php'],
  ['sh', 'application/x-sh'],
  ['tex', 'application/x-latex'],
]);

// For generic text/plain
const TEXTS = [
  'txt',
  'h',
  'hpp',
  'c',
  'cpp',
  'rs',
  'java',
  'ts',
  'ini',
  'cs',
  'vb',
  'swift',
  'lua',
  'rb',
  'vbs',
  'md',
  'log',
  'rst',
  'toml',
  'po',
  'vue',
  'kt',
  'yml',
  'yaml',
];

async function detectFileContentTypeFromBuffer(buffer: Uint8Array, fileExt?: string): Promise<DetectedFileType> {
  const result = await fileTypeFromBuffer(buffer);

  if (!result) {
    return { type: FileContentType.Unknown, extension: fileExt ?? '', mimeType: 'application/octet-stream' };
  }

  if (IMAGES.includes(result.mime)) {
    return { type: FileContentType.Image, extension: fileExt ?? result.ext, mimeType: result.mime };
  }
  if (
    SPREADSHEETS.includes(result.mime) ||
    (result.mime === 'application/zip' && fileExt === 'xlsx') ||
    (result.mime === 'application/x-cfb' && fileExt === 'xls')
  ) {
    return { type: FileContentType.Spreadsheet, extension: fileExt ?? result.ext, mimeType: result.mime };
  }
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

const READ_CHUNK_SIZE = 512;

async function _getFileAtBuffer(workspaceHandle: WorkspaceHandle, path: FsPath, at: DateTime): Promise<Uint8Array | undefined> {
  let fd: FileDescriptor | null = null;
  const workspaceInfoResult = await getWorkspaceInfo(workspaceHandle);
  if (!workspaceInfoResult.ok) {
    return undefined;
  }
  const history = new WorkspaceHistory(workspaceInfoResult.value.id);
  try {
    await history.start();
    await history.setCurrentTime(at);
    const openResult = await history.openFile(path);
    if (!openResult.ok) {
      return;
    }
    fd = openResult.value;
    const readResult = await history.readFile(fd, 0, READ_CHUNK_SIZE);
    if (!readResult.ok) {
      return;
    }
    return new Uint8Array(readResult.value);
  } finally {
    if (fd) {
      await history.closeFile(fd);
    }
    await history.stop();
  }
}

async function _getFileBuffer(workspaceHandle: WorkspaceHandle, path: FsPath): Promise<Uint8Array | undefined> {
  let fd: FileDescriptor | null = null;
  try {
    const openResult = await openFile(workspaceHandle, path, { read: true });
    if (!openResult.ok) {
      return;
    }
    fd = openResult.value;
    const readResult = await readFile(workspaceHandle, fd, 0, READ_CHUNK_SIZE);
    if (!readResult.ok) {
      return;
    }
    return new Uint8Array(readResult.value);
  } finally {
    if (fd) {
      await closeFile(workspaceHandle, fd);
    }
  }
}

async function detectFileContentType(workspaceHandle: WorkspaceHandle, path: FsPath, at?: DateTime): Promise<DetectedFileType | undefined> {
  const fileName = await Path.filename(path);

  if (!fileName) {
    return;
  }
  const ext = Path.getFileExtension(fileName).toLocaleLowerCase();

  if (SPECIAL_TEXTS.has(ext)) {
    return { type: FileContentType.Text, extension: ext, mimeType: SPECIAL_TEXTS.get(ext) as string };
  }
  if (TEXTS.includes(ext)) {
    return { type: FileContentType.Text, extension: ext, mimeType: 'text/plain' };
  }

  const buffer = at ? await _getFileAtBuffer(workspaceHandle, path, at) : await _getFileBuffer(workspaceHandle, path);
  if (buffer) {
    return await detectFileContentTypeFromBuffer(buffer, ext);
  }
}

export { DetectedFileType, detectFileContentType, detectFileContentTypeFromBuffer, FileContentType };

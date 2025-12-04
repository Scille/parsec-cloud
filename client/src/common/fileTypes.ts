// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryName, Path } from '@/parsec';
import { fileTypeFromBuffer } from 'file-type';

enum FileContentType {
  Image = 'image',
  Video = 'video',
  Audio = 'audio',
  Spreadsheet = 'spreadsheet',
  Document = 'document',
  Text = 'text',
  PdfDocument = 'pdf-document',
  Presentation = 'presentation',
  Unknown = 'unknown',
}

interface DetectedFileType {
  type: FileContentType;
  extension: string;
}

const IMAGES = ['png', 'webp', 'jpg', 'jpeg', 'svg', 'bmp', 'gif'];
const SPREADSHEETS = ['xlsx', 'xls'];
const DOCUMENTS = ['docx'];
const PDF_DOCUMENTS = ['pdf'];
const AUDIOS = ['wav', 'mp3', 'ogg'];
const VIDEOS = ['mp4', 'mpeg', 'webm'];
const PRESENTATIONS = ['pptx'];

// For generic text/plain
const TEXTS = [
  'xml',
  'json',
  'js',
  'html',
  'htm',
  'xhtml',
  'sh',
  'csv',
  'css',
  'py',
  'php',
  'sh',
  'tex',
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

async function getMimeTypeFromBuffer(data: Uint8Array): Promise<string | undefined> {
  try {
    const result = await fileTypeFromBuffer(data);
    if (result) {
      return result.mime;
    }
    return undefined;
  } catch (err: any) {
    console.log(`Cannot detect mimetype: ${String(err)}`);
    return undefined;
  }
}

function detectFileContentType(name: EntryName): DetectedFileType {
  const ext = Path.getFileExtension(name);

  if (IMAGES.includes(ext)) {
    return { type: FileContentType.Image, extension: ext };
  }
  if (DOCUMENTS.includes(ext)) {
    return { type: FileContentType.Document, extension: ext };
  }
  if (PDF_DOCUMENTS.includes(ext)) {
    return { type: FileContentType.PdfDocument, extension: ext };
  }
  if (SPREADSHEETS.includes(ext)) {
    return { type: FileContentType.Spreadsheet, extension: ext };
  }
  if (AUDIOS.includes(ext)) {
    return { type: FileContentType.Audio, extension: ext };
  }
  if (VIDEOS.includes(ext)) {
    return { type: FileContentType.Video, extension: ext };
  }
  if (TEXTS.includes(ext)) {
    return { type: FileContentType.Text, extension: ext };
  }
  if (PRESENTATIONS.includes(ext)) {
    return { type: FileContentType.Presentation, extension: ext };
  }
  return { type: FileContentType.Unknown, extension: ext };
}

export { DetectedFileType, detectFileContentType, FileContentType, getMimeTypeFromBuffer };

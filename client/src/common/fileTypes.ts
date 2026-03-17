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

const OPENABLE_FILES = {
  IMAGES: ['png', 'webp', 'jpg', 'jpeg', 'svg', 'bmp', 'gif'],
  SPREADSHEETS: ['xlsx', 'xls', 'ods'],
  DOCUMENTS: ['docx', 'odt'],
  PDF_DOCUMENTS: ['pdf'],
  AUDIOS: ['wav', 'mp3', 'ogg'],
  VIDEOS: ['mp4', 'mpeg', 'webm'],
  // TODO: enable ODP when supported: https://github.com/Scille/parsec-cloud/issues/12110
  PRESENTATIONS: ['pptx'], // , 'odp']
  // For generic text/plain
  TEXTS: [
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
  ],
};

const DETECTABLE_FILES = {
  IMAGES: [...OPENABLE_FILES.IMAGES, 'avif', 'heic', 'heif', 'raw', 'dng', 'cr2', 'nef', 'arw', 'ps', 'xcf', 'ai', 'eps', 'tiff', 'tif'],
  SPREADSHEETS: [...OPENABLE_FILES.SPREADSHEETS],
  DOCUMENTS: [...OPENABLE_FILES.DOCUMENTS, 'doc'],
  PDF_DOCUMENTS: [...OPENABLE_FILES.PDF_DOCUMENTS],
  AUDIOS: [...OPENABLE_FILES.AUDIOS, 'flac', 'aac', 'm4a', 'opus', 'oga', 'alac', 'aiff', 'wma', 'amr', 'mid', 'midi'],
  VIDEOS: ['mp4', 'mpeg', 'webm', ...OPENABLE_FILES.VIDEOS, 'mkv', 'mov', 'avi', 'ts', 'm2ts', 'mts', 'flv', 'wmv', '3gp', 'mpg', 'mpeg'],
  PRESENTATIONS: [...OPENABLE_FILES.PRESENTATIONS, 'odp', 'ppt'],
  TEXTS: [...OPENABLE_FILES.TEXTS],
};

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

function detectFile(name: EntryName, matcher: any): DetectedFileType {
  const ext = Path.getFileExtension(name);

  if (matcher.IMAGES.includes(ext)) {
    return { type: FileContentType.Image, extension: ext };
  }
  if (matcher.DOCUMENTS.includes(ext)) {
    return { type: FileContentType.Document, extension: ext };
  }
  if (matcher.PDF_DOCUMENTS.includes(ext)) {
    return { type: FileContentType.PdfDocument, extension: ext };
  }
  if (matcher.SPREADSHEETS.includes(ext)) {
    return { type: FileContentType.Spreadsheet, extension: ext };
  }
  if (matcher.AUDIOS.includes(ext)) {
    return { type: FileContentType.Audio, extension: ext };
  }
  if (matcher.VIDEOS.includes(ext)) {
    return { type: FileContentType.Video, extension: ext };
  }
  if (matcher.TEXTS.includes(ext)) {
    return { type: FileContentType.Text, extension: ext };
  }
  if (matcher.PRESENTATIONS.includes(ext)) {
    return { type: FileContentType.Presentation, extension: ext };
  }
  return { type: FileContentType.Unknown, extension: ext };
}

function detectOpenableFile(name: EntryName): DetectedFileType {
  return detectFile(name, OPENABLE_FILES);
}

function detectFileContentType(name: EntryName): DetectedFileType {
  return detectFile(name, DETECTABLE_FILES);
}

export { DetectedFileType, detectFileContentType, detectOpenableFile, FileContentType, getMimeTypeFromBuffer };

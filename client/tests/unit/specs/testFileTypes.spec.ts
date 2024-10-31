// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { detectFileContentTypeFromBuffer, FileContentType } from '@/common/fileTypes';
import fs from 'fs';
import path from 'path';
import { describe, expect, it } from 'vitest';

describe('File types detection', async () => {
  it.each([
    ['png', FileContentType.Image, 'image/png'],
    ['bmp', FileContentType.Image, 'image/bmp'],
    ['gif', FileContentType.Image, 'image/gif'],
    ['jpg', FileContentType.Image, 'image/jpeg'],
    ['webp', FileContentType.Image, 'image/webp'],
    ['docx', FileContentType.Document, 'application/vnd.openxmlformats-officedocument.wordprocessingml.document'],
    ['odt', FileContentType.Unknown, 'application/vnd.oasis.opendocument.text'],
    ['xlsx', FileContentType.Spreadsheet, 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'],
    ['pdf', FileContentType.PdfDocument, 'application/pdf'],
    ['mp3', FileContentType.Audio, 'audio/mpeg'],
    ['mp4', FileContentType.Video, 'video/mp4'],
  ])('Detect file content type %s', async (extension, expectedFileType, expectedMimeType) => {
    const data = fs.readFileSync(path.join('tests', 'unit', 'data', `example.${extension}`));
    const detected = await detectFileContentTypeFromBuffer(data);

    expect(detected.mimeType).to.equal(expectedMimeType);
    expect(detected.type).to.equal(expectedFileType);
    expect(detected.extension).to.equal(extension);
  });
});

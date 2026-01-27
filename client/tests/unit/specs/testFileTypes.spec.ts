// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { detectFileContentType, FileContentType } from '@/common/fileTypes';
import { describe, expect, it } from 'vitest';

describe('File types detection', async () => {
  it.each([
    ['png', FileContentType.Image],
    ['bmp', FileContentType.Image],
    ['gif', FileContentType.Image],
    ['jpg', FileContentType.Image],
    ['webp', FileContentType.Image],
    ['docx', FileContentType.Document],
    ['odt', FileContentType.Document],
    ['xlsx', FileContentType.Spreadsheet],
    ['ods', FileContentType.Spreadsheet],
    ['pdf', FileContentType.PdfDocument],
    ['mp3', FileContentType.Audio],
    ['mp4', FileContentType.Video],
    ['odp', FileContentType.Unknown],
  ])('Detect file content type %s', async (extension, expectedFileType) => {
    const entryName = `example.${extension}`;
    const detected = detectFileContentType(entryName);

    expect(detected).to.not.equal(undefined);
    if (detected !== undefined) {
      expect(detected.type).to.equal(expectedFileType);
      expect(detected.extension).to.equal(extension);
    }
  });
});

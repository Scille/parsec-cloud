// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { detectFileType, FileType } from '@/common/fileTypes';
import fs from 'fs';
import path from 'path';
import { it } from 'vitest';

describe('File types detection', async () => {
  it.each([
    ['png', FileType.Image, 'image/png'],
    ['bmp', FileType.Image, 'image/bmp'],
    ['gif', FileType.Image, 'image/gif'],
    ['jpg', FileType.Image, 'image/jpeg'],
    ['webp', FileType.Image, 'image/webp'],
    ['pdf', FileType.Unknown, 'application/pdf'],
  ])('Detect image type %s', async (extension, expectedFileType, expectedMimeType) => {
    const data = fs.readFileSync(path.join('tests', 'unit', 'data', `example.${extension}`));
    const detected = await detectFileType(data);

    expect(detected.mimeType).to.equal(expectedMimeType);
    expect(detected.type).to.equal(expectedFileType);
    expect(detected.extension).to.equal(extension);
  });
});

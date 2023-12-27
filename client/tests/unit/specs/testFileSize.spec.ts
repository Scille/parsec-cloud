// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { formatFileSize } from '@/common/file';
import { initTranslations } from '@/services/translation';
import { it } from 'vitest';

describe('File size', () => {
  beforeEach(() => {
    initTranslations('en-US');
  });

  // Values taken for the Python test:
  // https://github.com/Scille/parsec-cloud/blob/ed3599f46e74248da51bc876a58cd1bfd8475885/tests/core/gui/test_file_size_display.py
  it.each([
    ['0 B', 0],
    ['1 B', 1],
    ['25 B', 25],
    ['99 B', 99],
    ['0.99 KB', 1023],
    ['1.00 KB', 1024],
    ['1.00 KB', 1024 + 1],
    ['1.09 KB', 1024 + 100],
    ['1.94 KB', 1024 * 2 - 52],
    ['1.95 KB', 1024 * 2 - 51],
    ['2.00 KB', 1024 * 2],
    ['12.3 KB', 1024 * 12 + 300],
    ['123 KB', 1024 * 123 + 400],
    ['0.99 MB', 1023 * 1024],
    ['2.50 MB', 1024 ** 2 * 2.5],
    ['1024 TB', 1024 ** 5],
    [`${1024 ** 2} TB`, 1024 ** 6 + 5 * 1024 ** 3],
    ['0 B', 0],
    ['1 B', 1],
    ['9 B', 9],
    ['10 B', 10],
    ['99 B', 99],
    ['100 B', 100],
    ['999 B', 999],
    ['0.97 KB', 1_000],
    ['0.97 KB', 1_003],
    ['0.98 KB', 1_004],
    ['0.98 KB', 1_013],
    ['0.99 KB', 1_014],
    ['0.99 KB', 1_023],
    ['1.00 KB', 1_024],
    ['9.99 KB', 10_239],
    ['10.0 KB', 10_240],
    ['99.9 KB', 102_348],
    ['100 KB', 102_349],
    ['999 KB', 1_023_487],
    ['0.97 MB', 1_023_488],
    ['0.97 MB', 1_027_604],
    ['0.98 MB', 1_027_605],
    ['0.98 MB', 1_038_090],
    ['0.99 MB', 1_038_091],
    ['0.99 MB', 1_048_575],
    ['1.00 MB', 1_048_576],
    ['9.99 MB', 10_485_759],
    ['10.0 MB', 10_485_760],
    ['99.9 MB', 104_805_171],
    ['100 MB', 104_805_172],
    ['999 MB', 1_048_051_711],
    ['0.97 GB', 1_048_051_712],
    ['0.97 GB', 1_052_266_987],
    ['0.98 GB', 1_052_266_988],
    ['0.98 GB', 1_063_004_405],
    ['0.99 GB', 1_063_004_406],
    ['0.99 GB', 1_073_741_823],
    ['1.00 GB', 1_073_741_824],
    ['9.99 GB', 10_737_418_239],
    ['10.0 GB', 10_737_418_240],
  ])('Gets the right format for the size', async (expected, bytes) => {
    expect(formatFileSize(bytes)).to.equal(expected);
  });

  it('Handles negative bytes', async () => {
    expect(() => formatFileSize(-1234)).to.throw('Bytes must be >= 0');
  });
});

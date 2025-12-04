// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export enum MockFileType {
  DOCX = 'DOCX',
  PDF = 'PDF',
  PNG = 'PNG',
  JPG = 'JPG',
  PY = 'PY',
  TXT = 'TXT',
  XLSX = 'XLSX',
  MP4 = 'MP4',
  MP3 = 'MP3',
  PPTX = 'PPTX',
}

// Import content dynamically so it's not loaded if not needed
export async function getMockFileContent(type: MockFileType): Promise<Uint8Array> {
  switch (type) {
    case MockFileType.DOCX:
      return (await import('@/parsec/mock_files/docx')).default;
    case MockFileType.PDF:
      return (await import('@/parsec/mock_files/pdf')).default;
    case MockFileType.PNG:
      return (await import('@/parsec/mock_files/png')).default;
    case MockFileType.JPG:
      return (await import('@/parsec/mock_files/jpg')).default;
    case MockFileType.PY:
      return (await import('@/parsec/mock_files/py')).default;
    case MockFileType.TXT:
      return (await import('@/parsec/mock_files/txt')).default;
    case MockFileType.XLSX:
      return (await import('@/parsec/mock_files/xlsx')).default;
    case MockFileType.MP4:
      return (await import('@/parsec/mock_files/mp4')).default;
    case MockFileType.MP3:
      return (await import('@/parsec/mock_files/mp3')).default;
    case MockFileType.PPTX:
      return (await import('@/parsec/mock_files/pptx')).default;
  }
}

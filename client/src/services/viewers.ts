// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import * as pdfjs from 'pdfjs-dist';
import pdfjsWorker from 'pdfjs-dist/build/pdf.worker?worker&url';

async function _initPdf(): Promise<void> {
  pdfjs.GlobalWorkerOptions.workerSrc = pdfjsWorker;
}

export async function initViewers(): Promise<void> {
  await _initPdf();
}

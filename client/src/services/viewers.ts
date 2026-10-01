// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import MonacoEditorWorker from 'monaco-editor/editor/editor.worker?worker';
import MonacoCssWorker from 'monaco-editor/language/css/css.worker?worker';
import MonacoHtmlWorker from 'monaco-editor/language/html/html.worker?worker';
import MonacoJsonWorker from 'monaco-editor/language/json/json.worker?worker';
import MonacoTsWorker from 'monaco-editor/language/typescript/ts.worker?worker';
import * as pdfjs from 'pdfjs-dist';
import pdfjsWorker from 'pdfjs-dist/build/pdf.worker?worker&url';

async function _initPdf(): Promise<void> {
  window.nativeAPI.log('debug', 'Init PDF worker');
  pdfjs.GlobalWorkerOptions.workerSrc = pdfjsWorker;
}

async function _initMonaco(): Promise<void> {
  window.nativeAPI.log('debug', 'Init Monaco worker');
  self.MonacoEnvironment = {
    getWorker: function (_workerId: string, label: string) {
      switch (label) {
        case 'json':
          return new MonacoJsonWorker();
        case 'css':
        case 'scss':
        case 'less':
          return new MonacoCssWorker();
        case 'html':
        case 'handlebars':
        case 'razor':
          return new MonacoHtmlWorker();
        case 'typescript':
        case 'javascript':
          return new MonacoTsWorker();
        default:
          return new MonacoEditorWorker();
      }
    },
  };
}

export async function initViewers(): Promise<void> {
  _initPdf();
  _initMonaco();
}

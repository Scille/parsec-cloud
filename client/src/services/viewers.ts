// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import * as monaco from 'monaco-editor';
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
import cssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker';
import htmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker';
import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';
import jsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker';
import * as pdfjs from 'pdfjs-dist';
import pdfjsWorker from 'pdfjs-dist/build/pdf.worker?worker&url';

async function _initPdf(): Promise<void> {
  pdfjs.GlobalWorkerOptions.workerSrc = pdfjsWorker;
}

async function _initText(): Promise<void> {
  /* MsTheme: Monaco editor custom theme */
  monaco.editor.defineTheme('msEditorTheme', {
    base: 'vs',
    inherit: true,
    rules: [
      {
        token: 'editor.border',
        foreground: '#000000',
        fontStyle: '1px solid',
      },
      {
        token: 'editor.border',
        foreground: '#000000',
        fontStyle: '1px solid',
      },
    ],
    colors: {
      'editor.foreground': '#000000',
      'editor.background': '#ffffff',
      'editorCursor.foreground': '#0058cc',
      'editor.lineHighlightBackground': '#f3f3f7',
      'editorLineNumber.foreground': '#0058cc',
      'editor.selectionBackground': '#99c5ff',
      'editor.inactiveSelectionBackground': '#88000015',
      'scrollbar.shadow': '#f3f3f7',
      'scrollbarSlider.background': '#cce2ff',
      'scrollbarSlider.hoverBackground': '#99c5ff',
      'scrollbarSlider.activeBackground': '#66a8ff',
    },
  });
  self.MonacoEnvironment = {
    getWorker: function (_workerId, label): Worker {
      switch (label) {
        case 'json':
          return new jsonWorker();
        case 'css':
        case 'scss':
        case 'less':
          return new cssWorker();
        case 'html':
        case 'handlebars':
        case 'razor':
          return new htmlWorker();
        case 'typescript':
        case 'javascript':
          return new jsWorker();
        default:
          return new editorWorker();
      }
    },
  };
}

export async function initViewers(): Promise<void> {
  await _initPdf();
  await _initText();
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Env } from '@/services/environment';

export enum CryptpadDocumentType {
  Pad = 'pad',
  Sheet = 'sheet',
  Doc = 'doc',
  Presentation = 'presentation',
  Code = 'code',
  Unsupported = 'unsupported',
}

export const ENABLED_DOCUMENT_TYPES = [
  CryptpadDocumentType.Pad,
  CryptpadDocumentType.Sheet,
  CryptpadDocumentType.Doc,
  CryptpadDocumentType.Presentation,
  CryptpadDocumentType.Code,
];

interface CryptpadConfig {
  document: {
    url: string;
    fileType: string;
    title: string;
    key?: string;
  };
  documentType: CryptpadDocumentType;
  editorConfig: {
    lang: string;
  };
  autosave: number;
  events: {
    onSave: (file: Blob, callback: () => void) => void;
    onNewKey?: (data: { new: string; old: string }, callback: (key: string) => void) => void;
    onHasUnsavedChanges?: (unsaved: boolean) => void;
  };
}

export class Cryptpad {
  private scriptId = 'cryptpad-api-js';
  private containerElement: HTMLElement;
  private script: HTMLScriptElement;
  private instanceInitialized = false;

  constructor(containerElement: HTMLElement, serverUrl: string) {
    this.containerElement = containerElement;
    if (!Env.isEditicsEnabled()) {
      throw new Error('Editics is not enabled. Cannot initialize CryptPad.');
    }

    let script = document.getElementById(this.scriptId) as HTMLScriptElement | null;
    if (!script) {
      script = document.createElement('script');
      if (!script) {
        throw new Error('Failed to create script element for CryptPad API.');
      }
      script.id = this.scriptId;
      script.src = `${serverUrl}/cryptpad-api.js`;
      script.async = true;
      document.head.appendChild(script);
    }
    this.script = script;
  }

  async init(): Promise<void> {
    if (this.instanceInitialized) return;

    // Check if script is already loaded globally
    const isScriptAlreadyLoaded = document.getElementById(this.scriptId) && typeof (window as any).CryptPadAPI === 'function';

    if (!isScriptAlreadyLoaded) {
      await new Promise<void>((resolve, reject) => {
        this.script.onload = (): void => {
          if (typeof (window as any).CryptPadAPI === 'function') {
            window.electronAPI.log('info', 'CryptPad API script loaded successfully.');
            resolve();
          } else {
            const errorMessage = 'CryptPad API script loaded but CryptPadAPI function is not available.';
            window.electronAPI.log('error', errorMessage);
            reject(new Error(errorMessage));
          }
        };

        // Add error handling for HTTPS/security issues
        this.script.onerror = (error): void => {
          window.electronAPI.log('error', `Failed to load CryptPad script: ${error.toString()}`);
          window.electronAPI.log('error', 'This might be due to HTTPS requirements. Check if CryptPad server requires secure context.');
          reject(error);
        };
      });
    } else {
      window.electronAPI.log('info', 'CryptPad API script already loaded, reusing.');
    }

    this.instanceInitialized = true;
  }

  async open(config: CryptpadConfig): Promise<void> {
    await this.init();

    if (!ENABLED_DOCUMENT_TYPES.includes(config.documentType)) {
      throw new Error(`CryptPad edition for document type ${config.documentType} is not enabled.`);
    }

    if (!this.containerElement) {
      throw new Error('Container element is not initialized. Please call init() before open().');
    }
    if (!this.instanceInitialized) {
      throw new Error('CryptPad instance is not initialized yet. Please wait for it to initialize before calling open().');
    }

    (window as any).CryptPadAPI(this.containerElement.id, { ...config });
  }
}

export function getDocumentTypeFromExtension(extension: string): CryptpadDocumentType {
  switch (extension.toLowerCase()) {
    case 'txt':
    case 'rtf':
      return CryptpadDocumentType.Pad;
    case 'csv':
    case 'xlsx':
      return CryptpadDocumentType.Sheet;
    case 'docx':
    case 'odt':
      return CryptpadDocumentType.Doc;
    case 'pptx':
    case 'odp':
      return CryptpadDocumentType.Presentation;
    case 'js':
    case 'ts':
    case 'py':
    case 'md':
      return CryptpadDocumentType.Code;
    default:
      return CryptpadDocumentType.Unsupported;
  }
}

export function isEnabledCryptpadDocumentType(extension: string): boolean {
  return ENABLED_DOCUMENT_TYPES.includes(getDocumentTypeFromExtension(extension));
}

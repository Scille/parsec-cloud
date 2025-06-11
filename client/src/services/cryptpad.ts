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

export interface CryptpadConfig {
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
  private script: HTMLScriptElement | null = null;
  private scriptLoaded = false;

  constructor(containerElement: HTMLElement) {
    this.containerElement = containerElement;
    if (!this.isEnabled()) {
      return;
    }
    this.script = document.getElementById(this.scriptId) as HTMLScriptElement | null;
    if (!this.script) {
      this.script = document.createElement('script');
      this.script.id = this.scriptId;
      this.script.src = `${this.getServer()}/cryptpad-api.js`;
      this.script.async = true;
      document.head.appendChild(this.script);
    }
  }

  isEnabled(): boolean {
    return Env.isCryptpadEnabled();
  }

  getServer(): string {
    return Env.getCryptpadServer();
  }

  async init(): Promise<void> {
    if (!this.script) return;
    if (this.scriptLoaded) return;
    await new Promise<void>((resolve) => {
      this.script!.onload = (): void => {
        if (typeof (window as any).CryptPadAPI === 'function') {
          this.scriptLoaded = true;
          console.log('CryptPad API script loaded successfully.');
        }
        resolve();
      };

      // Add error handling for HTTPS/security issues
      this.script!.onerror = (error): void => {
        console.error('Failed to load CryptPad script:', error);
        console.error('This might be due to HTTPS requirements. Check if CryptPad server requires secure context.');
        resolve();
      };
    });
  }

  async open(config: CryptpadConfig): Promise<void> {
    await this.init();

    if (!ENABLED_DOCUMENT_TYPES.includes(config.documentType)) {
      console.warn(`CryptPad edition for document type ${config.documentType} is not enabled.`);
      return;
    }

    if (!this.containerElement) {
      console.warn('Container element is not initialized. Please call init() before open().');
      return;
    }
    if (!this.scriptLoaded) {
      console.warn('CryptPad API script is not loaded yet. Please wait for it to load before calling open().');
      return;
    }

    // Firefox-specific key prefix for POC
    if (config.document && config.document.key && isFirefox()) {
      if (!config.document.key.startsWith('ff-')) {
        config.document.key = `ff-${config.document.key}`;
      }
    }
    (window as any).CryptPadAPI(this.containerElement.id, { ...config });
  }

  async close(): Promise<void> {
    if (!this.scriptLoaded) {
      console.warn('CryptPad API script is not loaded yet. Please wait for it to load before calling close().');
      return;
    }
    if (typeof (window as any).CryptPadAPI === 'function') {
      (window as any).CryptPadAPI.close();
    } else {
      console.warn('CryptPad API is not initialized.');
    }
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

function isFirefox(): boolean {
  return typeof navigator !== 'undefined' && /firefox/i.test(navigator.userAgent);
}

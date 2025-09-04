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

export enum CryptpadErrorCode {
  NotEnabled = 'not-enabled',
  NotInitialized = 'not-initialized',
  InitFailed = 'init-failed',
  ScriptElementCreationFailed = 'script-element-creation-failed',
  DocumentTypeNotEnabled = 'document-type-not-enabled',
}

export class CryptpadError extends Error {
  public code: CryptpadErrorCode;

  constructor(code: CryptpadErrorCode, message?: string) {
    super(message ?? `Cryptpad error: ${code}`);
    this.name = this.constructor.name;
    this.code = code;
    Object.setPrototypeOf(this, new.target.prototype);
  }
}

export class CryptpadInitError extends CryptpadError {
  constructor(
    code: CryptpadErrorCode.NotEnabled | CryptpadErrorCode.ScriptElementCreationFailed | CryptpadErrorCode.InitFailed,
    message?: string,
  ) {
    super(code, message ?? `Failed to initialize CryptPad: ${code}`);
  }
}

export class CryptpadOpenError extends CryptpadError {
  public documentType: string;

  constructor(code: CryptpadErrorCode.NotInitialized | CryptpadErrorCode.DocumentTypeNotEnabled, documentType: string, message?: string) {
    super(code, message ?? `Failed to open document type '${documentType}' with Cryptpad: ${code}`);
    this.documentType = documentType;
  }
}

export class Cryptpad {
  private static SCRIPT_ID = 'cryptpad-api-js';
  private containerElement: HTMLElement;
  private script?: HTMLScriptElement;
  private serverUrl: string;

  constructor(containerElement: HTMLElement, serverUrl: string = Env.getDefaultCryptpadServer()) {
    this.containerElement = containerElement;
    this.serverUrl = serverUrl;
  }

  private static async loadFakeDocument(): Promise<void> {
    // Open a dummy docx to make things load in cache
    const docEl = document.createElement('div');
    docEl.id = 'cryptpad-fake-doc';
    docEl.innerHTML = '';
    docEl.style.visibility = 'hidden';
    document.body.appendChild(docEl);

    const data = (await import('@/parsec/mock_files/docx')).default;

    const config = {
      document: {
        url: URL.createObjectURL(new Blob([data as Uint8Array<ArrayBuffer>], { type: 'application/octet-stream' })),
        fileType: 'docx',
        title: window.crypto.randomUUID(),
        key: window.crypto.randomUUID(),
      },
      documentType: CryptpadDocumentType.Doc,
      editorConfig: {
        lang: 'en',
      },
      autosave: 10000,
      events: {
        onSave: async (_file: Blob, callback: () => void): Promise<void> => {
          callback();
        },
        onHasUnsavedChanges: (_unsaved: boolean): void => {},
      },
    };

    // Watch for the creation of the iframe
    const observer = new MutationObserver((mutationsList, obs) => {
      for (const mutation of mutationsList) {
        for (const node of mutation.addedNodes) {
          if (node instanceof HTMLElement && (node as HTMLElement).id === 'cryptpad-editor') {
            // Hide it immediately
            node.style.display = 'none';
            // Delete it later and remove the observer
            setTimeout(() => {
              node.remove();
              obs.disconnect();
            }, 10000);
          }
        }
      }
    });

    observer.observe(document.body, { childList: true, subtree: false });
    (window as any).CryptPadAPI(docEl.id, { ...config });
  }

  static async preload(serverUrl: string = Env.getDefaultCryptpadServer()): Promise<HTMLScriptElement> {
    if (!Env.isEditicsEnabled()) {
      throw new CryptpadInitError(CryptpadErrorCode.NotEnabled);
    }
    // Check if the script exist in the DOM, maybe created by another instance
    let script = document.getElementById(Cryptpad.SCRIPT_ID) as HTMLScriptElement | null;
    if (!script) {
      script = document.createElement('script');
      if (!script) {
        throw new CryptpadInitError(CryptpadErrorCode.ScriptElementCreationFailed);
      }
      script.id = Cryptpad.SCRIPT_ID;
      script.src = `${serverUrl}/cryptpad-api.js`;
      script.async = true;
      document.head.appendChild(script);

      await new Promise<void>((resolve, reject) => {
        (script as HTMLScriptElement).onload = (): void => {
          if (typeof (window as any).CryptPadAPI === 'function') {
            window.electronAPI.log('info', 'CryptPad API script loaded successfully.');
            Cryptpad.loadFakeDocument()
              .then(() => {
                resolve();
              })
              .catch(() => {
                reject(new CryptpadError(CryptpadErrorCode.InitFailed));
              });
          } else {
            const errorMessage = 'CryptPad API script loaded but CryptPadAPI function is not available.';
            window.electronAPI.log('error', errorMessage);
            reject(new CryptpadError(CryptpadErrorCode.InitFailed));
          }
        };

        // Add error handling for HTTPS/security issues
        (script as HTMLScriptElement).onerror = (error): void => {
          window.electronAPI.log('error', `Failed to load CryptPad script: ${error.toString()}`);
          window.electronAPI.log('error', 'This might be due to HTTPS requirements. Check if CryptPad server requires secure context.');
          reject(new CryptpadInitError(CryptpadErrorCode.InitFailed, error.toString()));
        };
      });
    } else {
      window.electronAPI.log('info', 'CryptPad API script already loaded');
    }
    return script;
  }

  async init(): Promise<void> {
    this.script = await Cryptpad.preload(this.serverUrl);
  }

  async open(config: CryptpadConfig): Promise<void> {
    if (!this.containerElement) {
      window.electronAPI.log('error', 'Container element is not initialized. Please call init() before open().');
      throw new CryptpadOpenError(CryptpadErrorCode.NotInitialized, config.documentType);
    }

    // for this case, a simple information modal "Failed to open document"
    if (!this.script) {
      window.electronAPI.log('error', 'CryptPad instance is not initialized yet. Please wait for it to initialize before calling open().');
      throw new CryptpadOpenError(CryptpadErrorCode.NotInitialized, config.documentType);
    }

    if (!ENABLED_DOCUMENT_TYPES.includes(config.documentType)) {
      window.electronAPI.log('error', `CryptPad edition for document type ${config.documentType} is not enabled.`);
      throw new CryptpadOpenError(CryptpadErrorCode.DocumentTypeNotEnabled, config.documentType);
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

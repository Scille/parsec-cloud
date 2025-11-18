// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { detectFileContentType, FileContentType } from '@/common/fileTypes';
import { Env } from '@/services/environment';

export enum CryptpadDocumentType {
  Pad = 'pad',
  Sheet = 'sheet',
  Doc = 'doc',
  Presentation = 'presentation',
  Code = 'code',
  Unsupported = 'unsupported',
}

export enum CryptpadAppMode {
  View = 'view',
  Edit = 'edit',
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
    user?: {
      name: string;
      id: string;
    };
  };
  autosave: number;
  mode?: CryptpadAppMode;
  events: {
    onSave: (file: Blob, callback: () => void) => void;
    onNewKey?: (data: { new: string; old: string }, callback: (key: string) => void) => void;
    onHasUnsavedChanges?: (unsaved: boolean) => void;
    onError?: (error: { message: string; errorType: string; originalError: string }) => void;
    onReady?: () => void;
  };
}

export enum CryptpadErrorCode {
  NotEnabled = 'not-enabled',
  NotInitialized = 'not-initialized',
  InitFailed = 'init-failed',
  ScriptElementCreationFailed = 'script-element-creation-failed',
  DocumentTypeNotEnabled = 'document-type-not-enabled',
  LoadingTimeout = 'loading-timeout',
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

  constructor(
    code: CryptpadErrorCode.NotInitialized | CryptpadErrorCode.DocumentTypeNotEnabled | CryptpadErrorCode.LoadingTimeout,
    documentType: string,
    message?: string,
  ) {
    super(code, message ?? `Failed to open document type '${documentType}' with Cryptpad: ${code}`);
    this.documentType = documentType;
  }
}

export class Cryptpad {
  private scriptId = 'cryptpad-api-js';
  private containerElement: HTMLElement;
  private script?: HTMLScriptElement;
  private serverUrl: string;

  constructor(containerElement: HTMLElement, serverUrl: string) {
    this.containerElement = containerElement;
    this.serverUrl = serverUrl;
  }

  async init(): Promise<void> {
    if (!Env.isEditicsEnabled()) {
      throw new CryptpadInitError(CryptpadErrorCode.NotEnabled);
    }

    // Script is already set for this instance (init() called multiple times)
    if (this.script) {
      window.electronAPI.log('info', 'CryptPad API script already loaded.');
      return;
    }

    // Check if the script exist in the DOM, maybe created by another instance
    let script = document.getElementById(this.scriptId) as HTMLScriptElement | null;
    if (!script) {
      script = document.createElement('script');
      if (!script) {
        throw new CryptpadInitError(CryptpadErrorCode.ScriptElementCreationFailed);
      }
      script.id = this.scriptId;
      script.src = `${this.serverUrl}/cryptpad-api.js`;
      script.async = true;
      document.head.appendChild(script);

      await new Promise<void>((resolve, reject) => {
        (script as HTMLScriptElement).onload = (): void => {
          if (typeof (window as any).CryptPadAPI === 'function') {
            window.electronAPI.log('info', 'CryptPad API script loaded successfully.');
            resolve();
          } else {
            const errorMessage = 'CryptPad API script loaded but CryptPadAPI function is not available.';
            window.electronAPI.log('error', errorMessage);
            reject(new CryptpadError(CryptpadErrorCode.InitFailed));
          }
        };

        // Add error handling for HTTPS/security, network issues
        (script as HTMLScriptElement).onerror = (error): void => {
          window.electronAPI.log('error', `Failed to load CryptPad script: ${error.toString()}`);
          window.electronAPI.log('error', 'This might be due to HTTPS requirements. Check if CryptPad server requires secure context.');
          reject(new CryptpadInitError(CryptpadErrorCode.InitFailed, error.toString()));
          // We remove the faulty script to allow retrying
          script?.remove();
        };
      });
    } else {
      window.electronAPI.log('info', 'CryptPad API script previously loaded, reusing');
    }
    this.script = script;
  }

  async open(config: CryptpadConfig): Promise<void> {
    await this.init();

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

    // Set up a loading timeout (30 seconds) to detect stuck/corrupted files
    const LOADING_TIMEOUT_MS = 30000;
    let loadingTimeoutId: any = undefined;

    // Create a promise that will be resolved when CryptPad successfully loads
    let resolveLoading!: () => void;
    const loadingComplete = new Promise<void>((resolve) => {
      resolveLoading = resolve;
    });

    const timeoutPromise = new Promise<void>((_, reject) => {
      loadingTimeoutId = window.setTimeout(() => {
        const message = 'CryptPad loading timeout - the file may be corrupted, too large, or the server is not responding.';
        window.electronAPI.log('warn', message);
        reject(new CryptpadOpenError(CryptpadErrorCode.LoadingTimeout, config.documentType, message));
      }, LOADING_TIMEOUT_MS);
    });
    // Wrap the original onReady callback to signal successful loading
    const originalOnReady = config.events.onReady;
    config.events.onReady = () => {
      resolveLoading();
      if (originalOnReady) originalOnReady();
    };

    // Store config globally so CryptPad customization can access the onError callback
    (window as any).cryptpadConfig = config;

    try {
      // Start CryptPad API (doesn't wait for loading to complete)
      void (window as any).CryptPadAPI(this.containerElement.id, { ...config });

      // Race between successful loading (onReady callback) and timeout
      await Promise.race([loadingComplete, timeoutPromise]);
    } finally {
      window.clearTimeout(loadingTimeoutId);
      // Clean up global config reference
      delete (window as any).cryptpadConfig;
    }
  }
}

export function getCryptpadDocumentType(contentType: FileContentType): CryptpadDocumentType {
  // CryptpadDocumentType.Pad adds html content to text files, avoiding using it for now
  // CryptpadDocumentType.Presentation works with .pptx, but is disabled as long as there's no viewer for it
  switch (contentType) {
    case FileContentType.Text:
      return CryptpadDocumentType.Code;
    case FileContentType.Spreadsheet:
      return CryptpadDocumentType.Sheet;
    case FileContentType.Document:
      return CryptpadDocumentType.Doc;
    default:
      return CryptpadDocumentType.Unsupported;
  }
}

export function isEnabledCryptpadDocumentType(contentType: FileContentType): boolean {
  return ENABLED_DOCUMENT_TYPES.includes(getCryptpadDocumentType(contentType));
}

export function isFileEditable(name: string): boolean {
  if (!Env.isEditicsEnabled()) {
    return false;
  }
  const fileContentType = detectFileContentType(name);
  if (!fileContentType) {
    return false;
  }
  return isEnabledCryptpadDocumentType(fileContentType.type);
}

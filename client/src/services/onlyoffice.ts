// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { detectOpenableFile, FileContentType } from '@/common/fileTypes';
import { Env } from '@/services/environment';

// POC: replaces the (broken) Cryptpad integration with a direct integration of OnlyOffice,
// using CryptPad's own end-to-end-encrypted fork of OnlyOffice (client-side code only, vendored
// in `client/vendors/onlyoffice`, see the README there).
//
// There is no collaborative server yet: `client/public/onlyoffice-host.html` wires the editor to
// CryptPad's `connectMockServer` API (their replacement for OnlyOffice's usual socket.io-based
// server communication) with handlers that keep everything local to the current tab. Nothing is
// ever sent back to Parsec: the caller is expected to read the file content ahead of time and pass
// it in, and no "save" mechanism is implemented on top of this module yet.

// Should be the same on both sides (this file and public/onlyoffice-host.html), don't modify one
// without updating the other.
namespace OnlyOfficeCommAPI {
  export enum Commands {
    HostReady = 'oo-host-ready',
    Open = 'oo-open',
    AppReady = 'oo-app-ready',
    Ready = 'oo-ready',
    Error = 'oo-error',
  }

  export enum DocumentTypes {
    Word = 'word',
    Cell = 'cell',
    Slide = 'slide',
    Unsupported = 'unsupported',
  }

  export enum OpenModes {
    View = 'view',
    Edit = 'edit',
  }

  export interface OpenDocumentOptions {
    documentContent: Uint8Array;
    documentName: string;
    documentType: OnlyOfficeCommAPI.DocumentTypes;
    key: string;
    userName: string;
    userId: string;
    mode: OnlyOfficeCommAPI.OpenModes;
    locale: string;
    theme?: 'light' | 'dark';
  }
}

export import OnlyOfficeDocumentType = OnlyOfficeCommAPI.DocumentTypes;
export import OnlyOfficeOpenModes = OnlyOfficeCommAPI.OpenModes;
export type OpenDocumentOptions = OnlyOfficeCommAPI.OpenDocumentOptions;

export enum OnlyOfficeErrorCodes {
  FrameNotLoaded = 'frame-not-loaded',
  FrameLoadFailed = 'frame-load-failed',
  EventError = 'event-error',
}

interface OnlyOfficeEventHandlers {
  onReady: () => void;
  onError: (error: any) => void;
}

export class OnlyOfficeError extends Error {
  public code: OnlyOfficeErrorCodes;
  public details?: string;

  constructor(code: OnlyOfficeErrorCodes, details?: string) {
    super(`OnlyOffice error: ${code}`);
    this.code = code;
    this.details = details;
    Object.setPrototypeOf(this, new.target.prototype);
  }
}

export interface OnlyOfficeSession {
  controller: AbortController;
}

const HOST_PAGE = `${import.meta.env.BASE_URL}onlyoffice-host.html`;

// The blank documents OnlyOffice itself uses when starting a new file from scratch, borrowed
// verbatim from CryptPad (see client/vendors/onlyoffice/README.md). Used as a fallback so that an
// empty file can still be opened (e.g. one freshly created through Parsec's generic "new file").
const BLANK_TEMPLATE_URLS: Partial<Record<OnlyOfficeDocumentType, string>> = {
  [OnlyOfficeDocumentType.Word]: `${import.meta.env.BASE_URL}onlyoffice-templates/word.bin`,
  [OnlyOfficeDocumentType.Cell]: `${import.meta.env.BASE_URL}onlyoffice-templates/cell.bin`,
  [OnlyOfficeDocumentType.Slide]: `${import.meta.env.BASE_URL}onlyoffice-templates/slide.bin`,
};

function onlyOfficeLog(level: 'debug' | 'warn' | 'error' | 'info', message: string): void {
  window.nativeAPI.log(level, `[OnlyOffice] ${message}`);
}

export function getOnlyOfficeDocumentType(contentType: FileContentType): OnlyOfficeDocumentType {
  switch (contentType) {
    case FileContentType.Document:
      return OnlyOfficeDocumentType.Word;
    case FileContentType.Spreadsheet:
      return OnlyOfficeDocumentType.Cell;
    case FileContentType.Presentation:
      return OnlyOfficeDocumentType.Slide;
    default:
      return OnlyOfficeDocumentType.Unsupported;
  }
}

export function isOnlyOfficeEnabledForDocumentType(contentType: FileContentType): boolean {
  if (!Env.isEditicsEnabled()) {
    return false;
  }
  return getOnlyOfficeDocumentType(contentType) !== OnlyOfficeDocumentType.Unsupported;
}

export function isFileEditable(name: string): boolean {
  if (!Env.isEditicsEnabled()) {
    return false;
  }
  const fileContentType = detectOpenableFile(name);
  return isOnlyOfficeEnabledForDocumentType(fileContentType.type);
}

// An empty file (e.g. freshly created and never opened yet) isn't a valid OnlyOffice document:
// substitute one of the blank templates above so the editor has something to load.
async function getEditableContent(documentContent: Uint8Array, documentType: OnlyOfficeDocumentType): Promise<Uint8Array> {
  if (documentContent.byteLength > 0) {
    return documentContent;
  }
  const templateUrl = BLANK_TEMPLATE_URLS[documentType];
  if (!templateUrl) {
    return documentContent;
  }
  try {
    const response = await fetch(templateUrl);
    return new Uint8Array(await response.arrayBuffer());
  } catch (e: any) {
    onlyOfficeLog('warn', `Failed to load blank document template: ${e.toString()}`);
    return documentContent;
  }
}

export async function openDocument(
  options: OpenDocumentOptions,
  handlers: OnlyOfficeEventHandlers,
  frame: HTMLIFrameElement,
): Promise<OnlyOfficeSession | undefined> {
  const controller = new AbortController();

  const documentContent = await getEditableContent(options.documentContent, options.documentType);

  try {
    await new Promise<void>((resolve, reject) => {
      const timeoutId = setTimeout(
        () => {
          reject();
        },
        (window as any).TESTING === true ? 500 : 5000,
      );

      window.addEventListener(
        'message',
        (event) => {
          if (event.source !== frame.contentWindow) {
            return;
          }
          switch (event.data?.command) {
            case OnlyOfficeCommAPI.Commands.HostReady: {
              clearTimeout(timeoutId);
              resolve();
              break;
            }
            case OnlyOfficeCommAPI.Commands.Ready: {
              onlyOfficeLog('debug', 'Document is ready');
              handlers.onReady();
              break;
            }
            case OnlyOfficeCommAPI.Commands.Error: {
              handlers.onError(new OnlyOfficeError(OnlyOfficeErrorCodes.EventError, event.data.details));
              break;
            }
          }
        },
        { signal: controller.signal },
      );
      frame.src = HOST_PAGE;
    });
  } catch (e: unknown) {
    controller.abort();
    handlers.onError(new OnlyOfficeError(OnlyOfficeErrorCodes.FrameLoadFailed, JSON.stringify(e)));
    return undefined;
  }

  onlyOfficeLog('debug', 'Host frame is ready, opening the document');

  if (!frame.contentWindow) {
    controller.abort();
    handlers.onError(new OnlyOfficeError(OnlyOfficeErrorCodes.FrameNotLoaded));
    return undefined;
  }

  frame.contentWindow.postMessage(
    {
      command: OnlyOfficeCommAPI.Commands.Open,
      options: { ...options, documentContent },
    },
    '*',
  );

  return { controller };
}

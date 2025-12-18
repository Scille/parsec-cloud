// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { detectFileContentType, FileContentType } from '@/common/fileTypes';
import { Env } from '@/services/environment';

// Should be the same on both sides, don't modify one
// without updating the other.
namespace CryptpadCommAPI {
  export enum Commands {
    Ready = 'editics-ready',
    Hello = 'editics-hello',
    Init = 'editics-init',
    InitResult = 'editics-init-result',
    Open = 'editics-open',
    OpenResult = 'editics-open-result',
    Event = 'editics-event',
  }

  export enum Events {
    SaveStatus = 'save-status',
    Save = 'save',
    Error = 'error',
    Ready = 'ready',
  }

  export enum ErrorCodes {
    OpenInvalidConfig = 'open-invalid-config',
    OpenFailed = 'open-failed',
    InitFailed = 'init-failed',
  }

  export enum Editors {
    Pad = 'pad',
    Sheet = 'sheet',
    Doc = 'doc',
    Presentation = 'presentation',
    Code = 'code',
    Unsupported = 'unsupported',
  }

  export enum OpenModes {
    View = 'view',
    Edit = 'edit',
  }

  export interface OpenDocumentOptions {
    documentContent: Uint8Array;
    documentName: string;
    documentExtension: string;
    cryptpadEditor: CryptpadCommAPI.Editors;
    key: string;
    userName: string;
    userId: string;
    autosaveInterval: number;
    mode: CryptpadCommAPI.OpenModes;
    locale: string;
  }
}

export import CryptpadEditors = CryptpadCommAPI.Editors;
export import CryptpadOpenModes = CryptpadCommAPI.OpenModes;

export enum CryptpadErrorCodes {
  OpenInvalidConfig = CryptpadCommAPI.ErrorCodes.OpenInvalidConfig,
  OpenFailed = CryptpadCommAPI.ErrorCodes.OpenFailed,
  InitFailed = CryptpadCommAPI.ErrorCodes.InitFailed,
  FrameNotLoaded = 'frame-not-loaded',
  FrameLoadFailed = 'frame-load-failed',
}

export type OpenDocumentOptions = CryptpadCommAPI.OpenDocumentOptions;

interface CryptpadEventHandlers {
  onSave: (file: Blob) => void;
  onError: (error: any) => void;
  onReady: () => void;
  onHasUnsavedChanges: (unsaved: boolean) => void;
}

export class CryptpadError extends Error {
  public code: CryptpadErrorCodes;
  public details?: string;

  constructor(code: CryptpadErrorCodes, details?: string) {
    super(`Cryptpad error: ${code}`);
    this.code = code;
    this.details = details;
    Object.setPrototypeOf(this, new.target.prototype);
  }
}

export function getCryptpadEditor(contentType: FileContentType): CryptpadEditors {
  switch (contentType) {
    case FileContentType.Text:
      return CryptpadEditors.Code;
    case FileContentType.Spreadsheet:
      return CryptpadEditors.Sheet;
    case FileContentType.Document:
      return CryptpadEditors.Doc;
    case FileContentType.Presentation:
      return CryptpadEditors.Presentation;
    default:
      return CryptpadEditors.Unsupported;
  }
}

export function isCryptpadEnabledForDocumentType(contentType: FileContentType): boolean {
  if (!Env.isEditicsEnabled()) {
    return false;
  }
  const ENABLED_EDITORS = [
    CryptpadEditors.Pad,
    CryptpadEditors.Sheet,
    CryptpadEditors.Doc,
    CryptpadEditors.Presentation,
    CryptpadEditors.Code,
  ];

  return ENABLED_EDITORS.includes(getCryptpadEditor(contentType));
}

export function isFileEditable(name: string): boolean {
  if (!Env.isEditicsEnabled()) {
    return false;
  }
  const fileContentType = detectFileContentType(name);
  if (!fileContentType) {
    return false;
  }
  return isCryptpadEnabledForDocumentType(fileContentType.type);
}

function cryptpadLog(level: 'debug' | 'warn' | 'error' | 'info', message: string): void {
  window.electronAPI.log(level, `[Cryptpad] ${message}`);
}

export async function openDocument(
  options: OpenDocumentOptions,
  handlers: CryptpadEventHandlers,
  frame: HTMLIFrameElement,
): Promise<AbortController | undefined> {
  const CRYPTPAD_SERVER = Env.getDefaultCryptpadServer();

  function sendMessageToFrame(command: CryptpadCommAPI.Commands, data?: any): void {
    if (!frame.contentWindow) {
      throw new CryptpadError(CryptpadErrorCodes.FrameNotLoaded);
    }
    frame.contentWindow.postMessage({ command: command, ...data }, CRYPTPAD_SERVER);
  }

  try {
    await new Promise<void>((resolve, reject) => {
      const abortController = new AbortController();

      const timeoutId = setTimeout(() => {
        abortController.abort();
        reject();
      }, 5000);

      window.addEventListener(
        'message',
        (event) => {
          if (event.data.command === CryptpadCommAPI.Commands.Ready) {
            clearTimeout(timeoutId);
            abortController.abort();
            resolve();
          }
        },
        { signal: abortController.signal },
      );
      // Loading the iframe
      frame.src = `${CRYPTPAD_SERVER}/frame.html`;
    });
  } catch (e: unknown) {
    handlers.onError(new CryptpadError(CryptpadErrorCodes.FrameLoadFailed, JSON.stringify(e)));
    return undefined;
  }

  cryptpadLog('debug', 'Frame is ready');

  const controller = new AbortController();

  // Init the listener
  window.addEventListener(
    'message',
    (event) => {
      if (event.origin === 'parsec-desktop://-') {
        return;
      }
      if (event.origin !== CRYPTPAD_SERVER) {
        cryptpadLog('debug', `Ignored origin '${event.origin}'`);
        return;
      }
      switch (event.data.command) {
        case CryptpadCommAPI.Commands.InitResult: {
          if (event.data.success) {
            cryptpadLog('debug', 'Init success, opening the file...');
            sendMessageToFrame(CryptpadCommAPI.Commands.Open, options);
          } else {
            handlers.onError(new CryptpadError(CryptpadErrorCodes.InitFailed, event.data.details));
          }
          break;
        }
        case CryptpadCommAPI.Commands.OpenResult: {
          if (event.data.success) {
            cryptpadLog('debug', 'Successfully opened the document');
          } else {
            handlers.onError(new CryptpadError(event.data.error, event.data.details));
          }
          break;
        }
        case CryptpadCommAPI.Commands.Event: {
          switch (event.data.event) {
            case CryptpadCommAPI.Events.Error: {
              handlers.onError(event.data.details);
              break;
            }
            case CryptpadCommAPI.Events.Ready: {
              handlers.onReady();
              break;
            }
            case CryptpadCommAPI.Events.Save: {
              handlers.onSave(event.data.documentContent);
              break;
            }
            case CryptpadCommAPI.Events.SaveStatus: {
              handlers.onHasUnsavedChanges(!event.data.saved);
              break;
            }
            case undefined: {
              cryptpadLog('warn', `Command ${CryptpadCommAPI.Commands.Event} received but without an event`);
              break;
            }
            default: {
              cryptpadLog('warn', `Unknown event '${event.data.event}'`);
              break;
            }
          }
        }
        case undefined: {
          cryptpadLog('debug', 'No command, ignored');
          break;
        }
        default: {
          cryptpadLog('warn', `Unknown command '${event.data.command}'`);
          break;
        }
      }
    },
    { signal: controller.signal },
  );

  // Init the frame
  cryptpadLog('debug', 'Saying hello to the frame');
  sendMessageToFrame(CryptpadCommAPI.Commands.Hello);
  cryptpadLog('debug', 'Initializing the frame');
  sendMessageToFrame(CryptpadCommAPI.Commands.Init);

  return controller;
}

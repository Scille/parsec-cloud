// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { detectOpenableFile, FileContentType } from '@/common/fileTypes';
import { Env } from '@/services/environment';

// `ParsecCryptpadCommAPI` is the API shared between the cryptpad editor running in an Iframe
// and the parent Parsec client (i.e. us !).
//
// - See `resources/www/frame.js` in custom Parsec-Cryptpad repository for the part living in the Iframe.
// - Don't modify it without keeping in sync both parts!
// - When modifying it, don't forget to bump `ParsecCryptpadCommAPI.version` so that
//   older Parsec client can detect they are no longer compatible.
namespace ParsecCryptpadCommAPI {
  export const version = 1;

  export enum MessageTag {
    CryptpadInitialized = 'cryptpad-initialized',
    ParsecOpenDocument = 'parsec-open-document',
    CryptpadOpenDocumentResult = 'cryptpad-open-document-result',
    ParsecRequestSaveDocument = 'parsec-request-save-document',
    CryptpadRequestSaveDocumentResult = 'cryptpad-request-save-document-result',
    CryptpadOnSave = 'cryptpad-on-save',
    ParsecOnSaveResult = 'parsec-on-save-result',
    CryptpadOnHasUnsavedChanges = 'cryptpad-on-has-unsaved-changes',
    CryptpadOnError = 'cryptpad-on-error',
    CryptpadOnNewKey = 'cryptpad-on-new-key',
    CryptpadOnInsertImage = 'cryptpad-on-insert-image',
    ParsecOnInsertImageResult = 'parsec-on-insert-image-result',
  }

  export enum Editor {
    Pad = 'pad',
    Sheet = 'sheet',
    Doc = 'doc',
    Presentation = 'presentation',
    Code = 'code',
    Unsupported = 'unsupported',
  }

  export enum OpenMode {
    View = 'view',
    Edit = 'edit',
  }

  export interface OpenDocumentConfig {
    documentContent: Uint8Array;
    documentName: string;
    documentExtension: string;
    cryptpadEditor: ParsecCryptpadCommAPI.Editor;
    key: string;
    userName: string;
    userId: string;
    autosaveInterval: number;
    mode: ParsecCryptpadCommAPI.OpenMode;
    locale: string;
  }

  // Sent by Cryptpad when the Iframe has finished loading `frame.html`
  export interface MessageCryptpadInitialized {
    tag: MessageTag.CryptpadInitialized;
    version: number;
    error: undefined | string;
  }

  export interface MessageParsecOpenDocument {
    tag: MessageTag.ParsecOpenDocument;
    config: OpenDocumentConfig;
  }

  export interface MessageCryptpadOpenDocumentResult {
    tag: MessageTag.CryptpadOpenDocumentResult;
    error: undefined | string;
  }

  export interface MessageParsecRequestSaveDocument {
    tag: MessageTag.ParsecRequestSaveDocument;
  }

  export interface MessageCryptpadRequestSaveDocumentResult {
    tag: MessageTag.CryptpadRequestSaveDocumentResult;
    error: undefined | string;
  }

  export interface MessageCryptpadOnSave {
    tag: MessageTag.CryptpadOnSave;
    messageId: number;
    documentContent: Blob;
  }

  export interface MessageParsecOnSaveResult {
    tag: MessageTag.ParsecOnSaveResult;
    messageId: number;
  }

  export interface MessageCryptpadOnHasUnsavedChanges {
    tag: MessageTag.CryptpadOnHasUnsavedChanges;
    unsaved: boolean;
  }

  export interface MessageCryptpadOnError {
    tag: MessageTag.CryptpadOnError;
    error: string;
  }

  export interface MessageCryptpadOnNewKey {
    tag: MessageTag.CryptpadOnNewKey;
  }

  export interface MessageCryptpadOnInsertImage {
    tag: MessageTag.CryptpadOnInsertImage;
    messageId: number;
  }

  export interface MessageParsecOnInsertImageResult {
    tag: MessageTag.ParsecOnInsertImageResult;
    messageId: number;
  }

  // Messages send by the Parsec client and received by the Cryptpad Iframe
  export type ParsecMessage =
    | MessageParsecOpenDocument
    | MessageParsecRequestSaveDocument
    | MessageParsecOnSaveResult
    | MessageParsecOnInsertImageResult;
  // Messages send by the Cryptpad Iframe and received by the Parsec client
  export type CryptpadMessage =
    | MessageCryptpadInitialized
    | MessageCryptpadOpenDocumentResult
    | MessageCryptpadRequestSaveDocumentResult
    | MessageCryptpadOnSave
    | MessageCryptpadOnHasUnsavedChanges
    | MessageCryptpadOnError
    | MessageCryptpadOnNewKey
    | MessageCryptpadOnInsertImage;
}

export import CryptpadEditor = ParsecCryptpadCommAPI.Editor;
export import CryptpadOpenMode = ParsecCryptpadCommAPI.OpenMode;

export type CryptpadOpenDocumentConfig = ParsecCryptpadCommAPI.OpenDocumentConfig;

interface CryptpadEventHandlers {
  onSave: (file: Blob) => void;
  onError: (error: any) => void;
  onReady: () => void;
  onHasUnsavedChanges: (unsaved: boolean) => void;
}

export enum CryptpadErrorCode {
  NotAvailable = 'cryptpad-not-available',
  FrameNotLoaded = 'frame-not-loaded',
  FrameLoadFailed = 'frame-load-failed',
  OpenDocumentInvalidConfig = 'open-document-invalid-config',
  OpenDocumentFailed = 'open-document-failed',
  EventError = 'event-error',
}

export class CryptpadError extends Error {
  public code: CryptpadErrorCode;
  public details?: string;

  constructor(code: CryptpadErrorCode, details?: string) {
    super(`Cryptpad error: ${code}`);
    this.code = code;
    this.details = details;
    Object.setPrototypeOf(this, new.target.prototype);
  }
}

export function getCryptpadEditor(contentType: FileContentType): CryptpadEditor {
  switch (contentType) {
    case FileContentType.Text:
      return CryptpadEditor.Code;
    case FileContentType.Spreadsheet:
      return CryptpadEditor.Sheet;
    case FileContentType.Document:
      return CryptpadEditor.Doc;
    case FileContentType.Presentation:
      return CryptpadEditor.Presentation;
    default:
      return CryptpadEditor.Unsupported;
  }
}

export function isCryptpadEnabledForDocumentType(contentType: FileContentType): boolean {
  if (!Env.isEditicsEnabled()) {
    return false;
  }
  const ENABLED_EDITORS = [CryptpadEditor.Pad, CryptpadEditor.Sheet, CryptpadEditor.Doc, CryptpadEditor.Presentation, CryptpadEditor.Code];

  return ENABLED_EDITORS.includes(getCryptpadEditor(contentType));
}

export function isFileEditable(name: string): boolean {
  if (!Env.isEditicsEnabled()) {
    return false;
  }
  const fileContentType = detectOpenableFile(name);
  return isCryptpadEnabledForDocumentType(fileContentType.type);
}

function cryptpadLog(level: 'debug' | 'warn' | 'error' | 'info', message: string): void {
  window.electronAPI.log(level, `[Cryptpad] ${message}`);
}

export interface CryptpadSession {
  controller: AbortController;
  save: () => void;
}

export async function openDocument(
  config: CryptpadOpenDocumentConfig,
  handlers: CryptpadEventHandlers,
  frame: HTMLIFrameElement,
): Promise<CryptpadSession | undefined> {
  const CRYPTPAD_SERVER = await Env.getCryptpadServer();

  if (!CRYPTPAD_SERVER) {
    handlers.onError(new CryptpadError(CryptpadErrorCode.NotAvailable));
    return undefined;
  }

  window.electronAPI.log('debug', `Trying to open document on server '${CRYPTPAD_SERVER}'`);

  function sendMessageToFrame(message: ParsecCryptpadCommAPI.ParsecMessage): void {
    if (!frame.contentWindow) {
      throw new CryptpadError(CryptpadErrorCode.FrameNotLoaded);
    }
    frame.contentWindow.postMessage(message, CRYPTPAD_SERVER as string);
  }

  // 1) Initialize Cryptpad in the Iframe
  //
  // This is done in 3 times:
  // - Configure the `frame.html` entrypoint
  // - Wait for the initialized message from the Iframe (i.e. Cryptpad is ready)
  // - Ensure the Iframe speaks the same API than us

  try {
    await new Promise<void>((resolve, reject) => {
      const abortEventListening = new AbortController();

      const timeoutId = setTimeout(
        () => {
          abortEventListening.abort();
          reject('Cryptpad Iframe takes too long to load');
        },
        (window as any).TESTING === true ? 500 : 5000,
      );

      window.addEventListener(
        'message',
        (event) => {
          console.log('1111 Parsec client receving message', event);
          if (event.origin !== CRYPTPAD_SERVER) {
            return;
          }

          if (event.data.tag === ParsecCryptpadCommAPI.MessageTag.CryptpadInitialized) {
            const message = event.data as ParsecCryptpadCommAPI.MessageCryptpadInitialized;
            clearTimeout(timeoutId);
            abortEventListening.abort();
            // API version check must always be done first!
            if (message.version !== ParsecCryptpadCommAPI.version) {
              // eslint-disable-next-line max-len
              const msg = `Incompatible API between Parsec client (version: ${ParsecCryptpadCommAPI.version}) and Cryptpad Iframe (version: ${message.version})`;
              cryptpadLog('warn', msg);
              reject(msg);
            }
            if (message.error !== undefined) {
              const msg = `Cryptpad Iframe has failed to initialize: ${message.error}`;
              cryptpadLog('warn', msg);
              reject(msg);
            }
            console.log('1111 Cryptpad initialized!');
            resolve();
          } else {
            cryptpadLog('warn', `Unknown command: ${JSON.stringify(event.data)}`);
          }
        },
        { signal: abortEventListening.signal },
      );
      // Loading the iframe
      // eslint-disable-next-line max-len
      frame.src = `${CRYPTPAD_SERVER}/frame.html?parsec_cryptpad_comm_api_version=${ParsecCryptpadCommAPI.version}&origin=${encodeURIComponent(window.location.origin)}`;
    });
  } catch (e: unknown) {
    handlers.onError(new CryptpadError(CryptpadErrorCode.FrameLoadFailed, JSON.stringify(e)));
    return undefined;
  }

  cryptpadLog('debug', 'Iframe is ready');

  // 2) Plug ourself to the API

  const controller = new AbortController();

  window.addEventListener(
    'message',
    (event) => {
      console.log('2222 Parsec client receving message', event);
      if (event.origin !== CRYPTPAD_SERVER) {
        return;
      }

      switch (event.data.tag) {
        case ParsecCryptpadCommAPI.MessageTag.CryptpadOpenDocumentResult: {
          console.log('2222 Cryptpad file opened!');
          const data = event.data as ParsecCryptpadCommAPI.MessageCryptpadOpenDocumentResult;
          if (data.error !== undefined) {
            handlers.onError(new CryptpadError(CryptpadErrorCode.OpenDocumentFailed, event.data.error));
          }
          break;
        }
        // TODO !!!!
        // case ParsecCryptpadCommAPI.MessageTag.CryptpadOnError:
        // case ParsecCryptpadCommAPI.MessageTag.CryptpadOnHasUnsavedChanges:
        // case ParsecCryptpadCommAPI.MessageTag.CryptpadOnInsertImage:
        // case ParsecCryptpadCommAPI.MessageTag.CryptpadOnNewKey:
        // case ParsecCryptpadCommAPI.MessageTag.CryptpadOnSave:
        // case ParsecCryptpadCommAPI.MessageTag.CryptpadRequestSaveDocumentResult:
        default: {
          cryptpadLog('warn', `Unknown command: ${JSON.stringify(event.data)}`);
          break;
        }
      }

      //     // if (event.origin === 'parsec-desktop://-') {
      //     //   return;
      //     // }
      //     if (event.origin !== CRYPTPAD_SERVER) {
      //       cryptpadLog('debug', `Ignored origin '${event.origin}'`);
      //       return;
      //     }
      //     switch (event.data.command) {
      //       case ParsecCryptpadCommAPI.Commands.InitResult: {
      //         if (event.data.success) {
      //           cryptpadLog('debug', 'Init success, opening the file...');
      //           sendMessageToFrame(ParsecCryptpadCommAPI.Commands.Open, options);
      //         } else {
      //           handlers.onError(new CryptpadError(CryptpadErrorCode.InitFailed, event.data.details));
      //         }
      //         break;
      //       }
      //       case ParsecCryptpadCommAPI.Commands.OpenResult: {
      //         if (event.data.success) {
      //           cryptpadLog('debug', 'Successfully opened the document');
      //         } else {
      //           handlers.onError(new CryptpadError(event.data.error, event.data.details));
      //         }
      //         break;
      //       }
      //       case ParsecCryptpadCommAPI.Commands.Event: {
      //         switch (event.data.event) {
      //           case ParsecCryptpadCommAPI.Events.Error: {
      //             handlers.onError(new CryptpadError(CryptpadErrorCode.EventError, event.data.details));
      //             break;
      //           }
      //           case ParsecCryptpadCommAPI.Events.Ready: {
      //             handlers.onReady();
      //             break;
      //           }
      //           case ParsecCryptpadCommAPI.Events.Save: {
      //             handlers.onSave(event.data.documentContent);
      //             break;
      //           }
      //           case ParsecCryptpadCommAPI.Events.SaveStatus: {
      //             handlers.onHasUnsavedChanges(!event.data.saved);
      //             break;
      //           }
      //           case undefined: {
      //             cryptpadLog('warn', `Command ${ParsecCryptpadCommAPI.Commands.Event} received but without an event`);
      //             break;
      //           }
      //           default: {
      //             cryptpadLog('warn', `Unknown event '${event.data.event}'`);
      //             break;
      //           }
      //         }
      //       }
      //       case undefined: {
      //         cryptpadLog('debug', 'No command, ignored');
      //         break;
      //       }
      //       default: {
      //         cryptpadLog('warn', `Unknown command '${event.data.command}'`);
      //         break;
      //       }
      //     }
    },
    { signal: controller.signal },
  );

  // 3) Finally open the document

  sendMessageToFrame({
    tag: ParsecCryptpadCommAPI.MessageTag.ParsecOpenDocument,
    config,
  } as ParsecCryptpadCommAPI.MessageParsecOpenDocument);

  return {
    controller,
    save: () => {
      cryptpadLog('debug', 'Triggering manual save');
      sendMessageToFrame({
        tag: ParsecCryptpadCommAPI.MessageTag.ParsecRequestSaveDocument,
      });
    },
  };
}

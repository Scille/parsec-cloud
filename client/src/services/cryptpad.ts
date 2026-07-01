// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

TODO:
- onlyoffice status bar always showing "all changes saved"
- onSave triggered every saveInterval time
- saveInterval time tout claqué (euristique avec min(interval / 2, 10s))
- onHasUnsavedChanged triggered n'importe comment (et boolean en paramètre qui veut rien dire), à ignorer ?
- save status à trigger dans à la fin du onSave (vu que onHasUnsavedChanged est bourré)
- ecrire dans frame.js du code pour gommer les soucis de onHasUnsavedChanged/onSave ?


// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { detectOpenableFile, FileContentType } from '@/common/fileTypes';
import { Env } from '@/services/environment';

type GenericError = string;

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
    CryptpadOpenDocumentReply = 'cryptpad-open-document-reply',
    ParsecRequestSaveDocument = 'parsec-request-save-document',
    CryptpadRequestSaveDocumentReply = 'cryptpad-request-save-document-reply',
    CryptpadOnSave = 'cryptpad-on-save',
    ParsecOnSaveReply = 'parsec-on-save-reply',
    CryptpadOnHasUnsavedChanges = 'cryptpad-on-has-unsaved-changes',
    CryptpadOnError = 'cryptpad-on-error',
    CryptpadOnNewKey = 'cryptpad-on-new-key',
    CryptpadOnInsertImage = 'cryptpad-on-insert-image',
    ParsecOnInsertImageReply = 'parsec-on-insert-image-reply',
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
    error: undefined | GenericError;
  }

  // Sent by us, once Cryptpad initialization is done, to open the file to edit/view
  export interface MessageParsecOpenDocument {
    tag: MessageTag.ParsecOpenDocument;
    messageId: number;
    config: OpenDocumentConfig;
  }

  // Sent by Cryptpad once the file is opened
  export interface MessageCryptpadOpenDocumentReply {
    tag: MessageTag.CryptpadOpenDocumentReply;
    messageId: number;
    error: undefined | GenericError;
  }

  // Sent by us to trigger a save. Basically Cryptpad will behave just
  // like `autosaveInterval` has been reached: convert the document then
  // send the Reply through a `MessageCryptpadOnSave` to let us do the
  // actual save.
  export interface MessageParsecRequestSaveDocument {
    tag: MessageTag.ParsecRequestSaveDocument;
    messageId: number;
  }

  // Sent by Cryptpad once the save is done (or has failed if `error` field is not `undefined`)
  export interface MessageCryptpadRequestSaveDocumentReply {
    tag: MessageTag.CryptpadRequestSaveDocumentReply;
    messageId: number;
    error: undefined | GenericError;
  }

  // Sent by Cryptpad to actually save the document.
  export interface MessageCryptpadOnSave {
    tag: MessageTag.CryptpadOnSave;
    messageId: number;
    documentContent: Blob;
  }

  // Sent by us once the document has been actually saved.
  // TODO: cannot return an error ?
  export interface MessageParsecOnSaveReply {
    tag: MessageTag.ParsecOnSaveReply;
    messageId: number;
    error: undefined | GenericError;
  }

  export interface MessageCryptpadOnHasUnsavedChanges {
    tag: MessageTag.CryptpadOnHasUnsavedChanges;
    unsaved: boolean;
  }

  export interface MessageCryptpadOnError {
    tag: MessageTag.CryptpadOnError;
    error: GenericError;
  }

  export interface MessageCryptpadOnNewKey {
    tag: MessageTag.CryptpadOnNewKey;
  }

  export interface MessageCryptpadOnInsertImage {
    tag: MessageTag.CryptpadOnInsertImage;
    messageId: number;
  }

  export interface MessageParsecOnInsertImageReply {
    tag: MessageTag.ParsecOnInsertImageReply;
    messageId: number;
    error: undefined | GenericError;
  }

  // Messages send by the Parsec client and received by the Cryptpad Iframe
  export type ParsecMessage =
    | MessageParsecOpenDocument
    | MessageParsecRequestSaveDocument
    | MessageParsecOnSaveReply
    | MessageParsecOnInsertImageReply;
  // Messages send by the Cryptpad Iframe and received by the Parsec client
  export type CryptpadMessage =
    | MessageCryptpadInitialized
    | MessageCryptpadOpenDocumentReply
    | MessageCryptpadRequestSaveDocumentReply
    | MessageCryptpadOnSave
    | MessageCryptpadOnHasUnsavedChanges
    | MessageCryptpadOnError
    | MessageCryptpadOnNewKey
    | MessageCryptpadOnInsertImage;
}

export import CryptpadEditor = ParsecCryptpadCommAPI.Editor;
export import CryptpadOpenMode = ParsecCryptpadCommAPI.OpenMode;
export type CryptpadOpenDocumentConfig = ParsecCryptpadCommAPI.OpenDocumentConfig;
export type CryptpadGenericError = GenericError;

export interface CryptpadEventHandlers {
  onSave: (file: Blob) => Promise<undefined | GenericError>;
  onError: (error: any) => void;
  onReady: () => void;
  onHasUnsavedChanges: (unsaved: boolean) => void;
}

// TODO: rethink this ?
export enum CryptpadErrorCode {
  NotAvailable = 'cryptpad-not-available',
  FrameNotLoaded = 'frame-not-loaded',
  FrameLoadFailed = 'frame-load-failed',
  OpenDocumentInvalidConfig = 'open-document-invalid-config',
  OpenDocumentFailed = 'open-document-failed',
  EventError = 'event-error',
}

// TODO: rethink this ?
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

// Track the messages we have sent and that produce a response.
namespace InFlightParsecMessage {
  type InFlightParsecMessageReply =
    | ParsecCryptpadCommAPI.MessageCryptpadOpenDocumentReply
    | ParsecCryptpadCommAPI.MessageCryptpadRequestSaveDocumentReply
  ;

  let lastMessageId = 0;
  const inFlightParsecMessages: Array<{ messageId: number; promiseWithResolvers: PromiseWithResolvers<InFlightParsecMessageReply> }> = [];

  export function registerParsecMessageForReply<T extends InFlightParsecMessageReply>(): [number, Promise<T>] {
    // Generate message ID
    lastMessageId += 1;
    const messageId = lastMessageId;

    const promiseWithResolvers: PromiseWithResolvers<InFlightParsecMessageReply> = Promise.withResolvers();
    inFlightParsecMessages.push({ messageId, promiseWithResolvers });

    return [messageId, promiseWithResolvers.promise as Promise<T>];
  }

  export function retrieveParsecMessageReplyPromise(messageId: number): PromiseWithResolvers<InFlightParsecMessageReply> | undefined {
    const index = inFlightParsecMessages.findIndex((x) => {
      return x.messageId === messageId;
    });
    if (index === -1) {
      return undefined;
    }
    return inFlightParsecMessages.splice(index, 1)[0].promiseWithResolvers;
  }
}

export interface CryptpadSession {
  controller: AbortController;
  save: () => Promise<undefined | GenericError>;
}

// Error handling is peculiar here: if something goes wrong this function
// returns `undefined` and `handlers.onError` is called with the actual
// error.
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

  function postMessageToIFrame(message: ParsecCryptpadCommAPI.ParsecMessage): void {
    console.log("========================!!! postMessageToIFrame", frame.id, frame.contentWindow, message);
    // `frame.contentWindow` is null until the `frame.src` has been set and the page
    // has finished to load.
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
              cryptpadLog('error', msg);
              reject(msg);
            }
            resolve();
          } else {
            cryptpadLog('error', `Unknown command: ${JSON.stringify(event.data)}`);
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
      if (event.origin !== CRYPTPAD_SERVER) {
        return;
      }

      switch (event.data.tag) {
        // TODO !!!!
        case ParsecCryptpadCommAPI.MessageTag.CryptpadOnError:
        case ParsecCryptpadCommAPI.MessageTag.CryptpadOnInsertImage:
        case ParsecCryptpadCommAPI.MessageTag.CryptpadOnNewKey: {
          console.log('WWWWWWIIIIIPPPPP: unsupported message', event.data);
          break;
        }
        case ParsecCryptpadCommAPI.MessageTag.CryptpadOnHasUnsavedChanges: {
          const data = event.data as ParsecCryptpadCommAPI.MessageCryptpadOnHasUnsavedChanges;
          console.log("************************** onHasUnsavedChanges", data.unsaved);
          handlers.onHasUnsavedChanges(data.unsaved)
          break;
        }
        case ParsecCryptpadCommAPI.MessageTag.CryptpadOnSave: {
          const data = event.data as ParsecCryptpadCommAPI.MessageCryptpadOnSave;
          (async () => {
            console.log("************************** onSave()");
            const maybeError = await handlers.onSave(data.documentContent);
            console.log("************************** onSave() ->", maybeError);

            postMessageToIFrame({
              tag: ParsecCryptpadCommAPI.MessageTag.ParsecOnSaveReply,
              messageId: data.messageId,
              error: maybeError,
            } as ParsecCryptpadCommAPI.MessageParsecOnSaveReply);
          })();
          break;
        }

        case ParsecCryptpadCommAPI.MessageTag.CryptpadOpenDocumentReply:
        case ParsecCryptpadCommAPI.MessageTag.CryptpadRequestSaveDocumentReply: {
          const data = event.data as
            | ParsecCryptpadCommAPI.MessageCryptpadOpenDocumentReply
            | ParsecCryptpadCommAPI.MessageCryptpadRequestSaveDocumentReply;
          const promiseWithResolvers = InFlightParsecMessage.retrieveParsecMessageReplyPromise(data.messageId);
          if (promiseWithResolvers !== undefined) {
            promiseWithResolvers.resolve(data);
          } else {
            cryptpadLog('error', `Message response with unknown messageId: ${JSON.stringify(event.data)}`);
          }
          break;
        }

        default: {
          cryptpadLog('error', `Unknown command: ${JSON.stringify(event.data)}`);
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
      //       case ParsecCryptpadCommAPI.Commands.InitReply: {
      //         if (event.data.success) {
      //           cryptpadLog('debug', 'Init success, opening the file...');
      //           postMessageToIFrame(ParsecCryptpadCommAPI.Commands.Open, options);
      //         } else {
      //           handlers.onError(new CryptpadError(CryptpadErrorCode.InitFailed, event.data.details));
      //         }
      //         break;
      //       }
      //       case ParsecCryptpadCommAPI.Commands.OpenReply: {
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

  const [messageId, replyPromise] = InFlightParsecMessage.registerParsecMessageForReply<ParsecCryptpadCommAPI.MessageCryptpadOpenDocumentReply>();
  postMessageToIFrame({
    tag: ParsecCryptpadCommAPI.MessageTag.ParsecOpenDocument,
    messageId,
    config,
  } as ParsecCryptpadCommAPI.MessageParsecOpenDocument);
  const reply = await replyPromise;

  if (reply !== undefined) {
    handlers.onError(reply.error);
    return undefined;
  }

  return {
    controller,
    save: async (): Promise<undefined | GenericError> => {
      cryptpadLog('debug', 'Triggering manual save');
      const [messageId, replyPromise] =
        InFlightParsecMessage.registerParsecMessageForReply<ParsecCryptpadCommAPI.MessageCryptpadRequestSaveDocumentReply>();
      postMessageToIFrame({
        tag: ParsecCryptpadCommAPI.MessageTag.ParsecRequestSaveDocument,
        messageId,
      });
      const reply = await replyPromise;
      // TODO: Throw exception ? Return a better typed thing ? Use `handlers.onError` ?
      return reply.error;
    },
  };
}

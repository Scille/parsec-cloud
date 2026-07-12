// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { detectOpenableFile, FileContentType } from '@/common/fileTypes';
import { Env } from '@/services/environment';

type GenericError = string;

// `Frame2CommAPI` is the API shared between the CryptPad integration Iframe and the parent
// Parsec client (i.e. us !).
//
// - See `resources/www/integration/main.js` in the custom Parsec-Cryptpad repository for the
//   part living in the Iframe. That Iframe talks directly to CryptPad's own editor modules
//   (no nested Iframe, no `cryptpad-api.js`). It replaces the older, broken `frame.html`/
//   `frame.js` integration, which is kept around unmodified for whatever else still depends
//   on it but is no longer used by this file.
// - Don't modify it without keeping in sync both parts!
// - When modifying it, don't forget to bump `Frame2CommAPI.version` so that
//   older Parsec client can detect they are no longer compatible.
namespace Frame2CommAPI {
  export const version = 1;

  export enum MessageTag {
    Frame2Initialized = 'frame2-initialized',
    OuterOpenDocument = 'outer-open-document',
    Frame2OpenDocumentReply = 'frame2-open-document-reply',
    OuterRequestSaveDocument = 'outer-request-save-document',
    Frame2RequestSaveDocumentReply = 'frame2-request-save-document-reply',
    Frame2OnSave = 'frame2-on-save',
    OuterOnSaveReply = 'outer-on-save-reply',
    Frame2OnHasUnsavedChanges = 'frame2-on-has-unsaved-changes',
    Frame2OnError = 'frame2-on-error',
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
    // Only required when `mode === Edit` and creating a brand-new document (i.e. no CryptPad
    // channel exists yet for `key`). Joining an existing collaborative session, or re-opening
    // in view mode, does not need it.
    documentContent?: Uint8Array;
    documentName: string;
    documentExtension: string;
    cryptpadEditor: Frame2CommAPI.Editor;
    // Base64 seed: an *edit* seed if mode === Edit, a *view* seed if mode === View.
    key: string;
    userName: string;
    userId: string;
    mode: Frame2CommAPI.OpenMode;
    locale: string;
  }

  // Sent by the Iframe once `integration/main.js` has finished loading
  export interface MessageFrame2Initialized {
    tag: MessageTag.Frame2Initialized;
    version: number;
    error: undefined | GenericError;
  }

  // Sent by us, once Iframe initialization is done, to open the file to edit/view
  export interface MessageOuterOpenDocument {
    tag: MessageTag.OuterOpenDocument;
    messageId: number;
    config: OpenDocumentConfig;
  }

  // Sent by the Iframe once the document is opened (i.e. CryptPad's inner app signaled it is ready)
  export interface MessageFrame2OpenDocumentReply {
    tag: MessageTag.Frame2OpenDocumentReply;
    messageId: number;
    error: undefined | GenericError;
  }

  // Sent by us to trigger a save. This is the *only* way a save happens: the Iframe always
  // disables CryptPad's own autosave, so nothing gets persisted unless we ask for it.
  export interface MessageOuterRequestSaveDocument {
    tag: MessageTag.OuterRequestSaveDocument;
    messageId: number;
  }

  // Sent by the Iframe once the save is done (or has failed if `error` field is not `undefined`)
  export interface MessageFrame2RequestSaveDocumentReply {
    tag: MessageTag.Frame2RequestSaveDocumentReply;
    messageId: number;
    error: undefined | GenericError;
  }

  // Sent by the Iframe to actually save the document.
  export interface MessageFrame2OnSave {
    tag: MessageTag.Frame2OnSave;
    messageId: number;
    documentContent: Blob;
  }

  // Sent by us once the document has been actually saved.
  export interface MessageOuterOnSaveReply {
    tag: MessageTag.OuterOnSaveReply;
    messageId: number;
    error: undefined | GenericError;
  }

  export interface MessageFrame2OnHasUnsavedChanges {
    tag: MessageTag.Frame2OnHasUnsavedChanges;
    unsavedChanges: boolean;
  }

  export interface MessageFrame2OnError {
    tag: MessageTag.Frame2OnError;
    error: GenericError;
  }

  // Messages sent by the Parsec client and received by the Iframe
  export type OuterMessage = MessageOuterOpenDocument | MessageOuterRequestSaveDocument | MessageOuterOnSaveReply;
  // Messages sent by the Iframe and received by the Parsec client
  export type Frame2Message =
    | MessageFrame2Initialized
    | MessageFrame2OpenDocumentReply
    | MessageFrame2RequestSaveDocumentReply
    | MessageFrame2OnSave
    | MessageFrame2OnHasUnsavedChanges
    | MessageFrame2OnError;
}

export import CryptpadEditor = Frame2CommAPI.Editor;
export import CryptpadOpenMode = Frame2CommAPI.OpenMode;
export type CryptpadOpenDocumentConfig = Frame2CommAPI.OpenDocumentConfig;
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

// TODO: remove once the Parsec server exposes real session/key management (see module TODOs in
// `FileEditor.vue`): CryptPad's `key` is not a free-form identifier, it must decode into exactly
// 18 raw bytes (a `chainpad-crypto` edit/view seed) or CryptPad rejects it with "The channel key
// and/or the encryption key is invalid". In the absence of a real per-session key handed out by
// the server, deterministically derive a valid one from the file id so that every collaborator
// opening the same file still lands on the same CryptPad channel.
export async function derivePocSessionKey(fileId: string): Promise<string> {
  const digest = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(fileId));
  const seed = new Uint8Array(digest).slice(0, 18);
  return btoa(String.fromCharCode(...seed));
}

// TODO: remove alongside `derivePocSessionKey` once real per-session view/edit keys come from the
// Parsec server. A CryptPad *view* key is not interchangeable with the *edit* key it is used
// alongside: `chainpad-crypto`'s `createEditCryptor2` deterministically derives it as
// `SHA-512(editSeed)[32:64]` (see `components/chainpad-crypto/crypto.js`). Passing the edit key
// as-is for a view-mode session (as this POC used to) makes CryptPad derive an entirely
// different, nonexistent channel: a read-only collaborator then just times out, since from that
// channel's point of view nobody has ever edited anything.
export async function derivePocViewOnlyKey(editKey: string): Promise<string> {
  const editSeed = Uint8Array.from(atob(editKey), (c) => c.charCodeAt(0));
  const digest = await crypto.subtle.digest('SHA-512', editSeed);
  const viewSeed = new Uint8Array(digest).slice(32, 64);
  return btoa(String.fromCharCode(...viewSeed));
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
namespace InFlightOuterMessage {
  type InFlightOuterMessageReply = Frame2CommAPI.MessageFrame2OpenDocumentReply | Frame2CommAPI.MessageFrame2RequestSaveDocumentReply;

  let lastMessageId = 0;
  const inFlightMessages: Array<{ messageId: number; promiseWithResolvers: PromiseWithResolvers<InFlightOuterMessageReply> }> = [];

  export function registerMessageForReply<T extends InFlightOuterMessageReply>(): [number, Promise<T>] {
    // Generate message ID
    lastMessageId += 1;
    const messageId = lastMessageId;

    const promiseWithResolvers: PromiseWithResolvers<InFlightOuterMessageReply> = Promise.withResolvers();
    inFlightMessages.push({ messageId, promiseWithResolvers });

    return [messageId, promiseWithResolvers.promise as Promise<T>];
  }

  export function retrieveMessageReplyPromise(messageId: number): PromiseWithResolvers<InFlightOuterMessageReply> | undefined {
    const index = inFlightMessages.findIndex((x) => {
      return x.messageId === messageId;
    });
    if (index === -1) {
      return undefined;
    }
    return inFlightMessages.splice(index, 1)[0].promiseWithResolvers;
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

  cryptpadLog('debug', `Trying to open document on server '${CRYPTPAD_SERVER}'`);

  function postMessageToIFrame(message: Frame2CommAPI.OuterMessage): void {
    // `frame.contentWindow` is null until the `frame.src` has been set and the page
    // has finished to load.
    if (!frame.contentWindow) {
      throw new CryptpadError(CryptpadErrorCode.FrameNotLoaded);
    }
    frame.contentWindow.postMessage(message, CRYPTPAD_SERVER as string);
  }

  // 1) Initialize the Iframe
  //
  // This is done in 3 times:
  // - Configure the `integration/index.html` entrypoint
  // - Wait for the initialized message from the Iframe (i.e. CryptPad is ready)
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

          if (event.data.tag === Frame2CommAPI.MessageTag.Frame2Initialized) {
            const message = event.data as Frame2CommAPI.MessageFrame2Initialized;
            clearTimeout(timeoutId);
            abortEventListening.abort();
            // API version check must always be done first!
            if (message.version !== Frame2CommAPI.version) {
              // eslint-disable-next-line max-len
              const msg = `Incompatible API between Parsec client (version: ${Frame2CommAPI.version}) and Cryptpad Iframe (version: ${message.version})`;
              cryptpadLog('warn', msg);
              reject(msg);
              return;
            }
            if (message.error !== undefined) {
              const msg = `Cryptpad Iframe has failed to initialize: ${message.error}`;
              cryptpadLog('error', msg);
              reject(msg);
              return;
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
      frame.src = `${CRYPTPAD_SERVER}/integration/?origin=${encodeURIComponent(window.location.origin)}&frame2_comm_api_version=${Frame2CommAPI.version}`;
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
        case Frame2CommAPI.MessageTag.Frame2OnError: {
          const data = event.data as Frame2CommAPI.MessageFrame2OnError;
          handlers.onError(new CryptpadError(CryptpadErrorCode.EventError, data.error));
          break;
        }

        case Frame2CommAPI.MessageTag.Frame2OnHasUnsavedChanges: {
          const data = event.data as Frame2CommAPI.MessageFrame2OnHasUnsavedChanges;
          handlers.onHasUnsavedChanges(data.unsavedChanges);
          break;
        }

        case Frame2CommAPI.MessageTag.Frame2OnSave: {
          const data = event.data as Frame2CommAPI.MessageFrame2OnSave;
          (async () => {
            const maybeError = await handlers.onSave(data.documentContent);

            postMessageToIFrame({
              tag: Frame2CommAPI.MessageTag.OuterOnSaveReply,
              messageId: data.messageId,
              error: maybeError,
            } as Frame2CommAPI.MessageOuterOnSaveReply);
          })();
          break;
        }

        case Frame2CommAPI.MessageTag.Frame2OpenDocumentReply:
        case Frame2CommAPI.MessageTag.Frame2RequestSaveDocumentReply: {
          const data = event.data as Frame2CommAPI.MessageFrame2OpenDocumentReply | Frame2CommAPI.MessageFrame2RequestSaveDocumentReply;
          const promiseWithResolvers = InFlightOuterMessage.retrieveMessageReplyPromise(data.messageId);
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
    },
    { signal: controller.signal },
  );

  // 3) Finally open the document

  const [messageId, replyPromise] = InFlightOuterMessage.registerMessageForReply<Frame2CommAPI.MessageFrame2OpenDocumentReply>();
  postMessageToIFrame({
    tag: Frame2CommAPI.MessageTag.OuterOpenDocument,
    messageId,
    config,
  } as Frame2CommAPI.MessageOuterOpenDocument);
  const reply = await replyPromise;

  if (reply.error !== undefined) {
    controller.abort();
    handlers.onError(new CryptpadError(CryptpadErrorCode.OpenDocumentFailed, reply.error));
    return undefined;
  }

  handlers.onReady();

  return {
    controller,
    save: async (): Promise<undefined | GenericError> => {
      cryptpadLog('debug', 'Triggering manual save');
      const [messageId, replyPromise] = InFlightOuterMessage.registerMessageForReply<Frame2CommAPI.MessageFrame2RequestSaveDocumentReply>();
      postMessageToIFrame({
        tag: Frame2CommAPI.MessageTag.OuterRequestSaveDocument,
        messageId,
      } as Frame2CommAPI.MessageOuterRequestSaveDocument);
      const reply = await replyPromise;
      // TODO: Throw exception ? Return a better typed thing ? Use `handlers.onError` ?
      return reply.error;
    },
  };
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { detectOpenableFile, FileContentType } from '@/common/fileTypes';
import { Env } from '@/services/environment';
import { convertToNativeFormat } from '@/services/x2t';

// POC: replaces the (broken) Cryptpad integration with a direct integration of OnlyOffice,
// using CryptPad's own end-to-end-encrypted fork of OnlyOffice (client-side code only, vendored
// in `client/vendors/onlyoffice`, see the README there).
//
// There is no collaborative server yet: `client/editics/index.html` wires the editor to
// CryptPad's `connectMockServer` API (their replacement for OnlyOffice's usual socket.io-based
// server communication) with handlers that keep everything local to the current tab. Nothing is
// ever sent back to Parsec: the caller is expected to read the file content ahead of time and pass
// it in, and no "save" mechanism is implemented on top of this module yet.

// Should be the same on both sides (this file and editics/index.html), don't modify one
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
    // File extension of `documentContent` as stored in Parsec, e.g. `docx` or `bin`. Anything other
    // than `bin` (OnlyOffice's own native format) is converted with x2t before being handed to the
    // editor, see services/x2t.ts.
    documentExtension: string;
    documentType: OnlyOfficeCommAPI.DocumentTypes;
    // Stable identifier of the document being edited (e.g. `${workspaceId}:${path}`), used only to
    // key the mock collaboration session in onlyoffice-mock-server.js (see step 3.2 in CLAUDE.md) so
    // that two tabs/users opening the same file join the same simulated session. Not to be confused
    // with `key` below, which is OnlyOffice's own per-editing-session key.
    documentId: string;
    key: string;
    userName: string;
    userId: string;
    mode: OnlyOfficeCommAPI.OpenModes;
    locale: string;
    theme?: 'light' | 'dark';
    // Editics collaborative session config (RFC 1030, step 0). When present, the
    // host page connects to the Parsec server's SSE + RPC editics routes
    // (see client/editics/main.js) instead of the
    // localStorage/BroadcastChannel-only mock server. `deviceId` is the
    // hyphenated UUID form of the client's DeviceID (matches the server's
    // `DeviceID.from_hex`); `workspaceId`/`vlobId` are the VlobID UUID strings.
    editics?: {
      // Origin + path prefix up to (but not including) the organization id,
      // e.g. `http://parsec.invalid`.
      baseUrl: string;
      organizationId: string;
      workspaceId: string;
      vlobId: string;
      // DeviceID hyphenated UUID string.
      deviceId: string;
      // The vlob version the client has loaded locally (RFC §1.2).
      vlobVersion: number;
      // 0=Word, 1=Spreadsheet, 2=Presentation, 3=Visio.
      editorType: number;
      // Resolve a DeviceID hex to a display name (libparsec lookup). The server
      // is not trusted for names; the client keeps its own table.
      resolveUserName: (deviceId: string) => Promise<string | undefined>;
      // Resolve a DeviceID hex to the user's `userId` (person id), to build
      // OnlyOffice's composite `<userId><indexUser>` participant id.
      resolveUserId: (deviceId: string) => Promise<string | undefined>;
    };
  }
}

export import OnlyOfficeDocumentType = OnlyOfficeCommAPI.DocumentTypes;
export import OnlyOfficeOpenModes = OnlyOfficeCommAPI.OpenModes;
export type OpenDocumentOptions = OnlyOfficeCommAPI.OpenDocumentOptions;

export enum OnlyOfficeErrorCodes {
  FrameNotLoaded = 'frame-not-loaded',
  FrameLoadFailed = 'frame-load-failed',
  EventError = 'event-error',
  ConversionFailed = 'conversion-failed',
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

const HOST_PAGE = `${import.meta.env.BASE_URL}editics/index.html`;

// OnlyOffice's own native format (see client/vendors/onlyoffice/README.md): anything else needs to
// go through x2t first, see prepareDocumentContent() below.
const NATIVE_EXTENSION = 'bin';

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
async function loadBlankTemplate(documentType: OnlyOfficeDocumentType, documentContent: Uint8Array): Promise<Uint8Array> {
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

// Gets `options.documentContent` into a shape the editor can open directly: OnlyOffice's own
// native format, either as-is, converted from another office format with x2t, or a blank template
// when there's nothing to open/convert (e.g. an empty file freshly created outside the editor).
async function prepareDocumentContent(options: OpenDocumentOptions): Promise<Uint8Array> {
  const { documentContent, documentExtension, documentType } = options;

  if (documentContent.byteLength === 0) {
    return loadBlankTemplate(documentType, documentContent);
  }
  if (documentExtension === NATIVE_EXTENSION) {
    return documentContent;
  }
  return convertToNativeFormat(documentContent, options.documentName, documentExtension);
}

export async function openDocument(
  options: OpenDocumentOptions,
  handlers: OnlyOfficeEventHandlers,
  frame: HTMLIFrameElement,
): Promise<OnlyOfficeSession | undefined> {
  const controller = new AbortController();

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
            case 'oo-resolve-user-name': {
              // The editics client in the host iframe asks for a user name given
              // a DeviceID hex (the server is not trusted for names, RFC §3.3).
              // Reply on the MessagePort it provided, if any.
              const port = (event as MessageEvent).ports[0];
              if (port && options.editics?.resolveUserName) {
                options.editics.resolveUserName(event.data.deviceId).then((userName) => {
                  port.postMessage({ userName });
                });
              } else if (port) {
                port.postMessage({ userName: undefined });
              }
              break;
            }
            case 'oo-resolve-user-id': {
              // The editics client asks for the user's `userId` (person id) given
              // a DeviceID hex, to build OnlyOffice's composite participant id
              // (see editics/index.html's resolveUserId).
              const port = (event as MessageEvent).ports[0];
              if (port && options.editics?.resolveUserId) {
                options.editics.resolveUserId(event.data.deviceId).then((userId) => {
                  port.postMessage({ userId });
                });
              } else if (port) {
                port.postMessage({ userId: undefined });
              }
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

  onlyOfficeLog('debug', 'Host frame is ready, preparing the document');

  if (!frame.contentWindow) {
    controller.abort();
    handlers.onError(new OnlyOfficeError(OnlyOfficeErrorCodes.FrameNotLoaded));
    return undefined;
  }

  let documentContent: Uint8Array;
  try {
    documentContent = await prepareDocumentContent(options);
  } catch (e: unknown) {
    controller.abort();
    handlers.onError(e instanceof OnlyOfficeError ? e : new OnlyOfficeError(OnlyOfficeErrorCodes.ConversionFailed, String(e)));
    return undefined;
  }

  // `resolveUserName` is a function and cannot survive structured-clone across
  // the iframe boundary; the host page bridges it back to us via the
  // `oo-resolve-user-name` message above. Strip it before posting.
  let postOptions: OpenDocumentOptions = options;
  if (options.editics) {
    // `resolveUserName` and `resolveUserId` are functions and cannot survive
    // structured-clone across the iframe boundary; the host page bridges them
    // back to us via the `oo-resolve-user-name` / `oo-resolve-user-id` messages
    // above. Strip them before posting.
    const { resolveUserName: _omitted, resolveUserId: _omitted2, ...editicsSerializable } = options.editics;
    postOptions = { ...options, editics: editicsSerializable } as OpenDocumentOptions;
  }
  frame.contentWindow.postMessage(
    {
      command: OnlyOfficeCommAPI.Commands.Open,
      options: { ...postOptions, documentContent },
    },
    '*',
  );

  return { controller };
}

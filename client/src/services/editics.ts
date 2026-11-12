// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import { FileContentType } from '@/common/fileTypes';
import ImageSelectionModal from '@/components/files/explorer/ImageSelectionModal.vue';
import {
  DeviceID,
  editicsDecryptBatch,
  editicsEncryptBatch,
  EntryID,
  entryStat,
  EntryStatFile,
  FsPath,
  generateEditicsToken,
  getClientInfo,
  getUserInfoFromDeviceID,
  getWorkspaceInfo,
  isElectron,
  isWeb,
  parseParsecAddr,
  WorkspaceHandle,
} from '@/parsec';
import { getWorkspaceHandle } from '@/router';
import { Env } from '@/services/environment';
import type {
  EditicsCollaborativeSession,
  EditicsDocumentTypes,
  EditicsHostToParentMessage,
  EditicsOpenOptions,
  EditicsParentToHostMessage,
  EditicsRequestParentSaveReply,
} from '@editics_parent_host_api';
import { modalController } from '@ionic/vue';
import { MsModalResult } from 'megashark-lib';

async function getEditicsOrigin(): Promise<string> {
  // Page is served on a different origin for security reasons.
  // Use this to filter the message we get.
  if (isElectron()) {
    // Using a custom scheme for Electron as it does count as a different origin
    // and provides a good isolation automatically.
    return 'parsec-editics://-';
  } else if (window.isDev()) {
    // Web dev mode

    if (window.location.host.startsWith('127.0.0.1')) {
      // `editics.127.0.0.1` is not a valid host, must force the use of localhost instead!
      return `${window.location.protocol}//editics.${window.location.host.replace('127.0.0.1', 'localhost')}`;
    } else {
      return `${window.location.protocol}//editics.${window.location.host}`;
    }
  } else {
    // Web release mode

    // Just add a prefix (e.g. `app.parsec.cloud` -> `editics.app.parsec.cloud`).
    // This would fail if the host is an IP address, but this is highly unlikely
    // in release since a website is expected to always be accessed by its hostname.
    // Note the web app is then expected to be served by the Parsec server itself
    // (see `server/parsec/asgi/__init__.py`), so the editics origin is also where
    // the collaborative session lives (see `getEditicsSessionServerUrl`).
    return `${window.location.protocol}//editics.${window.location.host}`;
  }
}

// The edition mode determines which host page is served in the iframe (see
// `editics/README.md`): the parent picks it depending on whether a
// collaborative session was resolved for the document (see
// `getEditicsCollaborativeSession`).
export type EditicsEditionMode = 'offline' | 'collaborative';

// In release builds, `scripts/vite_plugin_editics.ts` emits each host page with
// a content hash for cache busting (e.g. `editics/offline-1234abcd.html`), and
// advertises the actual name through a dedicated meta tag injected in the main
// app's `index.html` (i.e. `editics-offline-page` & `editics-collaborative-page`).
//
// In dev mode, the Vite dev server serves the pages un-hashed, hence the
// fallbacks (the meta tags are only injected in release builds).
const EDITICS_HOST_PAGES: Record<EditicsEditionMode, { metaName: string; devPage: string }> = {
  offline: { metaName: 'editics-offline-page', devPage: 'editics/offline.html' },
  collaborative: { metaName: 'editics-collaborative-page', devPage: 'editics/collaborative.html' },
};

function getEditicsHostPage(mode: EditicsEditionMode): string {
  const { metaName, devPage } = EDITICS_HOST_PAGES[mode];
  const content = document.querySelector<HTMLMetaElement>(`meta[name="${metaName}"]`)?.content;
  return !!content ? content : devPage;
}

function getEditicsFrameUrl(editicsOrigin: string, mode: EditicsEditionMode): URL {
  const BASE_URL = import.meta.env.BASE_URL.endsWith('/') ? import.meta.env.BASE_URL : `${import.meta.env.BASE_URL}/`;
  const url = new URL(`${BASE_URL}${getEditicsHostPage(mode)}`, editicsOrigin);
  // Passing our origin to the frame, so it knows how to contact us and check for us.
  // In web, this URL will really be used to fetch a page from the server; but
  // those parameters are only used within our client, so pass them after the
  // URL `#` to not send them to the server.
  url.hash = new URLSearchParams({ parentOrigin: window.location.origin }).toString();
  return url;
}

export enum EditicsErrorCodes {
  FrameNotLoaded = 'frame-not-loaded',
  FrameLoadFailed = 'frame-load-failed',
  EventError = 'event-error',
  SaveFailed = 'save-failed',
  SaveTimeout = 'save-timeout',
  ConversionFailed = 'conversion-failed',
  ConversionTimeout = 'conversion-timeout',
}

export class EditicsError extends Error {
  public code: EditicsErrorCodes;
  public details?: string;

  constructor(code: EditicsErrorCodes, details?: string) {
    super(`OnlyOffice error: ${code}`);
    this.code = code;
    this.details = details;
    Object.setPrototypeOf(this, new.target.prototype);
  }
}

// Progress of the document's save, surfaced by the editor component (mapped
// onto the save indicator in the file handler topbar).
export enum EditicsSaveState {
  Unsaved = 'unsaved',
  Saving = 'saving',
  Saved = 'saved',
  Error = 'error',
}

interface EditicsEventHandlers {
  onReady: () => void;
  onError: (error: any) => void;
  // Persists document content converted by the offline host back to the
  // workspace file's original format. Throwing/rejecting fails the save.
  onSave: (data: Uint8Array) => Promise<void>;
  // Optional notifications of the editor's save progress (see
  // EditicsSaveState), for e.g. the topbar save indicator.
  onSaveStateChange?: (state: EditicsSaveState) => void;
}

export interface EditicsHostSession {
  controller: AbortController;
  // Asks the editor to save now and resolves once the document has been
  // persisted (or `false` if it could not be). `nothingToSave` resolves `true`
  // too: there was simply nothing modified to write back.
  save: () => Promise<boolean>;
  // Applies a new theme to the editor while it is open.
  setTheme: (theme: EditicsOpenOptions['theme']) => void;
  // Asks the editor to export the document's current state (saved or not) to
  // PDF and resolves with the PDF file content. Rejects with an `EditicsError`
  // (`ConversionFailed` or `ConversionTimeout`) if it could not be done.
  convertToPdf: () => Promise<Uint8Array>;
}

// Time we give the host page to signal it is ready after its iframe is loaded.
// Short in tests to fail fast.
const HOST_READY_TIMEOUT_MS = (window as any).TESTING === true ? 500 : 5000;

// Time we give the host page to complete a save round-trip: it serializes the
// document, converts it back to its original office format with x2t, and asks
// us to write it to the workspace before running the editor's save handshake.
const SAVE_TIMEOUT_MS = 30000;

// Time we give the host page to export the document to PDF.
const PDF_CONVERSION_TIMEOUT_MS = 20000;

function onlyOfficeLog(level: 'debug' | 'warn' | 'error' | 'info', message: string): void {
  window.nativeAPI.log(level, `[OnlyOffice] ${message}`);
}

export function getEditicsDocumentType(contentType: FileContentType): EditicsDocumentTypes | undefined {
  switch (contentType) {
    case FileContentType.Document:
      return 'word';
    case FileContentType.Spreadsheet:
      return 'cell';
    case FileContentType.Presentation:
      return 'slide';
    // Note Editics also have `pdf`, but we don't use it for now
    default:
      return undefined;
  }
}

export function isEditicsEnabledForDocumentType(contentType: FileContentType): boolean {
  if (!Env.isEditicsEnabled()) {
    return false;
  }
  return getEditicsDocumentType(contentType) !== undefined;
}

export async function openDocument(
  options: EditicsOpenOptions,
  documentContent: Uint8Array,
  handlers: EditicsEventHandlers,
  frame: HTMLIFrameElement,
): Promise<EditicsHostSession | undefined> {
  const controller = new AbortController();

  // Save requests are asynchronous: the host converts the data, we perform
  // the workspace write, then reply on the port it provided. The pending reply
  // for a manual save request is tracked here so
  // `session.save()` below can await it.
  let pendingSaveResult: ((success: boolean, error?: string) => void) | undefined = undefined;
  let saving = false;

  // Pending PDF conversions, waiting for the host's `oo-convert-to-pdf-result`.
  // A request is removed on completion or timeout, so a late reply is ignored.
  const pendingPdfConversions = new Map<number, (reply: { data?: Uint8Array; error?: string }) => void>();
  let pdfRequestCounter = 0;

  const finishSave = (port: MessagePort | undefined, success: boolean, error?: string): void => {
    port?.postMessage({ success, error } satisfies EditicsRequestParentSaveReply);
    if (saving) {
      saving = false;
      handlers.onSaveStateChange?.(success ? EditicsSaveState.Saved : EditicsSaveState.Error);
    }
  };

  let session: EditicsHostSession | undefined = undefined;

  let frameUrl: URL;
  try {
    // The host page depends on the edition mode: `collaborative.html` joins
    // the editics session hosted by the Parsec server, while `offline.html`
    // runs a single-user local edition (see `editics/README.md`).
    const mode: EditicsEditionMode = options.collaborative !== undefined ? 'collaborative' : 'offline';
    frameUrl = getEditicsFrameUrl(await getEditicsOrigin(), mode);
  } catch (e: unknown) {
    handlers.onError(new EditicsError(EditicsErrorCodes.FrameLoadFailed, String(e)));
    return undefined;
  }
  const hostOrigin = frameUrl.origin;

  // In Electron the editics pages are served by the main process, which also
  // stamps their CSP: declare the origin of the server hosting the session so
  // it is allowed in `connect-src` (see `cspRules.ts`). This is done lazily
  // (i.e. right before an editics page is actually needed) so no bookkeeping
  // of the logged-in devices' servers is required; the main process takes
  // care of the deduplication.
  if (options.collaborative !== undefined && isElectron()) {
    await window.nativeAPI.declareEditicsServerOrigin(options.collaborative.serverUrl);
  }

  try {
    await new Promise<void>((resolve, reject) => {
      const hostReadyTimeoutId = setTimeout(() => {
        reject('timeout');
      }, HOST_READY_TIMEOUT_MS);

      window.addEventListener(
        'message',
        (event: MessageEvent<EditicsHostToParentMessage>): void => {
          // Defense in depth: only accept messages from our host frame, and
          // only from the origin it is expected to be served from.
          if (event.source !== frame.contentWindow || event.origin !== hostOrigin) {
            return;
          }

          switch (event.data?.command) {
            case 'oo-host-ready': {
              clearTimeout(hostReadyTimeoutId);
              // The host page asks for the document: send `oo-open` every time
              // it signals readiness (a reloaded host page restarts from
              // scratch and needs the document again). Also reset the save
              // bookkeeping: the reloaded host page
              // starts with a clean save state (the reload only ever happens
              // before any change exists, so there is no pending save to
              // preserve).
              saving = false;
              pendingSaveResult = undefined;
              frame.contentWindow?.postMessage(
                { command: 'oo-open', options, documentContent } satisfies EditicsParentToHostMessage,
                hostOrigin,
              );
              resolve();
              break;
            }

            case 'oo-app-ready': {
              onlyOfficeLog('debug', 'OnlyOffice app is ready');
              break;
            }

            case 'oo-ready': {
              onlyOfficeLog('debug', 'Document is ready');
              handlers.onReady();
              break;
            }

            case 'oo-error': {
              handlers.onError(new EditicsError(EditicsErrorCodes.EventError, event.data.details));
              break;
            }

            case 'oo-save': {
              // The host converted the editor's native serialization back to the
              // file's original office format and asks us to persist it. We answer
              // on the transferred port once the workspace write completes; only
              // this side has libparsec access.
              const port = event.ports[0];
              if (!port || !handlers.onSave) {
                port?.postMessage({ success: false, error: 'no save handler' } satisfies EditicsRequestParentSaveReply);
                return;
              }
              saving = true;
              handlers.onSaveStateChange?.(EditicsSaveState.Saving);
              handlers
                .onSave(event.data.data)
                .then(() => finishSave(port, true))
                .catch((e: unknown) => {
                  onlyOfficeLog('error', `Failed to save the document: ${String(e)}`);
                  finishSave(port, false, String(e));
                });
              break;
            }

            case 'oo-save-state': {
              // The editor's dirty state changed ('unsaved' when the document has
              // modifications, 'saved' when it is clean again).
              if (!saving) {
                const state = event.data.state === 'unsaved' ? EditicsSaveState.Unsaved : EditicsSaveState.Saved;
                handlers.onSaveStateChange?.(state);
              }
              break;
            }

            case 'oo-save-result': {
              // Completion of a `session.save()` request initiated by this side.
              const resolve = pendingSaveResult;
              pendingSaveResult = undefined;
              if (resolve) {
                resolve(event.data.success === true, event.data.error);
              } else {
                onlyOfficeLog('warn', `Unexpected save result (success: ${event.data.success})`);
              }
              break;
            }

            case 'oo-insert-image': {
              // The editor's user wants to insert an image: present a workspace
              // file picker and send its content back to the host.
              if (!frame.contentWindow) {
                return;
              }
              handleRequestImage(frame.contentWindow, event.data.requestId);
              break;
            }

            case 'oo-convert-to-pdf-result': {
              // When we send a request to convert a document to PDF to the frame,
              // we add an id and store that id with a function (resolver) that will
              // be called with the result.
              // Since we request a conversion and we get the result asynchronously,
              // this is done to match a request with the result (otherwise two quasi simultaneous
              // requests resolved at different speeds could end up mixing up their results).
              const resolver = pendingPdfConversions.get(event.data.requestId);
              if (resolver) {
                pendingPdfConversions.delete(event.data.requestId);
                resolver(event.data);
              } else {
                onlyOfficeLog('warn', `Ignoring PDF conversion result for unknown request ${event.data.requestId}`);
              }
              break;
            }

            case 'oo-get-user-name': {
              // Collaborative mode: the host needs the display name of a
              // session participant. The editics protocol deliberately
              // doesn't carry user names (the server is not trusted for
              // them), so the host asks the only side with access to
              // libparsec to resolve the participant's device ID.
              if (!frame.contentWindow) {
                return;
              }
              handleUserNameRequest(frame.contentWindow, event.data.requestId, event.data.deviceIdHex, hostOrigin);
              break;
            }

            case 'oo-encrypt-batch': {
              // Collaborative mode: the host wants to send editics payloads
              // (session event payloads such as change fragments, cursors,
              // messages, or the original content parts of the document) to
              // the editics session: they are sealed with the workspace's
              // latest realm key here (the only side with access to libparsec)
              // before being sent, so the server only ever sees opaque
              // ciphertexts.
              if (!frame.contentWindow) {
                return;
              }
              handleEncryptBatchRequest(
                frame.contentWindow,
                event.data.requestId,
                event.data.flat,
                event.data.offsets,
                hostOrigin,
                options,
              );
              break;
            }

            case 'oo-decrypt-batch': {
              // Collaborative mode: the host received editics payloads from
              // the editics session and asks the parent to open them (the
              // counterpart of the `oo-encrypt-batch` handling).
              if (!frame.contentWindow) {
                return;
              }
              handleDecryptBatchRequest(
                frame.contentWindow,
                event.data.requestId,
                event.data.keyIndex,
                event.data.flat,
                event.data.offsets,
                hostOrigin,
                options,
              );
              break;
            }
          }
        },
        { signal: controller.signal },
      );
      window.nativeAPI.log('debug', `Loading frame at ${frameUrl}`);
      frame.src = frameUrl.href;
    });
  } catch (e: unknown) {
    controller.abort();
    handlers.onError(new EditicsError(EditicsErrorCodes.FrameLoadFailed, String(e)));
    return undefined;
  }

  onlyOfficeLog('debug', 'Host frame is ready, preparing the document');

  if (!frame.contentWindow) {
    controller.abort();
    handlers.onError(new EditicsError(EditicsErrorCodes.FrameNotLoaded));
    return undefined;
  }

  session = {
    controller,
    save: (): Promise<boolean> => {
      if (!frame.contentWindow) {
        return Promise.resolve(false);
      }
      return new Promise<boolean>((resolve) => {
        let settled = false;
        const timer = setTimeout(() => {
          if (!settled) {
            settled = true;
            pendingSaveResult = undefined;
            onlyOfficeLog('error', 'Save request timed out');
            handlers.onSaveStateChange?.(EditicsSaveState.Error);
            resolve(false);
          }
        }, SAVE_TIMEOUT_MS);
        pendingSaveResult = (success: boolean, _error?: string): void => {
          if (settled) {
            return;
          }
          settled = true;
          clearTimeout(timer);
          resolve(success);
        };
        frame.contentWindow!.postMessage({ command: 'oo-save-request' } satisfies EditicsParentToHostMessage, hostOrigin);
      });
    },
    setTheme: (theme: EditicsOpenOptions['theme']): void => {
      frame.contentWindow?.postMessage({ command: 'oo-set-theme', theme } satisfies EditicsParentToHostMessage, hostOrigin);
    },
    convertToPdf: (): Promise<Uint8Array> => {
      if (!frame.contentWindow) {
        return Promise.reject(new EditicsError(EditicsErrorCodes.FrameNotLoaded));
      }
      return new Promise<Uint8Array>((resolve, reject) => {
        const requestId = ++pdfRequestCounter;
        const timer = setTimeout(() => {
          pendingPdfConversions.delete(requestId);
          onlyOfficeLog('error', `PDF conversion request ${requestId} timed out`);
          reject(new EditicsError(EditicsErrorCodes.ConversionTimeout));
        }, PDF_CONVERSION_TIMEOUT_MS);
        pendingPdfConversions.set(requestId, (reply): void => {
          clearTimeout(timer);
          // `data` comes from the frame's realm, `instanceof Uint8Array` would be false
          if (reply.data !== undefined && ArrayBuffer.isView(reply.data)) {
            resolve(reply.data);
          } else {
            const details = reply.error ?? 'no data';
            onlyOfficeLog('error', `PDF conversion request ${requestId} failed: ${details}`);
            reject(new EditicsError(EditicsErrorCodes.ConversionFailed, details));
          }
        });
        frame.contentWindow!.postMessage({ command: 'oo-convert-to-pdf', requestId } satisfies EditicsParentToHostMessage, hostOrigin);
      });
    },
  };

  // Note the document is *not* posted here: the host page asks for it with
  // `oo-host-ready` (see its handling above), and may ask again if it gets
  // reloaded (collaborative mode).

  return session;
}

// Resolves the display name of a collaborative session participant and
// replies to the host's `oo-get-user-name` request. An unknown device ID (or
// a failure to resolve it) makes the host fall back on a placeholder name,
// see `collaborative.ts`.
async function handleUserNameRequest(host: Window, requestId: number, deviceIdHex: string, hostOrigin: string): Promise<void> {
  let userName: string | undefined = undefined;
  // `DeviceID`s are UUIDs, whose hex form is exactly the string
  // representation expected by libparsec.
  const result = await getUserInfoFromDeviceID(deviceIdHex as DeviceID);
  if (result.ok) {
    userName = result.value.humanHandle.label;
  } else {
    onlyOfficeLog('warn', `Failed to resolve the user name of the device ${deviceIdHex}`);
  }
  host.postMessage({ command: 'oo-get-user-name-result', requestId, userName } satisfies EditicsParentToHostMessage, hostOrigin);
}

// The session payload buffers are freshly allocated (by libparsec on this
// side, by `TextEncoder.encode`/`Uint8Array.fromBase64` on the host side) and
// not used anymore once the reply is posted, so they can be transferred back
// instead of being copied by the structured clone. The ownership check keeps
// us safe should a future caller pass a view on a shared buffer (transferring
// it would detach the other views' data); empty buffers are left to the
// structured clone, which copies nothing for them.
function transferBuffer(data: Uint8Array): Transferable[] {
  return data.byteOffset === 0 && data.byteLength > 0 && data.byteLength === data.buffer.byteLength && data.buffer instanceof ArrayBuffer
    ? [data.buffer]
    : [];
}

// Seals the editics payloads (session event payloads such as change
// fragments, cursors, messages, or the original content parts of the
// document) with the workspace's latest realm key, so the host can send them
// to the collaborative editics session: only this side has access to
// libparsec, and the server only ever sees the opaque ciphertexts. The
// payloads arrive as a single flat buffer (the i-th payload is
// `flat[offsets[i]..offsets[i+1]]`) which is passed to libparsec as-is, and
// the ciphertexts travel back the same way, along the single key index they
// are all sealed with (the session participants need it to resolve the key
// on their side).
async function handleEncryptBatchRequest(
  host: Window,
  requestId: number,
  flat: Uint8Array,
  offsets: number[],
  hostOrigin: string,
  options: EditicsOpenOptions,
): Promise<void> {
  type EncryptReply = Extract<EditicsParentToHostMessage, { command: 'oo-encrypt-batch-result' }>;
  const postReply = (payload: Omit<EncryptReply, 'command' | 'requestId'>, transfer: Transferable[] = []): void => {
    host.postMessage(
      { command: 'oo-encrypt-batch-result', requestId, ...payload } satisfies EditicsParentToHostMessage,
      hostOrigin,
      transfer,
    );
  };

  const collaborative = options.collaborative;
  if (!collaborative) {
    postReply({ error: 'no collaborative session' });
    return;
  }
  const workspaceHandle = getWorkspaceHandle();
  if (!workspaceHandle) {
    postReply({ error: 'cannot get the workspace handle' });
    return;
  }

  const result = await editicsEncryptBatch(workspaceHandle, collaborative.vlobId as EntryID, flat, offsets);
  if (!result.ok) {
    onlyOfficeLog('warn', `Failed to encrypt the editics payloads: ${JSON.stringify(result.error)}`);
    postReply({ error: `failed to encrypt the payloads (${result.error.tag})` });
    return;
  }
  const [keyIndex, encryptedFlat, encryptedOffsets] = result.value;
  // The host page carries the key index as a plain number (the wire schema
  // uses JSON, which cannot serialize bigints).
  postReply({ keyIndex: Number(keyIndex), flat: encryptedFlat, offsets: encryptedOffsets }, transferBuffer(encryptedFlat));
}

// Opens the editics payloads received from the collaborative editics session:
// the counterpart of `handleEncryptBatchRequest`, resolving the key by index
// from the workspace's realm keys bundle. All the payloads of a batch are
// sealed with the same key, and travel as a single flat buffer like on the
// encrypt side.
async function handleDecryptBatchRequest(
  host: Window,
  requestId: number,
  keyIndex: number,
  flat: Uint8Array,
  offsets: number[],
  hostOrigin: string,
  options: EditicsOpenOptions,
): Promise<void> {
  type DecryptReply = Extract<EditicsParentToHostMessage, { command: 'oo-decrypt-batch-result' }>;
  const postReply = (payload: Omit<DecryptReply, 'command' | 'requestId'>, transfer: Transferable[] = []): void => {
    host.postMessage(
      { command: 'oo-decrypt-batch-result', requestId, ...payload } satisfies EditicsParentToHostMessage,
      hostOrigin,
      transfer,
    );
  };

  const collaborative = options.collaborative;
  if (!collaborative) {
    postReply({ error: 'no collaborative session' });
    return;
  }
  const workspaceHandle = getWorkspaceHandle();
  if (!workspaceHandle) {
    postReply({ error: 'cannot get the workspace handle' });
    return;
  }

  const result = await editicsDecryptBatch(workspaceHandle, collaborative.vlobId as EntryID, BigInt(keyIndex), flat, offsets);
  if (!result.ok) {
    onlyOfficeLog('warn', `Failed to decrypt the editics payloads: ${JSON.stringify(result.error)}`);
    postReply({ error: `failed to decrypt the payloads (${result.error.tag})` });
    return;
  }
  const [cleartextFlat, cleartextOffsets] = result.value;
  postReply({ flat: cleartextFlat, offsets: cleartextOffsets }, transferBuffer(cleartextFlat));
}

async function handleRequestImage(host: Window, requestId: number): Promise<void> {
  const hostOrigin = await getEditicsOrigin();

  const postReply = (
    reply: Omit<Extract<EditicsParentToHostMessage, { command: 'oo-insert-image-result' }>, 'command' | 'requestId'>,
  ): void => {
    host.postMessage({ command: 'oo-insert-image-result', requestId, ...reply } satisfies EditicsParentToHostMessage, hostOrigin);
  };

  const workspaceHandle = getWorkspaceHandle();
  if (!workspaceHandle) {
    postReply({
      error: 'cannot get the workspace handle',
    });
    return;
  }

  const image = await selectImage({ workspaceHandle });
  if (image === null) {
    postReply({
      error: 'cancelled',
    });
    return;
  }

  postReply({
    fileName: image.name,
    data: image.content,
  });
}

// Opens the workspace image picker (see `ImageSelectionModal`) and resolves
// with the name and content of the image, or `null` if the user cancelled.
async function selectImage(options: { workspaceHandle: WorkspaceHandle }): Promise<{ name: string; content: Uint8Array } | null> {
  const modal = await modalController.create({
    component: ImageSelectionModal,
    canDismiss: true,
    cssClass: 'image-selection-modal',
    componentProps: {
      workspaceHandle: options.workspaceHandle,
      title: 'fileEditors.insertImage.title',
      okButtonLabel: 'fileEditors.insertImage.insert',
    },
  });
  await modal.present();
  const { data, role } = await modal.onWillDismiss();
  await modal.dismiss();
  return role === MsModalResult.Confirm ? (data as { name: string; content: Uint8Array }) : null;
}

// --- collaborative session ------------------------------------------------
//
// The collaborative edition is hosted by the Parsec server:
// the host page joins the session with everything gathered here, since it is
// isolated from the app's data by design (the parent is the only side with
// access to libparsec, so it is the one resolving the organization/workspace/
// document IDs and providing the authentication token).

// The SSE/POST session routes live on the Parsec server hosting the
// organization (see `server/parsec/asgi/editics.py`), which is not always
// reachable on the same origin as the host page:
// - Web release: the host page is served by the Parsec server itself, from the
//   dedicated editics domain (see `EditicsHostIsolationMiddleware` in
//   `server/parsec/asgi/__init__.py`), and its content security policy
//   restricts it to `connect-src 'self'`: the session routes must then be
//   reached on that same origin.
// - Electron & web dev: the host page is served locally (custom scheme / Vite
//   dev server), the Parsec server is the one from the organization address
//   (note this requires the server to allow the editics origin for CORS in
//   dev mode, see `editics/README.md`).
async function getEditicsSessionServerUrl(organizationAddr: string): Promise<string | undefined> {
  if (isWeb() && !window.isDev()) {
    return await getEditicsOrigin();
  }
  const parsedAddr = await parseParsecAddr(organizationAddr);
  if (!parsedAddr.ok) {
    onlyOfficeLog('warn', `Failed to parse the organization address '${organizationAddr}'`);
    return undefined;
  }
  const { hostname, port, isDefaultPort, useSsl } = parsedAddr.value;
  return `${useSsl ? 'https' : 'http'}://${hostname}${isDefaultPort ? '' : `:${port}`}`;
}

// Gathers everything the collaborative host page needs to join the editics
// session of the document (see `EditicsCollaborativeSession`).
//
// Resolves `undefined` when the session cannot be joined (e.g. the
// organization address could not be resolved), in which case the caller
// falls back on the offline mode.
export async function getEditicsCollaborativeSession(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
): Promise<EditicsCollaborativeSession | undefined> {
  const clientInfo = await getClientInfo();
  if (!clientInfo.ok) {
    onlyOfficeLog('warn', `Cannot get the client info: ${JSON.stringify(clientInfo.error)}`);
    return undefined;
  }

  const serverUrl = await getEditicsSessionServerUrl(clientInfo.value.organizationAddr);
  if (serverUrl === undefined) {
    onlyOfficeLog('warn', 'Cannot resolve the editics server URL, falling back on the offline mode');
    return undefined;
  }

  const [workspaceInfo, entry] = await Promise.all([getWorkspaceInfo(workspaceHandle), entryStat(workspaceHandle, path)]);
  if (!workspaceInfo.ok) {
    onlyOfficeLog('warn', `Cannot get the workspace info: ${JSON.stringify(workspaceInfo.error)}`);
    return undefined;
  }
  if (!entry.ok || !entry.value.isFile()) {
    onlyOfficeLog('warn', `Cannot stat the document being opened: ${JSON.stringify(entry.ok ? undefined : entry.error)}`);
    return undefined;
  }

  // The collaborative edition is not possible in an archived workspace: the
  // server would refuse the session anyway (the realm is read-only, see the
  // `RealmArchived` outcome in `server/parsec/asgi/editics.py`).
  if (workspaceInfo.value.isArchived) {
    onlyOfficeLog('info', 'The workspace is archived, falling back on the offline mode');
    return undefined;
  }

  const vlobId = (entry.value as EntryStatFile).id;
  // The token is generated by libparsec (see `generateEditicsToken` in
  // `@/parsec`): only it has access to the device's signing key.
  const tokenResult = await generateEditicsToken(workspaceHandle, vlobId);
  if (!tokenResult.ok) {
    onlyOfficeLog('warn', `Cannot generate the editics token: ${JSON.stringify(tokenResult.error)}`);
    return undefined;
  }

  return {
    serverUrl,
    organizationId: clientInfo.value.organizationId,
    workspaceId: workspaceInfo.value.id,
    vlobId,
    vlobVersion: (entry.value as EntryStatFile).baseVersion,
    deviceIdHex: clientInfo.value.deviceId,
    token: tokenResult.value,
  };
}

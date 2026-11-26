// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import { FileContentType } from '@/common/fileTypes';
import ImageSelectionModal from '@/components/files/explorer/ImageSelectionModal.vue';
import {
  DeviceID,
  editicsDecryptBatch,
  editicsEncryptBatch,
  editicsGenerateAuthToken,
  EntryID,
  getUserInfoFromDeviceID,
  isElectron,
  WorkspaceHandle,
} from '@/parsec';
import { getWorkspaceHandle } from '@/router';
import { Env } from '@/services/environment';
import type {
  EditicsDocumentTypes,
  EditicsHostToParentMessage,
  EditicsOpenOptions,
  EditicsParentToHostMessage,
  EditicsRpcConvertToPdfReply,
  EditicsRpcDecryptBatchReply,
  EditicsRpcDocumentContentReply,
  EditicsRpcEncryptBatchReply,
  EditicsRpcImageReply,
  EditicsRpcParticipantNameReply,
  EditicsRpcSaveReply,
  EditicsRpcTokenReply,
} from '@editics_parent_host_api';
import { modalController } from '@ionic/vue';
import { MsModalResult } from 'megashark-lib';

// Determines the origin from which the editics host page (i.e. the IFrame
// running the editor) must be loaded, given the editics server hosting the
// document's session (as provided by the server configuration through
// `WorkspaceEditicsPreparedDocumentSession.editicsServerUrl`, `null` meaning
// no editics server is available):
// - Electron: the app itself hosts the editics pages, so the editics server is
//   not needed (and in particular the offline mode works out of the box).
// - Web dev mode: the Vite dev server hosts the editics pages (the editics
//   server hosting the collaborative session is not even guaranteed to be
//   serving web pages). This is fine given those pages have no
//   Content-Security-Policy, and the testbed server hosting the collaborative
//   session accepts requests from any origin.
// - Web release mode: the editics server hosts the editics pages (both in
//   collaborative and offline mode). This is required given those pages are
//   served with a Content-Security-Policy only allowing network access to
//   their own origin, and the server providing them must be the one hosting
//   the collaborative session anyway.
//
// Returns `undefined` when editics cannot be used at all: the editics pages
// cannot be fetched without an editics server in web release mode, not even
// for the offline mode.
export function getEditicsIframeOrigin(editicsServerUrl: string | null): string | undefined {
  // Page is served on a different origin for security reasons.
  // Use this origin to filter the messages we get.
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
  } else if (editicsServerUrl !== null) {
    // Web release mode: the editics server hosting the document's session also
    // hosts the editics pages.
    return editicsServerUrl;
  } else {
    // Web release mode without any editics server: editics cannot be used at all
    return undefined;
  }
}

// In release builds, `scripts/vite_plugin_editics.ts` emits the host page with
// a content hash for cache busting (e.g. `editics/index-1234abcd.html`), and
// advertises the actual name through a dedicated meta tag injected in the main
// app's `index.html` (i.e. `editics-page`).
//
// In dev mode, the Vite dev server serves the page un-hashed, hence the
// fallback (the meta tag is only injected in release builds).
const EDITICS_HOST_PAGE_META_NAME = 'editics-page';
const EDITICS_DEV_HOST_PAGE = 'editics/index.html';

function getEditicsHostPage(): string {
  const content = document.querySelector<HTMLMetaElement>(`meta[name="${EDITICS_HOST_PAGE_META_NAME}"]`)?.content;
  return !!content ? content : EDITICS_DEV_HOST_PAGE;
}

function getEditicsFrameUrl(editicsOrigin: string): URL {
  const BASE_URL = import.meta.env.BASE_URL.endsWith('/') ? import.meta.env.BASE_URL : `${import.meta.env.BASE_URL}/`;
  const url = new URL(`${BASE_URL}${getEditicsHostPage()}`, editicsOrigin);
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
  onDocumentReady: () => void;
  onError: (error: any) => void;
  // Optional notification that the editor application finished booting and
  // is about to open the document (i.e. OnlyOffice's own loading UI is
  // displayed from now on): used to hide the app's loading overlay at the
  // right time, without waiting for the document to be fully loaded.
  onAppReady?: () => void;
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

// Sends an RPC to the editics host page (see the `oo-rpc-*` messages in
// `client/editics/parent_host_api.ts`) and resolves with the payload it posts
// back on the MessagePort transferred along with the message. The request never
// resolves if the host page cannot answer: `timeoutMs` instead rejects with
// 'timeout' (e.g. when it is reloaded mid-request).
function rpcToHost<T>(host: Window, request: EditicsParentToHostMessage, hostOrigin: string, timeoutMs: number): Promise<T> {
  return new Promise((resolve, reject) => {
    const channel = new MessageChannel();
    const timer = setTimeout((): void => {
      channel.port1.close();
      reject(new Error('the host page did not reply in time'));
    }, timeoutMs);
    channel.port1.onmessage = (event: MessageEvent<T>): void => {
      clearTimeout(timer);
      channel.port1.close();
      resolve(event.data);
    };
    host.postMessage(request, hostOrigin, [channel.port2]);
  });
}

// The reply port of an `oo-rpc-*` message: it is always transferred along with
// the message (see `client/editics/parent_host_api.ts`), a missing one is a bug
// on the sender (the host page) side.
function expectRpcPort(event: MessageEvent<EditicsHostToParentMessage>): MessagePort {
  const port = event.ports[0];
  if (!port) {
    throw new Error(`Bug: the '${String(event.data?.command)}' RPC message arrived without its reply port`);
  }
  return port;
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
  getDocumentContent: () => Promise<Uint8Array | undefined>,
  handlers: EditicsEventHandlers,
  frame: HTMLIFrameElement,
): Promise<EditicsHostSession | undefined> {
  const controller = new AbortController();

  // Save requests are asynchronous: the host converts the data, we perform
  // the workspace write, then reply on the port it provided (see the
  // `oo-rpc-save` message).
  let saving = false;

  const finishSave = (port: MessagePort | undefined, success: boolean, error?: string): void => {
    if (port) {
      port.postMessage({ success, error } satisfies EditicsRpcSaveReply);
      port.close();
    }
    if (saving) {
      saving = false;
      handlers.onSaveStateChange?.(success ? EditicsSaveState.Saved : EditicsSaveState.Error);
    }
  };

  let session: EditicsHostSession | undefined = undefined;

  // Note: the editics server to use must have been determined beforehand by
  // the caller (see `getEditicsIframeOrigin`), given in web release mode the
  // editics host page is fetched from the editics server hosting the
  // collaborative session.
  let frameUrl: URL;
  try {
    frameUrl = getEditicsFrameUrl(options.editicsIframeOrigin);
  } catch (e: unknown) {
    handlers.onError(new EditicsError(EditicsErrorCodes.FrameLoadFailed, String(e)));
    return undefined;
  }
  const hostOrigin = frameUrl.origin;

  // We have a strict Content-Security-Policy (CSP) for the Editics IFrame (since
  // it has OnlyOffice loading an arbitrary document), and only allow communication
  // with the Editics server.
  // - In web this server is the one that host the Editics page, so it can provide
  //   the CSP itself.
  // - In Electron, the Editics page is served by the main process, so we need
  //   to register the allowed server origin.
  if (options.collaborative !== undefined && isElectron()) {
    // Note: a bare origin is expected here (the CSP does not support a path,
    // and the server URL may carry a trailing `/`).
    await window.nativeAPI.declareEditicsServerOriginForCsp(new URL(options.collaborative.serverUrl).origin);
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
            case 'oo-event-host-ready': {
              clearTimeout(hostReadyTimeoutId);
              // The host page signals it is ready to open a document: send
              // `oo-event-open` every time it does (a reloaded host page restarts
              // from scratch and will fetch the document's content again, see
              // `oo-rpc-get-document-content`). Also reset the save bookkeeping:
              // the reloaded host page starts with a clean save state (the reload
              // only ever happens before any change exists, so there is no
              // pending save to preserve).
              saving = false;
              frame.contentWindow?.postMessage({ command: 'oo-event-open', options } satisfies EditicsParentToHostMessage, hostOrigin);
              resolve();
              break;
            }

            case 'oo-event-app-ready': {
              onlyOfficeLog('debug', 'OnlyOffice app is ready');
              handlers.onAppReady?.();
              break;
            }

            case 'oo-event-document-ready': {
              onlyOfficeLog('debug', 'Document is ready');
              handlers.onDocumentReady();
              break;
            }

            case 'oo-event-error': {
              handlers.onError(new EditicsError(EditicsErrorCodes.EventError, event.data.details));
              break;
            }

            case 'oo-event-save-state': {
              // The editor's dirty state changed ('unsaved' when the document has
              // modifications, 'saved' when it is clean again).
              if (!saving) {
                const state = event.data.state === 'unsaved' ? EditicsSaveState.Unsaved : EditicsSaveState.Saved;
                handlers.onSaveStateChange?.(state);
              }
              break;
            }

            case 'oo-rpc-save': {
              // The host converted the editor's native serialization back to the
              // file's original office format and asks us to persist it. We answer
              // on the transferred port once the workspace write completes; only
              // this side has libparsec access.
              const port = expectRpcPort(event);
              if (!handlers.onSave) {
                port.postMessage({ success: false, error: 'no save handler' } satisfies EditicsRpcSaveReply);
                port.close();
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

            case 'oo-rpc-get-document-content': {
              const port = expectRpcPort(event);
              getDocumentContent()
                .then(
                  (documentContent): EditicsRpcDocumentContentReply => ({
                    data: documentContent,
                    error: documentContent === undefined ? 'failed to read the workspace file' : undefined,
                  }),
                  (err: unknown): EditicsRpcDocumentContentReply => ({ error: String(err) }),
                )
                .then((reply): void => {
                  port.postMessage(reply);
                  port.close();
                });
              break;
            }

            case 'oo-rpc-insert-image': {
              // The editor's user wants to insert an image: present a workspace
              // file picker and send its content back to the host.
              handleInsertImage(expectRpcPort(event));
              break;
            }

            case 'oo-rpc-get-participant-name': {
              handleGetParticipantNameRequest(expectRpcPort(event), event.data.deviceId);
              break;
            }

            case 'oo-rpc-encrypt-batch': {
              handleEncryptBatchRequest(expectRpcPort(event), event.data.flatCleartexts, event.data.offsets, options);
              break;
            }

            case 'oo-rpc-generate-auth-token': {
              handleGenerateAuthTokenRequest(expectRpcPort(event), options);
              break;
            }

            case 'oo-rpc-decrypt-batch': {
              handleDecryptBatchRequest(expectRpcPort(event), event.data.keyIndex, event.data.flatCiphertexts, event.data.offsets, options);
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
      // The reply travels on the port transferred with the `oo-rpc-request-save`
      // message: a host page reloaded mid-save would never answer (see
      // `oo-event-host-ready`), hence the generous timeout.
      return rpcToHost<EditicsRpcSaveReply>(frame.contentWindow, { command: 'oo-rpc-request-save' }, hostOrigin, SAVE_TIMEOUT_MS).then(
        (reply): boolean => reply.success === true,
        (): boolean => {
          onlyOfficeLog('error', 'Save request timed out');
          handlers.onSaveStateChange?.(EditicsSaveState.Error);
          return false;
        },
      );
    },
    setTheme: (theme: EditicsOpenOptions['theme']): void => {
      frame.contentWindow?.postMessage({ command: 'oo-event-set-theme', theme } satisfies EditicsParentToHostMessage, hostOrigin);
    },
    convertToPdf: (): Promise<Uint8Array> => {
      if (!frame.contentWindow) {
        return Promise.reject(new EditicsError(EditicsErrorCodes.FrameNotLoaded));
      }
      return rpcToHost<EditicsRpcConvertToPdfReply>(
        frame.contentWindow,
        { command: 'oo-rpc-convert-to-pdf' },
        hostOrigin,
        PDF_CONVERSION_TIMEOUT_MS,
      ).then(
        (reply): Uint8Array => {
          // `data` comes from the frame's realm, `instanceof Uint8Array` would be false
          if (reply.data !== undefined && ArrayBuffer.isView(reply.data)) {
            return reply.data;
          }
          const details = reply.error ?? 'no data';
          onlyOfficeLog('error', `PDF conversion failed: ${details}`);
          throw new EditicsError(EditicsErrorCodes.ConversionFailed, details);
        },
        (): Uint8Array => {
          onlyOfficeLog('error', 'PDF conversion request timed out');
          throw new EditicsError(EditicsErrorCodes.ConversionTimeout);
        },
      );
    },
  };

  return session;
}

async function handleInsertImage(port: MessagePort): Promise<void> {
  const postReply = (reply: EditicsRpcImageReply): void => {
    port.postMessage(reply);
    port.close();
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

async function handleGetParticipantNameRequest(port: MessagePort, deviceId: DeviceID): Promise<void> {
  let participantName: string | undefined = undefined;
  const result = await getUserInfoFromDeviceID(deviceId);
  if (result.ok) {
    participantName = result.value.humanHandle.label;
  } else {
    onlyOfficeLog('warn', `Failed to resolve the participant name of the device ${deviceId}`);
  }
  const reply = { participantName } satisfies EditicsRpcParticipantNameReply;
  port.postMessage(reply);
  port.close();
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

async function handleEncryptBatchRequest(
  port: MessagePort,
  flatCleartexts: Uint8Array,
  offsets: number[],
  options: EditicsOpenOptions,
): Promise<void> {
  const postReply = (payload: EditicsRpcEncryptBatchReply, transfer: Transferable[] = []): void => {
    port.postMessage(payload, transfer);
    port.close();
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

  const result = await editicsEncryptBatch(workspaceHandle, collaborative.documentId, flatCleartexts, offsets);
  if (!result.ok) {
    onlyOfficeLog('warn', `Failed to encrypt the editics payloads: ${JSON.stringify(result.error)}`);
    postReply({ error: `failed to encrypt the payloads (${result.error.tag})` });
    return;
  }
  const [keyIndex, flatCiphertexts, ciphertextOffsets] = result.value;
  // The host page carries the key index as a plain number (the wire schema
  // uses JSON, which cannot serialize bigints).
  postReply({ keyIndex: Number(keyIndex), flatCiphertexts, offsets: ciphertextOffsets }, transferBuffer(flatCiphertexts));
}

async function handleDecryptBatchRequest(
  port: MessagePort,
  keyIndex: number,
  flatCiphertexts: Uint8Array,
  offsets: number[],
  options: EditicsOpenOptions,
): Promise<void> {
  const postReply = (payload: EditicsRpcDecryptBatchReply, transfer: Transferable[] = []): void => {
    port.postMessage(payload, transfer);
    port.close();
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

  const result = await editicsDecryptBatch(
    workspaceHandle,
    collaborative.documentId as EntryID,
    BigInt(keyIndex),
    flatCiphertexts,
    offsets,
  );
  if (!result.ok) {
    onlyOfficeLog('warn', `Failed to decrypt the editics payloads: ${JSON.stringify(result.error)}`);
    postReply({ error: `failed to decrypt the payloads (${result.error.tag})` });
    return;
  }
  const [flatCleartexts, cleartextOffsets] = result.value;
  postReply({ flatCleartexts: flatCleartexts, offsets: cleartextOffsets }, transferBuffer(flatCleartexts));
}

async function handleGenerateAuthTokenRequest(port: MessagePort, options: EditicsOpenOptions): Promise<void> {
  const postReply = (payload: EditicsRpcTokenReply): void => {
    port.postMessage(payload);
    port.close();
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

  const result = await editicsGenerateAuthToken(workspaceHandle, collaborative.documentId);
  if (!result.ok) {
    onlyOfficeLog('warn', `Failed to generate the editics token: ${JSON.stringify(result.error)}`);
    postReply({ error: `failed to generate the token (${result.error.tag})` });
    return;
  }
  postReply({ token: result.value });
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

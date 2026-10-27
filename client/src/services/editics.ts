// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getFileContent } from '@/common/file';
import { FileContentType } from '@/common/fileTypes';
import ImageSelectionModal from '@/components/files/explorer/ImageSelectionModal.vue';
import {
  decryptEditicsOriginalContent,
  DeviceID,
  encryptEditicsOriginalContent,
  EntryID,
  entryStat,
  EntryStatFile,
  FsPath,
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
  // Passing our origin to the frame, so it knows how to contact us and check for us
  url.searchParams.set('parentOrigin', window.location.origin);
  return url;
}

export enum EditicsErrorCodes {
  FrameNotLoaded = 'frame-not-loaded',
  FrameLoadFailed = 'frame-load-failed',
  EventError = 'event-error',
  SaveFailed = 'save-failed',
  SaveTimeout = 'save-timeout',
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
}

// Time we give the host page to signal it is ready after its iframe is loaded.
// Short in tests to fail fast.
const HOST_READY_TIMEOUT_MS = (window as any).TESTING === true ? 500 : 5000;

// Time we give the host page to complete a save round-trip: it serializes the
// document, converts it back to its original office format with x2t, and asks
// us to write it to the workspace before running the editor's save handshake.
const SAVE_TIMEOUT_MS = 30000;

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
              // it signals readiness (a host page reloaded on `updateVersion`,
              // see RFC 1030 §1.2, restarts from scratch and needs the document
              // again). Also reset the save bookkeeping: the reloaded host page
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

            case 'oo-get-user-name': {
              // Collaborative mode: the host needs the display name of a
              // session participant. The editics protocol deliberately
              // doesn't carry user names (the server is not trusted for them,
              // RFC 1030 §3.3), so the host asks the only side with access to
              // libparsec to resolve the participant's device ID.
              if (!frame.contentWindow) {
                return;
              }
              handleUserNameRequest(frame.contentWindow, event.data.requestId, event.data.deviceIdHex, hostOrigin);
              break;
            }

            case 'oo-encrypt-original-content': {
              // Collaborative mode: the host wants to upload the document's
              // original content archive (its x2t conversion result, RFC 1030
              // §1.2) to the editics session: it is sealed with the workspace
              // key here (the only side with access to libparsec) before
              // being sent to the server, which only ever sees an opaque
              // blob.
              if (!frame.contentWindow) {
                return;
              }
              handleEncryptOriginalContentRequest(
                frame.contentWindow,
                event.data.requestId,
                event.data.bin,
                event.data.medias,
                hostOrigin,
                options,
              );
              break;
            }

            case 'oo-decrypt-original-content': {
              // Collaborative mode: the host fetched the session's original
              // content archive and asks the parent to open it (the
              // counterpart of the `oo-encrypt-original-content` handling).
              if (!frame.contentWindow) {
                return;
              }
              handleDecryptOriginalContentRequest(
                frame.contentWindow,
                event.data.requestId,
                event.data.keyIndex,
                event.data.encrypted,
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
  };

  // Note the document is *not* posted here: the host page asks for it with
  // `oo-host-ready` (see its handling above), and may ask again if it gets
  // reloaded (collaborative mode, see RFC 1030 §1.2).

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

// Seals the original content archive of the document (its x2t conversion
// result, RFC 1030 §1.2) with the workspace key, so the host can upload it to
// the collaborative editics session: only this side has access to libparsec,
// and the server only ever sees the opaque ciphertext.
async function handleEncryptOriginalContentRequest(
  host: Window,
  requestId: number,
  bin: Uint8Array,
  medias: Array<[string, Uint8Array]>,
  hostOrigin: string,
  options: EditicsOpenOptions,
): Promise<void> {
  type EncryptPayload = Omit<
    Extract<EditicsParentToHostMessage, { command: 'oo-encrypt-original-content-result' }>,
    'command' | 'requestId'
  >;
  const postReply = (payload: EncryptPayload): void => {
    host.postMessage(
      { command: 'oo-encrypt-original-content-result', requestId, ...payload } satisfies EditicsParentToHostMessage,
      hostOrigin,
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

  // The host page carries the media as `(name, content)` tuples while the
  // libparsec bindings use a structure (tuples are not supported by the
  // wasm-bindgen boundary).
  const mediasEntries = medias.map(([name, content]) => ({ name, content }));
  const result = await encryptEditicsOriginalContent(workspaceHandle, collaborative.vlobId as EntryID, bin, mediasEntries);
  if (result.ok) {
    const [keyIndex, encrypted] = result.value;
    // The host page carries the key index as a plain number (the wire schema
    // uses JSON, which cannot serialize bigints).
    postReply({ keyIndex: Number(keyIndex), encrypted });
  } else {
    onlyOfficeLog('warn', `Failed to encrypt the editics original content: ${JSON.stringify(result.error)}`);
    postReply({ error: `failed to encrypt the original content (${result.error.tag})` });
  }
}

// Opens an original content archive fetched from the collaborative editics
// session (the counterpart of `handleEncryptOriginalContentRequest`): the
// key is resolved by index from the workspace's realm keys bundle.
async function handleDecryptOriginalContentRequest(
  host: Window,
  requestId: number,
  keyIndex: number,
  encrypted: Uint8Array,
  hostOrigin: string,
  options: EditicsOpenOptions,
): Promise<void> {
  type DecryptPayload = Omit<
    Extract<EditicsParentToHostMessage, { command: 'oo-decrypt-original-content-result' }>,
    'command' | 'requestId'
  >;
  const postReply = (payload: DecryptPayload): void => {
    host.postMessage(
      { command: 'oo-decrypt-original-content-result', requestId, ...payload } satisfies EditicsParentToHostMessage,
      hostOrigin,
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

  const result = await decryptEditicsOriginalContent(workspaceHandle, collaborative.vlobId as EntryID, BigInt(keyIndex), encrypted);
  if (result.ok) {
    const [bin, medias] = result.value;
    // Back to the `(name, content)` tuple shape the host page expects (see
    // the encrypt handling above).
    postReply({ bin, medias: medias.map(({ name, content }) => [name, content]) });
  } else {
    onlyOfficeLog('warn', `Failed to decrypt the editics original content: ${JSON.stringify(result.error)}`);
    postReply({ error: `failed to decrypt the original content (${result.error.tag})` });
  }
}

async function handleRequestImage(host: Window, requestId: number): Promise<void> {
  const hostOrigin = await getEditicsOrigin();

  const postReply = (reply: Omit<Extract<EditicsParentToHostMessage, { command: 'oo-image-reply' }>, 'command' | 'requestId'>): void => {
    host.postMessage({ command: 'oo-insert-image-result', requestId, ...reply } satisfies EditicsParentToHostMessage, hostOrigin);
  };

  const workspaceHandle = getWorkspaceHandle();
  if (!workspaceHandle) {
    postReply({
      // @ts-expect-error
      error: 'cannot get the workspace handle',
    });
    return;
  }

  const path = await selectImage({ workspaceHandle });
  if (path === null) {
    postReply({
      // @ts-expect-error
      error: 'cancelled',
    });
    return;
  }

  const content = await getFileContent(workspaceHandle, path as FsPath);
  if (!content) {
    postReply({
      // @ts-expect-error
      error: `failed to read ${path} from the workspace`,
    });
    return;
  }

  postReply({
    // @ts-expect-error
    fileName: path.split('/').pop() ?? 'image',
    // @ts-expect-error
    data: content,
  });
}

// Opens the workspace image picker (see `ImageSelectionModal`) and resolves
// with the workspace-absolute path of the selected image, or `null` if the
// user cancelled.
async function selectImage(options: { workspaceHandle: WorkspaceHandle }): Promise<string | null> {
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
  const result = await modal.onWillDismiss();
  await modal.dismiss();
  return result.role === MsModalResult.Confirm ? (result.data as string | null) : null;
}

// --- collaborative session ------------------------------------------------
//
// The collaborative edition is hosted by the Parsec server (see RFC 1030):
// the host page joins the session with everything gathered here, since it is
// isolated from the app's data by design (the parent is the only side with
// access to libparsec, so it is the one resolving the organization/workspace/
// document IDs and providing the authentication token).

// Generates the editics authentication token expected by the server (see
// `AuthenticatedToken`/`EditicsToken` in `server/parsec/components/auth.py`):
// `PARSEC-EDITICS-SIGN-ED25519.<device_id_hex>.<timestamp>.<urlsafe_b64_signature>`
// with <timestamp> the current time in seconds since the UNIX epoch (checked
// against the server time within the usual ballpark tolerance).
//
// TODO(libparsec): a genuine token is signed with the device's signing key,
// which the libparsec bindings do not expose yet (a dedicated API is needed to
// generate the editics token, see RFC 1030 §3). Until then we rely on the
// `trust_me_bro` placeholder signature the server accepts for that exact
// purpose (see the `TODO: remove me when the client supports token signature!`
// in `server/parsec/components/auth.py`): the rest of the token is genuine
// (device ID + timestamp).
function generateEditicsAuthenticationToken(deviceIdHex: string): string {
  // URL-safe base64 of the `trust_me_bro` signature bytes, like any genuine
  // ed25519 signature would be (see `AuthenticatedToken.generate_raw`).
  const TRUST_ME_BRO_SIGNATURE = 'dHJ1c3RfbWVfYnJv';
  return `PARSEC-EDITICS-SIGN-ED25519.${deviceIdHex}.${Math.floor(Date.now() / 1000)}.${TRUST_ME_BRO_SIGNATURE}`;
}

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

  const token = generateEditicsAuthenticationToken(clientInfo.value.deviceId);

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

  return {
    serverUrl,
    organizationId: clientInfo.value.organizationId,
    workspaceId: workspaceInfo.value.id,
    vlobId: (entry.value as EntryStatFile).id,
    vlobVersion: (entry.value as EntryStatFile).baseVersion,
    deviceIdHex: clientInfo.value.deviceId,
    token,
  };
}

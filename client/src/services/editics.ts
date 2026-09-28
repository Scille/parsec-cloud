// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getFileContent } from '@/common/file';
import { FileContentType } from '@/common/fileTypes';
import ImageSelectionModal from '@/components/files/explorer/ImageSelectionModal.vue';
import { FsPath, WorkspaceHandle } from '@/parsec';
import { getWorkspaceHandle } from '@/router';
import { Env } from '@/services/environment';
import type {
  EditicsDocumentTypes,
  EditicsHostToParentMessage,
  EditicsOpenOptions,
  EditicsParentToHostMessage,
  EditicsRequestParentSaveReply,
} from '@editics_parent_host_api';
import { modalController } from '@ionic/vue';
import { MsModalResult } from 'megashark-lib';

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

const BASE_URL = import.meta.env.BASE_URL.endsWith('/') ? import.meta.env.BASE_URL : `${import.meta.env.BASE_URL}/`;
const HOST_PAGE = `${BASE_URL}editics/offline.html`;

// Time we give the host page to signal it is ready after its iframe is loaded.
// Short in tests to fail fast.
const HOST_READY_TIMEOUT_MS = 5000;

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

  try {
    await new Promise<void>((resolve, reject) => {
      const hostReadyTimeoutId = setTimeout(() => {
        reject();
      }, HOST_READY_TIMEOUT_MS);

      window.addEventListener(
        'message',
        (event: MessageEvent<EditicsHostToParentMessage>): void => {
          if (event.source !== frame.contentWindow) {
            return;
          }

          switch (event.data?.command) {
            case 'oo-host-ready': {
              clearTimeout(hostReadyTimeoutId);
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
          }
        },
        { signal: controller.signal },
      );
      frame.src = HOST_PAGE;
    });
  } catch (e: unknown) {
    controller.abort();
    handlers.onError(new EditicsError(EditicsErrorCodes.FrameLoadFailed, JSON.stringify(e)));
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
        frame.contentWindow!.postMessage({ command: 'oo-save-request' } satisfies EditicsParentToHostMessage, '*');
      });
    },
  };

  frame.contentWindow.postMessage(
    {
      command: 'oo-open',
      options,
      documentContent,
    } satisfies EditicsParentToHostMessage,
    '*',
  );

  return session;
}

async function handleRequestImage(host: Window, requestId: number): Promise<void> {
  const postReply = (reply: Omit<Extract<EditicsParentToHostMessage, { command: 'oo-image-reply' }>, 'command' | 'requestId'>): void => {
    host.postMessage({ command: 'oo-insert-image-result', requestId, ...reply } satisfies EditicsParentToHostMessage, '*');
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

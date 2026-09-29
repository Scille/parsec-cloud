// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import type OO from 'onlyoffice-editor';
import type {
  EditicsHostToParentMessage,
  EditicsOpenOptions,
  EditicsParentToHostMessage,
  EditicsRequestParentSaveReply,
} from './parent_host_api';

// Resolve assets relative to this standalone host page. In a web release the
// application may be mounted below a path prefix (e.g. `/client/`), unlike
// Electron and the Vite development server where it is served at the origin.
//
// `__EDITICS_ONLYOFFICE_BASE__` & `__EDITICS_X2T_BASE__` are build-time
// constants injected by the `editics-build-time-paths` plugin (see
// `scripts/vite_plugin_editics.ts`): they point to the versioned OnlyOffice
// asset folders (e.g. `../onlyoffice/9.3.0.140-parsec0/`), in order to handle
// cache busting when upgrading the dependencies.
declare const __EDITICS_ONLYOFFICE_BASE__: string;
declare const __EDITICS_X2T_BASE__: string;
const ONLYOFFICE_API_URL = new URL(`${__EDITICS_ONLYOFFICE_BASE__}web-apps/apps/api/documents/api.js`, window.location.href).href;
const X2T_SCRIPT_URL = new URL(`${__EDITICS_X2T_BASE__}x2t.js`, window.location.href).href;

const parentWindow = window.parent;

// The host page is served from a dedicated origin, so the parent (Parsec app)
// origin is provided by the parent in the URL.
function getParentOrigin(): string {
  const param = new URLSearchParams(window.location.search).get('parentOrigin');
  if (!param) {
    throw new Error('Missing `parentOrigin` URL parameter');
  }
  // Normalize (and reject anything that is not a valid URL)
  return new URL(param).origin;
}

const PARENT_ORIGIN = getParentOrigin();

function postToParent(message: EditicsHostToParentMessage): void {
  parentWindow.postMessage(message, PARENT_ORIGIN);
}

function loadScript(src: string): Promise<void> {
  return new Promise(function (resolve, reject) {
    const script = document.createElement('script');
    script.src = src;
    script.onload = function () {
      resolve();
    };
    script.onerror = function () {
      reject(new Error(`Failed to load ${src}`));
    };
    document.head.appendChild(script);
  });
}

// --- x2t conversion ------------------------------------------------------
//
// OnlyOffice only supports its .bin internal format, so we use x2t to convert
// the document (e.g. docx -x2t-> bin for opening, then bin -x2t-> docx for saving).
//
// The conversion process produces two things:
// - The .bin file to be loaded by OnlyOffice.
// - Additional media files (e.g. images) that are referenced in the document. They
//   are referenced by name in the document (e.g. `foo.png`) and end up exported in
//   a `media` folder next to the .bin file.
//
// The `media` folder is kept in the in-memory file system, this is because it is
// needed whenever we convert back the document ot its original format (i.e.
// when saving it).

interface X2TFileSystem {
  mkdir: (path: string) => void;
  writeFile: (path: string, data: Uint8Array | string) => void;
  readFile: (path: string) => Uint8Array;
  readdir: (path: string) => string[];
  unlink: (path: string) => void;
}

interface X2TModule {
  FS: X2TFileSystem;
  ccall: (name: string, returnType: string, argTypes: string[], args: unknown[]) => number;
  onRuntimeInitialized?: () => void;
}

// Type the interface exposed by `onlyoffice-x2t/x2t.js`
declare global {
  interface Window {
    // onlyoffice-x2t needs this global object for its own initialization.
    // Note the very generic name, this is because onlyoffice-x2t is just a
    // emscripten module without any wrapper.
    Module?: X2TModule;
  }
}

const NATIVE_EXTENSION = 'bin';
const BLANK_TEMPLATE_PATHS: Partial<Record<EditicsOpenOptions['documentType'], string>> = {
  word: '../onlyoffice-templates/word.bin',
  cell: '../onlyoffice-templates/cell.bin',
  slide: '../onlyoffice-templates/slide.bin',
};
const ODF_INTERMEDIARY_FORMAT: Record<string, string> = {
  odt: 'docx',
  ods: 'xlsx',
  odp: 'pptx',
};

let x2tModulePromise: Promise<X2TModule> | undefined;

function loadX2T(): Promise<X2TModule> {
  if (!x2tModulePromise) {
    x2tModulePromise = new Promise<X2TModule>((resolve, reject) => {
      const module = (window.Module = window.Module || ({} as X2TModule));
      module.onRuntimeInitialized = (): void => {
        module.FS.mkdir('/working');
        module.FS.mkdir('/working/media');
        module.FS.mkdir('/working/fonts');
        module.FS.mkdir('/working/themes');
        resolve(module);
      };
      loadScript(X2T_SCRIPT_URL).catch(reject);
    });
  }
  return x2tModulePromise;
}

function sanitizeFileName(name: string): string {
  const sanitized = name.replace(/[/\\?<>:*|"]/g, '');
  return sanitized || 'file';
}

function runConversion(module: X2TModule, inputFileName: string, data: Uint8Array, outputFormat: string): Uint8Array {
  const inputFilePath = `/working/${inputFileName}`;
  module.FS.writeFile(inputFilePath, data);

  const outputFileName = `${inputFileName}.${outputFormat}`;
  const outputFilePath = `/working/${outputFileName}`;
  const paramsFileName = '/working/params.xml';
  const params =
    '<?xml version="1.0" encoding="utf-8"?>' +
    '<TaskQueueDataConvert xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">' +
    `<m_sFileFrom>${inputFilePath}</m_sFileFrom>` +
    '<m_sThemeDir>/working/themes</m_sThemeDir>' +
    `<m_sFileTo>${outputFilePath}</m_sFileTo>` +
    '<m_bIsNoBase64>false</m_bIsNoBase64>' +
    '</TaskQueueDataConvert>';
  module.FS.writeFile(paramsFileName, params);

  module.ccall('main1', 'number', ['string'], [paramsFileName]);
  const output = module.FS.readFile(outputFilePath);

  module.FS.unlink(paramsFileName);
  module.FS.unlink(inputFilePath);
  module.FS.unlink(outputFilePath);
  // Additional medias in the document got extracted to `/working/media`.
  // We must keep this directory since it will be used when converting back the
  // document to its original format (i.e. when saving the document).

  return output;
}

async function convertToNativeFormat(data: Uint8Array, fileName: string, extension: string): Promise<Uint8Array> {
  const module = await loadX2T();

  // The media of the document being converted are extracted into the x2t
  // virtual filesystem (see `collectMediaFiles` below): start fresh so a
  // document opened after another one in the same host page cannot pick up
  // the media of its predecessor.
  resetMediaFiles(module);

  const safeName = sanitizeFileName(fileName);
  const intermediaryFormat = ODF_INTERMEDIARY_FORMAT[extension];

  let nativeContent: Uint8Array;
  if (intermediaryFormat) {
    const intermediaryData = runConversion(module, safeName, data, intermediaryFormat);
    nativeContent = runConversion(module, `${safeName}.${intermediaryFormat}`, intermediaryData, NATIVE_EXTENSION);
  } else {
    nativeContent = runConversion(module, safeName, data, NATIVE_EXTENSION);
  }

  collectMediaFiles(module);
  return nativeContent;
}

// --- document media (images) ---------------------------------------------
//
// Converting a document to the native .bin format makes x2t extract its
// embedded media files (images) next to the output file, under `media/`:
// the .bin itself only references them by name (e.g. `image1.png`). The
// editor asks for a loadable URL for each of those names through the
// `getImageURL` host hook (see `window.APP.getImageURL`), which we answer
// with a blob URL wrapping the bytes kept in memory.
//
// The files are also left in the x2t virtual filesystem so that converting
// the .bin back to the original document format on save re-embeds them.

const MEDIA_DIR = '/working/media';
// Media name (e.g. `image1.png`) -> content, as extracted by the last
// conversion to the native format.
const mediaFiles = new Map<string, Uint8Array>();
// Media name -> blob URL already handed to the editor (kept so that repeated
// requests for the same image reuse the same blob instead of leaking one
// per request).
const mediaUrls = new Map<string, string>();

const IMAGE_MIME_TYPES: Record<string, string> = {
  bmp: 'image/bmp',
  gif: 'image/gif',
  jpeg: 'image/jpeg',
  jpg: 'image/jpeg',
  png: 'image/png',
  svg: 'image/svg+xml',
  tif: 'image/tiff',
  tiff: 'image/tiff',
  webp: 'image/webp',
};

function mediaName(name: string): string {
  // The editor refers to the media files with their bare name, but be lenient
  // and accept the `media/`-prefixed form too.
  return name.startsWith('media/') ? name.slice('media/'.length) : name;
}

function imageMimeType(name: string): string {
  const extension = name.slice(name.lastIndexOf('.') + 1).toLowerCase();
  return IMAGE_MIME_TYPES[extension] ?? 'application/octet-stream';
}

// Drops the media of any previously converted document (both from the x2t
// virtual filesystem and from memory).
function resetMediaFiles(module: X2TModule): void {
  for (const name of module.FS.readdir(MEDIA_DIR)) {
    if (name !== '.' && name !== '..') {
      module.FS.unlink(`${MEDIA_DIR}/${name}`);
    }
  }
  mediaFiles.clear();
  for (const url of mediaUrls.values()) {
    URL.revokeObjectURL(url);
  }
  mediaUrls.clear();
}

// Keeps the media extracted by the conversion of the document being opened.
function collectMediaFiles(module: X2TModule): void {
  for (const name of module.FS.readdir(MEDIA_DIR)) {
    if (name === '.' || name === '..') {
      continue;
    }
    try {
      mediaFiles.set(name, module.FS.readFile(`${MEDIA_DIR}/${name}`));
    } catch {
      // Unreadable entry (should not happen): skip it, the editor will just
      // render the corresponding image as unloaded.
    }
  }
}

// `MockServer.getImageURL`: resolves a media name referenced by the document
// into a URL the editor can load. An empty string makes the editor fall back
// to loading the name directly as a URL, which is the right thing for the
// names that are already loadable as-is (e.g. the data: URLs of images
// inserted during the session).
function getMediaUrl(name: string): string {
  const key = mediaName(name);
  let url = mediaUrls.get(key);
  if (url === undefined) {
    const content = mediaFiles.get(key);
    if (content === undefined) {
      return '';
    }
    url = URL.createObjectURL(new Blob([content.slice()], { type: imageMimeType(key) }));
    mediaUrls.set(key, url);
    registerMediaUrl(key, url);
  }
  return url;
}

// The subset of the editor iframe's window we need to register media URLs
// (see `registerMediaUrl`).
interface EditorWindowWithUrlMaps {
  AscCommon?: {
    g_oDocumentUrls?: {
      addImageUrl?: (name: string, url: string) => void;
    };
  };
}

// Registers a media blob URL in the editor's URL maps (`g_oDocumentUrls`):
// the editor uses them to resolve media names into loadable URLs and, the
// other way around, to map an image URL back to its media name when
// serializing (e.g. so that an image referenced by its blob URL keeps its
// plain `image1.png` name in the saved document).
// This mirrors what the reference CryptPad integration does when serving
// an image.
function registerMediaUrl(key: string, url: string): void {
  const editorFrame = document.querySelector('iframe[name="frameEditor"]') as HTMLIFrameElement | null;
  const editorWindow = editorFrame?.contentWindow as EditorWindowWithUrlMaps | undefined;
  editorWindow?.AscCommon?.g_oDocumentUrls?.addImageUrl?.(key, url);
}

async function convertFromNativeFormat(data: Uint8Array, fileName: string, extension: string): Promise<Uint8Array> {
  if (extension === NATIVE_EXTENSION) {
    return data;
  }

  const module = await loadX2T();
  const safeName = `${sanitizeFileName(fileName)}.${NATIVE_EXTENSION}`;
  const intermediaryFormat = ODF_INTERMEDIARY_FORMAT[extension];

  if (intermediaryFormat) {
    const intermediaryData = runConversion(module, safeName, data, intermediaryFormat);
    return runConversion(module, `${safeName}.${intermediaryFormat}`, intermediaryData, extension);
  }
  return runConversion(module, safeName, data, extension);
}

async function loadBlankTemplate(documentType: EditicsOpenOptions['documentType'], fallback: Uint8Array): Promise<Uint8Array> {
  const templatePath = BLANK_TEMPLATE_PATHS[documentType];
  if (!templatePath) {
    return fallback;
  }
  try {
    const response = await fetch(new URL(templatePath, window.location.href));
    return new Uint8Array(await response.arrayBuffer());
  } catch (error: unknown) {
    throw new Error(`Failed to load blank document template: ${String(error)}`);
  }
}

async function prepareDocumentContent(options: EditicsOpenOptions, documentContent: Uint8Array): Promise<Uint8Array> {
  if (documentContent.byteLength === 0) {
    return loadBlankTemplate(options.documentType, documentContent);
  }
  if (options.documentExtension === NATIVE_EXTENSION) {
    return documentContent;
  }
  return convertToNativeFormat(documentContent, options.documentName, options.documentExtension);
}

// --- OfflineMockServer -----------------------------------------------------

// The OnlyOffice editor still runs its co-authoring state machine over the
// wrapper's message bridge, so it expects a "server" to answer the `auth`
// handshake, the save-lock requests and the element (block) locks.
class OfflineMockServer implements OO.MockServer {
  editor: OO.DocEditor;
  userId: string;
  username: string;
  // Server-assigned participant index (see `answerAuth` event). The OnlyOffice
  // protocol identifies users by the composite `<idOriginal><indexUser>` ID.
  indexUser: number;
  compositeUserId: string;
  // Monotonic change counter (the OnlyOffice "puckerIndex"): the total number
  // of changes the server has stored. Used to answer the saveChanges flow
  // (`savePartChanges`/`unSaveLock`).
  syncChangesIndex: number;
  // Absolute change index at which the in-progress save starts (the
  // "save point" reported back in the final `unSaveLock`).
  saveChangesIndex: number;

  constructor(editor: OO.DocEditor, options?: { user?: { id: string; username: string } }) {
    this.editor = editor;
    const u = options?.user ?? { id: 'uid-local', username: 'Local user' };
    this.userId = u.id;
    this.username = u.username;
    this.indexUser = 1;
    this.compositeUserId = this.userId + this.indexUser;
    this.syncChangesIndex = 0;
    this.saveChangesIndex = 0;
  }

  onMessage(msg: OO.OOClientEvent): void {
    switch (msg.type) {
      case 'auth':
        const participant = {
          id: this.compositeUserId,
          idOriginal: this.userId,
          username: this.username,
          indexUser: this.indexUser,
          connectionId: 'conn-local',
          isCloseCoAuthoring: false,
          view: false,
        };

        this.editor.sendMessageToOO({ type: 'authChanges', changes: [] } satisfies OO.OOServerEventAuthChanges);

        this.editor.sendMessageToOO({
          type: 'auth',
          result: 1,
          sessionId: 'local-session',
          participants: [participant],
          locks: [],
          changesIndex: 0,
          indexUser: this.indexUser,
          buildVersion: '5.2.6',
          buildNumber: 2,
          licenseType: 3,
          settings: {
            binaryChanges: true,
          },
        } satisfies OO.OOServerEventAuth);
        break;

      case 'isSaveLock':
        // Grant the save lock immediately so `asc_Save`'s `askSaveChanges`
        // resolves without the ~10 s timeout.
        this.editor.sendMessageToOO({ type: 'saveLock', saveLock: false } satisfies OO.OOServerEventSaveLock);
        break;

      case 'unSaveLock':
        // Cancellation of an in-progress save: release the save lock and
        // acknowledge with -1 indices (RFC 1030 §2.2).
        this.editor.sendMessageToOO({ type: 'unSaveLock', index: -1, time: -1, syncChangesIndex: -1 } satisfies OO.OOServerEventUnSaveLock);
        break;

      case 'saveChanges':
        // Only need to drive the client's save state machine: chunk acks + final unSaveLock

        if (msg.startSaveChanges) {
          // The save point of this save is the change index it starts at
          // (the previous total).
          this.saveChangesIndex = this.syncChangesIndex;
        }
        // Since we have enabled `binaryChanges` (see `auth` event), the changes
        // are provided as an array instead of being JSON-serialized.
        this.syncChangesIndex += (msg.changes as Array<Uint8Array>).length;
        if (msg.endSaveChanges) {
          this.editor.sendMessageToOO({
            type: 'unSaveLock',
            index: this.saveChangesIndex,
            time: Date.now(),
            syncChangesIndex: this.syncChangesIndex,
          } satisfies OO.OOServerEventUnSaveLock);
        } else {
          // Acknowledge the chunk so the client emits the next one (the -1
          // index leaves the client's save point unchanged).
          this.editor.sendMessageToOO({
            type: 'savePartChanges',
            changesIndex: msg.startSaveChanges ? this.saveChangesIndex : -1,
            syncChangesIndex: this.syncChangesIndex,
          } satisfies OO.OOServerEventSavePartChanges);
        }
        break;

      case 'getLock':
        // Element locks (cell ranges, slide objects, ...) are requested by the
        // editors even in single-user mode (the edit is applied optimistically
        // and the reply updates the lock table).

        const blocks = (typeof msg.block === 'string' ? [msg.block] : msg.block) || [];
        const locks: { [key: string]: { user: string; time: number; block: unknown } } = {};
        for (let i = 0; i < blocks.length; i++) {
          const b = blocks[i];
          const key = typeof b === 'object' && b !== null ? b.guid : b;
          if (key !== undefined) {
            locks[key] = {
              user: this.compositeUserId,
              time: typeof b === 'object' && b !== null && b.time !== undefined ? b.time : Date.now(),
              block: b,
            };
          }
        }
        this.editor.sendMessageToOO({ type: 'getLock', locks: locks } satisfies OO.OOServerEventGetLock);
        break;

      case 'unLockDocument':
        // Fire-and-forget cleanup (there is no other participant to notify
        // and no auth lock to manage in single-user offline mode); only an
        // in-progress save needs an answer (RFC 1030 §2.2).
        if (msg.isSave) {
          this.editor.sendMessageToOO({
            type: 'unSaveLock',
            index: -1,
            time: -1,
            syncChangesIndex: -1,
          } satisfies OO.OOServerEventUnSaveLock);
        }
        break;

      case 'openDocument':
        console.log('openDocument', msg);
        // cspell: ignore imgurls
        if (msg.message.c === 'imgurls') {
          // e.g.:
          // {
          //   "type": "openDocument",
          //   "message": {
          //     "id": "778b08ca879aa0da2fed",
          //     "c": "imgurls",
          //     "userid": "a11cec00100000000000000000000000",
          //     "saveindex": 18,
          //     "data": [
          //       "http://foo.com/bar/spam.png"
          //     ]
          //   }
          // }

          const urls = msg.message.data as string[];
          if (urls.length !== 1) {
            console.error('Expected a single URL in imgurls');
          }
          const url = urls[0];
          const fileName = url.split('/').pop();
          fetch(url).then(
            async (success) => {
              console.log('openDocument fetched ok', success);

              if (!success.ok) {
                this.editor.sendMessageToOO({
                  type: 'documentOpen',
                  status: 'ok', // Also "ok" when the original URL couldn't be fetched...
                  data: {
                    // @ts-expect-error
                    error: 1,
                    // TODO: how to pass the error message to the GUI ?
                  },
                } satisfies OO.OOServerEventDocumentOpen);
                return;
              }

              const content = await success.blob();
              this.editor.sendMessageToOO({
                type: 'documentOpen',
                status: 'ok', // Also "ok" when the original URL couldn't be fetched...
                data: {
                  // @ts-expect-error
                  error: 0, // `0`: no error
                  urls: [
                    {
                      url: URL.createObjectURL(content),
                      path: `media/${fileName}`,
                    },
                  ],
                },
              } satisfies OO.OOServerEventDocumentOpen);
            },

            (error) => {
              console.log('openDocument fetched error', error);
              this.editor.sendMessageToOO({
                type: 'documentOpen',
                status: 'ok', // Also "ok" when the original URL couldn't be fetched...
                data: {
                  // @ts-expect-error
                  error: 1,
                  // TODO: how to pass the error message to the GUI ?
                },
              } satisfies OO.OOServerEventDocumentOpen);
            },
          );
        }
        break;

      default:
        // Other message types (cursor, meta, authChangesAck, ...) are not
        // relevant in single-user offline mode: ignore them.
        console.debug('ignored message', msg);
        break;
    }
  }
}

// --- editor state --------------------------------------------------------

// Expose the OnlyOffice API globally since tests need to drive the editor
declare global {
  interface Window {
    ooDocEditor: OO.DocEditor | null;
    ooDocumentReady: boolean;
  }
}

window.ooDocEditor = null;
window.ooDocumentReady = false;

function getApi(): OO.OOApi | null {
  return window.ooDocEditor ? window.ooDocEditor.getApi() : null;
}

// --- save protocol -------------------------------------------------------

// In-flight save tracking: used to signal the completion of a
// parent-initiated save request (`oo-save-request`) once the last save
// triggered by it settles.
let saveInflight = 0;
let manualSavePending = false;
let lastSaveError: string | null = null;

// Ask the parent window (only he has access to libparsec) to persist the document
function requestParentSave(bytes: Uint8Array): Promise<EditicsRequestParentSaveReply> {
  return new Promise(function (resolve) {
    const channel = new MessageChannel();
    channel.port1.onmessage = function (ev: MessageEvent<EditicsRequestParentSaveReply>) {
      resolve(ev.data ?? { success: false, error: 'empty save reply' });
    };
    parentWindow.postMessage({ command: 'oo-save', data: bytes }, PARENT_ORIGIN, [channel.port2]);
  });
}

// Actual save operation (the OnlyOffice editor called us with an export of the document)
async function onSaveBytes(binDocumentContent: Uint8Array, options: EditicsOpenOptions): Promise<void> {
  saveInflight += 1;
  lastSaveError = null;
  try {
    const documentContent = await convertFromNativeFormat(binDocumentContent, options.documentName, options.documentExtension);
    const result = await requestParentSave(documentContent);
    if (!result || result.success !== true) {
      lastSaveError = (result && result.error) || 'save failed';
    }
  } catch (e) {
    lastSaveError = String(e);
  } finally {
    saveInflight -= 1;
    if (manualSavePending && saveInflight === 0) {
      manualSavePending = false;
      postToParent({
        command: 'oo-save-result',
        success: !lastSaveError,
        error: lastSaveError ?? undefined,
      });
    }
  }
  if (lastSaveError) {
    throw new Error(lastSaveError);
  }
}

// The parent asks for a save (the Parsec editor's save-on-close path, see
// FileEditor.vue's save()). Completion is signaled back with 'oo-save-result'.
function onSaveRequest(): void {
  if (!window.ooDocEditor || !window.ooDocumentReady) {
    // Nothing editable was ever opened (or it failed to).
    postToParent({ command: 'oo-save-result', success: true, nothingToSave: true });
    return;
  }
  const api = getApi();
  const modified = api && typeof api.isDocumentModified === 'function' ? api.isDocumentModified() : true;
  if (!modified) {
    postToParent({ command: 'oo-save-result', success: true, nothingToSave: true });
    return;
  }
  manualSavePending = true;
  // Goes through the wrapped `api.asc_Save`: serializeBinary ->
  // onSave(bytes) -> parent write -> editor save handshake.
  window.ooDocEditor.save();
}

// --- image insertion -------------------------------------------------------
//
// When the user inserts an image into the document, the editor calls the
// `AddImage` host hook (see `window.APP.AddImage`). The file lives in the
// Parsec workspace, which only the parent can read (libparsec), so the host
// asks the parent to prompt for the workspace-absolute path of the file (e.g.
// `/foo/bar.png`) and to load its content. Once received, the content is
// handed to the editor as a data URL so it doesn't need to fetch anything.

type EditicsImageReply = Extract<EditicsParentToHostMessage, { command: 'oo-insert-image-result' }>;

// Pending `AddImage` requests, waiting for the parent's `oo-insert-image-result`.
const pendingImageRequests = new Map<number, (reply: EditicsImageReply) => void>();
let imageRequestCounter = 0;

function requestParentImage(): Promise<{ fileName?: string; data: Uint8Array }> {
  return new Promise((resolve, reject) => {
    const requestId = ++imageRequestCounter;
    pendingImageRequests.set(requestId, (reply) => {
      if (reply.data !== undefined) {
        resolve({ fileName: reply.fileName, data: reply.data });
      } else {
        reject(new Error(reply.error ?? 'no image data'));
      }
    });
    postToParent({ command: 'oo-insert-image', requestId });
  });
}

function bytesToDataUrl(bytes: Uint8Array, fileName?: string): string {
  const mimeType = fileName ? imageMimeType(fileName) : 'application/octet-stream';
  return `data:${mimeType};base64,${bytes.toBase64()}`;
}

// --- editor dirty state (for the Parsec topbar save indicator) -----------

let lastModified = false;

function notifySaveState(): void {
  const api = getApi();
  const modified = api && typeof api.isDocumentModified === 'function' ? !!api.isDocumentModified() : false;
  if (modified === lastModified) {
    return;
  }
  lastModified = modified;
  postToParent({ command: 'oo-save-state', state: modified ? 'unsaved' : 'saved' });
}

function trackSaveState(): void {
  const api = getApi();
  if (!api) {
    return;
  }
  // Immediate updates where the editor fires the event. NB: not every
  // build/path does (the cell editor in particular doesn't fire it when adding
  // a worksheet), hence the polling fallback below.
  if (typeof api.attachEvent === 'function') {
    api.attachEvent('asc_onDocumentModifiedChanged', notifySaveState);
  }
  // Polling fallback: query the editor's own dirty state periodically.
  // The topbar indicator is not latency-critical, 1s is plenty.
  setInterval(notifySaveState, 1000);
}

// --- open ---------------------------------------------------------------

function buildConfig(options: EditicsOpenOptions): OO.DocEditorConfig {
  return {
    offline: true,
    // 0 disables the SDK's periodic autosave entirely (the value is only
    // tested against zero, it is not an interval).
    autosave: 0,
    width: '100%',
    height: '100%',
    documentType: options.documentType,
    document: {
      title: options.documentName,
      // A truthy placeholder URL: the vanilla `_checkConfigParams` rejects an
      // empty one. It is never fetched: the document bytes come from
      // `loadBinary()` below.
      // Dummy placeholder URL (since an empty one is rejected). This is never
      // fetched though, since the document bytes come from the `loadBinary()` call.
      url: 'parsec://offline-document',
      permissions: {
        edit: options.mode === 'edit',
        // TODO: Download-as / print would need an x2t conversion by the host (see
        // the deferred save-as work). Keep them hidden for now.
        download: false,
        print: false,
      },
    },
    editorConfig: {
      mode: options.mode,
      lang: options.locale,
      user: {
        id: options.userId,
        name: options.userName,
      },
      customization: {
        compactHeader: false,
        chat: false,
        comments: false,
        help: false,
        about: false,
        feedback: false,
        anonymous: { request: false },
        uiTheme: options.theme === 'dark' ? 'theme-dark' : 'theme-classic-light',
      },
    },
    events: {
      onAppReady: function () {
        postToParent({ command: 'oo-app-ready' });
      },
      onDocumentReady: function () {
        window.ooDocumentReady = true;
        trackSaveState();
        postToParent({ command: 'oo-ready' });
      },
      onSave: async function (bytes: Uint8Array) {
        await onSaveBytes(bytes, options);
      },
      onError: function (event: { data?: unknown }) {
        postToParent({
          command: 'oo-error',
          details: event && event.data !== undefined ? JSON.stringify(event.data) : String(event),
        });
      },
    },
  };
}

// `openDocument` is expected to be called only once since it loads OnlyOffice
// scripts and set global variables (e.g. `window.APP`, `window.ooDocEditor`).
async function openDocument(options: EditicsOpenOptions, documentContent: Uint8Array): Promise<void> {
  const prepareDocumentPromise = prepareDocumentContent(options, documentContent);

  await loadScript(ONLYOFFICE_API_URL);

  // 1. Create & initialize the editor

  const config = buildConfig(options);
  const editor = new window.DocsAPI.DocEditor('placeholder');
  const mockServer = new OfflineMockServer(editor, {
    user: { id: options.userId, username: options.userName },
  });
  window.APP = {
    // TODO: PDF print is disabled for now
    onPrintPdf: (_dataContainer: unknown, cb: (result: unknown) => void) => {
      cb(null);
    },
    // TODO: Download-as is disabled for now
    onDownloadAs: (_dataContainer: unknown, cb: (result: unknown) => void) => {
      cb(null);
    },
    // Resolves the media names referenced by the document
    getImageURL: (name: string, callback: (url: string) => void) => {
      // The name can in fact be a self-containing data URL (i.e. `data:[<media-type>][;base64],<data>`).
      // This is typically the case when inserting a new image during the edition session.
      if (name.startsWith('data:')) {
        callback(name);
      } else {
        callback(getMediaUrl(name));
      }
    },

    AddImage: (callback: (res: { url: string; [key: string]: unknown }) => void, errorCallback: () => void) => {
      // The parent prompts for the path of a workspace file and loads it; the
      // editor gets a data URL so it doesn't have to fetch the file itself.
      requestParentImage().then(
        (reply) => {
          // Don't use `URL.createObjectURL` here: on document save, this URL is
          // persisted verbatim into the saved Editor.bin, then x2t only supports
          // data URL (and hence the image would vanish from the document).
          const url = bytesToDataUrl(reply.data, reply.fileName);
          callback({ url });
        },
        () => errorCallback(),
      );
    },
  };
  await editor.init(config, mockServer);
  window.ooDocEditor = editor;

  // 2. Load the document and start the actual edition

  // Without a license the editor idles in WaitAuth with no permissions and
  // never opens the document.
  editor.sendMessageToOO({
    type: 'license',
    license: {
      type: 3,
      mode: 0,
      rights: 1,
      buildVersion: '7.3.3',
      buildNumber: 8,
    },
  } satisfies OO.OOServerEventLicense);

  const binDocumentContent = await prepareDocumentPromise;
  editor.loadBinary(binDocumentContent);
}

window.addEventListener('message', function (event: MessageEvent) {
  // Filter messages that are not from the parent window.
  if (event.source !== parentWindow || event.origin !== PARENT_ORIGIN) {
    return;
  }
  const data = event.data as EditicsParentToHostMessage | undefined | null;
  if (!data) {
    return;
  }
  switch (data.command) {
    case 'oo-open':
      openDocument(data.options, data.documentContent).catch(function (err: Error) {
        postToParent({ command: 'oo-error', details: JSON.stringify(err) });
      });
      break;

    case 'oo-save-request':
      onSaveRequest();
      break;

    case 'oo-insert-image-result': {
      const resolver = pendingImageRequests.get(data.requestId);
      if (resolver) {
        pendingImageRequests.delete(data.requestId);
        resolver(data);
      }
      break;
    }
  }
});

postToParent({ command: 'oo-host-ready' });

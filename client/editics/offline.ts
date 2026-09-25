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
const ONLYOFFICE_API_URL = new URL('../onlyoffice/web-apps/apps/api/documents/api.js', window.location.href).href;
const X2T_SCRIPT_URL = new URL('../onlyoffice-x2t/x2t.js', window.location.href).href;

const parentWindow = window.parent;

function postToParent(message: EditicsHostToParentMessage): void {
  parentWindow.postMessage(message, '*');
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

interface X2TFileSystem {
  mkdir: (path: string) => void;
  writeFile: (path: string, data: Uint8Array | string) => void;
  readFile: (path: string) => Uint8Array;
}

interface X2TModule {
  FS: X2TFileSystem;
  ccall: (name: string, returnType: string, argTypes: string[], args: unknown[]) => number;
  onRuntimeInitialized?: () => void;
}

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

function runConversion(module: X2TModule, fileName: string, data: Uint8Array, outputFormat: string): Uint8Array {
  module.FS.writeFile(`/working/${fileName}`, data);

  const outputFileName = `${fileName}.${outputFormat}`;
  const params =
    '<?xml version="1.0" encoding="utf-8"?>' +
    '<TaskQueueDataConvert xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">' +
    `<m_sFileFrom>/working/${fileName}</m_sFileFrom>` +
    '<m_sThemeDir>/working/themes</m_sThemeDir>' +
    `<m_sFileTo>/working/${outputFileName}</m_sFileTo>` +
    '<m_bIsNoBase64>false</m_bIsNoBase64>' +
    '</TaskQueueDataConvert>';
  module.FS.writeFile('/working/params.xml', params);

  module.ccall('main1', 'number', ['string'], ['/working/params.xml']);
  return module.FS.readFile(`/working/${outputFileName}`);
}

async function convertToNativeFormat(data: Uint8Array, fileName: string, extension: string): Promise<Uint8Array> {
  const module = await loadX2T();
  const safeName = sanitizeFileName(fileName);
  const intermediaryFormat = ODF_INTERMEDIARY_FORMAT[extension];

  if (intermediaryFormat) {
    const intermediaryData = runConversion(module, safeName, data, intermediaryFormat);
    return runConversion(module, `${safeName}.${intermediaryFormat}`, intermediaryData, NATIVE_EXTENSION);
  }
  return runConversion(module, safeName, data, NATIVE_EXTENSION);
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

        this.editor.sendMessageToOO({ type: 'authChanges', changes: [] });

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
        });
        break;

      case 'isSaveLock':
        // Grant the save lock immediately so `asc_Save`'s `askSaveChanges`
        // resolves without the ~10 s timeout.
        this.editor.sendMessageToOO({ type: 'saveLock', saveLock: false });
        break;

      case 'unSaveLock':
        // Cancellation of an in-progress save: release the save lock and
        // acknowledge with -1 indices (RFC 1030 §2.2).
        this.editor.sendMessageToOO({ type: 'unSaveLock', index: -1, time: -1, syncChangesIndex: -1 });
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
          });
        } else {
          // Acknowledge the chunk so the client emits the next one (the -1
          // index leaves the client's save point unchanged).
          this.editor.sendMessageToOO({
            type: 'savePartChanges',
            changesIndex: msg.startSaveChanges ? this.saveChangesIndex : -1,
            syncChangesIndex: this.syncChangesIndex,
          });
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
        this.editor.sendMessageToOO({ type: 'getLock', locks: locks });
        break;

      case 'unLockDocument':
        // Fire-and-forget cleanup (there is no other participant to notify
        // and no auth lock to manage in single-user offline mode); only an
        // in-progress save needs an answer (RFC 1030 §2.2).
        if (msg.isSave) {
          this.editor.sendMessageToOO({ type: 'unSaveLock', index: -1, time: -1, syncChangesIndex: -1 });
        }
        break;

      default:
        // Other message types (cursor, meta, authChangesAck, ...) are not
        // relevant in single-user offline mode: ignore them.
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
    parentWindow.postMessage({ command: 'oo-save', data: bytes }, '*', [channel.port2]);
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
      // Print and Download-as are bridged by the wrapper onto `window.APP`
      // instead of POSTing to a converter endpoint. They are disabled in the
      // permissions above; end the editor action cleanly in case the code path
      // still triggers.
      onPrintPdf: function (_dataContainer: unknown, cb: (result: unknown) => void) {
        cb(null);
      },
      onDownloadAs: function (_dataContainer: unknown, cb: (result: unknown) => void) {
        cb(null);
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

async function openDocument(options: EditicsOpenOptions, documentContent: Uint8Array): Promise<void> {
  const prepareDocumentPromise = prepareDocumentContent(options, documentContent);

  await loadScript(ONLYOFFICE_API_URL);

  const config = buildConfig(options);
  const editor = new window.DocsAPI.DocEditor('placeholder');
  const mockServer = new OfflineMockServer(editor, {
    user: { id: options.userId, username: options.userName },
  });
  await editor.init(config, mockServer);
  window.ooDocEditor = editor;

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
  });

  const binDocumentContent = await prepareDocumentPromise;
  editor.loadBinary(binDocumentContent);
}

window.addEventListener('message', function (event: MessageEvent) {
  if (event.source !== parentWindow) {
    return;
  }
  const data = event.data as EditicsParentToHostMessage | undefined | null;
  if (!data) {
    return;
  }
  switch (data.command) {
    case 'oo-open':
      openDocument(data.options, data.documentContent).catch(function (err: Error) {
        postToParent({ command: 'oo-error', details: err.message });
      });
      break;
    case 'oo-save-request':
      onSaveRequest();
      break;
  }
});

postToParent({ command: 'oo-host-ready' });

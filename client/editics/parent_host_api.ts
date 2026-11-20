// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

// Types of the postMessage protocol between the editics host page
// (`editics/offline.ts`, loaded in an iframe) and its parent window
// (`src/services/editics.ts`).
//
// Two kinds of messages travel between the host and the parent:
//
// - `oo-event-*`: fire-and-forget notifications, no reply is expected;
//
// - `oo-rpc-*`: a request which is answered through the MessagePort
//   transferred along with the message (the requester keeps one end and
//   resolves with the payload the responder posts on the other end): each
//   request below has a matching `EditicsRpc*Reply` interface describing
//   that payload, with its fields set on success and `error` otherwise.

export type EditicsDocumentTypes = 'word' | 'cell' | 'slide' | 'pdf';
export type EditicsOpenModes = 'view' | 'edit';

export interface EditicsCollaborativeSession {
  // Base URL of the Parsec server hosting the session, without a trailing slash
  // (e.g. https://editics.app.parsec.cloud).
  serverUrl: string;
}

export interface EditicsOpenOptions {
  documentName: string;
  // File extension of `documentContent` as stored in Parsec. The host
  // converts non-native formats to OnlyOffice's native format with x2t.
  documentExtension: string;
  documentType: EditicsDocumentTypes;
  userName: string;
  userId: string;
  mode: EditicsOpenModes;
  locale: string;
  theme: 'light' | 'dark';
  collaborative?: EditicsCollaborativeSession;
}

// Replied to `oo-rpc-save` by the parent, and to `oo-rpc-request-save` by the
// host (the two save RPCs are distinct: the former actually persists a
// document in the workspace, the latter asks the host to run a full save).
// `nothingToSave` is only ever set by `oo-rpc-request-save`, when there was
// nothing modified to write back.
export interface EditicsRpcSaveReply {
  success: boolean;
  error?: string;
  nothingToSave?: boolean;
}

// Returned by the parent on the port of an `oo-rpc-insert-image` message: the
// image's name and content on success, why the insert failed otherwise.
export interface EditicsRpcImageReply {
  fileName?: string;
  data?: Uint8Array;
  error?: string;
}

// Returned by the host on the port of an `oo-rpc-convert-to-pdf` message: the
// PDF file on success, why the conversion failed otherwise.
export interface EditicsRpcConvertToPdfReply {
  data?: Uint8Array;
  error?: string;
}

export type EditicsHostToParentMessage =
  | { command: 'oo-event-host-ready' }
  | { command: 'oo-event-app-ready' }
  | { command: 'oo-event-ready' }
  | { command: 'oo-event-error'; details: string }
  // The editor's dirty state changed, so the parent can keep its save
  // indicator up to date.
  | { command: 'oo-event-save-state'; state: 'unsaved' | 'saved' }
  // The host serialized the document to OnlyOffice's native format and asks
  // the parent (the only side with libparsec access) to persist it in the
  // workspace. Replied with `EditicsRpcSaveReply`.
  | { command: 'oo-rpc-save'; data: Uint8Array }
  // The user wants to insert an image into the document: the parent should
  // display a file picker, load the selected file and return its content.
  // Replied with `EditicsRpcImageReply`.
  | { command: 'oo-rpc-insert-image' };

export type EditicsParentToHostMessage =
  | { command: 'oo-event-open'; options: EditicsOpenOptions; documentContent: Uint8Array }
  // The parent asks the host to run a full save of the document (serialize
  // it, persist it through `oo-rpc-save`, then run the editor's save
  // handshake): this is what the parent's save button does.
  // Replied with `EditicsRpcSaveReply`.
  | { command: 'oo-rpc-request-save' }
  // The app theme changed while the editor is open.
  | { command: 'oo-event-set-theme'; theme: EditicsOpenOptions['theme'] }
  // The parent asks the host to export the document's current state (saved
  // or not) to PDF. Replied with `EditicsRpcConvertToPdfReply`.
  | { command: 'oo-rpc-convert-to-pdf' };

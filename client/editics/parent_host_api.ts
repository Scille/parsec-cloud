// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import { DeviceID, VlobID } from '@/plugins/libparsec';

// Types of the postMessage protocol between the editics host page
// (`editics/index.html`, loaded in an iframe) and its parent window
// (`src/services/editics.ts`).
//
// Two kinds of messages travel between the host and the parent:
//
// - `oo-event-*`: fire-and-forget notifications, no reply is expected.
//
// - `oo-rpc-*`: a request which is answered through the MessagePort
//   transferred along with the message.
//   Each request below has a matching `EditicsRpc*Reply` interface describing
//   that payload, with its fields set on success and `error` otherwise.

export type EditicsDocumentTypes = 'word' | 'cell' | 'slide' | 'pdf';
export type EditicsOpenModes = 'view' | 'edit';

export interface EditicsCollaborativeOptions {
  // Base URL of the Parsec server hosting the session, without a trailing slash
  // (e.g. https://editics.app.parsec.cloud).
  serverUrl: string;
  organizationId: string;
  realmId: string;
  // Document ID is basically the vlob ID (it is named differently since the
  // server doesn't enforce the ID to be an actual vlob, which is useful to
  // start an editics session on a document before its initial outbound sync).
  documentId: VlobID;
}

export interface EditicsOpenOptions {
  documentName: string;
  // File extension of the document as stored in Parsec. The host converts
  // non-native formats to OnlyOffice's native format with x2t.
  // The content itself is fetched from the parent on demand (see
  // `oo-rpc-get-document-content`).
  documentExtension: string;
  documentType: EditicsDocumentTypes;
  authorLabel: string;
  authorId: DeviceID;
  mode: EditicsOpenModes;
  locale: string;
  theme: 'light' | 'dark';
  collaborative?: EditicsCollaborativeOptions;
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

export interface EditicsRpcImageReply {
  fileName?: string;
  data?: Uint8Array;
  error?: string;
}

export interface EditicsRpcDocumentContentReply {
  data?: Uint8Array;
  error?: string;
}

export interface EditicsRpcConvertToPdfReply {
  data?: Uint8Array;
  error?: string;
}

export interface EditicsRpcParticipantNameReply {
  participantName?: string;
}

export interface EditicsRpcEncryptBatchReply {
  keyIndex?: number;
  flatCiphertexts?: Uint8Array;
  offsets?: number[];
  error?: string;
}

export interface EditicsRpcDecryptBatchReply {
  flatCleartexts?: Uint8Array;
  offsets?: number[];
  error?: string;
}

export interface EditicsRpcTokenReply {
  token?: string;
  error?: string;
}

export type EditicsHostToParentMessage =
  // The host IFrame is ready to receive messages from the parent
  | { command: 'oo-event-host-ready' }
  // The OnlyOffice editor is initialized and ready to load the document
  | { command: 'oo-event-app-ready' }
  // The OnlyOffice editor has finished to load the document
  | { command: 'oo-event-document-ready' }
  | { command: 'oo-event-error'; details: string }
  // The editor's dirty state changed, so the parent can keep its save
  // indicator up to date.
  | { command: 'oo-event-save-state'; state: 'unsaved' | 'saved' }
  // The host serialized the document to OnlyOffice's native format and asks
  // the parent (the only side with libparsec access) to persist it in the
  // workspace.
  // Replied with `EditicsRpcSaveReply`.
  | { command: 'oo-rpc-save'; data: Uint8Array }
  // The user wants to insert an image into the document: the parent should
  // display a file picker, load the selected file and return its content.
  // Replied with `EditicsRpcImageReply`.
  | { command: 'oo-rpc-insert-image' }
  // Replied with `EditicsRpcParticipantNameReply`.
  | { command: 'oo-rpc-get-participant-name'; deviceId: DeviceID }
  // Replied with `EditicsRpcEncryptBatchReply`.
  | { command: 'oo-rpc-encrypt-batch'; flatCleartexts: Uint8Array; offsets: number[] }
  // Replied with `EditicsRpcDecryptBatchReply`.
  | { command: 'oo-rpc-decrypt-batch'; keyIndex: number; flatCiphertexts: Uint8Array; offsets: number[] }
  // Replied with `EditicsRpcTokenReply`.
  | { command: 'oo-rpc-generate-auth-token' }
  // Replied with `EditicsRpcDocumentContentReply`.
  | { command: 'oo-rpc-get-document-content' };

export type EditicsParentToHostMessage =
  // The host page asked for its document (see `oo-event-host-ready`), the
  // parent opens it.
  | { command: 'oo-event-open'; options: EditicsOpenOptions }
  // The app theme changed while the editor is open.
  | { command: 'oo-event-set-theme'; theme: EditicsOpenOptions['theme'] }
  // The parent asks the host to run a full save of the document (serialize
  // it, persist it through `oo-rpc-save`, then run the editor's save
  // handshake): this is what the parent's save button does.
  // Replied with `EditicsRpcSaveReply`.
  | { command: 'oo-rpc-request-save' }
  // The parent asks the host to export the document's current state (saved
  // or not) to PDF.
  // Replied with `EditicsRpcConvertToPdfReply`.
  | { command: 'oo-rpc-convert-to-pdf' };

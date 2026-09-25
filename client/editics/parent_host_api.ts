// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Types of the postMessage protocol between the editics host page
// (`editics/offline.ts`, loaded in an iframe) and its parent window
// (`src/services/editics.ts`).

export type EditicsDocumentTypes = 'word' | 'cell' | 'slide' | 'pdf';
export type EditicsOpenModes = 'view' | 'edit';

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
  theme?: 'light' | 'dark';
}

// Reply sent back by the parent to the host on the MessagePort transferred with the `oo-save` message.
export interface EditicsRequestParentSaveReply {
  success: boolean;
  error?: string;
}

export type EditicsHostToParentMessage =
  | { command: 'oo-host-ready' }
  | { command: 'oo-app-ready' }
  | { command: 'oo-ready' }
  | { command: 'oo-error'; details: string }
  // The editor's dirty state changed, so the parent can keep its save
  // indicator up to date.
  | { command: 'oo-save-state'; state: 'unsaved' | 'saved' }
  // The host serialized the document to OnlyOffice's native format and asks
  // the parent to persist it (a MessagePort is transferred with the message
  // to reply on, see `SaveReply`).
  | { command: 'oo-save'; data: Uint8Array }
  // Completion of an `oo-save-request`: `nothingToSave` is set when there was
  // simply nothing modified to write back.
  | { command: 'oo-save-result'; success: boolean; error?: string; nothingToSave?: boolean };

export type EditicsParentToHostMessage =
  { command: 'oo-open'; options: EditicsOpenOptions; documentContent: Uint8Array } | { command: 'oo-save-request' };

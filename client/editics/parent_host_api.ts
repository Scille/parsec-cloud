// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Types of the postMessage protocol between the editics host pages
// (`editics/offline.ts` & `editics/collaborative.ts`, loaded in an iframe) and
// its parent window (`src/services/editics.ts`).

export type EditicsDocumentTypes = 'word' | 'cell' | 'slide' | 'pdf';
export type EditicsOpenModes = 'view' | 'edit';

// Description of the collaborative editics session to join.
//
// Only provided when the document is opened in collaborative mode (its
// absence means offline mode, see `EditicsOpenOptions.collaborative`).
// Everything the host needs to reach the editics session hosted by the Parsec
// server: the parent is the only side with access to libparsec, so it is the
// one resolving the workspace/document IDs and providing the authentication
// token (the host iframe is deliberately isolated from the app's data).
export interface EditicsCollaborativeSession {
  // Base URL of the Parsec server hosting the session (e.g.
  // `http://localhost:6777`), without a trailing slash. The session routes
  // live under `/authenticated/{organizationId}/editics/sessions/...`.
  serverUrl: string;
  organizationId: string;
  // Hex ID of the workspace (a.k.a. realm) the document belongs to.
  workspaceId: string;
  // Hex ID of the document's vlob (also used as the session ID, RFC §1.3).
  vlobId: string;
  // Version of the document's vlob `documentContent` was loaded from.
  vlobVersion: number;
  // Hex ID of the device joining the session (identifies the participant).
  deviceIdHex: string;
  // Token authenticating the HTTP requests to the editics server (e.g.
  // `PARSEC-EDITICS-SIGN-ED25519.<device_id_hex>.<timestamp>.<signature>`).
  token: string;
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
  theme?: 'light' | 'dark';
  // Only set in collaborative mode: describes the editics session to join.
  // The parent serves a different host page depending on the mode (see
  // `getEditicsOrigin` in `src/services/editics.ts`), so the host also uses
  // this field as a sanity check.
  collaborative?: EditicsCollaborativeSession;
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
  | { command: 'oo-save-result'; success: boolean; error?: string; nothingToSave?: boolean }
  // The user wants to insert an image into the document: the parents should
  // display a file picker to the user, load the selected file and return its
  // content to the host.
  | { command: 'oo-insert-image'; requestId: number }
  // The host needs the display name of a participant: the editics protocol
  // deliberately doesn't carry user names (the server is not trusted for
  // them, RFC §3.3), so the parent resolves the device ID through libparsec.
  // A missing/empty `userName` in the reply makes the host fall back on a
  // placeholder name.
  | { command: 'oo-get-user-name'; requestId: number; deviceIdHex: string };

export type EditicsParentToHostMessage =
  | { command: 'oo-open'; options: EditicsOpenOptions; documentContent: Uint8Array }
  | { command: 'oo-save-request' }
  | { command: 'oo-insert-image-result'; requestId: number; fileName?: string; data?: Uint8Array; error?: string }
  | { command: 'oo-get-user-name-result'; requestId: number; userName?: string };

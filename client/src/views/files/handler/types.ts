// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

export enum FileHandlerMode {
  View = 'view',
  Edit = 'edit',
}

export enum SaveState {
  None = 'none',
  Unsaved = 'unsaved',
  Saving = 'saving',
  Saved = 'saved',
  Error = 'save-error',
  Offline = 'offline',
}

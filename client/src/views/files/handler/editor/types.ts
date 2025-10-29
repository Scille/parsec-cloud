// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export enum SaveState {
  None = 'none',
  Unsaved = 'unsaved',
  Saving = 'saving',
  Saved = 'saved',
  Error = 'save-error',
  Offline = 'offline',
}

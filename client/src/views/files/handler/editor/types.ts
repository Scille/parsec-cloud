// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export enum SaveState {
  None = 'none',
  Unsaved = 'unsaved',
  Saving = 'saving',
  Saved = 'saved',
  Error = 'save-error',
  Offline = 'offline',
}

export enum EditorIssueStatus {
  UnsupportedFileType = 'unsupported_file_type',
  NetworkOffline = 'network_offline',
  EditionNotAvailable = 'edition_not_available',
  LoadingTimeout = 'loading_timeout',
}

export enum EditorErrorTitle {
  GenericError = 'fileViewers.errors.titles.genericError',
  UnsupportedFileType = 'fileViewers.errors.titles.unsupportedFileType',
  EditionNotAvailable = 'fileViewers.errors.titles.editionNotAvailable',
  NetworkOffline = 'fileEditors.errors.titles.networkOffline',
  TooLongToOpen = 'fileEditors.errors.titles.tooLongToOpen',
}

export enum EditorErrorMessage {
  EditableOnlyOnSystem = 'fileViewers.errors.informationEditDownload',
  NetworkOffline = 'fileEditors.errors.networkOfflineMessage',
  TooLongToOpenOnWeb = 'fileEditors.errors.tooLongToOpenOnWeb',
  TooLongToOpenOnDesktop = 'fileEditors.errors.tooLongToOpenOnDesktop',
}

export enum EditorButtonAction {
  BackToFiles = 'fileEditors.actions.backToFiles',
  Close = 'fileEditors.actions.close',
  Wait = 'fileEditors.actions.wait',
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export enum EditorIssueStatus {
  UnsupportedFileType = 'unsupported_file_type',
  NetworkOffline = 'network_offline',
  EditionNotAvailable = 'edition_not_available',
  LoadingTimeout = 'loading_timeout',
  CollaborativeUnavailable = 'collaborative_unavailable',
}

export enum EditorErrorTitle {
  GenericError = 'fileViewers.errors.titles.genericError',
  UnsupportedFileType = 'fileViewers.errors.titles.unsupportedFileType',
  EditionNotAvailable = 'fileViewers.errors.titles.editionNotAvailable',
  NetworkOffline = 'fileEditors.errors.titles.networkOffline',
  TooLongToOpen = 'fileEditors.errors.titles.tooLongToOpen',
  CollaborativeUnavailable = 'fileEditors.errors.titles.collaborativeUnavailable',
}

export enum EditorErrorMessage {
  EditableOnlyOnSystem = 'fileViewers.errors.informationEditDownload',
  NetworkOffline = 'fileEditors.errors.networkOfflineMessage',
  TooLongToOpenOnWeb = 'fileEditors.errors.tooLongToOpenOnWeb',
  TooLongToOpenOnDesktop = 'fileEditors.errors.tooLongToOpenOnDesktop',
  CollaborativeUnavailable = 'fileEditors.errors.collaborativeUnavailableMessage',
}

export enum EditorButtonAction {
  BackToFiles = 'fileEditors.actions.backToFiles',
  Close = 'fileEditors.actions.close',
  Wait = 'fileEditors.actions.wait',
  OpenOfflineMode = 'fileEditors.actions.openOfflineMode',
}

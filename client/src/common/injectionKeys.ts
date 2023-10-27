// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DateTime } from 'luxon';

export interface Formatters {
  timeSince(date: DateTime | undefined, defaultValue?: string, format?: string): string;
  fileSize(bytes: number): string;
}

const FormattersKey = 'formatters';
const StorageManagerKey = 'storageManager';
const NotificationKey = 'notification';
const ImportKey = 'import';

export {
  FormattersKey,
  StorageManagerKey,
  NotificationKey,
  ImportKey,
};

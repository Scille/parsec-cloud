// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { DateTime } from 'luxon';

export interface Formatters {
  timeSince(date: DateTime | undefined, defaultValue?: string, format?: string): string;
  fileSize(bytes: number): string;
}

const FormattersKey = 'formatters';
const StorageManagerKey = 'storageManager';
const ConfigPathKey = 'configPath';
const NotificationKey = 'notification';

export {
  FormattersKey,
  StorageManagerKey,
  ConfigPathKey,
  NotificationKey,
};

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { InjectionKey } from 'vue';
import { DateTime } from 'luxon';
import { StorageManager } from '@/services/storageManager';

export interface Formatters {
  timeSince(date: DateTime | undefined, defaultValue?: string, format?: string): string;
  fileSize(bytes: number): string;
}

const FormattersKey = Symbol('formatters') as InjectionKey<Formatters>;
const StorageManagerKey = Symbol('storageManager') as InjectionKey<StorageManager>;
const ConfigPathKey = Symbol('configPath') as InjectionKey<string>;

export {
  FormattersKey,
  StorageManagerKey,
  ConfigPathKey
};

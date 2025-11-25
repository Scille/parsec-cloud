// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DateTime } from 'luxon';

export type LogLevel = 'debug' | 'info' | 'warn' | 'error';

export interface LogEntry {
  message: string;
  timestamp: string;
  level: LogLevel;
}

export function formatLogEntry(entry: LogEntry): string {
  return `[${entry.timestamp}] [${entry.level}] ${entry.message}`;
}

class _WebLogger {
  private static readonly DB_NAME = 'parsec_logs';
  private static readonly DB_STORE_NAME = 'logs';
  private static readonly DB_VERSION = 1;
  private static readonly MAX_ENTRIES = 1000;
  private db: IDBDatabase | undefined = undefined;

  constructor() {}

  async init(): Promise<void> {
    this.db = await this.openDB();
    const tr = this.db.transaction(_WebLogger.DB_STORE_NAME, 'readwrite');
    this.purgeOld(tr);
  }

  async openDB(): Promise<IDBDatabase> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open(_WebLogger.DB_NAME, _WebLogger.DB_VERSION);

      request.onerror = (): void => {
        reject(request.error);
      };

      request.onsuccess = (): void => {
        resolve(request.result);
      };

      request.onupgradeneeded = (): void => {
        const db = request.result;
        if (!db.objectStoreNames.contains(_WebLogger.DB_STORE_NAME)) {
          db.createObjectStore(_WebLogger.DB_STORE_NAME, {
            keyPath: 'id',
            autoIncrement: true,
          });
        }
      };
    });
  }

  async log(level: LogLevel, message: string): Promise<void> {
    if ((window as any).TESTING === true) {
      return;
    }
    if (!this.db) {
      return;
    }
    const tr = this.db.transaction(_WebLogger.DB_STORE_NAME, 'readwrite');
    const store = tr.objectStore(_WebLogger.DB_STORE_NAME);

    const newEntry: LogEntry = { level: level, timestamp: DateTime.now().toISO(), message: message };
    store.add(newEntry);
    await this.purgeOld(tr);
  }

  async getEntries(): Promise<Array<LogEntry>> {
    if (!this.db) {
      return [];
    }
    const tr = this.db.transaction(_WebLogger.DB_STORE_NAME, 'readonly');
    const store = tr.objectStore(_WebLogger.DB_STORE_NAME);
    const query = store.getAll();

    return new Promise((resolve, reject) => {
      query.onsuccess = (): void => resolve(query.result as Array<LogEntry>);
      query.onerror = (): void => reject(query.error);
    });
  }

  async purgeOld(transaction: IDBTransaction): Promise<void> {
    if (!this.db || transaction.mode !== 'readwrite') {
      return;
    }
    const store = transaction.objectStore(_WebLogger.DB_STORE_NAME);
    const entries: Array<IDBValidKey> = [];
    const query = store.openCursor();

    query.onsuccess = (event): void => {
      const cursor = (event.target as IDBRequest<IDBCursorWithValue>).result;
      if (cursor) {
        entries.push(cursor.key);
        cursor.continue();
      } else {
        const limit = entries.length - _WebLogger.MAX_ENTRIES;
        if (limit > 0) {
          const toDelete = entries.slice(0, limit);
          for (const key of toDelete) {
            store.delete(key);
          }
        }
      }
    };
  }
}

export const WebLogger = new _WebLogger();

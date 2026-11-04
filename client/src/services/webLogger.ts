// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

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

export class WebLogger {
  private static readonly DB_STORE_NAME = 'logs';
  private static readonly DB_VERSION = 1;
  private static readonly LOG_LEVELS_WEIGHTS = { debug: 1, info: 2, warn: 3, error: 4, all: 5 };
  // Entries are buffered and written in a single transaction, either after this delay
  // or as soon as the buffer reaches `FLUSH_MAX_ENTRIES`.
  private static readonly FLUSH_DELAY_MS = 500;
  private static readonly FLUSH_MAX_ENTRIES = 200;

  private dbName;
  private maxEntries;
  private db: IDBDatabase | undefined = undefined;
  private threshold: LogLevel | 'all';
  private buffer: Array<LogEntry> = [];
  private flushTimeout: ReturnType<typeof setTimeout> | undefined = undefined;

  constructor(dbName: string = 'parsec_logs', maxEntries: number = 2000) {
    this.dbName = dbName;
    this.maxEntries = maxEntries;
    this.threshold = 'info';
  }

  async init(): Promise<void> {
    this.db = await this.openDB();
    // Also writes the entries logged before the database was opened
    await this.flush();
  }

  async openDB(): Promise<IDBDatabase> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open(this.dbName, WebLogger.DB_VERSION);

      request.onerror = (): void => {
        reject(request.error);
      };

      request.onsuccess = (): void => {
        resolve(request.result);
      };

      request.onupgradeneeded = (): void => {
        const db = request.result;
        if (!db.objectStoreNames.contains(WebLogger.DB_STORE_NAME)) {
          db.createObjectStore(WebLogger.DB_STORE_NAME, {
            keyPath: 'id',
            autoIncrement: true,
          });
        }
      };
    });
  }

  setThreshold(level: LogLevel | 'all'): void {
    this.threshold = level;
  }

  isLevelEnabled(level: LogLevel): boolean {
    return WebLogger.LOG_LEVELS_WEIGHTS[level] >= WebLogger.LOG_LEVELS_WEIGHTS[this.threshold];
  }

  log(level: LogLevel, message: string): void {
    if (!this.isLevelEnabled(level)) {
      return;
    }
    this.buffer.push({ level: level, timestamp: DateTime.utc().toISO(), message: message });
    // Don't grow indefinitely if the database is never opened
    if (this.buffer.length > this.maxEntries) {
      this.buffer.splice(0, this.buffer.length - this.maxEntries);
    }
    if (this.buffer.length >= WebLogger.FLUSH_MAX_ENTRIES) {
      this.flush();
    } else if (this.flushTimeout === undefined) {
      this.flushTimeout = setTimeout(() => this.flush(), WebLogger.FLUSH_DELAY_MS);
    }
  }

  async flush(): Promise<void> {
    if (this.flushTimeout !== undefined) {
      clearTimeout(this.flushTimeout);
      this.flushTimeout = undefined;
    }
    if (!this.db || this.buffer.length === 0) {
      return;
    }
    const entries = this.buffer;
    this.buffer = [];

    const tr = this.db.transaction(WebLogger.DB_STORE_NAME, 'readwrite');
    const store = tr.objectStore(WebLogger.DB_STORE_NAME);
    for (const entry of entries) {
      store.add(entry);
    }
    this.purgeOld(store);

    // Errors are silently ignored: logging them with `console` could end up calling
    // this logger again.
    return new Promise((resolve) => {
      tr.oncomplete = (): void => resolve();
      tr.onerror = (): void => resolve();
      tr.onabort = (): void => resolve();
    });
  }

  async getEntries(): Promise<Array<LogEntry>> {
    await this.flush();
    if (!this.db) {
      return [];
    }
    const tr = this.db.transaction(WebLogger.DB_STORE_NAME, 'readonly');
    const store = tr.objectStore(WebLogger.DB_STORE_NAME);
    const query = store.getAll();

    return new Promise((resolve, reject) => {
      query.onsuccess = (): void => resolve(query.result as Array<LogEntry>);
      query.onerror = (): void => reject(query.error);
    });
  }

  private purgeOld(store: IDBObjectStore): void {
    const countQuery = store.count();

    countQuery.onsuccess = (): void => {
      const excess = countQuery.result - this.maxEntries;
      if (excess <= 0) {
        return;
      }
      // Keys are auto-incremented, so the oldest entries come first: skip `excess`
      // entries to find the first key to keep, then delete everything before it.
      const cursorQuery = store.openKeyCursor();
      let advanced = false;
      cursorQuery.onsuccess = (): void => {
        const cursor = cursorQuery.result;
        if (!cursor) {
          return;
        }
        if (!advanced) {
          advanced = true;
          cursor.advance(excess);
        } else {
          store.delete(IDBKeyRange.upperBound(cursor.key, true));
        }
      };
    };
  }
}

function formatConsoleArg(arg: unknown): string {
  if (typeof arg === 'string') {
    return arg;
  }
  if (arg instanceof Error) {
    return arg.stack ?? arg.toString();
  }
  try {
    return JSON.stringify(arg) ?? String(arg);
  } catch {
    return String(arg);
  }
}

// Forward everything printed with `console` to `logger`. libparsec web logs end up
// in `console` (through the `console_log` crate), so this is how we get to store them.
export function captureConsole(logger: WebLogger): void {
  // `console_log` maps Rust levels as: trace -> `console.debug`, debug -> `console.log`
  const methods: Array<[keyof Console, LogLevel]> = [
    ['debug', 'debug'],
    ['log', 'debug'],
    ['info', 'info'],
    ['warn', 'warn'],
    ['error', 'error'],
  ];
  let inLogger = false;

  for (const [method, level] of methods) {
    const original = (console[method] as (...args: Array<unknown>) => void).bind(console);
    (console as any)[method] = (...args: Array<unknown>): void => {
      original(...args);
      // Guard against the logger itself printing something
      if (inLogger) {
        return;
      }
      inLogger = true;
      try {
        if (!logger.isLevelEnabled(level)) {
          return;
        }
        logger.log(level, args.map(formatConsoleArg).join(' '));
      } finally {
        inLogger = false;
      }
    };
  }
}

export const GUILogger = new WebLogger('parsec_gui_logs');
export const libparsecLogger = new WebLogger('libparsec_logs', 10000);

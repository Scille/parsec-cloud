// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

interface CacheTuple<T> {
  data: T;
  created: number;
}

const DEFAULT_KEEP_ALIVE = 1000 * 60 * 5; // 5min default

export class DataCache<K, V> {
  data: Map<K, CacheTuple<V>>;
  keepAlive: number;

  constructor(keepAlive: number = DEFAULT_KEEP_ALIVE) {
    this.data = new Map<K, CacheTuple<V>>();
    this.keepAlive = keepAlive;
  }

  get(key: K): V | undefined {
    const tuple = this.data.get(key);

    if (!tuple) {
      return undefined;
    }
    if (tuple.created + this.keepAlive < Date.now()) {
      this.data.delete(key);
      return undefined;
    }
    return tuple.data;
  }

  set(key: K, value: V): void {
    this.data.set(key, { data: value, created: Date.now() });
  }

  has(key: K): boolean {
    return this.data.has(key);
  }
}

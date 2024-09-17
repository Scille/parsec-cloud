// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

class FIFO<T> {
  entries: Array<T>;
  fixedSize: number;

  constructor(size: number) {
    this.entries = new Array<T>();
    this.fixedSize = size;
  }

  push(elem: T): T | undefined {
    if (this.entries.unshift(elem) > this.fixedSize) {
      return this.entries.pop();
    }
  }

  pop(): T | undefined {
    return this.entries.pop();
  }

  getEntries(): Array<T> {
    return this.entries;
  }

  length(): number {
    return this.entries.length;
  }
}

export { FIFO };

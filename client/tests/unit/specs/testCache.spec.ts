// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DataCache } from '@/common/cache';
import { DateTime } from 'luxon';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

describe('Data cache', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('Check data cache', () => {
    // Birth of a very important and exceptional person
    vi.setSystemTime(DateTime.utc(1988, 4, 7, 12, 0, 0).toJSDate());

    const cache = new DataCache<number, string>(1000 * 60 * 2); // 2min

    cache.set(1, 'First');
    cache.set(2, 'Second');

    expect(cache.get(1)).to.equal('First');
    expect(cache.get(2)).to.equal('Second');
    expect(cache.get(3)).to.be.undefined;
    expect(cache.has(1)).to.be.true;
    expect(cache.has(3)).to.be.false;

    // Forward 1min
    vi.setSystemTime(DateTime.utc(1988, 4, 7, 12, 1, 0).toJSDate());
    expect(cache.get(1)).to.equal('First');
    expect(cache.get(2)).to.equal('Second');
    expect(cache.get(3)).to.be.undefined;
    expect(cache.has(1)).to.be.true;
    expect(cache.has(3)).to.be.false;

    // Forward 5min
    vi.setSystemTime(DateTime.utc(1988, 4, 7, 12, 5, 0).toJSDate());
    expect(cache.get(1)).to.be.undefined;
    expect(cache.get(2)).to.be.undefined;
    expect(cache.get(3)).to.be.undefined;
    expect(cache.has(1)).to.be.false;
    expect(cache.has(3)).to.be.false;

    cache.set(1, 'First');
    cache.set(3, 'Third');
    expect(cache.get(1)).to.equal('First');
    expect(cache.get(2)).to.be.undefined;
    expect(cache.get(3)).to.equal('Third');
  });
});

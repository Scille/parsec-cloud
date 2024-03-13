// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { formatTimeSince } from '@/common/date';
import { DateTime } from 'luxon';
import { it, vi } from 'vitest';

describe('Date formatting', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it.each([
    [DateTime.utc(1988, 4, 7, 11, 59, 43), 'now', 'long'],
    [DateTime.utc(1988, 4, 7, 11, 59, 20), '< 1 minute', 'long'],
    [DateTime.utc(1988, 4, 7, 11, 57, 0), '3 minutes ago', 'long'],
    [DateTime.utc(1988, 4, 7, 11, 57, 20), '2 minutes ago', 'long'],
    [DateTime.utc(1988, 4, 7, 10, 0, 0), '2 hours ago', 'long'],
    [DateTime.utc(1988, 4, 7, 1, 0, 0), '11 hours ago', 'long'],
    [DateTime.utc(1988, 4, 2, 3, 0, 0), '5 days ago', 'long'],
    [DateTime.utc(1988, 3, 30, 3, 0, 0), /^Wednesday, March 30, 1988 at \d:00 AM$/, 'long'],
    [DateTime.utc(1988, 3, 30, 3, 0, 0), 'Mar 30, 1988', 'short'],
    [DateTime.utc(1958, 3, 30, 3, 0, 0), 'Mar 30, 1958', 'short'],
  ])('Formats the time since', (date: DateTime, expected: RegExp | string, format: string) => {
    // Birth of a very important and exceptional person
    vi.setSystemTime(DateTime.utc(1988, 4, 7, 12, 0, 0).toJSDate());

    if (expected instanceof RegExp) {
      expect(formatTimeSince(date, '', format as 'long' | 'short')).to.match(expected);
    } else {
      expect(formatTimeSince(date, '', format as 'long' | 'short')).to.equal(expected);
    }
  });

  it('Uses default value', () => {
    expect(formatTimeSince(undefined)).to.equal('');
    expect(formatTimeSince(undefined, 'Default Value')).to.equal('Default Value');
  });

  it('Round at day', () => {
    vi.setSystemTime(DateTime.utc(1988, 4, 7, 12, 0, 0).toJSDate());
    expect(formatTimeSince(DateTime.utc(1988, 4, 7, 11, 59, 30), '', 'long', true)).to.equal('Today');
    expect(formatTimeSince(DateTime.utc(1988, 4, 7, 10, 0, 0), '', 'long', true)).to.equal('Today');
    expect(formatTimeSince(DateTime.utc(1988, 4, 6, 10, 0, 0), '', 'long', true)).to.equal('Yesterday');
    expect(formatTimeSince(DateTime.utc(1988, 4, 2, 10, 0, 0), '', 'long', true)).to.equal('5 days ago');
  });
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { formatTimeSince } from '@/common/date';
import { it, vi } from 'vitest';
import { DateTime } from 'luxon';

function mockT(translationString: string, args?: object, count?: number): string {
  return `${translationString} ${JSON.stringify(args)} ${count}`;
}

function mockD(date: Date, format: 'long' | 'short'): string {
  const dateTime = DateTime.fromJSDate(date, {zone: 'UTC'});
  // Doesn't really matter how date is displayed, we're not testing this
  if (format === 'long') {
    return `${dateTime.day}/${dateTime.month}/${dateTime.year} ${dateTime.hour}:${dateTime.minute}:${dateTime.second}`;
  }
  return `${dateTime.day}/${dateTime.month}/${dateTime.year}`;
}

describe('Date formatting', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it.each([
    [DateTime.utc(1988, 4, 7, 11, 59, 30), 'common.date.lastLoginSeconds {"seconds":30} 30', 'long'],
    [DateTime.utc(1988, 4, 7, 11, 59, 58), 'common.date.lastLoginSeconds {"seconds":2} 2', 'long'],
    [DateTime.utc(1988, 4, 7, 11, 57, 0), 'common.date.lastLoginMinutes {"minutes":3} 3', 'long'],
    [DateTime.utc(1988, 4, 7, 11, 57, 20), 'common.date.lastLoginMinutes {"minutes":2} 2', 'long'],
    [DateTime.utc(1988, 4, 7, 10, 0, 0), 'common.date.lastLoginHours {"hours":2} 2', 'long'],
    [DateTime.utc(1988, 4, 7, 1, 0, 0), 'common.date.lastLoginHours {"hours":11} 11', 'long'],
    [DateTime.utc(1988, 4, 2, 3, 0, 0), 'common.date.lastLoginDays {"days":5} 5', 'long'],
    [DateTime.utc(1988, 3, 30, 3, 0, 0), '30/3/1988 3:0:0', 'long'],
    [DateTime.utc(1988, 3, 30, 3, 0, 0), '30/3/1988', 'short'],
    [DateTime.utc(1958, 3, 30, 3, 0, 0), '30/3/1958', 'short'],
  ])('Formats the time since', (date: DateTime, expected: string, format: string) => {
    // Birth of a very important and exceptional person
    vi.setSystemTime(DateTime.utc(1988, 4, 7, 12, 0, 0).toJSDate());

    expect(formatTimeSince(date, mockT as any, mockD as any, '', format as 'long' | 'short')).to.equal(expected);
  });

  it('Uses default value', () => {
    expect(formatTimeSince(undefined, mockT as any, mockD as any)).to.equal('');
    expect(formatTimeSince(undefined, mockT as any, mockD as any, 'Default Value')).to.equal('Default Value');
  });

  it('Round at day', () => {
    vi.setSystemTime(DateTime.utc(1988, 4, 7, 12, 0, 0).toJSDate());
    expect(
      formatTimeSince(DateTime.utc(1988, 4, 7, 11, 59, 30), mockT as any, mockD as any, '', 'long', true),
    ).to.equal('common.date.lastLoginDays {"days":0} 0');
    expect(
      formatTimeSince(DateTime.utc(1988, 4, 7, 10, 0, 0), mockT as any, mockD as any, '', 'long', true),
    ).to.equal('common.date.lastLoginDays {"days":0} 0');
    expect(
      formatTimeSince(DateTime.utc(1988, 4, 6, 10, 0, 0), mockT as any, mockD as any, '', 'long', true),
    ).to.equal('common.date.lastLoginDays {"days":1} 1');
    expect(
      formatTimeSince(DateTime.utc(1988, 4, 2, 10, 0, 0), mockT as any, mockD as any, '', 'long', true),
    ).to.equal('common.date.lastLoginDays {"days":5} 5');
  });
});

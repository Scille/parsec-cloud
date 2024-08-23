// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { formatExpirationTime, generateTrialOrganizationName, getDurationBeforeExpiration, isExpired } from '@/common/organization';
import { DateTime, Duration } from 'luxon';
import { Translatable } from 'megashark-lib';
import { it, vi } from 'vitest';

describe('Trial organization', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it.each([
    ['gordon.freeman@blackmesa.nm', 'trial-gordon_free-1712491200000'],
    ['a@b.c', 'trial-a-1712491200000'],
    ['G0rd0n.FREEMAN+spam@blackmesa.nm', 'trial-G0rd0n_FREE-1712491200000'],
  ])('Gets a trial organization name from user email %s -> %s', (email, expectedOrgName) => {
    vi.setSystemTime(DateTime.utc(2024, 4, 7, 12, 0, 0).toJSDate());
    const orgName = generateTrialOrganizationName(email);
    expect(orgName).to.equal(expectedOrgName);
    expect(orgName.length).to.be.lessThan(32);
  });

  it.each([
    [
      DateTime.fromISO('2024-04-07T11:00:00.000-00:00', { zone: 'utc' }),
      Duration.fromObject({ days: 14, hours: 23 }),
      false,
      { key: 'HomePage.organizationList.expiration.days', count: 14, data: { days: 15 } },
    ],
    [
      DateTime.fromISO('2024-03-31T12:00:00.000-00:00', { zone: 'utc' }),
      Duration.fromObject({ days: 8, hours: 0 }),
      false,
      { key: 'HomePage.organizationList.expiration.days', count: 8, data: { days: 9 } },
    ],
    [
      DateTime.fromISO('2024-03-23T14:00:00.000-00:00', { zone: 'utc' }),
      Duration.fromObject({ days: 0, hours: 2 }),
      false,
      { key: 'HomePage.organizationList.expiration.hours', count: 2, data: { hours: 2 } },
    ],
    [
      DateTime.fromISO('2024-03-01T12:00:00.000-00:00', { zone: 'utc' }),
      Duration.fromObject({ days: -22, hours: 0 }),
      true,
      'HomePage.organizationList.expiration.expired',
    ],
  ])(
    'Get duration before expiration %s',
    (creationDate: DateTime, expectedDuration: Duration, shouldBeExpired: boolean, formatted: Translatable) => {
      vi.setSystemTime(DateTime.utc(2024, 4, 7, 12, 0, 0).toJSDate());
      const duration = getDurationBeforeExpiration(creationDate);
      expect(duration.days).to.equal(expectedDuration.days);
      expect(Math.floor(duration.hours)).to.equal(Math.floor(expectedDuration.hours));
      expect(isExpired(duration)).to.equal(shouldBeExpired);
      expect(formatExpirationTime(duration)).to.deep.equal(formatted);
    },
  );
});

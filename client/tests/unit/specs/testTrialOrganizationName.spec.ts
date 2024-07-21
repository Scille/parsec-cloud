// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { generateTrialOrganizationName } from '@/common/organization';
import { DateTime } from 'luxon';
import { it, vi } from 'vitest';

describe('Date formatting', () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.setSystemTime(DateTime.utc(2024, 4, 7, 12, 0, 0).toJSDate());
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it.each([
    ['gordon.freeman@blackmesa.nm', 'trial-gordon_free-1712491200000'],
    ['a@b.c', 'trial-a_b_c-1712491200000'],
    ['G0rd0n.FREEMAN+spam@blackmesa.nm', 'trial-G0rd0n_FREE-1712491200000'],
  ])('Gets a trial organization name from user email %s -> %s', (email, expectedOrgName) => {
    const orgName = generateTrialOrganizationName(email);
    expect(orgName).to.equal(expectedOrgName);
    expect(orgName.length).to.be.lessThan(32);
  });
});

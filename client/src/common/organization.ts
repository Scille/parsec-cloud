// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TRIAL_EXPIRATION_DAYS } from '@/services/parsecServers';
import { DateTime, Duration } from 'luxon';
import { Translatable } from 'megashark-lib';

export function generateTrialOrganizationName(userEmail: string): string {
  const timestamp = new Date().valueOf().toString();
  const part = userEmail.slice(0, 11).replaceAll(/[^a-zA-Z0-9-_]/g, '_');

  return `trial-${part}-${timestamp}`;
}

export function getDurationBeforeExpiration(creationTime: DateTime): Duration {
  return creationTime.plus({ days: TRIAL_EXPIRATION_DAYS }).diffNow(['days', 'hours']);
}

export function isExpired(duration: Duration): boolean {
  return duration.days <= 0 && duration.hours <= 0;
}

export function formatExpirationTime(duration: Duration): Translatable {
  if (duration.days > 0) {
    return {
      key: 'HomePage.organizationList.expiration.days',
      count: duration.days,
      data: { days: duration.days },
    };
  } else if (duration.hours > 0) {
    return {
      key: 'HomePage.organizationList.expiration.hours',
      count: Math.floor(duration.hours),
      data: { hours: Math.floor(duration.hours) },
    };
  }
  return { key: 'HomePage.organizationList.expiration.expired' };
}

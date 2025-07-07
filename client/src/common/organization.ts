// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { AvailableDevice } from '@/parsec';
import { ServerType, TRIAL_EXPIRATION_DAYS, getServerTypeFromHost } from '@/services/parsecServers';
import { DateTime, Duration } from 'luxon';
import { Translatable } from 'megashark-lib';

export function generateTrialOrganizationName(userEmail: string): string {
  const timestamp = new Date().valueOf().toString();
  const parts = userEmail.split('@', 2);
  const firstPart = parts.length > 1 ? parts[0] : userEmail;
  const userNamePart = firstPart.slice(0, 11).replaceAll(/[^a-zA-Z0-9-_]/g, '_');

  return `trial-${userNamePart}-${timestamp}`;
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
      data: { days: duration.days + 1 },
    };
  } else if (duration.hours > 0) {
    return {
      key: 'HomePage.organizationList.expiration.hours',
      count: Math.floor(duration.hours),
      data: { hours: Math.floor(duration.hours) },
    };
  }
  return 'HomePage.organizationList.expiration.expired';
}

export function isTrialOrganizationDevice(device: AvailableDevice): boolean {
  const url = new URL(device.serverUrl);
  const serverType = getServerTypeFromHost(url.hostname, url.port.length > 0 ? parseInt(url.port) : undefined);
  return serverType === ServerType.Trial;
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { formatDate, msTranslate } from '@/services/translation';
import { DateTime } from 'luxon';

export function formatTimeSince(
  date: DateTime | undefined,
  defaultValue = '',
  format: 'long' | 'short' = 'long',
  roundDays = false,
): string {
  if (!date) {
    return defaultValue;
  }
  // Get the difference in ms
  const diff = DateTime.now().diff(date, ['years', 'months', 'days', 'hours', 'minutes', 'seconds']).toObject();

  // More than 6 days, just display the date as is
  if (!diff || (diff.years && diff.years > 0) || (diff.months && diff.months > 0) || (diff.days && diff.days > 6)) {
    return formatDate(date, format);
  } else if (roundDays) {
    return msTranslate({ key: 'common.date.lastLoginDays', data: { days: diff.days || 0 }, count: diff.days || 0 });
  } else if (diff.days && diff.days > 0) {
    return msTranslate({ key: 'common.date.lastLoginDays', data: { days: diff.days }, count: diff.days });
  } else if (diff.hours && diff.hours > 0) {
    return msTranslate({ key: 'common.date.lastLoginHours', data: { hours: diff.hours }, count: diff.hours });
  } else if (diff.minutes && diff.minutes > 0) {
    return msTranslate({ key: 'common.date.lastLoginMinutes', data: { minutes: diff.minutes }, count: diff.minutes });
  } else if (diff.seconds && diff.seconds > 30) {
    return msTranslate('common.date.lessThanAMinute');
  } else {
    return msTranslate('common.date.fewSeconds');
  }
}

export async function startCounter(ms: number, step: number, callback: (elapsed: number) => Promise<void>): Promise<void> {
  await callback(0);
  let count = 0;
  const interval = setInterval(async () => {
    count += 1;
    if (count * step > ms) {
      clearInterval(interval);
      await callback(ms);
    } else {
      await callback(count * step);
    }
  }, step);
  return new Promise((resolve) => setTimeout(resolve, ms));
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { formatDate, translate } from '@/services/translation';
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
    return translate('common.date.lastLoginDays', { days: diff.days || 0 }, diff.days || 0);
  } else if (diff.days && diff.days > 0) {
    return translate('common.date.lastLoginDays', { days: diff.days }, diff.days);
  } else if (diff.hours && diff.hours > 0) {
    return translate('common.date.lastLoginHours', { hours: diff.hours }, diff.hours);
  } else if (diff.minutes && diff.minutes > 0) {
    return translate('common.date.lastLoginMinutes', { minutes: diff.minutes }, diff.minutes);
  } else if (diff.seconds && diff.seconds > 30) {
    return translate('common.date.lessThanAMinute');
  } else {
    return translate('common.date.fewSeconds');
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

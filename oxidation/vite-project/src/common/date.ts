// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { DateTime } from 'luxon';
import { ComposerTranslation } from 'vue-i18n';

export function formatTimeSince(date: DateTime | undefined, t: ComposerTranslation, d: any, defaultValue=''): string {
  if (!date) {
    return defaultValue;
  }
  // Get the difference in ms
  const diff = DateTime.now().diff(date, ['years', 'months', 'days', 'hours', 'minutes', 'seconds']).toObject();

  // More than 6 days, just display the date as is
  if (!diff || diff.years && diff.years > 0 || diff.months && diff.months > 0 || diff.days && diff.days > 6) {
    return d(date.toJSDate(), 'long');
  } else if (diff.days && diff.days > 0) {
    return t('common.date.lastLoginDays', {days: diff.days}, diff.days);
  } else if (diff.hours && diff.hours > 0) {
    return t('common.date.lastLoginHours', {hours: diff.hours}, diff.hours);
  } else if (diff.minutes && diff.minutes > 0) {
    return t('common.date.lastLoginMinutes', {minutes: diff.minutes}, diff.minutes);
  } else {
    // Math.trunc is needed because seconds returns a float (https://github.com/moment/luxon/issues/565)
    if (!diff.seconds) {
      diff.seconds = 1;
    }
    diff.seconds = Math.trunc(diff.seconds) || 1;
    return t('common.date.lastLoginSeconds', {seconds: diff.seconds}, diff.seconds);
  }
}

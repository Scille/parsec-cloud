// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { ComposerTranslation } from 'vue-i18n';

export function formatTimeSince(date: Date | undefined, t: ComposerTranslation, d: any, defaultValue=''): string {
  if (!date) {
    return defaultValue;
  }
  // Get the difference in ms
  let diff = Date.now().valueOf() - date.valueOf();

  // To seconds
  diff = Math.ceil(diff / 1000);
  if (diff < 60) {
    return t('common.date.lastLoginSeconds', {seconds: diff}, diff);
  }

  // To minutes
  diff = Math.ceil(diff / 60);
  if (diff < 60) {
    return t('common.date.lastLoginMinutes', {minutes: diff}, diff);
  }

  // To hours
  diff = Math.ceil(diff / 60);
  if (diff < 24) {
    return t('common.date.lastLoginHours', {hours: diff}, diff);
  }

  // To days
  diff = Math.ceil(diff / 24);
  if (diff < 7) {
    return t('common.date.lastLoginDays', {days: diff}, diff);
  }

  // Let's use the date as is
  return d(date, 'long');
}

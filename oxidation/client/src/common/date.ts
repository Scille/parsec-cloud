// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { ComposerTranslation } from 'vue-i18n';

export function formatTimeSince(date: Date | undefined, t: ComposerTranslation, d: any, defaultValue=''): string {
  if (!date) {
    return defaultValue;
  }
  // Get the difference in ms
  let diff = Date.now() - date.getTime();

  // Convert to seconds
  diff = Math.ceil(diff / 1000);
  if (diff < 60) {
    return t('common.date.lastLoginSeconds', {seconds: diff}, diff);
  }

  // Convert to minutes
  diff = Math.ceil(diff / 60);
  if (diff < 60) {
    return t('common.date.lastLoginMinutes', {minutes: diff}, diff);
  }

  // Convert to hours
  diff = Math.ceil(diff / 60);
  if (diff < 24) {
    return t('common.date.lastLoginHours', {hours: diff}, diff);
  }

  // Convert to days
  diff = Math.ceil(diff / 24);
  if (diff < 7) {
    return t('common.date.lastLoginDays', {days: diff}, diff);
  }

  return d(date, 'long');
}

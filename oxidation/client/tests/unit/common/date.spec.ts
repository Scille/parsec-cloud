// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { createI18n } from 'vue-i18n';
import frFR from '../../../src/locales/fr-FR.json';
import enUS from '../../../src/locales/en-US.json';
import { formatTimeSince } from '@/common/date';

describe('common.date.ts EN', () => {
  type MessageSchema = typeof enUS;
  const supportedLocales:{[key: string]: string} = {
    en: 'en-US',
    'en-US': 'en-US'
  };
  const i18n = createI18n<[MessageSchema], 'en-US'>({
    legacy: false,
    globalInjection: true,
    locale: 'en-US',
    messages: {
      'en-US': enUS
    },
    datetimeFormats: {
      'en-US': {
        short: {
          year: 'numeric',
          month: 'short',
          day: 'numeric'
        },
        long: {
          year: 'numeric',
          month: 'long',
          day: 'numeric',
          weekday: 'long',
          hour: 'numeric',
          minute: 'numeric'
        }
      }
    }
  });

  const t = i18n.global.t;
  const d = i18n.global.d;

  it('formats date as time elapsed since date in English', () => {
    const now = new Date('11/19/1998 08:00:00');

    jest.useRealTimers();
    // Way way in the past, should display the full date
    expect(formatTimeSince(now, t, d)).toBe('Thursday, November 19, 1998 at 8:00 AM');

    jest.useFakeTimers();
    // 3 days before
    jest.setSystemTime(new Date('11/22/1998 08:00:00'));
    expect(formatTimeSince(now, t, d)).toBe('3 days ago');
    // 1 day before
    jest.setSystemTime(new Date('11/20/1998 08:00:00'));
    expect(formatTimeSince(now, t, d)).toBe('one day ago');
    // 5 hours before
    jest.setSystemTime(new Date('11/19/1998 13:00:00'));
    expect(formatTimeSince(now, t, d)).toBe('5 hours ago');
    // 1 hour before
    jest.setSystemTime(new Date('11/19/1998 09:00:00'));
    expect(formatTimeSince(now, t, d)).toBe('one hour ago');
    // 30 minutes before
    jest.setSystemTime(new Date('11/19/1998 08:30:00'));
    expect(formatTimeSince(now, t, d)).toBe('30 minutes ago');
    // 1 minutes before
    jest.setSystemTime(new Date('11/19/1998 08:01:00'));
    expect(formatTimeSince(now, t, d)).toBe('one minute ago');
    // 30 secondes before
    jest.setSystemTime(new Date('11/19/1998 08:00:30'));
    expect(formatTimeSince(now, t, d)).toBe('30 seconds ago');
    // 1 second before
    jest.setSystemTime(new Date('11/19/1998 08:00:01'));
    expect(formatTimeSince(now, t, d)).toBe('one second ago');

    expect(formatTimeSince(undefined, t, d, 'no_value')).toBe('no_value');
  });
});

describe('common.date.ts FR', () => {
  type MessageSchema = typeof frFR;
  const supportedLocales:{[key: string]: string} = {
    fr: 'fr-FR',
    'fr-FR': 'fr-FR'
  };
  const i18n = createI18n<[MessageSchema], 'fr-FR'>({
    legacy: false,
    globalInjection: true,
    locale: supportedLocales['fr-FR'],
    messages: {
      'fr-FR': frFR
    },
    datetimeFormats: {
      'fr-FR': {
        short: {
          year: 'numeric',
          month: 'short',
          day: 'numeric'
        },
        long: {
          year: 'numeric',
          month: 'long',
          day: 'numeric',
          weekday: 'long',
          hour: 'numeric',
          minute: 'numeric'
        }
      }
    }
  });

  const t = i18n.global.t;
  const d = i18n.global.d;

  it('formats date as time elapsed since date in French', () => {
    const now = new Date('11/19/1998 08:00:00');

    jest.useRealTimers();
    // Way way in the past, should display the full date
    expect(formatTimeSince(now, t, d)).toBe('jeudi 19 novembre 1998 à 08:00');

    jest.useFakeTimers();

    // 3 days before
    jest.setSystemTime(new Date('11/22/1998 08:00:00'));
    expect(formatTimeSince(now, t, d)).toBe('il y a 3 jours');
    // 1 day before
    jest.setSystemTime(new Date('11/20/1998 08:00:00'));
    expect(formatTimeSince(now, t, d)).toBe('il y a un jour');
    // 5 hours before
    jest.setSystemTime(new Date('11/19/1998 13:00:00'));
    expect(formatTimeSince(now, t, d)).toBe('il y a 5 heures');
    // 1 hour before
    jest.setSystemTime(new Date('11/19/1998 09:00:00'));
    expect(formatTimeSince(now, t, d)).toBe('il y a une heure');
    // 30 minutes before
    jest.setSystemTime(new Date('11/19/1998 08:30:00'));
    expect(formatTimeSince(now, t, d)).toBe('il y a 30 minutes');
    // 1 minutes before
    jest.setSystemTime(new Date('11/19/1998 08:01:00'));
    expect(formatTimeSince(now, t, d)).toBe('il y a une minute');
    // 30 secondes before
    jest.setSystemTime(new Date('11/19/1998 08:00:30'));
    expect(formatTimeSince(now, t, d)).toBe('il y a 30 secondes');
    // 1 second before
    jest.setSystemTime(new Date('11/19/1998 08:00:01'));
    expect(formatTimeSince(now, t, d)).toBe('il y a une seconde');

    expect(formatTimeSince(undefined, t, d, 'aucune_value')).toBe('aucune_value');
  });
});

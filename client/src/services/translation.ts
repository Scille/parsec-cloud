// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import enUS from '@/locales/en-US.json';
import frFR from '@/locales/fr-FR.json';
import { InvitationStatus, UserProfile, WorkspaceRole } from '@/parsec';
import { DateTime } from 'luxon';
import { App } from 'vue';
import { createI18n } from 'vue-i18n';

let i18n: any | null = null;

export interface TranslationData {
  key: string;
  data?: object;
  count?: number;
}

export type Translatable = string | TranslationData;

export const TranslationPlugin = {
  install: (app: App<any>): void => {
    app.config.globalProperties.$msTranslate = (translatable: Translatable | undefined): string => {
      return translate(translatable);
    };
    app.provide('msTranslate', translate);
  },
};

export type Locale = 'fr-FR' | 'en-US';
export type DateFormat = 'long' | 'short';

export function initTranslations(locale?: Locale): any {
  /* I18n variables */
  // Type-define 'fr-FR' as the master schema for the resource
  type MessageSchema = typeof frFR;
  const supportedLocales: { [key: string]: string } = {
    fr: 'fr-FR',
    en: 'en-US',
    'fr-FR': 'fr-FR',
    'en-US': 'en-US',
  };
  const defaultLocale = 'fr-FR';
  i18n = createI18n<[MessageSchema], Locale>({
    legacy: false,
    globalInjection: true,
    locale: locale || supportedLocales[window.navigator.language] || defaultLocale,
    messages: {
      'fr-FR': frFR,
      'en-US': enUS,
    },
    datetimeFormats: {
      'en-US': {
        short: {
          year: 'numeric',
          month: 'short',
          day: 'numeric',
        },
        long: {
          year: 'numeric',
          month: 'long',
          day: 'numeric',
          weekday: 'long',
          hour: 'numeric',
          minute: 'numeric',
        },
      },
      'fr-FR': {
        short: {
          year: 'numeric',
          month: 'short',
          day: 'numeric',
        },
        long: {
          year: 'numeric',
          month: 'long',
          day: 'numeric',
          weekday: 'long',
          hour: 'numeric',
          minute: 'numeric',
        },
      },
    },
  });
  return i18n;
}

export function translate(content: Translatable | undefined): string {
  if (typeof content === 'undefined' || content === '') {
    return '';
  }

  const { t } = i18n.global;

  return typeof content === 'string' ? t(content) : t(content.key, content.data, content.count);
}

export function formatDate(date: DateTime, format: DateFormat = 'long'): Translatable {
  const { d } = i18n.global;

  // Bit of a trickery.
  // `d` from i18n returns an already formatted date as string
  // But in our case, we want it to be a Translatable instead so it can be processed
  // as every other translation.
  // We use a translation key that just returns the string given as parameter.
  return { key: 'common.date.asIs', data: { date: d(date.toJSDate(), format) } };
}

export function getProfileTranslationKey(profile: UserProfile): Translatable {
  if (profile === UserProfile.Admin) {
    return 'UsersPage.profile.admin.label';
  } else if (profile === UserProfile.Standard) {
    return 'UsersPage.profile.standard.label';
  } else if (profile === UserProfile.Outsider) {
    return 'UsersPage.profile.outsider.label';
  }
  return '';
}

interface WorkspaceRoleTranslations {
  label: Translatable;
  description?: Translatable;
}

export function getWorkspaceRoleTranslationKey(role: WorkspaceRole | null): WorkspaceRoleTranslations {
  switch (role) {
    case null: {
      return {
        label: 'workspaceRoles.none',
      };
    }
    case WorkspaceRole.Reader: {
      return {
        label: 'workspaceRoles.reader.label',
        description: 'workspaceRoles.reader.description',
      };
    }
    case WorkspaceRole.Contributor: {
      return {
        label: 'workspaceRoles.contributor.label',
        description: 'workspaceRoles.contributor.description',
      };
    }
    case WorkspaceRole.Manager: {
      return {
        label: 'workspaceRoles.manager.label',
        description: 'workspaceRoles.manager.description',
      };
    }
    case WorkspaceRole.Owner: {
      return {
        label: 'workspaceRoles.owner.label',
        description: 'workspaceRoles.owner.description',
      };
    }
  }
}

export function getInvitationStatusTranslationKey(status: InvitationStatus): Translatable {
  switch (status) {
    case InvitationStatus.Ready:
      return 'UsersPage.invitation.status.ready';
    case InvitationStatus.Idle:
      return 'UsersPage.invitation.status.idle';
    case InvitationStatus.Finished:
      return 'UsersPage.invitation.status.finished';
    case InvitationStatus.Cancelled:
      return 'UsersPage.invitation.status.cancelled';
  }
}

export function changeLocale(locale: Locale): void {
  i18n.global.locale.value = locale;
}

export function getLocale(): any {
  return i18n.global.locale.value;
}

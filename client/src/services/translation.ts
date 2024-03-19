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

export type Translatable = string | TranslationData | undefined;

export const TranslationPlugin = {
  install: (app: App<any>): void => {
    app.config.globalProperties.$msTranslate = (translatable: Translatable): string => {
      return msTranslate(translatable);
    };
    app.provide('msTranslate', msTranslate);
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

export function msTranslate(content: Translatable): string {
  if (typeof content === 'undefined' || content === '') {
    return '';
  }

  const { t } = i18n.global;

  return typeof content === 'string' ? t(content) : t(content.key, content.data, content.count);
}

export function formatDate(date: DateTime, format: DateFormat = 'long'): string {
  const { d } = i18n.global;

  return d(date.toJSDate(), format);
}

export function getLocale(): any {
  return i18n.global.locale.value;
}

interface WorkspaceRoleTranslation {
  label: string;
  description?: string;
}

export function translateProfile(profile: UserProfile): string {
  if (profile === UserProfile.Admin) {
    return msTranslate('UsersPage.profile.admin.label');
  } else if (profile === UserProfile.Standard) {
    return msTranslate('UsersPage.profile.standard.label');
  } else if (profile === UserProfile.Outsider) {
    return msTranslate('UsersPage.profile.outsider.label');
  }
  return '';
}

export function translateWorkspaceRole(role: WorkspaceRole | null): WorkspaceRoleTranslation {
  switch (role) {
    case null: {
      return {
        label: msTranslate('workspaceRoles.none'),
      };
    }
    case WorkspaceRole.Reader: {
      return {
        label: msTranslate('workspaceRoles.reader.label'),
        description: msTranslate('workspaceRoles.reader.description'),
      };
    }
    case WorkspaceRole.Contributor: {
      return {
        label: msTranslate('workspaceRoles.contributor.label'),
        description: msTranslate('workspaceRoles.contributor.description'),
      };
    }
    case WorkspaceRole.Manager: {
      return {
        label: msTranslate('workspaceRoles.manager.label'),
        description: msTranslate('workspaceRoles.manager.description'),
      };
    }
    case WorkspaceRole.Owner: {
      return {
        label: msTranslate('workspaceRoles.owner.label'),
        description: msTranslate('workspaceRoles.owner.description'),
      };
    }
  }
}

export function translateInvitationStatus(status: InvitationStatus): string {
  switch (status) {
    case InvitationStatus.Ready:
      return msTranslate('UsersPage.invitation.status.ready');
    case InvitationStatus.Idle:
      return msTranslate('UsersPage.invitation.status.idle');
    case InvitationStatus.Finished:
      return msTranslate('UsersPage.invitation.status.finished');
    case InvitationStatus.Cancelled:
      return msTranslate('UsersPage.invitation.status.cancelled');
  }
}

export function changeLocale(locale: Locale): void {
  i18n.global.locale.value = locale;
}

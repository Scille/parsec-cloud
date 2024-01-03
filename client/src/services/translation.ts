// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import enUS from '@/locales/en-US.json';
import frFR from '@/locales/fr-FR.json';
import { InvitationStatus, WorkspaceRole } from '@/parsec';
import { DateTime } from 'luxon';
import { createI18n } from 'vue-i18n';

let i18n: any | null = null;

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

export function translate(key: string, attrs?: object, count?: number): string {
  const { t } = i18n.global;

  return t(key, attrs, count);
}

export function formatDate(date: DateTime, format: DateFormat = 'long'): string {
  const { d } = i18n.global;

  return d(date.toJSDate(), format);
}

interface WorkspaceRoleTranslation {
  label: string;
  description?: string;
}

export function translateWorkspaceRole(role: WorkspaceRole | null): WorkspaceRoleTranslation {
  const { t } = i18n.global;

  switch (role) {
    case null: {
      return {
        label: t('workspaceRoles.none'),
      };
    }
    case WorkspaceRole.Reader: {
      return {
        label: t('workspaceRoles.reader.label'),
        description: t('workspaceRoles.reader.description'),
      };
    }
    case WorkspaceRole.Contributor: {
      return {
        label: t('workspaceRoles.contributor.label'),
        description: t('workspaceRoles.contributor.description'),
      };
    }
    case WorkspaceRole.Manager: {
      return {
        label: t('workspaceRoles.manager.label'),
        description: t('workspaceRoles.manager.description'),
      };
    }
    case WorkspaceRole.Owner: {
      return {
        label: t('workspaceRoles.owner.label'),
        description: t('workspaceRoles.owner.description'),
      };
    }
  }
}

export function translateInvitationStatus(status: InvitationStatus): string {
  const { t } = i18n.global;

  switch (status) {
    case InvitationStatus.Ready:
      return t('UsersPage.invitation.status.ready');
    case InvitationStatus.Idle:
      return t('UsersPage.invitation.status.idle');
    case InvitationStatus.Deleted:
      return t('UsersPage.invitation.status.deleted');
  }
}

export function changeLocale(locale: Locale): void {
  i18n.global.locale.value = locale;
}

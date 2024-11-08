// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import appEnUS from '@/locales/en-US.json';
import appFrFR from '@/locales/fr-FR.json';
import { mockI18n } from '@tests/component/support/mocks';
import { I18n } from 'megashark-lib';
import { beforeEach } from 'vitest';

beforeEach(() => {
  I18n.init({
    defaultLocale: 'en-US',
    customAssets: {
      'fr-FR': appFrFR,
      'en-US': appEnUS,
    },
  });
  mockI18n();
});

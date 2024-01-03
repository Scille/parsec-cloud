// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { initTranslations } from '@/services/translation';
import { mockI18n } from '@tests/component/support/mocks';

beforeEach(() => {
  initTranslations('en-US');
  mockI18n();
});

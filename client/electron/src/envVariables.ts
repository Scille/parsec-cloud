// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export const Env = {
  DISABLE_SENTRY: process.env.PARSEC_APP_IS_CSPN === 'true',
  DISABLE_UPDATES: process.env.PARSEC_APP_IS_CSPN === 'true',
  ENABLE_CUSTOM_BRANDING: process.env.PARSEC_APP_IS_CSPN !== 'true' && process.env.PARSEC_APP_ENABLE_CUSTOM_BRANDING === 'true',
};

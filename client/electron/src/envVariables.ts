// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

const HARDENED = process.env.PARSEC_APP_HARDENED === 'true';

export const Env = {
  HARDENED,
  DISABLE_SENTRY: HARDENED,
  DISABLE_UPDATES: HARDENED,
  ENABLE_CUSTOM_BRANDING: !HARDENED && process.env.PARSEC_APP_ENABLE_CUSTOM_BRANDING === 'true',
};

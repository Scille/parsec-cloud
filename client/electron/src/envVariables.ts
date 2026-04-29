// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { FEATURE_FLAGS } from './features.js';

export const Env = {
  ENABLE_CUSTOM_BRANDING: !FEATURE_FLAGS.hardened() && process.env.PARSEC_APP_ENABLE_CUSTOM_BRANDING === 'true',
};

// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import { FEATURE_FLAGS } from './features.js';

export const Env = {
  ENABLE_CUSTOM_BRANDING: !FEATURE_FLAGS.hardened() && process.env.PARSEC_APP_ENABLE_CUSTOM_BRANDING === 'true',
};

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { CustomPublishOptions, ParsecFeatures } from '../assets/publishConfig';

const DEFAULT_FEATURES: ParsecFeatures = {
  hardened: false,
};

export default class FeaturesFlag {
  private data: ParsecFeatures;

  constructor() {
    let data: CustomPublishOptions | undefined = undefined;
    try {
      data = require('../assets/publishConfig.json');
    } catch {}
    this.data = data ? data.features : DEFAULT_FEATURES;

    console.debug('Configured features:', this.data);
  }

  sentryEnabled(): boolean {
    return !this.hardened();
  }

  updatesEnabled(): boolean {
    return !this.hardened();
  }

  hardened(): boolean {
    return this.data.hardened;
  }
}

export const FEATURE_FLAGS = new FeaturesFlag();

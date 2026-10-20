// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

const { GitHubPublisher } = require('electron-publish/out/gitHubPublisher');

const VERSION = '3.11.0-rc.2+dev';

class CustomGitHubPublisher extends GitHubPublisher {
  /**
   *
   * @param {import('electron-publish').PublishContext} context
   * @param {import('./publishConfig').CustomPublishOptions} publishConfig
   */
  constructor(context, publishConfig) {
    super(
      context,
      {
        ...publishConfig,
        provider: 'github',
      },
      VERSION,
      {},
    );
    this.useSafeArtifactName = false;
  }
}

exports.default = CustomGitHubPublisher;

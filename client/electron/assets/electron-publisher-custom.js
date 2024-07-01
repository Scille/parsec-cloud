// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

const { GitHubPublisher } = require('electron-publish/out/gitHubPublisher');

const VERSION = '3.0.0-b.6.dev.19905+47c28a6';

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

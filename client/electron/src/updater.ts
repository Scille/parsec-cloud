// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import type { CustomPublishOptions, ProgressInfo, UpdateInfo, XElement } from 'builder-util-runtime';
import { newError, parseXml } from 'builder-util-runtime';
import log from 'electron-log';
import type { BaseUpdater, AppUpdater as _AppUpdater } from 'electron-updater';
import { CancellationToken } from 'electron-updater';
import { GitHubProvider, computeReleaseNotes } from 'electron-updater/out/providers/GitHubProvider';
import { parseUpdateInfo, type ProviderRuntimeOptions } from 'electron-updater/out/providers/Provider';
import { getChannelFilename, newUrlFromBase } from 'electron-updater/out/util';
import * as semver from 'semver';
import type { CustomPublishOptions as CustomGitHubOptions } from '../assets/publishConfig';

// Greatly inspired by (it isn't exported by `electron-updater`)
// https://github.com/electron-userland/electron-builder/blob/77f977435c99247d5db395895618b150f5006e8f/packages/electron-updater/src/providers/GitHubProvider.ts#L11-L13
interface GithubUpdateInfo extends UpdateInfo {
  tag: string;
}

interface GithubReleaseInfo {
  readonly tag_name: string;
}

interface GithubReleaseData {
  tag?: string;
  feedElement?: XElement;
}

enum ErrorCodes {
  InvalidReleaseFeed = 'ERR_UPDATER_INVALID_RELEASE_FEED',
  LatestVersionNotFound = 'ERR_UPDATER_LATEST_VERSION_NOT_FOUND',
  NoPublishedVersions = 'ERR_UPDATER_NO_PUBLISHED_VERSIONS',
  ChannelFileNotFound = 'ERR_UPDATER_CHANNEL_FILE_NOT_FOUND',
}

/** Match the tag part of a github tag link */
const HREF_REGEXP = /\/tag\/([^/]+)$/;

enum PreReleaseTypes {
  Nightly = 'nightly',
  Alpha = 'alpha',
  Beta = 'beta',
  ReleaseCandidate = 'rc',
}
const PRERELEASE_TYPES: string[] = [PreReleaseTypes.Nightly, PreReleaseTypes.Alpha, PreReleaseTypes.Beta];
const PRERELEASE_TYPE_ALIAS: Record<string, string> = {
  a: PreReleaseTypes.Alpha,
  b: PreReleaseTypes.Beta,
  rc: PreReleaseTypes.ReleaseCandidate,
};

/** Check if the selected `next` channel would "downgrade" the `current` channel (nightly -> alpha -> beta). */
function isChannelMissmatch(current: string, next: string): boolean {
  switch (current) {
    case PreReleaseTypes.Alpha:
      // We should not got back to nightly.
      return next === PreReleaseTypes.Nightly;
    case PreReleaseTypes.Beta:
      // We should not got back to nightly or alpha.
      return next === PreReleaseTypes.Nightly || next === PreReleaseTypes.Alpha;
    case PreReleaseTypes.ReleaseCandidate:
      return next === PreReleaseTypes.Nightly || next === PreReleaseTypes.Alpha || next === PreReleaseTypes.Beta;
    default:
      return false;
  }
}

// @ts-expect-error TS2415: GithubProvider don't expose
//  - `getChannelFilePrefix` method from `Provider` which we need to override.
//  - `getLatestVersion` method that we need to override to allow the use of the nightly channel.
class CustomGithubProvider extends GitHubProvider {
  private logger;
  constructor(
    protected readonly options: CustomGitHubOptions,
    protected readonly updater: _AppUpdater,
    protected readonly runtimeOptions: ProviderRuntimeOptions,
  ) {
    super(
      {
        ...options,
        provider: 'github',
      },
      updater,
      runtimeOptions,
    );
    this.logger = log.scope('github provider');
  }

  /**
   * This function is used to determine the channel file suffix.
   * e.g.: `latest-linux-x64.yml` for the `latest` channel for `linux` platform on 64-bit architecture.
   * @returns {string}
   */
  protected override getChannelFilePrefix() {
    const { machine } = require('node:os');
    const arch = process.env['TEST_UPDATER_ARCH'] || this.options.buildMachineArch || machine();

    switch (this.runtimeOptions.platform) {
      case 'linux':
        return `-linux-${arch}`;
      case 'darwin':
        return `-mac-${arch}`;
      case 'win32':
        return `-win-${arch}`;
      default:
        return `-${this.runtimeOptions.platform}-${arch}`;
    }
  }

  async getLatestVersion(): Promise<GithubUpdateInfo> {
    const cancellationToken = new CancellationToken();

    // 1. Fetch the latest release.
    this.logger.debug('Fetching release feed');
    const feed = await this.fetchReleaseFeed(cancellationToken);
    this.logger.debug('Finding latest release');
    const latestReleaseData = await this.findLatestRelease(feed, cancellationToken);

    if (latestReleaseData.tag === null) {
      throw newError('No published versions on GitHub', ErrorCodes.NoPublishedVersions);
    }

    this.logger.debug(`Latest release tag: ${latestReleaseData.tag}`);

    // 2. Fetch the channel file.
    let rawData: string;
    let channelFile = '';
    let channelFileUrl: any = '';
    this.logger.debug('Fetching channel file');
    const fetchData = async (channelName: string) => {
      channelFile = getChannelFilename(channelName);
      channelFileUrl = newUrlFromBase(this.getBaseDownloadPath(latestReleaseData.tag, channelFile), this.baseUrl);
      const requestOptions = this.createRequestOptions(channelFileUrl);
      try {
        return await this.executor.request(requestOptions, cancellationToken);
      } catch (e) {
        throw newError(
          `Cannot find ${channelFile} in the latest release artifacts (${channelFileUrl}): ${e.stack || e.message}`,
          ErrorCodes.ChannelFileNotFound,
        );
      }
    };

    try {
      const channel = this.updater.allowPrerelease
        ? this.getCustomChannelName(String(semver.prerelease(latestReleaseData.tag)?.[0]) || 'latest')
        : this.getDefaultChannelName();
      rawData = await fetchData(channel);
    } catch (e) {
      if (this.updater.allowPrerelease) {
        // Fallback to the default channel if the custom channel doesn't exist.
        rawData = await fetchData(this.getDefaultChannelName());
      } else {
        throw e;
      }
    }

    // 3. Parse the channel file.
    this.logger.debug('Parsing channel file');
    const result = parseUpdateInfo(rawData, channelFile, channelFileUrl);
    if (result.releaseName === null) {
      result.releaseName = latestReleaseData.feedElement.elementValueOrEmpty('title');
    }

    if (result.releaseNotes === null) {
      result.releaseNotes = computeReleaseNotes(
        this.updater.currentVersion,
        this.updater.fullChangelog,
        feed,
        latestReleaseData.feedElement,
      );
    }

    return {
      tag: latestReleaseData.tag,
      ...result,
    };
  }

  async fetchReleaseFeed(cancellationToken: CancellationToken): Promise<XElement> {
    const feedXml = await this.httpRequest(
      newUrlFromBase(`${this.basePath}.atom`, this.baseUrl),
      { accept: 'application/xml, application/atom+xml, text/xml, */*' },
      cancellationToken,
    );
    return parseXml(feedXml);
  }

  private async findLatestRelease(feed: XElement, cancellationToken: CancellationToken): Promise<GithubReleaseData> {
    let latestRelease: XElement | null = feed.element('entry', false, 'No published versions on GitHub');
    let tag: string | null = null;

    try {
      if (this.updater.allowPrerelease) {
        [tag, latestRelease] = this.findLatestPreRelease(tag, latestRelease, feed);
      } else {
        tag = await this.getLatestTagName(cancellationToken);
        for (const element of feed.getElements('entry')) {
          if (HREF_REGEXP.exec(element.element('link').attribute('href'))![1] === tag) {
            latestRelease = element;
            break;
          }
        }
      }
    } catch (e) {
      throw newError(`Cannot parse releases feed: ${e.stack || e.message},\nXML:\n${feed}`, ErrorCodes.InvalidReleaseFeed);
    }

    return { tag, feedElement: latestRelease };
  }

  private findLatestPreRelease(tag: string, latestRelease: XElement, feed: XElement): [string, XElement] {
    const logger = log.scope('findLatestPreRelease');
    const currentChannel = this.getCurrentChannel();
    logger.debug(`Current channel: ${currentChannel}`);

    // If we don't have a channel, we use the first entry in the feed.
    if (currentChannel === null) {
      tag = HREF_REGEXP.exec(latestRelease.element('link').attribute('href'))![1];
    } else {
      for (const element of feed.getElements('entry')) {
        const hrefElement = HREF_REGEXP.exec(element.element('link').attribute('href'));

        // Skip entry without link.href
        if (hrefElement === null) {
          continue;
        }

        // Current entry GitHub tag.
        const hrefTag = hrefElement[1];
        logger.debug(`hrefTag: ${hrefTag}`);

        const rawHrefChannel =
          hrefTag === PreReleaseTypes.Nightly ? PreReleaseTypes.Nightly : (semver.prerelease(hrefTag)?.[0] as string) || null;
        const hrefChannel = PRERELEASE_TYPE_ALIAS[rawHrefChannel] || rawHrefChannel;

        const shouldFetchVersion = !currentChannel || PRERELEASE_TYPES.includes(currentChannel);
        const isCustomChannel = hrefChannel !== null && !PRERELEASE_TYPES.includes(String(hrefChannel));

        const channelMismatch = isChannelMissmatch(currentChannel, hrefChannel);

        logger.debug({
          hrefChannel,
          shouldFetchVersion,
          isCustomChannel,
          channelMismatch,
        });
        if (shouldFetchVersion && !isCustomChannel && !channelMismatch) {
          latestRelease = element;
          tag = hrefTag;
          break;
        }

        const isNextPreRelease = hrefChannel && hrefChannel === currentChannel;
        logger.debug({ isNextPreRelease });
        if (isNextPreRelease) {
          latestRelease = element;
          tag = hrefTag;
          break;
        }
      }
    }
    return [tag, latestRelease];
  }

  private getCurrentChannel(): string {
    if (this.options.nightlyBuild) {
      return PreReleaseTypes.Nightly;
    }
    const rawCurrentChannel = this.updater?.channel || (semver.prerelease(this.updater.currentVersion)?.[0] as string) || null;
    return PRERELEASE_TYPE_ALIAS[rawCurrentChannel] || rawCurrentChannel;
  }

  private async getLatestTagName(cancellationToken: CancellationToken): Promise<string | null> {
    const options = this.options;

    const url =
      options.host === null || options.host === 'github.com'
        ? newUrlFromBase(`${this.basePath}/latest`, this.baseUrl)
        : new URL(`${this.computeGithubBasePath(`/repos/${options.owner}/${options.repo}/releases`)}/latest`, this.baseApiUrl);

    try {
      const rawData = await this.httpRequest(url, { accept: 'application/json' }, cancellationToken);
      if (rawData == null) {
        return null;
      }

      const releaseInfo: GithubReleaseInfo = JSON.parse(rawData);
      return releaseInfo.tag_name;
    } catch (e) {
      throw newError(
        `Unable to find latest version on GitHub (${url}), please ensure a production release exists: ${e.stack || e.message}`,
        ErrorCodes.LatestVersionNotFound,
      );
    }
  }

  protected get basePath(): string {
    return `/${this.options.owner}/${this.options.repo}/releases`;
  }

  protected getBaseDownloadPath(tag: string, fileName: string): string {
    return `${this.basePath}/download/${tag}/${fileName}`;
  }
}

function loadPublishOption(): (CustomPublishOptions & CustomGitHubOptions) | undefined {
  let data = undefined;
  try {
    data = require('../assets/publishConfig.json');
  } catch {
    return undefined;
  }

  return {
    ...data,
    updateProvider: CustomGithubProvider,
  };
}

export function createAppUpdater(): AppUpdater | undefined {
  const logger = log.scope('updater');
  try {
    const publishOption = loadPublishOption();
    if (publishOption === undefined) {
      logger.error('Failed to load publish options, disabling updates');
      return undefined;
    }
    const updater = new AppUpdater(publishOption);
    return updater;
  } catch (error: any) {
    logger.error(error);
    return undefined;
  }
}

export interface UpdateAvailable {
  version: string;
}

export enum UpdaterState {
  Idle,
  CheckingForUpdate,
  UpdateAvailable,
  UpdateNotAvailable,
  DownloadingUpdate,
  UpdateDownloaded,
}

export type ListenerSignature = {
  [UpdaterState.Idle]: () => void;
  [UpdaterState.CheckingForUpdate]: () => void;
  [UpdaterState.UpdateAvailable]: (info: UpdateInfo) => void;
  [UpdaterState.UpdateNotAvailable]: (info: UpdateInfo) => void;
  [UpdaterState.DownloadingUpdate]: (progress: ProgressInfo) => void;
  [UpdaterState.UpdateDownloaded]: (info: UpdateInfo) => void;
};

export default class AppUpdater {
  private updater: BaseUpdater;
  private state: UpdaterState = UpdaterState.Idle;
  private lastUpdateInfo: UpdateInfo | undefined = undefined;
  private lastError: Error | undefined = undefined;
  private lastDownloadedUpdate: UpdateInfo | undefined = undefined;
  private listeners: { [K in UpdaterState]: ListenerSignature[K][] } = {
    [UpdaterState.Idle]: [],
    [UpdaterState.CheckingForUpdate]: [],
    [UpdaterState.UpdateAvailable]: [],
    [UpdaterState.UpdateNotAvailable]: [],
    [UpdaterState.DownloadingUpdate]: [],
    [UpdaterState.UpdateDownloaded]: [],
  };
  private logger;

  constructor(publishOption: CustomGitHubOptions) {
    this.logger = log.scope('updater');
    switch (process.platform) {
      case 'darwin':
        this.logger.debug('Using MacUpdater');
        const { MacUpdater } = require('electron-updater');
        this.updater = new MacUpdater();
        break;
      case 'win32':
        this.logger.debug('Using NsisUpdater');
        const { NsisUpdater } = require('electron-updater');
        this.updater = new NsisUpdater();
        break;
      default:
        this.logger.debug(`Using default autoUpdater (platform: ${process.platform})`);
        const { autoUpdater } = require('electron-updater');
        this.updater = autoUpdater;
        break;
    }

    if (this.updater === undefined) {
      throw new TypeError('Updater is undefined');
    }

    this.updater.logger = require('electron-log/node');

    publishOption.nightlyBuild = (process.env.FORCE_NIGHTLY || '0') === '1' || publishOption.nightlyBuild;

    this.updater.setFeedURL(publishOption);
    this.updater.autoDownload = true;
    this.updater.autoInstallOnAppQuit = false;

    this.logger.info(`App version: ${this.updater.currentVersion}`);
    this.logger.debug(`Nightly build: ${publishOption.nightlyBuild}`);
    this.logger.debug(`Auto download: ${this.updater.autoDownload}`);
    this.logger.debug(`Auto install on app quit: ${this.updater.autoInstallOnAppQuit}`);
    this.logger.debug(`Allow prerelease: ${this.updater.allowPrerelease}`);
    this.logger.debug(`Allow downgrade: ${this.updater.allowDowngrade}`);
    this.logger.debug(`Update channel: ${this.updater.channel}`);

    // https://www.electron.build/auto-update#event-error
    this.updater.on('error', (error) => {
      this.logger.error('Update error', error);
      this.state = UpdaterState.Idle;
      this.lastError = error;
      this.emit(this.state);
    });
    // https://www.electron.build/auto-update#event-checking-for-update
    this.updater.on('checking-for-update', () => {
      this.logger.debug('Checking for update');
      this.state = UpdaterState.CheckingForUpdate;
      this.emit(this.state);
    });
    // https://www.electron.build/auto-update#event-update-available
    this.updater.on('update-available', (info) => {
      this.logger.debug('Update available', info);
      this.state = UpdaterState.UpdateAvailable;
      this.lastUpdateInfo = info;
      this.emit(this.state, this.lastUpdateInfo);
    });
    // https://www.electron.build/auto-update#event-update-not-available
    this.updater.on('update-not-available', (info) => {
      this.logger.debug('Update not available', info);
      this.state = UpdaterState.UpdateNotAvailable;
      this.lastUpdateInfo = info;
      this.emit(this.state, this.lastUpdateInfo);
    });
    // https://www.electron.build/auto-update#event-download-progress
    this.updater.on('download-progress', (progress) => {
      this.logger.debug('Download progress', progress);
      this.state = UpdaterState.DownloadingUpdate;
      this.emit(this.state, progress);
    });
    // https://www.electron.build/auto-update#event-update-downloaded
    this.updater.on('update-downloaded', (info) => {
      this.logger.debug('Update downloaded', info);
      this.state = UpdaterState.UpdateDownloaded;
      this.lastDownloadedUpdate = info;
      this.emit(this.state, this.lastDownloadedUpdate);
    });
  }

  private canCheckForUpdates(): boolean {
    return (
      this.state === UpdaterState.Idle || this.state === UpdaterState.UpdateNotAvailable || this.state === UpdaterState.UpdateAvailable
    );
  }

  async checkForUpdates(): Promise<UpdateAvailable | undefined> {
    this.logger.debug('Will check for updates');
    try {
      if (!this.canCheckForUpdates()) {
        this.logger.debug('Cannot check for updates in current state', this.state);
        return;
      }
      this.state = UpdaterState.CheckingForUpdate as UpdaterState;
      try {
        await this.updater.checkForUpdates();
      } catch (error) {
        this.lastError = error;
        this.state = UpdaterState.Idle;
      }
      if (this.state === UpdaterState.UpdateAvailable) {
        return { version: this.lastUpdateInfo!.version };
      }
    } catch (error: any) {
      this.logger.error('Could not check for updates', error);
    }
  }

  quitAndInstall(): boolean {
    try {
      if (!this.isUpdateDownloaded()) {
        this.logger.warn('quitAndInstall() called but update has not been downloaded yet');
        return;
      }
      this.updater.quitAndInstall();
      return true;
    } catch (error: any) {
      this.logger.error('Could not install update', error);
      return false;
    }
  }

  isUpdateDownloaded(): boolean {
    return this.state === UpdaterState.UpdateDownloaded;
  }

  emit<U extends UpdaterState>(event: U, ...args: Parameters<ListenerSignature[U]>): void {
    this.listeners[event].forEach((listener) => (listener as (...args: Parameters<ListenerSignature[U]>) => void)(...args));
  }

  on<U extends UpdaterState>(event: U, listener: ListenerSignature[U]): void {
    this.listeners[event].push(listener);
  }

  removeListener<U extends UpdaterState>(event: U, listener: ListenerSignature[U]): void {
    (this.listeners[event] as ListenerSignature[U][]) = this.listeners[event].filter((l) => l !== listener);
  }

  removeAllListeners(event: UpdaterState): void {
    this.listeners[event] = [];
  }
}

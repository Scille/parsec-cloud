// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import type { CustomPublishOptions, ProgressInfo, UpdateInfo } from 'builder-util-runtime';
import type { Logger } from 'electron-log';
import type { BaseUpdater, AppUpdater as _AppUpdater } from 'electron-updater';
import { GitHubProvider } from 'electron-updater/out/providers/GitHubProvider';
import type { ProviderRuntimeOptions } from 'electron-updater/out/providers/Provider';
import type { CustomPublishOptions as CustomGitHubOptions } from '../assets/publishConfig';

// @ts-expect-error TS2415: GithubProvider don't expose `getChannelFilePrefix` method from `Provider` which we need to override.
class CustomGithubProvider extends GitHubProvider {
  protected readonly _runtimeOptions: ProviderRuntimeOptions;
  protected readonly _options: CustomGitHubOptions;

  constructor(options: CustomGitHubOptions, updater: _AppUpdater, runtimeOptions: ProviderRuntimeOptions) {
    super(
      {
        ...options,
        provider: 'github',
      },
      updater,
      runtimeOptions,
    );
    this._options = options;
    this._runtimeOptions = runtimeOptions;
  }

  /**
   * This function is used to determine the channel file suffix.
   * e.g.: `latest-linux-x64.yml` for the `latest` channel for `linux` platform on 64-bit architecture.
   * @returns {string}
   */
  protected override getChannelFilePrefix() {
    const { machine } = require('node:os');
    const arch = process.env['TEST_UPDATER_ARCH'] || this._options.buildMachineArch || machine();

    switch (this._runtimeOptions.platform) {
      case 'linux':
        return `-linux-${arch}`;
      case 'darwin':
        return `-mac-${arch}`;
      case 'win32':
        return `-win-${arch}`;
      default:
        return `-${this._runtimeOptions.platform}-${arch}`;
    }
  }
}

function loadPublishOption(): (CustomPublishOptions & CustomGitHubOptions) | undefined {
  let data = undefined;
  try {
    data = require('../assets/publishConfig.json');
  } catch {
    console.log('Failed to load publish config file');
    return undefined;
  }

  return {
    ...data,
    updateProvider: CustomGithubProvider,
  };
}

export function createAppUpdater(): AppUpdater | undefined {
  const publishOption = loadPublishOption();
  if (publishOption === undefined) {
    return undefined;
  }
  return new AppUpdater(publishOption);
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

  constructor(publishOption: CustomPublishOptions & CustomGitHubOptions) {
    switch (process.platform) {
      case 'darwin':
        const { MacUpdater } = require('electron-updater');
        this.updater = new MacUpdater();
        break;
      case 'win32':
        const { NsisUpdater } = require('electron-updater');
        this.updater = new NsisUpdater();
        break;
      default:
        console.log(`Unsupported platform: ${process.platform}, trying default updater`);
        const { autoUpdater } = require('electron-updater');
        this.updater = autoUpdater;
        break;
    }

    if (this.updater === undefined) {
      throw new TypeError('Updater is undefined');
    }

    this.updater.logger = require('electron-log/node');

    (this.updater.logger as Logger).transports.file.level = 'debug';
    (this.updater.logger as Logger).transports.console.level = 'debug';

    this.updater.setFeedURL(publishOption);
    this.updater.autoDownload = true;
    this.updater.autoInstallOnAppQuit = false;

    console.log('=============================');
    console.log(`App version: ${this.updater.currentVersion}`);
    console.log(`Auto download: ${this.updater.autoDownload}`);
    console.log(`Auto install on app quit: ${this.updater.autoInstallOnAppQuit}`);
    console.log(`Allow prerelease: ${this.updater.allowPrerelease}`);
    console.log(`Allow downgrade: ${this.updater.allowDowngrade}`);
    console.log(`Update channel: ${this.updater.channel}`);
    console.log('=============================');

    // https://www.electron.build/auto-update#event-error
    this.updater.on('error', (error) => {
      this.state = UpdaterState.Idle;
      this.lastError = error;
      this.emit(this.state);
    });
    // https://www.electron.build/auto-update#event-checking-for-update
    this.updater.on('checking-for-update', () => {
      console.debug('Checking for update');
      this.state = UpdaterState.CheckingForUpdate;
      this.emit(this.state);
    });
    // https://www.electron.build/auto-update#event-update-available
    this.updater.on('update-available', (info) => {
      console.debug('Update available', info);
      this.state = UpdaterState.UpdateAvailable;
      this.lastUpdateInfo = info;
      this.emit(this.state, this.lastUpdateInfo);
    });
    // https://www.electron.build/auto-update#event-update-not-available
    this.updater.on('update-not-available', (info) => {
      console.debug('Update not available', info);
      this.state = UpdaterState.UpdateNotAvailable;
      this.lastUpdateInfo = info;
      this.emit(this.state, this.lastUpdateInfo);
    });
    // https://www.electron.build/auto-update#event-download-progress
    this.updater.on('download-progress', (progress) => {
      console.debug('Download progress', progress);
      this.state = UpdaterState.DownloadingUpdate;
      this.emit(this.state, progress);
    });
    // https://www.electron.build/auto-update#event-update-downloaded
    this.updater.on('update-downloaded', (info) => {
      console.debug('Update downloaded', info);
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
    if (!this.canCheckForUpdates()) {
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
  }

  quitAndInstall(): void {
    if (!this.isUpdateDownloaded()) {
      console.log('quitAndInstall() called when update has not been downloaded.');
      return;
    }
    this.updater.quitAndInstall();
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

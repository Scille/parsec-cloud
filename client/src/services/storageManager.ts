// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { isElectron } from '@/parsec';
import { Storage } from '@ionic/storage';
import { DateTime } from 'luxon';
import { I18n, Locale, Theme } from 'megashark-lib';

export const StorageManagerKey = 'storageManager';
export const ThemeManagerKey = 'themeManager';

export interface StoredDeviceData {
  lastLogin?: DateTime;
  orgCreationDate?: DateTime;
}

export interface BmsAccessData {
  access: string;
  refresh: string;
}

export interface Config {
  locale: Locale;
  theme: Theme;
  enableTelemetry: boolean;
  minimizeToTray: boolean;
  confirmBeforeQuit: boolean;
  meteredConnection: boolean;
  unsyncFiles: boolean;
  skipViewers: boolean;
  skipAccount: boolean;
  defaultAccountServer?: string;
  skipLongPathsSupportWarning: boolean;
  disableDownloadWarning: boolean;
  hideParsecDownload: boolean;
  skipWorkspaceHiddenWarning: boolean;
}

export class StorageManager {
  static STORED_DEVICE_DATA_KEY = 'devicesData';
  static STORED_CONFIG_KEY = 'config';
  static STORED_COMPONENT_PREFIX = 'comp';
  static STORED_BMS_ACCESS_KEY = 'bmsAccess';

  static get DEFAULT_CONFIG(): Config {
    return {
      locale: I18n.getPreferredLocale(),
      theme: Theme.Light,
      // theme: Theme.System, Remove previous line and uncomment this when dark theme will be implemented
      enableTelemetry: true,
      minimizeToTray: true,
      confirmBeforeQuit: true,
      meteredConnection: false,
      unsyncFiles: false,
      skipViewers: false,
      skipAccount: false,
      defaultAccountServer: undefined,
      skipLongPathsSupportWarning: false,
      disableDownloadWarning: false,
      hideParsecDownload: false,
      skipWorkspaceHiddenWarning: false,
    };
  }

  internalStore: Storage;

  constructor() {
    this.internalStore = new Storage();
  }

  async create(): Promise<Storage> {
    const storage = this.internalStore.create();

    const config = await this.retrieveConfig();
    if (isElectron()) {
      window.electronAPI.sendConfig(config);
    }
    return storage;
  }

  async storeDevicesData(data: { [deviceId: string]: StoredDeviceData }): Promise<void> {
    const serialized: { [deviceId: string]: object } = {};

    Object.keys(data).forEach((deviceId: string, _data) => {
      if (data[deviceId]) {
        const deviceData = data[deviceId];
        serialized[deviceId] = {
          lastLogin: deviceData?.lastLogin ? deviceData.lastLogin.toISO() : undefined,
          orgCreationDate: deviceData?.orgCreationDate ? deviceData.orgCreationDate.toISO() : undefined,
        };
      }
    });

    this.internalStore.set(StorageManager.STORED_DEVICE_DATA_KEY, serialized);
  }

  async retrieveDevicesData(): Promise<{ [deviceId: string]: StoredDeviceData }> {
    const data = await this.internalStore.get(StorageManager.STORED_DEVICE_DATA_KEY);
    const deviceData: { [deviceId: string]: StoredDeviceData } = {};

    if (!data) {
      return deviceData;
    }
    Object.keys(data).forEach((deviceId, _data) => {
      if (data[deviceId]) {
        deviceData[deviceId] = {
          // Need to add setZone because Luxon (and JavaScript's date) ignore
          // the timezone part otherwise
          lastLogin: data[deviceId].lastLogin ? DateTime.fromISO(data[deviceId].lastLogin, { setZone: true }) : undefined,
          orgCreationDate: data[deviceId].orgCreationDate ? DateTime.fromISO(data[deviceId].orgCreationDate, { setZone: true }) : undefined,
        };
      }
    });
    return deviceData;
  }

  async storeComponentData<Type>(componentKey: string, data: Type): Promise<void> {
    const key = `${StorageManager.STORED_COMPONENT_PREFIX}_${componentKey}`;
    await this.internalStore.set(key, data);
  }

  async updateComponentData<Type>(componentKey: string, newData: Type, defaultValues: Required<Type>): Promise<void> {
    const key = `${StorageManager.STORED_COMPONENT_PREFIX}_${componentKey}`;
    const data = (await this.retrieveComponentData(componentKey, defaultValues)) as Type;

    for (const element in defaultValues) {
      if (newData[element] !== undefined) {
        data[element] = newData[element];
      } else if (data[element] === undefined) {
        data[element] = defaultValues[element];
      }
    }
    try {
      await this.internalStore.set(key, data);
    } catch (error) {
      console.log(`Failed to serialize ${componentKey}: ${error}`);
    }
  }

  async retrieveComponentData<Type>(componentKey: string, defaultValues: Required<Type>): Promise<Required<Type>> {
    const key = `${StorageManager.STORED_COMPONENT_PREFIX}_${componentKey}`;
    const data = await this.internalStore.get(key);

    try {
      const parsedData = data || {};
      for (const element in defaultValues) {
        if (!(element in parsedData)) {
          parsedData[element] = defaultValues[element];
        }
      }
      return parsedData;
    } catch (error) {
      console.log(`Failed to deserialize ${componentKey}: ${error}`);
      await this.internalStore.set(key, {});
    }
    return defaultValues;
  }

  async storeConfig(data: Config): Promise<void> {
    await this.internalStore.set(StorageManager.STORED_CONFIG_KEY, {
      locale: data.locale,
      theme: data.theme,
      minimizeToTray: data.minimizeToTray,
      confirmBeforeQuit: data.confirmBeforeQuit,
      enableTelemetry: data.enableTelemetry,
      synchroWifiOnly: data.meteredConnection,
      unsyncFiles: data.unsyncFiles,
      skipViewers: data.skipViewers,
      skipAccount: data.skipAccount,
      defaultAccountServer: data.defaultAccountServer,
      skipLongPathsSupportWarning: data.skipLongPathsSupportWarning,
      disableDownloadWarning: data.disableDownloadWarning,
      hideParsecDownload: data.hideParsecDownload,
      skipWorkspaceHiddenWarning: data.skipWorkspaceHiddenWarning,
    });
    window.electronAPI.sendConfig(data);
  }

  async retrieveConfig(): Promise<Config> {
    const data = await this.internalStore.get(StorageManager.STORED_CONFIG_KEY);

    if (!data) {
      return StorageManager.DEFAULT_CONFIG;
    }

    const currentTheme = Object.values(Theme).find((t) => t === data.theme);

    // We could just return directly but we create a new object explicitly
    // so we don't have any implicit deserialization that we don't want
    const config: Config = {
      locale: data.locale ? data.locale : StorageManager.DEFAULT_CONFIG.locale,
      theme: currentTheme ? currentTheme : StorageManager.DEFAULT_CONFIG.theme,
      enableTelemetry: data.enableTelemetry !== undefined ? data.enableTelemetry : StorageManager.DEFAULT_CONFIG.enableTelemetry,
      minimizeToTray: data.minimizeToTray !== undefined ? data.minimizeToTray : StorageManager.DEFAULT_CONFIG.minimizeToTray,
      confirmBeforeQuit: data.confirmBeforeQuit !== undefined ? data.confirmBeforeQuit : StorageManager.DEFAULT_CONFIG.confirmBeforeQuit,
      meteredConnection: data.meteredConnection !== undefined ? data.synchroWifiOnly : StorageManager.DEFAULT_CONFIG.meteredConnection,
      unsyncFiles: data.unsyncFiles !== undefined ? data.unsyncFiles : StorageManager.DEFAULT_CONFIG.unsyncFiles,
      skipViewers: data.skipViewers !== undefined ? data.skipViewers : StorageManager.DEFAULT_CONFIG.skipViewers,
      skipAccount: data.skipAccount ?? StorageManager.DEFAULT_CONFIG.skipAccount,
      defaultAccountServer: data.defaultAccountServer ?? StorageManager.DEFAULT_CONFIG.defaultAccountServer,
      skipLongPathsSupportWarning: data.skipLongPathsSupportWarning ?? StorageManager.DEFAULT_CONFIG.skipLongPathsSupportWarning,
      disableDownloadWarning: data.disableDownloadWarning ?? StorageManager.DEFAULT_CONFIG.disableDownloadWarning,
      hideParsecDownload: data.hideParsecDownload ?? StorageManager.DEFAULT_CONFIG.hideParsecDownload,
      skipWorkspaceHiddenWarning: data.skipWorkspaceHiddenWarning ?? StorageManager.DEFAULT_CONFIG.skipWorkspaceHiddenWarning,
    };
    return config;
  }

  async updateConfig(data: Partial<Config>): Promise<void> {
    const config = await this.retrieveConfig();

    for (const key of Object.keys(data) as (keyof Config)[]) {
      const value = data[key];
      if (value !== undefined) {
        // Don't understand why this doesn't work. `key` is of type `keyof Config`,
        // yet I cannot do `config[key]`.
        (config as any)[key] = value;
      }
    }
    await this.storeConfig(config);
  }

  async storeBmsAccess(tokens: BmsAccessData): Promise<void> {
    await this.internalStore.set(StorageManager.STORED_BMS_ACCESS_KEY, {
      access: tokens.access,
      refresh: tokens.refresh,
    });
  }

  async clearBmsAccess(): Promise<void> {
    await this.internalStore.remove(StorageManager.STORED_BMS_ACCESS_KEY);
  }

  async retrieveBmsAccess(): Promise<BmsAccessData | undefined> {
    return await this.internalStore.get(StorageManager.STORED_BMS_ACCESS_KEY);
  }

  async clearAll(): Promise<void> {
    window.electronAPI.log('warn', 'Clearing all cache');
    await this.internalStore.clear();
  }
}

class StorageManagerInstance {
  private _instance: StorageManager | null = null;
  private _isInit: boolean = false;

  get(): StorageManager {
    if (!this._instance) {
      this._instance = new StorageManager();
    }
    return this._instance;
  }

  async init(): Promise<void> {
    if (!this._isInit) {
      await this.get().create();
      this._isInit = true;
    }
  }
}

export const storageManagerInstance = new StorageManagerInstance();

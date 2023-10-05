// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Storage } from '@ionic/storage';
import { DateTime } from 'luxon';

export interface StoredDeviceData {
  lastLogin: DateTime;
}

export interface Config {
  locale: string;
  theme: string;
  enableTelemetry: boolean;
  minimizeToTray: boolean;
  meteredConnection: boolean;
  unsyncFiles: boolean;
}

export class StorageManager {
  static STORED_DEVICE_DATA_KEY = 'devicesData';
  static STORED_CONFIG_KEY = 'config';

  // TODO: CHANGE BACK THEME TO SYSTEM WHEN DARK MODE WILL BE HERE: https://github.com/Scille/parsec-cloud/issues/5427
  static DEFAULT_CONFIG: Config = {
    locale: '',
    theme: 'light',
    enableTelemetry: true,
    minimizeToTray: true,
    meteredConnection: false,
    unsyncFiles: false,
  };

  internalStore: Storage;

  constructor() {
    this.internalStore = new Storage();
  }

  async create(): Promise<Storage> {
    return this.internalStore.create();
  }

  async storeDevicesData(data: {[slug: string]: StoredDeviceData}): Promise<void> {
    const serialized: {[slug: string]: object} = {};

    Object.keys(data).forEach((slug: string, _data) => {
      if (data[slug] && data[slug].lastLogin) {
        serialized[slug] = {
          lastLogin: data[slug].lastLogin.toISO(),
        };
      }
    });

    this.internalStore.set(StorageManager.STORED_DEVICE_DATA_KEY, serialized);
  }

  async retrieveDevicesData(): Promise<{[slug: string]: StoredDeviceData}> {
    const data = await this.internalStore.get(StorageManager.STORED_DEVICE_DATA_KEY);
    const deviceData: {[slug: string]: StoredDeviceData} = {};

    if (!data) {
      return deviceData;
    }
    Object.keys(data).forEach((slug, _data) => {
      if (data[slug] && data[slug].lastLogin) {
        deviceData[slug] = {
          // Need to add setZone because Luxon (and JavaScript's date) ignore
          // the timezone part otherwise
          lastLogin: DateTime.fromISO(data[slug].lastLogin, {setZone: true}),
        };
      }
    });
    return deviceData;
  }

  async storeConfig(data: Config): Promise<void> {
    await this.internalStore.set(StorageManager.STORED_CONFIG_KEY, {
      locale: data.locale,
      theme: data.theme,
      minimizeToTray: data.minimizeToTray,
      enableTelemetry: data.enableTelemetry,
      synchroWifiOnly: data.meteredConnection,
      unsyncFiles: data.unsyncFiles,
    });
  }

  async retrieveConfig(): Promise<Config> {
    const data = await this.internalStore.get(StorageManager.STORED_CONFIG_KEY);

    if (!data) {
      return StorageManager.DEFAULT_CONFIG;
    }

    // We could just return directly but we create a new object explicitly
    // so we don't have any implicit deserialization that we don't want
    const config: Config = {
      locale: data.locale ? data.locale : StorageManager.DEFAULT_CONFIG.locale,
      theme: data.theme ? data.theme : StorageManager.DEFAULT_CONFIG.theme,
      enableTelemetry: data.enableTelemetry !== undefined ? data.enableTelemetry : StorageManager.DEFAULT_CONFIG.enableTelemetry,
      minimizeToTray: data.minimizeToTray !== undefined ? data.minimizeToTray : StorageManager.DEFAULT_CONFIG.minimizeToTray,
      meteredConnection: data.meteredConnection !== undefined ? data.synchroWifiOnly : StorageManager.DEFAULT_CONFIG.meteredConnection,
      unsyncFiles: data.unsyncFiles !== undefined ? data.unsyncFiles : StorageManager.DEFAULT_CONFIG.unsyncFiles,
    };
    return config;
  }
}

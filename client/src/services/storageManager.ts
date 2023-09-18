// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { isElectron } from '@/parsec';
import { Locale } from '@/services/translation';
import { Storage } from '@ionic/storage';
import { DateTime } from 'luxon';

export const StorageManagerKey = 'storageManager';

export interface StoredDeviceData {
  lastLogin: DateTime;
}

export interface Config {
  locale?: Locale;
  theme: string;
  enableTelemetry: boolean;
  minimizeToTray: boolean;
  confirmBeforeQuit: boolean;
  meteredConnection: boolean;
  unsyncFiles: boolean;
}

export class StorageManager {
  static STORED_DEVICE_DATA_KEY = 'devicesData';
  static STORED_CONFIG_KEY = 'config';
  static STORED_COMPONENT_PREFIX = 'comp';

  // TODO: CHANGE BACK THEME TO SYSTEM WHEN DARK MODE WILL BE HERE: https://github.com/Scille/parsec-cloud/issues/5427
  static get DEFAULT_CONFIG(): Config {
    return {
      theme: 'light',
      enableTelemetry: true,
      minimizeToTray: true,
      confirmBeforeQuit: true,
      meteredConnection: false,
      unsyncFiles: false,
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

  async storeDevicesData(data: { [slug: string]: StoredDeviceData }): Promise<void> {
    const serialized: { [slug: string]: object } = {};

    Object.keys(data).forEach((slug: string, _data) => {
      if (data[slug] && data[slug].lastLogin) {
        serialized[slug] = {
          lastLogin: data[slug].lastLogin.toISO(),
        };
      }
    });

    this.internalStore.set(StorageManager.STORED_DEVICE_DATA_KEY, serialized);
  }

  async retrieveDevicesData(): Promise<{ [slug: string]: StoredDeviceData }> {
    const data = await this.internalStore.get(StorageManager.STORED_DEVICE_DATA_KEY);
    const deviceData: { [slug: string]: StoredDeviceData } = {};

    if (!data) {
      return deviceData;
    }
    Object.keys(data).forEach((slug, _data) => {
      if (data[slug] && data[slug].lastLogin) {
        deviceData[slug] = {
          // Need to add setZone because Luxon (and JavaScript's date) ignore
          // the timezone part otherwise
          lastLogin: DateTime.fromISO(data[slug].lastLogin, { setZone: true }),
        };
      }
    });
    return deviceData;
  }

  async storeComponentData<Type>(componentKey: string, data: Type): Promise<void> {
    const key = `${StorageManager.STORED_COMPONENT_PREFIX}_${componentKey}`;
    await this.internalStore.set(key, JSON.stringify(data));
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
      await this.internalStore.set(key, JSON.stringify(data));
    } catch (error) {
      console.log(`Failed to serialize ${componentKey}: ${error}`);
    }
  }

  async retrieveComponentData<Type>(componentKey: string, defaultValues: Required<Type>): Promise<Required<Type>> {
    const key = `${StorageManager.STORED_COMPONENT_PREFIX}_${componentKey}`;
    const data = await this.internalStore.get(key);

    try {
      let parsedData = JSON.parse(data);
      if (!parsedData) {
        parsedData = {};
      }
      for (const element in defaultValues) {
        if (!(element in parsedData)) {
          parsedData[element] = defaultValues[element];
        }
      }
      return parsedData;
    } catch (error) {
      console.log(`Failed to deserialize ${componentKey}: ${error}`);
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
    });
    window.electronAPI.sendConfig(data);
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
      confirmBeforeQuit: data.confirmBeforeQuit !== undefined ? data.confirmBeforeQuit : StorageManager.DEFAULT_CONFIG.confirmBeforeQuit,
      meteredConnection: data.meteredConnection !== undefined ? data.synchroWifiOnly : StorageManager.DEFAULT_CONFIG.meteredConnection,
      unsyncFiles: data.unsyncFiles !== undefined ? data.unsyncFiles : StorageManager.DEFAULT_CONFIG.unsyncFiles,
    };
    return config;
  }
}

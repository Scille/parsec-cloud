// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { Storage } from '@ionic/storage';
import { StoredDeviceData, Config, StorageManager } from '@/services/storageManager';

describe('Storage manager', () => {
  it('Stores and retrieves the config', async () => {

    const store = new Storage();
    await store.create();

    const c: Config = {
      theme: 'TheTheme',
      locale: 'fr-FR',
      minimizeToTray: false,
      enableTelemetry: false
    };

    const manager = new StorageManager();
    await manager.create();

    // Default should be StorageManager.DEFAULT_CONFIG
    expect(await manager.retrieveConfig()).toStrictEqual(StorageManager.DEFAULT_CONFIG);

    // Nothing is stored in local storage
    let data = await store.get(StorageManager.STORED_CONFIG_KEY);
    expect(data).toBe(null);

    await manager.storeConfig(c);

    data = await store.get(StorageManager.STORED_CONFIG_KEY);
    expect(data).toStrictEqual({
      theme: 'TheTheme',
      locale: 'fr-FR',
      minimizeToTray: false,
      enableTelemetry: false
    });

    expect(await manager.retrieveConfig()).toStrictEqual(c);
  });

  it('Stored and retrieves devices data', async () => {
    const store = new Storage();
    await store.create();

    const manager = new StorageManager();
    await manager.create();

    const d: {[slug: string]: StoredDeviceData} = {
      'abcd': {lastLogin: new Date('2022-02-02T00:00:00.000Z')},
      'efgh': {lastLogin: new Date('2011-01-01T00:00:00.000Z')}
    };

    // Nothing is stored
    expect(await manager.retrieveDevicesData()).toStrictEqual({});
    expect(await store.get(StorageManager.STORED_DEVICE_DATA_KEY)).toStrictEqual(null);

    await manager.storeDevicesData(d);

    // Comparing dates as string
    expect(await store.get(StorageManager.STORED_DEVICE_DATA_KEY)).toStrictEqual({
      'abcd': {
        lastLogin: '2022-02-02T00:00:00.000Z'
      },
      'efgh': {
        lastLogin: '2011-01-01T00:00:00.000Z'
      }
    });

    // Comparing dates as Date()
    expect(await manager.retrieveDevicesData()).toStrictEqual(d);
  });
});

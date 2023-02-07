// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { Storage } from '@ionic/storage';

import { AvailableDevice } from '../plugins/libparsec/definitions';

const MOCK_DEVICES: AvailableDevice[] = [
  {
    organizationId: 'Planet Express',
    humanHandle: 'Dr. John A. Zoidberg',
    deviceLabel: 'device_label',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug1',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'Princeton–Plainsboro Hospital',
    humanHandle: 'Dr. Gregory House',
    deviceLabel: 'device_label',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug2',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'Black Mesa',
    humanHandle: 'Dr. Gordon Freeman',
    deviceLabel: 'device_label',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug3',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'OsCorp',
    humanHandle: 'Dr. Otto G. Octavius',
    deviceLabel: 'device_label',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug4',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'Sanctum Sanctorum',
    humanHandle: 'Dr. Stephen Strange',
    deviceLabel: 'device_label',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug5',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'Holmes Consulting',
    humanHandle: 'Dr John H. Watson',
    deviceLabel: 'device_label',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug6',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'Riviera M.D.',
    humanHandle: 'Dr. Nicholas "Nick" Riviera',
    deviceLabel: 'device_label',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug7',
    ty: {tag: 'Password'}
  }
];

export function getMockDevices(count = -1): AvailableDevice[] {
  if (count === -1) {
    return MOCK_DEVICES;
  }
  return MOCK_DEVICES.slice(0, count);
}

export function mockLastLogin(): void {
  const store = new Storage();
  const now = new Date();

  // Since dates don't handle +/- operators properly
  function newDateWithOffset(d: Date, n: number): Date {
    const x = new Date();
    x.setTime(d.getTime() + n);
    return x;
  }

  store.create().then(() => {
    store.set('devicesData', {
      slug1: { lastLogin: now },
      // 10 seconds ago
      slug2: { lastLogin: newDateWithOffset(now, -1 * 10 * 1000) },
      // 10 minutes ago
      slug3: { lastLogin: newDateWithOffset(now, -1 * 10 * 60 * 1000) },
      // Never logged
      slug4: { },
      // 5 hours ago
      slug5: { lastLogin: newDateWithOffset(now, -1 * 5 * 60 * 60 * 1000) },
      // 2 days ago
      slug6: { lastLogin: newDateWithOffset(now, -1 * 2 * 24 * 60 * 60 * 1000) },
      // 10 days ago
      slug7: { lastLogin: newDateWithOffset(now, -1 * 10 * 24 * 60 * 60 * 1000) }
    });
  });
}

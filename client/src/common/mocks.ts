// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// cSpell:disable

import { DateTime } from 'luxon';
import { AvailableDevice } from '../plugins/libparsec/definitions';
import { StorageManager } from '@/services/storageManager';
import { uniqueNamesGenerator, adjectives, colors, animals } from 'unique-names-generator';

export interface MockFile {
  name: string,
  type: 'folder' | 'file',
  size: number,
  lastUpdate: DateTime,
  updater: string,
  children: MockFile[]
}

export function pathInfo(_path: string): MockFile {
  const UPDATER = ['John', 'Steve', 'Bill', 'Marjolaine', 'Hilary'];

  const childrenCount = Math.floor(Math.random() * 25) + 10;
  const ret: MockFile = {
    name: uniqueNamesGenerator({dictionaries: [adjectives, colors, animals]}),
    type: 'folder',
    updater: UPDATER[Math.floor(Math.random() * UPDATER.length)],
    size: 0,
    lastUpdate: DateTime.now(),
    children: []
  };

  for (let i = 0; i < childrenCount; i++) {
    const isFolder = Math.floor(Math.random() * 2) === 0;
    ret.children.push({
      name: uniqueNamesGenerator({dictionaries: [adjectives, colors, animals]}),
      type: isFolder ? 'folder' : 'file',
      size: isFolder ? 0 : Math.floor(Math.random() * 10000000),
      lastUpdate: DateTime.now(),
      updater: UPDATER[Math.floor(Math.random() * UPDATER.length)],
      children: []
    });
  }
  return ret;
}

export enum WorkspaceRole {
  Owner = 'owner',
  Manager = 'manager',
  Contributor = 'contributor',
  Reader = 'reader'
}

export interface MockWorkspace {
  id: string;
  name: string;
  sharedWith: string[];
  size: number;
  role: WorkspaceRole;
  availableOffline: boolean;
  lastUpdate: DateTime;
}

const MOCK_WORKSPACES: MockWorkspace[] = [
  {
    id: 'id1',
    name: 'Trademeet',
    sharedWith: ['Me', 'Cernd', 'Valygar Corthala'],
    size: 60_817_408,
    role: WorkspaceRole.Reader,
    availableOffline: false,
    lastUpdate: DateTime.fromISO('2023-05-10T08:00:00')
  },
  {
    id: 'id2',
    name: 'The Copper Coronet',
    sharedWith: ['Me', 'Korgan Bloodaxe', 'Anomen Delryn', 'Nalia De\'Arnise', 'Jaheira', 'Yoshimo'],
    size: 8_589_934_592,
    role: WorkspaceRole.Owner,
    availableOffline: true,
    lastUpdate: DateTime.fromISO('2023-05-08T12:00:00')
  },
  {
    id: 'id3',
    name: 'The Asylum',
    sharedWith: ['Me', 'Imoen'],
    size: 628_097_024,
    role: WorkspaceRole.Contributor,
    availableOffline: true,
    lastUpdate: DateTime.fromISO('2023-04-07T12:00:00')
  },
  {
    id: 'id4',
    name: 'Druid Grove',
    sharedWith: ['Me'],
    size: 33_382,
    role: WorkspaceRole.Owner,
    availableOffline: false,
    lastUpdate: DateTime.fromISO('2023-05-07T02:00:00')
  },
  {
    id: 'id5',
    name: 'Menzoberranzan',
    sharedWith: ['Me', 'Drizzt Do\'Urden', 'Viconia', 'Jan Jansen'],
    size: 4_214_402_531,
    role: WorkspaceRole.Manager,
    availableOffline: true,
    lastUpdate: DateTime.fromISO('2023-05-09T08:00:00')
  }
];

export async function getMockWorkspaces(): Promise<MockWorkspace[]> {
  return MOCK_WORKSPACES;
}

const MOCK_DEVICES: AvailableDevice[] = [
  {
    organizationId: 'Planet Express',
    humanHandle: 'Dr. John A. Zoidberg',
    deviceLabel: 'Zoidberg_Device',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug1',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'Princeton-Plainsboro Hospital',
    humanHandle: 'Dr. Gregory House',
    deviceLabel: 'House_Device',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug2',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'Black Mesa',
    humanHandle: 'Dr. Gordon Freeman',
    deviceLabel: 'Freeman_Device',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug3',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'OsCorp',
    humanHandle: 'Dr. Otto G. Octavius',
    deviceLabel: 'Octavius_Device',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug4',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'Sanctum Sanctorum',
    humanHandle: 'Dr. Stephen Strange',
    deviceLabel: 'Strange_Device',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug5',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'Holmes Consulting',
    humanHandle: 'Dr John H. Watson',
    deviceLabel: 'Watson_Device',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug6',
    ty: {tag: 'Password'}
  },
  {
    organizationId: 'Riviera M.D.',
    humanHandle: 'Dr. Nicholas "Nick" Riviera',
    deviceLabel: 'Riviera_Device',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug7',
    ty: {tag: 'Password'}
  }
];

export function getMockDevices(count: number | undefined = undefined): AvailableDevice[] {
  if (!count) {
    return MOCK_DEVICES;
  }
  return MOCK_DEVICES.slice(0, count);
}

export async function mockLastLogin(manager: StorageManager): Promise<void> {
  const now = DateTime.now();

  await manager.storeDevicesData({
    slug1: { lastLogin: now },
    // 10 seconds ago
    slug2: { lastLogin: now.minus({seconds: 10}) },
    // 10 minutes ago
    slug3: { lastLogin: now.minus({minutes: 10}) },
    // 5 hours ago
    slug5: { lastLogin: now.minus({hours: 5}) },
    // 2 days ago
    slug6: { lastLogin: now.minus({days: 2}) },
    // 10 days ago
    slug7: { lastLogin: now.minus({days: 10}) }
  });
}

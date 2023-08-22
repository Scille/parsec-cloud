// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// cSpell:disable

import { DateTime } from 'luxon';
import { AvailableDevice, Handle } from '@/plugins/libparsec/definitions';
import { StorageManager } from '@/services/storageManager';
import { uniqueNamesGenerator, adjectives, colors, animals } from 'unique-names-generator';

export function getAppVersion(): string {
  return '3.0.0a';
}

export const DEFAULT_HANDLE: Handle = 42;

export interface MockFile {
  id: string,
  name: string,
  type: 'folder' | 'file',
  size: number,
  lastUpdate: DateTime,
  updater: string,
  children: MockFile[],
}

export function pathInfo(_path: string, random = false): MockFile {
  function fixedPathInfo(): MockFile {
    const ret: MockFile = {
      id: '0',
      name: 'My Folder 1',
      type: 'folder',
      updater: 'Marjolaine',
      size: 0,
      lastUpdate: DateTime.now(),
      children: [],
    };

    ret.children.push({
      id: '1',
      name: 'My File 1',
      type: 'file',
      size: 123_456_789,
      lastUpdate: DateTime.now(),
      updater: 'Steve',
      children: [],
    });
    ret.children.push({
      id: '2',
      name: 'My Folder 2',
      type: 'folder',
      size: 0,
      lastUpdate: DateTime.now(),
      updater: 'Hilary',
      children: [],
    });

    return ret;
  }

  function randomPathInfo(): MockFile {
    const UPDATER = ['John', 'Steve', 'Bill', 'Marjolaine', 'Hilary'];

    const ret: MockFile = {
      id: '0',
      name: uniqueNamesGenerator({ dictionaries: [adjectives, colors, animals] }),
      type: 'folder',
      updater: UPDATER[Math.floor(Math.random() * UPDATER.length)],
      size: 0,
      lastUpdate: DateTime.now(),
      children: [],
    };

    const childrenCount = Math.floor(Math.random() * 25) + 1;
    for (let i = 0; i < childrenCount; i++) {
      const isFolder = Math.floor(Math.random() * 2) === 0;
      ret.children.push({
        id: String(i + 1),
        name: uniqueNamesGenerator({ dictionaries: [adjectives, colors, animals] }),
        type: isFolder ? 'folder' : 'file',
        size: isFolder ? 0 : Math.floor(Math.random() * 10000000),
        lastUpdate: DateTime.now(),
        updater: UPDATER[Math.floor(Math.random() * UPDATER.length)],
        children: [],
      });
    }
    return ret;
  }

  if (random) {
    return randomPathInfo();
  }
  return fixedPathInfo();
}

export enum WorkspaceRole {
  Owner = 'owner',
  Manager = 'manager',
  Contributor = 'contributor',
  Reader = 'reader',
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

export enum Profile {
  Admin = 'administrator',
  Standard = 'standard',
  Outsider = 'outsider',
}

export interface MockUser {
  id: string,
  name: string,
  email: string,
  avatar: string,
  joined: DateTime,
  profile: Profile,
  revoked: boolean
}

export function getCurrentUserProfile(_handle: Handle): Profile {
  return Profile.Admin;
}

const MOCK_USERS: MockUser[] = [
  {
    id: 'id1',
    name: 'Cernd',
    email: 'cernd@gmail.com',
    avatar: 'ce',
    joined: DateTime.fromISO('2023-05-10T08:00:00'),
    profile: Profile.Standard,
    revoked: false,
  },
  {
    id: 'id2',
    name: 'Valygar Corthala',
    email: 'val@gmail.com',
    avatar: 'vc',
    joined: DateTime.fromISO('2022-01-10T08:00:00'),
    profile: Profile.Standard,
    revoked: false,
  },
  {
    id: 'id3',
    name: 'Drizzt Do\'Urden',
    email: 'drozrt@gmail.com',
    avatar: 'dd',
    joined: DateTime.fromISO('2023-04-10T08:00:00'),
    profile: Profile.Admin,
    revoked: false,
  },
  {
    id: 'id4',
    name: 'Coloia Hoji',
    email: 'coloia@gmail.com',
    avatar: 'ch',
    joined: DateTime.fromISO('2023-04-10T08:00:00'),
    profile: Profile.Outsider,
    revoked: false,
  },
];

export async function getMockUsers(): Promise<MockUser[]> {
  return MOCK_USERS;
}

export interface MockInvitation {
  token: string,
  email: string,
  date: DateTime,
}

export async function getInvitations(): Promise<MockInvitation[]> {
  return [{
    token: 'ebb4df8f132f1f6ced43c73c69c67ab48fcd26d3',
    email: 'shadowheart@swordcoast.faerun',
    date: DateTime.fromISO('2023-05-10T08:00:00'),

  }, {
    token: 'ef87e15fffecf4266191287f5213d8f96ca12ed6',
    email: 'gale@waterdeep.faerun',
    date: DateTime.fromISO('2023-05-10T08:00:00'),
  }];
}

const MOCK_WORKSPACES: MockWorkspace[] = [
  {
    id: 'id1',
    name: 'Trademeet',
    sharedWith: ['Me', 'Cernd', 'Valygar Corthala'],
    size: 60_817_408,
    role: WorkspaceRole.Reader,
    availableOffline: false,
    lastUpdate: DateTime.fromISO('2023-05-10T08:00:00'),
  },
  {
    id: 'id2',
    name: 'The Copper Coronet',
    sharedWith: ['Me', 'Korgan Bloodaxe', 'Anomen Delryn', 'Nalia De\'Arnise', 'Viconia', 'Yoshimo'],
    size: 8_589_934_592,
    role: WorkspaceRole.Owner,
    availableOffline: true,
    lastUpdate: DateTime.fromISO('2023-05-08T12:00:00'),
  },
  {
    id: 'id3',
    name: 'The Asylum',
    sharedWith: ['Me', 'Imoen'],
    size: 628_097_024,
    role: WorkspaceRole.Contributor,
    availableOffline: true,
    lastUpdate: DateTime.fromISO('2023-04-07T12:00:00'),
  },
  {
    id: 'id4',
    name: 'Druid Grove',
    sharedWith: ['Me'],
    size: 33_382,
    role: WorkspaceRole.Owner,
    availableOffline: false,
    lastUpdate: DateTime.fromISO('2023-05-07T02:00:00'),
  },
  {
    id: 'id5',
    name: 'Menzoberranzan',
    sharedWith: ['Me', 'Drizzt Do\'Urden', 'Viconia', 'Korgan Bloodaxe'],
    size: 4_214_402_531,
    role: WorkspaceRole.Manager,
    availableOffline: true,
    lastUpdate: DateTime.fromISO('2023-05-09T08:00:00'),
  },
];

export async function getMockWorkspaces(): Promise<MockWorkspace[]> {
  return MOCK_WORKSPACES;
}

export async function getUsers(): Promise<string[]> {
  return [
    'Cernd',
    'Valygar Corthala',
    'Drizzt Do\'Urden',
    'Viconia',
    'Jan Jansen',
    'Imoen',
    'Korgan Bloodaxe',
    'Anomen Delryn',
    'Nalia De\'Arnise',
    'Jaheira',
    'Yoshimo',
  ];
}

export async function getWorkspaceUsers(workspaceId: string): Promise<Map<string, WorkspaceRole | null>> {
  const users = new Map<string, WorkspaceRole | null>();

  const workspace: MockWorkspace | undefined = MOCK_WORKSPACES.find((w) => w.id === workspaceId);
  const ROLES = [WorkspaceRole.Contributor, WorkspaceRole.Manager, WorkspaceRole.Owner, WorkspaceRole.Reader];
  let index = 0;

  for (const user of await getUsers()) {
    if (workspace && workspace.sharedWith.includes(user)) {
      users.set(user, ROLES[index % ROLES.length]);
    } else {
      users.set(user, null);
    }
    index += 1;
  }

  return users;
}

const MOCK_DEVICES: AvailableDevice[] = [
  {
    organizationId: 'Planet Express',
    humanHandle: {
      label: 'Dr. John A. Zoidberg',
      email: 'john.zoidberg@platnet-express.com',
    },
    deviceLabel: 'Zoidberg_Device',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug1',
    ty: { tag: 'Password' },
  },
  {
    organizationId: 'Princeton-Plainsboro Hospital',
    humanHandle: {
      label: 'Dr. Gregory House',
      email: 'g.house@hp-princeton-nj.com',
    },
    deviceLabel: 'House_Device',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug2',
    ty: { tag: 'Password' },
  },
  {
    organizationId: 'Black Mesa',
    humanHandle: {
      label: 'Dr. Gordon Freeman',
      email: 'freeman.gordon@black-mesa.com',
    },
    deviceLabel: 'Freeman_Device',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug3',
    ty: { tag: 'Password' },
  },
  {
    organizationId: 'OsCorp',
    humanHandle: {
      label: 'Dr. Otto G. Octavius',
      email: 'octavius@oscorp.com',
    },
    deviceLabel: 'Octavius_Device',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug4',
    ty: { tag: 'Password' },
  },
  {
    organizationId: 'Sanctum Sanctorum',
    humanHandle: {
      label: 'Dr. Stephen Strange',
      email: 'stephen@strange.com',
    },
    deviceLabel: 'Strange_Device',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug5',
    ty: { tag: 'Password' },
  },
  {
    organizationId: 'Holmes Consulting',
    humanHandle: {
      label: 'Dr John H. Watson',
      email: 'john.watson@detective.uk',
    },
    deviceLabel: 'Watson_Device',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug6',
    ty: { tag: 'Password' },
  },
  {
    organizationId: 'Riviera M.D.',
    humanHandle: {
      label: 'Dr. Nicholas "Nick" Riviera',
      email: 'nick@riviera.com',
    },
    deviceLabel: 'Riviera_Device',
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug7',
    ty: { tag: 'Password' },
  },
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
    slug2: { lastLogin: now.minus({ seconds: 10 }) },
    // 10 minutes ago
    slug3: { lastLogin: now.minus({ minutes: 10 }) },
    // 5 hours ago
    slug5: { lastLogin: now.minus({ hours: 5 }) },
    // 2 days ago
    slug6: { lastLogin: now.minus({ days: 2 }) },
    // 10 days ago
    slug7: { lastLogin: now.minus({ days: 10 }) },
  });
}

export interface Change {
  description: string,
  issue?: string
}

export interface VersionChange {
  version: string,
  date: DateTime,
  features: Change[],
  fixes: Change[],
  misc: Change[],
}

// Added multiple values for testing
// can be removed before release
export function getChanges(): VersionChange[] {
  return [{
    version: '3.0.0a',
    date: DateTime.fromISO('2023-09-01T17:00:00'),
    features: [{
      description: 'Added changelog modal',
      issue: '4978',
    }, {
      description: 'Added user greet modal',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    }],
    fixes: [{
      description: 'Made links to accept TOS when creating a new organization clickable',
      issue: '4954',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    }],
    misc: [{
      description: 'Updates the design of the modal to greet a new user',
      issue: '4985',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    },
    {
      description: 'Added changelog modal',
      issue: '4978',
    }],
  }];
}

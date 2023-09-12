// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// cSpell:disable

import { DateTime } from 'luxon';
import { AvailableDevice, Handle, DeviceFileType } from '@/plugins/libparsec/definitions';
import { StorageManager } from '@/services/storageManager';
import { uniqueNamesGenerator, adjectives, colors, animals } from 'unique-names-generator';

export function getAppVersion(): string {
  return '3.0.0a';
}

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

export type WorkspaceName = string;
export type WorkspaceID = string;

export function getSharedWith(workspace: MockWorkspace): string[] {
  const people = [];
  for (const [key, value] of workspace.sharingInfo) {
    if (value) {
      people.push(key);
    }
  }
  return people;
}

export interface MockWorkspace {
  id: WorkspaceID;
  name: WorkspaceName;
  sharingInfo: Map<string, WorkspaceRole | null>,
  size: number;
  role: WorkspaceRole;
  availableOffline: boolean;
  lastUpdate: DateTime;
}

const MOCK_WORKSPACES: MockWorkspace[] = [
  {
    id: '1',
    name: 'Trademeet',
    sharingInfo: new Map(),
    size: 60_817_408,
    role: WorkspaceRole.Contributor,
    availableOffline: false,
    lastUpdate: DateTime.fromISO('2023-05-10T08:00:00'),
  },
  {
    id: '2',
    name: 'The Copper Coronet',
    sharingInfo: new Map(),
    size: 8_589_934_592,
    role: WorkspaceRole.Contributor,
    availableOffline: true,
    lastUpdate: DateTime.fromISO('2023-05-08T12:00:00'),
  },
  {
    id: '3',
    name: 'The Asylum',
    sharingInfo: new Map(),
    size: 628_097_024,
    role: WorkspaceRole.Owner,
    availableOffline: true,
    lastUpdate: DateTime.fromISO('2023-04-07T12:00:00'),
  },
  {
    id: '4',
    name: 'Druid Grove',
    sharingInfo: new Map(),
    size: 33_382,
    role: WorkspaceRole.Owner,
    availableOffline: false,
    lastUpdate: DateTime.fromISO('2023-05-07T02:00:00'),
  },
  {
    id: '5',
    name: 'Menzoberranzan',
    sharingInfo: new Map(),
    size: 4_214_402_531,
    role: WorkspaceRole.Reader,
    availableOffline: true,
    lastUpdate: DateTime.fromISO('2023-05-09T08:00:00'),
  },
];

const WORKSPACE_SHARING_INFO: Array<[WorkspaceID, Map<string, WorkspaceRole | null>]> = [
  [
    '1',
    new Map<string, WorkspaceRole | null>([
      ['Me', WorkspaceRole.Contributor],
      ['Cernd', WorkspaceRole.Reader],
      ['Valygar Corthala', WorkspaceRole.Owner],
    ]),
  ], [
    '2',
    new Map<string, WorkspaceRole | null>([
      ['Me', WorkspaceRole.Contributor],
      ['Korgan Bloodaxe', WorkspaceRole.Contributor],
      ['Anomen Delryn', WorkspaceRole.Contributor],
      ['Nalia De\'Arnise', WorkspaceRole.Owner],
      ['Viconia', WorkspaceRole.Owner],
      ['Yoshimo', WorkspaceRole.Reader],
    ]),
  ], [
    '3',
    new Map<string, WorkspaceRole | null>([
      ['Me', WorkspaceRole.Owner],
      ['Imoen', WorkspaceRole.Owner],
    ]),
  ], [
    '4',
    new Map<string, WorkspaceRole | null>([
      ['Me', WorkspaceRole.Owner],
    ]),
  ], [
    '5',
    new Map<string, WorkspaceRole | null>([
      ['Me', WorkspaceRole.Reader],
      ['Korgan Bloodaxe', WorkspaceRole.Contributor],
      ['Drizzt Do\'Urden', WorkspaceRole.Owner],
      ['Viconia', WorkspaceRole.Owner],
    ]),
  ],
];

export async function getWorkspaceInfo(workspaceId: WorkspaceID): Promise<MockWorkspace | null> {
  const workspace = MOCK_WORKSPACES.find((item) => item.id === workspaceId);

  if (!workspace) {
    return null;
  }
  workspace.sharingInfo = await getWorkspaceSharingInfo(workspaceId);
  return workspace;
}

export async function getWorkspaceSharingInfo(workspaceId: WorkspaceID): Promise<Map<string, WorkspaceRole | null>> {
  const elem: [WorkspaceID, Map<string, WorkspaceRole | null>] | undefined = WORKSPACE_SHARING_INFO.find((item) => item[0] === workspaceId);
  const sharingInfo = elem ? elem[1] : new Map<string, WorkspaceRole | null>();

  for (const user of await getUsers()) {
    if (!sharingInfo.get(user)) {
      sharingInfo.set(user, null);
    }
  }

  return sharingInfo;
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

const MOCK_REVOKED_USERS: MockUser[] = [
  {
    id: 'id5',
    name: 'Steve',
    email: 'steve@gmail.com',
    avatar: 'st',
    joined: DateTime.fromISO('2023-05-10T08:00:00'),
    profile: Profile.Standard,
    revoked: true,
  },
  {
    id: 'id6',
    name: 'Alex',
    email: 'alex@gmail.com',
    avatar: 'al',
    joined: DateTime.fromISO('2022-01-10T08:00:00'),
    profile: Profile.Standard,
    revoked: true,
  },
  {
    id: 'id7',
    name: 'Notch',
    email: 'notch@gmail.com',
    avatar: 'no',
    joined: DateTime.fromISO('2023-04-10T08:00:00'),
    profile: Profile.Admin,
    revoked: true,
  },
  {
    id: 'id8',
    name: 'Herobrine',
    email: 'herobrine@gmail.com',
    avatar: 'he',
    joined: DateTime.fromISO('2023-04-10T08:00:00'),
    profile: Profile.Outsider,
    revoked: true,
  },
];

export async function getMockUsers(): Promise<MockUser[]> {
  return MOCK_USERS;
}

export async function getMockRevokedUsers(): Promise<MockUser[]> {
  return MOCK_REVOKED_USERS;
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
    ty: DeviceFileType.Password,
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
    ty: DeviceFileType.Password,
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
    ty: DeviceFileType.Password,
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
    ty: DeviceFileType.Password,
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
    ty: DeviceFileType.Password,
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
    ty: DeviceFileType.Password,
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
    ty: DeviceFileType.Password,
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

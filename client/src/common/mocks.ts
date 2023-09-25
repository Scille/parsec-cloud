// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// cSpell:disable

import { DateTime } from 'luxon';
import { Handle, UserProfile } from '@/parsec';
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

export interface MockUser {
  id: string,
  name: string,
  email: string,
  avatar: string,
  joined: DateTime,
  profile: UserProfile,
  revoked: boolean
}

export function getCurrentUserProfile(_handle: Handle): UserProfile {
  return UserProfile.Admin;
}

const MOCK_USERS: MockUser[] = [
  {
    id: 'id1',
    name: 'Cernd',
    email: 'cernd@gmail.com',
    avatar: 'ce',
    joined: DateTime.fromISO('2023-05-10T08:00:00'),
    profile: UserProfile.Standard,
    revoked: false,
  },
  {
    id: 'id2',
    name: 'Valygar Corthala',
    email: 'val@gmail.com',
    avatar: 'vc',
    joined: DateTime.fromISO('2022-01-10T08:00:00'),
    profile: UserProfile.Standard,
    revoked: false,
  },
  {
    id: 'id3',
    name: 'Drizzt Do\'Urden',
    email: 'drozrt@gmail.com',
    avatar: 'dd',
    joined: DateTime.fromISO('2023-04-10T08:00:00'),
    profile: UserProfile.Admin,
    revoked: false,
  },
  {
    id: 'id4',
    name: 'Coloia Hoji',
    email: 'coloia@gmail.com',
    avatar: 'ch',
    joined: DateTime.fromISO('2023-04-10T08:00:00'),
    profile: UserProfile.Outsider,
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
    profile: UserProfile.Standard,
    revoked: true,
  },
  {
    id: 'id6',
    name: 'Alex',
    email: 'alex@gmail.com',
    avatar: 'al',
    joined: DateTime.fromISO('2022-01-10T08:00:00'),
    profile: UserProfile.Standard,
    revoked: true,
  },
  {
    id: 'id7',
    name: 'Notch',
    email: 'notch@gmail.com',
    avatar: 'no',
    joined: DateTime.fromISO('2023-04-10T08:00:00'),
    profile: UserProfile.Admin,
    revoked: true,
  },
  {
    id: 'id8',
    name: 'Herobrine',
    email: 'herobrine@gmail.com',
    avatar: 'he',
    joined: DateTime.fromISO('2023-04-10T08:00:00'),
    profile: UserProfile.Outsider,
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

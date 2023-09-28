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

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// cSpell:disable

import { DateTime } from 'luxon';
import { AvailableDevice, Handle, DeviceFileType, UserProfile, WorkspaceRole, WorkspaceID, WorkspaceName } from '@/parsec';
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
      ['Cernd', WorkspaceRole.Reader],
      ['Valygar Corthala', WorkspaceRole.Owner],
    ]),
  ], [
    '2',
    new Map<string, WorkspaceRole | null>([
      ['Korgan Bloodaxe', WorkspaceRole.Contributor],
      ['Anomen Delryn', WorkspaceRole.Contributor],
      ['Nalia De\'Arnise', WorkspaceRole.Owner],
      ['Viconia', WorkspaceRole.Owner],
      ['Yoshimo', WorkspaceRole.Reader],
    ]),
  ], [
    '3',
    new Map<string, WorkspaceRole | null>([
      ['Imoen', WorkspaceRole.Owner],
    ]),
  ], [
    '4',
    new Map<string, WorkspaceRole | null>([
    ]),
  ], [
    '5',
    new Map<string, WorkspaceRole | null>([
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
